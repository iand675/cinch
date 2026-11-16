# Owned and callback-backed heaps

The [public pipeline measurements](heaps-O2-2026-11-16.json) cover six
operations at 32, 1024 and 16384 elements. Every row matches an independent
reference checksum that consumes the complete popped sequence or both the old
and repaired root. Both variants change input between operations. Five measured
samples follow one excluded warmup; raw samples and exact source/compiler,
executable, LLVM and assembly identities are retained.

The four-way owned queue supports checked indexed replacement, removal and
repair, including priorities stored in mutable values. The binary heap view
calls application-owned length, comparison, swap, append and remove-last
callbacks. Its swap callback can maintain independent position metadata.
Callbacks execute in sequence. Arbitrary callbacks and pointer-dependent heap
paths cannot safely be reassociated into SIMD lanes. Dense storage growth uses
the existing bulk ownership-moving slot primitive.

Preparation and process startup are outside the measured interval. Prepared
queues and the source list are retained through the closing clock read, then
their lengths contribute to the final checksum. The
[initial measurements](heaps-with-prepared-teardown-O2-2026-11-16.json) ended
those owners' lifetimes inside the timed call. That included one linear teardown
of prepared input in the reported updates: the 16384-element owned root update
appeared to cost 27.2 microseconds. With the lifetime boundary corrected, it
measures about 0.96 microseconds. This corrects the measurement; it is not a
library or compiler speedup. The callback-backed update measures about 1.58
microseconds, against roughly 0.10 microseconds for the independent pipeline.

Full 16384-element construction/drain pipelines remain roughly 4–12 times
slower than the independent measurements. The
[generated-code inspection](heaps-codegen-2026-11-16.json) retains exact selected
function hashes and calls: owned sift workers use machine-word indices and
checked slot operations; callback traversal still includes indirect applications,
natural-number paths, and reference-count operations. Static call counts are
not executed calls per element. Reordering visible comparisons or hiding writes
in assembly would change the API's contract.

The [ordinary allocation probes](heaps-space-2026-11-16.json) cover all 18
rows and finish with zero live/leaked allocations. They include startup, the
source list, both prepared queues and one measured operation. They use a 32 MiB
heap and 8 MiB stack with quarantine disabled; poisoned-free validation is a
separate correctness check. The reference scalar representation and allocations
differ, so these results do not establish allocation or throughput parity.

Library validation currently passes the complete Rust CLI corpus: sixteen
initial sequences, 1040 exact stateful callback observations, sixteen owned
repair scenarios, and an additional empty/alias/unbounded-index control. Ordinary
32 MiB and separate 256 MiB poisoned-free runs have zero leaks. The Native gate
remains blocked by existing phantom-region kind inference, effectful local
generalization and mutation-row kind closure. The package remains partial until
those compiler repairs and the rebuilt Native checks pass.
