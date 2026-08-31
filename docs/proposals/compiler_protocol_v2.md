# Proposal: Shuttle-to-`clothc` protocol version 2

Status: **approved Stage 23.1 contract, 2026-08-31; not yet implemented**.

The compiler's companion proposal is
[`docs/proposals/stage_23_artifacts.md`](../../../docs/proposals/stage_23_artifacts.md)
in the Cloth checkout. It owns artifact contents, canonical identity,
compatibility, validation, and linking semantics. This document owns the public
process boundary and Shuttle orchestration. Neither proposal changes the
implemented [protocol v1](../compiler_protocol.md).

## Ownership and capability negotiation

Shuttle stays in Rust and `clothc` stays in C++23. The boundary remains direct
child-process invocation with OS-native argument vectors, no shell, FFI, shared
headers, or compiler library dependency. Manifest schema version 1 is unchanged.

The existing `clothc --shuttle-protocol-version` query continues to return
exactly `1` plus newline for older Shuttle clients. New Shuttle clients use:

```text
clothc --shuttle-protocol-capabilities
```

It accepts no other argument and returns one UTF-8 JSON object plus LF, empty
stderr, and status 0:

```json
{"schema":1,"protocols":[1,2],"artifact_formats":[1],"compiler_id":"<64 lowercase hex digits>","operations":["compile","inspect","link"],"interface_targets":["x86_64","wasm32"],"object_targets":["x86_64"]}
```

The digest placeholder represents the artifact contract's actual compiler
digest. Target lists describe implemented operations, not a promise that
optional native tools are installed. Interface checking must work without
them. Compiler/native-tool discovery failures are diagnosed only when the
requested operation needs those tools.

New Shuttle requires protocol 2 and artifact format 1 for this workflow. A
missing/malformed query or unsupported required capability fails clearly;
there is no silent fallback to a whole-project build. Old clients retain v1.
Tests invoke v1 explicitly as the equivalence oracle.

## Compile one package

Canonical argument order is:

```text
clothc --shuttle-protocol 2
       --operation compile
       --target x86_64|wasm32
       --artifact-kind interface|object
       --output ABSOLUTE_ARTIFACT_PATH
       --package PACKAGE_NAME PACKAGE_VERSION ABSOLUTE_SOURCE_ROOT
       [--entry SOURCE_RELATIVE_PATH]
       { --dependency ALIAS DEPENDENCY_PACKAGE }
       { --artifact PACKAGE_NAME PACKAGE_VERSION ARTIFACT_DIGEST ABSOLUTE_PATH }
```

There is exactly one source package record. The compiler enumerates and
validates all `.co` files under that root using v1's path, casing, symlink,
ordering, and duplicate rules. Dependency sources are not supplied or opened.

`--dependency` records describe only this package's direct alias mapping and
are sorted by alias. `--artifact` records supply its entire reachable dependency
closure, once per package, sorted by package name. Their names, versions, and
digests must match the actual files and transitive edge requirements. No extra
or unreachable packages are allowed. Multiple aliases may target one package
under the existing manifest rules; they still reference one artifact record.

`interface` requests perform complete source analysis and HIR/MIR/ABI
verification, producing metadata-only artifacts for the selected logical
target. `object` requests additionally emit native code and are initially
supported only for the configured native x86-64 environment. All dependency
artifacts must have the same kind and compatible target/configuration as the
request. Wasm object requests fail explicitly rather than selecting x86-64.

Optional `--entry` validates the existing eligible public static `Main` in the
exact logical file. Shuttle supplies it only for the selected root executable,
including during `check`; dependency executables are not entry candidates.
It does not produce a wrapper or alter artifact identity. Final linking
validates the entry again from metadata. A package need not declare an
executable to produce either artifact kind.

Success publishes the artifact and returns the receipt below. Compilation or
publication failure emits no receipt and preserves an existing output. A
receipt-transport failure after publication can leave the new artifact, but
Shuttle must stop and ignore incomplete output. It must never consume an old
artifact in place of the failed operation's result.

## Inspect one artifact

```text
clothc --shuttle-protocol 2
       --operation inspect
       --input ABSOLUTE_ARTIFACT_PATH
```

Inspection reads without modifying the artifact. It validates the envelope,
digest, schema, canonical identities, and internally checkable record/layout
invariants, then returns the same receipt as compilation. It needs neither
source files nor native tools. It does not claim that a dependency closure was
supplied, that external references resolve, or that the current native
environment is compatible. Compile and link perform those additional checks.

Shuttle uses the public receipt rather than opening the artifact's private
metadata. No compiler declaration, layout, or object structure becomes a Rust
API. Inspection is also a diagnostic boundary for explicitly supplied artifacts;
it does not add a user-facing cache or a new Shuttle subcommand in this stage.

## Link one executable

```text
clothc --shuttle-protocol 2
       --operation link
       --target x86_64
       --output ABSOLUTE_EXECUTABLE_PATH
       --root-package PACKAGE_NAME
       --entry SOURCE_RELATIVE_PATH
       { --artifact PACKAGE_NAME PACKAGE_VERSION ARTIFACT_DIGEST ABSOLUTE_PATH }
```

Records contain the root object artifact and its exact reachable closure, once
per package and sorted by package name. The compiler validates ownership,
compatibility, dependencies, signatures, and entry eligibility before native
linking. No source roots, aliases overridden by the caller, raw object files,
runtime paths, or arbitrary linker flags are accepted by this operation.

`clothc` owns wrapper generation, package-object extraction, deterministic
native link order, runtime selection, and native-tool invocation. It supplies
package objects in package-name order, followed by the single generated entry
object and compiler runtime in the native driver's required positions. It
passes each object and the runtime only once; private backing files use the
existing native-path-safe staging policy. Shuttle never invokes the platform
linker directly.

Success atomically replaces the executable and emits no stdout. Failure
preserves any previous executable and emits no success receipt.

## Artifact receipt, schema 1

Compile and inspect emit one JSON object followed by LF, and nothing else on
stdout. The object has these required fields, with no omitted fields:

```json
{
  "schema": 1,
  "artifact_format": 1,
  "artifact_id": "<64 lowercase hex digits>",
  "kind": "object",
  "package": {"name": "models", "version": "0.1.0"},
  "target": "x86_64",
  "compiler_id": "<64 lowercase hex digits>",
  "dependencies": [
    {
      "alias": "base",
      "package": {"name": "foundation", "version": "0.1.0"},
      "artifact_id": "<64 lowercase hex digits>"
    }
  ]
}
```

Examples are formatted for reading; writers emit compact single-line JSON in
the shown field order and dependencies sorted by alias. Receipts contain no
filesystem paths, semantic records, or native arguments. Protocol JSON is
separate from the artifact's canonical metadata encoding: the schema/format
integers shown above are ordinary JSON integers with values bounded by
`uint32`. There are no floating-point values.

Readers validate UTF-8, duplicate keys, required fields, field types, supported
schema/format/kind, identity spelling, and digest syntax. Unknown object fields
may be ignored for additive protocol evolution; unknown required capabilities
are not guessed. Unsupported schema versions, trailing JSON/log output,
truncated responses, or receipts larger than 16 MiB are protocol failures.
These reader rules also apply to capability responses, limited to 64 KiB, with
their own required fields and integer version lists. Writers validate response
size before publishing an artifact.
CRLF instead of LF is accepted on Windows. Successful inspect of an artifact
created by another compiler reports that artifact's producer, not the inspector.

Shuttle checks each compile receipt against the requested package, kind,
target, selected compiler identity, and supplied direct dependency digests.
It retains the returned artifact digest to construct consumer and link argv.
The compiler independently rereads and validates each input; trusting a
receipt does not bypass artifact verification.

## Diagnostics, validation, and output safety

Each operation rejects unknown or inapplicable options, duplicate singleton
options, missing values, and invalid identities/paths. Parsers may accept
option groups in arbitrary order, but Shuttle emits the canonical order above.
Paths, Unicode handling, logical path escaping, compiler discovery, and
human-readable diagnostic transport retain v1's rules. Standard input is not
a source of build inputs; prompts are forbidden.

- Status `0`: the requested operation succeeded with its defined stdout.
- Status `1`: source or semantic verification rejected the program, including
  a bad selected entry signature.
- Status `2`: invocation, artifact, protocol, compatibility, filesystem,
  unresolved link definition, output, or native-toolchain failure.
- Any other status, signal, or unexpected termination: compiler failure.

Shuttle drains stdout and stderr without deadlock, bounds receipt buffering,
and forwards stderr unchanged. It reports package/operation/compiler context
separately. It ignores partial stdout on failure and rejects success without a
valid required receipt. A missing executable/artifact after reported success
is a failed operation, never grounds to run stale output.

Artifact diagnostics identify the input path and owning package; dependency
errors identify the requesting package and expected/actual identity or digest.
Imported declaration locations use logical package-relative paths and do not
require source text. Validation must be deterministic regardless of argv order.

Shuttle creates output parents; `clothc` owns exclusive private staging and
atomic publication of each output. Outputs must not alias input artifacts,
sources, directories, or the selected tools/runtime, including through existing
symlinks or hard links. Generation or publication failure preserves old outputs
and unrelated neighbors; a later receipt-transport failure does not roll back
an already published artifact.
Shuttle stops the build on any failure; `run` executes only the output of the
successfully completed current link operation.

## Shuttle orchestration

1. Resolve and validate the complete local manifest graph before compilation.
2. Negotiate capabilities once and fix the selected compiler for the command.
3. Visit dependencies before consumers, choosing ascending package name among
   ready packages. Stage 23 starts with serial scheduling; graph-independent
   parallel execution is not needed to meet its exit criteria.
4. Compile each package once and reuse that exact artifact/digest for every
   consumer in the invocation, including diamond graphs. Never fall back to an
   old file following a compiler failure.
5. For `check`, use interface artifacts in an exclusively owned temporary
   build directory and remove them after completion/failure. Check remains
   toolchain-independent and leaves no persistent artifact or executable.
6. For `build`/`run`, place object artifacts under the root project's
   `target/x86_64/packages/PACKAGE_NAME.cpa`, then link the existing root
   executable output path. Different manifest packages have distinct names;
   entry filenames and dependency aliases are not artifact filenames.
7. Remove only temporary files/directories owned by the invocation. Published
   artifacts from successful earlier nodes may remain after a later failure,
   but the next command recompiles them; their presence is not freshness proof.

Automatic reuse across commands, dependency change detection, caching,
lockfiles, remote storage, build profiles, and user-supplied precompiled
dependencies remain deferred. Reusable artifacts mean that a completed package
can be consumed by multiple independent compiler invocations without source
access, not that Shuttle has acquired an incremental cache.

Concurrent Shuttle writers to the same root/target output directory must be
rejected using an exclusive OS-backed build lock held through compilation and
linking (and executable launch for `run`). Process exit releases the lock;
crashes must not require deleting stale lock state manually. Checks use their
private temporary directories and do not lock a persistent output directory.
Different output roots remain independent. This prevents two otherwise atomic
builds from exchanging package files or executing each other's result.

## Compatibility and verification

The existing CLI verbs, manifest semantics, imports, entry rules, executable
location, and direct/v1 compiler modes are preserved. Existing `check` on
wasm32 remains supported; native wasm and separate LLVM IR output are not added.

Tests must cover strict capability/receipt parsing, unknown options and invalid
argv, fake compiler failures, deterministic topological/argument ordering,
diamond reuse, inaccessible dependency sources after compilation, entry
selection, stale-output prevention, Unicode/space paths, output aliasing,
concurrent-writer rejection, and source/artifact/native failures. Cross-tool
tests compare separate and explicit v1 runs using shared fixtures. Compiler
development/sanitizer suites and Shuttle formatting, linting, MSRV, and test
suites remain coordinated Stage 23 exit requirements.
