# Shuttle work ledger

`ROADMAP.md` defines Shuttle's stage order and scope. This file owns the
concrete work required to close those stages. Public behavior must be documented
in owning contracts rather than only recorded as completed checkboxes.

## Stage status

Stages 22 through 26 are complete. Shared verification is
documented in `docs/testing.md`.

Stage 26 coordination is complete. The approved 26.3 transition is implemented:
capabilities and receipts require artifact format 3, while the compiler owns
ABI 4 and runtime ABI 2 compatibility. Older packages must be rebuilt.
The [26.4 exit audit](docs/testing.md#stage-264-struct-exit-audit) passed with
development and sanitizer compilers on 2026-09-02. Stage 26.5.1 coordinated
explicit-override verification is also complete; see `docs/testing.md`.

Stage 27 coordination is complete following approval and verification through
27.4 on 2026-09-02. The [exit audit](docs/testing.md#stage-274-switch-exit-audit)
closes keyword policy, source-free execution, dependency evolution, and
determinism. No compiler protocol changes or later stages are introduced.

## Scheduled work

### Stage 27: Switch keyword and dependency-evolution coordination

- [x] Mirror the approved reserved
  `switch`, `case`, and `default` words in dependency-alias validation; test
  rejection by both tools without changing package-name grammar or schemas.
  Completed with both compiler configurations and the Rust gates on 2026-09-02;
  see `docs/testing.md`.
- [x] Verify switch compilation and execution against source-free enum and
  scalar-constant artifacts, including switches in dependency object payloads,
  import aliases, grouped labels, defaults, and widened/full-width constants.
  Whole/separate/source-free output agrees with both compiler configurations.
- [x] Audit switch-specific dependency evolution in 27.4. Added cases must
  invalidate and reject uncovered consumers;
  explicit fallbacks must work, and failures must preserve completed output.
  Cover reordered/removed cases and constant-value edits too.
- [x] Prove whole/separate native behavior and deterministic serial/parallel
  artifacts; pass Rust and shared development/sanitizer protocol/native gates.
  Update maintainer testing records without duplicating the language reference.

Completed 2026-09-02: 24 shared protocol and 16 native tests pass with both
compiler configurations; each passes all 141 CTests. All 43 ordinary Rust tests,
formatting, Clippy with warnings denied, and Rust 1.85 checking also pass.

### Stage 26: Aggregate artifact compatibility

- [x] Coordinate the compiler's approved struct/ABI contract and require the
  reviewed artifact version in capabilities and receipts. Preserve process
  protocol 2, manifest schema 1, opaque artifact handling, and exact reuse checks.
- [x] Test source-free struct dependencies, private field layouts, aggregate
  parameters/results, GC-bearing nested values and arrays, and constructor/output
  behavior against both compiler configurations.
- [x] Verify layout/member edits invalidate consumers while independent packages
  remain reusable, serial/parallel artifacts are byte-identical, and separate
  execution matches whole-project behavior. Complete Rust and shared exit gates.

### Stage 26.5.1: Explicit interface overrides

- [x] Migrate shared implementing declarations to `override`; test missing and
  unmatched markers against source-free dependencies and inherited methods.
- [x] Pass both compiler configurations and Rust quality gates, preserving
  opaque artifacts and existing compatibility/scheduling protocols.

### Stage 25: Enum artifact compatibility

- [x] Require compiler-owned artifact format 2 in capabilities and receipts;
  retain process protocol v2, opaque artifact handling, and conservative reuse.
- [x] Verify enum dependency builds, source-free consumption, case-edit
  invalidation, and serial/parallel equivalence in the shared toolchain suites.

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
- [x] **24.4 — Parallel scheduling and exit audit.** Bound concurrent ready
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
