# Focused projector confirmation: production unchanged

The longer-loop run completes all 60 observations in 7.103 seconds under the granted 20-second exclusive CPU0 lease. Each process consumes 16 changed inputs. Five conditions each have one excluded warmup and five rotated/interleaved samples per variant. Both binaries use immutable C2/d91 and identical sources except f516. All complete output/retained-owner hashes and leak controls pass. Quota is four CPUs, all peer heavy work is held, and throttling counters do not change.

Before timing, all 25 new-count preflights pass: 20 Cinch normal/poison processes and five independent reference processes. Those preflights discard elapsed fields. The timing uses ordinary RC, 32 MiB heap and 8 MiB stacks; CLI compile-time checked-RC instrumentation is false.

| Condition | Baseline median µs | f516 median µs | Baseline / f516 | Baseline range µs | f516 range µs |
|---|---:|---:|---:|---:|---:|
| eager dense 4096 | 816.312 | 814.955 | 1.002 | 777.777–845.190 | 742.980–825.196 |
| eager unicode 4096 | 495.283 | 481.692 | 1.028 | 460.289–608.874 | 439.801–497.699 |
| eager dense 65536 | 13620.137 | 13460.683 | 1.012 | 13356.983–14020.131 | 13108.511–14983.024 |
| eager unicode 65536 | 7902.198 | 7530.484 | 1.049 | 7593.060–8222.506 | 7392.519–7596.087 |
| visit dense 65536 | 12936.197 | 12007.989 | 1.077 | 11948.947–14548.364 | 11639.717–12575.498 |

The earlier 4 KiB and visitor regressions do not repeat. Dense 4 KiB is effectively neutral in its sample median. Unicode 64 KiB retains a modest gain, and visitor dense 64 KiB improves in this run. Dense 64 KiB has overlapping ranges and a small median difference. These observations do not establish a universal gain.

Increasing from four to sixteen pipelines changes the allocator warmup and changing-input mixture within each process. This is a separate confirmation epoch: all original 104 samples, their outliers and the old706 negative result remain untouched. No observations are clipped. Raw samples, the sealed plan and all 25 preflight observations are committed separately.

The actual ABI and allocation findings from the previous receipt remain unchanged: aggregate returns use three registers without a hidden result pointer or larger scanner frame, one adapter path consumes the aggregate directly, and two expanded paths still rebox. The timing alone does not establish the cost of those rebox instructions. Production UTF8 remains f1e87; f516 remains unselected pending review and affected semantic/ownership qualification. No further runs were made.
