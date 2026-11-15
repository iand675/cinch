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
- `post-rebase-word-ascii-2026-11-15.json`: the same library and driver after
  rebasing onto upstream `3492d1b1`. Checksum pipelines generally improved;
  codec timings were mixed, so a rebase alone did not resolve their costs.
- `post-conversion-fast-paths-2026-11-15.json`: the same library and driver with
  immediate `Nat` conversion paths in codegen. Heap values and invalid inputs
  keep the runtime fallback. Byte results box directly, and 32-bit targets
  retain their smaller immediate range.
- `interleaved-conversion-check-2026-11-15.json`: alternating runs of the
  retained pre/post executables on CPU 0, with one warmup and five measured
  runs each. Executable hashes and matching observable checksums are retained.
  Validation processes were moved to CPUs 1–4 for these measurements.
- `post-base32-tail-2026-11-15.json` and
  `interleaved-base32-tail-check-2026-11-15.json`: Base32 resumes scalar
  decoding at the first undecoded UTF-8 boundary. It previously unpacked the
  entire original string to discard its already decoded SIMD prefix. The
  alternating 16 KiB pipeline fell from 4.289 ms to 1.207 ms (3.55×), with
  matching checksums; 32-byte and 1 KiB cases are retained too. Runtime tests
  cover both alphabets, padding/newlines and Unicode errors after long prefixes.

The alternating check gave the following median times for 16 KiB pipelines
(32 operations per process). These are local measurements, not general speed
guarantees:

| Pipeline | Before conversion change | After | Ratio |
| --- | ---: | ---: | ---: |
| Hex encode | 1.079 ms | 0.929 ms | 1.16× |
| Base32 encode | 4.345 ms | 4.043 ms | 1.07× |
| Base32 decode | 4.645 ms | 4.396 ms | 1.06× |
| Base64 decode | 9.151 ms | 8.650 ms | 1.06× |

The full non-interleaved sweep initially showed slower decoder results; the
alternating check did not reproduce that regression. Raw samples remain in
both reports so this variation is visible. Assembly inspection found static
call sites in `Text.Base32.encode-into` drop from 119 to 72 and those in
`Text.Hex.encode-into` from 112 to 76. Static call counts describe generated
function bodies, not executed calls per byte. Many runtime arithmetic,
ownership and byte-access calls remain; these pipelines are still candidates
for further codegen and allocation work.

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
