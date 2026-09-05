//! A real child process used only by the process-contract tests.
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

const COMPILER_ID: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const ARTIFACT_ID: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn main() {
    let arguments: Vec<_> = env::args_os().skip(1).collect();
    let query = arguments
        .first()
        .is_some_and(|value| value == "--shuttle-protocol-capabilities");
    let operation = option(&arguments, "--operation");
    let phase = if query {
        "query"
    } else if arguments.is_empty() {
        "run"
    } else {
        operation.as_deref().expect("operation")
    };
    let log_path = PathBuf::from(env::var_os("SHUTTLE_STUB_LOG").expect("stub log"));
    let mut log = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&log_path)
        .expect("open log");
    let detail = if matches!(phase, "compile" | "reuse") {
        package_name(&arguments).unwrap_or_default()
    } else {
        String::new()
    };
    writeln!(
        log,
        "{phase}:{detail}:{}",
        env::current_exe().expect("stub path").display()
    )
    .expect("write log");
    let mode = env::var("SHUTTLE_STUB_MODE").unwrap_or_default();
    if query {
        let capabilities = format!(
            "{{\"schema\":1,\"protocols\":[1,2],\"artifact_formats\":[5],\"compiler_id\":\"{COMPILER_ID}\",\"operations\":[\"compile\",\"inspect\",\"link\",\"reuse\"],\"interface_targets\":[\"wasm32\",\"x86_64\"],\"object_targets\":[\"x86_64\"]}}"
        );
        match mode.as_str() {
            "query-version" => println!("{{\"schema\":1,\"protocols\":[1]}}"),
            "query-old-artifact-format" => println!(
                "{}",
                capabilities.replace("\"artifact_formats\":[5]", "\"artifact_formats\":[4]")
            ),
            "query-no-newline" => print!("{capabilities}"),
            "query-stderr" => {
                println!("{capabilities}");
                eprintln!("unexpected diagnostic");
            }
            "query-exit" => std::process::exit(1),
            _ => println!("{capabilities}"),
        }
        return;
    }
    if phase == "run" {
        println!("program");
        eprintln!("program stderr");
        std::process::exit(7);
    }
    if phase == "compile" {
        if mode == "parallel-barrier" && matches!(detail.as_str(), "data-models" | "tools") {
            parallel_barrier(&log_path, &detail);
        } else if mode == "parallel-failure" && matches!(detail.as_str(), "data-models" | "tools") {
            if detail == "data-models" {
                thread::sleep(Duration::from_millis(200));
            }
            eprintln!("{detail}.co:3:5: error: parallel stub rejection");
            std::process::exit(1);
        } else if mode == "compile-wait" {
            thread::sleep(Duration::from_millis(500));
        } else if let Some(code) = mode.strip_prefix("compile-") {
            eprintln!("fixture.co:3:5: error: stub rejection");
            std::process::exit(code.parse().expect("stub exit code"));
        }
    }
    if phase == "compile" {
        compile(&arguments);
    } else if phase == "reuse" {
        if mode == "reuse-miss" {
            std::process::exit(3);
        }
        emit_receipt(&arguments);
    } else if phase == "link" {
        link(&arguments);
    }
}

fn parallel_barrier(log_path: &Path, package: &str) {
    let directory = log_path.parent().expect("stub log parent");
    fs::write(directory.join(format!("{package}.ready")), b"ready").expect("write ready marker");
    let peer = if package == "data-models" {
        "tools"
    } else {
        "data-models"
    };
    for _ in 0..1_000 {
        if directory.join(format!("{peer}.ready")).is_file() {
            return;
        }
        thread::sleep(Duration::from_millis(10));
    }
    eprintln!("parallel compiler barrier timed out waiting for {peer}");
    std::process::exit(42);
}

fn compile(arguments: &[std::ffi::OsString]) {
    let output = PathBuf::from(option(arguments, "--output").expect("output"));
    fs::write(&output, b"stub artifact").expect("create artifact");
    emit_receipt(arguments);
}

fn emit_receipt(arguments: &[std::ffi::OsString]) {
    let package_index = arguments
        .iter()
        .position(|argument| argument == "--package")
        .expect("package");
    let package = arguments[package_index + 1].to_string_lossy();
    let version = arguments[package_index + 2].to_string_lossy();
    let target = option(arguments, "--target").expect("target");
    let kind = option(arguments, "--artifact-kind").expect("kind");
    let mut dependencies = Vec::new();
    for (index, argument) in arguments.iter().enumerate() {
        if argument == "--dependency" {
            let alias = arguments[index + 1].to_string_lossy();
            let target_package = arguments[index + 2].to_string_lossy();
            let version =
                artifact_version(arguments, &target_package).expect("dependency artifact version");
            dependencies.push(format!(
                "{{\"alias\":\"{alias}\",\"package\":{{\"name\":\"{target_package}\",\"version\":\"{version}\"}},\"artifact_id\":\"{ARTIFACT_ID}\"}}"
            ));
        }
    }
    let reported_package =
        if env::var("SHUTTLE_STUB_MODE").as_deref() == Ok("receipt-wrong-package") {
            "wrong"
        } else {
            package.as_ref()
        };
    let artifact_id = if env::var("SHUTTLE_STUB_MODE").as_deref() == Ok("receipt-bad-digest") {
        "invalid"
    } else {
        ARTIFACT_ID
    };
    let receipt = format!(
        "{{\"schema\":1,\"artifact_format\":5,\"artifact_id\":\"{ARTIFACT_ID}\",\"kind\":\"{kind}\",\"package\":{{\"name\":\"{package}\",\"version\":\"{version}\"}},\"target\":\"{target}\",\"compiler_id\":\"{COMPILER_ID}\",\"dependencies\":[{}]}}",
        dependencies.join(",")
    );
    let receipt = receipt.replace(ARTIFACT_ID, artifact_id).replace(
        &format!("\"name\":\"{package}\""),
        &format!("\"name\":\"{reported_package}\""),
    );
    match env::var("SHUTTLE_STUB_MODE").as_deref() {
        Ok("receipt-no-newline") => print!("{receipt}"),
        Ok("receipt-old-artifact-format") => println!(
            "{}",
            receipt.replace("\"artifact_format\":5", "\"artifact_format\":4")
        ),
        Ok("receipt-trailing") => print!("{receipt}\ntrailing\n"),
        _ => println!("{receipt}"),
    }
}

fn artifact_version(arguments: &[std::ffi::OsString], package: &str) -> Option<String> {
    arguments.windows(5).find_map(|record| {
        (record[0] == "--artifact" && record[1] == package)
            .then(|| record[2].to_string_lossy().into_owned())
    })
}

fn link(arguments: &[std::ffi::OsString]) {
    if let Some(output) = option(arguments, "--output") {
        fs::copy(
            env::current_exe().expect("stub executable"),
            Path::new(&output),
        )
        .expect("create fake executable");
    }
}

fn option(arguments: &[std::ffi::OsString], name: &str) -> Option<String> {
    arguments
        .iter()
        .position(|argument| argument == name)
        .and_then(|index| arguments.get(index + 1))
        .map(|value| value.to_string_lossy().into_owned())
}

fn package_name(arguments: &[std::ffi::OsString]) -> Option<String> {
    option(arguments, "--package")
}
