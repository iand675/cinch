# Standard library measurements

Run the public API workload driver from the repository root after building the
native compiler and selecting a compatible Clang:

```sh
python3 tooling/bench_stdlib.py \
  --compiler compiler/target/debug/cinch \
  --sizes 32 1024 16384 --iterations 32 --samples 5 --warmup 1 \
  --artifacts build/benchmarks/current \
  --output build/benchmarks/current.json
```

`--cpu` can pin the driver and measured processes to one allowed CPU. Use the
same CPU, arguments and workload-driver hash when comparing compiler changes.
`--artifacts` retains the exact executable, LLVM, assembly and driver source for
investigating allocation, calls, representation conversions and missed inlining.

Timing uses the runtime monotonic clock around the operation loop. Compilation,
process startup, input preparation and printing are excluded. Input mutation,
result consumption and workload dispatch are included. Results are public API
pipelines: encoders include conversion to observable output bytes, decoders
include conversion of input bytes to text, and stream measurements include
adapter construction. Binary/varint pipelines do not report a whole-buffer
throughput because they access only a small field. These are not isolated
instruction timings.

The driver changes input between iterations and retains an observable checksum.
Every report retains raw samples, checksums, workload arguments, source and
compiler hashes, CPU, affinity and Clang version. Machine-dependent timings are
evidence for investigation, not CI pass/fail thresholds or release guarantees.
Correctness, malformed input, boundary behavior and memory lifetime have
separate consumer tests.

## Recorded snapshots

- `wire-2026-11-15.json`: initial wire-codec/hash pipelines.
- `wire-and-stream-2026-11-15.json`: adds stream pipelines, before native word
  loads and ASCII-word validation.
- `pre-rebase-word-ascii-2026-11-15.json`: native bounded loads and ASCII SWAR,
  before rebasing onto the newer upstream code generator. Its matching compiled
  artifacts remain in `build/benchmarks/pre-rebase` in the development lane.

The reports explicitly record dirty source snapshots and their actual compiler
binary/source identities. A parent commit in a report is not a claim that the
uncommitted measured sources were already part of that commit. Comparing reports
with different driver hashes also changes the experiment; use matching drivers
for an attribution to a particular optimization.

## Coverage audit

`design/stdlib-surface.json` inventories all 180 public packages in the pinned
reference SDK. It deliberately records partial or unreviewed coverage instead
of interpreting the presence of a Cinch module as full functionality.

```sh
python3 tooling/audit_stdlib_surface.py --go /path/to/reference-sdk/bin/go --check
python3 tooling/audit_stdlib_surface.py --go /path/to/reference-sdk/bin/go --package io
```

The second command lists the exact reference declarations for operation-level
review. Complete coverage requires an idiomatic capability mapping, working
implementations, error/runtime tests, and relevant benchmarks. The broad standard
library goal remains open; these measurements do not establish full coverage.
