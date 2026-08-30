use std::fmt;
use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::diagnostic::Diagnostic;
use crate::manifest::{load_manifest, resolve_manifest_path};

#[derive(Debug, Parser)]
#[command(
    name = "shuttle",
    version,
    about = "Cloth's project, build, and package manager"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate and type-check a Cloth package graph.
    Check(ProjectOptions),
    /// Build the root package executable.
    Build(ProjectOptions),
    /// Build and run the root package executable.
    Run(ProjectOptions),
}

#[derive(Debug, Args)]
struct ProjectOptions {
    /// Use this Shuttle.toml instead of searching parent directories.
    #[arg(long, value_name = "PATH")]
    manifest_path: Option<PathBuf>,

    /// Use this clothc executable.
    #[arg(long, value_name = "PATH")]
    compiler: Option<PathBuf>,

    /// Select the Cloth compilation target.
    #[arg(long, value_enum, default_value_t = Target::X86_64)]
    target: Target,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum Target {
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

#[derive(Debug)]
pub struct CommandFailure {
    pub exit_code: u8,
    pub diagnostics: Vec<Diagnostic>,
}

/// Resolves and validates the selected project command.
///
/// # Errors
///
/// Returns project diagnostics when manifest discovery or validation fails.
/// Valid Stage 22 commands currently stop at the Stage 22.3 integration
/// boundary with exit status 2.
pub fn execute(cli: Cli, current_directory: &Path) -> Result<(), CommandFailure> {
    let (command_name, options) = match cli.command {
        Command::Check(options) => ("check", options),
        Command::Build(options) => ("build", options),
        Command::Run(options) => ("run", options),
    };

    let manifest_path = resolve_manifest_path(options.manifest_path.as_deref(), current_directory)
        .map_err(|diagnostic| CommandFailure {
            exit_code: 1,
            diagnostics: vec![diagnostic],
        })?;
    let manifest = load_manifest(&manifest_path).map_err(|diagnostics| CommandFailure {
        exit_code: 1,
        diagnostics,
    })?;

    let compiler_context = options.compiler.as_ref().map_or_else(
        || "the default compiler".to_owned(),
        |path| format!("compiler '{}'", display_path(path)),
    );
    Err(CommandFailure {
        exit_code: 2,
        diagnostics: vec![Diagnostic::global(format!(
            "command '{command_name}' for package '{}' and target '{}' is unavailable until Stage 22.3 ({compiler_context})",
            manifest.package.name, options.target
        ))],
    })
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
