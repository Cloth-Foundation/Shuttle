# Stage 24 local artifact reuse

Status: **implemented; Stage 24.3 completed 2026-09-01**.

This contract extends protocol version 2 without changing Cloth source or
manifest syntax. Shuttle owns local build state and scheduling. `clothc` owns
artifact integrity, current-input validation, and compatibility decisions.

## Reuse operation

Shuttle may validate one existing package artifact with:

```text
clothc --shuttle-protocol 2
       --operation reuse
       --target x86_64|wasm32
       --artifact-kind interface|object
       --input ABSOLUTE_CANDIDATE_PATH
       --package PACKAGE_NAME PACKAGE_VERSION ABSOLUTE_SOURCE_ROOT
       [--entry SOURCE_RELATIVE_PATH]
       { --dependency ALIAS DEPENDENCY_PACKAGE }
       { --artifact PACKAGE_NAME PACKAGE_VERSION ARTIFACT_DIGEST ABSOLUTE_PATH }
```

The package, dependency, and artifact arguments have the same ordering and
meaning as `compile`. The artifact records supply the candidate's complete
reachable dependency closure; they do not include the candidate itself.

`clothc` accepts reuse only after validating all of the following:

- the candidate envelope, digest, schema, package identity, kind, and target;
- exact compiler, compiler ABI, runtime ABI, target-layout, and—when native—
  runtime, LLVM, linker, and code-generation compatibility;
- the current sorted source inventory and SHA-256 of every source byte;
- direct aliases, dependency identities and artifact digests, plus the complete
  validated dependency closure; and
- the selected root entry, when one is supplied.

Status 0 returns the ordinary artifact receipt and empty stderr. Status 3 with
empty stdout and stderr is a normal cache miss: the candidate is stale,
incompatible, malformed, or no longer represents the current source/entry.
Invocation errors, unreadable current build inputs, and unavailable required
tools remain status 2. Shuttle compiles only after status 3; it never falls back
to an old artifact after a compiler error.

## Shuttle state and layout

Object artifacts retain their existing location under
`target/x86_64/packages`. Persistent interface artifacts use
`target/TARGET/check/packages`. Each workspace has an independent build lock
and `.shuttle/state` directory, so checks and native builds cannot exchange
artifact kinds.

For each package, Shuttle records immutable schema-1 JSON containing the exact
UTF-8 manifest snapshot captured during graph resolution and the validated
artifact receipt. A record is useful only when its manifest is byte-identical
to the current snapshot and the compiler's returned reuse receipt exactly
matches the stored receipt. Unknown, oversized, malformed, or orphaned state is
ignored as a cache miss. State never contains source hashes copied from private
artifact metadata.

State publication uses a private `create_new` file in the package state
directory, flushes and synchronizes the complete JSON, then renames it to a
unique visible `.json` name. The rename never replaces an existing record.
Older records are best-effort cleanup only; a crash can leave them but cannot
make them authoritative. The artifact itself is still published atomically by
`clothc` before its receipt is recorded.

## Invalidation

There are no timestamp freshness decisions. Exact manifest-byte changes force
that package through compilation. Source changes are detected by `clothc`.
Dependency artifact changes invalidate consumers through the digest pins
already embedded in `.cpa`. Compiler, target, runtime, LLVM, linker, or native
configuration changes fail compatibility validation. Byte-identical artifacts
may therefore stop invalidation at the first unchanged digest, which is safe
and more precise than rebuilding every downstream package.

An unchanged command performs no package compilation but may still validate
artifacts and relink an executable. Per-file incremental compilation, link
caching, shared caches, daemons, watch mode, and remote artifacts remain out of
scope.
