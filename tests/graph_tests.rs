use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use shuttle::compiler::{ProjectCommand, Target, build_request};
use shuttle::graph::resolve_package_graph;
use tempfile::TempDir;

fn write_package(root: &Path, name: &str, dependencies: &[(&str, &str)], executable: bool) {
    fs::create_dir_all(root.join("src")).expect("create source root");
    fs::write(root.join("src/Main.co"), "static func Main() {}\n").expect("write source");
    let mut manifest =
        format!("manifest-version = 1\n\n[package]\nname = \"{name}\"\nversion = \"0.1.0\"\n");
    if executable {
        manifest.push_str("\n[executable]\nentry = \"Main.co\"\n");
    }
    if !dependencies.is_empty() {
        manifest.push_str("\n[dependencies]\n");
        for (alias, path) in dependencies {
            writeln!(manifest, "{alias} = {{ path = \"{path}\" }}")
                .expect("writing to a string cannot fail");
        }
    }
    fs::write(root.join("Shuttle.toml"), manifest).expect("write manifest");
}

#[test]
fn resolves_and_orders_a_transitive_local_graph() {
    let workspace = TempDir::new().expect("create temporary workspace");
    let app = workspace.path().join("app");
    let models = workspace.path().join("models");
    let core = workspace.path().join("core");
    write_package(&app, "app", &[("models", "../models")], true);
    write_package(&models, "models", &[("core", "../core")], false);
    write_package(&core, "core", &[], false);

    let graph = resolve_package_graph(&app.join("Shuttle.toml")).expect("valid graph");
    assert_eq!(graph.root_package, "app");
    assert_eq!(
        graph.packages.keys().collect::<Vec<_>>(),
        vec!["app", "core", "models"]
    );
    assert_eq!(
        graph
            .dependencies
            .iter()
            .map(|edge| (
                edge.owner.as_str(),
                edge.alias.as_str(),
                edge.target.as_str()
            ))
            .collect::<Vec<_>>(),
        vec![("app", "models", "models"), ("models", "core", "core")]
    );
}

#[test]
fn reports_the_complete_dependency_cycle() {
    let workspace = TempDir::new().expect("create temporary workspace");
    let alpha = workspace.path().join("alpha");
    let beta = workspace.path().join("beta");
    write_package(&alpha, "alpha", &[("beta", "../beta")], false);
    write_package(&beta, "beta", &[("alpha", "../alpha")], false);

    let diagnostics = resolve_package_graph(&alpha.join("Shuttle.toml")).expect_err("cycle fails");
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic
            .message()
            .contains("alpha --beta--> beta --alpha--> alpha")
    }));
}

#[test]
fn rejects_two_aliases_for_one_dependency_root() {
    let workspace = TempDir::new().expect("create temporary workspace");
    let app = workspace.path().join("app");
    let shared = workspace.path().join("shared");
    write_package(
        &app,
        "app",
        &[("first", "../shared"), ("second", "../shared")],
        false,
    );
    write_package(&shared, "shared", &[], false);

    let diagnostics =
        resolve_package_graph(&app.join("Shuttle.toml")).expect_err("duplicate target fails");
    assert!(
        diagnostics[0]
            .message()
            .contains("resolve to the same package root")
    );
}

#[test]
fn rejects_duplicate_package_names_at_different_roots() {
    let workspace = TempDir::new().expect("create temporary workspace");
    let app = workspace.path().join("app");
    let first = workspace.path().join("first");
    let second = workspace.path().join("second");
    write_package(
        &app,
        "app",
        &[("first", "../first"), ("second", "../second")],
        false,
    );
    write_package(&first, "shared", &[], false);
    write_package(&second, "shared", &[], false);

    let diagnostics =
        resolve_package_graph(&app.join("Shuttle.toml")).expect_err("duplicate name fails");
    assert!(
        diagnostics[0]
            .message()
            .contains("package name 'shared' is already provided")
    );
}

#[test]
fn constructs_the_canonical_compiler_argument_order() {
    let workspace = TempDir::new().expect("create temporary workspace");
    let app = workspace.path().join("app");
    let models = workspace.path().join("models");
    write_package(&app, "app", &[("models", "../models")], true);
    write_package(&models, "models", &[], false);
    let graph = resolve_package_graph(&app.join("Shuttle.toml")).expect("valid graph");

    let request = build_request(&graph, ProjectCommand::Check, Target::Wasm32)
        .expect("valid compiler request");
    let arguments = request
        .arguments()
        .iter()
        .map(|argument| argument.as_os_str().to_string_lossy())
        .collect::<Vec<_>>();
    assert_eq!(
        &arguments[..6],
        [
            "--shuttle-protocol",
            "1",
            "--target",
            "wasm32",
            "--output-kind",
            "check"
        ]
    );
    let package_names = arguments
        .windows(4)
        .filter(|window| window[0] == "--package")
        .map(|window| window[1].as_ref())
        .collect::<Vec<_>>();
    assert_eq!(package_names, ["app", "models"]);
    let dependency = arguments
        .windows(4)
        .find(|window| window[0] == "--dependency")
        .expect("dependency arguments");
    assert_eq!(&dependency[1..], ["app", "models", "models"]);
}
