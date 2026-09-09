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
Stage 38 coordination is complete following the 38.4 exit audit on 2026-09-06.
Stage 39 coordination is complete following the 39.4 exit audit on 2026-09-06.
Stage 40 coordination is complete following the compiler's 40.4 exit audit on
2026-09-07.
Stage 41 coordination is complete following the compiler's separately
authorized 41.4 exit audit on 2026-09-07.
Stage 42 coordination is complete following the compiler's separately
authorized 42.4 exit audit on 2026-09-08.
Stage 43 coordination is complete following the compiler's 43.4 exit audit on
2026-09-08.
Stage 44 coordination is complete following the compiler's 44.4 exit audit on
2026-09-08. Stage 45.5 coordination is complete following its shared 45.5d exit
audit on 2026-09-09.

## Scheduled work

### Stage 45.5: Universal Object coordination

- [x] Preserve exact compiler/library selection through the Object contract and
  root-dispatch checkpoints without interpreting compiler-owned object metadata.
- [x] During 45.5c, advance compatibility to artifact/compiler/runtime 8/7/11,
  select the exact `cloth` v0.5.0 distribution, invalidate stale packages, and
  retain schemas 2/1/1/1.

  Completed 2026-09-09. Capability checks, compiler stubs, both-target standard-
  library checks, source-free package tests, and stale-version rejection use the
  coordinated versions. No Shuttle manifest or process-protocol field changed.
- [x] During 45.5d, close exact reuse and invalidation, native and sanitizer
  execution, determinism, documentation, formatting, and repository gates.

  Completed 2026-09-09. The 38-case native suite permanently covers value-box
  execution, evaluation order, equality/hash, source-free linking,
  serial/parallel artifacts, and warm reuse with both development and sanitized
  compilers. All ordinary Rust, compiler-backed, formatting, lint, MSRV,
  documentation, and repository gates pass without a schema expansion.

### Stage 44: Self-hosted lexer coordination

- [x] Record that Shuttle is opaque to source bytes, tokens, diagnostics,
  scanners, and parity records; preserve exact compiler/library selection,
  unchanged compatibility and schemas, deterministic bootstrap builds, and
  the existing failure-preservation boundary.

  Completed with compiler 44.1 on 2026-09-08. This documentation-only
  checkpoint changes no Shuttle source, protocol, schema, cache input, or
  behavior. Compatibility remains 7/6/10 and 2/1/1/1 with `cloth` v0.4.0.
- [x] During 44.2, verify source/scanner foundation checks on x86-64 and wasm32,
  native execution, exact warm reuse, affected invalidation, and completed
  output preservation without a Shuttle production change.

  Completed 2026-09-08. Both targets accept the complete bootstrap source
  graph, development and sanitizer native executions pass the lexer foundation
  corpus, and exact package selection remains unchanged. No Shuttle source,
  protocol, schema, or cache semantics changed.
- [x] During 44.3, verify complete lexer and malformed-input bootstrap paths
  while keeping token and diagnostic meaning compiler-owned.

  Completed 2026-09-08. Both targets accept the complete literal scanner graph;
  development and sanitizer native runs pass the valid, malformed,
  unterminated, invalid-UTF-8, exact-buffer, and tracked-source corpus. A warm
  run reuses the exact `cloth` v0.4.0 and `clothc` package artifacts. No Shuttle
  production source, protocol, schema, manifest, receipt, or cache meaning
  changed.
- [ ] Complete the coordinated 44.4 determinism, bootstrap, native,
  sanitizer, Rust/MSRV, formatting, documentation, and repository gates.

### Stage 43: Portable file-byte coordination

- [x] Record that Shuttle is opaque to application file paths, bytes, runtime
  statuses, and bootstrap source meaning; preserve the inherited child working
  directory; and freeze current and planned compatibility and exact library
  pairing.

  Completed with compiler 43.1 on 2026-09-08. At that documentation-only
  checkpoint, compatibility remained 7/6/9 and 2/1/1/1 with `cloth` v0.3.0;
  runtime ABI 10 and `cloth` v0.4.0 were reserved for 43.2.
- [x] During 43.2, verify the compiler/runtime/library transition, both-target
  artifacts, exact dependency invalidation and reuse, input-independent cache
  behavior, and failure preservation without adding Shuttle production code.

  Completed with compiler 43.2 on 2026-09-08. The 7/6/10 compiler/runtime
  transition, exact `cloth` v0.4.0 pairing, both-target artifacts, warm reuse,
  runtime-input independence, and failure preservation pass without a Shuttle
  production or schema change.
- [x] During 43.3, verify the real bootstrap source-file consumer through
  direct and Shuttle x86-64/wasm32 builds, native and source-free execution,
  warm reuse, affected invalidation, deterministic artifacts, and failed-build
  preservation.

  Completed with compiler 43.3 on 2026-09-08. The real `F:\Cloth` project
  checks on both targets, runs natively with development and sanitizer
  compilers, links from source-free artifacts, reuses exact warm outputs, and
  preserves completed artifacts and the runnable executable after an invalid
  bootstrap edit. Shuttle production code and schemas remain unchanged.
- [x] Complete the coordinated 43.4 development, sanitizer, native,
  cross-platform, bootstrap, Rust/MSRV, editor, documentation, formatting, and
  repository gates.

  Completed 2026-09-08. Both 354-test compiler configurations, all 38 ignored
  toolchain and 37 ignored native cases, 51 ordinary Rust tests, Rust 1.85,
  Clippy, formatting, input-independent caching, deterministic packages,
  source-free/relocated builds, and the real bootstrap project pass. Shuttle
  production code and schemas remain unchanged.

### Stage 42: Runtime-sized fixed-array coordination

- [x] Record opaque source/default/layout ownership, planned unchanged
  artifact/compiler/runtime 7/6/9 compatibility, exact v0.3.0 standard-library
  pairing, unchanged protocol/schemas, bootstrap verification, and non-goals.

  Completed with compiler 42.1 on 2026-09-07. This documentation-only
  checkpoint changes no production Shuttle code or compatibility. Shuttle
  does not interpret `T[:length]`, element defaults, allocation failures, or
  token-buffer contents.
- [x] During 42.2, preserve existing compiler coordination while runtime-sized
  construction remains internal verified IR and is not a separately releasable
  native feature.

  Completed with compiler 42.2 on 2026-09-08. Shuttle production code and
  schemas remain unchanged. Compiler protocol-v2 interface and object compile
  operations reject runtime-sized construction before artifact staging or
  publication, including x86-64 and wasm32 interface requests.
- [x] During 42.3, verify both targets, whole/separate/source-free and native
  projects, exact reuse, affected invalidation, failure preservation,
  deterministic publication, and the `F:\Cloth` token-buffer smoke path under
  the approved unchanged compatibility boundary.

  Completed with compiler 42.3 on 2026-09-08. Dedicated runtime-array fixtures
  pass serial/parallel x86-64 and wasm32 checks, deterministic artifacts, exact
  reuse, affected invalidation, failed-build preservation, whole-project,
  separate-package, source-free, and native execution. The real bootstrap
  project passes Shuttle checks on both targets and runs natively.
- [x] Complete the coordinated 42.4 development, sanitizer, native,
  cross-target, bootstrap, Rust/MSRV, editor, documentation, formatting, and
  repository gates.

  Completed 2026-09-08. Distinct-root serial and parallel builds produce
  byte-identical x86-64/wasm32 artifacts and native executables. Whole,
  separate, and source-free execution, exact reuse, affected invalidation,
  failed-output preservation, both compiler configurations, and the real
  bootstrap project pass. All 51 ordinary Rust tests, Rust 1.85, Clippy, and
  formatting pass without changing Shuttle production code or schemas.

### Stage 41: Uniform-nullability coordination

- [x] Record opaque source/type/layout ownership, the planned
  artifact/compiler/runtime 7/6/9 transition, exact v0.3.0 standard-library
  pairing, unchanged protocol/schemas, coordinated verification, and
  non-goals.

  Completed with compiler 41.1 on 2026-09-07. This documentation-only
  checkpoint leaves compatibility at 6/5/8 and 2/1/1/1. Shuttle adds no source,
  tag, payload, flow, or safe-operation interpretation.
- [x] During 41.2, preserve existing compiler coordination while nullable
  values and safe operations are internal verified IR and not a separately
  releasable native feature.

  Completed with compiler 41.2 on 2026-09-07. Development and sanitizer
  configurations each pass all 312 compiler CTests, including the
  compiler-backed and native Shuttle suites. The native/artifact gate preserves
  production compatibility at 6/5/8 and schemas at 2/1/1/1.
- [x] During 41.3, carry artifact format 7, compiler ABI 6, and runtime ABI 9
  through capability, receipt, compiler identity, standard-library pairing,
  native linking, execution, reuse, invalidation, failure preservation, and
  stale-run prevention.

  Completed with compiler 41.3 on 2026-09-07. Shuttle requires format 7 in
  capabilities and receipts, rejects stale format 6 data, and retains schemas
  2/1/1/1. Nullable fixtures pass deterministic x86-64/wasm32 artifacts, exact
  reuse, affected dependency invalidation, whole/separate/source-free native
  execution, and serial/parallel equivalence. All 51 ordinary Rust tests, 37
  compiler-backed tests, 35 native tests, formatting, and warning-denied Clippy
  pass.
- [x] Complete the coordinated 41.4 development, sanitizer, native,
  cross-target, Rust/MSRV, editor, documentation, formatting, and repository
  gates.

  Completed with compiler 41.4 on 2026-09-07. Nullable package edits now prove
  deliberate failure diagnostics and atomic preservation of both-target
  artifacts and the last native executable in addition to determinism, reuse,
  affected invalidation, whole/separate/source-free execution, and stale-run
  prevention. Both 335-test compiler configurations, all 51 ordinary Rust
  tests, 37 compiler-backed tests, 35 native tests, Rust 1.85, Clippy,
  formatting, documentation, and repository gates pass.

### Stage 40: Unicode string-slicing coordination

- [x] Record opaque source/bounds/content/allocation ownership, the planned
  runtime-ABI-8 transition, exact v0.3.0 standard-library pairing, unchanged
  format/compiler/protocol/schemas, coordinated verification, and non-goals.

  Completed with compiler 40.1 on 2026-09-06. This documentation-only
  checkpoint leaves compatibility at 6/5/7 and 2/1/1/1. Shuttle adds no source
  parsing or string-content behavior.
- [x] During 40.2, preserve existing compiler coordination while slicing is
  internal verified IR and not a separately releasable feature.

  Completed with compiler 40.2 on 2026-09-06. Shuttle remains unchanged and
  opaque; compiler frontend validation succeeds while LLVM/native publication
  is rejected until runtime and lowering arrive together in 40.3.
- [x] During 40.3, carry runtime ABI 8 through native linking, slicing
  consumers, exact paired-library selection, reuse, invalidation,
  failure-preservation, and stale-run prevention.

  Completed with compiler 40.3 on 2026-09-07. The existing public protocol and
  schemas carry runtime ABI 8 without reinterpretation. Whole, separate, and
  source-free native fixtures execute the same Unicode-scalar slice while
  retaining exact compiler/library pairing, reuse, invalidation, and atomic
  publication behavior.
- [x] Complete the coordinated 40.4 development, sanitizer, native,
  cross-target, Rust/MSRV, editor, documentation, formatting, and repository
  gates.

  Completed with compiler 40.4 on 2026-09-07. Both compiler configurations pass
  all 36 toolchain and 34 native cases. All 51 ordinary Rust tests, Rust 1.85,
  warning-denied Clippy, formatting, documentation, and repository gates pass
  without a Shuttle production or schema change.

### Stage 39: Unicode string-traversal coordination

- [x] Record opaque source/scalar/traversal ownership, artifact-format-6 and
  runtime-ABI-7 transitions, exact v0.3.0 standard-library pairing, unchanged
  schemas, coordinated verification, and non-goals.

  Completed with compiler 39.1 on 2026-09-06. This checkpoint changes
  documentation only, so active compatibility remains 5/5/6 and 2/1/1/1.
- [x] During 39.2, accept artifact format 6 through existing capability and
  receipt fields; update compiler stubs, fixtures, rebuilds, source-free
  consumers, both targets, exact reuse, and invalidation without parsing
  character values.

  Completed 2026-09-06. Format 6 is required by production negotiation and
  stubs. Existing constant, cross-target, source-free, relocation, reuse,
  invalidation, and failure-preservation matrices now carry a non-BMP character
  while Shuttle remains opaque to its value.
- [x] During 39.3, carry runtime ABI 7 through native linking, Unicode traversal
  consumers, exact paired-library selection, reuse, invalidation,
  failure-preservation, and stale-run prevention.

  Completed 2026-09-06. Compiler-backed whole, separate, and source-free
  fixtures execute scalar indexing and iteration under runtime ABI 7 while
  exact pairing, reuse, invalidation, and atomic output behavior remain intact.
  Shuttle does not inspect the operations or their contents.
- [x] Complete the coordinated 39.4 development, sanitizer, native,
  cross-target, Rust/MSRV, editor, documentation, formatting, and repository
  gates.

  Completed 2026-09-06. Both 296-test compiler configurations, all 36
  compiler-backed toolchain cases, all 34 native cases, 51 ordinary Rust tests,
  Rust 1.85, warning-denied Clippy, formatting, editor, documentation, and
  repository gates pass. Compatibility remains 6/5/7 and 2/1/1/1 with
  `cloth` v0.3.0.

### Stage 38: Portable text-input coordination

- [x] Record standard-input ownership, input-independent compiler and cache
  behavior, the trusted compiler/library boundary, runtime and library version
  transitions, compatibility, verification, and non-goals.

  Completed with compiler 38.1 on 2026-09-06. Shuttle inherits stdin only for
  the launched application and never reads, decodes, buffers, logs, hashes, or
  sends it through the compiler protocol. This checkpoint changes documentation
  only; compatibility remains 5/5/5 and 2/1/1/1 with `cloth` v0.2.0.
- [x] During 38.2, carry runtime ABI 6 and `cloth` v0.3.0 through the existing
  capability, toolchain-metadata, package selection, artifact, cache,
  invalidation, receipt, and link boundaries without a schema change.

  Completed 2026-09-06. Exact v0.3.0 library selection and runtime-ABI-6
  artifacts pass compiler-backed capability, metadata, source-free, link, and
  native tests without a Shuttle production or schema change.
- [x] During 38.3, verify interactive and redirected stdin inheritance, strict
  input and parsing behavior, input-independent artifacts, direct/Shuttle and
  whole/separate/source-free equivalence, both targets, streams, statuses,
  failure preservation, and stale-run prevention.

  Completed 2026-09-06. Compiler-backed native coverage proves redirected stdin
  inheritance, strict success and `ParseError` failure behavior, unchanged
  artifacts, exact reuse on a second run, and source-free paired-library
  execution. Shuttle production code and schemas remain unchanged.
- [x] Complete the coordinated 38.4 development, sanitizer, native,
  cross-target, Rust/MSRV, editor, documentation, formatting, and repository
  gates.

  Completed 2026-09-06. Input-independent artifacts, exact paired-library
  invalidation and reuse, relocated serial/parallel determinism, source-free
  parsing, failed-output preservation, native and both-target behavior, and all
  Rust/MSRV, editor, documentation, formatting, sanitizer, and repository gates
  pass without a Shuttle production or schema change.

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
