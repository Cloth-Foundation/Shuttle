# Build execution and progress

Shuttle reports build activity on standard error. Standard output remains
reserved for the executed Cloth program and explicit machine-readable command
results.

The default local command sequence is:

```text
shuttle: preparing build for x86_64 (2 packages)
shuttle: compiling foundation v1.0.0 [1/2]
shuttle: compiling app v1.0.0 [2/2]
shuttle: linking app
shuttle: finished build for x86_64 in 842ms
```

`check` uses `checking` and omits linking. `run` reports the completed build,
then reports the executable path before transferring its standard input,
standard output, standard error, and exit status directly to the program.

Progress is emitted before each potentially long compiler or linker operation.
Elapsed time is informational and is not a deterministic build input or a
machine-readable protocol. Package order follows Shuttle's deterministic build
plan. Compiler diagnostics are forwarded unchanged after any preceding progress
line; Shuttle does not prefix or rewrite individual compiler diagnostic lines.

Pass `--quiet` after `check`, `build`, or `run` to suppress successful progress.
It does not suppress compiler diagnostics or executed-program output.

When local state identifies a candidate, Shuttle reports `validating` before
asking `clothc` to check it. A hit reports `reusing`; a miss reports the normal
`checking` or `compiling` line before compilation. Parallel scheduling will
retain canonical package order for progress and diagnostic replay even when
work completes in a different wall-clock order.
