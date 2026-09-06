// Part of the Cloth Compiler project, under the Apache License v2.0 with LLVM
// Exceptions. See LICENSE.txt in the project root for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#[allow(dead_code)]
mod support;

// Keep the standalone child fixture under Cargo's formatting/lint/MSRV gates.
#[allow(dead_code)]
#[path = "support/compiler_stub.rs"]
mod compiler_stub;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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
    write_stub_toolchain(&executable);
    executable
}

fn write_stub_toolchain(compiler: &Path) {
    let directory = compiler.parent().expect("compiler directory");
    let library = directory.join("standard-library");
    fs::create_dir_all(library.join("src/math")).expect("standard library source directory");
    fs::write(
        library.join("Shuttle.toml"),
        "manifest-version = 1\n\n[package]\nname = \"cloth\"\nversion = \"0.1.0\"\nsource-root = \"src\"\n",
    )
    .expect("standard library manifest");
    fs::write(library.join("src/math/Math.co"), "class {}\n").expect("standard library source");
    fs::write(
        directory.join("cloth-toolchain.json"),
        "{\"schema\":1,\"standard_library\":{\"package\":\"cloth\",\"version\":\"0.1.0\",\"manifest\":\"standard-library/Shuttle.toml\"}}\n",
    )
    .expect("toolchain metadata");
}

fn command(fixture: &Fixture, compiler: &Path, action: &str, mode: &str) -> Command {
    configured_command(fixture, compiler, action, mode, false, 1)
}

fn visible_command(fixture: &Fixture, compiler: &Path, action: &str, mode: &str) -> Command {
    configured_command(fixture, compiler, action, mode, true, 1)
}

fn configured_command(
    fixture: &Fixture,
    compiler: &Path,
    action: &str,
    mode: &str,
    visible: bool,
    jobs: usize,
) -> Command {
    let mut command = fixture.visible_shuttle(action, compiler);
    if !visible {
        command.arg("--quiet");
    }
    command
        .args(["--jobs", &jobs.to_string()])
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

fn compiled_packages(fixture: &Fixture) -> Vec<String> {
    fs::read_to_string(fixture.root.join("calls.log"))
        .expect("calls log")
        .lines()
        .filter_map(|line| line.strip_prefix("compile:"))
        .map(|line| line.split(':').next().expect("package").to_owned())
        .collect()
}

fn package_artifacts(fixture: &Fixture) -> Vec<Vec<u8>> {
    ["cloth", "foundation", "data-models", "tools", "app"]
        .map(|package| {
            fs::read(
                fixture
                    .root
                    .join(format!("app/target/x86_64/packages/{package}.cpa")),
            )
            .expect("package artifact")
        })
        .into()
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
fn reserves_the_implicit_standard_library_dependency_alias() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    for alias in ["cloth", "Cloth", "CLOTH"] {
        fixture.write(
            "app/Shuttle.toml",
            &format!(
                "manifest-version = 1\n[package]\nname = \"app\"\nversion = \"0.1.0\"\n[dependencies]\n{alias} = {{ path = \"../core\" }}\n"
            ),
        );
        let output = run(&mut command(&fixture, &compiler, "check", ""));
        expect_status(&output, 1);
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8(output.stderr)
                .expect("diagnostic")
                .contains("reserved for the compiler-paired standard library")
        );
    }
    assert!(!fixture.root.join("calls.log").exists());
}

#[test]
fn rejects_missing_metadata_and_standard_library_replacements() {
    let missing = Fixture::new();
    let missing_compiler = stub(&missing);
    fs::remove_file(missing.root.join("cloth-toolchain.json")).expect("remove metadata");
    let output = run(&mut command(&missing, &missing_compiler, "check", ""));
    expect_status(&output, 2);
    assert!(
        String::from_utf8(output.stderr)
            .expect("metadata diagnostic")
            .contains("toolchain metadata")
    );
    assert_eq!(phases(&missing), ["query"]);

    let replacement = Fixture::new();
    let replacement_compiler = stub(&replacement);
    let manifest = replacement.root.join("core/Shuttle.toml");
    let contents = fs::read_to_string(&manifest)
        .expect("foundation manifest")
        .replace("name = \"foundation\"", "name = \"cloth\"");
    fs::write(manifest, contents).expect("replace package identity");
    let output = run(&mut command(
        &replacement,
        &replacement_compiler,
        "check",
        "",
    ));
    expect_status(&output, 2);
    assert!(
        String::from_utf8(output.stderr)
            .expect("replacement diagnostic")
            .contains("reserved for the compiler-paired standard library")
    );
    assert_eq!(phases(&replacement), ["query"]);
}

#[test]
fn rejects_malformed_or_incompatible_toolchain_metadata() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    let metadata_path = fixture.root.join("cloth-toolchain.json");
    let cases = [
        ("not JSON\n", "invalid JSON"),
        (
            "{\"schema\":1,\"schema\":1,\"standard_library\":{\"package\":\"cloth\",\"version\":\"0.1.0\",\"manifest\":\"standard-library/Shuttle.toml\"}}\n",
            "invalid JSON",
        ),
        (
            "{\"schema\":2,\"standard_library\":{\"package\":\"cloth\",\"version\":\"0.1.0\",\"manifest\":\"standard-library/Shuttle.toml\"}}\n",
            "unsupported schema 2",
        ),
        (
            "{\"schema\":1,\"extra\":true,\"standard_library\":{\"package\":\"cloth\",\"version\":\"0.1.0\",\"manifest\":\"standard-library/Shuttle.toml\"}}\n",
            "invalid JSON",
        ),
        (
            "{\"schema\":1,\"standard_library\":{\"package\":\"cloth\",\"version\":\"0.3.0\",\"manifest\":\"standard-library/Shuttle.toml\"}}\n",
            "does not match the selected compiler",
        ),
        (
            "{\"schema\":1,\"standard_library\":{\"package\":\"cloth\",\"version\":\"0.1.0\",\"manifest\":\"standard-library/../Shuttle.toml\"}}\n",
            "normalized relative '/' path",
        ),
        (
            "{\"schema\":1,\"standard_library\":{\"package\":\"cloth\",\"version\":\"0.1.0\",\"manifest\":\"standard-library\\\\Shuttle.toml\"}}\n",
            "normalized relative '/' path",
        ),
        (
            "{\"schema\":1,\"standard_library\":{\"package\":\"cloth\",\"version\":\"0.1.0\",\"manifest\":\"standard-library/cloth.toml\"}}\n",
            "must name 'Shuttle.toml'",
        ),
        (
            "{\"schema\":1,\"standard_library\":{\"package\":\"cloth\",\"version\":\"0.1.0\",\"manifest\":\"missing/Shuttle.toml\"}}\n",
            "standard library distribution",
        ),
    ];
    for (metadata, expected) in cases {
        fs::write(&metadata_path, metadata).expect("replace toolchain metadata");
        let output = run(&mut command(&fixture, &compiler, "check", ""));
        expect_status(&output, 2);
        assert!(output.stdout.is_empty());
        let diagnostic = String::from_utf8(output.stderr).expect("toolchain diagnostic");
        assert!(
            diagnostic.contains(expected),
            "missing {expected:?} in {diagnostic:?}"
        );
    }
    assert!(phases(&fixture).iter().all(|phase| phase == "query"));
    assert!(!fixture.root.join("app/target").exists());
}

#[test]
fn rejects_invalid_standard_library_distributions() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    let manifest_path = fixture.root.join("standard-library/Shuttle.toml");
    fixture.write(
        "standard-library/other/Shuttle.toml",
        "manifest-version = 1\n[package]\nname = \"other\"\nversion = \"0.1.0\"\nsource-root = \"src\"\n",
    );
    fixture.write("standard-library/other/src/Other.co", "class {}\n");
    let cases = [
        (
            "manifest-version = 1\n[package]\nname = \"other\"\nversion = \"0.1.0\"\nsource-root = \"src\"\n",
            "must define package 'cloth'",
        ),
        (
            "manifest-version = 1\n[package]\nname = \"cloth\"\nversion = \"0.3.0\"\nsource-root = \"src\"\n",
            "does not match compiler version",
        ),
        (
            "manifest-version = 1\n[package]\nname = \"cloth\"\nversion = \"0.1.0\"\nsource-root = \"src\"\n[executable]\nentry = \"math/Math.co\"\n",
            "executable-free package 'cloth' with no dependencies",
        ),
        (
            "manifest-version = 1\n[package]\nname = \"cloth\"\nversion = \"0.1.0\"\nsource-root = \"src\"\n[dependencies]\nother = { path = \"other\" }\n",
            "executable-free package 'cloth' with no dependencies",
        ),
    ];
    for (manifest, expected) in cases {
        fs::write(&manifest_path, manifest).expect("replace standard library manifest");
        let output = run(&mut command(&fixture, &compiler, "check", ""));
        expect_status(&output, 2);
        assert!(output.stdout.is_empty());
        let diagnostic = String::from_utf8(output.stderr).expect("distribution diagnostic");
        assert!(
            diagnostic.contains(expected),
            "missing {expected:?} in {diagnostic:?}"
        );
    }
    assert!(phases(&fixture).iter().all(|phase| phase == "query"));
    assert!(!fixture.root.join("app/target").exists());
}

#[test]
fn rejects_incompatible_queries_without_compilation_or_output_creation() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    for mode in [
        "query-version",
        "query-old-artifact-format",
        "query-wrong-standard-library",
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
                .contains("capabilit")
        );
        assert!(phases(&fixture).iter().all(|phase| phase == "query"));
        assert!(!fixture.root.join("app/target").exists());
    }
}

#[test]
fn rejects_malformed_or_mismatched_compile_receipts() {
    for mode in [
        "receipt-no-newline",
        "receipt-trailing",
        "receipt-wrong-package",
        "receipt-bad-digest",
        "receipt-old-artifact-format",
    ] {
        let fixture = Fixture::new();
        let compiler = stub(&fixture);
        let output = run(&mut command(&fixture, &compiler, "check", mode));
        expect_status(&output, 2);
        assert!(output.stdout.is_empty());
        assert!(!phases(&fixture).iter().any(|phase| phase == "link"));
        let state = fixture.root.join("app/target/x86_64/check/.shuttle/state");
        let state_is_empty =
            fs::read_dir(state).map_or(true, |mut entries| entries.next().is_none());
        assert!(state_is_empty);
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
                error.contains("cloth") && error.contains("compiler stub"),
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
    assert_eq!(
        phases(&fixture),
        [
            "query", "compile", "compile", "compile", "compile", "compile", "link", "run"
        ]
    );
    assert_eq!(
        compiled_packages(&fixture),
        ["cloth", "foundation", "data-models", "tools", "app"]
    );
}

#[test]
fn forwards_program_arguments_after_the_explicit_boundary() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    let mut command = command(&fixture, &compiler, "run", "run-arguments");
    command
        .arg("--")
        .args(["", "two words", " ", "--quiet", "é🙂"]);
    let output = run(&mut command);
    expect_status(&output, 7);
    assert_eq!(
        String::from_utf8(output.stdout)
            .expect("program stdout")
            .replace("\r\n", "\n"),
        "5\n<>\n<two words>\n< >\n<--quiet>\n<é🙂>\n"
    );
    assert_eq!(
        String::from_utf8(output.stderr)
            .expect("program stderr")
            .replace("\r\n", "\n"),
        "program stderr\n"
    );
    assert_eq!(phases(&fixture).last().map(String::as_str), Some("run"));
}

#[test]
fn reports_build_progress_without_contaminating_program_output() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    let output = run(&mut visible_command(&fixture, &compiler, "run", ""));
    expect_status(&output, 7);
    assert_eq!(
        String::from_utf8(output.stdout)
            .expect("program stdout")
            .replace("\r\n", "\n"),
        "program\n"
    );
    let progress = String::from_utf8(output.stderr).expect("progress");
    let expected = [
        "shuttle: preparing build for x86_64 (5 packages)",
        "shuttle: compiling cloth v0.1.0 [1/5]",
        "shuttle: compiling foundation v1.0.0 [2/5]",
        "shuttle: compiling data-models v1.2.3-beta.1+local [3/5]",
        "shuttle: compiling tools v0.2.0 [4/5]",
        "shuttle: compiling app v0.1.0 [5/5]",
        "shuttle: linking app",
        "shuttle: finished build for x86_64 in ",
        "shuttle: running ",
        "program stderr",
    ];
    let mut previous = 0;
    for message in expected {
        let position = progress[previous..].find(message).map_or_else(
            || panic!("missing {message:?} in {progress:?}"),
            |position| previous + position,
        );
        previous = position + message.len();
    }
}

#[test]
fn bounds_parallel_ready_packages_and_preserves_canonical_results() {
    let serial = Fixture::named("serial project");
    let serial_compiler = stub(&serial);
    let mut serial_command = command(&serial, &serial_compiler, "build", "");
    expect_status(&run(&mut serial_command), 0);

    let parallel = Fixture::named("parallel project");
    let parallel_compiler = stub(&parallel);
    let mut parallel_command = configured_command(
        &parallel,
        &parallel_compiler,
        "build",
        "parallel-barrier",
        true,
        2,
    );
    let output = run(&mut parallel_command);
    expect_status(&output, 0);
    assert!(parallel.root.join("data-models.ready").is_file());
    assert!(parallel.root.join("tools.ready").is_file());
    assert_eq!(package_artifacts(&serial), package_artifacts(&parallel));

    let progress = String::from_utf8(output.stderr).expect("parallel progress");
    let expected = [
        "shuttle: scheduling with 2 jobs",
        "shuttle: compiling cloth v0.1.0 [1/5]",
        "shuttle: compiling foundation v1.0.0 [2/5]",
        "shuttle: compiling data-models v1.2.3-beta.1+local [3/5]",
        "shuttle: compiling tools v0.2.0 [4/5]",
        "shuttle: compiling app v0.1.0 [5/5]",
    ];
    let mut previous = 0;
    for message in expected {
        let position = progress[previous..].find(message).map_or_else(
            || panic!("missing {message:?} in {progress:?}"),
            |position| previous + position,
        );
        previous = position + message.len();
    }
}

#[test]
fn parallel_failures_replay_the_same_canonical_diagnostic_as_one_job() {
    let serial = Fixture::named("serial failure");
    let serial_compiler = stub(&serial);
    let mut serial_command = command(&serial, &serial_compiler, "check", "parallel-failure");
    let serial_output = run(&mut serial_command);
    expect_status(&serial_output, 1);

    let parallel = Fixture::named("parallel failure");
    let parallel_compiler = stub(&parallel);
    let mut parallel_command = configured_command(
        &parallel,
        &parallel_compiler,
        "check",
        "parallel-failure",
        false,
        2,
    );
    let parallel_output = run(&mut parallel_command);
    expect_status(&parallel_output, 1);

    assert_eq!(serial_output.stderr, parallel_output.stderr);
    assert_eq!(
        String::from_utf8(parallel_output.stderr)
            .expect("parallel diagnostic")
            .replace("\r\n", "\n"),
        "data-models.co:3:5: error: parallel stub rejection\n"
    );
    assert_eq!(
        compiled_packages(&serial),
        ["cloth", "foundation", "data-models"]
    );
    let mut parallel_packages = compiled_packages(&parallel);
    parallel_packages.sort();
    assert_eq!(
        parallel_packages,
        ["cloth", "data-models", "foundation", "tools"]
    );
}

#[test]
fn reuses_every_unchanged_package_and_reports_validation() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    let first = run(&mut command(&fixture, &compiler, "build", ""));
    expect_status(&first, 0);
    fs::remove_file(fixture.root.join("calls.log")).expect("reset call log");

    let second = run(&mut visible_command(&fixture, &compiler, "build", ""));
    expect_status(&second, 0);
    assert_eq!(
        phases(&fixture),
        ["query", "reuse", "reuse", "reuse", "reuse", "reuse", "link"]
    );
    assert!(compiled_packages(&fixture).is_empty());
    let progress = String::from_utf8(second.stderr).expect("progress");
    for package in ["cloth", "foundation", "data-models", "tools", "app"] {
        assert!(
            progress.contains(&format!("shuttle: validating {package} "))
                && progress.contains(&format!("shuttle: reusing {package} ")),
            "missing reuse progress for {package}: {progress}"
        );
    }
    assert!(!progress.contains("shuttle: compiling"));
}

#[test]
fn manifest_only_invalidation_stops_at_an_identical_artifact() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    let first = run(&mut command(&fixture, &compiler, "build", ""));
    expect_status(&first, 0);

    let manifest = fixture.root.join("core/Shuttle.toml");
    let mut contents = fs::read_to_string(&manifest).expect("foundation manifest");
    contents.push_str("\n# exact manifest snapshot changed\n");
    fs::write(manifest, contents).expect("change foundation manifest");
    fs::remove_file(fixture.root.join("calls.log")).expect("reset call log");

    let second = run(&mut command(&fixture, &compiler, "build", ""));
    expect_status(&second, 0);
    assert_eq!(compiled_packages(&fixture), ["foundation"]);
    assert_eq!(
        phases(&fixture),
        [
            "query", "reuse", "compile", "reuse", "reuse", "reuse", "link"
        ]
    );
}

#[test]
fn malformed_local_state_is_ignored_without_invalidating_consumers() {
    let fixture = Fixture::new();
    let compiler = stub(&fixture);
    let first = run(&mut command(&fixture, &compiler, "build", ""));
    expect_status(&first, 0);

    let state_directory = fixture
        .root
        .join("app/target/x86_64/.shuttle/state/foundation");
    let states = fs::read_dir(&state_directory)
        .expect("foundation state directory")
        .collect::<Result<Vec<_>, _>>()
        .expect("foundation state entries");
    assert_eq!(states.len(), 1);
    assert_eq!(
        states[0]
            .path()
            .extension()
            .and_then(|value| value.to_str()),
        Some("json")
    );
    fs::write(states[0].path(), b"{not valid state").expect("corrupt local state");
    fs::remove_file(fixture.root.join("calls.log")).expect("reset call log");

    let second = run(&mut command(&fixture, &compiler, "build", ""));
    expect_status(&second, 0);
    assert_eq!(compiled_packages(&fixture), ["foundation"]);
    assert_eq!(
        phases(&fixture),
        [
            "query", "reuse", "compile", "reuse", "reuse", "reuse", "link"
        ]
    );
    assert_eq!(
        fs::read_dir(state_directory)
            .expect("repaired state directory")
            .count(),
        1
    );
}

#[test]
fn rejects_a_concurrent_writer_for_the_same_target() {
    for action in ["build", "check"] {
        let fixture = Fixture::new();
        let compiler = stub(&fixture);
        let mut first = command(&fixture, &compiler, action, "compile-wait");
        let child = first
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start first build");
        let start = Instant::now();
        while !fs::read_to_string(fixture.root.join("calls.log"))
            .is_ok_and(|log| log.contains("compile:"))
        {
            assert!(start.elapsed() < Duration::from_secs(10));
            thread::sleep(Duration::from_millis(10));
        }

        let second = run(&mut command(&fixture, &compiler, action, ""));
        expect_status(&second, 2);
        assert!(
            String::from_utf8(second.stderr)
                .expect("lock diagnostic")
                .contains("owns")
        );
        let first_output = child.wait_with_output().expect("finish first build");
        expect_status(&first_output, 0);
    }
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
    write_stub_toolchain(&sibling);
    write_stub_toolchain(&fallback);
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
