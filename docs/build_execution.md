# Build execution and progress

Shuttle reports build activity on standard error. Standard output remains
reserved for the executed Cloth program and explicit machine-readable command
results.

The default local command sequence is:

```text
shuttle: preparing build for x86_64 (2 packages)
shuttle: scheduling with 2 jobs
shuttle: compiling foundation v1.0.0 [1/2]
shuttle: compiling app v1.0.0 [2/2]
shuttle: linking app
shuttle: finished build for x86_64 in 842ms
```

`check` uses `checking` and omits linking. `run` reports the completed build,
then reports the executable path before transferring its standard input,
standard output, standard error, and exit status directly to the program.

Progress is emitted before each potentially long compiler or linker operation.
Elapsed time and the effective job count are informational and are not
deterministic build inputs or a machine-readable protocol. Package order follows
Shuttle's deterministic dependency-level plan. Compiler diagnostics are spooled
per process and replayed unchanged in canonical package order; Shuttle does not
prefix or rewrite individual compiler diagnostic lines.

Pass `--quiet` after `check`, `build`, or `run` to suppress successful progress.
It does not suppress compiler diagnostics or executed-program output.

Pass `--jobs COUNT` to set a positive package-process limit. Without it, Shuttle
uses the host's available parallelism and caps the effective count at the graph's
package count. `--jobs 1` selects the reference serial execution.

When local state identifies a candidate, Shuttle reports `validating` before
asking `clothc` to check it. A hit reports `reusing`; a miss reports the normal
`checking` or `compiling` line before compilation. Independent ready packages
may run concurrently, but dependencies, progress, state publication, failure
selection, and diagnostic replay retain canonical package order.
