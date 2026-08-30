use std::fs;
use std::path::Path;

use shuttle::diagnostic::Diagnostic;
use shuttle::manifest::{MANIFEST_FILENAME, discover_manifest, load_manifest};
use tempfile::TempDir;

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create test directory");
    }
    fs::write(path, contents).expect("write test file");
}

fn valid_project(manifest: &str) -> TempDir {
    let project = TempDir::new().expect("create temporary project");
    write_file(&project.path().join("src/Main.co"), "static Main() {}\n");
    write_file(&project.path().join(MANIFEST_FILENAME), manifest);
    project
}

#[test]
fn loads_the_complete_version_one_manifest() {
    let project = valid_project(
        r#"manifest-version = 1

[package]
name = "hello-world"
version = "1.2.3-alpha.1+build.7"
source-root = "src"

[executable]
entry = "Main.co"

[dependencies]
text_utils = { path = "../text-utils" }
models = { path = "../models" }
"#,
    );

    let manifest = load_manifest(&project.path().join(MANIFEST_FILENAME)).expect("valid manifest");
    assert_eq!(manifest.package.name, "hello-world");
    assert_eq!(
        manifest.package.version.to_string(),
        "1.2.3-alpha.1+build.7"
    );
    assert_eq!(
        manifest.executable.as_ref().expect("executable").name,
        "hello-world"
    );
    assert_eq!(
        manifest.dependencies.keys().collect::<Vec<_>>(),
        vec!["models", "text_utils"]
    );
}

#[test]
fn applies_the_source_root_default() {
    let project = valid_project(
        r#"manifest-version = 1

[package]
name = "library"
version = "0.1.0"
"#,
    );

    let manifest = load_manifest(&project.path().join(MANIFEST_FILENAME)).expect("valid manifest");
    assert!(manifest.package.source_root.ends_with("src"));
    assert!(manifest.executable.is_none());
    assert!(manifest.dependencies.is_empty());
}

#[test]
fn rejects_unknown_schema_fields_at_their_source_location() {
    let project = valid_project(
        r#"manifest-version = 1
profile = "release"

[package]
name = "hello"
version = "0.1.0"
"#,
    );

    let diagnostics =
        load_manifest(&project.path().join(MANIFEST_FILENAME)).expect_err("unknown field fails");
    assert_eq!(diagnostics.len(), 1);
    assert!(diagnostics[0].message().contains("unknown field `profile`"));
    assert_eq!(diagnostics[0].position().expect("source position").line, 2);
}

#[test]
fn accumulates_and_orders_independent_validation_errors() {
    let project = valid_project(
        r#"manifest-version = 2

[package]
name = "Invalid_Name"
version = "not-semver"
source-root = "missing"

[dependencies]
for = { path = "../dependency" }
"#,
    );

    let diagnostics =
        load_manifest(&project.path().join(MANIFEST_FILENAME)).expect_err("invalid manifest");
    let lines = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.position().expect("source position").line)
        .collect::<Vec<_>>();
    assert!(lines.windows(2).all(|pair| pair[0] <= pair[1]));
    let messages = diagnostics
        .iter()
        .map(Diagnostic::message)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(messages.contains("manifest-version must be 1"));
    assert!(messages.contains("package name"));
    assert!(messages.contains("Semantic Versioning 2.0.0"));
    assert!(messages.contains("source-root"));
    assert!(messages.contains("is a Cloth keyword"));
}

#[test]
fn rejects_toml_eleven_only_string_escapes() {
    let project = valid_project(
        r#"manifest-version = 1

[package]
name = "hello\e"
version = "0.1.0"
"#,
    );

    let diagnostics = load_manifest(&project.path().join(MANIFEST_FILENAME))
        .expect_err("TOML 1.1-only syntax fails");
    assert_eq!(diagnostics.len(), 1);
    assert_eq!(diagnostics[0].position().expect("source position").line, 4);
}

#[test]
fn rejects_non_normalized_manifest_paths() {
    let project = valid_project(
        r#"manifest-version = 1

[package]
name = "hello"
version = "0.1.0"
source-root = "src//nested"
"#,
    );

    let diagnostics = load_manifest(&project.path().join(MANIFEST_FILENAME))
        .expect_err("empty path component fails");
    assert!(diagnostics[0].message().contains("empty components"));
}

#[test]
fn discovers_the_nearest_parent_manifest() {
    let project = valid_project(
        r#"manifest-version = 1

[package]
name = "hello"
version = "0.1.0"
"#,
    );
    let nested = project.path().join("src/deep/location");
    fs::create_dir_all(&nested).expect("create nested directory");

    assert_eq!(
        discover_manifest(&nested).expect("discover manifest"),
        project.path().join(MANIFEST_FILENAME)
    );
}

#[test]
fn reports_the_obsolete_manifest_name() {
    let project = TempDir::new().expect("create temporary project");
    write_file(&project.path().join("cloth.toml"), "");

    let diagnostic = discover_manifest(project.path()).expect_err("legacy manifest fails");
    assert!(diagnostic.message().contains("is obsolete"));
    assert!(diagnostic.to_string().contains("cloth.toml"));
}

#[test]
fn reports_case_mismatched_manifest_names() {
    let project = TempDir::new().expect("create temporary project");
    write_file(&project.path().join("shuttle.toml"), "");

    let diagnostic = discover_manifest(project.path()).expect_err("case mismatch fails");
    assert!(
        diagnostic
            .message()
            .contains("must be exactly 'Shuttle.toml'")
    );
}
