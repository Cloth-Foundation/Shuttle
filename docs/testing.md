# Shuttle verification

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
