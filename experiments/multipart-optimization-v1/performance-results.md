# Isolated optimization performance comparison

All384 interleaved warmup/sample processes agree on complete output and retained-input/context digests; all288 Cinch processes report zero leaks. Root explicitly isolatedCPU0 after the parent build and peer work closed. cpu.stat records zero throttled periods/usec. Counters wereOFF for timing; allocation diagnostics remain a separate counter-enabled correctness epoch.

The first standard350MiB launch paused before any program/sample. Root then explicitly authorized existing sealed binaries only with300MiB start/250MiB reserve and≤5MiB newreport. Actual report growth was145,288bytes; source/binary/ABI/fixture contracts and default350MiB build guard remained unchanged.

| Pipeline | Bytes | Optimized ns/op | Baseline / optimized | Scalar / optimized | Optimized / SDK |
|---|---:|---:|---:|---:|---:|
|read|64|12459.8|1.017|1.520|2.340|
|read|4096|31167.0|0.924|8.178|2.043|
|read-header|64|14108.8|0.967|1.719|2.377|
|read-header|4096|33337.2|1.172|8.129|1.339|
|read-qp|64|19302.6|1.096|1.754|2.251|
|read-qp|4096|177990.3|1.639|3.901|3.799|
|write-body|64|3085.7|1.039|0.993|2.353|
|write-body|4096|8885.9|0.884|0.865|0.727|
|write-sparse|64|3491.2|1.628|1.014|2.365|
|write-sparse|4096|18847.4|1.814|1.912|0.854|
|write-dense|64|4330.5|2.041|0.988|1.985|
|write-dense|4096|89775.8|4.643|1.106|1.698|
|disposition-sparse|64|1723.1|1.315|1.070|2.279|
|disposition-sparse|4096|33661.7|2.113|2.100|1.026|
|disposition-dense|64|3593.9|4.200|1.014|1.912|
|disposition-dense|4096|186242.2|4.223|1.047|2.392|

At4KiB, the targeted dense disposition and decoded-body pipelines are4.22x and1.64x faster than the same-compiler sealed baseline. Dense writes improve4.64x; sparse writes/disposition improve1.81x/2.11x. The separate allocation controls show dense disposition~24,597→22 allocations/op and decoded bodies~7,392→2,240. Those facts support the targeted allocation changes; they do not establish blanket attribution for every row.

The4KiB raw-read and body-write controls are about8% and13% slower in this run;64B header reading is about3% slower. All raw paired samples remain, and these controls need code/counter review before selection. Differences across independently generated whole-program layouts or short samples are not automatically dismissed as noise. SDK gaps remain, including2.39x for dense disposition and3.80x for decoded bodies. Mutable outputs/transport/runtime policies differ, so no SDK parity claim is made.

The candidate remains separate and unselected pending parent gates/control review. All baseline and candidate measured sources/binaries/IR/assembly/provider/runtime/reference inputs stay plain. No current Native family or package-complete claim follows.
