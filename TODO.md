# Shuttle work ledger

`ROADMAP.md` defines Shuttle's stage order and scope. This file owns the
concrete work required to close those stages. Public behavior must be documented
in owning contracts rather than only recorded as completed checkboxes.

## Stage status

Stages 22 and 23 are complete. Stage 24 is active. Shared verification is
documented in `docs/testing.md`.

## Scheduled work

### Stage 22: Local projects and compiler build protocol

- [x] **22.1 — Shared contract.** Approve Shuttle's Rust 2024 implementation
  policy, `Shuttle.toml`, package/workspace terminology, dependency namespace
  mapping, schema and protocol versioning, compiler request inputs, migration
  behavior, and diagnostics ownership with the Cloth compiler. The approved
  contracts are in `docs/`.
- [x] **22.2 — Bootstrap.** Establish the production command-line application,
  manifest model and parser, validation boundaries, deterministic diagnostics,
  build configuration, formatting, and unit-test harness.
- [x] **22.3 — Local build graph.** Resolve local path dependencies, validate
  source roots and identities, reject cycles and duplicates, and produce the
  approved ordered compiler request without parsing Cloth source.
- [x] **22.4 — Cross-tool verification.** Exercise valid and invalid
  multi-project fixtures against `clothc`, document the supported workflow, and
  complete Shuttle's development and sanitizer exit audit.

### Stage 23: Separate compilation and deterministic linking

- [x] **23.1 — Shared artifact contract.** Approve the compiler artifact and
  [process-v2 proposals](docs/proposals/compiler_protocol_v2.md), including
  public receipts, exact compatibility, check-only artifacts, output ownership,
  and the boundary between artifact reuse and deferred automatic caching.
- [x] **23.2 — Ordered compilation.** Compile local dependency packages in a
  deterministic topological order, consume and validate public compiler receipts,
  and reuse each compatible artifact across consumers within one invocation
  without depending on private compiler representations.
- [x] **23.3 — Link orchestration.** Invoke the approved link pipeline and add
  package context while preserving compiler and linker diagnostics.
- [x] **23.4 — Equivalence verification.** Prove clean and reused builds match
  whole-project behavior, then close the shared integration and Shuttle exit
  audits.

### Stage 24: Responsive and observable local builds

- [x] **24.1 — Progress and measurement.** Define and implement stable stderr
  progress plus repeatable phase baselines without altering compiler diagnostic
  or executed-program streams.
- [x] **24.2 — Cold-path efficiency.** Remove profiled identity and process
  overhead while retaining exact compiler and native-tool compatibility.
- [x] **24.3 — Validated local reuse.** Persist conservative package input
  fingerprints and reuse artifacts only after integrity and compatibility
  validation, with precise dependent invalidation and atomic state updates.
- [ ] **24.4 — Parallel scheduling and exit audit.** Bound concurrent ready
  packages, preserve deterministic output/diagnostics, and close all shared
  performance and correctness gates.

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
