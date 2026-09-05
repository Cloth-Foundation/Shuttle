use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use tempfile::TempDir;

pub struct Fixture {
    _directory: TempDir,
    pub root: PathBuf,
}

pub const STRUCT_OUTPUT: &[u8] = b"100\n101\ntrue\n101\nalive\n<data-models.Packet>\ndata-models.Packet\n100\n101\n104\ninitial\n";
pub const CHECKED_UPDATES_OUTPUT: &[u8] = b"7\n22\n1\n21\n42\n";
pub const INTEGER_CONVERSIONS_OUTPUT: &[u8] = b"4464\n65535\n0\n44\n255\n";
pub const NUMERIC_NOTATION_OUTPUT: &[u8] = b"240\n42\n18446744073709551615\n125\n44\n0\n8\n";
pub const TYPED_LITERALS_OUTPUT: &[u8] = b"7\n42\n18446744073709551615\n0.5\n44\n0\n8\n";
pub const TYPED_ERRORS_OUTPUT: &[u8] = b"7\n";
pub const SWITCH_OUTPUT: &[u8] =
    b"ready\nready\nactive\nfallback\ndone\nfallback\nsmall\nother\nmaximum\n";
pub const SWITCH_CASES: &[&str] = &["Ready", "ready", "_Done"];

pub const SWITCH_MAIN: &str = r#"
import models::State as Status;
import models::StateReader as Constants;
static func Describe(Status state): string {
  switch (state) {
    case Status.Ready: { return "ready"; }
    case Constants.Initial, Status._Done: { return Constants.Name(state); }
  }
}
static func Fallback(Status state): string {
  switch (state) { case Status.Ready: { return "ready"; } default: { return "fallback"; } }
}
static func Number(int64 value): string {
  switch (value) { case Constants.Small: { return "small"; } default: { return "other"; } }
}
static func Main() {
  for (var state in Constants.Values()) {
    println(Describe(state));
    println(Fallback(state));
  }
  println(Number(7));
  println(Number(9));
  uint64 maximum = 18446744073709551615;
  switch (maximum) { case Constants.Maximum: { println("maximum"); } default: { println("wrong"); } }
}
"#;

impl Fixture {
    pub fn new() -> Self {
        Self::named("project with spaces &")
    }

    pub fn named(name: &str) -> Self {
        Self::from_fixture("local_graph", name)
    }

    pub fn enums() -> Self {
        let fixture = Self::new();
        fixture.write("models/src/State.co", "enum { Ready, ready, _Done }\n");
        fixture.write("models/src/StateReader.co", "State Value;\nStateReader(State value) { Value = value; }\nfunc Read(): State { return Value; }\nstatic final State Initial = State.ready;\n");
        fixture.write("app/src/Main.co", "import models::State as JobState;\nimport models::StateReader;\nstatic func Main() {\n  JobState[] values = [JobState.Ready, StateReader.Initial, JobState._Done];\n  for (var value in values) { println(value); }\n  StateReader reader = StateReader(JobState._Done);\n  println(reader.Read() == JobState._Done);\n  println(reader.Read()::typeName);\n}\n");
        fixture
    }

    pub fn constants() -> Self {
        let fixture = Self::new();
        fixture.write(
            "core/src/Seed.co",
            r"
static final int16 Base = 3 * 7;
static final float32 Half = 1.0 / 2.0;
",
        );
        fixture.write("models/src/State.co", "enum { Ready, Done }\n");
        fixture.write(
            "models/src/Constants.co",
            r"
import foundation::Seed as Input;
static final int32 hidden = Input.Base;
static final int64 Answer = hidden * 2;
static final int8 Minimum = int8(-128);
static final int64 SignedMinimum = -9223372036854775808;
static final uint64 Maximum = uint64(0) - uint64(0) | 18446744073709551615;
static final State initial = State.Ready;
static final State Initial = initial;
static final float32 Quarter = Input.Half * Input.Half;
static final float64 Wide = Quarter + 1.0;
static final float32 Zero = -0.0;
static final float32 Tiny = 0.000000000000000000000000000000000000000000001;
static final bool Enabled = false && (1 / 0 == 0) || Answer == 42;
static final char Letter = 'Q';
",
        );
        fixture.write(
            "app/src/Main.co",
            r#"
import models::Constants as Values;
import models::State as Status;
static final int64 Answer = Values.Answer;
static final float32 Copy = Values.Tiny + 0.0;
static func Main() {
  println(Answer);
  println(Values.Minimum);
  println(Values.SignedMinimum);
  println(Values.Maximum);
  println(Values.Quarter == 0.25);
  println(Values.Wide == 1.25);
  println(Values.Enabled);
  println(Values.Letter);
  println(Values.Tiny > 0.0);
  println(Copy == Values.Tiny);
  println(Values.Zero == 0.0);
  switch (Values.SignedMinimum) { case Values.SignedMinimum: { println("minimum"); } }
  switch (Values.Maximum) { case Values.Maximum: { println("maximum"); } }
  switch (Values.Initial) {
    case Values.Initial: { println("ready"); }
    case Status.Done: { println("done"); }
  }
}
"#,
        );
        fixture
    }

    pub fn switches() -> Self {
        let fixture = Self::new();
        fixture.write_switch_model(SWITCH_CASES, "ready", 7);
        fixture.write("models/src/Other.co", "enum { Ready, ready, _Done }\n");
        fixture.write("app/src/Main.co", SWITCH_MAIN);
        fixture
    }

    pub fn write_switch_model(&self, cases: &[&str], initial: &str, small: u8) {
        self.write(
            "models/src/State.co",
            &format!("enum {{ {} }}\n", cases.join(", ")),
        );
        let values = cases
            .iter()
            .map(|name| format!("State.{name}"))
            .collect::<Vec<_>>()
            .join(", ");
        self.write(
            "models/src/StateReader.co",
            &format!(
                r#"
static final State Initial = State.{initial};
static final int8 Small = {small};
static final int32 hidden = 1;
static final uint64 Maximum = 18446744073709551615;
static func Values(): State[] {{ return [{values}]; }}
static func Name(State value): string {{
  switch (value) {{
    case State.Ready, State.ready: {{ return "active"; }}
    default: {{ return "done"; }}
  }}
}}
"#
            ),
        );
    }

    pub fn reverse_dependencies(&self) {
        let path = self.root.join("app/Shuttle.toml");
        let source = fs::read_to_string(path)
            .expect("fixture manifest")
            .replace("\r\n", "\n");
        let reversed = source.replace(
            "models = { path = \"../models\" }\ntools = { path = \"../tools\" }",
            "tools = { path = \"../tools\" }\nmodels = { path = \"../models\" }",
        );
        assert_ne!(source, reversed, "fixture dependency order did not change");
        self.write("app/Shuttle.toml", &reversed);
    }

    pub fn from_fixture(fixture: &str, name: &str) -> Self {
        let directory = TempDir::new().expect("create fixture directory");
        let root = directory.path().join(name);
        copy_tree(
            &Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures")
                .join(fixture),
            &root,
        );
        Self {
            _directory: directory,
            root,
        }
    }

    pub fn structs() -> Self {
        let fixture = Self::new();
        fixture.write(
            "models/src/Data.co",
            r#"
struct {
  byte tag = 1;
  string Text;
  uint64 CountTo;
  Data(string text, uint64 countTo) { Text = text; CountTo = countTo; }
  _Data(uint64 countTo) { Text = "private"; CountTo = countTo; }
  func Copy(): Data { return self; }
  func hidden(): uint64 { return CountTo; }
  static func Change(Data data): Data { data.CountTo += 1; return data; }
}
"#,
        );
        fixture.write(
            "models/src/Packet.co",
            r#"
struct {
  Data Value;
  string Tail;
  Packet(Data value) { Value = value; Tail = "tail"; }
  func Copy(): Packet { return self; }
  static func Score(Data value): uint64 { return value.CountTo; }
  static func Score(Packet value): uint64 { return value.Value.CountTo + 1; }
}
"#,
        );
        fixture.write(
            "models/src/Transformer.co",
            "interface { func Transform(Data value): Packet; }\n",
        );
        fixture.write(
            "models/src/Processor.co",
            r"
class is Transformer {
  Packet Initial;
  Processor(Packet initial) { Initial = initial; }
  override func Transform(Data value): Packet { return Packet(Data.Change(value)); }
}
",
        );
        fixture.write(
            "app/src/DerivedProcessor.co",
            r"
import models::Data;
import models::Packet;
import models::Processor;
class : Processor {
  DerivedProcessor(Packet initial): Processor(initial) {}
  override func Transform(Data value): Packet {
    var result = super.Transform(value);
    result.Value.CountTo += 3;
    return result;
  }
}
",
        );
        fixture.write(
            "app/src/Main.co",
            r#"
import models::Data;
import models::Packet;
import models::Transformer;
static func Main() {
  var data = Data("alive", 100);
  var changed = Data.Change(data);
  var packet = Packet(data);
  var copied = packet.Copy();
  Packet[] values = [packet, copied];
  values[0].Value.CountTo++;
  for (var item in values) { item.Value.CountTo = 0; }
  for (int32 i = 0; i < 10000; i++) { var garbage = "unused" + " allocation"; }
  println(data.CountTo);
  println(changed.CountTo);
  println(packet == copied);
  println(values[0].Value.CountTo);
  println(values[1].Value.Text);
  println(packet);
  println(copied::typeName);
  println(Packet.Score(data));
  println(Packet.Score(packet));
  var processor = DerivedProcessor(Packet(Data("initial", 0)));
  Transformer transformer = processor;
  println(transformer.Transform(data).Value.CountTo);
  println(processor.Initial.Value.Text);
}
"#,
        );
        fixture
    }

    pub fn checked_updates() -> Self {
        let fixture = Self::new();
        fixture.write(
            "models/src/User.co",
            r"
import foundation.data::Record;
int32 Calls;
int32 Value;
User(int32 value) { Calls = 0; Value = value; }
func Target(): User { Calls++; return self; }
static func Step(int32 value): int32 {
  value++;
  value += Record.Value();
  value *= 2;
  value -= 2;
  value /= 2;
  value %= 23;
  return value;
}
",
        );
        fixture.write(
            "tools/src/Helper.co",
            r"
import base.data::Record;
static func Adjust(int32 value): int32 {
  value--;
  value += Record.Value();
  return value;
}
",
        );
        fixture.write(
            "app/src/Main.co",
            r"
import models::User as ModelUser;
import tools::Helper;
static func Main() {
  println(ModelUser.Step(10));
  println(Helper.Adjust(3));
  ModelUser user = ModelUser(7);
  user.Target().Value *= 3;
  println(user.Calls);
  println(user.Value);
  int32 local = 40;
  println(++local + local++ - 40);
}
",
        );
        fixture
    }

    pub fn integer_conversions() -> Self {
        let fixture = Self::new();
        fixture.write(
            "core/src/data/Record.co",
            r"
static final int8 Wrapped = int8::wrap(300);
static final uint8 Limited = uint8::sat(999);
static func Saturate(int64 value): uint16 { return uint16::sat(value); }
static func Main(): int32 { return 98; }
",
        );
        fixture.write(
            "models/src/User.co",
            r"
import foundation.data::Record;
static final int8 Constant = Record.Wrapped;
static func Convert(int64 value): uint16 { return uint16::wrap(value); }
static func Foundation(int64 value): uint16 { return Record.Saturate(value); }
",
        );
        fixture.write(
            "tools/src/Helper.co",
            r"
import base.data::Record;
static func Adjust(int64 value): uint8 { return uint8::sat(value); }
static func Constant(): uint8 { return Record.Limited; }
",
        );
        fixture.write(
            "app/src/Main.co",
            r"
import models::User as ModelUser;
import tools::Helper;
static func Main() {
  println(ModelUser.Convert(70000));
  println(ModelUser.Foundation(70000));
  println(Helper.Adjust(-5));
  println(ModelUser.Constant);
  println(Helper.Constant());
}
",
        );
        fixture
    }

    pub fn typed_literals() -> Self {
        let fixture = Self::new();
        fixture.write(
            "core/src/data/Record.co",
            r"
static final int8 Small = 7i8;
static final uint64 Maximum = 18446744073709551615u64;
static final float32 Half = 0.5f32;
static func Main(): int32 { return 98; }
",
        );
        fixture.write(
            "models/src/User.co",
            r"
import foundation.data::Record;
static final int64 Wide = Record.Small;
static final uint64 Maximum = Record.Maximum;
static final float64 Half = Record.Half;
static func Add(int64 value): int64 { return value + 2i8; }
static func Wrapped(): int8 { return int8::wrap(300i16); }
",
        );
        fixture.write(
            "tools/src/Helper.co",
            r"
static func Saturated(): uint8 { return uint8::sat(-1i16); }
",
        );
        fixture.write(
            "app/src/Main.co",
            r"
import models::User as Values;
import tools::Helper;
static func Which(int8 value): int32 { return 8; }
static func Which(int64 value): int32 { return 64; }
static func Main() {
  println(Values.Wide);
  println(Values.Add(40i8));
  println(Values.Maximum);
  println(Values.Half);
  println(Values.Wrapped());
  println(Helper.Saturated());
  println(Which(1i8));
}
",
        );
        fixture
    }

    pub fn numeric_notation() -> Self {
        let fixture = Self::new();
        fixture.write(
            "core/src/data/Record.co",
            r"
static final uint8 Mask = 0b1111_0000u8;
static final uint64 Maximum = 0xFFFF_FFFF_FFFF_FFFFu64;
static final float32 Scale = 1.25e2f32;
static func Main(): int32 { return 98; }
",
        );
        fixture.write(
            "models/src/User.co",
            r"
import foundation.data::Record;
static final int64 Wide = Record.Mask;
static final uint64 Maximum = Record.Maximum;
static final float64 Scale = Record.Scale;
static func Add(int64 value): int64 { return value + 0o2i8; }
",
        );
        fixture.write(
            "tools/src/Helper.co",
            r"
static func Wrapped(): int8 { return int8::wrap(0x12Cu16); }
static func Saturated(): uint8 { return uint8::sat(-0b1i16); }
",
        );
        fixture.write(
            "app/src/Main.co",
            r"
import models::User as Values;
import tools::Helper;
static func Which(int8 value): int32 { return 8; }
static func Which(int64 value): int32 { return 64; }
static func Main() {
  println(Values.Wide);
  println(Values.Add(40));
  println(Values.Maximum);
  println(Values.Scale);
  println(Helper.Wrapped());
  println(Helper.Saturated());
  println(Which(0b1i8));
}
",
        );
        fixture
    }

    pub fn typed_errors() -> Self {
        let fixture = Self::new();
        let manifest =
            fs::read_to_string(fixture.root.join("app/Shuttle.toml")).expect("app manifest");
        fixture.write(
            "app/Shuttle.toml",
            &manifest.replace(
                "tools = { path = \"../tools\" }",
                "tools = { path = \"../tools\" }\nfoundation = { path = \"../core\" }",
            ),
        );
        fixture.write(
            "core/src/InvalidInput.co",
            r"
error {
  InvalidInput(string message): Error(message) {}
}
",
        );
        fixture.write(
            "models/src/Calculator.co",
            r#"
import foundation::InvalidInput;
static func Divide(int32 left, int32 right): int32 throws InvalidInput, DivisionByZero {
  if (left < 0) { throw InvalidInput("negative"); }
  return left / right;
}
"#,
        );
        fixture.write(
            "app/src/Main.co",
            r"
import foundation::InvalidInput;
import models::Calculator;
static func Main(): int32 throws InvalidInput, DivisionByZero {
  println(Calculator.Divide(42, 6));
  return 0;
}
",
        );
        fixture
    }

    pub fn manifest(&self) -> PathBuf {
        self.root.join("app/Shuttle.toml")
    }

    pub fn artifact_bytes(&self, relative: &str) -> BTreeMap<String, Vec<u8>> {
        let directory = self.root.join(relative);
        ["app", "data-models", "foundation", "tools"]
            .into_iter()
            .map(|package| {
                let path = directory.join(format!("{package}.cpa"));
                (
                    package.to_owned(),
                    fs::read(path).expect("package artifact"),
                )
            })
            .collect()
    }

    pub fn edit_struct(&self, layout: bool) {
        let source =
            fs::read_to_string(self.root.join("models/src/Data.co")).expect("struct source");
        let changed = if layout {
            source.replace("byte tag = 1;", "uint64 padding = 123;\n  byte tag = 1;")
        } else {
            source.replace(
                "func hidden(): uint64",
                "func Added(): uint64 { return CountTo; }\n  func hidden(): uint64",
            )
        };
        assert_ne!(source, changed, "fixture edit did not change the struct");
        self.write("models/src/Data.co", &changed);
    }

    pub fn write(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        fs::create_dir_all(path.parent().expect("file parent")).expect("create directory");
        fs::write(path, contents).expect("write fixture file");
    }

    pub fn shuttle(&self, command: &str, compiler: &Path) -> Command {
        let mut process = self.visible_shuttle(command, compiler);
        process.arg("--quiet");
        process
    }

    pub fn visible_shuttle(&self, command: &str, compiler: &Path) -> Command {
        let mut process = Command::new(env!("CARGO_BIN_EXE_shuttle"));
        process
            .current_dir(&self.root)
            .arg(command)
            .arg("--manifest-path")
            .arg(self.manifest())
            .arg("--compiler")
            .arg(compiler);
        process
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create fixture copy");
    for entry in fs::read_dir(source).expect("read fixture") {
        let entry = entry.expect("fixture entry");
        let target = destination.join(entry.file_name());
        if entry.file_type().expect("fixture type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).expect("copy fixture file");
        }
    }
}

// Bound every child, and drain both streams concurrently so diagnostics cannot
// fill a pipe and hide a compiler hang. Never mutate the parent environment.
pub fn run(command: &mut Command) -> Output {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start test process");
    let mut stdout = child.stdout.take().expect("stdout pipe");
    let mut stderr = child.stderr.take().expect("stderr pipe");
    let out = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).expect("read stdout");
        bytes
    });
    let err = thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).expect("read stderr");
        bytes
    });
    let start = Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait().expect("poll child") {
            break status;
        }
        if start.elapsed() > Duration::from_secs(300) {
            child.kill().expect("terminate timed-out child");
            child.wait().expect("reap timed-out child");
            panic!("process exceeded 300 seconds: {command:?}");
        }
        thread::sleep(Duration::from_millis(10));
    };
    Output {
        status,
        stdout: out.join().expect("stdout reader"),
        stderr: err.join().expect("stderr reader"),
    }
}

pub fn expect_status(output: &Output, status: i32) {
    assert_eq!(
        output.status.code(),
        Some(status),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

pub fn compiler() -> PathBuf {
    let path = PathBuf::from(
        std::env::var_os("CLOTHC_UNDER_TEST")
            .expect("set CLOTHC_UNDER_TEST to an absolute clothc path for cross-tool tests"),
    );
    assert!(
        path.is_absolute() && path.is_file(),
        "invalid CLOTHC_UNDER_TEST: {}",
        path.display()
    );
    path
}
