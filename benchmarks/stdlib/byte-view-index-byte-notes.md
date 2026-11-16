# Owner-sealed exact-byte queries

`byte-view-index-byte :: ByteView -> U8 -> Option Nat` finds the first current
matching byte relative to the checked view. Empty views and absent bytes return
`None`. Every byte value is compared directly. UTF-8 validation, repair and
scalar boundaries do not participate in this byte operation.

The adapter retains the existing owner around the existing bounded SWAR worker.
It adds no raw pointer, backing accessor, buffer copy, unsafe primitive or base
dependency. The public source changes by one export and one documented adapter.
The complete original `2b7bfa41…` source is preserved separately. The initial
Split source `2b534f95…`, public Split driver `c511a744…` and unselected counter
candidate `b68124de…` remain distinct epochs.

Independent reference generation uses the pinned 1.27.4 SDK. A second Python
implementation checks every ordered result: 344 owner recipes, 95,616 logical
spans and 22,423,296 exact byte queries. Short owners enumerate every legal
subspan and all 256 byte values. Larger owners cover all first 32 alignments,
midpoints, terminal offsets, complete 8/16/24/32-byte fringes and long inputs.
All input bytes are checked by SHA-256 before observing the query fingerprints.

The full corpus plus repeated temporary-owner and mutable-alias controls passes
Rust `b004049b…`/runtime `c4b1097b…` O0/O2 and Native
`2f07707e…`/provider object `46a7e57f…`/runtime `80bd5b54…` O0/O2. All 2,832
ordinary/poisoned memory profiles report zero leaks with fixed 32/256 MiB heaps
and 8 MiB program stacks. Compiler main-thread stacks are separately 64 MiB.
Native O2 links the exact hash-verified O0 IR with the same provider pair.

Actual full public-adapter programs emit and compile to Wasm32 and AArch64
objects. Coverage checks inspect the named adapter, its bounded block/tail
workers and their exact word, byte and matching-lane bodies. The host executes
the same public program at O0/O2 under both memory profiles. It reaches an
unaligned complete-word match, a scalar-fringe match, an absent full scan and an
empty range. The parent Rust target adds the complete PowerPC big-endian object
gate using the later supported-convention/local-binding compiler repair; the
older frozen `b004` compiler does not contain that repair. No extracted-kernel
object stands in for the public adapter.

The first object assertion mistakenly required an endpoint comparison kernel's
`%x/%a` load spelling. The retained full Wasm IR showed the single-byte worker's
actual `load-word`, `load-byte` and lane-mask chain. The repaired assertion checks
those exact named definitions. Production code did not change for that repair.

The public benchmark uses one executable per frontend, selecting the method at
runtime. It compares a current one-byte view snapshot plus pattern construction
against the exact-byte API, for a first query and complete nonoverlapping-query
pipelines. Input and delimiter bytes change before each operation. Setup,
retained-owner hashes and final owner destruction are outside the clocks; every
per-query allocation and normal result cleanup is inside. Both frontends pass
all 816 ordinary/poisoned preflight processes, including all complete-block
fringes. Exact executable, IR, source closure and provider identities are checked
before reuse.

Five alternating paired samples plus an excluded warmup complete in 3.38 seconds
with no additional cgroup throttling. The quota is four CPUs; root CPU3,
codegen CPU1 and shared JSON/Bufio CPU2 could run while CPU0 was exclusive and
the shared CPU4 work was paused. Raw samples and all regressions are retained.
At 65 KiB the dense complete-query pipeline changes from 243.9 to 127.5 us in
Rust (1.91×) and 1,053 to 364.5 us in Native (2.89×). At 128 bytes, first-query
ratios range 2.83–3.65× in Rust and 1.45–1.69× in Native. Long misses are mostly
scan-bound; Rust's long missing-byte first-query median regresses to 0.928×.
These are measured pipelines, not universal timing guarantees.

Retained assembly confirms that the exact-byte adapter has no byte-buffer
allocation/copy or pattern construction call. An `Option` result may allocate.
The Native baseline lookup retains explicit snapshot and pattern-construction
calls; the direct branch reaches the existing bounded worker. This helper is a
standalone dependency. Integrating it into Split and evaluating any counter
change are subsequent, separately measured changes. Complete strings/bytes and
full standard-library coverage remain open.
