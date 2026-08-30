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

## Status

Stage 22 is active. Its 22.1 public contract and 22.2 Rust bootstrap are
complete; Stage 22.3 has not started. The approved work shared with the Cloth
compiler is:

1. implement the approved manifest model and production CLI foundation;
2. implement deterministic local dependency planning in Shuttle;
3. add the compiler's explicit package graph interface; and
4. verify the boundary with cross-repository projects.

[`ROADMAP.md`](ROADMAP.md) owns Shuttle's stage order and scope.
[`TODO.md`](TODO.md) owns the concrete scheduled work and deferred backlog.
