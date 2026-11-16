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

The preserved portable/interface comparison is also complete: 94 of 106 targets
pass. All twelve failing targets were rerun with the immutable previous native
compiler; their exit codes, failed test names and full normalized logs match.
`bootstrap/stdlib-qualified-call-preserved-target-results.json` records that
comparison. It is additional regression evidence from a frozen self-built
Stage1 candidate, not the audited Stage3 gate or an all-green target set.

## ASCII85 caller buffers and source offsets

`ascii85-complete-O2-2026-11-16.json` measures eleven public buffer and stream
pipelines at three input sizes (33 rows). Every checksum matches an independent
SDK pipeline. The new APIs expose exact written/consumed counts, checked source
and destination offsets, complete prefixes before errors, and overlap snapshots.
Full quanta use SIMD digit arithmetic; eight-lane classification skips runs of
ignored controls without overreading short input. The control workloads prefix
4096 ignored bytes; preparation stays outside the measured loop.

The scalar-control and forced-snapshot reports are reconstructed ablations of
the same API, not historical committed versions. Their paired reports alternate
retained executables built with the same frozen compiler, driver, runtime and
`-O2`, verify checksums and retain source/executable/LLVM hashes. At 16 KiB,
`interleaved-ascii85-control-runs-O2-2026-11-16.json` measures approximately
1.37× improvements in both control-heavy decoding APIs; its ordinary decoder
control improves 1.04×. `interleaved-ascii85-snapshot-elision-O2-2026-11-16.json`
measures 7.51× faster decoding through a reusable four-byte output interval.
Its short encoder controls are 17–24% slower. The retained longer run,
`interleaved-ascii85-long-encoder-control-O2-2026-11-16.json`, uses 512
iterations per sample: byte-encoder controls range from 4.3% slower to 2% faster,
and into-encoder controls from 1.9% slower to 10.9% faster across sizes. Both
16 KiB encoder controls are within 0.3%. These results establish no bulk encoder
speedup; the original short-run variation remains visible.

`ascii85-offset-allocation-2026-11-16.json` separately observes the self-built
Stage3 compiler draining 64 KiB of zero bytes through one four-byte destination.
Checking actual source/destination overlap avoids cloning the whole remaining
source on each call. Cumulative allocated bytes fall from 271,613,980 to
2,785,309, about 97.5×; peak live heap is essentially unchanged at 164 KiB.
Both variants finish with an 8 MiB stack, 32 MiB heap limit and zero leaks.
These counters use normal reclamation, without diagnostic quarantine.

The small-output pipeline still takes about 439 µs versus the SDK's 48.6 µs
at 16 KiB. Inspection of the measured Rust-generated LLVM shows that inlining
and product-return optimization already remove the result/triple allocations;
they do not explain this remaining gap. Reference-count operations, tagged
natural arithmetic and checked per-byte writes remain around each small read.
The separate native allocation probe has different lowering and must not be
used to claim those boxes survive in the measured Rust loop. Buffer semantics,
allocation and representation differ between these public pipelines.

The payload-address inline hint was investigated separately in
`ascii85-payload-inline-experiment-O2-2026-11-16.json` and its interleaved
report. Clang already inlines this wrapper: the entire retained optimized
assembly matches after removing comments (instructions, labels and directives
are retained in that comparison). The compiler change was discarded. The
reported timing variation therefore establishes no codegen improvement.
`bench_stdlib_compare.py --compiler-change` explicitly permits different
compiler identities while requiring identical library sources, driver, runtime
and optimization level. Ordinary library comparisons still require identical
compilers. Pure inline primitives continue to reject visible stores; packed
output stores require a checked mutation intrinsic that preserves containment,
read-only faults and ownership rather than a raw store hidden in a pure row.

## Persistent FNV states, checkpoints and hashing streams

`fnv-state-O2-2026-11-16.json` measures all six FNV variants through fresh
digest bytes, binary checkpoint roundtrips and reader-to-hash copying. It has
54 rows at 32 bytes, 1 KiB and 16 KiB, with seven samples of 64 iterations after
warmup. Every observable checksum matches an independent SDK pipeline. The
stream workload uses 64-byte chunks and includes source snapshots, returned
owned chunks, adapter/state construction and final digest materialization.
The reference reuses a read buffer; these are functional API comparisons with
different ownership and allocation strategies.

`Text.Fnv.State` retains only immutable scalar words. Earlier states remain
valid when branches advance; reset preserves width and recurrence order. Digest
and checkpoint append operations copy their prefix. Checkpoints preserve all
bits in network order after a four-byte variant identifier; loading checks
identity before exact size. Independent fixtures cover 108 complete digest and
checkpoint cases, every two-chunk split, 216 arbitrary restored states, 600
identifier/length cases and 48 independently corrupted identifier bytes.
`Effect.Fnv` keeps the advanced hash and reader observable after source failures,
and its generic writer adapter composes with existing stream consumers.

Generated LLVM exposed a sink wrapper constructed inside each recursive copy
iteration. The copy worker now carries the scalar hash state directly and wraps
it at the public boundary. `fnv-state-before-worker-O2-2026-11-16.json` retains
the earlier implementation. `interleaved-fnv-scalar-worker-O2-2026-11-16.json`
alternates retained executables with identical compiler, driver, runtime,
optimization level and all other library sources. Across all six stream variants,
16 KiB median improvements range from 1.022× to 1.113×. The two digest controls
are within 0.4% at that size; the 1 KiB 64-bit digest control is 7.6% slower.
No timing threshold or bulk digest speedup is claimed.

`fnv-scalar-worker-allocation-2026-11-16.json` separately runs one 64 KiB input
through 64-byte chunks under an 8 MiB stack and 32 MiB heap limit, with ordinary
reclamation and zero leaks. The 32/64-bit variants allocate 510 fewer blocks
and about 12 KiB fewer cumulative bytes; the 128-bit variant allocates 1,022
fewer blocks and about 24 KiB fewer bytes. Peak live heap changes by only 15
bytes, remaining about 129 KiB. These counters include identical setup work.
Constructor reuse already bypasses some allocation sites in the earlier LLVM,
so removing a source-level per-chunk wrapper does not imply one fewer actual
allocation on every iteration. The hashes identify both retained executables.

The ordered eight-byte hash kernels remain unchanged; a single-message FNV
recurrence cannot be reassociated into SIMD lanes. Network-order state encoding
reuses the binary word operations. Remaining streaming costs include owned
input chunks, reference counting and per-chunk state/reader continuations;
checkpoint and digest APIs also allocate fresh buffers. Functional completeness
does not establish equal throughput or allocation complexity.

## Standalone Adler-32 and bounded modular reduction

`Text.Adler32` now owns the portable SIMD accumulator, persistent scalar states
and binary checkpoints. `Text.Checksum` preserves its public Adler-32 functions
as delegates. Keeping the standalone module independent of CRC assembly lets
both compilers exercise this complete checksum/state surface; it does not add
embedded-assembly support to the self-built frontend or repair the CRC facade.

The accumulator previously converted each sixteen-byte vector reduction to
natural numbers and reduced both sums modulo 65521 immediately. It now keeps
machine U32 sums for segments of at most 5552 bytes, then uses constant unsigned
LLVM remainders to normalize. SIMD weighted sums, exact short tails and network
byte order retain their semantics. The generated x86 kernel already lowers to
`pmaddwd` and `psadbw`; no wider ISA or speculative vector-width change is needed.

`adler32-state-O2-2026-11-16.json` records 24 pipeline/control rows, seven samples
of 128 iterations after warmup, at 32 bytes, 1 KiB, 16 KiB and 64 KiB. Scalar
checksums, fresh state digests, checkpoint roundtrips and 64-/4096-byte stream
copies all match independent SDK checksums. The FNV control also matches. Source
snapshots, owned read chunks, continuation states and fresh digest/checkpoint
buffers remain part of the pipelines. The SDK stream loop reuses a buffer.

`adler32-before-segments-O2-2026-11-16.json` retains the committed prior
`Text.Checksum` implementation under the same new driver, compiler, runtime and
optimization level. The standalone state and FNV controls use identical sources
on both sides. `interleaved-adler32-segments-O2-2026-11-16.json` alternates nine
retained-executable samples with matching checksums. Scalar checksum medians at
16 KiB fall from 12.80 to 8.41 µs (1.522×), and at 64 KiB from 51.20 to 33.60 µs
(1.524×). The 32-byte improvement is only 1.063×. Controls vary by up to about 5%
across sizes, and are within 2.3% at 16/64 KiB. This is a local
reduction improvement, not evidence of general throughput parity.

The new independent fixtures contain 46 digest/checkpoint cases, 414 arbitrary
restored-state cases and 60 malformed identifier/size cases. Every two-chunk
split through 257 bytes and selected long splits around 5552/11104 bytes are
checked, including all-ones and full-byte-permutation input. Both compilers
validate normal 32 MiB live heap/8 MiB stack bounds, separately budget poisoned
frees, and report zero leaks for one-byte reads of a 64 KiB source. Existing Rust
stdlib checks also retain ARM CRC and big-endian kernel compilation checks.

Large constant corpus initialization overflowed the Rust compiler stack during
an initial test run. A pure LLVM identity keeps fixture setup at runtime, so the
tests execute the generated buffer code; that compiler issue remains open.
Remaining runtime costs include checked buffer access, natural index bookkeeping,
reference counting, owned stream chunks and fresh state/checkpoint buffers.

## Wide inline LLVM primitive ABI support

The Rust checker and wrapper emitter now preserve `I128`/`U128` parameters and
results, including mixed narrow and floating operands. This permits private SIMD
kernels to return two complete machine words without an intermediate allocation.
Direct calls, escaping partial applications, shared closure captures, object ABI
metadata and a 200000-step wide tail loop are covered by the new Rust regression
target. The self-built compiler independently passes the direct, mixed-operand
and memory-containment cases; its existing wide closure, tail-group and public
primitive object restrictions remain as described in `design/wide-scalars.md`.

[Control samples](prim-wide-controls-O2-2026-11-16.json) cover four existing
workloads at two sizes with independent checksums. More decisively, the complete
retained stdlib driver's LLVM and O2 assembly are byte-identical before and after
the compiler change. No throughput improvement for existing word-only operations
is claimed. [Validation evidence](../../bootstrap/stdlib-prim-wide-validation-results.json)
records the exact compiler/source identities, 297 unit tests, existing primitive
and wide regressions, foreign object compilation and the pinned bootstrap gate.

## Prepared CRC-64 tables, states and streams

`Text.Crc64` prepares an immutable 16 KiB slicing-by-eight table from any
reflected polynomial. SIMD generates two independent base entries at a time;
the x86 output uses packed shifts, masks and XORs. Updates combine one packed
input word with eight masked table entries and consume exact scalar tails.
The table and input owners remain alive around the private traversal. Persistent
states retain their table, expose numeric/network-order digests and reset,
append to copied prefixes, and save/load twenty-byte table-checked checkpoints.
`Effect.Crc64` provides inspectable writes, generic sinks and bounded copying.

[Final samples](crc64-state-O2-2026-11-16.json) cover eight workloads and four
input sizes: polynomial convenience, prepared table, state/digest, checkpoint
round trip, 64/4096-byte stream chunks, construction and an Adler-32 control.
All 32 rows match the independent SDK's observable results. The shared prepared
table is initialized before timing. The explicit build workload and large
polynomial convenience measure their own table construction.
Construction intentionally prepares eight slices and caches the checkpoint
fingerprint; the SDK's public custom-table constructor prepares only the base
row. These construction workloads therefore have different work and storage
costs. Input cloning, per-chunk owned buffers and fresh digest/checkpoint
allocations remain included in their corresponding public pipelines.

The first prepared-table loop still emitted natural-number tag, overflow and
ownership paths within each block. Fixed-width remaining-length counters remove
those paths while preserving complete-word and positive-tail bounds. The
[isolated alternating comparison](interleaved-crc64-word-counter-O2-2026-11-16.json)
measures about 1.56x for the 64 KiB prepared-table pipeline; the 64 KiB Adler
control varied by about 8%. The generated word loop has masked scalar loads,
XORs and tail jumps, with no natural-number runtime calls or owner operations.

The polynomial convenience API now prepares a table from 16 KiB upward, leaving
short buffers on the direct shift-register path. A separate
[alternating comparison](interleaved-crc64-auto-table-O2-2026-11-16.json) measures
1.85x at 16 KiB and 5.56x at 64 KiB for that convenience, including table setup.
Its Adler controls differ by at most 5.7%. Retaining a prepared table amortizes
construction across repeated updates. Results are local paired measurements;
the reports retain every sample and both source identities.

[Ordinary-space probes](crc64-space-2026-11-16.json) measure one complete
64 KiB operation with normal reclamation, including startup, common input
setup and table construction. The prepared-table pipeline peaks at 82288 bytes;
64-byte stream copying peaks at 148073 bytes and allocates 6798 blocks, versus
236 blocks with 4096-byte chunks. Every probe finishes with zero live allocations.
These are complete pipeline counts, not isolated kernel costs. Separate poisoned
runs use quarantine for ownership validation and do not measure ordinary live space.

The [natural-counter baseline patch](crc64-natural-counter-baseline.patch)
and [counter-only patch](crc64-counter-before-auto-table.patch) reconstruct
the measured intermediate sources from this checkpoint's `Text/Crc64.cinch`.
Apply either patch to a temporary copy, check its SHA-256 against the linked
report and use `bench_stdlib.py --source-override Text.Crc64=PATH` to reproduce
that source snapshot. The corresponding unpaired reports are
[initial tables](crc64-natural-counter-O2-2026-11-16.json) and
[counter-only traversal](crc64-counter-before-auto-table-O2-2026-11-16.json).
Compiler, driver, runtime and optimization identities match in each paired
comparison. No allocation or universal throughput parity is claimed.


## SHA-224/SHA-256 states and portable SIMD compression

`Text.Sha256` implements both complete-buffer digests and immutable update,
reset, digest/append and checkpoint APIs. `Effect.Sha256` provides inspectable
writes, generic sinks and bounded copying. The generated compression expands
four 32-bit schedule words in LLVM SIMD lanes, resolving dependent upper lanes
in a second step. Four sixteen-round helpers keep the source below the
compiler's nesting bound; scalar message operands allow the optimizer to inline
the phases. The resulting compression has packed shifts/XORs and 384 scalar
round rotates, with no calls to the small round primitives. It evaluates each
round once. Chaining state uses four ordinary packed words, allowing constructor
reuse; private wide values carry only the SIMD schedule across primitive calls.
Exact word loads and machine remaining-length counters avoid partial-block
reads and natural-number work inside the block loop.

[Public pipeline samples](sha256-state-O2-2026-11-16.json) cover both variants
through whole-buffer, state, checkpoint and 64/4096-byte streaming operations,
plus the existing runtime digest and Adler-32 controls, at four input sizes.
All 48 observable checksums match the independent SDK. Timings include input
cloning, copied stream chunks, prefix/checkpoint buffers and fresh digest
allocation. They exclude process startup and common input/table preparation.
The immutable implementations have different allocation costs from mutable
reference accumulators.

Code inspection found two important lowering effects. A large inline LLVM
compression split into independent return fragments stayed out of line and
repeated work; it was discarded before this implementation. The small round
primitives and sixteen-round source helpers produce one complete compression.
A later whole-buffer path bypasses the temporary streaming state and copied
tail, but extracting a shared variable-size renderer changed inlining and
produced mixed small-input timings. Fixed-size renderers helped whole-buffer
calls but slowed state calls. The final source keeps the state path's original
rendering/finalization shape and uses fixed-size stores in the whole-buffer
path. The [final alternating comparison](interleaved-sha256-final-shape-O2-2026-11-16.json)
measures about 1.17x for each 32-byte whole-buffer variant. Large-buffer and
state cases vary by roughly 0–4%; unrelated controls vary by up to 3.4%.
No improvement for those cases is claimed. The
[shared-render comparison](interleaved-sha256-shared-render-O2-2026-11-16.json)
and [fixed-state-render comparison](interleaved-sha256-fixed-state-render-O2-2026-11-16.json)
retain the regressions that motivated the final shape.

The [same-binary CPU comparison](sha256-reference-cpu-O2-2026-11-16.json)
separates the reference's detected CPU features from `GODEBUG=cpu.all=off`.
On this host, the detected 64 KiB pipeline took about 45 microseconds and the
portable fallback about 304 microseconds; Cinch's portable implementation
was about 246 microseconds in the initial samples. The host advertises SHA-NI,
AVX, SSSE3 and SSE4.1; the SDK selects its SHA instruction path with those
features. Cinch currently uses portable SIMD schedules and scalar rounds.
Small inputs remain slower even against the reference portable fallback.
These results do not establish parity with hardware SHA acceleration.

[Ordinary-space probes](sha256-space-2026-11-16.json) use equal-length executable
paths, normal reclamation, one complete public operation and the same common
setup. At 32 bytes the final SHA-256 whole-buffer path allocates 114 blocks and
19700 bytes, versus 122 blocks and 19944 bytes in the state-based baseline.
The SHA-256 state pipeline allocates the same 114 blocks and 19794 bytes in
both versions. At 64 KiB, 64-byte stream chunks allocate 8342 blocks and peak
at 148135 bytes; 4096-byte chunks allocate 270 blocks and peak at 152171 bytes.
All probes finish with zero live allocations. These complete-process totals
include startup and an unrelated table setup shared by the driver; they are
not isolated kernel allocation counts. Poisoned-free validation is separate.

The [state-only baseline patch](sha256-state-only-baseline.patch),
[shared-render intermediate patch](sha256-shared-render-intermediate.patch)
and [fixed-state-render intermediate patch](sha256-fixed-state-render-intermediate.patch)
reconstruct every measured intermediate from the final `Text/Sha256.cinch`.
Apply a patch to a temporary copy, verify its source hash against the report,
and use `bench_stdlib.py --source-override Text.Sha256=PATH`.
Corresponding unpaired reports retain the
[state-only](sha256-state-only-O2-2026-11-16.json),
[shared-render](sha256-shared-render-O2-2026-11-16.json),
[fixed-state-render](sha256-fixed-state-render-O2-2026-11-16.json) and
[final-shape](sha256-final-shape-O2-2026-11-16.json) artifacts and samples.
Compiler, driver, optimization and runtime source identities match in every
alternating comparison. [Validation evidence](../../bootstrap/stdlib-sha256-state-validation-results.json)
records frontend, ownership, checkpoint and foreign-object checks.
