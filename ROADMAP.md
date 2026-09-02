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
verification. Stages 23 and 24 are also complete.

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
