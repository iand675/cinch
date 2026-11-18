# Multipart public benchmark checkpoint

The 4 KiB Rust read/header/decoded-part pipelines measured about 6.6×/6.6×/2.6×
faster with bounded SIMD/SWAR classification than with the matched scalar
snapshot. The older admitted Native frontend shows smaller read gains and a
larger sparse-write gain. Dense escaping and the body-write control include
regressions. Most complete pipelines remain slower than the pinned reference;
these results do not establish parity or qualify another frontend epoch.

All measured conditions are below. Values are median time ratios; lower is
faster. Five samples plus a warmup were interleaved across all three programs,
with twenty complete operations per process. The JSON retains every raw sample,
individual output/owner checksum, stderr, compiler/runtime/source/object hash,
background context and unadjusted cgroup statistics. There are 576 timed/warmup
processes: 384 Cinch zero-leak receipts and 192 independent reference checks.
Before timing, 448 separate normal/poison profiles passed sizes
0/1/15/16/17/64/4096 over all eight workloads.

| Pipeline | Bytes | Rust SIMD/scalar | Rust SIMD/reference | Native SIMD/scalar | Native SIMD/reference |
| --- | ---: | ---: | ---: | ---: | ---: |
| read | 64 | 0.773 | 1.508 | 0.887 | 4.446 |
| read | 4096 | 0.152 | 3.250 | 0.828 | 27.197 |
| read-header | 64 | 0.715 | 2.534 | 0.806 | 4.962 |
| read-header | 4096 | 0.151 | 1.682 | 0.785 | 17.329 |
| read-qp | 64 | 0.765 | 3.464 | 0.943 | 9.599 |
| read-qp | 4096 | 0.383 | 6.033 | 0.785 | 30.337 |
| write-body | 64 | 1.014 | 2.496 | 0.995 | 2.289 |
| write-body | 4096 | 1.339 | 0.775 | 1.344 | 1.245 |
| write-sparse | 64 | 0.993 | 2.552 | 0.755 | 4.118 |
| write-sparse | 4096 | 0.890 | 1.523 | 0.422 | 4.447 |
| write-dense | 64 | 1.001 | 4.157 | 1.071 | 6.160 |
| write-dense | 4096 | 0.977 | 8.584 | 1.338 | 20.782 |
| disposition-sparse | 64 | 0.911 | 3.072 | 1.129 | 26.712 |
| disposition-sparse | 4096 | 0.792 | 2.155 | 0.839 | 25.570 |
| disposition-dense | 64 | 0.952 | 6.642 | 1.092 | 26.836 |
| disposition-dense | 4096 | 1.056 | 9.729 | 1.226 | 55.459 |

Cinch allocates fresh mutable read/header outputs; the reference uses immutable
strings and a reusable 4 KiB body buffer. Cinch transport uses a borrowed real C
source and an observing scalar C sink. The reference uses its ordinary public
reader/writer and normal GC policy. Cinch runs use a 32 MiB heap, 8 MiB runtime
stack, ordinary RC compilation, poison disabled and leak checking enabled.
Allocation statistics were off in the timing receipts. Separate counter-on
whole-process diagnostics are retained and include preparation and post-clock
observations; they do not replace timing samples.

Both implementations initialize their reusable validated-boundary writer before
timing. Every operation includes complete output consumption and Close. Actual
LLVM and optimized assembly prove the second clock precedes retained input,
mode and writer-boundary traversal and final teardown. The corrected V2 driver
keeps the writer owner alive through that second clock. Untimed V1 predecessors
are retained separately and contribute no timing claim.

Actual x86 code uses `pcmpeqb`/`pmovmskb`: the Rust matcher remains a named
function, while Native inlines it into the real escape loop. Only complete proved
windows use matching SIMD, with scalar fringes. The scalar snapshot changes
byte/escape/prefix classification while retaining allocation, bulk-copy and
runtime policies. The separate public portability checkpoint covers actual
wasm/AArch64 objects. The parent Cargo gate for big-endian object compilation, extra resource/managed-backend
controls, source-absent admission and the reviewed live buffered adapter remain
separate gates.
