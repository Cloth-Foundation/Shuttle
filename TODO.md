# Shuttle work ledger

`ROADMAP.md` defines Shuttle's stage order and scope. This file owns the
concrete work required to close those stages. Public behavior must be documented
in owning contracts rather than only recorded as completed checkboxes.

## Active stage

Stage 22 is active. Stage 22.1 is complete; Stage 22.2 implementation must not
begin until it receives an explicit go-ahead.

## Scheduled work

### Stage 22: Local projects and compiler build protocol

- [x] **22.1 — Shared contract.** Approve Shuttle's Rust 2024 implementation
  policy, `Shuttle.toml`, package/workspace terminology, dependency namespace
  mapping, schema and protocol versioning, compiler request inputs, migration
  behavior, and diagnostics ownership with the Cloth compiler. The approved
  contracts are in `docs/`.
- [ ] **22.2 — Bootstrap.** Establish the production command-line application,
  manifest model and parser, validation boundaries, deterministic diagnostics,
  build configuration, formatting, and unit-test harness.
- [ ] **22.3 — Local build graph.** Resolve local path dependencies, validate
  source roots and identities, reject cycles and duplicates, and produce the
  approved ordered compiler request without parsing Cloth source.
- [ ] **22.4 — Cross-tool verification.** Exercise valid and invalid
  multi-project fixtures against `clothc`, document the supported workflow, and
  complete Shuttle's development and sanitizer exit audit.

### Stage 23: Separate compilation and deterministic linking

- [ ] **23.1 — Artifact consumption.** Read and validate the compiler-owned
  artifact metadata without depending on private compiler representations.
- [ ] **23.2 — Ordered compilation.** Compile local dependency packages in a
  deterministic topological order and reuse only compatible artifacts.
- [ ] **23.3 — Link orchestration.** Invoke the approved link pipeline and add
  package context while preserving compiler and linker diagnostics.
- [ ] **23.4 — Equivalence verification.** Prove clean and reused builds match
  whole-project behavior, then close the shared integration and Shuttle exit
  audits.

## Unscheduled backlog

These items require a future roadmap stage before implementation:

- registries, remote retrieval, semantic-version solving, and `Shuttle.lock`
  generation;
- workspaces containing remote packages;
- package publishing and signing;
- build scripts, plugins, and arbitrary command execution;
- incremental compilation, local shared caches, remote caches, and distributed
  builds;
- standard-library installation and toolchain distribution; and
- editor and language-server build integration.
