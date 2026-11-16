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

The driver now explicitly uses `--opt-level 2` for both the executable and
retained assembly. `--opt-level 0`, `1` or `3` selects another level and records
it in the report. Earlier reports omitted this flag, so their executables used
the CLI default `-O0`; their retained assembly was generated separately at
`-O2`. Those historical timings describe unoptimized execution. Their assembly
inspections show optimized code structure, rather than the exact instructions
executed by the measured binaries. Earlier comparisons with the independently
optimized SDK must not be read as optimized throughput parity measurements.

`--cpu` can pin the driver and measured processes to one allowed CPU. Use the
same CPU, arguments and workload-driver hash when comparing compiler changes.
`--artifacts` retains the exact executable, LLVM, assembly and driver source for
investigating allocation, calls, representation conversions and missed inlining.

Timing uses the runtime monotonic clock around the operation loop. Compilation,
process startup, input preparation and printing are excluded. Input mutation,
result consumption and workload dispatch are included. Results are public API
pipelines: encoders include conversion to observable output bytes, decoders
include conversion of input bytes to text when the API takes text, and stream
measurements include
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

## Primitive wrapper inlining

`post-primitive-inlining-2026-11-15.json` measures the same driver and library
sources as the suffix-traversal build, with forced LLVM inlining for immediate
natural arithmetic/conversions and byte length/get/set/copy wrappers. Their
ownership and bignum fallback paths inflated the generic inlining cost; exposing
the wrappers lets LLVM simplify the common tagged paths at each call site.
Runtime checks and fallback semantics remain intact. Allocating byte-buffer
creation and unrelated primitives keep the ordinary policy.

`interleaved-primitive-inlining-2026-11-15.json` alternates the retained
executables. For 16 KiB inputs, medians changed as follows:

| Pipeline | Before wrapper inlining | After |
| --- | ---: | ---: |
| Hex encode | 0.936 ms | 0.718 ms |
| Base32 encode | 3.867 ms | 3.221 ms |
| Base64 decode | 8.701 ms | 8.050 ms |
| CRC-32C | 71.6 μs | 57.3 μs |
| FNV-1a, 32 bit | 70.0 μs | 57.6 μs |
| Stream read-all | 3.72 μs | 3.67 μs |

The small-field binary and varint checks use 1,024 operations per sample to
reduce timer noise; their medians stayed within 2%. The 32-byte stream case
also improved. Executable size grew from 6,117,280 to 6,231,968 bytes (1.9%) for
this multi-workload driver, including the static runtime. Assembly text grew
more: inlining exposes cold fallback calls as well as hot arithmetic, so total
static call counts are not a proxy for executed hot-path work. The alternating
report retains the per-row iteration count and raw samples. This is evidence
for the selected primitive policy, not a recommendation to force-inline every
standard-library function.

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

## ASCII85 and checked byte access

The ASCII85 driver adds whole-buffer and stream encode/decode pipelines.
`--reference-go /path/to/sdk/bin/go` also measures an independent program calling
the reference SDK's buffer and stream APIs. Both programs mutate the same input,
consume the same output fingerprint, and must produce matching checksums.
The reference process uses `GOMAXPROCS=1`. Allocation strategies and immutable
string guarantees differ; these are useful public API comparisons, not identical
allocation microbenchmarks.

- `ascii85-2026-11-15.json` records the initial mixed block/fringe implementation.
- `ascii85-fixed-loop-2026-11-15.json` separates full encoding words from the
  incremental fringe; the improvement was small.
- `ascii85-reference-2026-11-15.json` adds the independent SDK measurements.
- `ascii85-direct-bytes-2026-11-15.json` includes the code generator's checked
  direct access to byte payloads. The compiler executable is copied before
  hashing and building so a concurrent Cargo build cannot replace it. These
  samples overlap compiler validation and show substantial scheduling variation.
- `interleaved-ascii85-byte-access-2026-11-15.json` alternates the retained
  before/after executables and reference executable on CPU 0, with validation
  confined to CPUs 1–4. It records actual executable hashes and raw samples.

For 16 KiB inputs, the alternating measurements (64 operations per process) are:

| Pipeline | Before direct byte access | After | Reference SDK |
| --- | ---: | ---: | ---: |
| Buffer encode | 563 μs | 503 μs | 53.2 μs |
| Buffer decode | 641 μs | 553 μs | 51.7 μs |
| Stream encode | 464 μs | 423 μs | 74.3 μs |
| Stream decode | 625 μs | 552 μs | 77.8 μs |

The change improves these local large-buffer medians by roughly 9–14%, while
small-input results are mixed. The remaining 6–11× gap is recorded work, not
performance parity. All variants have matching observable checksums.

Assembly inspection finds call-free SIMD digit kernels (72 and 49 assembly
lines for encoding and decoding respectively), surrounded by much larger
buffer loops. The new payload paths avoid runtime calls on valid byte length,
get, set and data accesses. Cold fault calls remain in the assembly, as do
natural-number and ownership fallbacks, so counting every static call does not
measure executed calls per byte. Reference-count validation, poisoned frees,
bounds and read-only mutation tests verify the changed paths separately.

## UTF-16 ASCII pipelines

`utf16-ascii-2026-11-15.json` measures both wire byte orders at zero, 32,
1,024 and 16,384 input UTF-8 bytes. The input is ASCII (including controls),
mutated once per operation without invalidating UTF-8 or UTF-16. Input text
preparation is outside timing; public conversions and result consumption remain
inside. The reference driver now lives in `tooling/bench_stdlib_reference.go`
and uses the SDK's UTF-16 scalar APIs plus explicit byte-order reads/writes.
The reports record its source and executable identities.

The four-unit SWAR widening/narrowing paths are exercised here. These timings
do not characterize mixed Unicode or surrogate-heavy text. At 16 KiB, this
local sweep records roughly 634–661 μs for encoding and 429–444 μs for decoding,
versus 137–139 μs and 177–180 μs for the corresponding reference pipelines.
Checksums match. Correctness tests separately cover Unicode scalars, malformed
surrogates, every single code unit and short tails under poisoned-memory and
reference-count checks. Remaining natural-number and ownership costs also
apply to these loops; the benchmark makes no performance-parity claim.

`utf16-before-order-2026-11-15.json` and
`interleaved-utf16-order-2026-11-15.json` retain the byte-order experiment.
Private loops now carry a `Bool` resolved from the public constructor once at
entry. This simplifies their representation, but alternating runs showed no
consistent large-input speedup: encoding stayed within 1%, little-endian
decoding within 1%, and big-endian decoding improved about 2%. The flag change
is not presented as a performance fix for the remaining gap.

## Word arithmetic and measurement controls

`word-ops-string-dispatch-2026-11-15.json` records an initial tiny-operation
sweep dominated by repeated workload-name comparisons in the timed loop.
The driver now resolves a workload to an unboxed code before starting the clock;
numeric dispatch, mutation, the loop and checksum consumption remain timed.
The independent SDK driver uses the same arrangement. This harness correction
is not a standard-library performance improvement. Reports with different driver
hashes must not be used to attribute a compiler change.

`word-ops-before-divrem-2026-11-15.json` and `word-ops-2026-11-15.json`
use identical driver and library sources, before and after immediate `Nat`
division/remainder lowering. A zero divisor, heap operand and out-of-width
immediate retain the checked runtime path. `interleaved-word-divrem-2026-11-15.json`
alternates the retained executables for 100,000 iterations per process, with
one warmup and nine measured samples. Local medians improve roughly 2–4%; raw
samples and the load-only control remain visible. Assembly confirms immediate
constant remainders become masks or reciprocal multiplies, while cold runtime
calls remain present. This does not remove ownership and dispatch costs.

`word-ops-proven-2026-11-15.json` adds proof-bearing division. It measures about
114 ns per operation versus 148 ns for the checked-result form and a 100 ns
load/checksum control in this local sweep. The operands and observable checksums
match. The tuple form permits the compiler's unboxed worker return; the checked
form keeps its named error results. Assembly still contains a portable
`__udivti3` helper for generic double-width division and two constructor allocations
in the checked form. No claim of instruction-level or SDK throughput parity is
made: these are small-field public pipelines, so whole-buffer MiB/s is omitted.

## Explicit optimization and 128-bit hash coverage

`interleaved-word-divrem-O2-2026-11-15.json` rebuilds the retained matching
before/after LLVM at `-O2`, links both with the same runtime archive, and
alternates nine measured runs after warmup. The report records LLVM, runtime and
executable hashes. Compiler validation runs on CPUs 1–4 while timing uses CPU 0.
This separates the immediate-natural division change from changing optimization
levels; it does not compare an unoptimized baseline to an optimized result.

The 128-bit FNV APIs return high/low U64 pairs. Their kernels use full i128
arithmetic internally and consume eight ordered bytes per block; short tails
remain exact. A single-message FNV recurrence cannot be reassociated into SIMD
lanes. The public carrier works with both compilers and on 32-bit targets.
Correctness tests compare independent SDK digests, arbitrary previous states,
every two-chunk split, block tails and a one-MiB buffer under leak validation.

`optimized-stdlib-2026-11-15.json` is the fresh explicit-O2 baseline for 27
public pipelines at three input sizes (81 rows). `fnv128-words-O2-2026-11-15.json`
uses longer sampling for the two new hash variants. Both include matching
independent SDK checksums for the supported reference workloads. Raw samples
show variation between runs; neither report changes the historical measurements.

Wide-value compiler regressions separately verify that borrowed calls retain
i128 return temporaries while releasing arguments, and that extracted shared
helpers retain i128 returns. A 200,000-step loop runs without intermediate
wide boxes or stack growth; this is tested independently of the pair-based
FNV API.

## Hexadecimal direct output and prefix APIs

`hex-direct-output-2026-11-16.json` measures the unchanged public workload
driver with direct decoded output. The decoder stores each SIMD block in a
single bounded buffer, replacing the per-byte list and its final traversal.
Prefix and append APIs preserve completed pairs and original byte error offsets;
caller-buffer operations validate capacity before mutation and snapshot overlap.
The text convenience retains its all-or-error result.

`interleaved-hex-direct-output-2026-11-16.json` alternates retained executables
built with the same frozen compiler, driver, runtime and `-O2`; only Text.Hex
source differs. It includes one warmup and nine measured samples at each size,
matching output checksums and an encoder control. A short encoder wrapper was
removed after the control exposed added call overhead. Raw samples and both
library identities remain available instead of treating timing as a CI gate.

In this paired run, decoding medians fell from 1.48 to 0.492 µs for 32 bytes,
46.3 to 6.30 µs for 1 KiB, and 691 to 83.7 µs for 16 KiB (about 8.3× at the
largest size). The encoder control has a 16% slower 1 KiB median, while its
32-byte and 16 KiB medians are within 4%. Its loop instruction sequence is
identical after symbol/label normalization, whose hashes are recorded in the
report. These pipeline measurements do not establish an encoder speedup.


## Hexadecimal buffers, dumps and streams

`hex-complete-O2-2026-11-16.json` measures twelve public pipelines at 32 bytes,
1 KiB and 16 KiB, with seven samples after warmup. Independent SDK pipelines
match each observable checksum. Append checksums observe bytes beyond the fixed
prefix; destination checksums observe the written interval; dump checksums include
input-dependent columns after the fixed address. Adapter construction, snapshot
semantics, formatting and output materialization remain part of each pipeline.
These are functional API comparisons, with different allocation strategies.

`hex-complete-before-swar-scatter-O2-2026-11-16.json` retains the initial dump
implementation. Four-pair SWAR masks now interleave already SIMD-encoded digits
and spaces into eight-byte and four-byte stores. The ASCII column classifies
eight lanes at once; exact short tails retain scalar behavior. No block overreads.
`interleaved-hex-swar-scatter-O2-2026-11-16.json` alternates both retained
executables, verifies compiler/driver/optimization/runtime source identities,
and checks output equality. At 16 KiB its local medians improve about 1.56× for
whole dumps and 1.50× for streaming dumps. The unchanged byte encoder control
ranges from 6% slower to 3% faster across sizes. Unpaired sweep differences
include larger control variation and must not be attributed to the scatter change.
The reusable `tooling/bench_stdlib_compare.py` performs these matched checks.

`qualified-calls-space-2026-11-16.json` records a separate self-built compiler
probe: decode a buffer while requesting one output byte at a time. Saturated
source calls with runtime evidence now pass their full checked argument prefix
directly rather than allocating curried application chains. Source tail groups
also include evidence prefixes in their shared worker prototypes, with new values
retained before old roots are released. The 64 KiB probe completes with an 8 MiB
stack, 328,149 peak live heap bytes, and zero leaks; both earlier compiler variants
exhaust that stack. At 1 KiB cumulative allocations fall from 1,635,738 to 568,191
bytes. Retained LLVM and compiler hashes identify each variant.

These live-space measurements disable RC debug quarantine. Poisoning runs keep
freed blocks deliberately and therefore report cumulative diagnostic storage in
the heap peak. Functional adapter tests run separately with normal 32 MiB live
heap/8 MiB stack bounds and a 256 MiB poisoned-block quarantine budget, checking
zero leaks in both modes. The larger diagnostic budget does not establish a live
memory bound. Owned-dictionary, partial/excess-application and mixed-arity mutual
tail tests separately validate evidence ownership and calling conventions.


The evidence-call regressions also check observable application-stage ordering.
The Rust elaborator previously flattened an effectful function-returning call
and its excess argument into one application frame, so an argument marker ran
before the function's own marker (`12F` instead of `1F2`). It now retains a
strict binding after an effectful arrow when further arguments remain. Pure
arrows retain their flattened call path. A second regression checks multiple
stages through a local alias; ordinary, partial and owned-dictionary calls keep
their ownership checks. These are correctness checks, independent of the
retained throughput reports' frozen compiler identities.
