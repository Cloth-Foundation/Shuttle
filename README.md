# Shuttle

Shuttle is Cloth's official project, build, and package manager. Shuttle
owns projects and repeatable builds; the Cloth compiler owns the language and
translation of explicit inputs into verified outputs.

Shuttle is implemented in stable Rust 2024. It communicates with the C++ Cloth
compiler through a versioned child-process protocol rather than C++ headers,
FFI, or a shared-library ABI. Released Shuttle binaries do not require users to
install Rust. See [Implementation language](docs/implementation_language.md).

## Responsibilities

Shuttle will own:

- the versioned `Shuttle.toml` project manifest and future `Shuttle.lock`;
- package, target, and workspace configuration;
- local and eventually remote dependency resolution;
- deterministic build planning and compiler/linker invocation;
- build profiles, incremental state, caches, and output layout; and
- the `shuttle build`, `run`, `check`, `test`, and publishing workflows as they
  are scheduled.

Shuttle will not parse Cloth source, implement imports or visibility, reproduce
type or ABI rules, or depend on private compiler representations. It will call
the compiler through a versioned process-level build protocol.

The Cloth compiler will not parse `Shuttle.toml`, resolve dependency versions,
access registries, select build profiles, or apply workspace policy.

## Contracts

- [`Shuttle.toml` version 1](docs/manifest.md)
- [Shuttle-to-`clothc` protocol version 1](docs/compiler_protocol.md)
- [Shuttle-to-`clothc` protocol version 2](docs/proposals/compiler_protocol_v2.md)
- [Cloth package artifact contract](../docs/proposals/stage_23_artifacts.md)
- [Build execution and progress](docs/build_execution.md)
- [Stage 24 local artifact reuse](docs/proposals/stage_24_reuse.md)
- [Implementation language](docs/implementation_language.md)

## Build and test

Shuttle requires stable Rust with Cargo. The minimum supported Rust version is
1.85.0.

```sh
cargo build --locked
cargo test --all-targets --locked
cargo clippy --all-targets --locked -- -D warnings
cargo fmt --all -- --check
```

Run `cargo run -- --help` to inspect the command surface while developing.
Released builds use `cargo build --release --locked`.

Real-compiler and native tests are opt-in for standalone Cargo runs and are
enabled by Cloth's development/sanitizer CTest presets. See
[Testing](docs/testing.md) for the shared fixture and standalone commands.

For a local Cloth project and compiler checkout:

```sh
cargo run --locked -- check --manifest-path ../examples/Shuttle.toml \
  --compiler ../build/dev/clothc
cargo run --locked -- run --manifest-path ../examples/Shuttle.toml \
  --compiler ../build/dev/clothc
```

`check` keeps validated interface artifacts under
`target/TARGET/check/packages/`. `build` and `run` keep object artifacts under
`target/x86_64/packages/` and the native executable under `target/x86_64/` in
the root package. Local state is scoped to each target and artifact kind.

## Status

Stage 23 is complete and Stage 24 is active. Local projects, recursive local
dependencies, and `check`, `build`, and `run` use compiler protocol version 2
for deterministic separate compilation and linking. Protocol version 1 remains
available to older clients and explicit compiler tests. Unchanged local package
artifacts are reused only after compiler-owned validation; remote dependencies
and registries are not implemented.

[`ROADMAP.md`](ROADMAP.md) owns Shuttle's stage order and scope.
[`TODO.md`](TODO.md) owns the concrete scheduled work and deferred backlog.
