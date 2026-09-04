// Part of the Cloth Compiler project, under the Apache License v2.0 with LLVM
// Exceptions. See LICENSE.txt in the project root for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

//! Opt-in tests of the public process protocol, never compiler internals.
#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;
use shuttle::compiler::{ProjectCommand, Target, build_request};
use shuttle::graph::resolve_package_graph;
use support::{Fixture, SWITCH_CASES, SWITCH_MAIN, compiler, expect_status, run};

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn checked_updates_have_deterministic_cross_target_artifacts() {
    let selected = compiler();
    for target in ["x86_64", "wasm32"] {
        let serial = Fixture::checked_updates();
        let parallel = Fixture::checked_updates();
        parallel.reverse_dependencies();
        for (fixture, jobs) in [(&serial, "1"), (&parallel, "4")] {
            let checked = run(fixture
                .shuttle("check", &selected)
                .args(["--target", target, "--jobs", jobs]));
            expect_status(&checked, 0);
            assert!(checked.stdout.is_empty() && checked.stderr.is_empty());
        }
        let directory = format!("app/target/{target}/check/packages");
        assert_eq!(
            serial.artifact_bytes(&directory),
            parallel.artifact_bytes(&directory)
        );
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn integer_conversions_have_deterministic_cross_target_artifacts() {
    let selected = compiler();
    for target in ["x86_64", "wasm32"] {
        let serial = Fixture::integer_conversions();
        let parallel = Fixture::integer_conversions();
        parallel.reverse_dependencies();
        for (fixture, jobs) in [(&serial, "1"), (&parallel, "4")] {
            let checked = run(fixture
                .shuttle("check", &selected)
                .args(["--target", target, "--jobs", jobs]));
            expect_status(&checked, 0);
            assert!(checked.stdout.is_empty() && checked.stderr.is_empty());
        }
        let directory = format!("app/target/{target}/check/packages");
        assert_eq!(
            serial.artifact_bytes(&directory),
            parallel.artifact_bytes(&directory)
        );
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn typed_literals_have_deterministic_cross_target_artifacts() {
    let selected = compiler();
    for target in ["x86_64", "wasm32"] {
        let serial = Fixture::typed_literals();
        let parallel = Fixture::typed_literals();
        parallel.reverse_dependencies();
        for (fixture, jobs) in [(&serial, "1"), (&parallel, "4")] {
            let checked = run(fixture
                .shuttle("check", &selected)
                .args(["--target", target, "--jobs", jobs]));
            expect_status(&checked, 0);
            assert!(checked.stdout.is_empty() && checked.stderr.is_empty());
        }
        let directory = format!("app/target/{target}/check/packages");
        assert_eq!(
            serial.artifact_bytes(&directory),
            parallel.artifact_bytes(&directory)
        );
    }
}

#[derive(Clone, Debug)]
struct ProtocolArtifact {
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

fn arguments(fixture: &Fixture) -> Vec<OsString> {
    let graph = resolve_package_graph(&fixture.manifest()).expect("fixture graph");
    build_request(&graph, ProjectCommand::Check, Target::Wasm32)
        .expect("check request")
        .arguments()
        .to_vec()
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn computed_constants_have_deterministic_artifacts_and_failure_order() {
    let selected = compiler();
    for target in ["x86_64", "wasm32"] {
        let serial = Fixture::constants();
        let parallel = Fixture::constants();
        parallel.reverse_dependencies();
        let directory = format!("app/target/{target}/check/packages");
        let mut failures = Vec::new();
        for (fixture, jobs, reverse) in [(&serial, "1", false), (&parallel, "4", true)] {
            let checked = run(fixture
                .shuttle("check", &selected)
                .args(["--target", target, "--jobs", jobs]));
            expect_status(&checked, 0);
            assert!(checked.stdout.is_empty() && checked.stderr.is_empty());
            let previous = fixture.artifact_bytes(&directory);
            let mut files = [
                (
                    "models/src/First.co",
                    "static final int32 A = Last.Z; static final int32 unused = 1 / 0;",
                ),
                ("models/src/Last.co", "static final int32 Z = First.A;"),
            ];
            if reverse {
                files.reverse();
            }
            for (path, source) in files {
                fixture.write(path, source);
            }
            let failed = run(fixture
                .shuttle("check", &selected)
                .args(["--target", target, "--jobs", jobs]));
            expect_status(&failed, 1);
            assert!(failed.stdout.is_empty());
            let detail = String::from_utf8(failed.stderr).unwrap();
            assert!(
                detail.contains("cyclic static constant") && detail.contains("by zero"),
                "{detail}"
            );
            failures.push(detail.replace(
                &fixture.root.to_string_lossy().replace('\\', "/"),
                "<project>",
            ));
            assert_eq!(previous, fixture.artifact_bytes(&directory));
        }
        assert_eq!(
            serial.artifact_bytes(&directory),
            parallel.artifact_bytes(&directory)
        );
        assert_eq!(failures[0], failures[1]);
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn computed_constants_enforce_source_free_access_types_and_switch_labels() {
    let fixture = Fixture::constants();
    let artifacts = compile_switch_dependencies(&fixture);
    let app = compile_interface(
        &fixture,
        ("app", "0.1.0", "app/src"),
        Some("Main.co"),
        &[("models", "data-models"), ("tools", "tools")],
        &artifacts,
    );
    let completed = fs::read(&app.path).unwrap();
    for package in ["core", "models", "tools"] {
        fs::rename(
            fixture.root.join(package).join("src"),
            fixture.root.join(package).join("hidden-source"),
        )
        .unwrap();
    }
    for (source, diagnostic) in [
        ("static final int32 Value = Values.hidden;", "private"),
        (
            "static final int8 Value = Values.Answer;",
            "expected 'int8'",
        ),
        (
            "static final int64 Value = Values.Answer + 2; static func Check(int64 n) { switch(n) { case Value: {} case 44: {} } }",
            "duplicate switch case",
        ),
        (
            "static final bool Value = true || Value;",
            "cyclic static constant",
        ),
        (
            "static func Check(int64 n) { switch(n) { case Values.Quarter: {} } }",
            "case constant of type 'float32'",
        ),
    ] {
        fixture.write(
            "app/src/Main.co",
            &format!("import models::Constants as Values;\n{source}\nstatic func Main() {{}}\n"),
        );
        let (mut command, _) = interface_command(
            &fixture,
            ("app", "0.1.0", "app/src"),
            Some("Main.co"),
            &[("models", "data-models"), ("tools", "tools")],
            &artifacts,
        );
        let failed = run(&mut command);
        expect_status(&failed, 1);
        assert!(failed.stdout.is_empty());
        let detail = String::from_utf8_lossy(&failed.stderr);
        assert!(detail.contains(diagnostic), "{detail}");
        assert_eq!(completed, fs::read(&app.path).unwrap());
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn switch_interface_artifacts_and_diagnostics_are_deterministic() {
    let selected = compiler();
    for target in ["x86_64", "wasm32"] {
        let serial = Fixture::switches();
        let parallel = Fixture::switches();
        parallel.reverse_dependencies();
        for (fixture, jobs) in [(&serial, "1"), (&parallel, "4")] {
            let checked = run(fixture
                .shuttle("check", &selected)
                .args(["--target", target, "--jobs", jobs]));
            expect_status(&checked, 0);
            assert!(checked.stdout.is_empty() && checked.stderr.is_empty());
        }
        let directory = format!("app/target/{target}/check/packages");
        let previous = serial.artifact_bytes(&directory);
        assert_eq!(previous, parallel.artifact_bytes(&directory));
        let mut failures = Vec::new();
        for (fixture, jobs) in [(&serial, "1"), (&parallel, "4")] {
            fixture.write_switch_model(&["Ready", "ready", "_Done", "Added"], "ready", 7);
            let failed = run(fixture
                .shuttle("check", &selected)
                .args(["--target", target, "--jobs", jobs]));
            expect_status(&failed, 1);
            assert!(failed.stdout.is_empty());
            assert!(String::from_utf8_lossy(&failed.stderr).contains("missing cases: Added"));
            let current = fixture.artifact_bytes(&directory);
            for package in ["app", "foundation", "tools"] {
                assert_eq!(
                    current[package], previous[package],
                    "failed check replaced {package}"
                );
            }
            assert_ne!(current["data-models"], previous["data-models"]);
            // Relocation changes only the absolute project prefix. Keep source
            // locations, diagnostic text, notes, and ordering in the comparison.
            failures.push(
                String::from_utf8(failed.stderr)
                    .expect("UTF-8 diagnostics")
                    .replace(
                        &fixture.root.to_string_lossy().replace('\\', "/"),
                        "<project>",
                    ),
            );
        }
        assert_eq!(
            failures[0], failures[1],
            "switch diagnostics depend on scheduling/path"
        );
        assert_eq!(
            serial.artifact_bytes(&directory),
            parallel.artifact_bytes(&directory)
        );
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn switch_source_free_evolution_and_access_checks_preserve_output() {
    let fixture = Fixture::switches();
    let mut artifacts = compile_switch_dependencies(&fixture);
    let app = compile_interface(
        &fixture,
        ("app", "0.1.0", "app/src"),
        Some("Main.co"),
        &[("models", "data-models"), ("tools", "tools")],
        &artifacts,
    );
    let mut previous = fs::read(&app.path).expect("completed switch artifact");
    for package in ["core", "tools"] {
        fs::rename(
            fixture.root.join(package).join("src"),
            fixture.root.join(package).join("hidden-source"),
        )
        .expect("hide test dependency sources");
    }
    for (cases, initial, small, expected) in [
        (&["_Done", "ready", "Ready"][..], "ready", 7, None),
        (SWITCH_CASES, "ready", 9, None),
        (
            &["Ready", "ready", "_Done", "Added"][..],
            "ready",
            7,
            Some("missing cases: Added"),
        ),
        (
            &["Ready", "ready"][..],
            "ready",
            7,
            Some("has no case '_Done'"),
        ),
        (
            &["Ready", "ready", "Finished"][..],
            "ready",
            7,
            Some("has no case '_Done'"),
        ),
        (SWITCH_CASES, "Ready", 7, Some("duplicate switch case")),
    ] {
        fixture.write_switch_model(cases, initial, small);
        artifacts[1] = compile_interface(
            &fixture,
            ("data-models", "1.2.3-beta.1+local", "models/src"),
            None,
            &[("foundation", "foundation")],
            std::slice::from_ref(&artifacts[0]),
        );
        fs::rename(
            fixture.root.join("models/src"),
            fixture.root.join("models/hidden-source"),
        )
        .expect("hide edited model sources");
        let (mut command, _) = interface_command(
            &fixture,
            ("app", "0.1.0", "app/src"),
            Some("Main.co"),
            &[("models", "data-models"), ("tools", "tools")],
            &artifacts,
        );
        let output = run(&mut command);
        expect_status(&output, i32::from(expected.is_some()));
        let current = fs::read(&app.path).expect("consumer artifact");
        if let Some(expected) = expected {
            assert!(output.stdout.is_empty());
            let diagnostic = String::from_utf8_lossy(&output.stderr);
            assert!(diagnostic.contains(expected), "{diagnostic}");
            assert_eq!(
                current, previous,
                "failed source-free compilation replaced output"
            );
        } else {
            assert!(output.stderr.is_empty());
            assert_ne!(
                current, previous,
                "consumer did not retain changed dependency identity"
            );
            previous = current;
        }
        fs::rename(
            fixture.root.join("models/hidden-source"),
            fixture.root.join("models/src"),
        )
        .expect("restore test model sources for next edit");
    }
    check_switch_access_policy(&fixture, &artifacts, &app, &previous);
}

fn compile_switch_dependencies(fixture: &Fixture) -> [ProtocolArtifact; 3] {
    let foundation =
        compile_interface(fixture, ("foundation", "1.0.0", "core/src"), None, &[], &[]);
    let models = compile_interface(
        fixture,
        ("data-models", "1.2.3-beta.1+local", "models/src"),
        None,
        &[("foundation", "foundation")],
        std::slice::from_ref(&foundation),
    );
    let tools = compile_interface(
        fixture,
        ("tools", "0.2.0", "tools/src"),
        None,
        &[("base", "foundation")],
        std::slice::from_ref(&foundation),
    );
    [foundation, models, tools]
}

fn check_switch_access_policy(
    fixture: &Fixture,
    artifacts: &[ProtocolArtifact],
    app: &ProtocolArtifact,
    previous: &[u8],
) {
    for body in [
        "int32 n = 1; switch (n) { case Constants.hidden: {} }",
        "switch (Status.Ready) { case Foreign.Ready: {} default: {} }",
    ] {
        fixture.write("app/src/Main.co", &format!(
            "import models::State as Status; import models::StateReader as Constants; import models::Other as Foreign; static func Main() {{ {body} }}"));
        let (mut command, _) = interface_command(
            fixture,
            ("app", "0.1.0", "app/src"),
            Some("Main.co"),
            &[("models", "data-models"), ("tools", "tools")],
            artifacts,
        );
        // No dependency source root is present in this request; hide them too.
        fs::rename(
            fixture.root.join("models/src"),
            fixture.root.join("models/hidden-source"),
        )
        .expect("hide model sources for access check");
        let rejected = run(&mut command);
        expect_status(&rejected, 1);
        let diagnostic = String::from_utf8_lossy(&rejected.stderr);
        assert!(
            diagnostic.contains(if body.contains("hidden") {
                "private"
            } else {
                "cannot be used"
            }),
            "{diagnostic}"
        );
        assert!(rejected.stdout.is_empty());
        assert_eq!(fs::read(&app.path).expect("preserved artifact"), previous);
        fs::rename(
            fixture.root.join("models/hidden-source"),
            fixture.root.join("models/src"),
        )
        .expect("restore model sources");
    }
    fixture.write("app/src/Main.co", SWITCH_MAIN);
    for keyword in ["switch", "case", "default"] {
        let (mut command, _) = interface_command(
            fixture,
            ("app", "0.1.0", "app/src"),
            Some("Main.co"),
            &[(keyword, "data-models"), ("tools", "tools")],
            artifacts,
        );
        let rejected = run(&mut command);
        expect_status(&rejected, 2);
        assert!(rejected.stdout.is_empty());
        assert_eq!(fs::read(&app.path).expect("preserved artifact"), previous);
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn enum_case_edits_invalidate_consumers_and_preserve_independent_reuse() {
    let fixture = Fixture::enums();
    let selected = compiler();
    expect_status(
        &run(fixture
            .shuttle("check", &selected)
            .args(["--target", "wasm32"])),
        0,
    );
    let unchanged = run(fixture
        .visible_shuttle("check", &selected)
        .args(["--target", "wasm32"]));
    expect_status(&unchanged, 0);
    assert!(!String::from_utf8_lossy(&unchanged.stderr).contains("shuttle: checking"));
    fixture.write(
        "models/src/State.co",
        "enum { _Done, ready, Ready, Added }\n",
    );
    let changed = run(fixture
        .visible_shuttle("check", &selected)
        .args(["--target", "wasm32"]));
    expect_status(&changed, 0);
    let progress = String::from_utf8_lossy(&changed.stderr);
    assert!(
        progress.contains("shuttle: checking data-models ")
            && progress.contains("shuttle: checking app "),
        "{progress}"
    );
    assert!(
        progress.contains("shuttle: reusing foundation ")
            && progress.contains("shuttle: reusing tools "),
        "{progress}"
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn enum_cases_and_constants_are_available_without_dependency_sources() {
    let fixture = Fixture::enums();
    let foundation = compile_interface(
        &fixture,
        ("foundation", "1.0.0", "core/src"),
        None,
        &[],
        &[],
    );
    let models = compile_interface(
        &fixture,
        ("data-models", "1.2.3-beta.1+local", "models/src"),
        None,
        &[("foundation", "foundation")],
        std::slice::from_ref(&foundation),
    );
    let tools = compile_interface(
        &fixture,
        ("tools", "0.2.0", "tools/src"),
        None,
        &[("base", "foundation")],
        std::slice::from_ref(&foundation),
    );
    for directory in ["core/src", "models/src", "tools/src"] {
        fs::remove_dir_all(fixture.root.join(directory))
            .expect("remove temporary dependency sources");
    }
    let app = compile_interface(
        &fixture,
        ("app", "0.1.0", "app/src"),
        Some("Main.co"),
        &[("models", "data-models"), ("tools", "tools")],
        &[foundation, models, tools],
    );
    assert!(app.path.is_file());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn structs_are_available_without_dependency_sources() {
    let fixture = Fixture::structs();
    let foundation = compile_interface(
        &fixture,
        ("foundation", "1.0.0", "core/src"),
        None,
        &[],
        &[],
    );
    let models = compile_interface(
        &fixture,
        ("data-models", "1.2.3-beta.1+local", "models/src"),
        None,
        &[("foundation", "foundation")],
        std::slice::from_ref(&foundation),
    );
    let tools = compile_interface(
        &fixture,
        ("tools", "0.2.0", "tools/src"),
        None,
        &[("base", "foundation")],
        std::slice::from_ref(&foundation),
    );
    for directory in ["core/src", "models/src", "tools/src"] {
        fs::remove_dir_all(fixture.root.join(directory))
            .expect("remove temporary dependency sources");
    }
    let artifacts = [foundation, models, tools];
    let app = compile_interface(
        &fixture,
        ("app", "0.1.0", "app/src"),
        Some("Main.co"),
        &[("models", "data-models"), ("tools", "tools")],
        &artifacts,
    );
    assert!(app.path.is_file());
    let previous = fs::read(&app.path).expect("completed consumer artifact");
    for body in [
        "var value = Data(1);",
        "var value = Data(\"visible\", 1); println(value.tag);",
        "var value = Data(\"visible\", 1); println(value.hidden());",
    ] {
        fixture.write(
            "app/src/Main.co",
            &format!("import models::Data;\nstatic func Main() {{ {body} }}\n"),
        );
        let (mut command, _) = interface_command(
            &fixture,
            ("app", "0.1.0", "app/src"),
            Some("Main.co"),
            &[("models", "data-models"), ("tools", "tools")],
            &artifacts,
        );
        let rejected = run(&mut command);
        expect_status(&rejected, 1);
        assert!(rejected.stdout.is_empty());
        let diagnostic = String::from_utf8_lossy(&rejected.stderr);
        assert!(diagnostic.contains("private"), "{diagnostic}");
        assert_eq!(fs::read(&app.path).expect("preserved artifact"), previous);
    }

    fixture.write("app/src/Main.co", "static func Main() {}\n");
    for (modifier, name, expected) in [
        ("", "Transform", Some("add 'override'")),
        ("override ", "Unmatched", Some("does not override")),
        ("override ", "Transform", None),
    ] {
        fixture.write(
            "app/src/Implementation.co",
            &format!(
                "import models::Transformer; import models::Data; import models::Packet;\n\
                 class is Transformer {{\n\
                 {modifier}func {name}(Data value): Packet {{ return Packet(value); }}\n\
                 }}\n"
            ),
        );
        let (mut command, _) = interface_command(
            &fixture,
            ("app", "0.1.0", "app/src"),
            Some("Main.co"),
            &[("models", "data-models"), ("tools", "tools")],
            &artifacts,
        );
        let checked = run(&mut command);
        expect_status(&checked, i32::from(expected.is_some()));
        if let Some(expected) = expected {
            assert!(checked.stdout.is_empty());
            let diagnostic = String::from_utf8_lossy(&checked.stderr);
            assert!(diagnostic.contains(expected), "{diagnostic}");
            assert_eq!(fs::read(&app.path).expect("preserved artifact"), previous);
        } else {
            assert!(checked.stderr.is_empty());
            let _: ArtifactReceipt =
                serde_json::from_slice(&checked.stdout).expect("override consumer receipt");
        }
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn struct_interface_artifacts_are_deterministic_and_invalidate_after_edits() {
    let selected = compiler();
    for target in ["x86_64", "wasm32"] {
        let serial = Fixture::structs();
        let parallel = Fixture::structs();
        for (fixture, jobs) in [(&serial, "1"), (&parallel, "4")] {
            let checked = run(fixture
                .shuttle("check", &selected)
                .args(["--target", target, "--jobs", jobs]));
            expect_status(&checked, 0);
            assert!(checked.stdout.is_empty() && checked.stderr.is_empty());
        }
        let directory = format!("app/target/{target}/check/packages");
        let mut previous = serial.artifact_bytes(&directory);
        assert_eq!(previous, parallel.artifact_bytes(&directory));
        for layout in [true, false] {
            serial.edit_struct(layout);
            let changed = run(serial
                .visible_shuttle("check", &selected)
                .args(["--target", target]));
            expect_status(&changed, 0);
            assert!(changed.stdout.is_empty());
            let progress = String::from_utf8_lossy(&changed.stderr);
            let current = serial.artifact_bytes(&directory);
            for package in ["data-models", "app"] {
                assert!(
                    progress.contains(&format!("shuttle: checking {package} ")),
                    "{progress}"
                );
                assert_ne!(
                    previous[package], current[package],
                    "stale {package} artifact"
                );
            }
            for package in ["foundation", "tools"] {
                assert!(
                    progress.contains(&format!("shuttle: reusing {package} ")),
                    "{progress}"
                );
                assert_eq!(
                    previous[package], current[package],
                    "unrelated {package} changed"
                );
            }
            let unchanged = run(serial
                .visible_shuttle("check", &selected)
                .args(["--target", target]));
            expect_status(&unchanged, 0);
            let progress = String::from_utf8_lossy(&unchanged.stderr);
            assert_eq!(
                progress.matches("shuttle: reusing ").count(),
                4,
                "{progress}"
            );
            assert!(!progress.contains("shuttle: checking "), "{progress}");
            assert_eq!(current, serial.artifact_bytes(&directory));
            previous = current;
        }
    }
}

fn invoke(fixture: &Fixture, arguments: &[OsString]) -> std::process::Output {
    run(Command::new(compiler())
        .current_dir(&fixture.root)
        .args(arguments))
}

fn replace(arguments: &mut [OsString], option: &str, value: &str) {
    let position = arguments
        .iter()
        .position(|argument| argument == option)
        .expect("option");
    arguments[position + 1] = value.into();
}

fn interface_command(
    fixture: &Fixture,
    package: (&str, &str, &str),
    entry: Option<&str>,
    dependencies: &[(&str, &str)],
    artifacts: &[ProtocolArtifact],
) -> (Command, PathBuf) {
    let (name, version, source_root) = package;
    let output = fixture.root.join("artifacts").join(format!("{name}.cpa"));
    fs::create_dir_all(output.parent().expect("artifact parent"))
        .expect("create artifact directory");
    let mut arguments = vec![
        "--shuttle-protocol".into(),
        "2".into(),
        "--operation".into(),
        "compile".into(),
        "--target".into(),
        "wasm32".into(),
        "--artifact-kind".into(),
        "interface".into(),
        "--output".into(),
        output.clone().into_os_string(),
        "--package".into(),
        name.into(),
        version.into(),
        fixture.root.join(source_root).into_os_string(),
    ];
    if let Some(entry) = entry {
        arguments.extend(["--entry".into(), entry.into()]);
    }
    for (alias, target) in dependencies {
        arguments.extend(["--dependency".into(), (*alias).into(), (*target).into()]);
    }
    for artifact in artifacts {
        arguments.extend([
            "--artifact".into(),
            artifact.name.clone().into(),
            artifact.version.clone().into(),
            artifact.digest.clone().into(),
            artifact.path.clone().into_os_string(),
        ]);
    }
    let mut command = Command::new(compiler());
    command.current_dir(&fixture.root).args(arguments);
    (command, output)
}

fn compile_interface(
    fixture: &Fixture,
    package: (&str, &str, &str),
    entry: Option<&str>,
    dependencies: &[(&str, &str)],
    artifacts: &[ProtocolArtifact],
) -> ProtocolArtifact {
    let (mut command, output) = interface_command(fixture, package, entry, dependencies, artifacts);
    let result = run(&mut command);
    expect_status(&result, 0);
    assert!(result.stderr.is_empty());
    let receipt: ArtifactReceipt =
        serde_json::from_slice(&result.stdout).expect("artifact receipt");
    assert_eq!(
        (
            receipt.package.name.as_str(),
            receipt.package.version.as_str()
        ),
        (package.0, package.1)
    );
    ProtocolArtifact {
        name: receipt.package.name,
        version: receipt.package.version,
        digest: receipt.artifact_id,
        path: output,
    }
}

fn reuse_interface_arguments(
    fixture: &Fixture,
    artifact: &ProtocolArtifact,
    source_root: &str,
) -> Vec<OsString> {
    vec![
        "--shuttle-protocol".into(),
        "2".into(),
        "--operation".into(),
        "reuse".into(),
        "--target".into(),
        "wasm32".into(),
        "--artifact-kind".into(),
        "interface".into(),
        "--input".into(),
        artifact.path.clone().into_os_string(),
        "--package".into(),
        artifact.name.clone().into(),
        artifact.version.clone().into(),
        fixture.root.join(source_root).into_os_string(),
    ]
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn capability_query_has_exact_streams_and_rejects_extra_arguments() {
    let output = run(Command::new(compiler()).arg("--shuttle-protocol-version"));
    expect_status(&output, 0);
    assert!(output.stdout == b"1\n" || output.stdout == b"1\r\n");
    assert!(output.stderr.is_empty());
    let output = run(Command::new(compiler()).args(["--shuttle-protocol-version", "extra"]));
    expect_status(&output, 2);
    assert!(output.stdout.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn reuse_operation_returns_an_exact_receipt_or_a_clean_cache_miss() {
    let fixture = Fixture::new();
    let foundation = compile_interface(
        &fixture,
        ("foundation", "1.0.0", "core/src"),
        None,
        &[],
        &[],
    );
    let arguments = reuse_interface_arguments(&fixture, &foundation, "core/src");
    let hit = invoke(&fixture, &arguments);
    expect_status(&hit, 0);
    assert!(hit.stderr.is_empty());
    let receipt: ArtifactReceipt = serde_json::from_slice(&hit.stdout).expect("reuse receipt");
    assert_eq!(receipt.artifact_id, foundation.digest);

    let source = fixture.root.join("core/src/data/Record.co");
    let mut contents = fs::read_to_string(&source).expect("foundation source");
    contents.push_str("\n// changed after artifact publication\n");
    fs::write(source, contents).expect("change foundation source");
    let miss = invoke(&fixture, &arguments);
    expect_status(&miss, 3);
    assert!(miss.stdout.is_empty() && miss.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn checks_the_complete_graph_and_a_library_without_artifacts() {
    let fixture = Fixture::new();
    for target in ["x86_64", "wasm32"] {
        let output = run(fixture
            .shuttle("check", &compiler())
            .args(["--target", target]));
        expect_status(&output, 0);
        assert!(output.stdout.is_empty() && output.stderr.is_empty());
    }
    let output = run(Command::new(env!("CARGO_BIN_EXE_shuttle"))
        .args(["check", "--manifest-path"])
        .arg(fixture.root.join("tools/Shuttle.toml"))
        .arg("--compiler")
        .arg(compiler()));
    expect_status(&output, 0);
    let check_artifacts = fs::read_dir(fixture.root.join("app/target/wasm32/check/packages"))
        .expect("persistent check artifacts")
        .collect::<Result<Vec<_>, _>>()
        .expect("check artifact entries");
    assert_eq!(check_artifacts.len(), 4);
    let library_artifacts = fs::read_dir(fixture.root.join("tools/target/x86_64/check/packages"))
        .expect("library check artifacts")
        .collect::<Result<Vec<_>, _>>()
        .expect("library artifact entries");
    assert_eq!(library_artifacts.len(), 2);
    for package in ["models", "core"] {
        assert!(!fixture.root.join(package).join("target").exists());
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn check_validates_the_selected_executable_signature_without_emission() {
    for source in [
        "static func NotMain() {}\n",
        "func Main() {}\n",
        "static func Main(int32 value) {}\n",
    ] {
        let fixture = Fixture::new();
        fixture.write("app/src/Main.co", source);
        let output = run(&mut fixture.shuttle("check", &compiler()));
        expect_status(&output, 1);
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8(output.stderr)
                .expect("diagnostic")
                .contains("Main")
        );
        assert!(
            !fixture
                .root
                .join(format!(
                    "app/target/x86_64/app{}",
                    std::env::consts::EXE_SUFFIX
                ))
                .exists()
        );
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn check_reuses_unchanged_artifacts_and_invalidates_source_dependents() {
    let fixture = Fixture::new();
    let selected_compiler = compiler();
    let first = run(fixture
        .shuttle("check", &selected_compiler)
        .args(["--target", "wasm32"]));
    expect_status(&first, 0);

    let second = run(fixture
        .visible_shuttle("check", &selected_compiler)
        .args(["--target", "wasm32"]));
    expect_status(&second, 0);
    assert!(second.stdout.is_empty());
    let progress = String::from_utf8(second.stderr).expect("reuse progress");
    for package in ["foundation", "data-models", "tools", "app"] {
        assert!(
            progress.contains(&format!("shuttle: reusing {package} ")),
            "missing reuse for {package}: {progress}"
        );
    }
    assert!(!progress.contains("shuttle: checking"));

    let source = fixture.root.join("core/src/data/Record.co");
    let mut contents = fs::read_to_string(&source).expect("foundation source");
    contents.push_str("\n// invalidate the exact source digest\n");
    fs::write(source, contents).expect("change foundation source");
    let changed = run(fixture
        .visible_shuttle("check", &selected_compiler)
        .args(["--target", "wasm32"]));
    expect_status(&changed, 0);
    let progress = String::from_utf8(changed.stderr).expect("invalidation progress");
    for package in ["foundation", "data-models", "tools", "app"] {
        assert!(
            progress.contains(&format!("shuttle: checking {package} ")),
            "dependency invalidation did not reach {package}: {progress}"
        );
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn check_separates_targets_and_invalidates_a_changed_compiler() {
    let fixture = Fixture::new();
    let selected_compiler = compiler();
    expect_status(
        &run(fixture
            .shuttle("check", &selected_compiler)
            .args(["--target", "wasm32"])),
        0,
    );
    let x86 = run(fixture
        .visible_shuttle("check", &selected_compiler)
        .args(["--target", "x86_64"]));
    expect_status(&x86, 0);
    let progress = String::from_utf8(x86.stderr).expect("target progress");
    assert_eq!(progress.matches("shuttle: checking").count(), 4);
    assert!(!progress.contains("shuttle: reusing"));

    let changed_compiler = fixture
        .root
        .join(format!("changed-clothc{}", std::env::consts::EXE_SUFFIX));
    fs::copy(&selected_compiler, &changed_compiler).expect("copy compiler");
    OpenOptions::new()
        .append(true)
        .open(&changed_compiler)
        .expect("open compiler copy")
        .write_all(b"stage-24-compiler-identity")
        .expect("change compiler identity");
    let changed = run(fixture
        .visible_shuttle("check", &changed_compiler)
        .args(["--target", "wasm32"]));
    expect_status(&changed, 0);
    let progress = String::from_utf8(changed.stderr).expect("compiler progress");
    assert_eq!(progress.matches("shuttle: checking").count(), 4);
    assert!(!progress.contains("shuttle: reusing"));
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn rejects_transitive_imports_private_types_and_private_members() {
    for source in [
        "import foundation.data::Record;\nstatic func Main() {}\n",
        "import models::secret;\nstatic func Main() {}\n",
        "import models::User as ModelUser;\nstatic func Main() { ModelUser.hidden(); }\n",
    ] {
        let fixture = Fixture::new();
        fixture.write("app/src/Main.co", source);
        let output = run(&mut fixture.shuttle("check", &compiler()));
        expect_status(&output, 1);
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8(output.stderr)
                .expect("UTF-8 diagnostic")
                .contains("Main.co:")
        );
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn forwards_utf8_source_diagnostics_without_rewriting_them() {
    let fixture = Fixture::named("Cloth \u{03bb} spaces &");
    fixture.write("app/src/Main.co", "static func Main() { missing(); }\n");
    let direct = invoke(&fixture, &arguments(&fixture));
    let shuttle = run(&mut fixture.shuttle("check", &compiler()));
    expect_status(&direct, 1);
    expect_status(&shuttle, 1);
    assert!(direct.stdout.is_empty() && shuttle.stdout.is_empty());
    assert_eq!(direct.stderr, shuttle.stderr);
    let text = String::from_utf8(shuttle.stderr).expect("diagnostics must be UTF-8");
    assert!(
        text.contains("Cloth \u{03bb} spaces &/app/src/Main.co:1:"),
        "{text}"
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn parallel_and_single_job_diagnostics_are_identical() {
    let fixture = Fixture::named("parallel diagnostic equivalence");
    fixture.write(
        "models/src/Broken.co",
        "static func Broken() { missingModel(); }\n",
    );
    fixture.write(
        "tools/src/Broken.co",
        "static func Broken() { missingTool(); }\n",
    );
    let selected_compiler = compiler();
    let mut serial = fixture.shuttle("check", &selected_compiler);
    serial.args(["--jobs", "1"]);
    let serial = run(&mut serial);
    expect_status(&serial, 1);

    let mut parallel = fixture.shuttle("check", &selected_compiler);
    parallel.args(["--jobs", "2"]);
    let parallel = run(&mut parallel);
    expect_status(&parallel, 1);
    assert!(serial.stdout.is_empty() && parallel.stdout.is_empty());
    assert_eq!(serial.stderr, parallel.stderr);
    assert!(
        String::from_utf8(parallel.stderr)
            .expect("parallel diagnostic")
            .contains("missingModel")
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn separate_and_whole_project_diagnostic_categories_are_equivalent() {
    let fixture = Fixture::from_fixture("equivalence_graph", "diagnostic equivalence");
    fixture.write(
        "app/src/Bad.co",
        "import models::Derived;\n\
         class : Derived {\n\
           Bad(): Derived(\"bad\", 0) {}\n\
           override func Score(): int32 { return 0; }\n\
         }\n",
    );
    let direct = invoke(&fixture, &arguments(&fixture));
    let separate = run(&mut fixture.shuttle("check", &compiler()));
    expect_status(&direct, 1);
    expect_status(&separate, 1);
    assert!(direct.stdout.is_empty() && separate.stdout.is_empty());
    let direct = String::from_utf8(direct.stderr).expect("whole-project diagnostic");
    let separate = String::from_utf8(separate.stderr).expect("separate diagnostic");
    let messages = |text: &str| {
        text.lines()
            .map(|line| {
                line.find(": error:")
                    .or_else(|| line.find(": note:"))
                    .map_or_else(|| line.to_owned(), |position| line[position..].to_owned())
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(messages(&direct), messages(&separate));
    assert!(
        separate
            .lines()
            .any(|line| line.starts_with("Derived.co:6:3: note:"))
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn consumer_compilation_does_not_reopen_dependency_sources() {
    let fixture = Fixture::new();
    let foundation = compile_interface(
        &fixture,
        ("foundation", "1.0.0", "core/src"),
        None,
        &[],
        &[],
    );
    fs::remove_dir_all(fixture.root.join("core/src")).expect("remove foundation sources");

    let models = compile_interface(
        &fixture,
        ("data-models", "1.2.3-beta.1+local", "models/src"),
        None,
        &[("foundation", "foundation")],
        std::slice::from_ref(&foundation),
    );
    let tools = compile_interface(
        &fixture,
        ("tools", "0.2.0", "tools/src"),
        None,
        &[("base", "foundation")],
        std::slice::from_ref(&foundation),
    );
    fs::remove_dir_all(fixture.root.join("models/src")).expect("remove model sources");
    fs::remove_dir_all(fixture.root.join("tools/src")).expect("remove tool sources");

    let app = compile_interface(
        &fixture,
        ("app", "0.1.0", "app/src"),
        Some("Main.co"),
        &[("models", "data-models"), ("tools", "tools")],
        &[models, foundation, tools],
    );
    assert!(app.path.is_file());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn rejects_invalid_protocol_requests_with_status_two_without_crashing() {
    let fixture = Fixture::new();
    let baseline = arguments(&fixture);
    let mut cases = Vec::new();
    for (option, value) in [
        ("--shuttle-protocol", "2"),
        ("--target", "arm64"),
        ("--output-kind", "object"),
        ("--root-package", "absent"),
        ("--entry", "../Main.co"),
        ("--entry", "./Main.co"),
        ("--entry", "data\\Record.co"),
        ("--entry", "Main.CO"),
    ] {
        let mut args = baseline.clone();
        replace(&mut args, option, value);
        cases.push(args);
    }
    for extra in [
        vec!["--target", "wasm32"],
        vec!["--unknown"],
        vec!["--package", "only-one-value"],
        vec!["extra.co"],
        vec!["--dependency", "app", "missing", "absent"],
        vec!["--dependency", "app", "self", "app"],
        vec!["--dependency", "foundation", "app", "app"],
        vec!["--dependency", "app", "models", "data-models"],
    ] {
        let mut args = baseline.clone();
        args.extend(extra.into_iter().map(OsString::from));
        cases.push(args);
    }
    for version in ["1", "1.0", "1.0.0.1", "01.0.0", "1.0.0-01", "1.0.0+"] {
        let mut args = baseline.clone();
        let package = args
            .iter()
            .position(|arg| arg == "--package")
            .expect("package");
        args[package + 2] = version.into();
        cases.push(args);
    }
    for args in cases {
        let output = invoke(&fixture, &args);
        expect_status(&output, 2);
        assert!(output.stdout.is_empty(), "{args:?}");
        assert!(
            String::from_utf8(output.stderr)
                .expect("UTF-8 error")
                .starts_with("clothc: error:")
        );
    }
}

#[cfg(any(unix, windows))]
#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn rejects_invalid_native_text_encoding_without_crashing() {
    let fixture = Fixture::new();
    let mut args = arguments(&fixture);
    let package = args
        .iter()
        .position(|arg| arg == "--package")
        .expect("package");
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStringExt;
        args[package + 1] = OsString::from_wide(&[0xd800]);
    }
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        args[package + 1] = OsString::from_vec(vec![0xff]);
    }
    let output = invoke(&fixture, &args);
    expect_status(&output, 2);
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .expect("UTF-8 error")
            .contains("must be valid UTF-8")
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn rejects_invalid_source_layouts_and_alias_collisions() {
    for keyword in ["switch", "case", "default"] {
        let fixture = Fixture::new();
        let manifest = fs::read_to_string(fixture.manifest()).expect("fixture manifest");
        fixture.write(
            "app/Shuttle.toml",
            &manifest.replace("models =", &format!("{keyword} =")),
        );
        let output = run(&mut fixture.shuttle("check", &compiler()));
        expect_status(&output, 1);
        assert!(String::from_utf8_lossy(&output.stderr).contains("is a Cloth keyword"));
    }
    for (path, source, message) in [
        (
            "app/src/models/Local.co",
            "",
            "collides with a local source package",
        ),
        ("app/src/bad-name/Local.co", "", "invalid source directory"),
        ("app/src/bad-name.co", "", "invalid Cloth file stem"),
    ] {
        let fixture = Fixture::new();
        fixture.write(path, source);
        let output = run(&mut fixture.shuttle("check", &compiler()));
        expect_status(&output, 2);
        assert!(
            String::from_utf8(output.stderr)
                .expect("UTF-8 error")
                .contains(message)
        );
    }
    let fixture = Fixture::new();
    fs::remove_file(fixture.root.join("tools/src/Helper.co")).expect("remove fixture source");
    fixture.write("tools/src/Helper.CO", "not Cloth source");
    let output = run(&mut fixture.shuttle("check", &compiler()));
    expect_status(&output, 2);
    assert!(
        String::from_utf8(output.stderr)
            .expect("UTF-8 error")
            .contains("contains no Cloth files")
    );
}

fn emit_ir(fixture: &Fixture) -> Vec<u8> {
    let output_path = fixture.root.join("result.ll");
    let mut args = arguments(fixture);
    replace(&mut args, "--output-kind", "llvm-ir");
    args.extend([OsString::from("--output"), output_path.into_os_string()]);
    let output = invoke(fixture, &args);
    expect_status(&output, 0);
    assert!(output.stdout.is_empty() && output.stderr.is_empty());
    fs::read(fixture.root.join("result.ll")).expect("read emitted IR")
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn relocation_manifest_order_and_working_directory_do_not_change_ir() {
    let first = Fixture::new();
    let second = Fixture::new();
    let manifest = fs::read_to_string(second.manifest()).expect("manifest");
    second.write(
        "app/Shuttle.toml",
        &manifest.replace(
            "models = { path = \"../models\" }\ntools = { path = \"../tools\" }",
            "tools = { path = \"../tools\" }\nmodels = { path = \"../models\" }",
        ),
    );
    assert_eq!(emit_ir(&first), emit_ir(&second));
    let output = run(Command::new(compiler())
        .current_dir(&second.root)
        .args(arguments(&first)));
    expect_status(&output, 0);
    assert!(output.stdout.is_empty() && output.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn failed_emission_preserves_completed_outputs_and_creates_no_partial_file() {
    let fixture = Fixture::new();
    let previous = emit_ir(&fixture);
    fixture.write("app/src/Main.co", "static func Main() { missing(); }\n");
    let mut args = arguments(&fixture);
    replace(&mut args, "--output-kind", "llvm-ir");
    args.extend([
        "--output".into(),
        fixture.root.join("result.ll").into_os_string(),
    ]);
    expect_status(&invoke(&fixture, &args), 1);
    assert_eq!(
        fs::read(fixture.root.join("result.ll")).expect("previous output"),
        previous
    );
    assert!(!fixture.root.join("result.ll.tmp").exists());
    let missing = fixture.root.join("missing/result.ll");
    *args.last_mut().expect("output argument") = missing.clone().into_os_string();
    expect_status(&invoke(&fixture, &args), 2);
    assert!(!missing.exists());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn computed_constants_emit_and_preserve_outputs_on_failure() {
    let fixture = Fixture::new();
    let source = fixture.root.join("constants/Main.co");
    for initializer in ["6 * 7", "int8(-128)", "Next", "false && (1 / 0 == 0)"] {
        let ty = if initializer.starts_with("false") {
            "bool"
        } else {
            "int32"
        };
        fixture.write("constants/Main.co", &format!(
            "static final {ty} Value = {initializer}; static final int32 Next = 42; static func Main() {{ println(Value); }}\n"
        ));
        for target in ["x86_64", "wasm32"] {
            let output = run(Command::new(compiler())
                .args(["--emit-llvm", &format!("--target={target}")])
                .arg(&source));
            expect_status(&output, 0);
            assert!(output.stderr.is_empty());
            let ir = String::from_utf8(output.stdout).expect("LLVM IR");
            assert!(ir.contains("constant i"));
            assert!(
                !ir.contains("_C1I"),
                "static constant generated an initializer"
            );
        }
        let artifact = compile_interface(
            &fixture,
            ("constants", "1.0.0", "constants"),
            Some("Main.co"),
            &[],
            &[],
        );
        let previous = fs::read(&artifact.path).expect("completed artifact");
        fixture.write(
            "constants/Main.co",
            "static final int32 Bad = 1 / 0; static func Main() {}\n",
        );
        for mode in ["--emit-llvm=", "--build="] {
            let output = fixture.root.join("completed output");
            fs::write(&output, b"completed output").expect("seed output");
            let mut option = OsString::from(mode);
            option.push(&output);
            let failed = run(Command::new(compiler()).arg(option).arg(&source));
            expect_status(&failed, 1);
            assert!(String::from_utf8_lossy(&failed.stderr).contains("by zero"));
            assert_eq!(
                fs::read(output).expect("preserved output"),
                b"completed output"
            );
        }
        let (mut command, output) = interface_command(
            &fixture,
            ("constants", "1.0.0", "constants"),
            Some("Main.co"),
            &[],
            &[],
        );
        let failed = run(&mut command);
        expect_status(&failed, 1);
        assert!(failed.stdout.is_empty());
        assert_eq!(fs::read(output).expect("preserved artifact"), previous);
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn direct_compilation_ignores_both_manifest_names() {
    let fixture = Fixture::named("direct \u{03bb} project");
    fixture.write("standalone/Shuttle.toml", "not valid TOML");
    fixture.write("standalone/cloth.toml", "obsolete marker");
    fixture.write(
        "standalone/src/Main.co",
        "import local::Value;\nstatic func Main() { println(Value.Get()); }\n",
    );
    fixture.write(
        "standalone/src/local/Value.co",
        "static func Get(): int32 { return 41; }\n",
    );
    let source_root = fixture.root.join("standalone/src");
    let mut option = OsString::from("--source-root=");
    option.push(&source_root);
    let output = run(Command::new(compiler())
        .arg(option)
        .arg("--emit-llvm")
        .arg(source_root.join("Main.co")));
    expect_status(&output, 0);
    assert!(!output.stdout.is_empty() && output.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn output_failures_preserve_directories_and_unrelated_temporary_neighbors() {
    let fixture = Fixture::new();
    let output = fixture.root.join("result.ll");
    fs::create_dir(&output).expect("existing directory at output path");
    let mut args = arguments(&fixture);
    replace(&mut args, "--output-kind", "llvm-ir");
    args.extend(["--output".into(), output.clone().into_os_string()]);
    expect_status(&invoke(&fixture, &args), 2);
    assert!(output.is_dir(), "failed output must not remove a directory");
    fs::remove_dir(&output).expect("remove test-owned empty directory");
    fixture.write("result.ll.tmp", "unrelated neighboring file");
    expect_status(&invoke(&fixture, &args), 0);
    assert_eq!(
        fs::read_to_string(fixture.root.join("result.ll.tmp")).expect("neighbor preserved"),
        "unrelated neighboring file"
    );
    assert!(
        fs::read_dir(&fixture.root)
            .expect("fixture directory")
            .all(|entry| !entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .starts_with(".cloth-output.")),
        "temporary LLVM output directories remain"
    );
    let previous = fs::read(&output).expect("completed output");
    fixture.write(
        "app/src/Main.co",
        "static func Main() { switch (0) { case 0, 0: {} } }\n",
    );
    let failure = invoke(&fixture, &args);
    expect_status(&failure, 1);
    assert!(String::from_utf8_lossy(&failure.stderr).contains("duplicate switch case"));
    assert_eq!(fs::read(&output).expect("preserved output"), previous);
}

#[cfg(unix)]
#[test]
#[ignore = "requires CLOTHC_UNDER_TEST"]
fn source_links_cannot_escape_and_directory_links_are_not_followed() {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new();
    fixture.write("outside/Bad.co", "invalid source");
    symlink(
        fixture.root.join("outside"),
        fixture.root.join("app/src/linked"),
    )
    .expect("directory symlink");
    expect_status(&invoke(&fixture, &arguments(&fixture)), 0);
    symlink(
        fixture.root.join("outside/Bad.co"),
        fixture.root.join("app/src/Bad.co"),
    )
    .expect("file symlink");
    expect_status(&invoke(&fixture, &arguments(&fixture)), 2);
}
