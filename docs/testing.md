# Shuttle verification

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
