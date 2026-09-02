# Stage 24 bounded parallel scheduling

Status: **implemented; Stage 24.4 completed 2026-09-01**.

This document owns Shuttle's local package concurrency and deterministic output
contract. It does not change Cloth semantics, compiler protocol version 2, or
artifact format 1.

## Job policy

`shuttle check`, `build`, and `run` accept `--jobs COUNT`. `COUNT` must be
positive. When omitted, Shuttle uses the host's available parallelism. The
effective count never exceeds the number of packages in the resolved graph.
`--jobs 1` is the reference serial execution.

The limit bounds concurrent package compiler processes. Capability negotiation,
linking, and execution remain single operations. A root-package build lock still
excludes a second writer for the same target and artifact kind.

## Ready-package scheduling

Shuttle assigns each package a dependency level: packages with no dependencies
are level zero, and every other package is one level after its deepest direct
dependency. Levels and packages within a level retain the canonical graph order.

Shuttle processes levels in order. Within a level it validates all candidates in
canonical chunks no larger than the effective job count, then compiles all
misses and packages without candidates under the same chunk bound. Therefore:

- no package starts before all of its dependencies have produced validated
  receipts;
- independent packages in one chunk may validate or compile concurrently;
- candidate validation for a level completes before fallback compilation;
- every compiler invocation is preceded by its `validating`, `checking`, or
  `compiling` progress line; and
- changing the job count cannot change compiler arguments, dependency receipts,
  artifact identities, or link order.

Candidate hits are inserted into the produced-artifact closure only after their
compiler receipt matches the exact local state. Misses and packages without a
candidate enter the compilation phase. Successful compilations publish state in
canonical order.

## Deterministic streams and failure selection

Compiler stdout remains a bounded protocol response. Each compiler stderr stream
is drained concurrently into a private temporary spool, so a verbose compiler
cannot block on a full pipe and parallel diagnostics do not interleave. After a
chunk completes, Shuttle replays those bytes unchanged in canonical package
order.

When multiple speculative workers fail, Shuttle reports the first failure in
canonical order, matching `--jobs 1`; later diagnostic spools are discarded.
All started workers are joined before their scoped scheduling phase ends. An
artifact written by a later speculative worker is never trusted unless it also
has matching local state and passes compiler-owned validation on a later build.

Progress is human-readable and may report a different effective job count across
hosts. It is not a build input or machine protocol. `--quiet` suppresses progress
but never compiler diagnostics.

## Verification

The process suite proves a two-worker barrier between the independent fixture
packages, rejects `--jobs 0`, checks canonical progress, compares serial and
parallel package bytes, and verifies exact diagnostic equality when two workers
fail in the opposite wall-clock order. Real-compiler tests compare serial and
parallel diagnostics byte for byte. Native tests build relocated serial and
parallel graphs and compare every package artifact and final executable byte for
byte.
