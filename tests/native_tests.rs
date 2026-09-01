// Part of the Cloth Compiler project, under the Apache License v2.0 with LLVM
// Exceptions. See LICENSE.txt in the project root for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Opt-in execution tests; require a compiler with its native toolchain.
#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde::Deserialize;
use shuttle::compiler::{ProjectCommand, Target, build_request};
use shuttle::graph::resolve_package_graph;
use support::{Fixture, compiler, expect_status, run};

#[derive(Clone, Debug)]
struct ArtifactRecord {
    name: String,
    version: String,
    digest: String,
    path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ReceiptPackage {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct ArtifactReceipt {
    artifact_id: String,
    package: ReceiptPackage,
}

fn whole_project_run(fixture: &Fixture) -> Output {
    let graph = resolve_package_graph(&fixture.manifest()).expect("fixture graph");
    let request = build_request(&graph, ProjectCommand::Build, Target::X86_64)
        .expect("whole-project request");
    let output = request
        .output_path()
        .expect("executable output")
        .to_path_buf();
    fs::create_dir_all(output.parent().expect("output parent")).expect("create output parent");
    let compiled = run(Command::new(compiler())
        .current_dir(&fixture.root)
        .args(request.arguments()));
    expect_status(&compiled, 0);
    assert!(compiled.stdout.is_empty() && compiled.stderr.is_empty());
    run(&mut Command::new(output))
}

fn inspect_artifact(path: &Path) -> ArtifactRecord {
    let output = run(Command::new(compiler())
        .args(["--shuttle-protocol", "2", "--operation", "inspect"])
        .arg("--input")
        .arg(path));
    expect_status(&output, 0);
    assert!(output.stderr.is_empty());
    let receipt: ArtifactReceipt =
        serde_json::from_slice(&output.stdout).expect("artifact receipt");
    ArtifactRecord {
        name: receipt.package.name,
        version: receipt.package.version,
        digest: receipt.artifact_id,
        path: path.to_path_buf(),
    }
}

fn artifact_records(fixture: &Fixture) -> Vec<ArtifactRecord> {
    let directory = fixture.root.join("app/target/x86_64/packages");
    let mut paths = fs::read_dir(directory)
        .expect("package artifacts")
        .map(|entry| entry.expect("artifact entry").path())
        .collect::<Vec<_>>();
    paths.sort();
    paths.iter().map(|path| inspect_artifact(path)).collect()
}

fn link_arguments(output: &Path, artifacts: &[ArtifactRecord]) -> Vec<OsString> {
    let mut arguments = vec![
        "--shuttle-protocol".into(),
        "2".into(),
        "--operation".into(),
        "link".into(),
        "--target".into(),
        "x86_64".into(),
        "--output".into(),
        output.as_os_str().to_owned(),
        "--root-package".into(),
        "app".into(),
        "--entry".into(),
        "Main.co".into(),
    ];
    for artifact in artifacts {
        arguments.extend([
            "--artifact".into(),
            OsString::from(&artifact.name),
            OsString::from(&artifact.version),
            OsString::from(&artifact.digest),
            artifact.path.as_os_str().to_owned(),
        ]);
    }
    arguments
}

fn invoke_link(fixture: &Fixture, output: &Path, artifacts: &[ArtifactRecord]) -> Output {
    run(Command::new(compiler())
        .current_dir(&fixture.root)
        .args(link_arguments(output, artifacts)))
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn builds_and_runs_only_the_selected_root_entry() {
    let fixture = Fixture::new();
    let built = run(&mut fixture.shuttle("build", &compiler()));
    expect_status(&built, 0);
    assert!(built.stdout.is_empty() && built.stderr.is_empty());
    let target = fixture.root.join("app/target/x86_64");
    assert!(
        target
            .join(format!("app{}", std::env::consts::EXE_SUFFIX))
            .is_file()
    );
    let artifacts = fs::read_dir(target.join("packages"))
        .expect("package artifacts")
        .collect::<Result<Vec<_>, _>>()
        .expect("package artifact entries");
    assert_eq!(artifacts.len(), 4);
    assert!(artifacts.iter().all(|entry| {
        entry.path().is_file() && entry.path().extension().is_some_and(|value| value == "cpa")
    }));
    let output = run(&mut fixture.shuttle("run", &compiler()));
    expect_status(&output, 0);
    assert_eq!(
        String::from_utf8(output.stdout)
            .expect("UTF-8 output")
            .replace("\r\n", "\n"),
        "41\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn separate_and_whole_project_programs_are_equivalent() {
    let whole = Fixture::from_fixture("equivalence_graph", "whole project");
    let separate = Fixture::from_fixture("equivalence_graph", "separate project");
    let whole_output = whole_project_run(&whole);
    let separate_output = run(&mut separate.shuttle("run", &compiler()));
    expect_status(&whole_output, 0);
    expect_status(&separate_output, 0);
    assert!(whole_output.stderr.is_empty() && separate_output.stderr.is_empty());
    assert_eq!(whole_output.stdout, separate_output.stdout);
    let text = String::from_utf8(separate_output.stdout).expect("program output");
    for expected in ["Entity->Derived", "Derived", "Cloth", "2000"] {
        assert!(text.contains(expected), "missing {expected:?} in {text:?}");
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn native_build_accepts_unicode_project_paths() {
    let fixture = Fixture::named("Cloth \u{03bb} native &");
    let output = run(&mut fixture.shuttle("run", &compiler()));
    expect_status(&output, 0);
    assert_eq!(
        String::from_utf8(output.stdout)
            .expect("UTF-8 output")
            .trim(),
        "41"
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn propagates_the_program_exit_status_and_does_not_run_stale_outputs() {
    let fixture = Fixture::new();
    fixture.write(
        "app/src/Main.co",
        "static func Main(): int32 { println(\"ran\"); return 7; }\n",
    );
    let output = run(&mut fixture.shuttle("run", &compiler()));
    expect_status(&output, 7);
    assert_eq!(
        String::from_utf8(output.stdout).expect("output").trim(),
        "ran"
    );
    let executable = fixture
        .root
        .join("app/target/x86_64")
        .join(format!("app{}", std::env::consts::EXE_SUFFIX));
    let previous = fs::read(&executable).expect("completed executable");
    fixture.write("app/src/Main.co", "static func Main() { missing(); }\n");
    let output = run(&mut fixture.shuttle("run", &compiler()));
    expect_status(&output, 1);
    assert!(output.stdout.is_empty());
    assert_eq!(fs::read(executable).expect("previous executable"), previous);
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn rejects_an_ineligible_entry_instead_of_using_another_main() {
    let fixture = Fixture::new();
    fixture.write("app/src/Main.co", "static func NotMain() {}\n");
    let output = run(&mut fixture.shuttle("build", &compiler()));
    expect_status(&output, 1);
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .expect("error")
            .contains("Main")
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn relocated_native_builds_are_byte_identical() {
    let first = Fixture::new();
    let second = Fixture::new();
    let executable = format!("app/target/x86_64/app{}", std::env::consts::EXE_SUFFIX);
    expect_status(&run(&mut first.shuttle("build", &compiler())), 0);
    // Cross a PE timestamp boundary to detect wall-clock-dependent link output.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    expect_status(&run(&mut second.shuttle("build", &compiler())), 0);
    assert!(
        fs::read(first.root.join(&executable)).expect("first binary")
            == fs::read(second.root.join(&executable)).expect("second binary"),
        "relocated native builds differ"
    );
    for package in ["app", "data-models", "foundation", "tools"] {
        let relative = format!("app/target/x86_64/packages/{package}.cpa");
        assert_eq!(
            fs::read(first.root.join(&relative)).expect("first artifact"),
            fs::read(second.root.join(&relative)).expect("second artifact"),
            "relocated artifact {package} differs"
        );
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn malformed_link_inputs_fail_without_replacing_outputs() {
    let fixture = Fixture::new();
    expect_status(&run(&mut fixture.shuttle("build", &compiler())), 0);
    let artifacts = artifact_records(&fixture);
    assert_eq!(artifacts.len(), 4);
    let output = fixture.root.join(format!(
        "app/target/x86_64/invalid{}",
        std::env::consts::EXE_SUFFIX
    ));
    let previous = b"previous executable";

    let expect_rejected = |records: &[ArtifactRecord]| {
        fs::write(&output, previous).expect("previous output");
        let result = invoke_link(&fixture, &output, records);
        expect_status(&result, 2);
        assert!(result.stdout.is_empty());
        assert_eq!(fs::read(&output).expect("preserved output"), previous);
    };

    let root_only = artifacts
        .iter()
        .filter(|artifact| artifact.name == "app")
        .cloned()
        .collect::<Vec<_>>();
    expect_rejected(&root_only);

    let mut wrong_digest = artifacts.clone();
    wrong_digest[0].digest = "0".repeat(64);
    expect_rejected(&wrong_digest);

    let mut wrong_version = artifacts.clone();
    wrong_version[0].version = "9.9.9".to_owned();
    expect_rejected(&wrong_version);

    let mut corrupt = artifacts.clone();
    let corrupt_path = fixture.root.join("corrupt.cpa");
    let mut bytes = fs::read(&corrupt[0].path).expect("artifact bytes");
    let last = bytes.last_mut().expect("nonempty artifact");
    *last ^= 1;
    fs::write(&corrupt_path, bytes).expect("corrupt artifact");
    corrupt[0].path = corrupt_path;
    expect_rejected(&corrupt);

    let app = artifacts
        .iter()
        .find(|artifact| artifact.name == "app")
        .expect("app artifact");
    let app_bytes = fs::read(&app.path).expect("app artifact bytes");
    let aliased = invoke_link(&fixture, &app.path, &artifacts);
    expect_status(&aliased, 2);
    assert!(aliased.stdout.is_empty());
    assert_eq!(fs::read(&app.path).expect("preserved artifact"), app_bytes);

    let mut duplicate_arguments = link_arguments(&output, &artifacts);
    duplicate_arguments.extend([
        "--artifact".into(),
        OsString::from(&app.name),
        OsString::from(&app.version),
        OsString::from(&app.digest),
        app.path.as_os_str().to_owned(),
    ]);
    fs::write(&output, previous).expect("previous output");
    let duplicate = run(Command::new(compiler()).args(duplicate_arguments));
    expect_status(&duplicate, 2);
    assert!(duplicate.stdout.is_empty());
    assert_eq!(fs::read(&output).expect("preserved output"), previous);
}
