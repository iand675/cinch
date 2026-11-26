# Standard library benchmarks (`benchmarks/stdlib/`)

Benchmark harnesses, paired comparisons, and recorded measurement snapshots for Cinch's standard library (`seed/prelude/`) across codecs, checksums/hashes, byte scanning, CSV/URL/HTML/MIME/multipart processing, and streaming pipelines.

## Running the benchmark driver

Build the release Rust compiler (or point `--compiler` at a staged Cinch binary) and run `tooling/bench_stdlib.py` from the repository root:

```sh
cargo build --manifest-path seed/Cargo.toml --release --bin cinch

python3 tooling/bench_stdlib.py \
  --compiler seed/target/release/cinch \
  --opt-level 2 \
  --sizes 32 1024 16384 \
  --iterations 32 --samples 5 --warmup 1 \
  --artifacts build/benchmarks/current \
  --output build/benchmarks/current.json
```

Useful options:
- `--opt-level 0|1|2|3`: selects the Clang optimization level for both the measured binary and retained assembly (defaults to `2`).
- `--cpu N`: pins the driver and child processes to a specific CPU core.
- `--reference-go /path/to/go`: also builds and measures the independent Go SDK comparison driver (`tooling/bench_stdlib_reference.go`), asserting matching observable checksums on every workload.
- `--source-override Module.Name=/path/to/file.cinch`: substitutes one module source for controlled A/B runs.
- `--artifacts DIR`: retains the generated driver source, LLVM IR, assembly, and compiled binary for inspection.

## Paired comparisons

For attributing a library or compiler optimization cleanly, alternate the retained before/after executables with `tooling/bench_stdlib_compare.py`:

```sh
python3 tooling/bench_stdlib_compare.py \
  --before build/benchmarks/before.json \
  --after build/benchmarks/after.json \
  --output build/benchmarks/interleaved.json
```

By default, `bench_stdlib_compare.py` requires identical compiler, driver, runtime, and optimization-level hashes across both runs and verifies identical output checksums. Pass `--compiler-change` when comparing two compiler binaries over identical library sources.

## Measurement methodology

- **Public API pipelines:** timing uses the runtime monotonic clock around the operation loop (excluding compilation, process startup, and input setup, and including input mutation, result materialization, and checksum consumption).
- **Observable checksums:** inputs mutate between iterations and every run records an output fingerprint so optimizations cannot silently change behavior.
- **No CI timing thresholds:** wall-clock timings depend on host hardware and load; reports retain raw samples, source/binary hashes, CPU affinity, and Clang versions as diagnostic evidence rather than pass/fail CI gates.
- **Allocation vs quarantine:** live-space probes run with normal reference-count reclamation (`CINCH_ALLOC_STATS=1`, leak checking enabled). RC-debug quarantine builds intentionally retain freed blocks to catch use-after-free and are never used for peak-memory measurements.

## Coverage audit

Standard library coverage against Go's 180 public packages is tracked in [`design/stdlib-surface.json`](../../design/stdlib-surface.json) (see [`design/stdlib-architecture.md`](../../design/stdlib-architecture.md)):

```sh
python3 tooling/audit_stdlib_surface.py --go /path/to/go --check
python3 tooling/audit_stdlib_surface.py --go /path/to/go --package io
```

## Recorded snapshots and module notes

This directory holds retained JSON measurement snapshots (`*.json`), reconstruction patches (`*.patch`), specialized workload drivers (`*.cinch`), and per-area investigation notes (`*-notes.md`, `base32-benchmark-report-*.md`, `multipart-*.md`) documenting historical experiments and remaining codegen costs.
