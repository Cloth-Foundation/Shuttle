# Shuttle-to-`clothc` protocol version 1

Status: approved Stage 22.1 contract.

This process protocol connects the Rust Shuttle executable to the C++ Cloth
compiler without a library ABI, FFI, shared headers, or internal compiler data.
Protocol version 1 compiles one complete local package graph in one `clothc`
process. Separate package artifacts begin in Stage 23.

## Capability query

Shuttle queries a selected compiler with:

```text
clothc --shuttle-protocol-version
```

A compatible compiler writes exactly `1` followed by a newline to standard
output, writes nothing to standard error, and exits successfully. The query
accepts no other argument. An unsupported or missing query is an incompatibility
diagnostic; Shuttle does not guess from the compiler's product version.

## Invocation

The argument-vector grammar is:

```text
clothc --shuttle-protocol 1
       --target TARGET
       --output-kind check|llvm-ir|executable
       [--output ABSOLUTE_PATH]
       --root-package PACKAGE_NAME
       [--entry SOURCE_RELATIVE_PATH]
       { --package PACKAGE_NAME PACKAGE_VERSION ABSOLUTE_SOURCE_ROOT }
       { --dependency OWNER_PACKAGE ALIAS DEPENDENCY_PACKAGE }
```

Shuttle starts `clothc` directly with an argument vector. It never constructs a
shell command. The absolute compiler executable path is selected by the
Shuttle command's `--compiler` option, an installed sibling toolchain binary, or
the host `PATH`, in that precedence order. The selected path is made absolute
before invocation and is not stored in `Shuttle.toml`.

`--shuttle-protocol 1` selects protocol mode and is mutually exclusive with the
legacy direct source-file command shape. Protocol options may be parsed in any
order, but Shuttle emits them in the canonical order shown above.

## Package records

Each `--package` supplies the manifest name, exact declared semantic version,
and absolute normalized source root for one graph package. There must be exactly
one record for every resolved package. Names are unique and records are ordered
by ascending package name.

`--root-package` names one supplied record. It is the package selected by the
user's initial manifest.

Each `--dependency` supplies one direct edge:

1. the package whose source may use the alias;
2. the lowercase Cloth identifier used as the import prefix; and
3. the target package name.

Edges refer only to supplied package records and are ordered by owner package,
then alias. Missing records, duplicate records or edges, cycles, self-edges, and
invalid identities are protocol errors even though Shuttle validates them
first. The compiler does not trust the caller.

## Sources and entry point

For every supplied source root, the compiler recursively enumerates regular
files with the exact `.co` extension. It does not follow directory symbolic
links. Logical source identities are sorted by Shuttle package, relative source
package, and file stem before parsing.

Every source unit retains its owning Shuttle package. Same-package and import
resolution use that owner's dependency edges, so equal relative source paths in
different Shuttle packages remain distinct.

`--entry` is a `/`-separated path relative to the root package's source root. It
is required for `executable`, optional for `check`, and forbidden when the root
manifest has no executable. When present, the compiler selects the eligible
public static `Main` from that exact file. A `Main` in a dependency or another
root-package file is not a competing native entry point.

The compiler rejects an empty package source root, invalid Cloth path
components, duplicate logical identities, a source escaping its supplied root,
and dependency aliases that collide with a local top-level source package.

## Target and output

Protocol version 1 accepts the target names `x86_64` and `wasm32`.

- `check` performs parsing and semantic/IR verification without writing an
  artifact. `--output` is forbidden.
- `llvm-ir` writes LLVM IR to the required absolute `--output` path.
- `executable` writes a native executable to the required absolute `--output`
  path and requires a supported native target and `--entry`.

Protocol mode never prints token, AST, HIR, MIR, or ABI debug dumps. Parent
directories for an output must already exist; Shuttle owns output-directory
creation and policy. The compiler writes through a temporary sibling and
replaces the requested output only after successful generation, so a failed
build does not leave a partial artifact.

## Paths and encoding

Package names, versions, aliases, target names, option names, and entry logical
paths are ASCII or UTF-8 as constrained by their owning contracts. Filesystem
paths are passed as individual OS-native arguments, not embedded in delimited
strings.

On Windows, protocol-mode `clothc` must consume the wide-character command line
and construct `std::filesystem::path` values without a locale-dependent narrow
conversion. On Unix-like hosts, it consumes the native argument bytes. Shuttle
version 1 rejects project-significant paths that cannot be represented as
Unicode or that contain control characters.

Diagnostics are UTF-8. Paths printed in diagnostics use `/` as the logical
separator. Characters that cannot appear literally in the line-oriented format
are escaped.

## Diagnostics and streams

The compiler writes human diagnostics to standard error using its existing
shape:

```text
path/to/File.co:line:column: error: message
clothc: error: invocation or toolchain message
```

Standard output is empty for `check` and for successful file-producing output
kinds. Shuttle forwards compiler standard error without rewriting source
locations or messages. It may print package/build context as separate lines but
must not prefix individual compiler diagnostic lines.

Machine-readable diagnostics are not part of protocol version 1. Adding them is
a backward-compatible protocol capability only after their schema is separately
versioned.

## Exit status

`clothc` protocol mode uses:

- `0`: the requested operation completed successfully;
- `1`: Cloth source or semantic verification rejected the program; and
- `2`: the invocation, protocol, filesystem input, output, or native toolchain
  failed.

Termination by a signal, exception, or any other status is a compiler failure.
Shuttle reports it with the root package and compiler path. Shuttle never treats
a nonzero compiler status as a successful build.

## Determinism and environment

Package and dependency argument ordering is normative. The compiler allocates
semantic and ABI identities from logical package/source ordering rather than
absolute paths or filesystem enumeration order.

The current working directory is not a project input in protocol mode. All
project and output paths are explicit and absolute. Locale, terminal color,
wall-clock time, and unrelated environment variables must not affect compiled
program bytes or diagnostic ordering.

Compiler discovery is Shuttle policy, but the chosen compiler path is fixed for
one command. Protocol version, manifest version, package version, compiler ABI,
and future artifact format remain distinct compatibility dimensions.

## Direct compiler mode

Direct `clothc` use remains supported outside protocol mode. Stage 22 adds an
explicit source-root option for multi-file direct builds and removes all
`cloth.toml` discovery. When no source root is supplied, the first entry file's
parent is the standalone root.

Direct mode does not resolve Shuttle packages or dependencies and never reads
`Shuttle.toml`.
