# Matched projector compiler comparison: source still unselected

Both variants use immutable C2/d91, unchanged public driver 84d6 and the same library sources except the single f516 owner-touch placement. Both 650-observation preflights and 24 ordinary allocation diagnostics pass complete output/retained-owner checks with zero leaks. CLI checked-RC instrumentation is disabled; poisoned memory is independently enabled for preflights.

The exclusive CPU0 comparison completes 104 observations in 2.635 seconds:13 workloads, two variants, one excluded warmup and three included rotated/interleaved samples. All peers are held, quota is four CPUs, and no throttling counters change. All raw samples, including outliers, remain in `../evidence/stdlib/stdlib-field-projector-benchmark-samples.json`.

| Pipeline | Bytes | Baseline median µs | f516 median µs | Baseline / f516 |
|---|---:|---:|---:|---:|
| eager text | 4096 | 1.600 | 1.545 | 1.036 |
| eager dense | 4096 | 907.578 | 1164.666 | 0.779 |
| eager unicode | 4096 | 546.912 | 611.334 | 0.895 |
| eager text | 65536 | 4.344 | 4.276 | 1.016 |
| eager space | 65536 | 4.727 | 4.264 | 1.109 |
| eager dense | 65536 | 14000.127 | 13138.733 | 1.066 |
| eager unicode | 65536 | 8189.305 | 7292.000 | 1.123 |
| producer text | 65536 | 5.123 | 5.448 | 0.940 |
| producer dense | 65536 | 14751.855 | 13268.129 | 1.112 |
| visit text | 65536 | 4.800 | 5.065 | 0.948 |
| visit dense | 65536 | 12845.321 | 13459.936 | 0.954 |
| predicate text | 65536 | 4521.850 | 4260.431 | 1.061 |
| effect unicode | 65536 | 8182.876 | 7858.341 | 1.041 |

Ratios above one favor f516. Default eager and producer dense 64 KiB medians improve, as does Unicode eager 64 KiB. Dense/Unicode 4 KiB and visitor dense 64 KiB medians regress. Overlapping raw ranges prevent treating every median difference as a repeatable gain or regression. Generic callback allocation totals remain unchanged, so their modest timing gains are not allocation-removal evidence. Production UTF8 remains unchanged.

The current inliner removes both scalar getter calls. The candidate can consume one aggregate View-adapter path directly; two expanded decoder branches still reconstruct the step. Four-iteration allocation totals drop by 131072 blocks / 4194304 bytes in dense eager and 87380 blocks / 2796160 bytes in Unicode eager. Generic predicate/effect totals do not drop. These are matched whole-process totals, not clock-region allocation measurements.

Actual x86-64 O2 code returns three words in RAX/RDX/RCX; there is no hidden result pointer. The boxed decoder returns one pointer in RAX. Decoder saved-register prologues match, as do View 0x18, scanner-space 0x38 and scanner-with 0x58 stack reservations. Remaining rebox sites preserve the three returned words across a 24-byte constructor allocation and then store them. This is actual extra work at those paths, without a causal claim about the noisy timings. Full symbol-range assembly hashes are in the JSON receipt.

A proposed confirmation uses five conditions: eager dense/Unicode at 4 KiB and 64 KiB, plus visitor dense 64 KiB. It raises each process from 4 to 16 changed-input pipelines and retains an excluded warmup plus five interleaved samples. The 60 observations are expected to fit a 20-second lease; the existing launcher will preserve partial samples if the lease is exhausted. Before timing, the new count needs existing-binary output/owner/leak preflights and the reused independent reference; those remain unrun.

Observed per-operation sample ranges motivating confirmation (µs):

| Condition | Baseline range | f516 range |
|---|---:|---:|
| eager dense 4096 | 871.354–1607.998 | 763.489–1248.545 |
| eager unicode 4096 | 514.991–547.198 | 399.718–627.685 |
| eager dense 65536 | 13037.453–14572.337 | 12742.011–14129.276 |
| eager unicode 65536 | 7642.301–8389.993 | 7186.370–7353.488 |
| visit dense 65536 | 12785.991–13218.022 | 11711.995–13488.380 |

The proposed 16-iteration five-sample run is a confirmation plan, not evidence. The observed maximum samples imply roughly 8 seconds of measured process work plus setup/hash/launch overhead, within a 20-second ceiling. This changes the within-process allocator warmup mixture, so it will be a separate epoch and will not replace or clip the original 104 samples.
