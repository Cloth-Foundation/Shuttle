use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

use semver::Version;
use serde::Deserialize;
use toml::Spanned;

use crate::diagnostic::{Diagnostic, SourcePosition, position_for_offset, sort_diagnostics};

pub const MANIFEST_FILENAME: &str = "Shuttle.toml";
const LEGACY_MANIFEST_FILENAME: &str = "cloth.toml";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Manifest {
    pub path: PathBuf,
    pub package_root: PathBuf,
    pub package: Package,
    pub executable: Option<Executable>,
    pub dependencies: BTreeMap<String, Dependency>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: Version,
    pub source_root: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Executable {
    pub name: String,
    pub entry: PathBuf,
    pub entry_path: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dependency {
    pub alias: String,
    pub package_root: PathBuf,
    pub declaration_position: SourcePosition,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawManifest {
    #[serde(rename = "manifest-version")]
    manifest_version: Spanned<i64>,
    package: Spanned<RawPackage>,
    executable: Option<Spanned<RawExecutable>>,
    #[serde(default)]
    dependencies: BTreeMap<String, Spanned<RawDependency>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPackage {
    name: Spanned<String>,
    version: Spanned<String>,
    #[serde(rename = "source-root")]
    source_root: Option<Spanned<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExecutable {
    name: Option<Spanned<String>>,
    entry: Spanned<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDependency {
    path: Spanned<String>,
}

/// Resolves an explicit manifest path or discovers the nearest parent manifest.
///
/// # Errors
///
/// Returns a diagnostic when the explicit path is invalid, discovery cannot
/// inspect the filesystem, no manifest exists, or an obsolete or incorrectly
/// cased manifest name is present.
pub fn resolve_manifest_path(
    explicit_path: Option<&Path>,
    current_directory: &Path,
) -> Result<PathBuf, Diagnostic> {
    if let Some(path) = explicit_path {
        let candidate = if path.is_absolute() {
            path.to_path_buf()
        } else {
            current_directory.join(path)
        };
        return validate_explicit_manifest(&candidate);
    }

    discover_manifest(current_directory)
}

/// Finds the nearest canonical manifest at or above `start`.
///
/// # Errors
///
/// Returns a diagnostic when a directory cannot be inspected, no manifest is
/// found, or an obsolete or incorrectly cased manifest name is present.
pub fn discover_manifest(start: &Path) -> Result<PathBuf, Diagnostic> {
    let mut directory = absolute_path(start)?;
    loop {
        match inspect_manifest_directory(&directory)? {
            DirectoryManifest::Current(path) => return Ok(path),
            DirectoryManifest::Legacy(path) => return Err(legacy_manifest_error(path)),
            DirectoryManifest::CaseMismatch(path) => {
                return Err(Diagnostic::file(
                    path,
                    format!("manifest filename must be exactly '{MANIFEST_FILENAME}'"),
                ));
            }
            DirectoryManifest::Missing => {}
        }

        let Some(parent) = directory.parent() else {
            break;
        };
        if parent == directory {
            break;
        }
        directory = parent.to_path_buf();
    }

    Err(Diagnostic::global(format!(
        "could not find '{MANIFEST_FILENAME}' in '{}' or any parent directory",
        display_path(start)
    )))
}

/// Resolves the canonical manifest directly inside a package root.
///
/// # Errors
///
/// Returns a diagnostic when the root cannot be inspected or does not contain
/// exactly one correctly cased `Shuttle.toml` without an obsolete manifest.
pub fn manifest_in_package_root(package_root: &Path) -> Result<PathBuf, Diagnostic> {
    match inspect_manifest_directory(package_root)? {
        DirectoryManifest::Current(path) => Ok(path),
        DirectoryManifest::Legacy(path) => Err(legacy_manifest_error(path)),
        DirectoryManifest::CaseMismatch(path) => Err(Diagnostic::file(
            path,
            format!("manifest filename must be exactly '{MANIFEST_FILENAME}'"),
        )),
        DirectoryManifest::Missing => Err(Diagnostic::file(
            package_root,
            format!("dependency package root does not contain '{MANIFEST_FILENAME}'"),
        )),
    }
}

/// Parses and validates a manifest version 1 document.
///
/// # Errors
///
/// Returns deterministic diagnostics for filesystem, UTF-8, TOML, schema, and
/// root-package validation failures.
pub fn load_manifest(path: &Path) -> Result<Manifest, Vec<Diagnostic>> {
    let manifest_path = match validate_explicit_manifest(path) {
        Ok(path) => path,
        Err(diagnostic) => return Err(vec![diagnostic]),
    };
    let bytes = match fs::read(&manifest_path) {
        Ok(bytes) => bytes,
        Err(error) => {
            return Err(vec![Diagnostic::file(
                &manifest_path,
                format!("could not read manifest: {error}"),
            )]);
        }
    };
    let source = match String::from_utf8(bytes) {
        Ok(source) => source,
        Err(error) => {
            return Err(vec![Diagnostic::file(
                &manifest_path,
                format!("manifest is not valid UTF-8: {error}"),
            )]);
        }
    };

    let raw: RawManifest = match toml::from_str(&source) {
        Ok(raw) => raw,
        Err(error) => {
            let diagnostic = if let Some(span) = error.span() {
                Diagnostic::at_span(&manifest_path, &source, span, error.message())
            } else {
                Diagnostic::file(&manifest_path, error.message())
            };
            return Err(vec![diagnostic]);
        }
    };

    validate_manifest(manifest_path, &source, raw)
}

fn validate_manifest(
    manifest_path: PathBuf,
    source: &str,
    raw: RawManifest,
) -> Result<Manifest, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    if *raw.manifest_version.get_ref() != 1 {
        diagnostics.push(Diagnostic::at_span(
            &manifest_path,
            source,
            raw.manifest_version.span(),
            "manifest-version must be 1",
        ));
    }

    let package_root = manifest_path
        .parent()
        .expect("a validated manifest has a parent directory")
        .to_path_buf();
    validate_portable_native_path(
        &manifest_path,
        source,
        raw.package.span(),
        &package_root,
        "package root",
        &mut diagnostics,
    );

    let package_span = raw.package.span();
    let raw_package = raw.package.into_inner();
    let package = validate_package(
        &manifest_path,
        source,
        &package_root,
        package_span,
        &raw_package,
        &mut diagnostics,
    );

    let executable = match (raw.executable, package.as_ref()) {
        (Some(raw_executable), Some(package)) => validate_executable(
            &manifest_path,
            source,
            package,
            raw_executable,
            &mut diagnostics,
        ),
        (None, _) | (Some(_), None) => None,
    };

    let dependencies = validate_dependencies(
        &manifest_path,
        source,
        &package_root,
        raw.dependencies,
        &mut diagnostics,
    );

    if diagnostics.is_empty() {
        Ok(Manifest {
            path: manifest_path,
            package_root,
            package: package.expect("a valid manifest has a package"),
            executable,
            dependencies,
        })
    } else {
        sort_diagnostics(&mut diagnostics);
        Err(diagnostics)
    }
}

fn validate_package(
    manifest_path: &Path,
    source: &str,
    package_root: &Path,
    package_span: std::ops::Range<usize>,
    raw: &RawPackage,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<Package> {
    let name = raw.name.get_ref();
    if !is_package_name(name) {
        diagnostics.push(Diagnostic::at_span(
            manifest_path,
            source,
            raw.name.span(),
            "package name must be 1-64 lowercase ASCII letters, digits, or single '-' separators",
        ));
    }

    let version = match Version::parse(raw.version.get_ref()) {
        Ok(version) => Some(version),
        Err(error) => {
            diagnostics.push(Diagnostic::at_span(
                manifest_path,
                source,
                raw.version.span(),
                format!("package version is not valid Semantic Versioning 2.0.0: {error}"),
            ));
            None
        }
    };

    let source_root_value = raw
        .source_root
        .as_ref()
        .map_or("src", |value| value.get_ref().as_str());
    let source_root_span = raw.source_root.as_ref().map_or(package_span, Spanned::span);
    let source_root = validate_source_root(
        manifest_path,
        source,
        package_root,
        source_root_value,
        source_root_span,
        diagnostics,
    );

    if is_package_name(name) {
        Some(Package {
            name: name.clone(),
            version: version?,
            source_root: source_root?,
        })
    } else {
        None
    }
}

fn validate_source_root(
    manifest_path: &Path,
    source: &str,
    package_root: &Path,
    value: &str,
    span: std::ops::Range<usize>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<PathBuf> {
    if let Err(message) = validate_relative_path(value, false) {
        diagnostics.push(Diagnostic::at_span(
            manifest_path,
            source,
            span,
            format!("invalid package source-root: {message}"),
        ));
        return None;
    }

    let candidate = package_root.join(value);
    let canonical_source_root = match fs::canonicalize(&candidate) {
        Ok(path) if path.is_dir() => path,
        Ok(_) => {
            diagnostics.push(Diagnostic::at_span(
                manifest_path,
                source,
                span,
                "package source-root is not a directory",
            ));
            return None;
        }
        Err(error) => {
            diagnostics.push(Diagnostic::at_span(
                manifest_path,
                source,
                span,
                format!("could not resolve package source-root: {error}"),
            ));
            return None;
        }
    };
    let canonical_package_root = match fs::canonicalize(package_root) {
        Ok(path) => path,
        Err(error) => {
            diagnostics.push(Diagnostic::file(
                manifest_path,
                format!("could not resolve package root: {error}"),
            ));
            return None;
        }
    };
    if !canonical_source_root.starts_with(&canonical_package_root) {
        diagnostics.push(Diagnostic::at_span(
            manifest_path,
            source,
            span,
            "package source-root resolves outside the package root",
        ));
        return None;
    }
    validate_portable_native_path(
        manifest_path,
        source,
        span,
        &canonical_source_root,
        "package source-root",
        diagnostics,
    );
    Some(canonical_source_root)
}

fn validate_executable(
    manifest_path: &Path,
    source: &str,
    package: &Package,
    raw: Spanned<RawExecutable>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<Executable> {
    let raw = raw.into_inner();
    let name = raw
        .name
        .as_ref()
        .map_or(package.name.as_str(), |value| value.get_ref().as_str());
    if !is_package_name(name) {
        let span = raw.name.as_ref().map_or(raw.entry.span(), Spanned::span);
        diagnostics.push(Diagnostic::at_span(
            manifest_path,
            source,
            span,
            "executable name must follow the package-name grammar",
        ));
    }

    let entry_value = raw.entry.get_ref();
    if let Err(message) = validate_relative_path(entry_value, false) {
        diagnostics.push(Diagnostic::at_span(
            manifest_path,
            source,
            raw.entry.span(),
            format!("invalid executable entry: {message}"),
        ));
        return None;
    }
    if Path::new(entry_value)
        .extension()
        .and_then(|value| value.to_str())
        != Some("co")
    {
        diagnostics.push(Diagnostic::at_span(
            manifest_path,
            source,
            raw.entry.span(),
            "executable entry must use the exact '.co' extension",
        ));
        return None;
    }

    let entry_path = package.source_root.join(entry_value);
    let canonical_entry = match fs::canonicalize(&entry_path) {
        Ok(path) if path.is_file() => path,
        Ok(_) => {
            diagnostics.push(Diagnostic::at_span(
                manifest_path,
                source,
                raw.entry.span(),
                "executable entry is not a regular file",
            ));
            return None;
        }
        Err(error) => {
            diagnostics.push(Diagnostic::at_span(
                manifest_path,
                source,
                raw.entry.span(),
                format!("could not resolve executable entry: {error}"),
            ));
            return None;
        }
    };
    if !canonical_entry.starts_with(&package.source_root) {
        diagnostics.push(Diagnostic::at_span(
            manifest_path,
            source,
            raw.entry.span(),
            "executable entry resolves outside the package source-root",
        ));
        return None;
    }

    if is_package_name(name) {
        Some(Executable {
            name: name.to_owned(),
            entry: PathBuf::from(entry_value),
            entry_path: canonical_entry,
        })
    } else {
        None
    }
}

fn validate_dependencies(
    manifest_path: &Path,
    source: &str,
    package_root: &Path,
    raw_dependencies: BTreeMap<String, Spanned<RawDependency>>,
    diagnostics: &mut Vec<Diagnostic>,
) -> BTreeMap<String, Dependency> {
    let mut dependencies = BTreeMap::new();
    for (alias, raw_dependency) in raw_dependencies {
        let declaration_span = raw_dependency.span();
        let declaration_position = position_for_offset(source, declaration_span.start);
        let raw_dependency = raw_dependency.into_inner();
        let mut valid = true;
        if !is_dependency_alias(&alias) {
            diagnostics.push(Diagnostic::at_span(
                manifest_path,
                source,
                declaration_span,
                format!("dependency alias '{alias}' must be a lowercase Cloth identifier"),
            ));
            valid = false;
        } else if is_cloth_keyword(&alias) {
            diagnostics.push(Diagnostic::at_span(
                manifest_path,
                source,
                declaration_span,
                format!("dependency alias '{alias}' is a Cloth keyword"),
            ));
            valid = false;
        }

        let dependency_path = raw_dependency.path.get_ref();
        if let Err(message) = validate_relative_path(dependency_path, true) {
            diagnostics.push(Diagnostic::at_span(
                manifest_path,
                source,
                raw_dependency.path.span(),
                format!("invalid path for dependency '{alias}': {message}"),
            ));
            valid = false;
        }

        if valid {
            dependencies.insert(
                alias.clone(),
                Dependency {
                    alias,
                    package_root: package_root.join(dependency_path),
                    declaration_position,
                },
            );
        }
    }
    dependencies
}

fn validate_explicit_manifest(candidate: &Path) -> Result<PathBuf, Diagnostic> {
    let file_name = candidate.file_name().and_then(|name| name.to_str());
    match file_name {
        Some(MANIFEST_FILENAME) => {}
        Some(LEGACY_MANIFEST_FILENAME) => {
            return Err(legacy_manifest_error(candidate.to_path_buf()));
        }
        Some(name) if name.eq_ignore_ascii_case(MANIFEST_FILENAME) => {
            return Err(Diagnostic::file(
                candidate,
                format!("manifest filename must be exactly '{MANIFEST_FILENAME}'"),
            ));
        }
        _ => {
            return Err(Diagnostic::file(
                candidate,
                format!("manifest path must name '{MANIFEST_FILENAME}'"),
            ));
        }
    }

    if !candidate.is_file() {
        return Err(Diagnostic::file(
            candidate,
            "manifest is not a regular file",
        ));
    }
    let absolute = absolute_path(candidate)?;
    let package_root = absolute
        .parent()
        .expect("a manifest file has a parent directory");
    if package_root.join(LEGACY_MANIFEST_FILENAME).is_file() {
        return Err(Diagnostic::file(
            package_root.join(LEGACY_MANIFEST_FILENAME),
            format!(
                "remove obsolete '{LEGACY_MANIFEST_FILENAME}' before using '{MANIFEST_FILENAME}'"
            ),
        ));
    }
    Ok(absolute)
}

enum DirectoryManifest {
    Current(PathBuf),
    Legacy(PathBuf),
    CaseMismatch(PathBuf),
    Missing,
}

fn inspect_manifest_directory(directory: &Path) -> Result<DirectoryManifest, Diagnostic> {
    let entries = fs::read_dir(directory).map_err(|error| {
        Diagnostic::file(
            directory,
            format!("could not inspect directory for a manifest: {error}"),
        )
    })?;
    let mut current = None;
    let mut legacy = None;
    let mut case_mismatch = None;
    for entry in entries {
        let entry = entry.map_err(|error| {
            Diagnostic::file(
                directory,
                format!("could not inspect directory entry: {error}"),
            )
        })?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if name == MANIFEST_FILENAME {
            current = Some(entry.path());
        } else if name == LEGACY_MANIFEST_FILENAME {
            legacy = Some(entry.path());
        } else if name.eq_ignore_ascii_case(MANIFEST_FILENAME) {
            case_mismatch = Some(entry.path());
        }
    }

    if let (Some(_), Some(legacy)) = (&current, &legacy) {
        return Err(Diagnostic::file(
            legacy,
            format!(
                "remove obsolete '{LEGACY_MANIFEST_FILENAME}' before using '{MANIFEST_FILENAME}'"
            ),
        ));
    }
    if let Some(path) = case_mismatch {
        return Ok(DirectoryManifest::CaseMismatch(path));
    }
    if let Some(path) = current {
        return Ok(DirectoryManifest::Current(absolute_path(&path)?));
    }
    if let Some(path) = legacy {
        return Ok(DirectoryManifest::Legacy(path));
    }
    Ok(DirectoryManifest::Missing)
}

fn absolute_path(path: &Path) -> Result<PathBuf, Diagnostic> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        std::env::current_dir()
            .map(|directory| directory.join(path))
            .map_err(|error| {
                Diagnostic::file(path, format!("could not make path absolute: {error}"))
            })
    }
}

fn legacy_manifest_error(path: PathBuf) -> Diagnostic {
    Diagnostic::file(
        path,
        format!(
            "'{LEGACY_MANIFEST_FILENAME}' is obsolete; create '{MANIFEST_FILENAME}' with manifest-version and package fields"
        ),
    )
}

fn validate_relative_path(value: &str, allow_parent: bool) -> Result<(), &'static str> {
    if value.is_empty() {
        return Err("path must not be empty");
    }
    if value.contains('\\') {
        return Err("path must use '/' separators");
    }
    if value.starts_with('/') || value.ends_with('/') || value.contains("//") {
        return Err("path must not contain empty components");
    }
    if value.chars().any(char::is_control) {
        return Err("path must not contain control characters");
    }
    let path = Path::new(value);
    if path.is_absolute() {
        return Err("path must be relative");
    }
    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::ParentDir if allow_parent => {}
            Component::ParentDir => return Err("path must not escape its root"),
            Component::CurDir => return Err("path must not contain '.' components"),
            Component::Prefix(_) | Component::RootDir => return Err("path must be relative"),
        }
    }
    Ok(())
}

fn validate_portable_native_path(
    manifest_path: &Path,
    source: &str,
    span: std::ops::Range<usize>,
    path: &Path,
    description: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(value) = path.to_str() else {
        diagnostics.push(Diagnostic::at_span(
            manifest_path,
            source,
            span,
            format!("{description} is not representable as Unicode"),
        ));
        return;
    };
    if value.chars().any(char::is_control) {
        diagnostics.push(Diagnostic::at_span(
            manifest_path,
            source,
            span,
            format!("{description} contains a control character"),
        ));
    }
}

fn is_package_name(value: &str) -> bool {
    if value.is_empty() || value.len() > 64 || !value.is_ascii() {
        return false;
    }
    let mut parts = value.split('-');
    let Some(first) = parts.next() else {
        return false;
    };
    if first.is_empty()
        || !first.as_bytes()[0].is_ascii_lowercase()
        || !first
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    {
        return false;
    }
    parts.all(|part| {
        !part.is_empty()
            && part
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    })
}

fn is_dependency_alias(value: &str) -> bool {
    let mut bytes = value.bytes();
    bytes.next().is_some_and(|byte| byte.is_ascii_lowercase())
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn is_cloth_keyword(value: &str) -> bool {
    const KEYWORDS: &[&str] = &[
        "abstract",
        "as",
        "bool",
        "break",
        "byte",
        "char",
        "class",
        "const",
        "continue",
        "else",
        "enum",
        "extern",
        "false",
        "final",
        "float",
        "float32",
        "float64",
        "for",
        "func",
        "if",
        "import",
        "in",
        "int",
        "int8",
        "int16",
        "int32",
        "int64",
        "interface",
        "is",
        "let",
        "match",
        "null",
        "object",
        "override",
        "return",
        "sealed",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "uint",
        "uint8",
        "uint16",
        "uint32",
        "uint64",
        "unsafe",
        "var",
        "void",
        "while",
    ];
    KEYWORDS.binary_search(&value).is_ok()
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::{is_cloth_keyword, is_dependency_alias, is_package_name};

    #[test]
    fn validates_package_names() {
        for valid in ["a", "cloth", "hello-world", "cloth2-runtime"] {
            assert!(is_package_name(valid), "expected '{valid}' to be valid");
        }
        for invalid in ["", "A", "hello_world", "-hello", "hello-", "hello--world"] {
            assert!(
                !is_package_name(invalid),
                "expected '{invalid}' to be invalid"
            );
        }
    }

    #[test]
    fn validates_dependency_aliases_and_keywords() {
        assert!(is_dependency_alias("models"));
        assert!(is_dependency_alias("text_utils2"));
        assert!(!is_dependency_alias("Models"));
        assert!(!is_dependency_alias("text-utils"));
        assert!(is_cloth_keyword("for"));
        assert!(!is_cloth_keyword("models"));
    }
}
