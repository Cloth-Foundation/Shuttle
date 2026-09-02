// Part of the Cloth Compiler project, under the Apache License v2.0 with LLVM
// Exceptions. See LICENSE.txt in the project root for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use semver::Version;

use crate::diagnostic::{Diagnostic, SourcePosition, sort_diagnostics};
use crate::manifest::{Dependency, Executable, Manifest, load_manifest, manifest_in_package_root};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageRecord {
    pub name: String,
    pub version: Version,
    pub manifest_path: PathBuf,
    pub manifest_contents: String,
    pub package_root: PathBuf,
    pub source_root: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyEdge {
    pub owner: String,
    pub alias: String,
    pub target: String,
    pub declaration_path: PathBuf,
    pub declaration_position: SourcePosition,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageGraph {
    pub root_package: String,
    pub root_executable: Option<Executable>,
    pub packages: BTreeMap<String, PackageRecord>,
    pub dependencies: Vec<DependencyEdge>,
}

struct LoadedPackage {
    manifest: Manifest,
    canonical_manifest_path: PathBuf,
}

struct GraphResolver {
    root_package: String,
    root_executable: Option<Executable>,
    diagnostics: Vec<Diagnostic>,
    loaded: Vec<LoadedPackage>,
    package_by_path: BTreeMap<PathBuf, String>,
    path_by_package: BTreeMap<String, PathBuf>,
    edges: Vec<DependencyEdge>,
    next_package: usize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum VisitState {
    Visiting,
    Complete,
}

/// Resolves and validates the complete local dependency closure.
///
/// # Errors
///
/// Returns deterministic manifest and graph diagnostics for invalid roots,
/// dependency paths, duplicate identities, repeated targets, self-edges, and
/// cycles.
pub fn resolve_package_graph(root_manifest_path: &Path) -> Result<PackageGraph, Vec<Diagnostic>> {
    let root_path = match canonical_file(root_manifest_path) {
        Ok(path) => path,
        Err(diagnostic) => return Err(vec![diagnostic]),
    };
    let root_manifest = load_manifest(&root_path)?;
    GraphResolver::new(root_manifest, root_path).resolve()
}

impl GraphResolver {
    fn new(root_manifest: Manifest, root_path: PathBuf) -> Self {
        let root_package = root_manifest.package.name.clone();
        Self {
            root_executable: root_manifest.executable.clone(),
            diagnostics: Vec::new(),
            loaded: vec![LoadedPackage {
                manifest: root_manifest,
                canonical_manifest_path: root_path.clone(),
            }],
            package_by_path: BTreeMap::from([(root_path.clone(), root_package.clone())]),
            path_by_package: BTreeMap::from([(root_package.clone(), root_path)]),
            edges: Vec::new(),
            next_package: 0,
            root_package,
        }
    }

    fn resolve(mut self) -> Result<PackageGraph, Vec<Diagnostic>> {
        self.resolve_dependencies();
        self.edges.sort_by(|left, right| {
            left.owner
                .cmp(&right.owner)
                .then_with(|| left.alias.cmp(&right.alias))
        });
        if let Some(cycle) = find_cycle(&self.path_by_package, &self.edges) {
            self.diagnostics.push(cycle);
        }
        if !self.diagnostics.is_empty() {
            sort_diagnostics(&mut self.diagnostics);
            return Err(self.diagnostics);
        }

        let packages = self
            .loaded
            .into_iter()
            .map(package_record)
            .collect::<BTreeMap<_, _>>();
        Ok(PackageGraph {
            root_package: self.root_package,
            root_executable: self.root_executable,
            packages,
            dependencies: self.edges,
        })
    }

    fn resolve_dependencies(&mut self) {
        while self.next_package < self.loaded.len() {
            let owner_path = self.loaded[self.next_package]
                .canonical_manifest_path
                .clone();
            let owner_name = self.loaded[self.next_package].manifest.package.name.clone();
            let owner_manifest_path = self.loaded[self.next_package].manifest.path.clone();
            let dependencies = self.loaded[self.next_package].manifest.dependencies.clone();
            self.next_package += 1;

            let mut aliases_by_target = BTreeMap::<PathBuf, String>::new();
            for dependency in dependencies.values() {
                self.resolve_edge(
                    &owner_path,
                    &owner_name,
                    &owner_manifest_path,
                    dependency,
                    &mut aliases_by_target,
                );
            }
        }
    }

    fn resolve_edge(
        &mut self,
        owner_path: &Path,
        owner_name: &str,
        owner_manifest_path: &Path,
        dependency: &Dependency,
        aliases_by_target: &mut BTreeMap<PathBuf, String>,
    ) {
        let Some(target_path) =
            resolve_dependency_manifest(owner_manifest_path, dependency, &mut self.diagnostics)
        else {
            return;
        };
        if let Some(previous_alias) = aliases_by_target.get(&target_path) {
            self.diagnostics.push(edge_diagnostic(
                owner_manifest_path,
                dependency,
                format!(
                    "dependency aliases '{}' and '{}' resolve to the same package root",
                    previous_alias, dependency.alias
                ),
            ));
            return;
        }
        aliases_by_target.insert(target_path.clone(), dependency.alias.clone());
        if target_path == owner_path {
            self.diagnostics.push(edge_diagnostic(
                owner_manifest_path,
                dependency,
                format!("package '{owner_name}' cannot depend on itself"),
            ));
            return;
        }

        if let Some(target_name) = self.load_target(&target_path, owner_manifest_path, dependency) {
            self.edges.push(DependencyEdge {
                owner: owner_name.to_owned(),
                alias: dependency.alias.clone(),
                target: target_name,
                declaration_path: owner_manifest_path.to_path_buf(),
                declaration_position: dependency.declaration_position,
            });
        }
    }

    fn load_target(
        &mut self,
        target_path: &Path,
        owner_manifest_path: &Path,
        dependency: &Dependency,
    ) -> Option<String> {
        if let Some(name) = self.package_by_path.get(target_path) {
            return Some(name.clone());
        }
        let target_manifest = match load_manifest(target_path) {
            Ok(manifest) => manifest,
            Err(mut errors) => {
                self.diagnostics.append(&mut errors);
                return None;
            }
        };
        let name = target_manifest.package.name.clone();
        if let Some(previous_path) = self.path_by_package.get(&name) {
            self.diagnostics.push(edge_diagnostic(
                owner_manifest_path,
                dependency,
                format!(
                    "package name '{}' is already provided by '{}'",
                    name,
                    display_path(previous_path)
                ),
            ));
            return None;
        }
        let target_path = target_path.to_path_buf();
        self.path_by_package
            .insert(name.clone(), target_path.clone());
        self.package_by_path
            .insert(target_path.clone(), name.clone());
        self.loaded.push(LoadedPackage {
            manifest: target_manifest,
            canonical_manifest_path: target_path,
        });
        Some(name)
    }
}

fn package_record(loaded_package: LoadedPackage) -> (String, PackageRecord) {
    let manifest = loaded_package.manifest;
    let name = manifest.package.name.clone();
    (
        name.clone(),
        PackageRecord {
            name,
            version: manifest.package.version,
            manifest_path: loaded_package.canonical_manifest_path,
            manifest_contents: manifest.contents,
            package_root: manifest.package_root,
            source_root: manifest.package.source_root,
        },
    )
}

fn resolve_dependency_manifest(
    owner_manifest_path: &Path,
    dependency: &Dependency,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<PathBuf> {
    let package_root = match fs::canonicalize(&dependency.package_root) {
        Ok(path) if path.is_dir() => path,
        Ok(_) => {
            diagnostics.push(edge_diagnostic(
                owner_manifest_path,
                dependency,
                "dependency path is not a directory",
            ));
            return None;
        }
        Err(error) => {
            diagnostics.push(edge_diagnostic(
                owner_manifest_path,
                dependency,
                format!("could not resolve dependency path: {error}"),
            ));
            return None;
        }
    };
    if !is_portable_path(&package_root) {
        diagnostics.push(edge_diagnostic(
            owner_manifest_path,
            dependency,
            "dependency package root is not a portable Unicode path",
        ));
        return None;
    }
    let manifest_path = match manifest_in_package_root(&package_root) {
        Ok(path) => path,
        Err(error) => {
            diagnostics.push(edge_diagnostic(
                owner_manifest_path,
                dependency,
                format!("dependency '{}': {}", dependency.alias, error.message()),
            ));
            return None;
        }
    };
    match canonical_file(&manifest_path) {
        Ok(path) => Some(path),
        Err(error) => {
            diagnostics.push(edge_diagnostic(
                owner_manifest_path,
                dependency,
                format!("dependency '{}': {}", dependency.alias, error.message()),
            ));
            None
        }
    }
}

fn canonical_file(path: &Path) -> Result<PathBuf, Diagnostic> {
    match fs::canonicalize(path) {
        Ok(path) if path.is_file() => Ok(path),
        Ok(_) => Err(Diagnostic::file(path, "manifest is not a regular file")),
        Err(error) => Err(Diagnostic::file(
            path,
            format!("could not resolve manifest: {error}"),
        )),
    }
}

fn edge_diagnostic(
    owner_manifest_path: &Path,
    dependency: &Dependency,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic::at_position(
        owner_manifest_path,
        dependency.declaration_position,
        message,
    )
}

fn find_cycle(
    path_by_package: &BTreeMap<String, PathBuf>,
    edges: &[DependencyEdge],
) -> Option<Diagnostic> {
    let mut adjacency = BTreeMap::<String, Vec<usize>>::new();
    for (index, edge) in edges.iter().enumerate() {
        adjacency.entry(edge.owner.clone()).or_default().push(index);
    }

    let mut states = BTreeMap::<String, VisitState>::new();
    for start in path_by_package.keys() {
        if states.contains_key(start) {
            continue;
        }
        states.insert(start.clone(), VisitState::Visiting);
        let mut frames = vec![(start.clone(), 0_usize)];
        let mut stack_nodes = vec![start.clone()];
        let mut incoming_aliases = vec![None::<String>];

        while !frames.is_empty() {
            let edge_index = {
                let (node, next) = frames.last_mut().expect("frame exists");
                let outgoing = adjacency.get(node).map_or(&[][..], Vec::as_slice);
                if *next < outgoing.len() {
                    let edge_index = outgoing[*next];
                    *next += 1;
                    Some(edge_index)
                } else {
                    None
                }
            };

            let Some(edge_index) = edge_index else {
                let (node, _) = frames.pop().expect("frame exists");
                states.insert(node, VisitState::Complete);
                stack_nodes.pop();
                incoming_aliases.pop();
                continue;
            };
            let edge = &edges[edge_index];
            match states.get(&edge.target) {
                Some(VisitState::Complete) => {}
                Some(VisitState::Visiting) => {
                    let cycle_start = stack_nodes
                        .iter()
                        .position(|node| node == &edge.target)
                        .expect("visiting package is on the active stack");
                    let mut path = stack_nodes[cycle_start].clone();
                    for index in (cycle_start + 1)..stack_nodes.len() {
                        let alias = incoming_aliases[index]
                            .as_deref()
                            .expect("non-root stack node has an incoming alias");
                        write!(path, " --{alias}--> {}", stack_nodes[index])
                            .expect("writing to a string cannot fail");
                    }
                    write!(path, " --{}--> {}", edge.alias, edge.target)
                        .expect("writing to a string cannot fail");
                    return Some(Diagnostic::at_position(
                        &edge.declaration_path,
                        edge.declaration_position,
                        format!("dependency cycle: {path}"),
                    ));
                }
                None => {
                    states.insert(edge.target.clone(), VisitState::Visiting);
                    frames.push((edge.target.clone(), 0));
                    stack_nodes.push(edge.target.clone());
                    incoming_aliases.push(Some(edge.alias.clone()));
                }
            }
        }
    }
    None
}

fn is_portable_path(path: &Path) -> bool {
    path.to_str()
        .is_some_and(|value| !value.chars().any(char::is_control))
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
