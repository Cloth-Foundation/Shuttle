# Shuttle work ledger

`ROADMAP.md` defines Shuttle's stage order and scope. This file owns the
concrete work required to close those stages. Public behavior must be documented
in owning contracts rather than only recorded as completed checkboxes.

## Stage status

Stages 22 through 26 are complete. Shared verification is
documented in `docs/testing.md`.

Stage 26 coordination is complete. The approved 26.3 transition is implemented:
that checkpoint introduced artifact format 3, while the compiler owns
ABI 4 and runtime ABI 2 compatibility. Older packages must be rebuilt.
The [26.4 exit audit](docs/testing.md#stage-264-struct-exit-audit) passed with
development and sanitizer compilers on 2026-09-02. Stage 26.5.1 coordinated
explicit-override verification is also complete; see `docs/testing.md`.

Stage 27 coordination is complete following approval and verification through
27.4 on 2026-09-02. The [exit audit](docs/testing.md#stage-274-switch-exit-audit)
closes keyword policy, source-free execution, dependency evolution, and
determinism. No compiler protocol changes or later stages are introduced.

Stage 28 coordination is complete following the separately authorized
[28.4 exit audit](docs/testing.md#stage-284-scalar-constant-exit-audit) on
2026-09-02. Format-4 integration, source-free values, dependency evolution,
output preservation, and determinism are verified. Stage 29 coordination is
complete following the separately authorized
[29.4 exit audit](docs/testing.md#coordinated-294-checked-runtime-arithmetic-exit-audit)
on 2026-09-03. Stage 30 coordination is complete following the compiler's
separately authorized 30.4 exit audit on 2026-09-03. No Shuttle production
change was required. Stage 31 coordination is complete following the compiler's
separately authorized 31.4 exit audit on 2026-09-04. No Shuttle production
change was required.

## Scheduled work

### Stage 31: MIR optimization coordination

- [x] Record the compiler-owned always-on MIR optimization contract and its
  unchanged artifact, ABI, runtime, protocol, receipt, manifest, and scheduler
  boundaries. Approved 2026-09-03; Shuttle production code is unchanged.
- [x] Record compiler 31.2 canonical constants and scalar folding. Imported
  scalar constants fold through source-free declarations in compiler tests;
  Shuttle production code and every compatibility version remain unchanged.
- [x] With compiler 31.3, verify optimized whole-project, separate-package, and
  source-free behavior through the public process boundary without parsing MIR
  or optimizer metadata. Both compiler configurations pass the shared
  protocol and native package suites; no Shuttle implementation changed.
- [x] Prove affected-only invalidation, unrelated reuse, failure preservation,
  and relocated serial/parallel x86-64/wasm32 artifact determinism with both
  compiler configurations. The existing deterministic package matrix remains
  clean with the always-on optimizer and unchanged compatibility versions.
- [x] Pass ordinary Rust, Rust 1.85, formatting, Clippy, shared
  protocol/native, editor, and repository gates during the coordinated 31.4
  exit audit.

  Completed 2026-09-04. Both 215-test compiler configurations pass all 29
  public protocol/toolchain and 24 native Shuttle cases. All 43 ordinary Rust
  tests, Rust 1.85, warning-denied Clippy, formatting, six editor tests per
  compiler, and repository gates pass. Existing package cases preserve
  whole/separate/source-free equivalence, affected-only invalidation,
  failed-output safety, and relocated serial/parallel x86-64/wasm32
  determinism. Compatibility versions and Shuttle production code are
  unchanged.

### Stage 30: Integer conversion-mode coordination

- [x] Record the compiler-owned `Target::wrap(value)` and
  `Target::sat(value)` contract and its unchanged compatibility boundary.
  Approved 2026-09-03; Shuttle production code is unchanged.
- [x] Record compiler 30.2 frontend and constant verification. Both 188-test
  compiler configurations, 43 ordinary Rust tests, Rust 1.85, formatting,
  warning-denied Clippy, editor, and repository gates pass. Runtime package
  behavior was assigned to 30.3; Shuttle production code is unchanged.
- [x] With compiler 30.3, verify runtime and scalar-constant results through the
  public process boundary without parsing source expressions or conversion
  metadata in Shuttle.
- [x] Prove whole/separate/source-free equivalence, affected-only invalidation,
  unrelated reuse, output preservation, and relocated serial/parallel artifact
  determinism with both compiler configurations.

  Completed 2026-09-03. The shared matrix now contains 29 public
  protocol/toolchain and 24 native cases per compiler. Integer conversion
  fixtures prove runtime and constant results, whole/separate/source-free
  equivalence, affected-only invalidation, unrelated reuse, failure
  preservation, and deterministic x86-64/wasm32 artifacts. Shuttle remains an
  opaque artifact coordinator and no production or compatibility change was
  required.
- [x] Pass ordinary Rust, Rust 1.85, formatting, Clippy, shared protocol/native,
  editor, and repository gates during the coordinated 30.4 exit audit.

  Completed 2026-09-03. Both compiler configurations pass all 200 CTests,
  including all 29 shared protocol/toolchain and 24 native Shuttle cases. All
  43 ordinary Rust tests, Rust 1.85 checking, Rust formatting, warning-denied
  Clippy, editor tests, C++ formatting, and repository whitespace gates pass.
  Relocated serial/parallel artifacts and whole, separate, and source-free
  behavior remain deterministic. Stage 30 coordination is complete with every
  compatibility version unchanged.

### Stage 29: Checked-arithmetic runtime ABI coordination

- [x] Record approval of the compiler-owned Stage 29 source/failure/lowering
  contract and runtime ABI 3. Approved 2026-09-03.
- [x] With compiler 29.2, verify the compiler-owned runtime-ABI-3 artifact
  transition and runtime-ABI-2 rejection through the existing opaque boundary.
  Runtime ABI is not exposed in Shuttle capabilities, receipts, or stubs, so
  their schemas remain unchanged alongside artifact format 4, compiler ABI 4,
  and all process/manifest schema versions.
- [x] Verify direct/update/compound arithmetic through the public compiler
  protocol without parsing language expressions or object payloads in Shuttle.
- [x] Prove runtime-ABI-2 rejection, affected dependency invalidation, unrelated
  reuse, completed-output preservation, and no stale execution after failures.
- [x] Prove relocated serial/parallel artifact determinism and whole/separate/
  source-free native equivalence with both compiler configurations; pass Rust
  formatting, Clippy, ordinary tests, and Rust 1.85 checking.

  Completed 2026-09-03. The checked-update fixture passes whole, separate, and
  source-free native execution, preserves unaffected artifacts and completed
  output across valid/invalid dependency edits, and produces identical serial/
  parallel artifacts for both targets. The compiler remains the sole owner of
  runtime-ABI validation and arithmetic semantics.

- [x] Complete the coordinated 29.4 exit audit against both compiler
  configurations, including all shared protocol/native, ordinary Rust, Rust
  1.85, formatting, Clippy, editor, and repository gates.

  Completed 2026-09-03. Each compiler passes 28 public protocol and 22 native
  Shuttle cases inside its 186-test compiler run. Shuttle required no production
  or schema change and continues to treat runtime ABI as opaque compiler-owned
  artifact metadata.

### Stage 28: Scalar-constant artifact coordination

- [x] Record the compiler-owned draft and compatibility boundary in the roadmap.
  Format 3 cannot encode negative signed constants; proposed format 4 leaves
  the physical ABI/runtime and process/receipt/manifest versions unchanged.
- [x] Record approval of the concrete contract, including artifact format 4,
  on 2026-09-02.
- [x] Verify the 28.2 direct-check/emission boundary through the public CLI and
  process protocol. New constant forms preserve previous LLVM/native/interface
  outputs when emission fails; no Shuttle format or production behavior changes.
- [x] Obtain the separate coordinated implementation go-ahead (2026-09-02).
- [x] During compiler 28.3, require the reviewed format in capabilities/receipts;
  update fixtures, diagnostics, and docs together with the compiler reader/writer.
  Reject old artifacts and retain exact compiler, target, and dependency checks.
- [x] Verify source-free negative/computed constants, cross-package constant
  chains, public values derived from private constants without granting private
  access, aliases, and nominal integer/enum switch-label behavior.
  Completed 2026-09-02 with 25 protocol and 17 native tests against both compiler
  configurations, plus all ordinary Rust and tool-quality gates. See
  [the checkpoint record](docs/testing.md#compiler-283-constant-integration-checkpoint).

- [x] Test value/source edits and invalid constants: affected consumer rebuilds,
  unrelated-package reuse, changed coverage/duplicate labels, stale-link refusal,
  preserved outputs, and no stale program execution after failed `run`.
- [x] Prove relocated serial/parallel interface/native artifact determinism and
  whole/separate/source-free execution equivalence. Keep target-specific artifact
  comparisons separate from the compiler's cross-target scalar-bit tests.
- [x] Pass Rust formatting, Clippy, ordinary tests, Rust 1.85 checking, and shared
  protocol/native suites with both compiler configurations. Record the 28.4 exit
  audit without duplicating language rules or interpreting compiler metadata.

Completed 2026-09-02: all 27 shared protocol and 20 native tests pass against
both compilers, alongside 43 ordinary Rust tests and all Rust quality gates.
Same-value source edits still invalidate dependency digests; private-value and
transitive edits propagate, unrelated packages reuse, invalid constants and
duplicate labels preserve outputs, and stale links fail. Relocated one/four-job
artifacts match within each target; native executables match across a PE timestamp
boundary. See the [exit record](docs/testing.md#stage-284-scalar-constant-exit-audit).

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
