# Initial bounded byte views and compiled substring search

`Data.Bytes.View` adds opaque mutable `ByteView` aliases and opaque immutable-by-
interface `BytePattern` snapshots. A view retains the entire fixed owner, even
when its own range is small or empty. Overlapping views and the original buffer
observe shared mutations. Snapshot and clone results are fresh; compiled needles
and their failure tables have no outward mutable aliases.

Ranges are half-open. Owner ranges and relative subranges return explicit
reversed-range or past-end errors. Single-byte writes report refusal without
mutation. Complete copies report the destination offset, required count and
capacity on atomic refusal, and use the checked overlap-safe runtime range copy.
Take and drop clamp their counts. Private owners and addresses are never exported.

The module exposes first/last/contains/count, exact prefix/suffix, first/last cut,
prefix/suffix cut, equality and normalized lexicographic comparison for views.
Raw-needle convenience functions coexist with reusable compiled-pattern variants;
whole-buffer convenience functions return retained view aliases for cuts.
Raw prefix/suffix queries need neither snapshots nor pattern allocation.

Compiled substring search uses a proper-prefix failure table and linear traversal.
Last-match search retains overlaps; count advances past each complete match.
Empty patterns match index zero and last index length. Their count is the number
of bounded decoded scalar steps plus one: each malformed or span-truncated byte
advances one step. Empty first/last cuts produce `(empty, whole)` and
`(whole, empty)` respectively. Missing cuts return `None`.

Private local LLVM arithmetic keeps counters unboxed. First-byte skipping uses
exact eight-byte SWAR lane masks; matching prefixes compare complete eight-byte
blocks before exact scalar fringes. Checked lengths and retained owners surround
the private traversals. Failure entries use checked little-endian stores and
endian-normalized reads. No writes or control-flow branches are hidden in inline
assembly or LLVM blocks.

The independent SDK fixture and separate Python byte/scalar model agree on 5,421
rows: 5,376 short spans plus 45 large malformed/repeated-prefix cases. The complete
corpus and nested alias/refusal/snapshot owner cycles pass final cad70 Rust O0/O2
and managed-code Native586 O0/O2. All four gates identify the same View source
SHA-256 `7f18724c3a082f1ced03418dea52b85ebe590460f369ff49b61e764d1a60b98c`.
Native O2 reuses the hash-verified O0 IR and explicitly relinks it at `-O2`.
Each run uses ordinary 32 MiB or poison 256 MiB heap limits, an 8 MiB program
stack and leak checking. Compiler main-thread stack allowance is separately
64 MiB. The expanded positive subrange-composition/empty-copy checks have focused
Rust O2 and Native O0 ownership gates as well.

The current Native baseline is the fully gated managed-code586 snapshot with its
matched provider/runtime; its older embedded String epoch is not a new String
provider claim. View, raw UTF8 span helpers and fixed-word helpers are explicit
source dependencies. The exact raw UTF8 span dependency is published checkpoint
`899168d5d6d17d48fd1a319687c5755159c902e5`.

All 98 declarations, including private workers and constructors, have canonical
documentation with no parser diagnostics or orphan comments. Actual production
kernels compile to wasm32 and AArch64 objects. The supplied six-test Rust target
`lib_byte_view` adds the big-endian PowerPC object gate and isolated source-closure
consumer tests; parent integration owns running Cargo. This checkpoint does not
claim the full strings/bytes package surfaces are complete.

Functional observations and exact artifact identities are recorded in
`bootstrap/evidence/stdlib/stdlib-byte-view-results.json`. Public changed-input scalar/packed
pipeline measurements are a separate follow-up and make no claim here.


# Selected endpoint and suffix refinement

The selected implementation is source SHA-256
`c55e57028c56d673d75c29ba91936ee614c3631addda328abe75c12a1bbf3e7e`.
The public surface, error values, owner retention and mutation behavior remain
unchanged. The source adds seven documented private declarations (105 total).

Last-index checks an exact suffix first: a matching suffix proves the final
possible occurrence immediately. General first-index first uses the existing
fast first-byte scan, then filters impossible initial starts with intersected
first/final-byte SWAR lane masks. Empty first-byte masks return to the fast
scanner. The displaced complete load is bounded by the exclusive start limit
`end - (needle_length - 1)`. Only the initial start region is filtered; the
complete KMP failure traversal begins once, preserving linear work even when
endpoints agree but the interior fails late. Needle block cutoffs are loop
invariants; haystack availability retains exact fringe checks.

The complete 5,421-row corpus plus 62 new endpoint/interior-mismatch controls
passes all four Rust/Native O0/O2 gates. Independent SDK and Python generators
agree on the new inputs, their complete SHA-256 digests and their query answers.
All 2,280 normal/poison processes, including repeated temporary-owner and
composed-subrange checks, report zero leaks with the same heap/stack caps.
Production kernels compile to wasm32/AArch64 objects; host O0 controls
independently observe four matching packed bytes and endpoint match offset eight.
The parent owns the updated seven-test Rust target and PowerPC64 object gate.

`bench_byte_view.py` records complete public pipelines with a reused compiled
pattern and checked runtime input mutation. Initialization/input digests precede
the first clock; complete retained-input and stable-pattern digests follow the
ending clock. Assembly retains checked mutation and public observation calls
inside work-loop backedges under both frontends. Five alternating samples plus
an excluded warmup cover all 32 conditions per frontend; all results and leaks
are checked. The final runs total 1,152 measured/warmup processes, with 384
separate normal/poison preflight processes.

For the 65,537-byte workloads, the selected complete pipelines are 2.94–45.85x
faster than the scalar matching reference under Rust and 2.50–4.67x under Native.
The shared-prefix first-index case is 12.05x/9.84x faster respectively. The
Native repeated-prefix count alone remains slower (0.81x scalar in the selected
sample); generic KMP state/batch predicates add branch work. Small/constant-work
results and unchanged-function layout effects vary. These are shared-machine
measurements, not universal claims, and do not include per-query pattern
construction costs for one-shot convenience calls.

The initial two-cutoff variant reduced byte-loop arithmetic but increased live
register pressure and spills around Native skip calls. An always-used endpoint
loop then regressed Rust sparse scans. The selected loop reuses the better
optimized fast scan: its actual assembly is three unrolled eight-byte SWAR
operations, not vector SIMD. All original, intermediate, rejected-baseline and
selected sources/samples are retained. The rejected scalar control incorrectly
disabled prefix/suffix equality; the independent preflight caught it before
performance measurement. Exact selected functional/performance/object identities
are in `bootstrap/evidence/stdlib/stdlib-byte-view-selected-results.json`.
