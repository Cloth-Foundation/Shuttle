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
