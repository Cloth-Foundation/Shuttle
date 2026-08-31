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
process status/stream handling. The process tests compile a test-only Rust
compiler stub with `rustc`; they never use shell scripts or mutate the parent
process environment.

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
path cases. Every child has a 30-second timeout and both streams are drained
concurrently. CTest gives each Cargo suite 120 seconds and serializes Cargo
access. Unix-only symlink behavior is tested only on Unix hosts; Windows tests
do not require symlink-creation privileges.

Stable Rust and `forbid(unsafe_code)` remain Shuttle's baseline. No nightly
Rust sanitizer toolchain is required or claimed; the applicable sanitizer gate
is the shared suite against the C++ ASan/UBSan build.
