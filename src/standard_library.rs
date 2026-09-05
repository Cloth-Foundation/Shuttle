// Part of the Cloth Compiler project, under the Apache License v2.0 with LLVM
// Exceptions. See LICENSE.txt in the project root for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::fs;
use std::path::{Component, Path};

use semver::Version;
use serde::Deserialize;

use crate::diagnostic::SourcePosition;
use crate::graph::{DependencyEdge, PackageGraph, resolve_package_graph};

pub const PACKAGE_NAME: &str = "cloth";
pub const TOOLCHAIN_METADATA_FILENAME: &str = "cloth-toolchain.json";

const TOOLCHAIN_METADATA_SCHEMA: u32 = 1;
const TOOLCHAIN_METADATA_LIMIT: u64 = 64 * 1024;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ToolchainMetadata {
    schema: u32,
    standard_library: StandardLibrarySelection,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StandardLibrarySelection {
    package: String,
    version: String,
    manifest: String,
}

/// Adds the compiler-paired standard library to an ordinary package graph.
///
/// # Errors
///
/// Returns an error when toolchain metadata is absent or invalid, the selected
/// standard library does not match the compiler, or a project tries to replace
/// the reserved `cloth` package.
pub fn inject_standard_library(
    graph: &PackageGraph,
    compiler: &Path,
    expected_package: &str,
    expected_version: &str,
) -> Result<PackageGraph, String> {
    let compiler_directory = compiler.parent().ok_or_else(|| {
        format!(
            "compiler '{}' has no toolchain directory",
            display_path(compiler)
        )
    })?;
    let metadata_path = compiler_directory.join(TOOLCHAIN_METADATA_FILENAME);
    let metadata = read_metadata(&metadata_path)?;
    if metadata.standard_library.package != expected_package
        || metadata.standard_library.version != expected_version
    {
        return Err(format!(
            "standard library selection in '{}' does not match the selected compiler",
            display_path(&metadata_path)
        ));
    }
    if expected_package != PACKAGE_NAME {
        return Err(format!(
            "selected compiler identifies reserved standard library package as '{expected_package}' instead of '{PACKAGE_NAME}'"
        ));
    }
    let expected_version = Version::parse(expected_version).map_err(|error| {
        format!("selected compiler has an invalid standard library version: {error}")
    })?;
    let relative_manifest = validate_manifest_path(&metadata.standard_library.manifest)?;
    let selected_manifest = compiler_directory.join(relative_manifest);
    let standard_graph = resolve_package_graph(&selected_manifest).map_err(|diagnostics| {
        let details = diagnostics
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "standard library distribution '{}' is invalid:\n{details}",
            display_path(&selected_manifest)
        )
    })?;
    validate_distribution(&standard_graph, &expected_version, &selected_manifest)?;

    let standard_package = standard_graph.packages.get(PACKAGE_NAME).ok_or_else(|| {
        "validated standard library graph unexpectedly lacks package 'cloth'".to_owned()
    })?;
    if graph.root_package == PACKAGE_NAME
        && graph
            .packages
            .get(PACKAGE_NAME)
            .is_some_and(|package| package.manifest_path == standard_package.manifest_path)
    {
        return Ok(graph.clone());
    }
    if let Some(package) = graph.packages.get(PACKAGE_NAME) {
        return Err(format!(
            "package '{}' at '{}' is reserved for the compiler-paired standard library",
            PACKAGE_NAME,
            display_path(&package.manifest_path)
        ));
    }

    let mut result = graph.clone();
    let owners = result
        .packages
        .iter()
        .map(|(name, package)| (name.clone(), package.manifest_path.clone()))
        .collect::<Vec<_>>();
    for (owner, manifest_path) in owners {
        result.dependencies.push(DependencyEdge {
            owner,
            alias: PACKAGE_NAME.to_owned(),
            target: PACKAGE_NAME.to_owned(),
            declaration_path: manifest_path,
            declaration_position: SourcePosition { line: 1, column: 1 },
        });
    }
    result
        .packages
        .insert(PACKAGE_NAME.to_owned(), standard_package.clone());
    result.dependencies.sort_by(|left, right| {
        left.owner
            .cmp(&right.owner)
            .then_with(|| left.alias.cmp(&right.alias))
    });
    Ok(result)
}

fn read_metadata(path: &Path) -> Result<ToolchainMetadata, String> {
    let metadata = fs::metadata(path).map_err(|error| {
        format!(
            "could not read compiler toolchain metadata '{}': {error}",
            display_path(path)
        )
    })?;
    if !metadata.is_file() || metadata.len() > TOOLCHAIN_METADATA_LIMIT {
        return Err(format!(
            "compiler toolchain metadata '{}' must be a file no larger than 64 KiB",
            display_path(path)
        ));
    }
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "could not read compiler toolchain metadata '{}': {error}",
            display_path(path)
        )
    })?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > TOOLCHAIN_METADATA_LIMIT {
        return Err(format!(
            "compiler toolchain metadata '{}' exceeds its 64 KiB limit",
            display_path(path)
        ));
    }
    let parsed: ToolchainMetadata = serde_json::from_slice(&bytes).map_err(|error| {
        format!(
            "compiler toolchain metadata '{}' is invalid JSON: {error}",
            display_path(path)
        )
    })?;
    if parsed.schema != TOOLCHAIN_METADATA_SCHEMA {
        return Err(format!(
            "compiler toolchain metadata '{}' uses unsupported schema {}",
            display_path(path),
            parsed.schema
        ));
    }
    Ok(parsed)
}

fn validate_manifest_path(value: &str) -> Result<&Path, String> {
    if value.is_empty()
        || value.contains('\\')
        || value.starts_with('/')
        || value.ends_with('/')
        || value.contains("//")
        || value.split('/').any(|component| component == ".")
        || value.chars().any(char::is_control)
    {
        return Err("standard library manifest must be a normalized relative '/' path".to_owned());
    }
    let path = Path::new(value);
    let mut saw_normal = false;
    for component in path.components() {
        match component {
            Component::ParentDir if !saw_normal => {}
            Component::Normal(_) => saw_normal = true,
            _ => {
                return Err(
                    "standard library manifest must be a normalized relative '/' path".to_owned(),
                );
            }
        }
    }
    if path.file_name().and_then(|name| name.to_str()) != Some("Shuttle.toml") {
        return Err("standard library manifest must name 'Shuttle.toml'".to_owned());
    }
    Ok(path)
}

fn validate_distribution(
    graph: &PackageGraph,
    expected_version: &Version,
    selected_manifest: &Path,
) -> Result<(), String> {
    let package = graph.packages.get(PACKAGE_NAME).ok_or_else(|| {
        format!(
            "standard library distribution '{}' must define package 'cloth'",
            display_path(selected_manifest)
        )
    })?;
    if graph.root_package != PACKAGE_NAME
        || graph.packages.len() != 1
        || !graph.dependencies.is_empty()
        || graph.root_executable.is_some()
    {
        return Err(format!(
            "standard library distribution '{}' must contain only the executable-free package 'cloth' with no dependencies",
            display_path(selected_manifest)
        ));
    }
    if package.version != *expected_version {
        return Err(format!(
            "standard library package version '{}' does not match compiler version '{}'",
            package.version, expected_version
        ));
    }
    Ok(())
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::validate_manifest_path;

    #[test]
    fn accepts_distribution_relative_manifest_paths() {
        assert!(validate_manifest_path("standard-library/Shuttle.toml").is_ok());
        assert!(validate_manifest_path("../../std/Shuttle.toml").is_ok());
    }

    #[test]
    fn rejects_ambiguous_or_non_manifest_paths() {
        for path in [
            "",
            "/std/Shuttle.toml",
            "std\\Shuttle.toml",
            "std//Shuttle.toml",
            "std/../Shuttle.toml",
            "std/./Shuttle.toml",
            "std/cloth.toml",
        ] {
            assert!(validate_manifest_path(path).is_err(), "accepted {path:?}");
        }
    }
}
