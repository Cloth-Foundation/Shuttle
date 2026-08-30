use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

fn shuttle(project: &Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_shuttle"))
        .args(arguments)
        .current_dir(project)
        .output()
        .expect("run Shuttle")
}

fn project(manifest: &str) -> TempDir {
    let project = TempDir::new().expect("create temporary project");
    fs::create_dir(project.path().join("src")).expect("create source root");
    fs::write(project.path().join("src/Main.co"), "static Main() {}\n").expect("write Cloth entry");
    fs::write(project.path().join("Shuttle.toml"), manifest).expect("write manifest");
    project
}

#[test]
fn help_exposes_only_the_stage_twenty_two_commands() {
    let project = TempDir::new().expect("create temporary directory");
    let output = shuttle(project.path(), &["--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 standard output");
    for command in ["check", "build", "run"] {
        assert!(stdout.contains(command), "help omits {command}");
    }
    for unavailable in ["new", "init", "test", "clean"] {
        assert!(!stdout.contains(&format!("  {unavailable}")));
    }
}

#[test]
fn invalid_manifest_fails_before_command_execution() {
    let project = project(
        r#"manifest-version = 1
unsupported = true

[package]
name = "hello"
version = "0.1.0"
"#,
    );
    let output = shuttle(project.path(), &["check"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 standard error");
    assert!(stderr.contains("Shuttle.toml:2:"));
    assert!(stderr.contains("unknown field `unsupported`"));
}

#[test]
fn valid_manifest_reaches_the_deliberate_stage_boundary() {
    let project = project(
        r#"manifest-version = 1

[package]
name = "hello-world"
version = "0.1.0"

[executable]
entry = "Main.co"
"#,
    );
    let output = shuttle(
        project.path(),
        &["build", "--target", "wasm32", "--compiler", "tools/clothc"],
    );

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 standard error");
    assert!(stderr.contains("package 'hello-world'"));
    assert!(stderr.contains("target 'wasm32'"));
    assert!(stderr.contains("unavailable until Stage 22.3"));
    assert!(stderr.contains("tools/clothc"));
}

#[test]
fn explicit_manifest_path_is_resolved_from_the_working_directory() {
    let project = project(
        r#"manifest-version = 1

[package]
name = "hello"
version = "0.1.0"
"#,
    );
    let nested = project.path().join("nested");
    fs::create_dir(&nested).expect("create nested directory");
    let output = shuttle(&nested, &["check", "--manifest-path", "../Shuttle.toml"]);

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 standard error");
    assert!(stderr.contains("package 'hello'"));
}

#[test]
fn unsupported_targets_are_rejected_by_the_cli() {
    let project = TempDir::new().expect("create temporary directory");
    let output = shuttle(project.path(), &["check", "--target", "arm64"]);

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).expect("UTF-8 standard error");
    assert!(stderr.contains("invalid value 'arm64'"));
    assert!(stderr.contains("x86_64"));
    assert!(stderr.contains("wasm32"));
}
