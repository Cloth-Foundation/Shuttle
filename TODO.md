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
change was required. Stage 32 coordination is complete following the compiler's
separately authorized 32.4 exit audit on 2026-09-04. No Shuttle production or
compatibility change was required. Stage 33 coordination is complete following
the compiler's separately authorized 33.4 exit audit on 2026-09-04. No Shuttle
production or compatibility change was required. Stage 34 coordination is
complete following the compiler's 34.4 exit audit on 2026-09-05.
Stage 35 coordination is complete following the separately authorized 35.4
exit audit on 2026-09-05.
Stage 36 coordination is complete following the 36.4 exit audit on 2026-09-06.
Stage 37 coordination is complete following the 37.4 exit audit on 2026-09-06.

## Scheduled work

### Stage 37: Portable program-argument coordination

- [x] Record the approved `shuttle run -- [ARGUMENT]...` boundary, host-native
  forwarding, compiler/runtime ownership, compatibility, diagnostics,
  verification, and non-goals.

  Completed with compiler 37.1 on 2026-09-06. Only `run` may receive values,
  and only after an explicit `--`. Shuttle must preserve argument boundaries and
  host-native values without decoding or command reconstruction. This checkpoint
  changes documentation only.
- [x] During compiler/runtime 37.2, keep Shuttle production behavior unchanged
  while coordinated fixtures establish the new `Main(string[] args)` and runtime
  ABI 5 boundary.

  Completed with compiler 37.2 on 2026-09-06. Direct and source-free entry
  adapters, strict Unicode conversion, managed ownership, GC rooting, malformed
  host-state failures, and zero-argument compatibility pass with development and
  sanitizer compilers. Shuttle production code and schemas remain unchanged.
- [x] During 37.3, implement exact trailing-argument forwarding and cover zero,
  empty, whitespace, option-like, Unicode, status, stream, failure, and reuse
  behavior.

  Completed 2026-09-06. The CLI accepts an `OsString` vector only after the
  `run --` delimiter and passes it directly to the completed executable. Values
  do not enter compiler requests or cache keys. Process-contract and native
  tests cover exact forwarding, zero arguments, warm reuse, status and stream
  behavior, malformed Unicode, and whole/source-free equivalence.
- [x] Complete the coordinated 37.4 development, sanitizer, native,
  cross-target, Rust, editor, documentation, formatting, and repository gates.

  Completed 2026-09-06. Both 269-test compiler configurations pass all 36
  compiler-backed toolchain cases and 33 native cases. Exact forwarding,
  statuses, typed errors, invalid Unicode, artifact identity, failure
  preservation, and stale-run prevention pass with Rust 1.85, warning-denied
  Clippy, editor, documentation, formatting, and repository gates.
  **Stage 37 is complete.**

### Stage 36: Standard-library prelude coordination

- [x] Record compiler ownership of `cloth.lang` eligibility and lookup while
  retaining Shuttle ownership of exact library selection, injection, build
  inputs, cache behavior, and toolchain diagnostics.

  Approved with compiler 36.1 on 2026-09-05. This checkpoint changes
  documentation only. Shuttle does not parse library declarations or maintain
  a prelude list, and every active compatibility/schema version remains
  unchanged.
- [x] During 36.2, keep production behavior unchanged while shared fixtures
  verify whole-project, separate-package, source-free, both-target, diagnostic,
  reuse, and deterministic artifact behavior.

  Completed with compiler 36.2 on 2026-09-05. A public process fixture copies
  the paired distribution, adds a synthetic `cloth.lang` type, and verifies
  source-free consumer artifacts on x86-64 and wasm32 with one and four jobs.
  Artifact bytes match and no Shuttle production path changed.
- [x] During 36.3, carry the separately approved library version and exact
  digest through existing capability, metadata, invalidation, receipt, and link
  paths without adding independent version solving.

  Completed with compiler 36.3 on 2026-09-05 and amended 2026-09-06. The paired
  distribution remains `cloth` v0.2.0 while the public errors move to recursive
  `cloth.lang.errors` identities. Existing metadata selection, capability
  validation, dependency receipts, cache keys, invalidation, and link closure
  carry the exact package identity; Shuttle does not interpret prelude layout.
- [x] Complete the coordinated 36.4 compiler, runtime, Shuttle, standard-
  library, editor, documentation, formatting, link, sanitizer, and repository
  exit gates.

  Completed 2026-09-06. Both 255-test compiler configurations pass, including
  all 36 compiler-backed toolchain cases and 32 native cases. All 49 ordinary
  Rust tests, Rust 1.85, warning-denied Clippy, formatting, editor,
  documentation, and repository gates pass. Shuttle production behavior and
  compatibility schemas remain unchanged.

  **Stage 36 coordination is complete.**

### Stage 35: Standard library coordination

- [x] Record the compiler-paired `cloth` package, reserved source root and
  dependency alias, executable-free bootstrap, automatic direct dependency,
  exact version/digest inputs, diagnostics ownership, compatibility,
  distribution, and non-goals.

  Approved with compiler 35.1 on 2026-09-05. The standard library remains
  explicitly imported in Cloth source, Shuttle does not parse its APIs, and
  compatibility remains artifact/compiler/runtime 5/5/4 with protocol 2,
  receipt schema 1, and manifest schema 1. This checkpoint changes
  documentation only.

- [x] During 35.2, keep production behavior unchanged while compiler and
  standard-library fixtures prove the ordinary artifact boundary carries the
  `cloth` package correctly.

  Completed with compiler 35.2 on 2026-09-05. Existing protocol-2 interface and
  object package boundaries carry `cloth` without a Shuttle production change.
- [x] During 35.3, locate the distribution paired with the selected compiler,
  inject it into every ordinary package without a manifest entry, reject user
  `cloth` aliases and replacement attempts, and retain deterministic graph,
  progress, reuse, invalidation, and atomic publication behavior.

  Completed 2026-09-05. The selected compiler's adjacent schema-1 metadata
  names one exact `cloth` distribution. Shuttle validates and injects it into
  every ordinary package, reserves the user alias and package identity, and
  uses the existing artifact, cache, scheduler, and linker paths.
- [x] Verify compiler-only packages, standard-library consumers, source-free
  artifacts, exact dependency edits, failed-output preservation, relocated
  one-job/four-job builds, x86-64/wasm32 outputs, and native execution.
- [x] Pass ordinary Rust, Rust 1.85, formatting, warning-denied Clippy, shared
  compiler, standard-library, editor, documentation, and repository gates in
  35.4.

  Completed 2026-09-05. Both compiler configurations pass all 35 public
  toolchain cases and 32 native cases inside their 255-test matrices. All 49
  ordinary Rust tests and coordinated quality gates pass. See the
  [exit record](docs/testing.md#stage-354-standard-library-foundation-exit-audit).

### Stage 34: Typed error coordination

- [x] Record the compiler-owned error declarations, throw expressions, typed
  public and inferred private throws sets, automatic propagation, constructor
  failure, inheritance/interface rules, division-by-zero migration, portable
  ABI, compatibility targets, verification, and non-goal boundaries.

  Approved with compiler 34.1 on 2026-09-04. The implementation targets artifact
  format 5, compiler ABI 5, and runtime ABI 4 while protocol 2, receipt schema 1,
  and manifest schema 1 remain unchanged. This checkpoint changes documentation
  only; active compiler and runtime compatibility constants remain unchanged.

- [x] With compiler 34.2, retain complete public error kinds and throws sets in
  deterministic compiler-owned interfaces and verify whole/source-free semantic
  agreement without Shuttle parsing or adapting error metadata.
- [x] With compiler 34.3, verify the format-5/compiler-ABI-5/runtime-ABI-4
  transition, native result/error lowering, terminal reporting, and
  division-by-zero behavior through public compiler and linker boundaries.
- [x] Prove relocated serial/parallel x86-64/wasm32 determinism with both
  compiler configurations. Existing integrity, stale-artifact, and
  failed-output matrices remain green; typed-error-specific expansion belongs
  to the 34.4 exit audit.
- [x] Pass ordinary Rust, Rust 1.85, formatting, Clippy, shared protocol/native,
  editor, documentation, link, and repository gates during compiler 34.4.

  Completed with compiler 34.4 on 2026-09-05. Both compiler configurations pass
  all 32 public protocol/toolchain and 30 native cases inside their 249-test
  matrices. Whole, separate, and source-free execution agree; one-job and four-
  job artifacts agree on x86-64 and wasm32; native success and failure reporting
  pass; and a typed-error API edit proves affected-only invalidation and
  failed-output preservation. All 43 ordinary Rust tests, Rust 1.85 checking,
  warning-denied Clippy, formatting, editor, documentation, link, and repository
  gates pass. Shuttle production retains only the opaque compatibility
  expectation at 5/5/4.

  **Stage 34 coordination is complete.**

### Stage 33: Numeric literal notation coordination

- [x] Record the compiler-owned scientific-notation, integer-base, separator,
  suffix-interaction, canonical-representation, compatibility, package, and
  non-goal boundaries. Approved 2026-09-04; this is documentation-only and
  Shuttle production code remains unchanged.
- [x] Record compiler 33.2 frontend completion. Both 226-test compiler matrices
  retain all 30 public protocol/toolchain and 26 native Shuttle cases. No
  Shuttle production, fixture, schema, or compatibility change was required.
- [x] With compiler 33.3, verify constants and executable behavior across
  whole-project, separate-package, and source-free compilation without parsing
  numeric syntax or compiler internals.
- [x] Prove affected-only invalidation, failed-output preservation, and
  relocated serial/parallel x86-64/wasm32 determinism with both compiler
  configurations.
- [x] Pass ordinary Rust, Rust 1.85, formatting, Clippy, shared
  protocol/native, editor, documentation, and repository gates during the
  coordinated compiler 33.4 exit audit.

  Completed with compiler 33.4 on 2026-09-04. Both compilers pass all 31 public
  toolchain and 28 native cases inside their 232-test matrices. All 43 ordinary
  Rust tests, Rust 1.85 checking, warning-denied Clippy, Rust/C++ formatting, 10
  editor tests per compiler, all 100 local Markdown target checks, and
  repository hygiene gates pass. Compatibility versions and Shuttle production
  code remain unchanged.

  **Stage 33 coordination is complete.** Stage 34 coordination is tracked above.

### Stage 32: Typed numeric literal coordination

- [x] Record the compiler-owned canonical suffix, exact typing, compatibility,
  package, and non-goal boundaries. Approved 2026-09-04; Shuttle production
  code remains unchanged.
- [x] Record compiler 32.2 frontend completion. Both compiler configurations
  pass the unchanged shared protocol and native Shuttle cases inside their
  216-test matrices; no Shuttle production code or compatibility version
  changed.
- [x] With compiler 32.3, verify suffixed constants and executable behavior
  across whole-project, separate-package, and source-free compilation without
  parsing numeric syntax or compiler internals.
- [x] Prove affected-only invalidation, failed-output preservation, and
  relocated serial/parallel x86-64/wasm32 determinism with both compiler
  configurations.
- [x] Pass ordinary Rust, Rust 1.85, formatting, Clippy, shared
  protocol/native, editor, documentation, and repository gates during the
  coordinated 32.4 exit audit.

  Completed 2026-09-04. Both compilers pass all 30 public toolchain and 26
  native cases inside their 224-test matrices. All 43 ordinary Rust tests, the
  Rust 1.85 minimum check, warning-denied Clippy, Rust/C++ formatting, both
  nine-test editor runs, documentation links, and repository hygiene gates
  pass. No production code or compatibility version changed.

  **Stage 32 coordination is complete.** Stage 33 coordination is tracked above.

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
- editor and language-server build integration.
