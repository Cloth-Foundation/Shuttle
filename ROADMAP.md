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
verification. Stage 23 is active under the approved shared contract.

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

Status: **active**

The [process-v2 proposal](docs/proposals/compiler_protocol_v2.md) and its linked
compiler artifact proposal were approved with implementation authorization on
2026-08-31. Compiler identity/artifact foundations precede Shuttle's process-v2
integration; approval does not claim those operations already exist.

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
