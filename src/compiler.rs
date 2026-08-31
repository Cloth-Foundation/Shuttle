use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use clap::ValueEnum;

use crate::diagnostic::Diagnostic;
use crate::graph::PackageGraph;

const PROTOCOL_VERSION: &str = "1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectCommand {
    Check,
    Build,
    Run,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum Target {
    #[value(name = "x86_64")]
    X86_64,
    #[value(name = "wasm32")]
    Wasm32,
}

impl fmt::Display for Target {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X86_64 => formatter.write_str("x86_64"),
            Self::Wasm32 => formatter.write_str("wasm32"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompilerRequest {
    root_package: String,
    arguments: Vec<OsString>,
    output_path: Option<PathBuf>,
    run_after_build: bool,
}

impl CompilerRequest {
    #[must_use]
    pub fn arguments(&self) -> &[OsString] {
        &self.arguments
    }

    #[must_use]
    pub fn output_path(&self) -> Option<&Path> {
        self.output_path.as_deref()
    }
}

#[derive(Debug)]
pub struct ProcessFailure {
    pub exit_code: u8,
    pub message: Option<String>,
}

/// Constructs the canonical protocol version 1 compiler request.
///
/// # Errors
///
/// Returns a diagnostic when an executable command lacks an executable target
/// or requests native emission for an unsupported target.
pub fn build_request(
    graph: &PackageGraph,
    command: ProjectCommand,
    target: Target,
) -> Result<CompilerRequest, Diagnostic> {
    let output_kind = match command {
        ProjectCommand::Check => "check",
        ProjectCommand::Build | ProjectCommand::Run => "executable",
    };
    let executable = if command == ProjectCommand::Check {
        graph.root_executable.as_ref()
    } else {
        Some(graph.root_executable.as_ref().ok_or_else(|| {
            Diagnostic::global(format!(
                "package '{}' has no [executable] target",
                graph.root_package
            ))
        })?)
    };
    if output_kind == "executable" && target != Target::X86_64 {
        return Err(Diagnostic::global(
            "native executable output currently supports only target 'x86_64'",
        ));
    }

    let output_path = if output_kind == "executable" {
        let root = graph
            .packages
            .get(&graph.root_package)
            .ok_or_else(|| Diagnostic::global("package graph does not contain its root package"))?;
        let executable = executable.ok_or_else(|| {
            Diagnostic::global("executable compiler request does not contain a target")
        })?;
        let mut file_name = executable.name.clone();
        if !std::env::consts::EXE_EXTENSION.is_empty() {
            file_name.push('.');
            file_name.push_str(std::env::consts::EXE_EXTENSION);
        }
        Some(
            windows_compatible_path(&root.package_root)
                .join("target")
                .join(target.to_string())
                .join(file_name),
        )
    } else {
        None
    };

    let mut arguments = vec![
        OsString::from("--shuttle-protocol"),
        OsString::from(PROTOCOL_VERSION),
        OsString::from("--target"),
        OsString::from(target.to_string()),
        OsString::from("--output-kind"),
        OsString::from(output_kind),
    ];
    if let Some(path) = &output_path {
        arguments.push(OsString::from("--output"));
        arguments.push(protocol_path_argument(path));
    }
    arguments.push(OsString::from("--root-package"));
    arguments.push(OsString::from(&graph.root_package));
    if let Some(executable) = executable {
        arguments.push(OsString::from("--entry"));
        arguments.push(executable.entry.as_os_str().to_owned());
    }
    for package in graph.packages.values() {
        arguments.push(OsString::from("--package"));
        arguments.push(OsString::from(&package.name));
        arguments.push(OsString::from(package.version.to_string()));
        arguments.push(protocol_path_argument(&package.source_root));
    }
    for dependency in &graph.dependencies {
        arguments.push(OsString::from("--dependency"));
        arguments.push(OsString::from(&dependency.owner));
        arguments.push(OsString::from(&dependency.alias));
        arguments.push(OsString::from(&dependency.target));
    }

    Ok(CompilerRequest {
        root_package: graph.root_package.clone(),
        arguments,
        output_path,
        run_after_build: command == ProjectCommand::Run,
    })
}

/// Selects an absolute compiler executable using the protocol precedence.
///
/// # Errors
///
/// Returns a diagnostic when an explicit compiler is invalid or no compiler is
/// available beside Shuttle or on `PATH`.
pub fn select_compiler(
    explicit_path: Option<&Path>,
    current_directory: &Path,
) -> Result<PathBuf, Diagnostic> {
    if let Some(path) = explicit_path {
        let candidate = if path.is_absolute() {
            path.to_path_buf()
        } else {
            current_directory.join(path)
        };
        return canonical_executable(&candidate).map_err(|message| {
            Diagnostic::global(format!(
                "invalid compiler '{}': {message}",
                display_path(&candidate)
            ))
        });
    }

    if let Some(directory) = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf))
    {
        let sibling = directory.join(compiler_file_name());
        if sibling.is_file() {
            return canonical_executable(&sibling).map_err(Diagnostic::global);
        }
    }

    if let Some(search_path) = std::env::var_os("PATH") {
        for directory in std::env::split_paths(&search_path) {
            let candidate = directory.join(compiler_file_name());
            if candidate.is_file() {
                return canonical_executable(&candidate).map_err(Diagnostic::global);
            }
        }
    }
    Err(Diagnostic::global(
        "could not find 'clothc'; use --compiler to select the compiler executable",
    ))
}

/// Verifies and invokes the selected compiler, then runs a requested program.
///
/// # Errors
///
/// Returns a process failure for an incompatible compiler, filesystem or spawn
/// failure, compiler rejection, or a nonzero program exit.
pub fn execute_request(compiler: &Path, request: &CompilerRequest) -> Result<(), ProcessFailure> {
    verify_protocol(compiler)?;
    if let Some(parent) = request.output_path().and_then(Path::parent) {
        if let Err(error) = fs::create_dir_all(parent) {
            return Err(process_error(format!(
                "could not create output directory '{}': {error}",
                display_path(parent)
            )));
        }
    }

    let status = Command::new(compiler)
        .args(request.arguments())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|error| {
            process_error(format!(
                "could not start compiler '{}': {error}",
                display_path(compiler)
            ))
        })?;
    if !status.success() {
        let mut failure = compiler_status_failure(status.code());
        if let Some(message) = &mut failure.message {
            *message = format!(
                "package '{}', compiler '{}': {message}",
                request.root_package,
                display_path(compiler)
            );
        }
        return Err(failure);
    }

    if request.run_after_build {
        let Some(output) = request.output_path() else {
            return Err(process_error(
                "run request does not contain an executable output".to_owned(),
            ));
        };
        let status = Command::new(output)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map_err(|error| {
                process_error(format!(
                    "could not run executable '{}': {error}",
                    display_path(output)
                ))
            })?;
        if !status.success() {
            return Err(program_status_failure(status.code()));
        }
    }
    Ok(())
}

fn verify_protocol(compiler: &Path) -> Result<(), ProcessFailure> {
    let output = Command::new(compiler)
        .arg("--shuttle-protocol-version")
        .output()
        .map_err(|error| {
            process_error(format!(
                "could not query compiler '{}': {error}",
                display_path(compiler)
            ))
        })?;
    let valid_stdout = output.stdout == b"1\n" || output.stdout == b"1\r\n";
    if !output.status.success() || !valid_stdout || !output.stderr.is_empty() {
        return Err(process_error(format!(
            "compiler '{}' does not support Shuttle protocol version 1",
            display_path(compiler)
        )));
    }
    Ok(())
}

fn canonical_executable(path: &Path) -> Result<PathBuf, String> {
    match fs::canonicalize(path) {
        Ok(path) if path.is_file() => Ok(path),
        Ok(_) => Err("path is not a regular file".to_owned()),
        Err(error) => Err(error.to_string()),
    }
}

fn compiler_file_name() -> String {
    if std::env::consts::EXE_EXTENSION.is_empty() {
        "clothc".to_owned()
    } else {
        format!("clothc.{}", std::env::consts::EXE_EXTENSION)
    }
}

fn protocol_path_argument(path: &Path) -> OsString {
    windows_compatible_path(path).into_os_string()
}

fn windows_compatible_path(path: &Path) -> PathBuf {
    #[cfg(windows)]
    {
        let value = path.to_string_lossy();
        if let Some(rest) = value.strip_prefix(r"\\?\UNC\") {
            return PathBuf::from(format!(r"\\{rest}"));
        }
        if let Some(rest) = value.strip_prefix(r"\\?\") {
            return PathBuf::from(rest);
        }
    }
    path.to_path_buf()
}

fn process_error(message: String) -> ProcessFailure {
    ProcessFailure {
        exit_code: 2,
        message: Some(message),
    }
}

fn compiler_status_failure(code: Option<i32>) -> ProcessFailure {
    match code {
        Some(code @ (1 | 2)) => ProcessFailure {
            exit_code: if code == 1 { 1 } else { 2 },
            message: None,
        },
        Some(code) => process_error(format!(
            "compiler terminated with unsupported exit status {code}"
        )),
        None => process_error("compiler terminated without an exit status".to_owned()),
    }
}

fn program_status_failure(code: Option<i32>) -> ProcessFailure {
    match code.and_then(|value| u8::try_from(value).ok()) {
        Some(exit_code) => ProcessFailure {
            exit_code,
            message: None,
        },
        None => process_error("program terminated without a supported exit status".to_owned()),
    }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
