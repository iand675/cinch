# Retained fields: public pipeline measurements

All four ordinary-profile Cinch SIMD/scalar preflights pass 650 normal/poison
conditions each, with zero leaks; the pinned SDK reference passes 325 independent
observations. Checksums include every ordered field extent and both raw endpoints.
Complete original/retained owner hashes and logical sizes are also checked.

The immutable paired run contains 260 observations: 13 conditions, five binaries,
one excluded warmup and three rotated/interleaved samples, four changed-input
operations per process. CPU0 was exclusive; no cgroup throttling counter changed.
All raw results, including controls and regressions, are in
`stdlib-byte-fields-benchmark-samples.json`.

The columns below are scalar-control time divided by SIMD time. Values above
one favor the packed ASCII run path.

| Pipeline | Pattern | Bytes | Rust ratio | Historical Native ratio |
| --- | --- | ---: | ---: | ---: |
| eager | text | 4096 | 189.182 | 93.475 |
| eager | dense | 4096 | 0.851 | 0.804 |
| eager | unicode | 4096 | 1.019 | 0.949 |
| eager | text | 65536 | 1129.771 | 1144.002 |
| eager | space | 65536 | 753.631 | 915.320 |
| eager | dense | 65536 | 0.872 | 0.989 |
| eager | unicode | 65536 | 0.941 | 0.928 |
| producer | text | 65536 | 967.962 | 1612.394 |
| producer | dense | 65536 | 0.871 | 0.793 |
| visit | text | 65536 | 1008.251 | 1625.517 |
| visit | dense | 65536 | 0.874 | 0.806 |
| predicate | text | 65536 | 0.990 | 0.910 |
| effect | unicode | 65536 | 1.020 | 1.335 |

Long ASCII fields benefit greatly because the packed run path avoids repeated
high-level UTF8 decoding and state allocation. The large ratios measure that
complete pipeline change. Dense short fields and mixed Unicode retain regressions.
Native neutral controls also vary materially: their source never invokes the
changed ASCII helper, so small differences require confirmation.

The SDK comparison retains different result containers and memory-management
models. Dense eager 64 KiB input takes about 0.493 ms in that reference, 12.27 ms
in current Rust and 28.71 ms in the historical Native implementation. This gap
motivates allocation/decoder review; it is not hidden by the long-ASCII gains.

Cinch view retention uses a genuine typed mutable cell created before timing
and read afterward. Actual linked instructions keep that read and complete raw
owner hashing after the second clock. Input creation and retention hashing are
outside timing; complete result consumption and ordinary reclamation are inside.

The current Rust706/d91 and historical Native2f/V4/c4 epochs remain separate.
No new Native family or production optimization is selected. The independent
owner-touch reassociation experiment (f516) is absent from these measurements.
