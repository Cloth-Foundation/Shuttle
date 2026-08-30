# Shuttle

Shuttle is Cloth's official project, build, and package manager. Its
relationship to Cloth is the same kind of boundary Cargo has with Rust: Shuttle
owns projects and repeatable builds; the Cloth compiler owns the language and
translation of explicit inputs into verified outputs.

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

## Status

Shuttle is pre-bootstrap. No implementation stage is active. The first
scheduled work is the Stage 22 contract shared with the Cloth compiler:

1. freeze the manifest schema and package/workspace terminology;
2. freeze dependency namespace mapping and the build request;
3. implement deterministic local dependency planning in Shuttle;
4. add the compiler's explicit source-root and dependency interface; and
5. verify the boundary with cross-repository projects.

Until that contract is approved, the repository intentionally contains no
manifest parser or build engine.

[`ROADMAP.md`](ROADMAP.md) owns Shuttle's stage order and scope.
[`TODO.md`](TODO.md) owns the concrete scheduled work and deferred backlog.
