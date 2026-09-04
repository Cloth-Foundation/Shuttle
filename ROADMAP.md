# Shuttle roadmap

This roadmap is the authoritative implementation order for Shuttle. Cloth's
compiler roadmap owns compiler work; coordinated stages close only when each
repository has satisfied its own exit criteria and their shared protocol tests
pass.

## Stage discipline

Only one Shuttle stage may be active. A stage moves through `planned`, `active`,
and `complete` in that order. Activation requires written scope, non-goals, exit
criteria, concrete work in `TODO.md`, approval of public contracts, and an
explicit implementation go-ahead.

Stage 22 is complete, including the shared process, native, and sanitizer
verification. Stages 23 through 26 are also complete, including the approved
aggregate compatibility transition and the coordinated equivalence and exit
audit. The coordinated 26.5.1 override follow-up is also complete. The compiler
roadmap owns the language and ABI contracts. Stage 27 coordination is complete;
the approved switch contract and its shared exit audit passed on 2026-09-02.

Stage 28 coordination is complete, including the separately authorized
[28.4 exit audit](docs/testing.md#stage-284-scalar-constant-exit-audit) on
2026-09-02. Format-4 integration, source-free values, dependency evolution,
output preservation, and deterministic builds pass with both compilers.
Stage 29 coordination is complete following the separately authorized
[29.4 exit audit](docs/testing.md#coordinated-294-checked-runtime-arithmetic-exit-audit)
on 2026-09-03. The compiler/runtime transition and every coordinated verification
gate pass.
Stage 30 coordination is complete following the compiler's separately
authorized 30.4 exit audit on 2026-09-03. No Shuttle implementation or protocol
change was required.

Stage 31 coordination is complete following the compiler's separately
authorized 31.4 exit audit on 2026-09-04. Coordinated package paths and every
quality gate pass without a Shuttle production change.

## Stage 31: MIR optimization coordination

Status: **complete — coordinated 31.4 exit audit passed 2026-09-04**

The compiler-owned contract inserts an always-on target-independent optimizer
between verified MIR and ABI lowering. Shuttle continues to treat interfaces,
scalar constants, and object code as opaque compiler output; it does not parse
MIR, folded constants, pass names, or optimizer metadata.

Artifact format 4, compiler ABI 4, runtime ABI 3, protocol 2, receipt schema 1,
and manifest schema 1 remain unchanged. The compiler executable digest already
invalidates artifacts produced by a different compiler binary. Stage 31 adds no
optimization manifest option, cache-key field, capability, receipt field, or
scheduler behavior.

Coordinated exit requires whole/separate/source-free behavioral equivalence,
affected-only invalidation, failed-output preservation, and relocated
serial/parallel artifact determinism for x86-64 and wasm32 with both compiler
configurations. Optimizer implementation, MIR verification, scalar semantics,
and baseline/optimized compiler comparisons remain compiler responsibilities.

The exit audit passes those package guarantees in both 215-test compiler
configurations, plus all 43 ordinary Rust tests, Rust 1.85, formatting,
warning-denied Clippy, editor checks, and repository gates. Compatibility
versions remain unchanged, and no later stage is active.

## Stage 30: Integer conversion-mode coordination

Status: **complete — compiler 30.4 exit audit passed on 2026-09-03**

The compiler-owned contract defines `Target::wrap(value)` and
`Target::sat(value)` as integer-only primitive meta conversions. Shuttle will
treat their object code and scalar constant values as opaque artifact content.

Artifact format 4, compiler ABI 4, runtime ABI 3, protocol 2, receipt schema 1,
and manifest schema 1 remain unchanged. Stage 30 adds no runtime helper,
capability field, receipt field, manifest behavior, or scheduler behavior.

The coordinated exit verifies whole/separate/source-free equivalence,
affected-only invalidation, output preservation, relocated serial/parallel
determinism, and all Rust/shared tests against both compiler configurations.
Language syntax, conversion semantics, HIR/MIR, LLVM lowering, and diagnostics
remain compiler responsibilities.

## Stage 29: Checked-arithmetic runtime ABI coordination

Status: **complete — coordinated 29.4 exit audit passed 2026-09-03**

The compiler's `docs/proposals/stage_29_checked_runtime_arithmetic.md` proposal
defines checked runtime integer arithmetic and uses runtime ABI **3**. The
user approved the concrete contract, including runtime ABI 3, on 2026-09-03.
The separately authorized 29.2 compiler/runtime implementation completed on
2026-09-03.

Runtime ABI is private compiler artifact metadata and is absent from public
capabilities and receipts. Shuttle keeps artifacts opaque and delegates
runtime-ABI validation to `clothc` during inspect, reuse, and link. Its remaining
role is to prove whole/separate/source-free reuse, failure preservation, and
determinism without inspecting expressions, MIR, guard codes, or object payloads.

Artifact format 4, compiler ABI 4, protocol 2, receipt schema 1, and manifest
schema 1 remain unchanged. Runtime-ABI-2 artifacts require rebuilding
after the coordinated transition. No manifest, scheduler, or dependency behavior
changes.

Exit requires the compiler's approved semantics, exact ABI coordination, stale
artifact rejection, affected-consumer invalidation, unrelated reuse, preserved
outputs, source-free execution, relocated deterministic builds, and all Rust and
shared tests against both compiler configurations.

## Stage 28: Scalar-constant artifact coordination

Status: **complete — coordinated 28.4 exit audit passed 2026-09-02**

Objective: consume the compiler's reviewed scalar-constant artifact contract and
verify computed values, dependency evolution, and whole/separate/source-free
equivalence through the existing public process boundary.

Prerequisites satisfied: completed Stage 27 and approval of the compiler's
concrete Stage 28 source/evaluation/format contract. Compiler 28.2 was authorized;
the separate 28.3 integration and 28.4 audit go-aheads were received on
2026-09-02. The compiler owns the approved contract in
`docs/proposals/stage_28_scalar_constants.md` in its checkout.

The approved contract requires artifact format **4** because format 3 cannot
represent negative signed static constants. Compiler ABI **4**, runtime ABI **2**, process
protocol **2**, receipt schema **1**, and manifest schema **1** remain unchanged.
Capabilities/receipts require format 4. Old artifacts must be rebuilt,
not interpreted under the new format. No expression trees enter the protocol.

Deliverables:

1. Record the dependency and compatibility review without changing behavior.
2. Coordinate format requirements, compiler capability/receipt validation,
   fixtures, old-format diagnostics, and documentation with compiler 28.3.
3. Verify negative and computed scalar constants, private/public dependencies,
   nominal enums, and switch references without reopening dependency sources.
4. Prove constant/dependency edits invalidate affected consumers, stale links
   fail, failures preserve output and never run stale programs, and relocated
   serial/parallel artifacts and native execution remain equivalent.

Non-goals: parsing/evaluating Cloth expressions, decoding private compiler
metadata, new manifest or process fields, new cache/scheduling policy, runtime
initialization, remote dependencies, or unrelated language/tooling features.

Exit requires all Stage 28 work items plus ordinary Rust quality gates and
shared protocol/native tests with both development and sanitizer compilers.
Compiler/source semantics and editor/user-language documentation remain compiler
stage responsibilities. Further format/ABI deviations require separate review.

## Stage 27: Switch keyword and dependency-evolution coordination

Status: **complete — coordinated 27.4 exit audit passed 2026-09-02**

The [exit audit](docs/testing.md#stage-274-switch-exit-audit) verifies source-free
case/constant evolution, consumer invalidation, stale-link rejection, preserved
outputs, and relocated serial/parallel equivalence with both compilers.

The compiler owns switch syntax, exhaustive enum checking, and lowering. Shuttle
work is limited to matching the new `switch`/`case`/`default` dependency-alias
restrictions and verifying source-free enum/scalar constants, dependency edits,
failed-output preservation, and whole/separate/serial/parallel behavior.
Package-name grammar, opaque artifact ownership, and scheduling remain unchanged.

Prerequisite satisfied: compiler Stage 27 contract and implementation approval.
No artifact/ABI/process/receipt/manifest revision is expected; coordinate
any separately approved change before implementing it. Exit requires the
compiler's development/sanitizer audit plus Rust and shared protocol/native gates.

## Stage 26.5.1: Explicit interface-override compatibility

Status: **complete — coordinated audit passed 2026-09-02**

The [exit audit](docs/testing.md#stage-2651-explicit-interface-overrides) records
shared source-free/native verification with both compilers and the Rust gates.

Update shared interface fixtures for mandatory local `override` and verify
source-free enforcement, inherited implementation reuse, diagnostics, and native
dispatch with both compiler configurations. Artifact format 3, compiler ABI 4,
runtime ABI 2, and all process/receipt/manifest protocols remain unchanged.
No scheduling or dependency-resolution feature is included. Close after the
compiler/editor audit and ordinary Rust plus shared toolchain/native gates pass.

## Stage 26: Aggregate artifact compatibility

Status: **complete — coordinated 26.4 exit audit passed 2026-09-02**

The [exit audit](docs/testing.md#stage-264-struct-exit-audit) verifies relocated
serial/parallel artifacts and executables, private-layout/member invalidation,
unrelated-package reuse, source-free aggregate calls and privacy, whole-project
equivalence, and both compiler configurations with the Rust quality gates.

Objective: support the compiler-owned struct artifact/ABI transition and prove
aggregate behavior across the existing separate-compilation boundary.

Prerequisite: completed Stage 25. The compiler's source contract was approved
on 2026-09-02, followed by the separate 26.3 ABI/schema approval. The implemented
transition is artifact format 3, compiler ABI 4, and runtime ABI 2. Shuttle
requires format 3 capabilities/receipts; older packages must be rebuilt.
Process protocol 2, receipt schema 1, and manifest schema 1 are unchanged.

Deliverables: coordinate reviewed capability/receipt version requirements;
test source-free struct dependencies, aggregate calls and GC-bearing values,
layout-change invalidation, and whole-project/separate and serial/parallel
equivalence. Continue treating artifacts as opaque compiler-owned data.

Non-goals: manifest or process-protocol changes, compiler internals in Rust,
artifact deserialization, remote dependencies, or new build scheduling policy.

Exit criteria: reviewed version mismatches fail clearly; aggregate dependency
execution and source-free consumption pass; layout changes invalidate affected
consumers; deterministic artifacts, Rust checks, and shared development/native/
sanitizer suites pass. The compiler roadmap owns language and runtime work.

## Stage 25: Enum artifact compatibility

Status: **complete**

Completed on 2026-09-02 with the coordinated compiler/Rust audit recorded in
[`docs/testing.md`](docs/testing.md#stage-25-enum-exit-audit).

Objective: support Cloth's approved named-enum stage through compiler artifact
format 2 without importing language semantics into Shuttle.

Prerequisite: completed Stage 24. The compiler's Stage 25 implementation
authorization includes this coordinated compatibility work.

Deliverables: validate format-2 capabilities/receipts, update protocol fixtures,
and verify enum builds, dependency reuse/invalidation, and whole-project versus
separate and serial versus parallel equivalence.

Non-goals: manifest or process-protocol changes, artifact deserialization,
remote dependencies, and additional build-system features.

Exit criteria: format mismatches fail deterministically; source-free enum
consumption and case-edit invalidation pass; Rust and shared compiler native,
development, and sanitizer suites pass.

## Stage 22: Local projects and compiler build protocol

Status: **complete**

Objective: bootstrap Shuttle around a deterministic local-project contract and
invoke `clothc` without transferring build-system policy into the compiler.

Prerequisites: Cloth compiler Stage 21 and the approved Shuttle/compiler
ownership boundary.

Deliverables:

1. Freeze the implementation-language policy, versioned `Shuttle.toml` schema,
   and Shuttle-to-compiler build protocol.
2. Parse and validate the approved manifest, then resolve the ordered local
   dependency graph and diagnose invalid manifests, roots, identities, paths,
   duplicates, and cycles before compiler invocation.
3. Construct and invoke the approved compiler build request without
   depending on compiler implementation details.
4. Verify deterministic behavior with Shuttle unit tests and cross-repository
   projects covering direct and dependency builds.

Non-goals:

- remote registries, dependency downloads, semantic-version solving, or
  lockfile generation;
- build scripts, plugins, or arbitrary command execution;
- incremental compilation, shared or remote caches, and package publishing;
- separately compiled dependency artifacts, which belong to Stage 23.

Exit criteria:

- every Stage 22 item in `TODO.md` is complete;
- equal manifests and filesystem inputs produce an equal ordered build request;
- invalid project graphs fail before `clothc` is invoked;
- Shuttle never parses Cloth source or consumes private compiler structures;
- local dependency projects pass the shared compiler integration suite; and
- Shuttle's development and applicable sanitizer suites pass.

The repeatable standalone and shared commands are in
[`docs/testing.md`](docs/testing.md). The compiler's `docs/testing.md` records
the coordinated exit audit.

## Stage 23: Separate compilation and deterministic linking

Status: **complete**

The [process-v2 proposal](docs/proposals/compiler_protocol_v2.md) and its linked
compiler artifact proposal were approved with implementation authorization on
2026-08-31. Ordered package compilation and link orchestration are implemented
through Stage 23.3. Stage 23.4 completed equivalence and exit verification on
2026-09-01.

Objective: orchestrate independent local-package compilation and deterministic
linking through the compiler-owned, versioned artifact contract.

Prerequisite: Stage 22.

Deliverables:

1. Consume and validate public compiler artifact receipts without parsing
   semantic/ABI metadata or native objects.
2. Build dependencies in deterministic topological order and share each
   compatible artifact across consumers within that invocation. Automatic
   reuse across commands remains deferred.
3. Invoke deterministic linking and report package context for compiler or
   linker failures without changing their meaning.
4. Verify equivalence with the compiler's whole-project pipeline.

Non-goals:

- automatic incremental caching, remote caches, distributed builds,
  registries, dynamic loading, or ABI stability across Cloth releases;
- optimization policy beyond forwarding approved build-profile inputs.

Exit criteria:

- every Stage 23 item in `TODO.md` is complete;
- incompatible, missing, or duplicate artifacts fail deterministically;
- independently compiled and reused artifacts produce programs equivalent to
  whole-project compilation, without a source-freshness/cache claim; and
- shared compiler integration and Shuttle verification suites pass.

## Stage 24: Responsive and observable local builds

Status: **complete**

Objective: make local builds visibly active, accelerate clean builds at measured
bottlenecks, and reuse unchanged verified package artifacts safely.

Prerequisite: Stage 23.

Deliverables:

1. Emit concise package, link, completion, and run progress on standard error,
   with elapsed timing and no changes to program standard output.
2. Remove measured cold-path identity and process overhead without weakening
   the compiler-owned compatibility contract.
3. Own conservative local incremental state and reuse validated artifacts across
   commands with exact invalidation and atomic publication.
4. Schedule independent packages concurrently under a bounded job count while
   retaining deterministic outputs and diagnostics.

Non-goals:

- compiler-internal incremental compilation, a daemon, or watch mode;
- shared, remote, or distributed caches;
- remote dependencies, registries, or package publication; and
- timestamp-only freshness decisions or unvalidated artifact reuse.

Exit criteria:

- long operations are preceded by stable progress and successful `run` program
  streams remain unchanged;
- the recorded small-project clean build is materially faster;
- unchanged builds compile no packages and all declared invalidation inputs are
  covered by tests;
- single-job and parallel builds are byte- and diagnostic-equivalent; and
- Rust, shared compiler, development, sanitizer, native, and responsiveness
  suites pass.
