// Part of the Cloth Compiler project, under the Apache License v2.0 with LLVM
// Exceptions. See LICENSE.txt in the project root for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use clap::ValueEnum;
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

use crate::diagnostic::Diagnostic;
use crate::graph::PackageGraph;

const PROTOCOL_VERSION: &str = "1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectCommand {
    Check,
    Build,
    Run,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgressMode {
    Visible,
    Quiet,
}

struct BuildProgress {
    mode: ProgressMode,
    command: ProjectCommand,
    target: Target,
    package_count: usize,
    started: Instant,
}

impl BuildProgress {
    fn start(
        mode: ProgressMode,
        command: ProjectCommand,
        target: Target,
        package_count: usize,
    ) -> Self {
        let progress = Self {
            mode,
            command,
            target,
            package_count,
            started: Instant::now(),
        };
        progress.write(format_args!(
            "preparing {} for {} ({} package{})",
            progress.build_action(),
            target,
            package_count,
            if package_count == 1 { "" } else { "s" }
        ));
        progress
    }

    fn package(&self, index: usize, package: &crate::graph::PackageRecord) {
        let action = if self.command == ProjectCommand::Check {
            "checking"
        } else {
            "compiling"
        };
        self.package_status(action, index, package);
    }

    fn validating(&self, index: usize, package: &crate::graph::PackageRecord) {
        self.package_status("validating", index, package);
    }

    fn reusing(&self, index: usize, package: &crate::graph::PackageRecord) {
        self.package_status("reusing", index, package);
    }

    fn package_status(&self, action: &str, index: usize, package: &crate::graph::PackageRecord) {
        self.write(format_args!(
            "{action} {} v{} [{}/{}]",
            package.name,
            package.version,
            index + 1,
            self.package_count
        ));
    }

    fn scheduling(&self, jobs: usize) {
        self.write(format_args!(
            "scheduling with {jobs} job{}",
            if jobs == 1 { "" } else { "s" }
        ));
    }

    fn linking(&self, package: &str) {
        self.write(format_args!("linking {package}"));
    }

    fn finished(&self) {
        self.write(format_args!(
            "finished {} for {} in {}",
            self.build_action(),
            self.target,
            display_duration(self.started.elapsed())
        ));
    }

    fn running(&self, output: &Path) {
        self.write(format_args!("running {}", display_path(output)));
    }

    fn build_action(&self) -> &'static str {
        if self.command == ProjectCommand::Check {
            "check"
        } else {
            "build"
        }
    }

    fn write(&self, message: std::fmt::Arguments<'_>) {
        if self.mode == ProgressMode::Visible {
            eprintln!("shuttle: {message}");
        }
    }
}

fn display_duration(duration: Duration) -> String {
    if duration < Duration::from_secs(1) {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{:.2}s", duration.as_secs_f64())
    }
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
    validate_project_command(graph, command, target)?;
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

/// Validates target-independent command requirements before compiler selection.
///
/// # Errors
///
/// Returns a diagnostic when a build lacks an executable or selects an
/// unsupported native target.
pub fn validate_project_command(
    graph: &PackageGraph,
    command: ProjectCommand,
    target: Target,
) -> Result<(), Diagnostic> {
    if command != ProjectCommand::Check && graph.root_executable.is_none() {
        return Err(Diagnostic::global(format!(
            "package '{}' has no [executable] target",
            graph.root_package
        )));
    }
    if command != ProjectCommand::Check && target != Target::X86_64 {
        return Err(Diagnostic::global(
            "native executable output currently supports only target 'x86_64'",
        ));
    }
    Ok(())
}

const PROTOCOL_V2: &str = "2";
const ARTIFACT_FORMAT: u32 = 3;
const CAPABILITY_LIMIT: usize = 64 * 1024;
const RECEIPT_LIMIT: usize = 16 * 1024 * 1024;
const BUILD_STATE_SCHEMA: u32 = 1;
const BUILD_STATE_LIMIT: u64 = 32 * 1024 * 1024;
static BUILD_STATE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Deserialize)]
struct Capabilities {
    schema: u32,
    protocols: Vec<u32>,
    artifact_formats: Vec<u32>,
    compiler_id: String,
    operations: Vec<String>,
    interface_targets: Vec<String>,
    object_targets: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ReceiptPackage {
    name: String,
    version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ReceiptDependency {
    alias: String,
    package: ReceiptPackage,
    artifact_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct ArtifactReceipt {
    schema: u32,
    artifact_format: u32,
    artifact_id: String,
    kind: String,
    package: ReceiptPackage,
    target: String,
    compiler_id: String,
    dependencies: Vec<ReceiptDependency>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PackageBuildState {
    schema: u32,
    manifest: String,
    receipt: ArtifactReceipt,
}

#[derive(Clone, Debug)]
struct ProducedArtifact {
    path: PathBuf,
    receipt: ArtifactReceipt,
}

struct PackageCompilation<'a> {
    graph: &'a PackageGraph,
    target: Target,
    artifact_kind: &'a str,
    compiler_id: &'a str,
    order: &'a [String],
    artifact_directory: &'a Path,
    state_directory: &'a Path,
    progress: &'a BuildProgress,
    jobs: usize,
}

struct PackageTask<'a> {
    index: usize,
    name: &'a str,
    package: &'a crate::graph::PackageRecord,
    output: PathBuf,
    cached: Vec<ArtifactReceipt>,
    candidate: bool,
}

struct CapturedProcess {
    status: std::process::ExitStatus,
    stdout: Vec<u8>,
    stdout_oversized: bool,
    stderr: NamedTempFile,
    stderr_length: u64,
}

struct CompilerExecution<T> {
    captured: Option<CapturedProcess>,
    result: Result<T, ProcessFailure>,
}

impl<T> CompilerExecution<T> {
    fn failed(failure: ProcessFailure) -> Self {
        Self {
            captured: None,
            result: Err(failure),
        }
    }
}

struct BuildLock {
    file: File,
}

impl BuildLock {
    fn acquire(directory: &Path) -> Result<Self, ProcessFailure> {
        fs::create_dir_all(directory).map_err(|error| {
            process_error(format!(
                "could not create target directory '{}': {error}",
                display_path(directory)
            ))
        })?;
        let path = directory.join(".shuttle.lock");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .map_err(|error| {
                process_error(format!(
                    "could not open build lock '{}': {error}",
                    display_path(&path)
                ))
            })?;
        file.try_lock_exclusive().map_err(|error| {
            process_error(format!(
                "another Shuttle build owns '{}': {error}",
                display_path(directory)
            ))
        })?;
        Ok(Self { file })
    }
}

impl Drop for BuildLock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
    }
}

struct BuildWorkspace {
    artifact_directory: PathBuf,
    state_directory: PathBuf,
    _lock: BuildLock,
}

/// Executes protocol-v2 separate package compilation and linking.
///
/// # Errors
///
/// Returns a process failure for capability, compiler, receipt, filesystem,
/// locking, linking, or launched-program failures.
pub fn execute_graph(
    compiler: &Path,
    graph: &PackageGraph,
    command: ProjectCommand,
    target: Target,
    progress_mode: ProgressMode,
    jobs: usize,
) -> Result<(), ProcessFailure> {
    if jobs == 0 {
        return Err(process_error(
            "package job limit must be greater than zero".to_owned(),
        ));
    }
    if command != ProjectCommand::Check && target != Target::X86_64 {
        return Err(process_error(
            "native executable output currently supports only target 'x86_64'".to_owned(),
        ));
    }
    let progress = BuildProgress::start(progress_mode, command, target, graph.packages.len());
    let capabilities = query_capabilities(compiler)?;
    validate_capabilities(&capabilities, target, command)?;
    let order = topological_order(graph)?;
    let effective_jobs = jobs.min(order.len().max(1));
    progress.scheduling(effective_jobs);
    let workspace = create_build_workspace(graph, command, target)?;
    let artifact_kind = if command == ProjectCommand::Check {
        "interface"
    } else {
        "object"
    };
    let produced = compile_packages(
        compiler,
        &PackageCompilation {
            graph,
            target,
            artifact_kind,
            compiler_id: &capabilities.compiler_id,
            order: &order,
            artifact_directory: &workspace.artifact_directory,
            state_directory: &workspace.state_directory,
            progress: &progress,
            jobs: effective_jobs,
        },
    )?;
    if command != ProjectCommand::Check {
        progress.linking(&graph.root_package);
        link_executable(compiler, graph, target, &produced)?;
    }
    progress.finished();
    if command == ProjectCommand::Run {
        let output = executable_output(graph, target)?;
        progress.running(&output);
        run_executable(&output)?;
    }
    Ok(())
}

fn create_build_workspace(
    graph: &PackageGraph,
    command: ProjectCommand,
    target: Target,
) -> Result<BuildWorkspace, ProcessFailure> {
    let root = graph
        .packages
        .get(&graph.root_package)
        .ok_or_else(|| process_error("package graph is missing its root package".to_owned()))?;
    let target_directory = windows_compatible_path(&root.package_root)
        .join("target")
        .join(target.to_string());
    let workspace_directory = if command == ProjectCommand::Check {
        target_directory.join("check")
    } else {
        target_directory
    };
    let lock = BuildLock::acquire(&workspace_directory)?;
    let artifact_directory = workspace_directory.join("packages");
    let state_directory = workspace_directory.join(".shuttle").join("state");
    fs::create_dir_all(&artifact_directory).map_err(|error| {
        process_error(format!(
            "could not create package artifact directory '{}': {error}",
            display_path(&artifact_directory)
        ))
    })?;
    fs::create_dir_all(&state_directory).map_err(|error| {
        process_error(format!(
            "could not create build state directory '{}': {error}",
            display_path(&state_directory)
        ))
    })?;
    Ok(BuildWorkspace {
        artifact_directory,
        state_directory,
        _lock: lock,
    })
}

fn cached_receipts(
    state_directory: &Path,
    package: &crate::graph::PackageRecord,
) -> Vec<ArtifactReceipt> {
    let directory = state_directory.join(&package.name);
    let Ok(entries) = fs::read_dir(directory) else {
        return Vec::new();
    };
    let mut paths = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let file_type = entry.file_type().ok()?;
            let path = entry.path();
            (file_type.is_file() && path.extension().is_some_and(|value| value == "json"))
                .then_some(path)
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
        .into_iter()
        .filter_map(|path| {
            let metadata = fs::metadata(&path).ok()?;
            if metadata.len() > BUILD_STATE_LIMIT {
                return None;
            }
            let bytes = fs::read(path).ok()?;
            if u64::try_from(bytes.len()).ok()? > BUILD_STATE_LIMIT {
                return None;
            }
            let state = serde_json::from_slice::<PackageBuildState>(&bytes).ok()?;
            (state.schema == BUILD_STATE_SCHEMA && state.manifest == package.manifest_contents)
                .then_some(state.receipt)
        })
        .collect()
}

fn publish_build_state(
    state_directory: &Path,
    package: &crate::graph::PackageRecord,
    receipt: &ArtifactReceipt,
) -> Result<(), ProcessFailure> {
    if !valid_digest(&receipt.artifact_id) {
        return Err(process_error(
            "refusing to publish build state with an invalid artifact identity".to_owned(),
        ));
    }
    let directory = state_directory.join(&package.name);
    fs::create_dir_all(&directory).map_err(|error| {
        process_error(format!(
            "could not create package build state directory '{}': {error}",
            display_path(&directory)
        ))
    })?;
    let state = PackageBuildState {
        schema: BUILD_STATE_SCHEMA,
        manifest: package.manifest_contents.clone(),
        receipt: receipt.clone(),
    };
    let bytes = serde_json::to_vec(&state)
        .map_err(|error| process_error(format!("could not encode package build state: {error}")))?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > BUILD_STATE_LIMIT {
        return Err(process_error(
            "package build state exceeds its 32 MiB limit".to_owned(),
        ));
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    let sequence = BUILD_STATE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let stem = format!(
        "{}-{}-{timestamp}-{sequence}",
        receipt.artifact_id,
        std::process::id()
    );
    let temporary = directory.join(format!(".{stem}.tmp"));
    let published = directory.join(format!("{stem}.json"));
    if published.try_exists().map_err(|error| {
        process_error(format!(
            "could not inspect build state destination '{}': {error}",
            display_path(&published)
        ))
    })? {
        return Err(process_error(format!(
            "build state destination already exists: '{}'",
            display_path(&published)
        )));
    }
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|error| {
            process_error(format!(
                "could not create private build state '{}': {error}",
                display_path(&temporary)
            ))
        })?;
    let write_result = output.write_all(&bytes).and_then(|()| output.sync_all());
    drop(output);
    if let Err(error) = write_result {
        let _ = fs::remove_file(&temporary);
        return Err(process_error(format!(
            "could not write private build state '{}': {error}",
            display_path(&temporary)
        )));
    }
    if let Err(error) = fs::rename(&temporary, &published) {
        let _ = fs::remove_file(&temporary);
        return Err(process_error(format!(
            "could not atomically publish build state '{}': {error}",
            display_path(&published)
        )));
    }

    if let Ok(entries) = fs::read_dir(&directory) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path != published
                && path.extension().is_some_and(|value| value == "json")
                && entry.file_type().is_ok_and(|file_type| file_type.is_file())
            {
                let _ = fs::remove_file(path);
            }
        }
    }
    Ok(())
}

fn compile_packages(
    compiler: &Path,
    compilation: &PackageCompilation<'_>,
) -> Result<BTreeMap<String, ProducedArtifact>, ProcessFailure> {
    let mut produced = BTreeMap::<String, ProducedArtifact>::new();
    for level in dependency_levels(compilation.graph, compilation.order)? {
        compile_package_level(compiler, compilation, &level, &mut produced)?;
    }

    Ok(produced)
}

fn compile_package_level(
    compiler: &Path,
    compilation: &PackageCompilation<'_>,
    level: &[(usize, String)],
    produced: &mut BTreeMap<String, ProducedArtifact>,
) -> Result<(), ProcessFailure> {
    let tasks = level
        .iter()
        .map(|(index, package_name)| {
            let package = compilation
                .graph
                .packages
                .get(package_name)
                .ok_or_else(|| process_error("ordered package is missing".to_owned()))?;
            let output = compilation
                .artifact_directory
                .join(format!("{package_name}.cpa"));
            let cached = cached_receipts(compilation.state_directory, package);
            let candidate = output.is_file() && !cached.is_empty();
            Ok(PackageTask {
                index: *index,
                name: package_name,
                package,
                output,
                cached,
                candidate,
            })
        })
        .collect::<Result<Vec<_>, ProcessFailure>>()?;
    let compile_positions = validate_package_candidates(compiler, compilation, &tasks, produced)?;
    compile_selected_packages(compiler, compilation, &tasks, &compile_positions, produced)
}

fn validate_package_candidates(
    compiler: &Path,
    compilation: &PackageCompilation<'_>,
    tasks: &[PackageTask<'_>],
    produced: &mut BTreeMap<String, ProducedArtifact>,
) -> Result<BTreeSet<usize>, ProcessFailure> {
    let mut compile_positions = tasks
        .iter()
        .enumerate()
        .filter(|(_, task)| !task.candidate)
        .map(|(position, _)| position)
        .collect::<BTreeSet<_>>();
    let candidate_positions = tasks
        .iter()
        .enumerate()
        .filter(|(_, task)| task.candidate)
        .map(|(position, _)| position)
        .collect::<Vec<_>>();
    for positions in candidate_positions.chunks(compilation.jobs) {
        for position in positions {
            let task = &tasks[*position];
            compilation.progress.validating(task.index, task.package);
        }
        let mut validations =
            run_candidate_validations(compiler, compilation, tasks, positions, produced);
        for position in positions {
            let task = &tasks[*position];
            let execution = validations.remove(position).ok_or_else(|| {
                process_error(format!(
                    "package '{}' validation worker returned no result",
                    task.name
                ))
            })?;
            if let Some(captured) = &execution.captured {
                replay_compiler_stderr(captured)?;
            }
            if let Some(receipt) = execution.result? {
                compilation.progress.reusing(task.index, task.package);
                produced.insert(
                    task.name.to_owned(),
                    ProducedArtifact {
                        path: task.output.clone(),
                        receipt,
                    },
                );
            } else {
                compile_positions.insert(*position);
            }
        }
    }
    Ok(compile_positions)
}

fn run_candidate_validations(
    compiler: &Path,
    compilation: &PackageCompilation<'_>,
    tasks: &[PackageTask<'_>],
    positions: &[usize],
    produced: &BTreeMap<String, ProducedArtifact>,
) -> BTreeMap<usize, CompilerExecution<Option<ArtifactReceipt>>> {
    thread::scope(|scope| {
        let handles = positions
            .iter()
            .map(|position| {
                let position = *position;
                let task = &tasks[position];
                (
                    position,
                    scope.spawn(move || {
                        validate_package_candidate(compiler, compilation, task, produced)
                    }),
                )
            })
            .collect::<Vec<_>>();
        handles
            .into_iter()
            .map(|(position, handle)| {
                (
                    position,
                    handle.join().unwrap_or_else(|_| {
                        CompilerExecution::failed(process_error(
                            "package validation worker panicked".to_owned(),
                        ))
                    }),
                )
            })
            .collect()
    })
}

fn compile_selected_packages(
    compiler: &Path,
    compilation: &PackageCompilation<'_>,
    tasks: &[PackageTask<'_>],
    compile_positions: &BTreeSet<usize>,
    produced: &mut BTreeMap<String, ProducedArtifact>,
) -> Result<(), ProcessFailure> {
    let positions = compile_positions.iter().copied().collect::<Vec<_>>();
    for positions in positions.chunks(compilation.jobs) {
        for position in positions {
            let task = &tasks[*position];
            compilation.progress.package(task.index, task.package);
        }
        let mut compilations =
            run_package_compilations(compiler, compilation, tasks, positions, produced);
        for position in positions {
            let task = &tasks[*position];
            let execution = compilations.remove(position).ok_or_else(|| {
                process_error(format!(
                    "package '{}' compiler worker returned no result",
                    task.name
                ))
            })?;
            if let Some(captured) = &execution.captured {
                replay_compiler_stderr(captured)?;
            }
            let receipt = execution.result?;
            publish_build_state(compilation.state_directory, task.package, &receipt)?;
            produced.insert(
                task.name.to_owned(),
                ProducedArtifact {
                    path: task.output.clone(),
                    receipt,
                },
            );
        }
    }
    Ok(())
}

fn run_package_compilations(
    compiler: &Path,
    compilation: &PackageCompilation<'_>,
    tasks: &[PackageTask<'_>],
    positions: &[usize],
    produced: &BTreeMap<String, ProducedArtifact>,
) -> BTreeMap<usize, CompilerExecution<ArtifactReceipt>> {
    thread::scope(|scope| {
        let handles = positions
            .iter()
            .map(|position| {
                let position = *position;
                let task = &tasks[position];
                (
                    position,
                    scope.spawn(move || {
                        compile_package_artifact(compiler, compilation, task, produced)
                    }),
                )
            })
            .collect::<Vec<_>>();
        handles
            .into_iter()
            .map(|(position, handle)| {
                (
                    position,
                    handle.join().unwrap_or_else(|_| {
                        CompilerExecution::failed(process_error(
                            "package compiler worker panicked".to_owned(),
                        ))
                    }),
                )
            })
            .collect()
    })
}

fn validate_package_candidate(
    compiler: &Path,
    compilation: &PackageCompilation<'_>,
    task: &PackageTask<'_>,
    produced: &BTreeMap<String, ProducedArtifact>,
) -> CompilerExecution<Option<ArtifactReceipt>> {
    let arguments = match package_arguments(
        compilation.graph,
        task.name,
        compilation.target,
        compilation.artifact_kind,
        PackageOperation::Reuse(&task.output),
        produced,
    ) {
        Ok(arguments) => arguments,
        Err(failure) => return CompilerExecution::failed(failure),
    };
    let captured = match invoke_compiler(compiler, &arguments, RECEIPT_LIMIT) {
        Ok(captured) => captured,
        Err(failure) => return CompilerExecution::failed(failure),
    };
    let result = parse_reuse_receipt(compiler, task.name, &captured).and_then(|receipt| {
        receipt
            .map(|receipt| {
                validate_compile_receipt(
                    &receipt,
                    task.package,
                    compilation.artifact_kind,
                    compilation.target,
                    compilation.compiler_id,
                    compilation.graph,
                    produced,
                )?;
                Ok(task.cached.contains(&receipt).then_some(receipt))
            })
            .transpose()
            .map(Option::flatten)
    });
    CompilerExecution {
        captured: Some(captured),
        result,
    }
}

fn compile_package_artifact(
    compiler: &Path,
    compilation: &PackageCompilation<'_>,
    task: &PackageTask<'_>,
    produced: &BTreeMap<String, ProducedArtifact>,
) -> CompilerExecution<ArtifactReceipt> {
    let arguments = match package_arguments(
        compilation.graph,
        task.name,
        compilation.target,
        compilation.artifact_kind,
        PackageOperation::Compile(&task.output),
        produced,
    ) {
        Ok(arguments) => arguments,
        Err(failure) => return CompilerExecution::failed(failure),
    };
    let captured = match invoke_compiler(compiler, &arguments, RECEIPT_LIMIT) {
        Ok(captured) => captured,
        Err(failure) => return CompilerExecution::failed(failure),
    };
    let result = require_compiler_success(compiler, task.name, "compile", &captured)
        .and_then(|()| parse_receipt(&captured.stdout))
        .and_then(|receipt| {
            validate_compile_receipt(
                &receipt,
                task.package,
                compilation.artifact_kind,
                compilation.target,
                compilation.compiler_id,
                compilation.graph,
                produced,
            )?;
            if !task.output.is_file() {
                return Err(process_error(format!(
                    "compiler reported success without artifact '{}'",
                    display_path(&task.output)
                )));
            }
            Ok(receipt)
        });
    CompilerExecution {
        captured: Some(captured),
        result,
    }
}

fn parse_reuse_receipt(
    compiler: &Path,
    package: &str,
    captured: &CapturedProcess,
) -> Result<Option<ArtifactReceipt>, ProcessFailure> {
    if captured.status.code() == Some(3) {
        if captured.stdout_oversized || !captured.stdout.is_empty() || captured.stderr_length != 0 {
            return Err(process_error(format!(
                "package '{package}', reuse: compiler cache miss polluted its protocol streams"
            )));
        }
        return Ok(None);
    }
    require_compiler_success(compiler, package, "reuse", captured)?;
    if captured.stderr_length != 0 {
        return Err(process_error(format!(
            "package '{package}', reuse: compiler success produced unexpected stderr"
        )));
    }
    parse_receipt(&captured.stdout).map(Some)
}

#[derive(Clone, Copy)]
enum PackageOperation<'a> {
    Compile(&'a Path),
    Reuse(&'a Path),
}

fn package_arguments(
    graph: &PackageGraph,
    package_name: &str,
    target: Target,
    artifact_kind: &str,
    operation: PackageOperation<'_>,
    produced: &BTreeMap<String, ProducedArtifact>,
) -> Result<Vec<OsString>, ProcessFailure> {
    let package = graph
        .packages
        .get(package_name)
        .ok_or_else(|| process_error("ordered package is missing".to_owned()))?;
    let (operation_name, path_option, path) = match operation {
        PackageOperation::Compile(path) => ("compile", "--output", path),
        PackageOperation::Reuse(path) => ("reuse", "--input", path),
    };
    let mut arguments = vec![
        OsString::from("--shuttle-protocol"),
        OsString::from(PROTOCOL_V2),
        OsString::from("--operation"),
        OsString::from(operation_name),
        OsString::from("--target"),
        OsString::from(target.to_string()),
        OsString::from("--artifact-kind"),
        OsString::from(artifact_kind),
        OsString::from(path_option),
        protocol_path_argument(path),
        OsString::from("--package"),
        OsString::from(&package.name),
        OsString::from(package.version.to_string()),
        protocol_path_argument(&package.source_root),
    ];
    if package_name == graph.root_package {
        if let Some(executable) = &graph.root_executable {
            arguments.extend([
                OsString::from("--entry"),
                executable.entry.as_os_str().to_owned(),
            ]);
        }
    }
    for dependency in graph
        .dependencies
        .iter()
        .filter(|dependency| dependency.owner == package_name)
    {
        arguments.extend([
            OsString::from("--dependency"),
            OsString::from(&dependency.alias),
            OsString::from(&dependency.target),
        ]);
    }
    for dependency_name in transitive_dependencies(graph, package_name) {
        let artifact = produced.get(&dependency_name).ok_or_else(|| {
            process_error(format!(
                "package '{package_name}' was scheduled before dependency '{dependency_name}'"
            ))
        })?;
        arguments.extend([
            OsString::from("--artifact"),
            OsString::from(&artifact.receipt.package.name),
            OsString::from(&artifact.receipt.package.version),
            OsString::from(&artifact.receipt.artifact_id),
            protocol_path_argument(&artifact.path),
        ]);
    }
    Ok(arguments)
}

fn link_executable(
    compiler: &Path,
    graph: &PackageGraph,
    target: Target,
    produced: &BTreeMap<String, ProducedArtifact>,
) -> Result<(), ProcessFailure> {
    let executable = graph.root_executable.as_ref().ok_or_else(|| {
        process_error(format!(
            "package '{}' has no [executable] target",
            graph.root_package
        ))
    })?;
    let output = executable_output(graph, target)?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            process_error(format!(
                "could not create executable directory '{}': {error}",
                display_path(parent)
            ))
        })?;
    }
    let mut arguments = vec![
        OsString::from("--shuttle-protocol"),
        OsString::from(PROTOCOL_V2),
        OsString::from("--operation"),
        OsString::from("link"),
        OsString::from("--target"),
        OsString::from(target.to_string()),
        OsString::from("--output"),
        protocol_path_argument(&output),
        OsString::from("--root-package"),
        OsString::from(&graph.root_package),
        OsString::from("--entry"),
        executable.entry.as_os_str().to_owned(),
    ];
    for artifact in produced.values() {
        arguments.push(OsString::from("--artifact"));
        arguments.push(OsString::from(&artifact.receipt.package.name));
        arguments.push(OsString::from(&artifact.receipt.package.version));
        arguments.push(OsString::from(&artifact.receipt.artifact_id));
        arguments.push(protocol_path_argument(&artifact.path));
    }
    let captured = invoke_compiler(compiler, &arguments, 1)?;
    replay_compiler_stderr(&captured)?;
    require_compiler_success(compiler, &graph.root_package, "link", &captured)?;
    if !captured.stdout.is_empty() {
        return Err(process_error(
            "compiler link operation produced unexpected stdout".to_owned(),
        ));
    }
    if !output.is_file() {
        return Err(process_error(format!(
            "compiler reported link success without executable '{}'",
            display_path(&output)
        )));
    }
    Ok(())
}

fn run_executable(output: &Path) -> Result<(), ProcessFailure> {
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
    if status.success() {
        Ok(())
    } else {
        Err(program_status_failure(status.code()))
    }
}

fn query_capabilities(compiler: &Path) -> Result<Capabilities, ProcessFailure> {
    let arguments = [OsString::from("--shuttle-protocol-capabilities")];
    let captured = invoke_compiler(compiler, &arguments, CAPABILITY_LIMIT)?;
    replay_compiler_stderr(&captured)?;
    if !captured.status.success() || captured.stderr_length != 0 || captured.stdout_oversized {
        return Err(process_error(format!(
            "compiler '{}' does not provide valid Shuttle protocol capabilities",
            display_path(compiler)
        )));
    }
    parse_json_line(&captured.stdout, CAPABILITY_LIMIT, "capability response")
}

fn validate_capabilities(
    capabilities: &Capabilities,
    target: Target,
    command: ProjectCommand,
) -> Result<(), ProcessFailure> {
    let target_name = target.to_string();
    let targets = if command == ProjectCommand::Check {
        &capabilities.interface_targets
    } else {
        &capabilities.object_targets
    };
    if capabilities.schema != 1
        || !capabilities.protocols.contains(&2)
        || !capabilities.artifact_formats.contains(&ARTIFACT_FORMAT)
        || !valid_digest(&capabilities.compiler_id)
        || !["compile", "inspect", "link", "reuse"]
            .iter()
            .all(|required| {
                capabilities
                    .operations
                    .iter()
                    .any(|value| value == required)
            })
        || !targets.contains(&target_name)
    {
        return Err(process_error(
            "selected compiler lacks required Shuttle protocol-v2 capabilities".to_owned(),
        ));
    }
    Ok(())
}

fn invoke_compiler(
    compiler: &Path,
    arguments: &[OsString],
    stdout_limit: usize,
) -> Result<CapturedProcess, ProcessFailure> {
    let stderr_spool = NamedTempFile::new().map_err(|error| {
        process_error(format!(
            "could not create private compiler diagnostic spool: {error}"
        ))
    })?;
    let mut child = Command::new(compiler)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| {
            process_error(format!(
                "could not start compiler '{}': {error}",
                display_path(compiler)
            ))
        })?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| process_error("could not capture compiler stdout".to_owned()))?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or_else(|| process_error("could not capture compiler stderr".to_owned()))?;
    let stdout_thread = thread::spawn(move || -> std::io::Result<(Vec<u8>, bool)> {
        let mut captured = Vec::new();
        let mut oversized = false;
        let mut buffer = [0_u8; 8192];
        loop {
            let count = stdout.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            let remaining = stdout_limit.saturating_sub(captured.len());
            captured.extend_from_slice(&buffer[..count.min(remaining)]);
            if count > remaining {
                oversized = true;
            }
        }
        Ok((captured, oversized))
    });
    let stderr_thread = thread::spawn(move || -> io::Result<(NamedTempFile, u64)> {
        let mut spool = stderr_spool;
        let length = io::copy(&mut stderr, spool.as_file_mut())?;
        Ok((spool, length))
    });
    let status = child.wait().map_err(|error| {
        process_error(format!(
            "could not wait for compiler '{}': {error}",
            display_path(compiler)
        ))
    })?;
    let (stdout, stdout_oversized) = stdout_thread
        .join()
        .map_err(|_| process_error("compiler stdout reader panicked".to_owned()))?
        .map_err(|error| process_error(format!("could not read compiler stdout: {error}")))?;
    let (stderr, stderr_length) = stderr_thread
        .join()
        .map_err(|_| process_error("compiler stderr reader panicked".to_owned()))?
        .map_err(|error| process_error(format!("could not read compiler stderr: {error}")))?;
    Ok(CapturedProcess {
        status,
        stdout,
        stdout_oversized,
        stderr,
        stderr_length,
    })
}

fn replay_compiler_stderr(captured: &CapturedProcess) -> Result<(), ProcessFailure> {
    if captured.stderr_length == 0 {
        return Ok(());
    }
    let mut source = captured.stderr.reopen().map_err(|error| {
        process_error(format!(
            "could not reopen private compiler diagnostic spool: {error}"
        ))
    })?;
    let destination = io::stderr();
    let mut destination = destination.lock();
    io::copy(&mut source, &mut destination)
        .and_then(|_| destination.flush())
        .map_err(|error| process_error(format!("could not replay compiler diagnostics: {error}")))
}

fn require_compiler_success(
    compiler: &Path,
    package: &str,
    operation: &str,
    captured: &CapturedProcess,
) -> Result<(), ProcessFailure> {
    if !captured.status.success() {
        let mut failure = compiler_status_failure(captured.status.code());
        if let Some(message) = &mut failure.message {
            *message = format!(
                "package '{package}', {operation}, compiler '{}': {message}",
                display_path(compiler)
            );
        }
        return Err(failure);
    }
    if captured.stdout_oversized {
        return Err(process_error(format!(
            "package '{package}', {operation}: compiler stdout exceeded its protocol limit"
        )));
    }
    Ok(())
}

fn parse_receipt(bytes: &[u8]) -> Result<ArtifactReceipt, ProcessFailure> {
    let receipt: ArtifactReceipt = parse_json_line(bytes, RECEIPT_LIMIT, "artifact receipt")?;
    if receipt.schema != 1
        || receipt.artifact_format != ARTIFACT_FORMAT
        || !valid_digest(&receipt.artifact_id)
        || !valid_digest(&receipt.compiler_id)
        || !matches!(receipt.kind.as_str(), "interface" | "object")
    {
        return Err(process_error("artifact receipt is invalid".to_owned()));
    }
    let mut previous = None::<&str>;
    for dependency in &receipt.dependencies {
        if !valid_digest(&dependency.artifact_id)
            || previous.is_some_and(|value| value >= dependency.alias.as_str())
        {
            return Err(process_error(
                "artifact receipt dependencies are noncanonical".to_owned(),
            ));
        }
        previous = Some(&dependency.alias);
    }
    Ok(receipt)
}

fn parse_json_line<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    limit: usize,
    description: &str,
) -> Result<T, ProcessFailure> {
    if bytes.len() > limit || !bytes.ends_with(b"\n") {
        return Err(process_error(format!(
            "{description} is missing its required bounded line"
        )));
    }
    let mut body = &bytes[..bytes.len() - 1];
    if body.ends_with(b"\r") {
        body = &body[..body.len() - 1];
    }
    if body.is_empty() || body.contains(&b'\n') || body.contains(&b'\r') {
        return Err(process_error(format!(
            "{description} contains trailing or multiline data"
        )));
    }
    serde_json::from_slice(body)
        .map_err(|error| process_error(format!("malformed {description}: {error}")))
}

fn validate_compile_receipt(
    receipt: &ArtifactReceipt,
    package: &crate::graph::PackageRecord,
    kind: &str,
    target: Target,
    compiler_id: &str,
    graph: &PackageGraph,
    produced: &BTreeMap<String, ProducedArtifact>,
) -> Result<(), ProcessFailure> {
    if receipt.kind != kind
        || receipt.package.name != package.name
        || receipt.package.version != package.version.to_string()
        || receipt.target != target.to_string()
        || receipt.compiler_id != compiler_id
    {
        return Err(process_error(format!(
            "compiler receipt does not match package '{}'",
            package.name
        )));
    }
    let expected = graph
        .dependencies
        .iter()
        .filter(|dependency| dependency.owner == package.name)
        .map(|dependency| {
            let artifact = produced.get(&dependency.target).ok_or_else(|| {
                process_error(format!(
                    "receipt dependency '{}' was not produced",
                    dependency.target
                ))
            })?;
            Ok(ReceiptDependency {
                alias: dependency.alias.clone(),
                package: artifact.receipt.package.clone(),
                artifact_id: artifact.receipt.artifact_id.clone(),
            })
        })
        .collect::<Result<Vec<_>, ProcessFailure>>()?;
    if receipt.dependencies != expected {
        return Err(process_error(format!(
            "compiler receipt dependency set does not match package '{}'",
            package.name
        )));
    }
    Ok(())
}

fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn topological_order(graph: &PackageGraph) -> Result<Vec<String>, ProcessFailure> {
    let mut indegrees = graph
        .packages
        .keys()
        .map(|name| (name.clone(), 0_usize))
        .collect::<BTreeMap<_, _>>();
    let mut dependents = BTreeMap::<String, Vec<String>>::new();
    for dependency in &graph.dependencies {
        *indegrees
            .get_mut(&dependency.owner)
            .ok_or_else(|| process_error("dependency owner is missing".to_owned()))? += 1;
        dependents
            .entry(dependency.target.clone())
            .or_default()
            .push(dependency.owner.clone());
    }
    let mut ready = indegrees
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(name, _)| name.clone())
        .collect::<BTreeSet<_>>();
    let mut order = Vec::with_capacity(graph.packages.len());
    while let Some(name) = ready.pop_first() {
        order.push(name.clone());
        if let Some(packages) = dependents.get(&name) {
            for package in packages {
                let degree = indegrees
                    .get_mut(package)
                    .ok_or_else(|| process_error("dependent package is missing".to_owned()))?;
                *degree -= 1;
                if *degree == 0 {
                    ready.insert(package.clone());
                }
            }
        }
    }
    if order.len() != graph.packages.len() {
        return Err(process_error(
            "package graph contains a dependency cycle".to_owned(),
        ));
    }
    Ok(order)
}

fn dependency_levels(
    graph: &PackageGraph,
    order: &[String],
) -> Result<Vec<Vec<(usize, String)>>, ProcessFailure> {
    let mut direct_dependencies = BTreeMap::<String, Vec<String>>::new();
    for dependency in &graph.dependencies {
        direct_dependencies
            .entry(dependency.owner.clone())
            .or_default()
            .push(dependency.target.clone());
    }
    let mut package_levels = BTreeMap::<String, usize>::new();
    let mut levels = Vec::<Vec<(usize, String)>>::new();
    for package_name in order {
        let level = direct_dependencies
            .get(package_name)
            .into_iter()
            .flatten()
            .map(|dependency| {
                package_levels.get(dependency).copied().ok_or_else(|| {
                    process_error(format!(
                        "package '{package_name}' was ordered before dependency '{dependency}'"
                    ))
                })
            })
            .collect::<Result<Vec<_>, ProcessFailure>>()?
            .into_iter()
            .max()
            .map_or(0, |dependency_level| dependency_level + 1);
        while levels.len() <= level {
            levels.push(Vec::new());
        }
        levels[level].push((0, package_name.clone()));
        package_levels.insert(package_name.clone(), level);
    }
    for (index, package) in levels.iter_mut().flatten().enumerate() {
        package.0 = index;
    }
    Ok(levels)
}

fn transitive_dependencies(graph: &PackageGraph, package: &str) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let mut pending = graph
        .dependencies
        .iter()
        .filter(|dependency| dependency.owner == package)
        .map(|dependency| dependency.target.clone())
        .collect::<Vec<_>>();
    while let Some(current) = pending.pop() {
        if !result.insert(current.clone()) {
            continue;
        }
        pending.extend(
            graph
                .dependencies
                .iter()
                .filter(|dependency| dependency.owner == current)
                .map(|dependency| dependency.target.clone()),
        );
    }
    result
}

fn executable_output(graph: &PackageGraph, target: Target) -> Result<PathBuf, ProcessFailure> {
    let root = graph
        .packages
        .get(&graph.root_package)
        .ok_or_else(|| process_error("package graph is missing its root package".to_owned()))?;
    let executable = graph.root_executable.as_ref().ok_or_else(|| {
        process_error(format!(
            "package '{}' has no [executable] target",
            graph.root_package
        ))
    })?;
    let mut file_name = executable.name.clone();
    if !std::env::consts::EXE_EXTENSION.is_empty() {
        file_name.push('.');
        file_name.push_str(std::env::consts::EXE_EXTENSION);
    }
    Ok(windows_compatible_path(&root.package_root)
        .join("target")
        .join(target.to_string())
        .join(file_name))
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
        Ok(path) if path.is_file() => Ok(windows_compatible_path(&path)),
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
