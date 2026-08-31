//! Opt-in tests of the public process protocol, never compiler internals.
#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::fs;
use std::process::Command;

use shuttle::compiler::{ProjectCommand, Target, build_request};
use shuttle::graph::resolve_package_graph;
use support::{Fixture, compiler, expect_status, run};

fn arguments(fixture: &Fixture) -> Vec<OsString> {
    let graph = resolve_package_graph(&fixture.manifest()).expect("fixture graph");
    build_request(&graph, ProjectCommand::Check, Target::Wasm32)
        .expect("check request")
        .arguments()
        .to_vec()
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
    for package in ["app", "models", "tools", "core"] {
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
        assert!(!fixture.root.join("app/target").exists());
    }
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
