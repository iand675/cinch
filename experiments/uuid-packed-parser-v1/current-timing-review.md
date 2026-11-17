# Matched packed-parser measurements

Exact `ad6e72f9` compiler and `d91ce012` runtime; identical original harness and closure except Data.Uuid, same immutable SDK reference. All 180 observations pass. The 0.586-second protocol retains one warmup and five measurements per variant. Raw quota counters show no additional throttled periods or microseconds during the captured interval.

| Workload | Baseline ns/op | Packed ns/op | Reference ns/op | Packed/baseline |
| --- | ---: | ---: | ---: | ---: |
| parse form0 | 500.98 | 320.65 | 58.31 | 0.640 |
| parse form1 | 296.25 | 148.13 | 52.76 | 0.500 |
| parse form2 | 508.49 | 325.39 | 77.32 | 0.640 |
| parse form3 | 585.32 | 306.25 | 66.84 | 0.523 |
| parse form4 | 679.62 | 163.20 | 32.95 | 0.240 |
| format | 233.89 | 237.87 | 59.39 | 1.017 |
| append prefix0 | 236.08 | 241.76 | 61.91 | 1.024 |
| append prefix64 | 236.21 | 251.43 | 85.88 | 1.064 |
| append prefix4096 | 336.99 | 356.80 | 728.35 | 1.059 |
| compare | 5.40 | 5.93 | 17.59 | 1.098 |

Parsing improves, while the unchanged controls are 2–10% slower in this window. Several sample ranges are wide. Normalized control-loop instructions match; layout and machine variation remain possible. These are measurements of each workload, without a broad improvement or production selection claim. Every warmup, sample and regression is retained in the companion JSON files.

Allocation diagnostics independently show canonical parsing decreasing from 16.002 to 10.002 blocks and from 444.069 to 300.069 bytes per operation under (4096−1)/4095 subtraction. The String copy and fresh observable value snapshot remain included. Allocations for the other public loops remain effectively unchanged.
