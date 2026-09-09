# Shuttle verification

## Stage 45.5d Object-model exit audit

Completed with compiler 45.5d on Windows on 2026-09-09. The compiler-backed
toolchain and native suites each contain 38 cases. The permanent value-box
native case covers exact casts and type tests, overload preference, one-time
left-to-right evaluation, nullable lifting, heterogeneous object arrays, and
primitive, enum, and struct equality-compatible hashes. It compares serial and
parallel artifacts and executables, verifies all five expected warm reuses, and
links successfully after project and standard-library source removal.

All 51 ordinary Rust tests pass together with formatting, warning-denied
Clippy, and Rust 1.85 compatibility. The complete development and Clang
ASan/UBSan compiler matrices each pass 355 CTests, including both Shuttle
suites and the real self-hosted lexer parity test. No Shuttle protocol,
manifest, receipt, cache, or schema surface changed. Compatibility remains
artifact/compiler/runtime 8/7/11, schemas 2/1/1/1, and `cloth` v0.5.0.

## Stage 45.5c Object compatibility coordination

Completed with compiler 45.5c on Windows on 2026-09-09. Shuttle advertises and
requires artifact/compiler/runtime 8/7/11, selects the exact compiler-paired
`cloth` v0.5.0 distribution, and retains process, receipt, manifest, and
toolchain schemas 2/1/1/1. Compiler stubs reject the prior artifact version and
the complete standard-library graph checks on x86-64 and wasm32.

All 51 ordinary Rust tests pass, Clippy is warning-free, and the compiler-backed
Shuttle entries pass inside the 354-test CTest matrix. Object payload metadata
remains compiler-owned; Shuttle adds no manifest field, source interpretation,
or process-protocol surface. Sanitizer and final repository gates remain the
separately authorized 45.5d exit audit.

## Stage 44.3 self-hosted literal coordination

Completed with compiler 44.3 on Windows on 2026-09-08. Shuttle checks the real
`F:\Cloth` source graph on x86-64 and wasm32. Development and ASan/UBSan native
runs execute valid and malformed numeric, text, Unicode, UTF-8, termination,
recovery, exact-buffer, and tracked-source cases and print
`lexer literals ok`.

A warm development run reuses the exact `cloth` v0.4.0 and `clothc` package
artifacts and reproduces the output. The complete 354-test development compiler
matrix passes, including existing affected-invalidation and completed-output
preservation paths. Shuttle remains opaque to source bytes, token kinds,
diagnostic categories, scan passes, and test expectations.

No Shuttle production code, process protocol, receipt, manifest, toolchain
schema, or cache meaning changed. Compatibility remains
artifact/compiler/runtime 7/6/10 and schemas 2/1/1/1. Differential lexer parity
and the exit audit remain separately authorized compiler 44.4 work.

## Stage 44.2 self-hosted scanner-foundation coordination

Completed with compiler 44.2 on Windows on 2026-09-08. Shuttle checks the real
`F:\Cloth` source graph on x86-64 and wasm32. Development and ASan/UBSan native
runs execute the empty, canonical-token, comment, recovery, exact-buffer, and
source-coordinate corpus and print `lexer foundation ok`.

A warm development run reuses the exact `cloth` v0.4.0 and `clothc` package
artifacts and reproduces the output. The complete 354-test development compiler
matrix passes, including existing affected-invalidation and completed-output
preservation paths. Shuttle remains opaque to source bytes, token kinds,
diagnostic categories, scan passes, and test expectations.

No Shuttle production code, process protocol, receipt, manifest, toolchain
schema, or cache meaning changed. Compatibility remains
artifact/compiler/runtime 7/6/10 and schemas 2/1/1/1. At this checkpoint,
literal completion remained separately authorized compiler 44.3 work.

## Stage 44.1 self-hosted lexer coordination contract

Approved with compiler 44.1 on Windows on 2026-09-08. Shuttle remains opaque
to the bootstrap's source bytes, tokens, diagnostics, scanners, and canonical
parity records. The new contract retains exact compiler and `cloth` v0.4.0
selection, ordinary source hashing, deterministic project builds, and completed
output preservation without adding a manifest key, process field, cache input,
or production code.

This checkpoint changes coordinated roadmap, ledger, proposal, and bootstrap
maintainer documentation only. Compatibility remains artifact/compiler/runtime
7/6/10 and schemas 2/1/1/1. Scanner builds and execution remain separately
authorized 44.2 work.

The formatted bootstrap source passes `shuttle check` and native `shuttle run`.
The current LF source reports `962`, `105`, `10`, `2`, `src/Main.co`, and
`true`; the warm run reuses both `cloth` and `clothc` package artifacts and
reproduces the output. No Shuttle production code changed.

## Stage 43.4 portable file-byte exit coordination

Completed with compiler 43.4 on Windows on 2026-09-08. Development and
ASan/UBSan compiler configurations each pass all 354 CTests, including all 38
compiler-backed and 37 native Shuttle cases. The file-byte fixture confirms
that application paths and contents remain runtime-only inputs: warm reuse,
affected invalidation, failed-output preservation, source-free linking,
relocation, serial/parallel determinism, and both targets remain unchanged.

The real `F:\Cloth` bootstrap checks on x86-64 and wasm32 and runs its
file-backed source/token-buffer path with both compilers. Warm builds reuse the
exact `cloth` v0.4.0 and `clothc` artifacts. All 51 ordinary Rust tests, Rust
1.85 and all-target checks, warning-denied Clippy, Rust/C++ formatting,
TypeScript and editor tests, documentation links, and repository gates pass.
All 367 local Markdown targets across 118 files are valid. Compatibility
remains 7/6/10 and 2/1/1/1. Shuttle production code and schemas are unchanged,
and Stage 43 coordination is complete.

## Stage 43.3 bootstrap source integration

Completed with compiler 43.3 on Windows on 2026-09-08. The real `F:\Cloth`
project checks through Shuttle on x86-64 and wasm32 and runs its file-backed
`SourceFile` and length-sized token buffer natively with development and
ASan/UBSan compilers. The LF checkpoint records 992 source bytes, first and
final bytes 105 and 10, two tokens, `src/Main.co`, and a successful EOF check;
the reported length follows the checkout's exact bytes.

Warm x86-64 builds reuse the exact `cloth` v0.4.0 and `clothc` artifacts.
Direct protocol-v2 compilation and linking confirm source-free execution. An
isolated invalid source edit recompiles only `clothc`, produces its stable
source diagnostic, does not launch the stale executable, and preserves both
package artifacts and the executable. Shuttle remains opaque to paths and file
contents and adds no production or schema change. Compatibility remains
7/6/10 and 2/1/1/1.

## Stage 43.2 portable file-foundation coordination

Completed with compiler 43.2 on Windows on 2026-09-08. Shuttle selects the
exact compiler-paired `cloth` v0.4.0 distribution and carries runtime ABI 10
requirements through both targets without a production, process, receipt,
manifest, or toolchain-schema change. The new native fixture proves that file
contents are runtime inputs: changing them reuses all build outputs while the
program observes the new bytes, and removing the file reports the stable
`IoError` without replacing artifacts or the executable.

All 51 ordinary Rust tests pass, along with 38 compiler-backed and 37 native
integration tests in both development and ASan/UBSan compiler matrices. Rust
1.85 checking, all-target checking, warning-denied Clippy, and Rust formatting
also pass. Active artifact/compiler/runtime compatibility is 7/6/10, schemas
remain 2/1/1/1, and `cloth` is v0.4.0. Bootstrap coordination remains planned
43.3 work.

## Stage 43.1 portable file-byte coordination contract

Approved and recorded with compiler 43.1 on Windows on 2026-09-08. This
documentation-only checkpoint leaves Shuttle production code, protocol,
schemas, manifests, caches, and execution behavior unchanged. Compatibility
remains 7/6/9 and 2/1/1/1 with the exact compiler-paired `cloth` v0.3.0.

Shuttle remains opaque to `File.ReadBytes`, paths, contents, runtime statuses,
and bootstrap source representation. `check` and `build` do not perform
application file reads. A child launched by `run` continues to inherit the
invocation working directory and host permissions. Runtime ABI 10, `cloth`
v0.4.0, file-backed fixtures, and bootstrap integration require separate 43.2
and 43.3 authorization.

## Stage 42.4 runtime-sized array exit coordination

Completed with compiler 42.4 on Windows on 2026-09-08. Development and
ASan/UBSan compiler configurations each pass all 350 CTests, including every
38 compiler-backed and 36 native Shuttle case. The dedicated fixture produces
byte-identical x86-64 and wasm32 package artifacts from distinct roots and job
schedules; its relocated native executables are also byte-identical.

Whole-project, separate-package, and source-free execution agree. Exact reuse,
affected invalidation, failed-check and failed-run output preservation, and the
previous runnable executable all pass. The real `F:\Cloth` project checks on
both targets, emits the exact token-buffer output, reuses `cloth` and `clothc`,
and preserves its artifact hashes on a warm build.

All 51 ordinary Rust tests, Rust 1.85 checking, warning-denied Clippy, and Rust
formatting pass. Shuttle remains opaque to runtime-sized array syntax, default
values, layouts, and failures. Production code, artifact/compiler/runtime
compatibility 7/6/9, schemas 2/1/1/1, and `cloth` v0.3.0 are unchanged.

## Stage 42.3 runtime-sized array integration

Completed with compiler 42.3 on Windows on 2026-09-08. Runtime-sized arrays
pass serial and parallel package checks on x86-64 and wasm32 with deterministic
artifacts. Exact reuse, affected dependency invalidation, and failed-check
artifact preservation remain observable through normal Shuttle progress and
publication behavior.

Separate-package, whole-project, and source-free native execution agree for a
nullable struct array whose element type comes from a dependency artifact. A
failed native rebuild preserves both completed artifacts and the runnable
executable. The real `F:\Cloth` project checks on both targets and its
`Token?[]` buffer runs through Shuttle with the expected EOF-terminated output.

Shuttle production code, protocol and schemas remain unchanged. Compatibility
is still 7/6/9 and 2/1/1/1 with the compiler-paired `cloth` v0.3.0 package.

## Stage 42.2 runtime-sized array frontend coordination

Completed with compiler 42.2 on Windows on 2026-09-08. Development and
ASan/UBSan compiler configurations each pass all 338 CTests, including the
unchanged 37 compiler-backed and 35 native Shuttle cases. A dedicated compiler
integration test proves that x86-64 interface/object and wasm32 interface
protocol-v2 compile requests reject runtime-sized construction before staging
or publishing an artifact.

All 51 ordinary Rust tests, Rust 1.85 checking, warning-denied Clippy, and Rust
formatting pass. No ignored native-wrapper test is counted as ordinary Rust
coverage.

Shuttle remains opaque to `T[:length]`, element defaults, length evaluation,
and MIR. No production Rust, protocol, schema, manifest, cache, or publication
logic changes. Compatibility remains 7/6/9 and 2/1/1/1, with the exact
compiler-paired `cloth` v0.3.0 distribution. Native/toolchain integration
remains gated on separately authorized compiler 42.3.

## Stage 42.1 runtime-sized fixed-array coordination

Approved and recorded with compiler 42.1 on 2026-09-07. This
documentation-only checkpoint makes Shuttle explicitly opaque to
`T[:length]`, element defaults, allocation failures, layouts, and bootstrap
token contents. Compatibility remains artifact/compiler/runtime 7/6/9,
schemas remain 2/1/1/1, and the compiler-paired `cloth` package remains
v0.3.0.

No Shuttle production code, protocol, schema, manifest, cache, or publication
behavior changes. The completed Stage 41 baseline remains 51 ordinary Rust
tests, 37 compiler-backed cases, and 35 native cases. Coordinated implementation
and bootstrap verification begin only after separate compiler 42.2
authorization; compiler 42.2 has since completed.

## Stage 41.4 uniform-nullability exit coordination

Completed with compiler checkpoint 41.4 on Windows on 2026-09-07. Development
and ASan/UBSan compiler configurations each pass all 335 CTests, including all
37 compiler-backed Shuttle cases and 35 native cases. All 51 ordinary Rust
tests, Rust 1.85 checking, warning-denied Clippy, Rust formatting,
documentation links, and repository whitespace checks pass.

The nullable fixture retains deterministic x86-64/wasm32 artifacts under
relocation and scheduling changes, exact reuse, affected-only invalidation,
unchanged independent packages, and equivalent whole, separate, source-free,
and native execution. A malformed repeated-nullability edit now explicitly
proves that failed checks preserve every completed artifact and failed native
runs preserve both artifacts and the previous executable without running stale
code. Compatibility remains 7/6/9, schemas remain 2/1/1/1, and `cloth` remains
v0.3.0.

## Stage 41.3 uniform-nullability toolchain coordination

Completed with compiler checkpoint 41.3 on Windows on 2026-09-07. Development
and ASan/UBSan compiler configurations each pass all 329 CTests, including 37
compiler-backed Shuttle cases and 35 native cases. All 51 ordinary Rust tests,
warning-denied Clippy, and Rust formatting pass.

Shuttle advertises and requires artifact format 7 through the existing
capability and receipt schemas and rejects stale format 6 data. Nullable
fixtures verify deterministic x86-64 and wasm32 artifacts, exact warm reuse,
affected dependency invalidation, unchanged independent packages, and
serial/parallel equivalence. Whole-project, separate-package, and source-free
native execution produce the same output. Compiler ABI 6 and runtime ABI 9
remain opaque compiler-owned compatibility data; schemas stay 2/1/1/1 and
`cloth` stays v0.3.0.

## Stage 41.2 uniform-nullability frontend coordination

Completed with compiler checkpoint 41.2 on Windows on 2026-09-07. Development
and ASan/UBSan compiler configurations each pass all 312 CTests, including the
compiler-backed Shuttle toolchain suite and the native Shuttle suite. Nullable
values and safe operations remain compiler-owned verified IR; the explicit
native/artifact gate prevents a partial package feature from being published.

Shuttle production code, process protocol 2, receipt/manifest/toolchain schemas
1/1/1, artifact/compiler/runtime compatibility 6/5/8, and the paired `cloth`
v0.3.0 distribution remain unchanged.

## Stage 41.1 uniform-nullability contract coordination

Approved with compiler checkpoint 41.1 on Windows on 2026-09-07. Shuttle owns
only opaque compatibility transport, exact compiler/library selection, cache
and dependency behavior, native linking, and atomic output publication. It does
not parse nullable syntax, inspect presence tags or payloads, implement flow
analysis, or determine safe-operation behavior.

Checkpoint 41.3 will advance artifact/compiler/runtime compatibility from
6/5/8 to 7/6/9 through existing capability, receipt, compiler-identity,
invalidation, and link fields. Process protocol 2 and
receipt/manifest/toolchain-metadata schemas 1/1/1 remain unchanged; `cloth`
remains v0.3.0. This checkpoint changes documentation only and adds no Shuttle
production behavior.

## Stage 40.4 string-slicing exit audit coordination

Completed with compiler checkpoint 40.4 on Windows on 2026-09-07. Development
and ASan/UBSan compiler configurations each pass all 309 CTests, including all
36 compiler-backed Shuttle toolchain cases and 34 native cases. All 51 ordinary
Rust tests, Rust 1.85 all-target checking, warning-denied Clippy, Rust
formatting, both 19-test editor runs, documentation links, and repository gates
pass.

Whole, separate, and source-free slicing retain exact compiler/library pairing,
warm reuse, affected invalidation, relocated serial/parallel determinism,
stale-run prevention, and completed-output preservation. Shuttle remains opaque
to slicing syntax, Unicode-scalar bounds, string contents, allocation, and GC
roots. Compatibility remains artifact/compiler/runtime 6/5/8 with `cloth`
v0.3.0 and unchanged process/receipt/manifest/toolchain schemas 2/1/1/1.

## Stage 40.3 string-slicing runtime coordination

Completed with compiler checkpoint 40.3 on Windows on 2026-09-07. Development
and ASan/UBSan compiler configurations each pass all 308 CTests, including 36
compiler-backed Shuttle toolchain cases and 34 native cases. All 51 ordinary
Rust tests, warning-denied Clippy, and the Rust 1.85 MSRV check also pass.

The package fixture executes one Unicode-scalar slice through whole-project,
separate-package, and source-free native paths. Existing tests verify exact
compiler/library selection, reuse, dependency invalidation, link behavior, and
failure-preserving atomic publication. Shuttle remains opaque to source syntax,
string contents, scalar bounds, allocation, and GC roots.

Artifact/compiler/runtime compatibility is 6/5/8 with `cloth` v0.3.0.
Process, receipt, manifest, and toolchain-metadata schemas remain 2/1/1/1; no
Shuttle production or schema change was required.

## Stage 40.2 string-slicing compiler coordination

Completed with compiler checkpoint 40.2 on Windows on 2026-09-06. Clean
development and ASan/UBSan compiler configurations each pass all 296 CTests,
including the existing compiler-backed Shuttle toolchain and native matrices.
Shuttle code and schemas are unchanged: it remains opaque to slice syntax,
bounds, HIR, MIR, and string contents.

Compiler frontend validation now retains dedicated verified slicing IR, while
LLVM emission rejects the operation behind an explicit Stage 40.3 gate. No
partial native or package artifact can therefore be published. Compatibility
remains artifact/compiler/runtime 6/5/7, schemas remain 2/1/1/1, and `cloth`
remains v0.3.0.

## Stage 40.1 Unicode string-slicing contract coordination

Approved with compiler checkpoint 40.1 on Windows on 2026-09-06. Shuttle owns
only opaque compatibility transport, exact compiler/library selection, cache
and dependency behavior, native linking, and atomic output publication. It does
not parse slice syntax, inspect scalar bounds or string contents, or own result
allocation.

Checkpoint 40.3 will advance runtime ABI 7 to 8 through existing capability,
receipt, compiler-identity, invalidation, and link fields. Artifact/compiler
compatibility remains 6/5; protocol and schemas remain 2/1/1/1; `cloth` remains
v0.3.0. This checkpoint changes documentation only and adds no Shuttle
production behavior.

## Stage 39.4 Unicode string-traversal exit audit

Completed with the compiler exit audit on Windows on 2026-09-06. Development
and ASan/UBSan compiler matrices each pass all 296 CTests, including all 36
compiler-backed Shuttle toolchain cases and 34 native cases. All 51 ordinary
Rust tests, warning-denied Clippy, Rust formatting, the Rust 1.85 MSRV check,
both 17-test editor runs, documentation links, and repository gates pass.

Whole, separate, and source-free traversal fixtures remain equivalent on both
targets. Exact v0.3.0 standard-library pairing, warm reuse, affected
invalidation, relocated serial/parallel determinism, stale-run prevention, and
completed-output preservation all remain intact. Shuttle remains opaque to
source traversal and string contents. Compatibility stays
artifact/compiler/runtime **6/5/7** with unchanged
process/receipt/manifest/toolchain schemas **2/1/1/1**.

## Stage 39.3 Unicode string-traversal coordination

Completed with compiler checkpoint 39.3 on Windows on 2026-09-06. Development
and ASan/UBSan compiler matrices each pass all 294 CTests, including 36
compiler-backed Shuttle toolchain cases and 34 native cases. All 51 ordinary
Rust tests, warning-denied Clippy, Rust formatting, and the Rust 1.85 MSRV check
also pass.

Compiler-backed fixtures execute Unicode scalar indexing and linear string
iteration through whole, separate, and source-free dependency builds. Exact
v0.3.0 standard-library pairing, warm reuse, affected invalidation, relocated
artifacts, failed-output preservation, and stale-run prevention remain intact.
Shuttle does not inspect source traversal or string contents. Compatibility is
artifact/compiler/runtime **6/5/7** with unchanged
process/receipt/manifest/toolchain schemas **2/1/1/1**.

## Stage 39.2 Unicode-scalar artifact coordination

Completed with compiler checkpoint 39.2 on Windows on 2026-09-06. Shuttle now
requires artifact format 6 through the existing capability and receipt fields;
protocol 2 and all schemas remain unchanged. Its compiler stubs reject the old
format, while compiler-backed x86-64 and wasm32 fixtures carry a non-BMP
character constant through whole, separate, and source-free builds.

All 51 ordinary Rust tests, 36 compiler-backed toolchain cases, 34 native cases,
warning-denied Clippy, and Rust formatting pass with both development and
ASan/UBSan compilers. Exact warm reuse, affected invalidation, relocated
serial/parallel bytes, source-free execution, failed-output preservation, and
compiler-paired `cloth` v0.3.0 selection remain intact. Compatibility is now
artifact/compiler/runtime 6/5/6. Documentation links and repository whitespace
checks pass; traversal remains compiler work for 39.3.

## Stage 39.1 Unicode string-traversal contract coordination

Approved with compiler checkpoint 39.1 on Windows on 2026-09-06. Shuttle owns
only opaque transport, exact compiler/library selection, cache and dependency
behavior, linking, and atomic output publication. It does not decode source,
inspect character constants, expose string storage, or implement traversal.

This checkpoint changes documentation only. Active compatibility remains
artifact/compiler/runtime 5/5/6 and process/receipt/manifest/toolchain schemas
2/1/1/1 with `cloth` v0.3.0. Artifact format 6 and runtime ABI 7 remain planned
for separately authorized checkpoints using existing protocol/schema fields.

## Stage 38.4 text-input and primitive-parsing exit audit coordination

Completed with compiler checkpoint 38.4 on Windows on 2026-09-06. Development
and ASan/UBSan configurations each pass all 281 compiler CTests, including all
36 compiler-backed Shuttle toolchain cases and 34 native cases. All 51 ordinary
Rust tests, Rust 1.85 all-target checking, warning-denied Clippy, Rust and C++
formatting, 14 editor tests, 338 local Markdown targets across 111 files, and
repository whitespace checks pass.

The coordinated matrix proves redirected stdin inheritance, successful and
failing primitive parsing, input-independent artifact bytes, exact warm reuse,
ParseError-specific standard-library invalidation, source-free execution,
relocated serial/parallel determinism, failed-output preservation, stale-run
prevention, and direct/whole/separate equivalence. Shuttle continues to treat
compiler artifacts opaquely and required no production-code change.

Compatibility remains artifact/compiler/runtime 5/5/6 and
process/receipt/manifest/toolchain-metadata schemas 2/1/1/1 with `cloth`
v0.3.0. Stage 38 coordination is complete without a format or schema change.

## Stage 38.3 primitive-parsing integration coordination

Completed with compiler checkpoint 38.3 on Windows on 2026-09-06. Development
and ASan/UBSan configurations each pass all 276 compiler CTests, including all
36 compiler-backed Shuttle toolchain cases and 34 native cases. All 51 ordinary
Rust tests, the Rust 1.85 all-target check, warning-denied Clippy, Rust and C++
formatting, 14 editor tests, documentation links, and repository whitespace
gates pass.

The native Shuttle matrix inherits redirected stdin unchanged, parses two
successful values, reports an exact source-defined `ParseError` for invalid
input, preserves program streams and status, reuses all packages on the second
run, and proves that input cannot change artifact bytes. Compiler-backed
toolchain coverage also carries primitive parsing through the exact paired
`cloth` v0.3.0 source-free package on both targets. Shuttle production code and
schemas remain unchanged.

Compatibility remains artifact/compiler/runtime 5/5/6 and
process/receipt/manifest/toolchain-metadata schemas 2/1/1/1.

## Stage 38.2 text-input and parsing foundation coordination

Completed with compiler checkpoint 38.2 on Windows on 2026-09-06. Development
and ASan/UBSan configurations each pass all 276 compiler CTests, including all
36 compiler-backed Shuttle toolchain cases and 33 native cases. All 51 ordinary
Rust tests, warning-denied Clippy, and Rust formatting pass.
The Rust 1.85 all-target check also remains green.

The selected compiler now advertises and requires `cloth` v0.3.0, while its
opaque artifacts carry runtime ABI 6. Capability validation, toolchain
metadata, exact paired-library selection, source-free linking, and native
execution accept that coordinated identity. Existing cache, invalidation,
receipt, and output-preservation behavior remains unchanged. Shuttle does not
read or encode standard input and has no production change in this checkpoint.

Compatibility is artifact/compiler/runtime 5/5/6. Process protocol 2 and
receipt/manifest/toolchain-metadata schemas 1/1/1 are unchanged. Input
inheritance and public primitive parsing integration remain scheduled for 38.3.

## Stage 38.1 portable text-input contract

Approved with compiler checkpoint 38.1 on Windows on 2026-09-06. Shuttle owns
only inheritance of standard input by the launched application. It does not
read, decode, buffer, log, hash, or place input in compiler requests, artifacts,
or cache keys. `check` and `build` do not consume it; existing `run --`, stream,
status, failure-preservation, and stale-run contracts remain unchanged.

This checkpoint changes coordinated roadmap, ledger, proposal, and verification
documentation only. Shuttle production behavior, tests, package version, and
schemas are unchanged. Compatibility remains artifact/compiler/runtime 5/5/5
and process/receipt/manifest/toolchain schemas 2/1/1/1 with `cloth` v0.2.0.
Runtime ABI 6 and `cloth` v0.3.0 wait for separately authorized checkpoint
38.2. Documentation links and repository whitespace gates pass.

## Stage 37.4 portable program-argument exit audit

Completed with the compiler audit on Windows on 2026-09-06. Development and
ASan/UBSan configurations each pass all 269 compiler CTests, including all 36
compiler-backed Shuttle toolchain cases and 33 native cases. All 51 ordinary
Rust tests, a Rust 1.85 all-target check, warning-denied Clippy, Rust and C++
formatting, both 12-test editor runs, documentation links, and repository gates
pass.

The final Shuttle matrix preserves exact application values, streams, statuses,
typed errors, and strict invalid-host-text failure. Arguments remain outside
compiler requests and artifact identity: changed values completely reuse the
package graph and executable. A failed argument-taking rebuild preserves every
completed artifact and executable without launching stale output. Direct,
whole-project, separate-package, source-free, and `shuttle run --` execution
agree.

Compatibility remains artifact/compiler/runtime 5/5/5 and process/receipt/
manifest/toolchain-metadata 2/1/1/1. **Stage 37 is complete.**

## Stage 37.3 exact program-argument forwarding

Completed on Windows on 2026-09-06. Development and ASan/UBSan configurations
each pass all 263 compiler CTests, including all 36 compiler-backed Shuttle
toolchain cases and 33 native cases. All 51 ordinary Rust tests pass with Rust
1.89 while preserving the Rust 1.85 MSRV. Warning-denied Clippy and Rust
formatting pass.

CLI and child-process tests require the explicit `run --` boundary and reject
program values for `check`, `build`, or undelimited `run`. Native tests preserve
zero, empty, whitespace-only, option-looking, and Unicode values; statuses and
streams; warm reuse with changed arguments; malformed host Unicode; and direct,
whole-project, separate-package, and source-free equivalence. Program arguments
remain outside compiler requests, artifact identity, and cache keys.

Compatibility remains artifact/compiler/runtime 5/5/5 and process/receipt/
manifest/toolchain-metadata 2/1/1/1. Stage 37.4 is the remaining exit audit.

## Stage 37.2 compiler and runtime coordination

Completed with compiler checkpoint 37.2 on Windows on 2026-09-06. Development
and ASan/UBSan configurations each pass all 263 compiler CTests, including the
compiler-backed Shuttle toolchain and native suites. Direct and source-free
artifact executables accept the new managed `string[]` entry parameter while
the existing zero-parameter entry path remains valid.

The compiler/runtime boundary now uses artifact/compiler/runtime compatibility
5/5/5. Shuttle production code, process protocol 2, receipt schema 1, manifest
schema 1, toolchain-metadata schema 1, and cache semantics remain unchanged.
Exact `shuttle run --` forwarding remains the separately authorized 37.3 work.

## Stage 37.1 portable program-argument contract

Approved with compiler checkpoint 37.1 on Windows on 2026-09-06. The contract
reserves `shuttle run -- [ARGUMENT]...` for exact host-native forwarding. Only
`run` accepts application values; Shuttle does not decode, parse, normalize,
join, or reconstruct them. Compiler and runtime code own conversion into a
managed `string[]` and strict invalid-Unicode handling.

This checkpoint changes coordinated roadmap, ledger, proposal, and verification
documentation only. Shuttle production behavior, tests, package version,
artifact/compiler/runtime compatibility 5/5/4, and process/receipt/manifest/
toolchain schemas 2/1/1/1 remain unchanged. Runtime ABI 5 and compiler behavior
wait for a separately authorized 37.2 checkpoint. Documentation links and
repository whitespace gates pass.

## Stage 36.4 standard-library prelude exit audit

Completed with both compiler configurations on Windows on 2026-09-06. Each
255-test CTest matrix passes all 36 compiler-backed toolchain cases and 32
native cases. All 49 ordinary Rust tests, Rust 1.85, warning-denied Clippy,
formatting, both 12-test editor runs, documentation links, and repository gates
also pass.

The coordinated matrix verifies source-free recursive prelude lookup, standard-
library bootstrap without a self-dependency, x86-64 and wasm32 artifacts,
relocated one-job/four-job determinism, native linking and execution, exact
invalidation, warm reuse, failed-output preservation, and stale-run prevention.
Shuttle remains opaque to `cloth.lang` declarations. Compatibility remains
artifact/compiler/runtime 5/5/4 and process/receipt/manifest/toolchain schemas
2/1/1/1. **Stage 36 coordination is complete.**

## Stage 36.3 initial standard-library API

Completed with compiler checkpoint 36.3 on Windows on 2026-09-05 and amended
2026-09-06 for the recursive prelude. The paired distribution remains `cloth`
v0.2.0. Existing strict metadata selection,
capability validation, artifact dependency records, cache inputs,
invalidation, and link closure carry that exact version and digest without a
schema or version-solving change.

A manifest without a standard-library dependency uses unqualified
`ArgumentError` and `StateError` with canonical `cloth.lang.errors` identities
from source-free `cloth` artifacts on x86-64 and wasm32. A nested synthetic
prelude package proves recursive lookup and deterministic artifacts. The native
path links independent package artifacts, prints both messages, and completely
reuses both artifacts on a warm run. Exact library source edits still
invalidate the library and consumer on both targets.

All 36 compiler-backed toolchain cases, 32 native cases, and 49 ordinary Rust
tests pass, together with Rust formatting, warning-denied Clippy, documentation
targets, and repository checks. Shuttle remains unaware of library
declarations. Compatibility stays 5/5/4 and 2/1/1/1. The completed coordinated
Stage 36.4 audit is recorded above.

## Stage 36.2 standard-library prelude resolution

Completed with compiler checkpoint 36.2 on Windows on 2026-09-05. The public
toolchain suite copies the selected compiler and paired standard-library
distribution, adds a synthetic public type directly beneath `src/lang/`, and
checks an application that uses the type without an import. The library builds
without a self-dependency and the consumer receives only its verified package
artifact.

The fixture passes on x86-64 and wasm32 with one and four jobs; both the
`cloth.cpa` and consumer artifacts are byte-identical across relocated
fixtures. All 36 compiler-backed toolchain cases and 49 ordinary Rust tests
pass, along with warning-denied Clippy and Rust formatting. No Shuttle
production path, manifest or receipt schema, protocol, package version, or
production standard-library source changed.

## Stage 36.1 standard-library prelude contract

Approved and recorded on Windows on 2026-09-05. The compiler owns the
nonrecursive `cloth.lang` fallback and derives it only from verified whole-
project or imported declarations. Shuttle retains its Stage 35 responsibility
for selecting and injecting one exact `cloth` package and remains unaware of
the library's namespaces and APIs.

This checkpoint changes coordinated roadmap, ledger, and contract documentation
only. Artifact/compiler/runtime compatibility remains 5/5/4 and process/
receipt/manifest/toolchain schemas remain 2/1/1/1. It adds no Shuttle
production behavior, public library declaration, package-version change, or
user-facing language claim. Documentation and repository gates passed. The
separately authorized compiler checkpoint 36.2 is recorded above.

## Stage 35.4 standard-library foundation exit audit

Verified on Windows on 2026-09-05 with development and ASan/UBSan compilers.
Each compiler passes all 35 public toolchain cases and 32 native cases inside
its 255-test repository matrix. All 49 ordinary Rust tests, Rust 1.85,
formatting, warning-denied Clippy, editor checks, documentation links, and
repository gates pass.

Strict selection tests reject missing, malformed, duplicate-field, unknown-
field, unsupported-schema, mismatched, non-normal, and missing-manifest
metadata. Wrong package names or versions and distributions with an executable
or dependencies also fail before package compilation. Case-only `cloth`
aliases, replacement packages, incompatible capabilities, artifacts, targets,
receipts, compiler identities, runtime identities, and link closures remain
rejected by their owning boundary.

Exact standard-library edits rebuild `cloth` and its consumer on x86-64 and
wasm32, followed by complete warm reuse. Corrupt `cloth` candidates are repaired
without rebuilding byte-identical consumers. Failed library compilation keeps
the completed library artifact, consumer artifact, and executable and does not
run stale output. Existing relocated one-job/four-job tests include `cloth` and
prove byte-identical artifacts on both targets and byte-identical native
executables. Compatibility remains 5/5/4 and protocol/receipt/manifest/
toolchain schemas remain 2/1/1/1. **Stage 35 coordination is complete.**

## Stage 35.3 standard-library integration

Verified on Windows on 2026-09-05 with development and ASan/UBSan compilers.
Shuttle now selects one exact `cloth` v0.1.0 distribution from strict metadata
beside the chosen compiler, injects it directly into every ordinary package,
and compiles the standard library without a self-dependency. User `cloth`
aliases and replacement packages are rejected before application compilation.

The real-toolchain matrix checks implicit `cloth.math::Math` imports and
artifact receipts on x86-64 and wasm32, native linking and output, warm reuse,
and standalone standard-library checking. Existing graph, scheduling, cache,
invalidation, relocation, source-free, and failure tests include the implicit
package.

Both compiler configurations pass all 255 CTests, including 34 public
toolchain cases and 31 native cases. All 47 ordinary Rust tests, Rust 1.85,
formatting, warning-denied Clippy, and C++ formatting pass. Artifact/compiler/
runtime compatibility remains 5/5/4, process/receipt/manifest schemas remain
2/1/1, and toolchain metadata begins at schema 1.

## Stage 35.2 compiler/library bootstrap coordination

Recorded on 2026-09-05. Shuttle production behavior remains unchanged. The
compiler-owned integration fixture passes the standard library through the
existing protocol-2 package boundary, producing and consuming interface
artifacts on x86-64 and wasm32 and independent object artifacts for native
linking. A direct Shuttle check also accepts the executable-free `cloth` v0.1.0
manifest without a self-dependency.

Both development and ASan/UBSan compiler configurations pass all 255 CTests,
including 32 public Shuttle toolchain cases and 30 native cases. Compatibility
remains artifact/compiler/runtime 5/5/4 with protocol 2, receipt schema 1, and
manifest schema 1. All 43 ordinary Rust tests, formatting, and warning-denied
Clippy pass. Distribution selection and automatic graph injection remain
separately authorized 35.3 work.

## Stage 35.1 standard library coordination

Recorded on 2026-09-05. The coordinated contract reserves `cloth` for the
compiler-paired standard-library package and assigns later Shuttle work to
automatic dependency injection, exact selection, reuse, invalidation, and
diagnostics. Library types remain explicitly imported and Shuttle remains
opaque to Cloth source and compiler artifact internals.

This checkpoint changes documentation only. Artifact/compiler/runtime
compatibility remains 5/5/4, and process protocol 2, receipt schema 1, and
manifest schema 1 remain unchanged. Compiler/library bootstrap belongs to 35.2
and Shuttle production integration belongs to separately authorized 35.3.

## Coordinated compiler 34.4 typed error exit audit

Verified on Windows on 2026-09-05 with development and ASan/UBSan compilers.
Each compiler passes all **32 public compiler-protocol/toolchain cases** and
**30 native Shuttle cases** inside its **249-test** matrix. All **43 ordinary
Rust tests**, the Rust **1.85.0** minimum check, Rust formatting,
warning-denied Clippy, editor checks, documentation links, and repository gates
pass.

The existing four-package fixture continues to prove whole-project, separate-
package, and source-free equivalence plus relocated one-job/four-job x86-64 and
wasm32 artifact determinism. The exit audit adds a typed-error public-contract
edit: only the defining package and consumer rebuild, independent packages are
reused, and a subsequent invalid throws declaration preserves every completed
artifact and executable without running stale output.

Artifact format **5**, compiler ABI **5**, and runtime ABI **4** remain active.
Shuttle transports compiler-owned error metadata and object code opaquely;
process protocol **2**, receipt schema **1**, manifest schema **1**, cache
policy, and scheduling remain unchanged. **Stage 34 coordination is complete.**

## Compiler 34.3 typed error integration

Verified on Windows on 2026-09-05 with development and ASan/UBSan compilers.
Each compiler passes all **32 public compiler-protocol/toolchain and 29 native
Shuttle cases** inside its **241-test** matrix. All **43 ordinary Rust tests**,
the Rust **1.85.0** minimum check, Rust formatting, and warning-denied Clippy
pass.

The typed-error fixture defines a public error in one package, exposes a
throwing function from another, and consumes it from the application. Whole-
project, separate-package, and source-free execution agree. Reversed dependency
declarations and one-job/four-job schedules produce identical x86-64 and wasm32
artifacts. Native execution verifies both the success value and stable
`DivisionByZero` reporting through a throwing entry point.

Artifact format **5**, compiler ABI **5**, and runtime ABI **4** are active.
Shuttle still treats error metadata and object code as opaque compiler output;
process protocol **2**, receipt schema **1**, manifest schema **1**, cache
policy, and scheduling are unchanged. The separately authorized compiler 34.4
audit is recorded above.

## Compiler 34.1 typed error contract

Recorded on 2026-09-04. Error declarations, throw expressions, throws-set
analysis, automatic propagation, constructor failure, runtime reporting, and
the result/error calling convention remain compiler-owned. Shuttle will treat
the new interface metadata and object code as opaque artifacts.

The approved implementation target is artifact format **5**, compiler ABI
**5**, and runtime ABI **4** with unchanged process protocol **2**, receipt
schema **1**, and manifest schema **1**. Compiler 34.1 changes documentation
only: active compatibility remains format 4, compiler ABI 4, and runtime ABI 3,
and no Shuttle source, fixture, schema, cache, scheduler, or launcher changes.
All 101 local Markdown target checks and repository whitespace gates pass.

Compiler 34.2, 34.3, and the separately authorized 34.4 audit are recorded
above.

## Coordinated compiler 33.4 numeric literal notation exit audit

Verified on Windows on 2026-09-04 with development and ASan/UBSan compilers.
Each compiler passes all **31 public compiler-protocol/toolchain and 28 native
Shuttle cases** inside its **232-test** matrix. All **43 ordinary Rust tests**,
the Rust **1.85.0** minimum check, formatting, warning-denied Clippy, 10 editor
tests per compiler, all 100 local Markdown target checks, and repository gates
pass.

The unchanged four-package matrix proves whole-project, separate-package, and
source-free behavior; affected-only invalidation; failed-output preservation;
and relocated serial/parallel x86-64/wasm32 determinism. Shuttle continues to
transport opaque compiler artifacts without parsing numeric spelling or
canonical values. Shuttle production code, schemas, cache keys, scheduling, and
compatibility versions remain unchanged.

**Stage 33 coordination is complete.**

## Compiler 33.3 numeric literal notation integration

Verified on Windows on 2026-09-04 with development and ASan/UBSan compilers.
Each compiler passes all **31 public compiler-protocol/toolchain and 28 native
Shuttle cases** inside its **232-test** matrix. All **43 ordinary Rust tests**,
Rust 1.85 checking, formatting, and warning-denied Clippy pass.

The dedicated four-package fixture transports scientific, base-prefixed,
separated, and suffixed values only through compiler-owned source and opaque
artifacts. Whole-project, separate-package, and source-free output agrees.
Relocated serial/parallel builds produce identical interface artifacts on both
targets and identical native artifacts and executables on x86-64. A valid edit
rebuilds only the affected package and consumer; an invalid base digit preserves
completed package artifacts and the executable without running stale output.

Shuttle production code, schemas, cache keys, and scheduling remain unchanged.
Artifact format **4**, compiler ABI **4**, runtime ABI **3**, process protocol
**2**, receipt schema **1**, and manifest schema **1** remain unchanged.
Compiler 33.4 coordinated exit audit requires separate authorization.

## Compiler 33.2 numeric literal notation frontend

Verified on Windows on 2026-09-04 with development and ASan/UBSan compilers.
Each compiler passes all **30 public compiler-protocol/toolchain and 26 native
Shuttle cases** inside its **226-test** matrix. The two new target-specific
checks exercise the compiler frontend directly; no Shuttle fixture changes.

Numeric spelling, exact evaluation, canonical HIR, and diagnostics remain
compiler-owned. Shuttle continues to transport opaque compiler artifacts, and
its production code, schemas, cache keys, and scheduling are unchanged.
Artifact format **4**, compiler ABI **4**, runtime ABI **3**, process protocol
**2**, receipt schema **1**, and manifest schema **1** remain unchanged.
The separately authorized compiler 33.3 integration checkpoint is recorded
above.

## Compiler 33.1 numeric literal notation contract

Recorded on 2026-09-04. Scientific notation, integer base prefixes, digit
separators, exact decoding, diagnostics, and canonical HIR remain compiler-owned.
Artifact format 4, compiler ABI 4, runtime ABI 3, protocol 2, receipt schema 1,
and manifest schema 1 remain unchanged.

This documentation-only checkpoint added no Shuttle implementation or fixture.
The separately authorized compiler 33.2 frontend checkpoint is recorded above;
coordinated package verification remains scheduled for later Stage 33
checkpoints.

## Coordinated 32.4 typed numeric literal exit audit

Verified on Windows on 2026-09-04 with development and ASan/UBSan compilers.
Each compiler passes all **30 public compiler-protocol/toolchain and 26 native
Shuttle cases** inside its **224-test** matrix. All **43 ordinary Rust tests**,
the Rust **1.85.0** minimum check, formatting, warning-denied Clippy, nine editor
tests per compiler, documentation links, and repository hygiene gates pass.

The unchanged public package fixtures prove whole-project, separate-package,
and source-free typed-constant behavior; affected-only invalidation;
failed-output preservation; and relocated serial/parallel x86-64/wasm32
determinism. Shuttle remains unaware of numeric spelling and transports only
opaque compiler artifacts. Artifact format **4**, compiler ABI **4**, runtime
ABI **3**, process protocol **2**, receipt schema **1**, and manifest schema
**1** remain unchanged. No Shuttle production code or configuration changed.

**Stage 32 coordination is complete.** The completed Stage 33 coordination audit
is recorded above.

## Compiler 32.3 typed numeric literal integration

Verified on Windows on 2026-09-04 with development and ASan/UBSan compilers.
Each compiler passes all **30 public compiler-protocol/toolchain and 26 native
Shuttle cases** inside the compiler's **224-test** matrix. All **43 ordinary
Rust tests**, formatting, warning-denied Clippy, and nine editor tests per
compiler pass.

Public fixtures prove suffixed constants and executable behavior across
whole-project, separate-package, and source-free compilation. They also prove
affected-only invalidation, failed-output preservation, and relocated
serial/parallel x86-64/wasm32 artifact determinism. Shuttle treats source
spelling and canonical compiler values as opaque data; no production code,
manifest option, cache field, or compatibility version changed. At this
checkpoint compiler 32.4 remained separately authorized; its completed audit is
recorded above.

## Compiler 32.2 typed numeric literal frontend

Recorded on 2026-09-04. Both development and sanitizer compiler matrices pass
all **216 CTests**, including the unchanged **29 public compiler-protocol/
toolchain and 24 native Shuttle cases** per compiler. Numeric suffix decoding,
typing, recovery, and HIR canonicalization remain compiler-owned; Shuttle does
not parse or persist source spelling. No Shuttle production code or
compatibility version changed. Coordinated package work remains scheduled for
compiler 32.3 and requires separate authorization.

## Compiler 32.1 typed numeric literal contract

Recorded on 2026-09-04. Typed numeric literal syntax and semantics remain
compiler-owned. Artifact format 4, compiler ABI 4, runtime ABI 3, protocol 2,
receipt schema 1, and manifest schema 1 remain unchanged. This checkpoint adds
no Shuttle implementation or verification fixture; the later compiler 32.2
frontend checkpoint is recorded above.

## Coordinated 31.4 MIR optimization exit audit

Verified on Windows on 2026-09-04 with development and ASan/UBSan compilers.
Each compiler passes all **29 public compiler-protocol/toolchain and 24 native
Shuttle cases** inside the compiler's **215-test** matrix. All **43 ordinary
Rust tests**, Rust **1.85.0**, formatting, warning-denied Clippy, six editor
checks per compiler, C++ formatting, documentation, and repository whitespace
gates pass.

The unchanged Shuttle package matrix proves whole-project, separate-package,
and source-free behavior; affected-only invalidation; failed-output
preservation; and relocated serial/parallel x86-64/wasm32 determinism with the
always-on optimizer. Shuttle continues to treat MIR optimization and optimized
artifacts as opaque compiler concerns. Artifact format **4**, compiler ABI
**4**, runtime ABI **3**, process protocol **2**, receipt schema **1**, and
manifest schema **1** remain unchanged. No Shuttle production behavior or
configuration was added.

**Stage 31 coordination is complete.** No later stage is active.

## Coordinated 30.4 integer conversion-mode exit audit

Verified on Windows on 2026-09-03 with development and ASan/UBSan compilers.
Each compiler passes all **29 public compiler-protocol/toolchain and 24 native
Shuttle cases** inside the compiler's **200-test** matrix. All **43 ordinary
Rust tests**, Rust **1.85.0**, formatting, warning-denied Clippy, editor checks,
C++ formatting, and repository whitespace gates pass.

Integer conversion fixtures run from distinct temporary project roots and
produce byte-identical serial/parallel package artifacts for x86-64 and wasm32.
Whole-project, separate-package, and source-free native results agree.
Conversion edits rebuild affected packages while reusing unrelated packages;
invalid follow-up input preserves completed artifacts and executables and never
runs stale output.

The compiler owns the exhaustive 81 canonical-pair constant oracle and 121
accepted-spelling-pair runtime/LLVM matrix. Shuttle continues to treat scalar
constants and object code as opaque artifacts. Artifact format **4**, compiler
ABI **4**, runtime ABI **3**, process protocol **2**, receipt schema **1**, and
manifest schema **1** remain unchanged. No Shuttle production behavior or
test-only compiler switch was added.

**Stage 30 coordination is complete.** Later work requires a separately
approved stage.

## Compiler 30.3 integer conversion lowering and integration checkpoint

Verified on Windows on 2026-09-03 with development and ASan/UBSan compilers.
Each compiler passes all **29 public compiler-protocol/toolchain and 24 native
Shuttle cases** inside the compiler's **194-test** matrix. All **43 ordinary Rust
tests**, Rust **1.85.0**, formatting, warning-denied Clippy, editor checks, C++
formatting, and repository whitespace gates also pass.

Shared real-compiler fixtures prove runtime and scalar-constant wrapping and
saturating results through Shuttle's public process boundary. Whole, separate,
and source-free package builds agree. Serial and parallel builds produce
deterministic x86-64 and wasm32 artifacts, conversion edits invalidate only
affected packages, unrelated packages are reused, and an invalid follow-up build
preserves completed artifacts and executable output.

Shuttle does not parse source expressions or conversion metadata. It remains an
opaque artifact coordinator, and no production, capability, receipt, process,
manifest, scheduler, or compatibility version changed at this checkpoint.

**Compiler 30.3 coordination is complete.** At that checkpoint, Stage 30
remained active with compiler 30.4 awaiting separate authorization.

## Compiler 30.2 integer conversion frontend and constant checkpoint

Verified on Windows on 2026-09-03 with development and ASan/UBSan compilers.
Each compiler passes all **28 public compiler-protocol and 22 native Shuttle
tests** inside its **188/188 CTest** run. All **43 ordinary Rust tests**, Rust
1.85 checking, formatting, Clippy with warnings denied, and six compiler-backed
editor tests per compiler pass. C++ formatting and both repository whitespace
checks also pass.

The compiler now validates runtime `Target::wrap(value)` and
`Target::sat(value)` expressions through `--check` and evaluates required scalar
constants. At this checkpoint, runtime MIR/LLVM and package behavior remained
assigned to 30.3, and ordinary compilation stopped at an explicit compiler
diagnostic. The 30.3 checkpoint above supersedes that temporary boundary.
Shuttle continues to treat artifacts and constants as opaque. No production,
capability, receipt, process, manifest, scheduler, or compatibility version
changed at this checkpoint.

At this checkpoint, Stage 30 coordination remained active with compiler 30.3
awaiting separate authorization.

## Coordinated 29.4 checked runtime arithmetic exit audit

Verified on Windows on 2026-09-03 with development and ASan/UBSan compilers.
Each compiler passes all **28 public compiler-protocol and 22 native Shuttle
tests** inside its **186/186 CTest** run. All **43 ordinary Rust tests**, Rust
1.85 checking, formatting, Clippy with warnings denied, and six compiler-backed
editor tests per compiler pass. C++ formatting and repository whitespace checks
also pass.

The shared 29.3 package matrix remains green for whole, separate, and
source-free native execution; runtime-ABI-2 rejection; affected-only
invalidation; unrelated reuse; output preservation; stale-run prevention; and
relocated serial/parallel artifact determinism on x86-64 and wasm32. The compiler
owns the expanded width, endpoint, exact-runtime-failure, and malformed-model
coverage completed in 29.4.

**Stage 29 coordination is complete.** Shuttle required no production,
capability, receipt, process, manifest, or scheduler change. Runtime ABI 3
remains opaque compiler-owned artifact metadata; no later Shuttle stage is
assigned or active.

## Coordinated 29.3 checked update integration checkpoint

Verified on Windows on 2026-09-03 with development and ASan/UBSan compilers.
Each compiler passes all **28 public compiler-protocol and 22 native tests**
inside its **182/182 CTest** run. All **43 ordinary Rust tests**, Rust 1.85
checking, formatting, Clippy with warnings denied, and six compiler-backed editor
tests per compiler pass.

The checked-update fixture exercises prefix/postfix and every arithmetic compound
through dependency object code. Whole-project, separate-package, and source-free
native runs agree. Serial and parallel builds at relocated roots produce identical
package artifacts for x86-64 and wasm32. A checked-arithmetic dependency edit
rebuilds it and its consumer while unrelated packages retain exact bytes; an
invalid follow-up preserves completed package/executable bytes and does not run
the stale executable.

Runtime ABI remains opaque to Shuttle. Existing compiler-owned runtime-ABI-2
rejection and artifact integrity checks remain authoritative; no capability,
receipt, process, manifest, scheduler, or production Shuttle behavior changed.

At the coordinated 29.3 checkpoint, Stage 29 remained active for the separately
authorized 29.4 exit audit recorded above.

## Compiler 29.2 checked integer lowering checkpoint

Verified on Windows on 2026-09-03 with development and ASan/UBSan compilers.
Each passes all **27 public compiler-protocol and 20 native tests** inside its
**166/166 CTest** run. All **43 ordinary Rust tests**, Rust 1.85 checking,
formatting, Clippy with warnings denied, and six compiler-backed editor tests
per compiler also pass.

The compiler now emits runtime-ABI-3 artifacts and rejects re-signed
runtime-ABI-2 metadata. Shuttle requires no production change: runtime ABI is
not present in public capabilities or receipts, artifacts remain opaque, and
`clothc` validates compatibility during inspect, reuse, and link. Artifact
format 4, compiler ABI 4, process protocol 2, receipt schema 1, and manifest
schema 1 remain unchanged. Update/compound and broader separate-compilation
coverage remains scheduled for 29.3.

## Stage 28.4 scalar constant exit audit

Verified on Windows on 2026-09-02 with development and ASan/UBSan compilers:
all **27 public compiler-protocol and 20 native tests** pass with each compiler,
inside its **148/148 CTest** run. All **43 ordinary Rust tests**, Rust 1.85
checking, formatting, and Clippy with warnings denied pass.

Computed-constant fixtures verify source-free privacy, typed narrowing, integer
and enum labels, and failed-output preservation. Private-value and transitive
dependency edits rebuild affected packages; unchanged evaluated values do not
erase changed source/artifact identity. Unrelated packages preserve their bytes,
and subsequent warm builds reuse all four packages.

Old consumers fail to link against changed dependencies without replacing the
completed executable. Invalid arithmetic, cycles, and duplicate-producing edits
preserve completed consumer outputs; failed `run` never executes stale code.
Whole-project, separate, and source-free execution agree after valid edits.

Relocated serial/parallel builds with reversed dependency declarations produce
identical interface artifacts on x86-64 and wasm32, and identical native objects,
package artifacts, and executables on x86-64 across a PE timestamp boundary.
Cycle and independent arithmetic diagnostics retain source locations and order
after normalizing only the fixture-root path. Existing enum/struct/switch and
integrity regressions remain part of the full runs.

**Stage 28 coordination is complete.** Artifact format 4, compiler ABI 4,
runtime ABI 2, process protocol 2, receipt schema 1, and manifest schema 1 are
unchanged from 28.3. Tests use the existing fixtures and public compiler protocol;
Shuttle does not interpret or evaluate constant metadata. No scheduling feature,
dependency source, or later stage is introduced.

## Compiler 28.3 constant integration checkpoint

Verified on Windows on 2026-09-02 with development and ASan/UBSan compilers:
all **25 public compiler-protocol and 17 native tests** pass with each compiler,
within the compiler's **148/148 CTest** runs. All **43 ordinary Rust tests**,
Rust 1.85 checking, formatting, and Clippy with warnings denied pass.

Shuttle now requires artifact format **4** in capabilities and receipts.
Process stubs reject old format-3 claims. Compiler ABI 4, runtime ABI 2,
process protocol 2, receipt schema 1, and manifest schema 1 are unchanged.
Shuttle still treats package artifacts as opaque and retains exact compiler,
target, and dependency-digest checks.

New native coverage compares whole-project, separate-package, and source-free
execution for computed and negative constants, unsigned endpoints, bool/char and
float values, private-to-public dependencies, cross-package chains, aliases,
and integer/enum switch labels. Public CLI tests emit new forms on both targets;
invalid constants preserve completed LLVM/native/interface outputs. The former
check-only test now verifies supported emission and failure preservation.

**Compiler 28.3 coordination is complete.** The broader constant-specific
dependency-evolution, stale-link, relocated serial/parallel, and exit audit
remain **28.4** work. This checkpoint does not close Stage 28.

## Stage 27.4 switch exit audit

Verified 2026-09-02 with development and ASan/UBSan compilers: all **24 shared
protocol and 16 native tests** pass; each compiler passes all 141 CTests. All
43 ordinary Rust tests, formatting, Clippy with warnings denied, and Rust 1.85
checking pass.

The shared switch fixture exercises enum declarations and retained integer/enum
constants across whole-project, separate, and source-free compilation:

- reordered cases and changed constants rebuild affected packages, preserve
  unrelated artifacts, and reuse all four packages on the next unchanged run;
- added, removed, renamed, and duplicate-producing case edits reject invalid
  consumers without replacing their completed artifacts or executable, and
  failed `run` never executes the stale program;
- an explicit default accepts a new case, with matching output in all three
  compilation modes; private constants and unrelated enum labels stay invalid
  without dependency sources;
- old consumer artifacts cannot link against edited dependencies, and rejected
  link requests preserve the completed executable;
- relocated serial/parallel builds with reversed dependency declaration order
  produce identical interface artifacts on x86-64/wasm32 and identical native
  artifacts/executables on x86-64 across a PE timestamp boundary; and
- added-case diagnostics agree after normalizing only the absolute fixture-root
  prefix, while explicit protocol-v2 keyword aliases fail without replacing
  completed output.

**Stage 27 coordination is complete.** Tests extend existing fixture helpers and
suites; Shuttle still treats artifacts as opaque compiler-owned data. Artifact
format 3, compiler ABI 4, runtime ABI 2, protocol 2, receipt schema 1, and manifest
schema 1 are unchanged. No scheduling or dependency-resolution feature is added.

## Stage 27.3 source-free switch lowering

Verified 2026-09-02 with development and ASan/UBSan compilers: all 139 CTests
pass, including 22 shared protocol and 13 native tests. All 43 ordinary Rust
tests, formatting, Clippy with warnings denied, and Rust 1.85 checking pass.

The switch native test compares whole-project and separate execution, hides
dependency sources, and compiles/links the consumer from verified artifacts.
It covers dependency-owned switch bodies, nominal enum aliases and constants,
grouped labels, defaults, widening of imported integer constants, and full-width
unsigned labels. Failed-emission checks now use a duplicate-label source error
and retain the completed LLVM output. Switch works in artifact-based checking
and builds; Shuttle still does not parse language bodies or interpret enum tags.

Artifact format 3, compiler ABI 4, runtime ABI 2, process protocol 2, receipt
schema 1, and manifest schema 1 are unchanged. At this checkpoint, evolution,
invalidation, and serial/parallel verification remained scheduled for 27.4;
the exit audit above closes that matrix.

## Stage 27.2 switch keyword coordination

Verified 2026-09-02 with development and sanitizer compilers: 22 shared protocol
tests and 12 native tests pass, alongside 43 ordinary Rust tests, formatting,
Clippy with warnings denied, and Rust 1.85 checking. Both compiler configurations
pass all 127 CTest entries.

Manifest validation rejects `switch`, `case`, and `default` as dependency aliases;
compiler protocols independently use lexer classification. Integer keyword ordering
in Shuttle's binary-search table is corrected and tested. Package-name grammar
and persistent schemas are unchanged. The emission-failure test checks that a
switch lowering diagnostic does not replace a completed LLVM output.

At that checkpoint switch was supported only by direct compiler frontend
checking. The 27.3 audit above supersedes the temporary native/artifact gate;
the 27.4 audit above closes the broader dependency-evolution verification.

## Quality gates

Run the standalone quality gates from this checkout:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --all-targets --locked
cargo +1.85.0 check --all-targets --locked
```

These tests cover schema validation, portable paths, discovery, graph ordering,
cycles, duplicate identities, request construction, compiler selection, and
process status/stream handling. They also verify default progress ordering,
quiet mode, and standard-output isolation. The process tests compile a test-only
Rust compiler stub with `rustc`; they never use shell scripts or mutate the
parent process environment.

## Real compiler tests

`tests/fixtures/local_graph/` is the shared four-package project. The app imports
two packages that use different aliases for one shared dependency. Other source
files contain private declarations, equal relative type names, and competing
`Main` methods. Invalid cases modify isolated copies of this fixture.

The real-compiler suites are explicitly ignored by ordinary Cargo runs. Run
them against an absolute compiler path:

```sh
export CLOTHC_UNDER_TEST=/absolute/path/to/clothc
cargo test --locked --test toolchain_tests -- --ignored
cargo test --locked --test native_tests -- --ignored
```

On PowerShell, set the path with
`$env:CLOTHC_UNDER_TEST = 'C:\absolute\path\clothc.exe'`.
`toolchain_tests` needs only the compiler. `native_tests` additionally requires
the compiler's configured LLVM `llc`, native linker, and Cloth runtime library.
Missing or invalid compiler paths fail rather than silently skip these runs.

When this checkout is used as the Cloth compiler's `shuttle` submodule, its
development and sanitizer CMake presets register these commands automatically.
Run `ctest --preset dev -L toolchain` or
`ctest --preset sanitize -L toolchain` from the compiler checkout. The latter
tests the same public process boundary against the instrumented compiler.

Fixtures are copied into temporary directories, including spaces and Unicode
path cases. Every child has a 300-second timeout and both streams are drained
concurrently. CTest gives each Cargo suite 1,200 seconds and serializes Cargo
access. Unix-only symlink behavior is tested only on Unix hosts; Windows tests
do not require symlink-creation privileges.

Stable Rust and `forbid(unsafe_code)` remain Shuttle's baseline. No nightly
Rust sanitizer toolchain is required or claimed; the applicable sanitizer gate
is the shared suite against the C++ ASan/UBSan build.

## Stage 23 exit verification

Protocol process tests cover strict capability and receipt transport, stable
topological order, one compilation per diamond node, compiler failure context,
stale-output refusal, and exclusive writer locking. Real-compiler checks use
interface artifacts without native tools and prove consumer compilation does
not reopen removed dependency sources. Native tests compile package objects,
link one entry wrapper and runtime, compare behavior with protocol v1, reject
malformed link closures atomically, verify relocated artifact bytes, and
exercise spaces and Unicode in project paths.

The coordinated GNU development and Clang ASan/UBSan runs each pass all 92
CTest entries, including all 14 protocol and seven native cases. Rust format,
Clippy with warnings denied, all 36 ordinary tests, and the Rust 1.85 baseline
also pass. Stage 23 is complete; automatic cache reuse remains deferred.

## Stage 24.2 responsiveness checkpoint

Verified on Windows on 2026-09-01 with a release Shuttle executable and the GNU
development compiler. The one-package `examples/Shuttle.toml` `wasm32` check is
the repeatable cold-path benchmark; `--quiet` excludes terminal rendering while
retaining the same compilation path. Its initial observed time was 9.35 seconds.
After optimizing exact executable hashing and sized binary reads, five warm-file
system runs measured 216.3, 161.9, 154.6, 158.4, and 162.0 milliseconds. The
median is 161.9 milliseconds, a 98.3% reduction from the initial observation.

The compiler capability query accounted for 5.86 seconds of the initial run.
Its five corresponding post-change runs measured 82.9, 70.2, 66.1, 68.5, and
70.3 milliseconds, with a 70.2-millisecond median. SHA-256 values and all exact
compiler, runtime, native-tool, source, and dependency identities are unchanged.

The checkpoint passes all 92 development and all 92 sanitizer CTest entries,
all 37 ordinary Shuttle tests, Rust formatting, Clippy with warnings denied,
the Rust 1.85 baseline, C++ formatting, and repository whitespace checks.
At this checkpoint, validated unchanged-package reuse and deterministic parallel
scheduling remained the active Stage 24.3 and 24.4 work.

## Stage 24.3 reuse checkpoint

Verified on Windows on 2026-09-01 with the GNU development compiler and the
Clang/MSVC-library ASan/UBSan compiler:

- unchanged interface and object builds validate and reuse every package with
  no package compilation;
- exact manifest changes invalidate their package, while an unchanged artifact
  digest stops downstream invalidation;
- source changes invalidate every consumer reached through changed dependency
  digests, while target and compiler changes reject all incompatible entries;
- runtime and native-tool identities remain exact compatibility gates, and a
  corrupt candidate is rebuilt without invalidating consumers when the repaired
  artifact is byte-identical;
- malformed local state is an ordinary miss, immutable state publication leaves
  one current record, and both interface and object workspaces reject concurrent
  writers; and
- all 92 development and 92 sanitizer CTest entries pass, including 17 shared
  protocol and eight native cases. All 40 ordinary Shuttle tests, Rust 1.85,
  Rust formatting and Clippy, C++ formatting, and whitespace checks pass.

Stage 24.3 is complete. Deterministic bounded parallel scheduling remains the
active Stage 24.4 work.

## Stage 24.4 parallel scheduling and exit audit

Verified on Windows on 2026-09-01 with the GNU development compiler and the
Clang/MSVC-library ASan/UBSan compiler:

- `--jobs` rejects zero, defaults to available host parallelism, and never runs
  more package compiler processes than its effective bound;
- a two-worker barrier proves that the independent fixture packages overlap,
  while canonical progress remains stable;
- private diagnostic spools prevent interleaving and select the same exact
  failure bytes as `--jobs 1` even when another worker fails first;
- real-compiler serial and parallel diagnostics are byte-identical; and
- relocated one-job and four-job native builds produce byte-identical package
  artifacts and executables.

All 92 development and 92 sanitizer CTest entries pass, including 18 shared
protocol and eight native cases. All 43 ordinary Shuttle tests, Rust 1.85, Rust
formatting and Clippy, C++ formatting, and both repositories' whitespace checks
pass. The Stage 24.2 responsiveness baseline and Stage 24.3 unchanged-build
coverage remain intact. Stage 24 is complete.

## Stage 25 enum exit audit

Verified on Windows on 2026-09-02. Both compiler development and ASan/UBSan
configurations pass all 95 CTest entries, including 20 real-compiler protocol
tests and nine native tests. All 43 ordinary Shuttle tests, formatting, Clippy
with warnings denied, and the Rust 1.85 baseline pass.

Enum coverage verifies format-2 receipts, every public case spelling,
source-free imports and static constants, case-edit dependent invalidation,
independent-package reuse, byte-identical serial/parallel artifacts, and
whole-project/separate native equivalence. Shuttle continues to treat compiler
artifacts as opaque. Process protocol 2 and manifest schema 1 are unchanged;
format-1 packages must be rebuilt. Stage 25 is complete.

## Stage 26.3 aggregate compatibility checkpoint

Verified on Windows on 2026-09-02. Both compiler development and ASan/UBSan
configurations pass all 121 CTest entries, including 21 real-compiler protocol
tests and ten native tests. All 43 ordinary Shuttle tests, formatting, Clippy
with warnings denied, and the Rust 1.85 baseline pass.

Capabilities and receipts require artifact format 3; tests reject old format-2
responses before compilation or reusable-state publication. Source-free
aggregate dependencies work for wasm32 interface checking and x86-64 native
compilation, retaining private layouts and aggregate calls. Nested values,
arrays, shallow references, equality, and output match whole-project behavior.
Shuttle still treats compiler artifacts as opaque and introduces no process,
receipt, manifest, or scheduling protocol change.

The compiler owns ABI 4 and runtime ABI 2 compatibility; older packages must
be rebuilt. That closed the 26.3 compatibility checkpoint, not Stage 26. The
remaining struct-specific invalidation and equivalence work was verified in
the 26.4 audit below.

## Stage 26.4 struct exit audit

Verified on Windows on 2026-09-02. Development and ASan/UBSan compiler builds
each pass all 121 CTests, including **22 shared protocol and 12 native tests**.
All 43 ordinary Rust tests, Rust 1.85 checking, formatting, and Clippy with
warnings denied pass.

Struct fixtures prove:

- relocated one-job/four-job interface artifacts are byte-identical on x86-64
  and wasm32; native artifacts and executables are byte-identical on x86-64
  across a PE timestamp boundary;
- private-layout and member additions rebuild affected consumers, preserve
  unrelated artifact bytes and reuse, and permit complete reuse on the next
  unchanged invocation;
- whole-project and separate execution match both before and after those edits;
- source-free packages preserve aggregate overloads, constructor arguments,
  results, inherited fields, interface dispatch, and `super` calls; and
- private constructors/fields/methods remain inaccessible with dependency
  sources removed, and failed compilation preserves completed output.

**Stage 26 coordination is complete.** Artifacts remain opaque to Shuttle.
Format 3, compiler ABI 4, runtime ABI 2, process protocol 2, receipt schema 1,
and manifest schema 1 are unchanged by this audit. No scheduling policy,
new build system, remote dependency feature, or subsequent stage is introduced.

## Stage 26.5.1 explicit interface overrides

Verified on Windows on 2026-09-02 with both compiler configurations. Each passes
122 CTests, including all 22 shared protocol and 12 native tests. All 43 ordinary
Rust tests, Rust 1.85 checking, formatting, and Clippy with warnings denied pass.

Shared implementing declarations now use `override`. The source-free struct
consumer test rejects missing and unmatched markers, preserves the completed
artifact on failure, then accepts the corrected interface implementation.
Native dispatch, inherited calls, whole-project/separate equivalence,
serial/parallel bytes, and reuse/invalidation remain covered by existing suites.

The compiler/editor audit is complete. Artifacts remain opaque to Shuttle;
format 3, compiler ABI 4, runtime ABI 2, process protocol 2, receipt schema 1,
and manifest schema 1 are unchanged. No new Shuttle feature is scheduled.

## Compiler 28.2 constant checkpoint

Verified on 2026-09-02 with development and sanitizer compilers: all 25 shared
protocol and 16 native tests pass, as do all 43 ordinary Rust tests, Rust 1.85
checking, formatting, and Clippy with warnings denied.

The added public-process test accepts arithmetic, forward references, signed
literal conversions, and skipped division-by-zero expressions through direct
`clothc --check`. It verifies explicit LLVM/native/interface emission refusal
and byte-for-byte preservation of completed outputs. An evaluated zero divisor
still fails checking with source-error status one. Existing native and package
behavior remains covered without adding launch scripts.

Shuttle production code and format requirements are unchanged. New constant
forms are not yet usable by `shuttle check`, because it produces package
interfaces. Format-4 integration and new-form source-free/native execution
remain scheduled for compiler 28.3; this is not the Stage 28 exit audit.
