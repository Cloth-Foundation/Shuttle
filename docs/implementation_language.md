# Shuttle implementation language

## Decision

Shuttle is implemented in stable Rust using the Rust 2024 edition. The initial
minimum supported Rust version is 1.85.0, the first stable release supporting
that edition.

Shuttle's implementation language is not part of the Cloth language or compiler
protocol. Cloth users consume released Shuttle binaries and are not required to
install Rust or Cargo.

## Why Rust

Shuttle's core work is modeling and validating manifests, dependency graphs,
build requests, paths, processes, and failures. Rust provides:

- enums and exhaustive matching for closed manifest and build states;
- explicit `Result`-based failure paths without exceptions;
- memory safety without a managed runtime;
- OS-native path and process types for Windows, macOS, and Unix-like systems;
- deterministic ownership of graph and cache data; and
- native release binaries with no C++ ABI dependency.

Go would also provide a portable implementation and straightforward deployment.
Rust is preferred because Shuttle benefits more from explicit domain-state and
error modeling than from a garbage-collected application model.

C++ would allow reuse of a language, but not a useful interface. Sharing
compiler headers, internal representations, or a C++ ABI would couple Shuttle to
compiler implementation details. The process protocol provides stronger and
more portable compatibility regardless of implementation language.

Shuttle cannot be implemented in Cloth until Cloth has the library, filesystem,
process, and self-hosting support required to build it. A future rewrite or
self-hosting effort requires its own roadmap stage and must preserve the public
manifest and compiler protocols.

## Rust baseline

The Stage 22 bootstrap will use:

- `edition = "2024"`;
- `rust-version = "1.85"`;
- stable Rust only;
- `rustfmt` as the formatter;
- Clippy with warnings denied in continuous integration;
- `#![forbid(unsafe_code)]` in Shuttle-owned crates; and
- a committed `Cargo.lock` for reproducible Shuttle development and releases.

`Cargo.toml` and `Cargo.lock` build Shuttle itself. They are unrelated to the
`Shuttle.toml` and future `Shuttle.lock` files consumed by Shuttle for Cloth
projects.

The MSRV is verified in continuous integration. Raising it is an explicit
toolchain-policy change rather than an accidental consequence of a dependency
update.

## Dependency policy

Stage 22 keeps the initial dependency set small and justified. A dependency must
provide clear correctness or portability value, have a compatible license, and
respect Shuttle's MSRV. The lockfile is reviewed with dependency changes.

Manifest parsing must use a standards-compliant TOML implementation rather than
a hand-written partial parser. Command-line parsing and diagnostics may use
focused libraries when they reduce custom behavior without hiding Shuttle's
domain model.

The deterministic package graph, manifest validation, namespace rules, and build
request remain Shuttle-owned code and are tested independently of third-party
libraries.

## Compiler compatibility

Shuttle invokes `clothc` as a child process. It does not link to the compiler,
load it as a shared library, or use private C++ headers.

The Stage 22 protocol follows these rules:

- every invocation declares a protocol version;
- arguments are passed directly as an argument vector, never through a shell;
- project-significant paths and package mappings are explicit;
- Shuttle uses an absolute compiler path selected by toolchain configuration;
- compiler standard output, standard error, and exit status retain their
  documented meanings; and
- unsupported protocol versions fail before source compilation.

This boundary is equally implementable with Rust's process API and C++ argument
handling. The approved argument shape, path encoding, version query, exit
statuses, and diagnostic transport are defined in
[`compiler_protocol.md`](compiler_protocol.md).
