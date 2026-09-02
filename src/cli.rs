// Part of the Cloth Compiler project, under the Apache License v2.0 with LLVM
// Exceptions. See LICENSE.txt in the project root for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use clap::{Args, Parser, Subcommand};

use crate::compiler::{
    ProgressMode, ProjectCommand, Target, execute_graph, select_compiler, validate_project_command,
};
use crate::diagnostic::Diagnostic;
use crate::graph::resolve_package_graph;
use crate::manifest::resolve_manifest_path;

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

    /// Suppress successful build progress.
    #[arg(long)]
    quiet: bool,

    /// Limit concurrent package compiler processes.
    #[arg(long, value_name = "COUNT")]
    jobs: Option<NonZeroUsize>,
}

#[derive(Debug)]
pub struct CommandFailure {
    pub exit_code: u8,
    pub diagnostics: Vec<Diagnostic>,
}

/// Resolves, validates, and executes the selected project command.
///
/// # Errors
///
/// Returns project diagnostics when discovery, graph validation, compiler
/// selection, protocol negotiation, compilation, or program execution fails.
pub fn execute(cli: Cli, current_directory: &Path) -> Result<(), CommandFailure> {
    let (project_command, options) = match cli.command {
        Command::Check(options) => (ProjectCommand::Check, options),
        Command::Build(options) => (ProjectCommand::Build, options),
        Command::Run(options) => (ProjectCommand::Run, options),
    };

    let manifest_path = resolve_manifest_path(options.manifest_path.as_deref(), current_directory)
        .map_err(|diagnostic| CommandFailure {
            exit_code: 1,
            diagnostics: vec![diagnostic],
        })?;
    let graph = resolve_package_graph(&manifest_path).map_err(|diagnostics| CommandFailure {
        exit_code: 1,
        diagnostics,
    })?;
    validate_project_command(&graph, project_command, options.target).map_err(|diagnostic| {
        CommandFailure {
            exit_code: 1,
            diagnostics: vec![diagnostic],
        }
    })?;
    let compiler =
        select_compiler(options.compiler.as_deref(), current_directory).map_err(|diagnostic| {
            CommandFailure {
                exit_code: 2,
                diagnostics: vec![diagnostic],
            }
        })?;
    let progress = if options.quiet {
        ProgressMode::Quiet
    } else {
        ProgressMode::Visible
    };
    let jobs = options.jobs.map_or_else(
        || std::thread::available_parallelism().map_or(1, NonZeroUsize::get),
        NonZeroUsize::get,
    );
    execute_graph(
        &compiler,
        &graph,
        project_command,
        options.target,
        progress,
        jobs,
    )
    .map_err(|failure| CommandFailure {
        exit_code: failure.exit_code,
        diagnostics: failure
            .message
            .map(Diagnostic::global)
            .into_iter()
            .collect(),
    })
}
