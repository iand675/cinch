# Current combined-source public pipeline performance

The exact330/d91 annotated both-effect candidate passes both matched scalar/SIMD source builds,224normal/poison preflights and actual LLVM/optimized assembly clock-boundary gates. Root authorized one exclusiveCPU0 run after pausing its release group and peer work; ownCPU1 builds were held. All288 warmup/sample processes match complete output and retained-input/context digests; all192Cinch processes report zero leaks. The cgroup reports zero throttled periods/usec during the slot.

Each condition retains five samples plus a warmup, interleaved across SIMD, scalar and pinned go1.27.4. No sample or regression is removed. These are complete public operations including fresh outputs and output traversal. Original input preparation and post-clock retained-input/context traversal are excluded on both sides. Cinch mutable allocation and the SDK immutable-string/GC/runtime policies differ. The richer callback factories have separate live functional gates; this driver measures complete reader/writer/disposition pipelines.

| Pipeline | Bytes | SIMD ns/op | Scalar / SIMD | SIMD / SDK |
|---|---:|---:|---:|---:|
| read | 64 | 11865.8 | 1.530 | 1.633 |
| read | 4096 | 25145.8 | 9.529 | 1.701 |
| read-header | 64 | 13101.7 | 1.615 | 2.014 |
| read-header | 4096 | 33774.2 | 7.699 | 1.645 |
| read-qp | 64 | 19231.4 | 1.660 | 2.735 |
| read-qp | 4096 | 249364.6 | 2.945 | 5.612 |
| write-body | 64 | 2992.0 | 1.015 | 2.465 |
| write-body | 4096 | 7821.8 | 0.989 | 0.777 |
| write-sparse | 64 | 3607.4 | 1.030 | 1.380 |
| write-sparse | 4096 | 31847.3 | 1.318 | 1.487 |
| write-dense | 64 | 8437.7 | 0.977 | 4.248 |
| write-dense | 4096 | 382799.1 | 0.968 | 7.222 |
| disposition-sparse | 64 | 2153.2 | 1.097 | 2.917 |
| disposition-sparse | 4096 | 65558.5 | 1.208 | 2.241 |
| disposition-dense | 64 | 11958.0 | 0.979 | 6.507 |
| disposition-dense | 4096 | 824333.9 | 0.970 | 9.172 |

At4KiB, matching SIMD speeds up the same-source scalar read/header/QP paths by9.53x/7.70x/2.95x. Dense output paths remain2–3% slower than scalar in this run, and the4KiB body-write control is about1% slower; these small differences and all raw samples stay visible. SDK gaps are substantial for some operations, including quoted-printable and dense disposition output. No parity, cross-epoch compiler improvement, current Native admission or package-complete claim follows from this run.

`performance-results.json` contains exact selected sources, SDK/toolchain, all raw paired observations and quota evidence. Benchmark binaries, IR, assembly, source/foreign/runtime/reference inputs remain plain. Historical f0e8 performance artifacts and Source/Native family evidence are untouched.
