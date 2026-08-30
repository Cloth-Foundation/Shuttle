# `Shuttle.toml` manifest version 1

Status: approved Stage 22.1 contract.

`Shuttle.toml` describes one Shuttle package and its direct local dependencies.
The manifest uses TOML 1.0.0 and is a UTF-8 document. Shuttle manifest version 1
deliberately rejects TOML 1.1-only syntax so every supported parser accepts the
same language.

## Complete example

```toml
manifest-version = 1

[package]
name = "hello-world"
version = "0.1.0"
source-root = "src"

[executable]
name = "hello-world"
entry = "Main.co"

[dependencies]
models = { path = "../models" }
text_utils = { path = "../text-utils" }
```

The only required top-level entries are `manifest-version` and `[package]`.
`[executable]` and `[dependencies]` are optional. Unknown fields and tables are
errors in manifest version 1; this catches misspellings and prevents unsupported
configuration from being silently ignored.

TOML table order has no semantic meaning. Shuttle orders validated values by
their specified logical identities rather than their textual order.

## Manifest discovery

The canonical filename is exactly `Shuttle.toml`, including case. A Shuttle
command either:

1. uses the file supplied by `--manifest-path`; or
2. searches the current directory and each parent, selecting the nearest
   `Shuttle.toml`.

An explicit path must name a regular file. Discovery never searches below the
current directory. A case-only filename variation is an error even on a
case-insensitive filesystem so a project behaves the same on every host.

The manifest's parent is the **package root**. Paths in the manifest use `/` as
their separator and are resolved relative to the table's package root unless a
field says otherwise.

## Manifest version

`manifest-version` is a required top-level integer. Version 1 accepts only the
value `1`. Missing, boolean, string, negative, zero, and unsupported future
values are errors.

The manifest version controls Shuttle's schema. It is independent from the TOML
language version, the Shuttle executable version, the compiler protocol, the
package version, and the future package-artifact format.

## Package table

```toml
[package]
name = "models"
version = "0.1.0"
source-root = "src"
```

`name` and `version` are required. `source-root` is optional and defaults to
`"src"`.

### Package name

A package name is 1 to 64 ASCII characters and must match:

```text
[a-z][a-z0-9]*(?:-[a-z0-9]+)*
```

Package names are lowercase and use `-` between words. They identify packages
inside a Stage 22 graph but do not appear in Cloth import statements. Two
different package roots with the same name are an error, even when their
versions differ. Supporting multiple versions of one package requires the Stage
23 artifact and identity contract.

### Package version

`version` is a string containing a valid Semantic Versioning 2.0.0 version.
Stage 22 records and validates the exact version but performs no constraint
matching or version selection. A local dependency uses the version declared by
the manifest at its resolved path.

### Source root

`source-root` is a non-empty relative path to an existing directory. After
normalization and symbolic-link resolution, it must remain inside the package
root. Absolute paths and roots that escape the package are errors.

All directory components beneath the source root and every `.co` file stem must
be valid Cloth identifiers. The compiler recursively includes regular files
with the exact lowercase `.co` extension. Directory symbolic links are not
followed. Case-only logical identity collisions are errors.

Every package exposes the public Cloth file types beneath its source root. A
package does not need an executable declaration to be used as a dependency.

## Workspace boundary

A **workspace** is a future manifest-defined collection with more than one root
package. The Stage 22 dependency closure of one root package is a build graph,
not a workspace. Manifest version 1 rejects a `[workspace]` table; workspace
membership, shared configuration, and multi-root command selection require a
later roadmap stage.

## Executable table

```toml
[executable]
name = "hello-world"
entry = "Main.co"
```

Manifest version 1 supports at most one executable. `entry` is required and
`name` is optional, defaulting to the package name. An executable name follows
the package-name grammar.

`entry` is a non-empty path relative to the package source root. It must use `/`,
end in `.co`, resolve to a regular source file inside that root, and contain the
eligible public static `Main` selected for the executable. A dependency's own
executable declaration does not contribute an entry point when that package is
built as a dependency.

A package without `[executable]` can be checked and consumed as a local
dependency in Stage 22. Emitting a separately compiled library belongs to Stage
23.

## Local dependencies

```toml
[dependencies]
models = { path = "../models" }
text_utils = { path = "../text-utils" }
```

Each key is a source-visible dependency alias. An alias must match
`[a-z][a-z0-9_]*` and must not be a Cloth keyword. Lowercase aliases make it
clear that the prefix is a namespace rather than a public Cloth declaration.

Each value is a table containing exactly one required string field, `path`.
Equivalent standard TOML table syntax is accepted. String shorthand is not:

```toml
[dependencies.models]
path = "../models"
```

The path is relative to the depending package root. It must resolve to a
directory containing a canonical `Shuttle.toml`. Unlike `source-root`, a
dependency path may leave the depending package root so sibling projects work.
Absolute dependency paths are rejected because checked-in manifests must be
portable.

Manifest version 1 has no version constraints, optional dependencies, features,
registry sources, Git sources, or platform-conditional dependencies.

## Dependency imports

A dependency alias becomes the leading package component for imports from that
dependency. Given the `models` dependency above:

```cloth
import models::User;
import models.data::Record;
import models.services.*;
```

`models::User` selects `User.co` at the dependency source root.
`models.data::Record` selects `data/Record.co`. The prefix is resolved through
the importing Shuttle package's own dependency table.

Package names never enter source syntax. A dependency may therefore be exposed
under different aliases by different direct dependents without changing its
source. Imports within a package continue to use that package's ordinary
source-relative identities and do not prefix their own Shuttle package name.

Only direct dependencies are visible. A package cannot import through a
transitive dependency unless it declares that dependency itself. Imports are
not re-exported.

A dependency alias that equals the first component of a local source package is
ambiguous and is rejected by the compiler. Ordinary capitalization-based
visibility still applies across the dependency boundary: only public file types
and public members can be accessed.

## Graph validation and ordering

Shuttle resolves dependency manifests recursively before invoking `clothc`.
It canonicalizes manifest paths for graph identity and diagnoses:

- a missing manifest, source root, or entry file;
- duplicate package names at different roots;
- duplicate aliases or two aliases to the same dependency from one package;
- a package depending on itself;
- dependency cycles, including the complete alias path; and
- invalid or non-portable paths and identities.

The root package is selected by the initial manifest. Dependencies are visited
by alias in ascending ASCII byte order. The resulting package records are
ordered by package name, and dependency edges by owner package then alias. The
filesystem's enumeration order never affects a build request or diagnostic.

All project-significant paths must be representable as Unicode and must not
contain control characters. This keeps manifests, diagnostics, and build logs
portable across supported hosts.

## Commands covered by Stage 22

Manifest version 1 supports these project commands:

- `shuttle check` validates and type-checks the package graph without emitting
  a program;
- `shuttle build` builds the root package's executable; and
- `shuttle run` builds and runs that executable without source-visible command
  arguments.

All accept `--manifest-path`, `--compiler`, and `--target`. The Stage 22 target
names are `x86_64` and `wasm32`; executable emission remains limited to the
compiler's supported native target. `build` and `run` require `[executable]`.

Stage 22 does not provide `new`, `init`, `test`, `clean`, workspaces, profiles,
features, publishing, or program-argument delivery. Those commands and concepts
require later scheduled contracts.

## Diagnostics

Malformed TOML is reported at the parser's most precise manifest location.
Schema errors identify the owning key or table. Graph errors identify the
dependency declaration that introduced the failing edge and include the
resolved package or cycle when applicable.

Human diagnostics use:

```text
path/to/Shuttle.toml:line:column: error: message
```

Independent diagnostics are sorted by canonical logical manifest path and
source position. Shuttle must finish manifest and graph validation before it
queries or invokes the compiler.

## Migration from `cloth.toml`

`cloth.toml` is the obsolete metadata-only Stage 8 marker. It is not parsed as a
Shuttle manifest.

- If discovery finds `cloth.toml` without `Shuttle.toml`, Shuttle reports a
  migration error with the required `Shuttle.toml` fields.
- If both names exist in one package root, Shuttle reports an error requiring
  removal of `cloth.toml`.
- Protocol-mode and direct-mode `clothc` never search for either manifest.
- Direct compiler callers provide a source root explicitly or use the
  documented single-entry fallback.

There is no automatic content migration because the old marker carried no
defined project metadata.
