# UTF8 owner-touch reassociation: not selected

The candidate preserves the owner through all bounded loads and enables existing aggregate returns, but every Fields scanner call immediately reconstructs `DecodedStep`. It does not remove the main allocation cost. Production UTF8 and Fields remain unchanged.

The matched Rust706/d91 public run completed all 104 observations in 2.66 seconds:13 workloads, two binaries, an excluded warmup and three rotated/interleaved samples. Complete output and retained-input hashes, plus leak checks, passed. All peer heavy work was paused, affinity was CPU0, quota was four CPUs, and throttling counters did not change. The report-only 300 MiB start/250 MiB reserve exception did not alter the 350 MiB build guard.

| Pipeline | Bytes | Baseline median µs | Candidate median µs | Baseline / candidate |
|---|---:|---:|---:|---:|
| eager text | 4096 | 1.477 | 1.337 | 1.105 |
| eager dense | 4096 | 774.943 | 866.140 | 0.895 |
| eager unicode | 4096 | 487.316 | 491.603 | 0.991 |
| eager text | 65536 | 4.504 | 4.071 | 1.106 |
| eager space | 65536 | 4.614 | 4.712 | 0.979 |
| eager dense | 65536 | 13888.439 | 14891.465 | 0.933 |
| eager unicode | 65536 | 7670.185 | 8021.210 | 0.956 |
| producer text | 65536 | 5.210 | 4.639 | 1.123 |
| producer dense | 65536 | 14433.532 | 13491.176 | 1.070 |
| visit text | 65536 | 5.037 | 4.830 | 1.043 |
| visit dense | 65536 | 13206.504 | 13083.243 | 1.009 |
| predicate text | 65536 | 4375.839 | 4875.686 | 0.897 |
| effect unicode | 65536 | 7700.033 | 7805.577 | 0.986 |

Ratios above one favor the candidate. Dense eager, Unicode eager and generic predicate sample medians are worse; producer dense is better and visitor dense is near neutral. The microsecond ASCII runs contain large individual outliers. Three samples do not establish the cause of every difference. All raw samples remain in `stdlib-byte-fields-cpr-benchmark-samples.json`.

An independent existing-binary allocation diagnostic uses normalized argv0 and counts 0/1/4 for dense eager, Unicode eager, generic predicate and effectful Unicode. All 24 observations preserve complete output/owner hashes and report zero leaks; elapsed values are discarded. Matched allocation totals are identical for the first three workloads at every count. Effectful Unicode is identical at counts 0/4; at count 1 the candidate uses 14 fewer blocks and 276 fewer bytes. These are whole-process totals, not timed-region measurements. Raw kind/size census output is retained in `stdlib-byte-fields-cpr-allocation-diagnostics.json`.

Actual linked O2 assembly covers each scanner entry through the next distinct function, including the private tail body. Its static constructor allocation sites move from 12 to 15 per scanner while the aggregate decoder drops from 4 to 0. This is consistent with allocation moving to the caller. Those code-site counts are distinct from dynamic runtime counts. Exact assembly hashes and scope are in the results JSON.

The next investigation is a general compiler treatment of known field projectors and local product bindings, preserving strict evaluation and ownership. This candidate is kept as a measured negative result; no historical Native adoption or current checked-RC API claim is made.
