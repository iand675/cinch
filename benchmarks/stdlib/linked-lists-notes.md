# Mutable linked-list evidence

`Data.Mutable.LinkedList` implements stable element handles, checked navigation,
insertion/removal/movement, independent value mutation, bounded self-copy in both
directions, construction, clearing and capacity reservation. Handles retain their
own nodes and identity tokens; they do not retain the root or other list nodes.
Removed/cleared handles remain readable and writable, while structural operations
reject stale or foreign membership. Arena indices are reused; immutable insertion
serials do not wrap or reset.

Eight independent reference traces cover sizes 0, 1, 2, 3, 8, 17, 64 and 257. Each
has 96 actions, with exact action results and a complete post-action checksum of
both forward/backward sequences, every retained value, both membership relations,
both checked navigation directions and alias identity. The fixture reproduces
byte-for-byte. Additional controls exercise detached owned byte buffers after
root destruction, replacement of retained values, unbounded capacity refusals and
full-width raw machine-word payloads.

All nine consumers pass normal 32 MiB heap / 8 MiB stack and separate 256 MiB
poisoned-free runs with zero live/leaked objects on the frozen Rust frontend and
the corrected Cinch frontend. The Native corpus also passes after O2 relinking.
The compiler host uses a 64 MiB stack for the deliberately large observation
fixture; generated programs retain the 8 MiB runtime limit. Parent Cargo and the
complete self-built Native fixed-point gates remain pending. The corrected
frontend was compiled by the frozen Rust compiler; it is a separate compiler
epoch from the final parent bootstrap.

The five public pipelines are append/drain, prepend/drain, move every prepared
handle to the front, remove/reinsert with slot reuse, and copy/append-self/prepend-
self. The last pipeline produces four copies of its source. Values and checksums
use U64; every output value contributes in order. The move/reuse/copy pipelines
materialize their complete output before hashing: an immutable list in Cinch and
a slice in the reference. Those representations and reclamation costs differ.
No allocation or throughput parity is claimed.

The immutable input, prepared list and caller-owned handle vector are built
before the clock and observed again after the final clock. Their teardown is
excluded. Nodes constructed inside an operation belong to that complete public
pipeline, including its ordinary reclamation. The reference uses the same
post-clock observations. The 32 MiB heap budget applies to Cinch; the reference
uses its normal allocator/collector with one processor.

Each timing report contains 15 rows (32/1024/16384 values), 16 iterations, five
samples and one excluded warmup. CPU0 was reserved for timing; compiler and
validation work on other assigned CPUs shared memory/caches. Reports retain
process snapshots and exact compiler, source, executable, IR and assembly hashes.
Independent checksums pass for all five pipelines at sizes 0/1/32/257/1024/16384.

Code inspection found `copy-members` constructing an element handle and public
success wrapper for every copied node, then immediately discarding both. The
private worker now links the allocated node directly. In the 16K copy pipeline,
ordinary process allocation probes record 131072 fewer allocations and about
3.67 MB less cumulative allocation on each frontend. Peak live memory is
unchanged: approximately 12.58 MB for Rust and 14.16 MB for Native, including
preparation and startup. The ten-byte artifact-path difference also appears in
whole-process byte/peak accounting.

Separate timing windows varied even for unchanged workloads, so the worker
comparison uses an additional interleaved baseline/revised/reference run in one
window. At 16K source values, median copy-pipeline time changes from 18.99 to
12.68 ms with Rust and 56.36 to 48.21 ms with Native (about 33% and 14% faster).
Reference medians in that paired run are 5.43 and 5.72 ms. See
[paired timing](linked-lists-copy-paired-2026-11-16.json). The full-window reports
are retained as measurements of their own windows, not controlled worker-only
comparisons.

The optimized IR reports record the remaining call operands, not inferred dynamic
allocation counts. Ownership validation still observes identity-token lengths
and compares natural values; generic result/constructor paths and reference-count
work remain visible. Dependent linked traversal and ordered mutations are serial.
Index and flag operations use existing inline machine-word primitives; arena
growth uses the ownership-moving bulk slot transfer rather than a scalar copy
loop. No pointer-dependent SIMD speedup is claimed.

The [baseline reconstruction patch](linked-lists-before-direct-copy.patch), applied
to a temporary copy with `git apply --unidiff-zero`, reproduces the exact retained
baseline library hash. The timed driver is identical across both source versions.
