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

impl Fixture {
    pub fn new() -> Self {
        Self::named("project with spaces &")
    }

    pub fn named(name: &str) -> Self {
        Self::from_fixture("local_graph", name)
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

    pub fn manifest(&self) -> PathBuf {
        self.root.join("app/Shuttle.toml")
    }

    pub fn write(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        fs::create_dir_all(path.parent().expect("file parent")).expect("create directory");
        fs::write(path, contents).expect("write fixture file");
    }

    pub fn shuttle(&self, command: &str, compiler: &Path) -> Command {
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
