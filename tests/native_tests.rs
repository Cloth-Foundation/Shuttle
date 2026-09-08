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
use shuttle::standard_library::inject_standard_library;
use support::{
    CHECKED_UPDATES_OUTPUT, Fixture, INTEGER_CONVERSIONS_OUTPUT, NUMERIC_NOTATION_OUTPUT,
    STRUCT_OUTPUT, SWITCH_CASES, SWITCH_MAIN, SWITCH_OUTPUT, TYPED_ERRORS_OUTPUT,
    TYPED_LITERALS_OUTPUT, compiler, expect_status, pair_compiler_with_standard_library, run,
};

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
    whole_project_run_with_arguments(fixture, &[])
}

fn whole_project_run_with_arguments(fixture: &Fixture, program_arguments: &[OsString]) -> Output {
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
    run(Command::new(output).args(program_arguments))
}

#[cfg(unix)]
fn invalid_unicode_argument() -> OsString {
    use std::os::unix::ffi::OsStringExt;

    OsString::from_vec(vec![0xff])
}

#[cfg(windows)]
fn invalid_unicode_argument() -> OsString {
    use std::os::windows::ffi::OsStringExt;

    OsString::from_wide(&[0xd800])
}

fn verify_program_argument_failure_preservation(
    fixture: &Fixture,
    selected: &Path,
    executable: &Path,
) {
    let artifact_bytes = fixture.artifact_bytes("app/target/x86_64/packages");
    let executable_bytes = fs::read(executable).expect("program executable");
    fixture.write(
        "app/src/Main.co",
        "static func Main(string[] arguments) { missing(arguments); }\n",
    );
    let mut failed = fixture.shuttle("run", selected);
    failed.arg("--").arg("must-not-run");
    let failed = run(&mut failed);
    assert!(!failed.status.success());
    assert!(failed.stdout.is_empty());
    assert!(String::from_utf8_lossy(&failed.stderr).contains("unknown name 'missing'"));
    assert_eq!(
        fixture.artifact_bytes("app/target/x86_64/packages"),
        artifact_bytes
    );
    assert_eq!(
        fs::read(executable).expect("preserved program executable"),
        executable_bytes
    );
    let preserved = run(Command::new(executable).arg("preserved"));
    expect_status(&preserved, 1);
    assert_eq!(preserved.stdout, b"1\n9\npreserved\n");
    assert!(preserved.stderr.is_empty());
}

fn verify_throwing_program_argument_entry(selected: &Path) {
    let fixture = Fixture::new();
    fixture.write(
        "app/src/Failure.co",
        r"
error {
  Failure(string message): Error(message) {}
}
",
    );
    fixture.write(
        "app/src/Main.co",
        r"
import Failure;
static func Main(string[] arguments) throws Failure {
  throw Failure(arguments[0]);
}
",
    );
    let mut command = fixture.shuttle("run", selected);
    command.arg("--").arg("boom");
    let output = run(&mut command);
    expect_status(&output, 1);
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8(output.stderr)
            .expect("typed error output")
            .replace("\r\n", "\n"),
        "cloth error: app.Failure: boom\n"
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn enums_preserve_native_and_parallel_package_behavior() {
    let serial = Fixture::enums();
    let parallel = Fixture::enums();
    let selected = compiler();
    let first = run(serial.shuttle("run", &selected).args(["--jobs", "1"]));
    let second = run(parallel.shuttle("run", &selected).args(["--jobs", "4"]));
    expect_status(&first, 0);
    expect_status(&second, 0);
    assert_eq!(first.stdout, b"data-models.State.Ready\ndata-models.State.ready\ndata-models.State._Done\ntrue\ndata-models.State\n");
    assert_eq!(first.stdout, second.stdout);
    assert!(first.stderr.is_empty() && second.stderr.is_empty());
    for package in ["app", "data-models", "foundation", "tools"] {
        let path = format!("app/target/x86_64/packages/{package}.cpa");
        assert_eq!(
            fs::read(serial.root.join(&path)).unwrap(),
            fs::read(parallel.root.join(&path)).unwrap()
        );
    }
    let whole = whole_project_run(&serial);
    expect_status(&whole, 0);
    assert_eq!(first.stdout, whole.stdout);
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn nullable_values_cross_whole_separate_and_source_free_packages() {
    let serial = Fixture::nullable_values();
    let parallel = Fixture::nullable_values();
    parallel.reverse_dependencies();
    let selected = compiler();
    let first = run(serial.shuttle("run", &selected).args(["--jobs", "1"]));
    let second = run(parallel.shuttle("run", &selected).args(["--jobs", "4"]));
    expect_status(&first, 0);
    expect_status(&second, 0);
    assert_eq!(first.stdout, support::NULLABLE_VALUES_OUTPUT);
    assert_eq!(second.stdout, first.stdout);
    assert!(first.stderr.is_empty() && second.stderr.is_empty());
    assert_eq!(
        serial.artifact_bytes("app/target/x86_64/packages"),
        parallel.artifact_bytes("app/target/x86_64/packages")
    );

    let artifacts = serial.artifact_bytes("app/target/x86_64/packages");
    let executable = serial.root.join(format!(
        "app/target/x86_64/app{}",
        std::env::consts::EXE_SUFFIX
    ));
    let executable_bytes = fs::read(&executable).expect("nullable executable");
    let maybe_path = serial.root.join("models/src/Maybe.co");
    let valid_maybe = fs::read_to_string(&maybe_path).expect("nullable source");
    serial.write(
        "models/src/Maybe.co",
        "import Payload;\nint32?? invalid;\nstatic func Echo(Payload? value): Payload? { return value; }\n",
    );
    let failed = run(&mut serial.shuttle("run", &selected));
    expect_status(&failed, 1);
    assert!(failed.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&failed.stderr)
            .contains("nullable qualification cannot be repeated")
    );
    assert_eq!(
        artifacts,
        serial.artifact_bytes("app/target/x86_64/packages")
    );
    assert_eq!(
        executable_bytes,
        fs::read(&executable).expect("preserved nullable executable")
    );
    let preserved = run(&mut Command::new(&executable));
    expect_status(&preserved, 0);
    assert_eq!(preserved.stdout, support::NULLABLE_VALUES_OUTPUT);
    assert!(preserved.stderr.is_empty());
    serial.write("models/src/Maybe.co", &valid_maybe);

    let whole = whole_project_run(&serial);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, support::NULLABLE_VALUES_OUTPUT);
    assert!(whole.stderr.is_empty());
    let source_free = source_free_run(&serial);
    expect_status(&source_free, 0);
    assert_eq!(source_free.stdout, support::NULLABLE_VALUES_OUTPUT);
    assert!(source_free.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn structs_preserve_relocated_serial_parallel_artifacts() {
    let serial = Fixture::structs();
    let parallel = Fixture::structs();
    let selected = compiler();
    let first = run(serial.shuttle("run", &selected).args(["--jobs", "1"]));
    // Cross a PE timestamp boundary as well as changing path and scheduling.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let second = run(parallel.shuttle("run", &selected).args(["--jobs", "4"]));
    expect_status(&first, 0);
    expect_status(&second, 0);
    assert_eq!(first.stdout, STRUCT_OUTPUT);
    assert_eq!(second.stdout, first.stdout);
    assert!(first.stderr.is_empty() && second.stderr.is_empty());
    assert_eq!(
        serial.artifact_bytes("app/target/x86_64/packages"),
        parallel.artifact_bytes("app/target/x86_64/packages")
    );
    let executable = format!("app/target/x86_64/app{}", std::env::consts::EXE_SUFFIX);
    assert_eq!(
        fs::read(serial.root.join(&executable)).expect("serial executable"),
        fs::read(parallel.root.join(&executable)).expect("parallel executable")
    );
    let whole = whole_project_run(&serial);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, STRUCT_OUTPUT);
    assert!(whole.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn checked_updates_cross_whole_separate_and_source_free_packages() {
    let serial = Fixture::checked_updates();
    let parallel = Fixture::checked_updates();
    parallel.reverse_dependencies();
    let selected = compiler();
    let first = run(serial.shuttle("run", &selected).args(["--jobs", "1"]));
    let second = run(parallel.shuttle("run", &selected).args(["--jobs", "4"]));
    expect_status(&first, 0);
    expect_status(&second, 0);
    assert_eq!(first.stdout, CHECKED_UPDATES_OUTPUT);
    assert_eq!(second.stdout, first.stdout);
    assert!(first.stderr.is_empty() && second.stderr.is_empty());
    assert_eq!(
        serial.artifact_bytes("app/target/x86_64/packages"),
        parallel.artifact_bytes("app/target/x86_64/packages")
    );

    let whole = whole_project_run(&serial);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, CHECKED_UPDATES_OUTPUT);
    assert!(whole.stderr.is_empty());
    let source_free = source_free_run(&serial);
    expect_status(&source_free, 0);
    assert_eq!(source_free.stdout, CHECKED_UPDATES_OUTPUT);
    assert!(source_free.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn checked_update_edits_invalidate_only_affected_packages() {
    let fixture = Fixture::checked_updates();
    let selected = compiler();
    expect_status(&run(&mut fixture.shuttle("build", &selected)), 0);
    let directory = "app/target/x86_64/packages";
    let previous = fixture.artifact_bytes(directory);
    let executable = fixture.root.join(format!(
        "app/target/x86_64/app{}",
        std::env::consts::EXE_SUFFIX
    ));
    let model_path = fixture.root.join("models/src/User.co");
    let model = fs::read_to_string(&model_path).expect("checked model source");
    let equivalent = model.replace("value *= 2;", "value += value;");
    assert_ne!(model, equivalent);
    fixture.write("models/src/User.co", &equivalent);
    let rebuilt = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&rebuilt, 0);
    assert_eq!(rebuilt.stdout, CHECKED_UPDATES_OUTPUT);
    let progress = String::from_utf8_lossy(&rebuilt.stderr);
    let current = fixture.artifact_bytes(directory);
    for package in ["data-models", "app"] {
        assert!(
            progress.contains(&format!("shuttle: compiling {package} ")),
            "{progress}"
        );
        assert_ne!(previous[package], current[package]);
    }
    for package in ["foundation", "tools"] {
        assert!(
            progress.contains(&format!("shuttle: reusing {package} ")),
            "{progress}"
        );
        assert_eq!(previous[package], current[package]);
    }
    let completed = fs::read(&executable).expect("completed executable");

    let invalid = equivalent.replace("value += Record.Value();", "value += Missing;");
    assert_ne!(equivalent, invalid);
    fixture.write("models/src/User.co", &invalid);
    let failed = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&failed, 1);
    assert!(
        failed.stdout.is_empty(),
        "failed build ran a stale executable"
    );
    assert!(String::from_utf8_lossy(&failed.stderr).contains("Missing"));
    assert_eq!(
        fs::read(&executable).expect("preserved executable"),
        completed
    );
    assert_eq!(fixture.artifact_bytes(directory), current);
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn integer_conversions_cross_whole_separate_and_source_free_packages() {
    let serial = Fixture::integer_conversions();
    let parallel = Fixture::integer_conversions();
    parallel.reverse_dependencies();
    let selected = compiler();
    let first = run(serial.shuttle("run", &selected).args(["--jobs", "1"]));
    let second = run(parallel.shuttle("run", &selected).args(["--jobs", "4"]));
    expect_status(&first, 0);
    expect_status(&second, 0);
    assert_eq!(first.stdout, INTEGER_CONVERSIONS_OUTPUT);
    assert_eq!(second.stdout, first.stdout);
    assert!(first.stderr.is_empty() && second.stderr.is_empty());
    assert_eq!(
        serial.artifact_bytes("app/target/x86_64/packages"),
        parallel.artifact_bytes("app/target/x86_64/packages")
    );

    let whole = whole_project_run(&serial);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, INTEGER_CONVERSIONS_OUTPUT);
    assert!(whole.stderr.is_empty());
    let source_free = source_free_run(&serial);
    expect_status(&source_free, 0);
    assert_eq!(source_free.stdout, INTEGER_CONVERSIONS_OUTPUT);
    assert!(source_free.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn integer_conversion_edits_preserve_outputs_and_reuse_unaffected_packages() {
    let fixture = Fixture::integer_conversions();
    let selected = compiler();
    expect_status(&run(&mut fixture.shuttle("build", &selected)), 0);
    let directory = "app/target/x86_64/packages";
    let previous = fixture.artifact_bytes(directory);
    let executable = fixture.root.join(format!(
        "app/target/x86_64/app{}",
        std::env::consts::EXE_SUFFIX
    ));
    let model_path = fixture.root.join("models/src/User.co");
    let model = fs::read_to_string(&model_path).expect("conversion model source");
    let saturated = model.replace("uint16::wrap(value)", "uint16::sat(value)");
    assert_ne!(model, saturated);
    fixture.write("models/src/User.co", &saturated);

    let rebuilt = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&rebuilt, 0);
    assert_eq!(rebuilt.stdout, b"65535\n65535\n0\n44\n255\n");
    let progress = String::from_utf8_lossy(&rebuilt.stderr);
    let current = fixture.artifact_bytes(directory);
    for package in ["data-models", "app"] {
        assert!(
            progress.contains(&format!("shuttle: compiling {package} ")),
            "{progress}"
        );
        assert_ne!(previous[package], current[package]);
    }
    for package in ["foundation", "tools"] {
        assert!(
            progress.contains(&format!("shuttle: reusing {package} ")),
            "{progress}"
        );
        assert_eq!(previous[package], current[package]);
    }
    let completed = fs::read(&executable).expect("completed executable");

    let invalid = saturated.replace("uint16::sat(value)", "uint16::clip(value)");
    assert_ne!(saturated, invalid);
    fixture.write("models/src/User.co", &invalid);
    let failed = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&failed, 1);
    assert!(
        failed.stdout.is_empty(),
        "failed build ran a stale executable"
    );
    assert!(
        String::from_utf8_lossy(&failed.stderr)
            .contains("integer type 'uint16' has no conversion mode 'clip'")
    );
    assert_eq!(
        fs::read(&executable).expect("preserved executable"),
        completed
    );
    assert_eq!(fixture.artifact_bytes(directory), current);
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn typed_literals_cross_whole_separate_and_source_free_packages() {
    let serial = Fixture::typed_literals();
    let parallel = Fixture::typed_literals();
    parallel.reverse_dependencies();
    let selected = compiler();
    let first = run(serial.shuttle("run", &selected).args(["--jobs", "1"]));
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let second = run(parallel.shuttle("run", &selected).args(["--jobs", "4"]));
    expect_status(&first, 0);
    expect_status(&second, 0);
    assert_eq!(first.stdout, TYPED_LITERALS_OUTPUT);
    assert_eq!(second.stdout, first.stdout);
    assert!(first.stderr.is_empty() && second.stderr.is_empty());
    assert_eq!(
        serial.artifact_bytes("app/target/x86_64/packages"),
        parallel.artifact_bytes("app/target/x86_64/packages")
    );
    let executable = format!("app/target/x86_64/app{}", std::env::consts::EXE_SUFFIX);
    assert_eq!(
        fs::read(serial.root.join(&executable)).expect("serial executable"),
        fs::read(parallel.root.join(&executable)).expect("parallel executable")
    );

    let whole = whole_project_run(&serial);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, TYPED_LITERALS_OUTPUT);
    assert!(whole.stderr.is_empty());
    let source_free = source_free_run(&serial);
    expect_status(&source_free, 0);
    assert_eq!(source_free.stdout, TYPED_LITERALS_OUTPUT);
    assert!(source_free.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn typed_literal_edits_invalidate_consumers_and_preserve_outputs() {
    let fixture = Fixture::typed_literals();
    let selected = compiler();
    expect_status(&run(&mut fixture.shuttle("build", &selected)), 0);
    let directory = "app/target/x86_64/packages";
    let previous = fixture.artifact_bytes(directory);
    let executable = fixture.root.join(format!(
        "app/target/x86_64/app{}",
        std::env::consts::EXE_SUFFIX
    ));
    let model_path = fixture.root.join("models/src/User.co");
    let model = fs::read_to_string(&model_path).expect("typed literal model source");
    let changed = model.replace("value + 2i8", "value + 3i8");
    assert_ne!(model, changed);
    fixture.write("models/src/User.co", &changed);

    let rebuilt = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&rebuilt, 0);
    assert_eq!(
        rebuilt.stdout,
        b"7\n43\n18446744073709551615\n0.5\n44\n0\n8\n"
    );
    let progress = String::from_utf8_lossy(&rebuilt.stderr);
    let current = fixture.artifact_bytes(directory);
    for package in ["data-models", "app"] {
        assert!(
            progress.contains(&format!("shuttle: compiling {package} ")),
            "{progress}"
        );
        assert_ne!(previous[package], current[package]);
    }
    for package in ["foundation", "tools"] {
        assert!(
            progress.contains(&format!("shuttle: reusing {package} ")),
            "{progress}"
        );
        assert_eq!(previous[package], current[package]);
    }
    let completed = fs::read(&executable).expect("completed executable");

    let invalid = changed.replace("value + 3i8", "value + 3i7");
    assert_ne!(changed, invalid);
    fixture.write("models/src/User.co", &invalid);
    let failed = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&failed, 1);
    assert!(
        failed.stdout.is_empty(),
        "failed build ran a stale executable"
    );
    assert!(String::from_utf8_lossy(&failed.stderr).contains("invalid numeric suffix 'i7'"));
    assert_eq!(
        fs::read(&executable).expect("preserved executable"),
        completed
    );
    assert_eq!(fixture.artifact_bytes(directory), current);
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn numeric_notation_crosses_whole_separate_and_source_free_packages() {
    let serial = Fixture::numeric_notation();
    let parallel = Fixture::numeric_notation();
    parallel.reverse_dependencies();
    let selected = compiler();
    let first = run(serial.shuttle("run", &selected).args(["--jobs", "1"]));
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let second = run(parallel.shuttle("run", &selected).args(["--jobs", "4"]));
    expect_status(&first, 0);
    expect_status(&second, 0);
    assert_eq!(first.stdout, NUMERIC_NOTATION_OUTPUT);
    assert_eq!(second.stdout, first.stdout);
    assert!(first.stderr.is_empty() && second.stderr.is_empty());
    assert_eq!(
        serial.artifact_bytes("app/target/x86_64/packages"),
        parallel.artifact_bytes("app/target/x86_64/packages")
    );
    let executable = format!("app/target/x86_64/app{}", std::env::consts::EXE_SUFFIX);
    assert_eq!(
        fs::read(serial.root.join(&executable)).expect("serial executable"),
        fs::read(parallel.root.join(&executable)).expect("parallel executable")
    );

    let whole = whole_project_run(&serial);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, NUMERIC_NOTATION_OUTPUT);
    assert!(whole.stderr.is_empty());
    let source_free = source_free_run(&serial);
    expect_status(&source_free, 0);
    assert_eq!(source_free.stdout, NUMERIC_NOTATION_OUTPUT);
    assert!(source_free.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn typed_errors_cross_whole_separate_and_source_free_packages() {
    let serial = Fixture::typed_errors();
    let parallel = Fixture::typed_errors();
    parallel.reverse_dependencies();
    let selected = compiler();
    let first = run(serial.shuttle("run", &selected).args(["--jobs", "1"]));
    let second = run(parallel.shuttle("run", &selected).args(["--jobs", "4"]));
    expect_status(&first, 0);
    expect_status(&second, 0);
    assert_eq!(first.stdout, TYPED_ERRORS_OUTPUT);
    assert_eq!(second.stdout, first.stdout);
    assert!(first.stderr.is_empty() && second.stderr.is_empty());
    assert_eq!(
        serial.artifact_bytes("app/target/x86_64/packages"),
        parallel.artifact_bytes("app/target/x86_64/packages")
    );

    let whole = whole_project_run(&serial);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, TYPED_ERRORS_OUTPUT);
    assert!(whole.stderr.is_empty());
    let source_free = source_free_run_with_dependencies(
        &serial,
        &[
            ("models", "data-models"),
            ("tools", "tools"),
            ("foundation", "foundation"),
        ],
    );
    expect_status(&source_free, 0);
    assert_eq!(source_free.stdout, TYPED_ERRORS_OUTPUT);
    assert!(source_free.stderr.is_empty());

    let failure = Fixture::typed_errors();
    failure.write(
        "app/src/Main.co",
        r"
import foundation::InvalidInput;
import models::Calculator;
static func Main(): int32 throws InvalidInput, DivisionByZero {
  println(Calculator.Divide(42, 0));
  return 0;
}
",
    );
    let failed = run(&mut failure.shuttle("run", &selected));
    expect_status(&failed, 1);
    assert!(failed.stdout.is_empty());
    let expected_error = if cfg!(windows) {
        b"cloth error: DivisionByZero\r\n".as_slice()
    } else {
        b"cloth error: DivisionByZero\n".as_slice()
    };
    assert_eq!(failed.stderr, expected_error);
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn typed_error_edits_invalidate_consumers_and_preserve_outputs() {
    let fixture = Fixture::typed_errors();
    let selected = compiler();
    expect_status(&run(&mut fixture.shuttle("build", &selected)), 0);
    let directory = "app/target/x86_64/packages";
    let previous = fixture.artifact_bytes(directory);
    let executable = fixture.root.join(format!(
        "app/target/x86_64/app{}",
        std::env::consts::EXE_SUFFIX
    ));

    let calculator_path = fixture.root.join("models/src/Calculator.co");
    let calculator = fs::read_to_string(&calculator_path).expect("typed error consumer source");
    let changed_calculator =
        calculator.clone() + "\nstatic func Validate() throws InvalidInput {}\n";
    fixture.write("models/src/Calculator.co", &changed_calculator);

    let rebuilt = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&rebuilt, 0);
    assert_eq!(rebuilt.stdout, TYPED_ERRORS_OUTPUT);
    let progress = String::from_utf8_lossy(&rebuilt.stderr);
    let current = fixture.artifact_bytes(directory);
    for package in ["data-models", "app"] {
        assert!(
            progress.contains(&format!("shuttle: compiling {package} ")),
            "{progress}"
        );
        assert_ne!(previous[package], current[package]);
    }
    for package in ["foundation", "tools"] {
        assert!(
            progress.contains(&format!("shuttle: reusing {package} ")),
            "{progress}"
        );
        assert_eq!(previous[package], current[package]);
    }
    let completed = fs::read(&executable).expect("completed executable");

    let invalid = changed_calculator.replace("throws InvalidInput, DivisionByZero", "throws int32");
    assert_ne!(changed_calculator, invalid);
    fixture.write("models/src/Calculator.co", &invalid);
    let failed = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&failed, 1);
    assert!(
        failed.stdout.is_empty(),
        "failed build ran a stale executable"
    );
    assert!(String::from_utf8_lossy(&failed.stderr).contains("is not a non-null error"));
    assert_eq!(
        fs::read(&executable).expect("preserved executable"),
        completed
    );
    assert_eq!(fixture.artifact_bytes(directory), current);
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn numeric_notation_edits_invalidate_consumers_and_preserve_outputs() {
    let fixture = Fixture::numeric_notation();
    let selected = compiler();
    expect_status(&run(&mut fixture.shuttle("build", &selected)), 0);
    let directory = "app/target/x86_64/packages";
    let previous = fixture.artifact_bytes(directory);
    let executable = fixture.root.join(format!(
        "app/target/x86_64/app{}",
        std::env::consts::EXE_SUFFIX
    ));
    let model_path = fixture.root.join("models/src/User.co");
    let model = fs::read_to_string(&model_path).expect("numeric notation model source");
    let changed = model.replace("value + 0o2i8", "value + 0o3i8");
    assert_ne!(model, changed);
    fixture.write("models/src/User.co", &changed);

    let rebuilt = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&rebuilt, 0);
    assert_eq!(
        rebuilt.stdout,
        b"240\n43\n18446744073709551615\n125\n44\n0\n8\n"
    );
    let progress = String::from_utf8_lossy(&rebuilt.stderr);
    let current = fixture.artifact_bytes(directory);
    for package in ["data-models", "app"] {
        assert!(
            progress.contains(&format!("shuttle: compiling {package} ")),
            "{progress}"
        );
        assert_ne!(previous[package], current[package]);
    }
    for package in ["foundation", "tools"] {
        assert!(
            progress.contains(&format!("shuttle: reusing {package} ")),
            "{progress}"
        );
        assert_eq!(previous[package], current[package]);
    }
    let completed = fs::read(&executable).expect("completed executable");

    let invalid = changed.replace("value + 0o3i8", "value + 0o8i8");
    assert_ne!(changed, invalid);
    fixture.write("models/src/User.co", &invalid);
    let failed = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&failed, 1);
    assert!(
        failed.stdout.is_empty(),
        "failed build ran a stale executable"
    );
    assert!(String::from_utf8_lossy(&failed.stderr).contains("invalid digit in base-8"));
    assert_eq!(
        fs::read(&executable).expect("preserved executable"),
        completed
    );
    assert_eq!(fixture.artifact_bytes(directory), current);
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn struct_edits_invalidate_native_consumers_and_preserve_independent_reuse() {
    let fixture = Fixture::structs();
    let selected = compiler();
    expect_status(&run(&mut fixture.shuttle("build", &selected)), 0);
    let directory = "app/target/x86_64/packages";
    let mut previous = fixture.artifact_bytes(directory);
    for layout in [true, false] {
        fixture.edit_struct(layout);
        let changed = run(&mut fixture.visible_shuttle("run", &selected));
        expect_status(&changed, 0);
        assert_eq!(changed.stdout, STRUCT_OUTPUT);
        let progress = String::from_utf8_lossy(&changed.stderr);
        let current = fixture.artifact_bytes(directory);
        for package in ["data-models", "app"] {
            assert!(
                progress.contains(&format!("shuttle: compiling {package} ")),
                "{progress}"
            );
            assert_ne!(
                previous[package], current[package],
                "stale {package} artifact"
            );
        }
        for package in ["cloth", "foundation", "tools"] {
            assert!(
                progress.contains(&format!("shuttle: reusing {package} ")),
                "{progress}"
            );
            assert_eq!(
                previous[package], current[package],
                "unrelated {package} changed"
            );
        }
        let unchanged = run(&mut fixture.visible_shuttle("run", &selected));
        expect_status(&unchanged, 0);
        assert_eq!(unchanged.stdout, STRUCT_OUTPUT);
        let progress = String::from_utf8_lossy(&unchanged.stderr);
        assert_eq!(
            progress.matches("shuttle: reusing ").count(),
            5,
            "{progress}"
        );
        assert!(!progress.contains("shuttle: compiling "), "{progress}");
        assert_eq!(current, fixture.artifact_bytes(directory));
        previous = current;
    }
    let whole = whole_project_run(&fixture);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, STRUCT_OUTPUT);
    assert!(whole.stderr.is_empty());
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

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn structs_link_and_execute_without_dependency_sources() {
    let fixture = Fixture::structs();
    let selected = compiler();
    let separate = run(&mut fixture.shuttle("run", &selected));
    expect_status(&separate, 0);
    let expected = STRUCT_OUTPUT;
    assert_eq!(separate.stdout, expected);
    assert!(separate.stderr.is_empty());
    let whole = whole_project_run(&fixture);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, separate.stdout);

    let executed = source_free_run(&fixture);
    expect_status(&executed, 0);
    assert_eq!(executed.stdout, expected);
    assert!(executed.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn constants_link_and_execute_without_dependency_sources() {
    let fixture = Fixture::constants();
    let separate = run(&mut fixture.shuttle("run", &compiler()));
    let expected = b"42\n-128\n-9223372036854775808\n18446744073709551615\ntrue\ntrue\ntrue\ntrue\ntrue\n2\ntrue\ntrue\ntrue\ntrue\nminimum\nmaximum\nready\n";
    expect_status(&separate, 0);
    assert_eq!(separate.stdout, expected);
    assert!(separate.stderr.is_empty());
    let whole = whole_project_run(&fixture);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, expected);
    let executed = source_free_run(&fixture);
    expect_status(&executed, 0);
    assert_eq!(executed.stdout, expected);
    assert!(executed.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn computed_constants_preserve_relocated_parallel_native_artifacts() {
    let serial = Fixture::constants();
    let parallel = Fixture::constants();
    parallel.reverse_dependencies();
    let selected = compiler();
    let first = run(serial.shuttle("run", &selected).args(["--jobs", "1"]));
    expect_status(&first, 0);
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let second = run(parallel.shuttle("run", &selected).args(["--jobs", "4"]));
    expect_status(&second, 0);
    assert_eq!(first.stdout, second.stdout);
    assert!(first.stderr.is_empty() && second.stderr.is_empty());
    assert_eq!(
        serial.artifact_bytes("app/target/x86_64/packages"),
        parallel.artifact_bytes("app/target/x86_64/packages")
    );
    let executable = format!("app/target/x86_64/app{}", std::env::consts::EXE_SUFFIX);
    assert_eq!(
        fs::read(serial.root.join(&executable)).unwrap(),
        fs::read(parallel.root.join(&executable)).unwrap()
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn computed_constant_edits_invalidate_consumers_even_when_value_is_unchanged() {
    let selected = compiler();
    for (path, before, after, expected, upstream) in [
        (
            "models/src/Constants.co",
            "hidden * 2",
            "hidden + hidden",
            &b"43\nmatch\n"[..],
            false,
        ),
        (
            "models/src/Constants.co",
            "hidden = Input.Base",
            "hidden = Input.Base + 1",
            &b"45\nother\n"[..],
            false,
        ),
        (
            "core/src/Seed.co",
            "3 * 7",
            "3 * 8",
            &b"49\nother\n"[..],
            true,
        ),
    ] {
        let fixture = Fixture::constants();
        fixture.write(
            "app/src/Main.co",
            r#"
import models::Constants as Values;
static final int64 Next = Values.Answer + 1;
static func Main() {
  println(Next);
  switch (int64(42)) {
    case Values.Answer: { println("match"); }
    default: { println("other"); }
  }
}
"#,
        );
        let first = run(&mut fixture.shuttle("run", &selected));
        expect_status(&first, 0);
        assert_eq!(first.stdout, b"43\nmatch\n");
        let directory = "app/target/x86_64/packages";
        let previous = fixture.artifact_bytes(directory);
        let mut stale = artifact_records(&fixture)
            .into_iter()
            .find(|record| record.name == "app")
            .unwrap();
        let snapshot = fixture.root.join("previous-app.cpa");
        fs::copy(&stale.path, &snapshot).unwrap();
        stale.path = snapshot;
        let source = fs::read_to_string(fixture.root.join(path)).unwrap();
        let changed = source.replace(before, after);
        assert_ne!(source, changed);
        fixture.write(path, &changed);
        let rebuilt = run(&mut fixture.visible_shuttle("run", &selected));
        expect_status(&rebuilt, 0);
        assert_eq!(rebuilt.stdout, expected);
        let progress = String::from_utf8_lossy(&rebuilt.stderr);
        let current = fixture.artifact_bytes(directory);
        for package in ["cloth", "foundation", "tools", "data-models", "app"] {
            let changed =
                package != "cloth" && (upstream || matches!(package, "data-models" | "app"));
            let action = if changed { "compiling" } else { "reusing" };
            assert!(
                progress.contains(&format!("shuttle: {action} {package} ")),
                "{progress}"
            );
            assert_eq!(previous[package] != current[package], changed, "{package}");
        }
        let warm = run(&mut fixture.visible_shuttle("run", &selected));
        expect_status(&warm, 0);
        assert_eq!(warm.stdout, expected);
        assert_eq!(
            String::from_utf8_lossy(&warm.stderr)
                .matches("shuttle: reusing ")
                .count(),
            5
        );
        assert_eq!(current, fixture.artifact_bytes(directory));
        expect_stale_link_rejected(&fixture, stale);
        let whole = whole_project_run(&fixture);
        expect_status(&whole, 0);
        assert_eq!(whole.stdout, expected);
        let source_free = source_free_run(&fixture);
        expect_status(&source_free, 0);
        assert_eq!(source_free.stdout, expected);
        assert!(source_free.stderr.is_empty());
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn computed_constant_failures_never_replace_outputs_or_run_stale_programs() {
    let selected = compiler();
    for (before, after, diagnostic, producer_valid) in [
        ("hidden = Input.Base", "hidden = 1 / 0", "by zero", false),
        (
            "hidden = Input.Base",
            "hidden = hidden",
            "cyclic static constant",
            false,
        ),
        (
            "Minimum = int8(-128)",
            "Minimum = int8(-(-(-128)))",
            "overflow",
            false,
        ),
        (
            "hidden = Input.Base",
            "hidden = Input.Base + 1",
            "duplicate switch case",
            true,
        ),
        (
            "initial = State.Ready",
            "initial = State.Done",
            "duplicate switch case",
            true,
        ),
    ] {
        let fixture = Fixture::constants();
        let main = fs::read_to_string(fixture.root.join("app/src/Main.co")).unwrap();
        fixture.write("app/src/Main.co", &(main + "\nstatic func Check(int64 n) { switch(n) { case Values.Answer: {} case 44: {} default: {} } }\n"));
        expect_status(&run(&mut fixture.shuttle("build", &selected)), 0);
        let directory = "app/target/x86_64/packages";
        let previous = fixture.artifact_bytes(directory);
        let executable = fixture.root.join(format!(
            "app/target/x86_64/app{}",
            std::env::consts::EXE_SUFFIX
        ));
        let completed = fs::read(&executable).unwrap();
        let path = "models/src/Constants.co";
        let source = fs::read_to_string(fixture.root.join(path)).unwrap();
        let changed = source.replace(before, after);
        assert_ne!(source, changed);
        fixture.write(path, &changed);
        let failed = run(&mut fixture.shuttle("run", &selected));
        expect_status(&failed, 1);
        assert!(
            failed.stdout.is_empty(),
            "failed rebuild ran a stale program"
        );
        let detail = String::from_utf8_lossy(&failed.stderr);
        assert!(detail.contains(diagnostic), "{detail}");
        assert_eq!(completed, fs::read(&executable).unwrap());
        let current = fixture.artifact_bytes(directory);
        for package in ["app", "foundation", "tools"] {
            assert_eq!(previous[package], current[package]);
        }
        assert_eq!(
            previous["data-models"] != current["data-models"],
            producer_valid
        );
    }
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn switches_link_and_execute_without_dependency_sources() {
    let fixture = Fixture::switches();
    let separate = run(&mut fixture.shuttle("run", &compiler()));
    let expected = SWITCH_OUTPUT;
    expect_status(&separate, 0);
    assert_eq!(separate.stdout, expected);
    assert!(separate.stderr.is_empty());
    let whole = whole_project_run(&fixture);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, expected);
    let executed = source_free_run(&fixture);
    expect_status(&executed, 0);
    assert_eq!(executed.stdout, expected);
    assert!(executed.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn switch_native_artifacts_are_relocated_and_order_independent() {
    let serial = Fixture::switches();
    let parallel = Fixture::switches();
    parallel.reverse_dependencies();
    let selected = compiler();
    let first = run(serial.shuttle("run", &selected).args(["--jobs", "1"]));
    expect_status(&first, 0);
    assert_eq!(first.stdout, SWITCH_OUTPUT);
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let second = run(parallel.shuttle("run", &selected).args(["--jobs", "4"]));
    expect_status(&second, 0);
    assert_eq!(first.stdout, second.stdout);
    assert!(first.stderr.is_empty() && second.stderr.is_empty());
    assert_eq!(
        serial.artifact_bytes("app/target/x86_64/packages"),
        parallel.artifact_bytes("app/target/x86_64/packages")
    );
    let executable = format!("app/target/x86_64/app{}", std::env::consts::EXE_SUFFIX);
    assert_eq!(
        fs::read(serial.root.join(&executable)).unwrap(),
        fs::read(parallel.root.join(&executable)).unwrap()
    );
    let whole = whole_project_run(&serial);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, SWITCH_OUTPUT);
    assert!(whole.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn switch_edits_invalidate_native_consumers_and_reject_stale_links() {
    let selected = compiler();
    let constant_main = SWITCH_MAIN
        .replace(
            "case Constants.Initial, Status._Done:",
            "case Status.ready, Status._Done:",
        )
        .replace(
            "switch (state) { case Status.Ready:",
            "switch (state) { case Constants.Initial:",
        );
    for (cases, initial, small, source, baseline, expected) in [
        (
            &["_Done", "ready", "Ready"][..],
            "ready",
            7,
            SWITCH_MAIN,
            SWITCH_OUTPUT,
            &b"done\nfallback\nactive\nfallback\nready\nready\nsmall\nother\nmaximum\n"[..],
        ),
        (
            SWITCH_CASES,
            "ready",
            9,
            SWITCH_MAIN,
            SWITCH_OUTPUT,
            &b"ready\nready\nactive\nfallback\ndone\nfallback\nother\nsmall\nmaximum\n"[..],
        ),
        (
            SWITCH_CASES,
            "Ready",
            7,
            constant_main.as_str(),
            &b"ready\nfallback\nactive\nready\ndone\nfallback\nsmall\nother\nmaximum\n"[..],
            SWITCH_OUTPUT,
        ),
    ] {
        let fixture = Fixture::switches();
        fixture.write("app/src/Main.co", source);
        let first = run(&mut fixture.shuttle("run", &selected));
        expect_status(&first, 0);
        assert_eq!(first.stdout, baseline);
        let directory = "app/target/x86_64/packages";
        let previous = fixture.artifact_bytes(directory);
        let old_records = artifact_records(&fixture);
        let mut stale_app = old_records
            .iter()
            .find(|artifact| artifact.name == "app")
            .unwrap()
            .clone();
        let snapshot = fixture.root.join("previous-app.cpa");
        fs::copy(&stale_app.path, &snapshot).expect("snapshot old consumer");
        stale_app.path = snapshot;

        fixture.write_switch_model(cases, initial, small);
        let changed = run(&mut fixture.visible_shuttle("run", &selected));
        expect_status(&changed, 0);
        assert_eq!(
            changed.stdout, expected,
            "edited labels/tags changed meaning"
        );
        let current = fixture.artifact_bytes(directory);
        let progress = String::from_utf8_lossy(&changed.stderr);
        for package in ["data-models", "app"] {
            assert!(
                progress.contains(&format!("shuttle: compiling {package} ")),
                "{progress}"
            );
            assert_ne!(current[package], previous[package], "stale {package}");
        }
        for package in ["cloth", "foundation", "tools"] {
            assert!(
                progress.contains(&format!("shuttle: reusing {package} ")),
                "{progress}"
            );
            assert_eq!(
                current[package], previous[package],
                "unrelated {package} changed"
            );
        }
        let unchanged = run(&mut fixture.visible_shuttle("run", &selected));
        expect_status(&unchanged, 0);
        assert_eq!(unchanged.stdout, expected);
        let progress = String::from_utf8_lossy(&unchanged.stderr);
        assert_eq!(
            progress.matches("shuttle: reusing ").count(),
            5,
            "{progress}"
        );
        assert!(!progress.contains("shuttle: compiling "));
        assert_eq!(current, fixture.artifact_bytes(directory));

        expect_stale_link_rejected(&fixture, stale_app);
        let whole = whole_project_run(&fixture);
        expect_status(&whole, 0);
        assert_eq!(whole.stdout, expected);
        let source_free = source_free_run(&fixture);
        expect_status(&source_free, 0);
        assert_eq!(source_free.stdout, expected);
        assert!(source_free.stderr.is_empty());
    }
}

fn expect_stale_link_rejected(fixture: &Fixture, stale_app: ArtifactRecord) {
    let mut mixed = artifact_records(fixture);
    let app_index = mixed
        .iter()
        .position(|artifact| artifact.name == "app")
        .unwrap();
    mixed[app_index] = stale_app;
    let executable = fixture.root.join(format!(
        "app/target/x86_64/app{}",
        std::env::consts::EXE_SUFFIX
    ));
    let completed = fs::read(&executable).expect("completed executable");
    let rejected = invoke_link(fixture, &executable, &mixed);
    expect_status(&rejected, 2);
    assert!(rejected.stdout.is_empty());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("dependency"));
    assert_eq!(
        fs::read(&executable).expect("preserved executable"),
        completed
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn switch_coverage_failures_preserve_outputs_and_default_accepts_added_cases() {
    let fixture = Fixture::switches();
    let selected = compiler();
    let first = run(&mut fixture.shuttle("run", &selected));
    expect_status(&first, 0);
    assert_eq!(first.stdout, SWITCH_OUTPUT);
    let directory = "app/target/x86_64/packages";
    let previous = fixture.artifact_bytes(directory);
    let executable = fixture.root.join(format!(
        "app/target/x86_64/app{}",
        std::env::consts::EXE_SUFFIX
    ));
    let completed = fs::read(&executable).expect("completed executable");
    for (cases, initial, expected) in [
        (&["Ready", "ready"][..], "ready", "has no case '_Done'"),
        (
            &["Ready", "ready", "Finished"][..],
            "ready",
            "has no case '_Done'",
        ),
        (SWITCH_CASES, "Ready", "duplicate switch case"),
        (
            &["Ready", "ready", "_Done", "Added"][..],
            "ready",
            "missing cases: Added",
        ),
    ] {
        fixture.write_switch_model(cases, initial, 7);
        let failed = run(&mut fixture.visible_shuttle("run", &selected));
        expect_status(&failed, 1);
        assert!(
            failed.stdout.is_empty(),
            "failed rebuild ran a stale executable"
        );
        assert!(String::from_utf8_lossy(&failed.stderr).contains(expected));
        assert_eq!(
            fs::read(&executable).expect("preserved executable"),
            completed
        );
        let current = fixture.artifact_bytes(directory);
        for package in ["app", "foundation", "tools"] {
            assert_eq!(current[package], previous[package]);
        }
        assert_ne!(current["data-models"], previous["data-models"]);
    }
    let fallback = SWITCH_MAIN.replace(
        "case Constants.Initial, Status._Done: { return Constants.Name(state); }",
        "case Constants.Initial, Status._Done: { return Constants.Name(state); }\n    default: { return \"new\"; }",
    );
    assert_ne!(fallback, SWITCH_MAIN);
    fixture.write("app/src/Main.co", &fallback);
    let expected =
        b"ready\nready\nactive\nfallback\ndone\nfallback\nnew\nfallback\nsmall\nother\nmaximum\n";
    let repaired = run(&mut fixture.shuttle("run", &selected));
    expect_status(&repaired, 0);
    assert_eq!(repaired.stdout, expected);
    let whole = whole_project_run(&fixture);
    expect_status(&whole, 0);
    assert_eq!(whole.stdout, expected);
    let source_free = source_free_run(&fixture);
    expect_status(&source_free, 0);
    assert_eq!(source_free.stdout, expected);
}

fn source_free_run(fixture: &Fixture) -> Output {
    source_free_run_with_dependencies(fixture, &[("models", "data-models"), ("tools", "tools")])
}

fn source_free_run_with_dependencies(fixture: &Fixture, dependencies: &[(&str, &str)]) -> Output {
    let selected = compiler();
    let mut artifacts = artifact_records(fixture);
    // Hide only test-owned sources. The next compile and link must use package
    // declarations, aggregate layouts, physical signatures, and object payloads.
    for package in ["core", "models", "tools"] {
        fs::rename(
            fixture.root.join(package).join("src"),
            fixture.root.join(package).join("hidden-source"),
        )
        .expect("hide dependency sources");
    }
    let app_path = fixture.root.join("source-free-app.cpa");
    let mut compile = Command::new(&selected);
    compile
        .current_dir(&fixture.root)
        .args([
            "--shuttle-protocol",
            "2",
            "--operation",
            "compile",
            "--target",
            "x86_64",
            "--artifact-kind",
            "object",
            "--output",
        ])
        .arg(&app_path)
        .args(["--package", "app", "0.1.0"])
        .arg(fixture.root.join("app/src"))
        .args(["--entry", "Main.co"]);
    for (alias, package) in dependencies {
        compile.args(["--dependency", alias, package]);
    }
    for artifact in &artifacts {
        if artifact.name == "app" {
            continue;
        }
        compile
            .args([
                "--artifact",
                &artifact.name,
                &artifact.version,
                &artifact.digest,
            ])
            .arg(&artifact.path);
    }
    let compiled = run(&mut compile);
    expect_status(&compiled, 0);
    assert!(compiled.stderr.is_empty());
    let app_index = artifacts
        .iter()
        .position(|artifact| artifact.name == "app")
        .expect("app artifact");
    artifacts[app_index] = inspect_artifact(&app_path);
    let executable = fixture
        .root
        .join(format!("source-free-app{}", std::env::consts::EXE_SUFFIX));
    let linked = invoke_link(fixture, &executable, &artifacts);
    expect_status(&linked, 0);
    run(&mut Command::new(executable))
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
    assert_eq!(artifacts.len(), 5);
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
#[cfg(any(unix, windows))]
fn shuttle_run_preserves_managed_program_arguments() {
    const SOURCE: &str = r"
static func Main(string[] arguments): int32 {
  println(arguments::length);
  for (string argument in arguments) {
    println(argument::length);
    println(argument);
  }
  return arguments::length;
}
";
    let fixture = Fixture::new();
    fixture.write("app/src/Main.co", SOURCE);
    let selected = compiler();
    let arguments = ["first", "two words", "", " ", "--quiet", "é🙂"].map(OsString::from);
    let expected = "6\n5\nfirst\n9\ntwo words\n0\n\n1\n \n7\n--quiet\n2\né🙂\n";

    let mut shuttle = fixture.shuttle("run", &selected);
    shuttle.arg("--").args(&arguments);
    let forwarded = run(&mut shuttle);
    expect_status(&forwarded, 6);
    assert_eq!(
        String::from_utf8(forwarded.stdout.clone()).expect("program output"),
        expected
    );
    assert!(forwarded.stderr.is_empty());

    let executable = fixture
        .root
        .join("app/target/x86_64")
        .join(format!("app{}", std::env::consts::EXE_SUFFIX));
    let artifact_bytes = fixture.artifact_bytes("app/target/x86_64/packages");
    let executable_bytes = fs::read(&executable).expect("program executable");
    let direct = run(Command::new(&executable).args(&arguments));
    expect_status(&direct, 6);
    assert_eq!(direct.stdout, forwarded.stdout);
    assert_eq!(direct.stderr, forwarded.stderr);

    let whole = Fixture::new();
    whole.write("app/src/Main.co", SOURCE);
    let whole_output = whole_project_run_with_arguments(&whole, &arguments);
    expect_status(&whole_output, 6);
    assert_eq!(whole_output.stdout, forwarded.stdout);
    assert_eq!(whole_output.stderr, forwarded.stderr);

    let mut changed = fixture.visible_shuttle("run", &selected);
    changed.arg("--").arg("changed");
    let changed = run(&mut changed);
    expect_status(&changed, 1);
    assert_eq!(changed.stdout, b"1\n7\nchanged\n");
    let progress = String::from_utf8(changed.stderr).expect("reuse progress");
    assert_eq!(progress.matches("shuttle: reusing ").count(), 5);
    assert!(!progress.contains("shuttle: compiling "));
    assert_eq!(
        fixture.artifact_bytes("app/target/x86_64/packages"),
        artifact_bytes
    );
    assert_eq!(
        fs::read(&executable).expect("reused program executable"),
        executable_bytes
    );

    let mut empty = fixture.shuttle("run", &selected);
    empty.arg("--");
    let empty = run(&mut empty);
    expect_status(&empty, 0);
    assert_eq!(empty.stdout, b"0\n");
    assert!(empty.stderr.is_empty());

    let mut invalid = fixture.shuttle("run", &selected);
    invalid.arg("--").arg(invalid_unicode_argument());
    let invalid = run(&mut invalid);
    assert!(!invalid.status.success());
    assert!(invalid.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&invalid.stderr)
            .contains("cloth runtime error: program argument is not valid Unicode")
    );

    verify_program_argument_failure_preservation(&fixture, &selected, &executable);
    verify_throwing_program_argument_entry(&selected);
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn links_and_reuses_the_implicit_standard_library() {
    let fixture = Fixture::new();
    fixture.write(
        "app/Shuttle.toml",
        "manifest-version = 1\n\n[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[executable]\nentry = \"Main.co\"\n",
    );
    fixture.write(
        "app/src/Main.co",
        "import cloth.math::Math;\nstatic func Main() throws DivisionByZero, ParseError { println(Math.Gcd(84, 30)); println(int32::parse(\"38\")); println(ArgumentError(\"invalid argument\").Message); println(StateError(\"invalid state\").Message); }\n",
    );
    let selected = compiler();
    let first = run(&mut fixture.shuttle("run", &selected));
    expect_status(&first, 0);
    assert_eq!(first.stdout, b"6\n38\ninvalid argument\ninvalid state\n");
    assert!(first.stderr.is_empty());
    for package in ["cloth", "app"] {
        assert!(
            fixture
                .root
                .join(format!("app/target/x86_64/packages/{package}.cpa"))
                .is_file()
        );
    }

    let second = run(&mut fixture.visible_shuttle("run", &selected));
    expect_status(&second, 0);
    assert_eq!(second.stdout, b"6\n38\ninvalid argument\ninvalid state\n");
    let progress = String::from_utf8(second.stderr).expect("reuse progress");
    assert_eq!(progress.matches("shuttle: reusing ").count(), 2);
    assert!(!progress.contains("shuttle: compiling "));

    let whole = Fixture::new();
    whole.write(
        "app/Shuttle.toml",
        "manifest-version = 1\n\n[package]\nname = \"app\"\nversion = \"0.1.0\"\nsource-root = \"src\"\n\n[executable]\nentry = \"Main.co\"\n",
    );
    whole.write(
        "app/src/Main.co",
        "import cloth.math::Math;\nstatic func Main() throws DivisionByZero, ParseError { println(Math.Gcd(84, 30)); println(int32::parse(\"38\")); println(ArgumentError(\"invalid argument\").Message); println(StateError(\"invalid state\").Message); }\n",
    );
    let graph = resolve_package_graph(&whole.manifest()).expect("whole-project graph");
    let graph = inject_standard_library(&graph, &selected, "cloth", "0.3.0")
        .expect("whole-project standard library");
    let request = build_request(&graph, ProjectCommand::Build, Target::X86_64)
        .expect("whole-project request");
    let output = request
        .output_path()
        .expect("whole-project executable")
        .to_path_buf();
    fs::create_dir_all(output.parent().expect("output parent")).expect("create output parent");
    let compiled = run(Command::new(&selected)
        .current_dir(&whole.root)
        .args(request.arguments()));
    expect_status(&compiled, 0);
    assert!(compiled.stdout.is_empty() && compiled.stderr.is_empty());
    let whole_output = run(&mut Command::new(output));
    expect_status(&whole_output, 0);
    assert_eq!(whole_output.stdout, first.stdout);
    assert!(whole_output.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn primitive_parsing_inherits_stdin_without_changing_artifacts() {
    let fixture = Fixture::new();
    fixture.write(
        "app/src/Main.co",
        r#"
import cloth.io::Console;
static func Main() throws IoError, ParseError {
  string text = Console.ReadLine() ?? throw IoError("missing input");
  println(int32::parse(text));
}
"#,
    );
    fixture.write("first.input", "42\n");
    fixture.write("second.input", "84\n");
    fixture.write("invalid.input", "not-a-number\n");
    let selected = compiler();

    let mut first = fixture.shuttle("run", &selected);
    first.stdin(fs::File::open(fixture.root.join("first.input")).expect("first input"));
    let first = run(&mut first);
    expect_status(&first, 0);
    assert_eq!(first.stdout, b"42\n");
    assert!(first.stderr.is_empty());
    let artifacts = fixture.artifact_bytes("app/target/x86_64/packages");

    let mut second = fixture.visible_shuttle("run", &selected);
    second.stdin(fs::File::open(fixture.root.join("second.input")).expect("second input"));
    let second = run(&mut second);
    expect_status(&second, 0);
    assert_eq!(second.stdout, b"84\n");
    let progress = String::from_utf8(second.stderr).expect("reuse progress");
    assert_eq!(progress.matches("shuttle: reusing ").count(), 5);
    assert!(!progress.contains("shuttle: compiling "));
    assert_eq!(
        fixture.artifact_bytes("app/target/x86_64/packages"),
        artifacts
    );

    let mut invalid = fixture.shuttle("run", &selected);
    invalid.stdin(fs::File::open(fixture.root.join("invalid.input")).expect("invalid input"));
    let invalid = run(&mut invalid);
    expect_status(&invalid, 1);
    assert!(invalid.stdout.is_empty());
    let expected = if cfg!(windows) {
        b"cloth error: cloth.lang.errors.ParseError: invalid int32 text\r\n".as_slice()
    } else {
        b"cloth error: cloth.lang.errors.ParseError: invalid int32 text\n".as_slice()
    };
    assert_eq!(invalid.stderr, expected);
    assert_eq!(
        fixture.artifact_bytes("app/target/x86_64/packages"),
        artifacts
    );
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn broken_standard_library_preserves_completed_consumer_outputs() {
    let fixture = Fixture::new();
    fixture.write(
        "app/Shuttle.toml",
        "manifest-version = 1\n\n[package]\nname = \"app\"\nversion = \"0.1.0\"\nsource-root = \"src\"\n\n[executable]\nentry = \"Main.co\"\n",
    );
    fixture.write(
        "app/src/Main.co",
        "static func Main() throws ParseError { println(int32::parse(\"42\")); }\n",
    );
    let selected = compiler();
    let toolchain_directory = fixture.root.join("toolchain");
    fs::create_dir(&toolchain_directory).expect("toolchain directory");
    let paired_compiler =
        toolchain_directory.join(format!("clothc{}", std::env::consts::EXE_SUFFIX));
    fs::copy(&selected, &paired_compiler).expect("copy paired compiler");
    pair_compiler_with_standard_library(&selected, &paired_compiler);

    let built = run(&mut fixture.shuttle("run", &paired_compiler));
    expect_status(&built, 0);
    assert_eq!(built.stdout, b"42\n");
    assert!(built.stderr.is_empty());

    let package_directory = fixture.root.join("app/target/x86_64/packages");
    let cloth_artifact = package_directory.join("cloth.cpa");
    let app_artifact = package_directory.join("app.cpa");
    let executable = fixture
        .root
        .join("app/target/x86_64")
        .join(format!("app{}", std::env::consts::EXE_SUFFIX));
    let previous_cloth = fs::read(&cloth_artifact).expect("completed cloth artifact");
    let previous_app = fs::read(&app_artifact).expect("completed app artifact");
    let previous_executable = fs::read(&executable).expect("completed executable");

    let parse_error = toolchain_directory.join("standard-library/src/lang/errors/ParseError.co");
    let mut source = fs::read_to_string(&parse_error).expect("paired ParseError source");
    source.push_str("\nfunc Broken(): int32 { return missing; }\n");
    fs::write(parse_error, source).expect("break paired ParseError source");
    let failed = run(&mut fixture.shuttle("run", &paired_compiler));
    expect_status(&failed, 1);
    assert!(failed.stdout.is_empty());
    assert!(String::from_utf8_lossy(&failed.stderr).contains("ParseError.co:"));
    assert_eq!(
        fs::read(&cloth_artifact).expect("preserved cloth artifact"),
        previous_cloth
    );
    assert_eq!(
        fs::read(&app_artifact).expect("preserved app artifact"),
        previous_app
    );
    assert_eq!(
        fs::read(&executable).expect("preserved executable"),
        previous_executable
    );

    let preserved = run(&mut Command::new(executable));
    expect_status(&preserved, 0);
    assert_eq!(preserved.stdout, b"42\n");
    assert!(preserved.stderr.is_empty());
}

#[test]
#[ignore = "requires CLOTHC_UNDER_TEST and a native linker"]
fn native_build_reuses_unchanged_packages_and_repairs_a_corrupt_candidate() {
    let fixture = Fixture::new();
    let selected_compiler = compiler();
    expect_status(&run(&mut fixture.shuttle("build", &selected_compiler)), 0);

    let unchanged = run(&mut fixture.visible_shuttle("build", &selected_compiler));
    expect_status(&unchanged, 0);
    let progress = String::from_utf8(unchanged.stderr).expect("reuse progress");
    assert_eq!(progress.matches("shuttle: reusing").count(), 5);
    assert!(!progress.contains("shuttle: compiling"));

    let foundation = fixture
        .root
        .join("app/target/x86_64/packages/foundation.cpa");
    let mut bytes = fs::read(&foundation).expect("foundation artifact");
    *bytes.last_mut().expect("artifact byte") ^= 1;
    fs::write(&foundation, bytes).expect("corrupt candidate");
    let repaired = run(&mut fixture.visible_shuttle("build", &selected_compiler));
    expect_status(&repaired, 0);
    let progress = String::from_utf8(repaired.stderr).expect("repair progress");
    assert!(progress.contains("shuttle: compiling foundation "));
    for package in ["data-models", "tools", "app"] {
        assert!(
            progress.contains(&format!("shuttle: reusing {package} ")),
            "unchanged consumer was not reused: {progress}"
        );
    }

    let cloth = fixture.root.join("app/target/x86_64/packages/cloth.cpa");
    let mut bytes = fs::read(&cloth).expect("standard-library artifact");
    *bytes.last_mut().expect("artifact byte") ^= 1;
    fs::write(&cloth, bytes).expect("corrupt standard-library candidate");
    let repaired = run(&mut fixture.visible_shuttle("build", &selected_compiler));
    expect_status(&repaired, 0);
    let progress = String::from_utf8(repaired.stderr).expect("repair progress");
    assert!(progress.contains("shuttle: compiling cloth "));
    for package in ["foundation", "data-models", "tools", "app"] {
        assert!(
            progress.contains(&format!("shuttle: reusing {package} ")),
            "unchanged standard-library consumer was not reused: {progress}"
        );
    }
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
    let selected_compiler = compiler();
    let mut serial = first.shuttle("build", &selected_compiler);
    serial.args(["--jobs", "1"]);
    expect_status(&run(&mut serial), 0);
    // Cross a PE timestamp boundary to detect wall-clock-dependent link output.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let mut parallel = second.shuttle("build", &selected_compiler);
    parallel.args(["--jobs", "4"]);
    expect_status(&run(&mut parallel), 0);
    assert!(
        fs::read(first.root.join(&executable)).expect("first binary")
            == fs::read(second.root.join(&executable)).expect("second binary"),
        "relocated native builds differ"
    );
    for package in ["app", "cloth", "data-models", "foundation", "tools"] {
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
    assert_eq!(artifacts.len(), 5);
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
