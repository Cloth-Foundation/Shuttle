#[allow(dead_code)]
mod support;

// Keep the standalone child fixture under Cargo's formatting/lint/MSRV gates.
#[allow(dead_code)]
#[path = "support/compiler_stub.rs"]
mod compiler_stub;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use support::{Fixture, expect_status, run};

fn stub(fixture: &Fixture) -> PathBuf {
    let executable = fixture
        .root
        .join(format!("compiler stub{}", std::env::consts::EXE_SUFFIX));
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/support/compiler_stub.rs");
    let output = run(Command::new("rustc")
        .args(["--edition=2024", "-Dwarnings"])
        .arg(source)
        .arg("-o")
        .arg(&executable));
    expect_status(&output, 0);
    executable
}

fn command(fixture: &Fixture, compiler: &Path, action: &str, mode: &str) -> Command {
    let mut command = fixture.shuttle(action, compiler);
    command
        .env("SHUTTLE_STUB_LOG", fixture.root.join("calls.log"))
        .env("SHUTTLE_STUB_MODE", mode);
    command
}

fn phases(fixture: &Fixture) -> Vec<String> {
    fs::read_to_string(fixture.root.join("calls.log"))
        .expect("calls log")
        .lines()
        .map(|line| line.split(':').next().expect("phase").to_owned())
        .collect()
}

#[test]
fn invalid_graphs_fail_before_even_querying_the_compiler() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    for manifest in [
        "manifest-version = 2\n[package]\nname = \"app\"\nversion = \"0.1.0\"\n",
        "manifest-version = 1\n[package]\nname = \"app\"\nversion = \"0.1.0\"\n[dependencies]\nmissing = { path = \"../missing\" }\n",
        "manifest-version = 1\n[package]\nname = \"app\"\nversion = \"0.1.0\"\n[dependencies]\nself = { path = \"../app\" }\n",
    ] {
        fixture.write("app/Shuttle.toml", manifest);
        let output = run(&mut command(&fixture, &compiler, "check", ""));
        expect_status(&output, 1);
        assert!(output.stdout.is_empty());
        assert!(!fixture.root.join("calls.log").exists());
    }
}

#[test]
fn rejects_incompatible_queries_without_compilation_or_output_creation() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    for mode in [
        "query-version",
        "query-no-newline",
        "query-stderr",
        "query-exit",
    ] {
        let output = run(&mut command(&fixture, &compiler, "build", mode));
        expect_status(&output, 2);
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8(output.stderr)
                .expect("diagnostic")
                .contains("does not support Shuttle protocol")
        );
        assert!(phases(&fixture).iter().all(|phase| phase == "query"));
        assert!(!fixture.root.join("app/target").exists());
    }
}

#[test]
fn preserves_compiler_failures_and_reports_abnormal_status_with_context() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    for (mode, code) in [("compile-1", 1), ("compile-2", 2), ("compile-42", 2)] {
        let output = run(&mut command(&fixture, &compiler, "run", mode));
        expect_status(&output, code);
        assert!(output.stdout.is_empty());
        let error = String::from_utf8(output.stderr).expect("diagnostic");
        assert!(error.starts_with("fixture.co:3:5: error: stub rejection"));
        if mode == "compile-42" {
            assert!(
                error.contains("app") && error.contains("compiler stub"),
                "{error}"
            );
            assert!(error.contains("42"));
        } else {
            assert_eq!(
                error.replace("\r\n", "\n"),
                "fixture.co:3:5: error: stub rejection\n"
            );
        }
        assert!(!phases(&fixture).iter().any(|phase| phase == "run"));
    }
}

#[test]
fn runs_only_after_success_and_forwards_program_streams_and_status() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    let output = run(&mut command(&fixture, &compiler, "run", ""));
    expect_status(&output, 7);
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout").trim(),
        "program"
    );
    assert_eq!(
        String::from_utf8(output.stderr).expect("stderr").trim(),
        "program stderr"
    );
    assert_eq!(phases(&fixture), ["query", "compile", "run"]);
}

#[test]
fn compiler_precedence_is_explicit_then_sibling_then_path() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    let bin = fixture.root.join("bin");
    let search = fixture.root.join("search");
    fs::create_dir(&bin).expect("bin directory");
    fs::create_dir(&search).expect("search directory");
    let shuttle = bin.join(format!("shuttle{}", std::env::consts::EXE_SUFFIX));
    let sibling = bin.join(format!("clothc{}", std::env::consts::EXE_SUFFIX));
    let fallback = search.join(format!("clothc{}", std::env::consts::EXE_SUFFIX));
    fs::copy(env!("CARGO_BIN_EXE_shuttle"), &shuttle).expect("copy Shuttle");
    fs::copy(&compiler, &sibling).expect("copy sibling compiler");
    fs::copy(&compiler, &fallback).expect("copy PATH compiler");
    for (explicit, expected) in [(true, &compiler), (false, &sibling), (false, &fallback)] {
        if expected == &fallback {
            fs::remove_file(&sibling).expect("remove test sibling");
        }
        let mut child = Command::new(&shuttle);
        child
            .arg("check")
            .arg("--manifest-path")
            .arg(fixture.manifest())
            .env("PATH", &search)
            .env("SHUTTLE_STUB_LOG", fixture.root.join("calls.log"));
        if explicit {
            child.arg("--compiler").arg(&compiler);
        }
        expect_status(&run(&mut child), 0);
        let log = fs::read_to_string(fixture.root.join("calls.log")).expect("log");
        assert!(
            log.lines()
                .last()
                .expect("last call")
                .ends_with(&expected.display().to_string()),
            "{log}"
        );
    }
    let output = run(&mut command(
        &fixture,
        &fixture.root.join("absent"),
        "check",
        "",
    ));
    expect_status(&output, 2);
    assert!(
        String::from_utf8(output.stderr)
            .expect("error")
            .contains("invalid compiler")
    );
}
