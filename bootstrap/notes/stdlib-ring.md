# Mutable circular sequences

`Data.Mutable.Ring` provides nonempty typed cursors and uses `Option (Ring r a)`
for an empty sequence. Every public observation or mutation stays in the same
`Mut r` scope. Node identities survive linking, splitting, and arena growth;
an old cursor continues to refer to its original node.

The chosen source has passed the complete functional and ownership corpus on
frozen repaired Rust `b004049b` with matching runtime `c4b1097b`, and on the
Rust-produced experimental Native `645c5185` with runtime `80bd5b54`.
`../evidence/stdlib/stdlib-ring-validation-results.json` records 43 memory/observation processes
per frontend, all with zero leaks. The experimental Native result is not a
self-built distribution claim. The next merged self-built Native family and
parent `lib_ring` Cargo target remain pending.

This source requires the sealed finite-Array positivity extension and the
frame-noncapture repair (validated Rust production revision `0a1f6283`). The
older compiler source in this isolated library lane does not provide those
dependencies; this checkpoint stays local for parent integration rather than
publishing a compiler-incomplete lane.

## Surface and semantics

The reference audit covers the complete `container/ring` operation surface.
The public module's documentation describes its own API without depending on
that reference implementation.

| Audited operation | Cinch surface | Semantics and adaptation |
| --- | --- | --- |
| Zero-value initialization / New | `ring-singleton`, `ring-new`, `ring-new-sized` | Typed explicit initialization replaces lazy object initialization. Natural zero and nonpositive signed sizes return `Ok None`; a singleton is nonempty and self-linked. Values have an explicit typed initializer. |
| Next / Prev | `ring-next`, `ring-prev` | Return a stable neighboring cursor without mutating links. |
| Move | `ring-move` | Accept an arbitrary mathematical integer; normalize large distances before narrowing. Small distances walk directly. |
| Link | `ring-link` | `None` leaves links unchanged. A present cursor joins cycles or splits an existing cycle, preserving all aliases. Return the first cursor's original successor. Linking a node to itself isolates it; linking its current successor leaves the cycle unchanged. |
| Unlink | `ring-unlink` | Nonpositive counts return `None` unchanged. Positive counts remove the count modulo the cycle length and return its original successor; a positive multiple of the length leaves links unchanged. |
| Len | `ring-length` | Count the current cycle, or zero for `None`. |
| Do | `ring-for-each` | Visit through current links at most the original cycle length. Value mutations are observed. Structural callback changes can alter the visit set but cannot create an unbounded traversal. |
| Value field | `ring-value`, `ring-set-value` | Read or replace the current node's value; cursor identities and value cells remain independent. |
| Additional construction | `ring-from-list`, `ring-copy` | Preserve source order. Copy only the current cycle into fresh node/value cells while retaining ordinary shared value owners. |
| Additional observation | `ring-same`, `ring-to-list`, `ring-retained-length` | Compare stable node identity, obtain immutable values, or inspect all nodes retained by a joined arena. |

Construction and cross-arena joins report `RingCapacityExceeded` before a
logical size/layout overflow changes any links. The dense slot limit accounts
for the target's signed allocation range, array metadata, and extended block
headers. Resource exhaustion for a representable allocation follows the
runtime allocator's normal behavior.

## Representation, ownership, and costs

Each node holds a typed value slot and two raw link indices. Neighbors are
indices rather than owning node pointers, so a circular sequence creates no
automatic reference-count cycle. Private arenas hold the dense node storage,
an identity token, and one initially empty parent slot. Only trusted library
code accesses these slots; the parent is initialized before its forwarding
flag is published and is never read while uninitialized.

Joining distinct canonical arenas moves the smaller complete arena into the
larger arena. Every transferred node's link indices receive the destination
offset. The old arena then retains one permanent forwarding edge and its
storage becomes empty. A forwarding edge always points to an arena whose
retained population is at least twice the smaller arena's population at the
time of transfer. The edges therefore form a directed acyclic graph with
logarithmically bounded forwarding depth. Cursor canonicalization accumulates
the offsets while retaining every visited owner.

The move includes disconnected cycles in the smaller arena. This preserves
old aliases even when a previously split component is later joined elsewhere.
It also means that any surviving cursor retains every node in its joined
arena, including disconnected components. `ring-retained-length` exposes that
cost. Copying the desired cycle and releasing old cursors permits unrelated
components to be reclaimed. A copied value cell is independent, while a value
such as a mutable byte buffer can still be ordinarily shared.

After cursor resolution, stepping, identity checks, value access, and
same-arena link updates take constant time. Length and conversion traverse one
cycle. A move of at most 64 positions walks directly; larger moves scan the
cycle once, reduce the full integer distance, and walk the shorter direction.
A cross-arena join costs the smaller retained population plus amortized
geometric storage growth. Copying a cycle costs its current length. Splitting
does not automatically shrink or partition the backing arena.

Pointer chasing and dependent link updates are serial. The independent
previous/next offset additions use an inline LLVM two-lane integer vector;
the explicit integer packing is independent of host endianness. No ownership or pointer word enters this SIMD operation. Dense storage transfers use the
runtime's bulk ownership move rather than per-node reference-count copies.

## Independent verification and measurements

The independent pinned-SDK generator records eight initial sizes and 96
operations per size: 768 complete traces. Observations include both traversal
directions from every retained original node, cursor identity, saved aliases,
current component lengths, callback observations, and value mutations.
The consumer derives no expected values from the implementation under test.

Separate controls cover huge signed movement, empty and rejected constructors,
self/adjacent links, positive-multiple unlink, forwarding of disconnected
components, structural callbacks, fresh copies, full-width machine values,
managed payload owners, and aliases that outlive their initializer/root
owners. The scalar-lane kernel fixture uses 49 independently computed pairs,
including zero, high-bit values, and wrapping limits.

`tooling/validate_rings.py` freezes exact source, compiler, consumer, LLVM, and
executable hashes. Every consumer requires zero leaks with a normal 32 MiB
heap / 8 MiB stack and a separate poisoned-free 256 MiB heap / 8 MiB stack.
The largest complete observation trace allocates 1.89 GiB cumulatively while
its normal live peak is about 72 KiB. Poison quarantine deliberately retains
those freed observation blocks, so its complete post-state checks run in 24
fresh processes, four actions per window. Each process replays every prefix
action and checks every replayed result/callback, then performs exhaustive
state observations only in its window. All 96 expected states remain covered;
the full normal trace is unchanged. One parameterized executable serves all
windows, without rebuilding or increasing the poisoned budget.
The compiler's recursive Array owner control is included separately; checker
acceptance does not substitute for runtime owner validation.

The prepared benchmark harness measures complete changing operations:
construction and traversal, growing singleton joins, balanced joins with all
saved aliases, repeated split/rejoin, and detached-cycle copying. Inputs and
retained root/cursor owners are prepared before the clock and observed after
it. Their destruction is excluded; allocations and reclamation performed by
the operation remain included. Timing receipts must identify background work,
memory profiles, independent checksums, and exact artifacts. The reference
implementation uses a different object/GC representation, so timings alone
do not establish representation-independent cost parity.

The Go 1.27.4 reference generator reproduced all 768 fixture actions exactly;
its executable, source and fixture hashes are retained in
`../evidence/stdlib/stdlib-ring-reference-validation-results.json`.

## Benchmark review and rejected extraction experiment

Five complete pipelines were checked at sizes 0, 1, 2, 32, 1,024 and 16,384:
all 30 conditions matched the independent reference, with separate small
poisoned-free controls. The initial interleaved timing receipt used generated
out-of-line RC checks for Native and ordinary generated RC for Rust, so it is
an explicitly qualified exploratory comparison, not normalized compiler-speed
parity. It also used each executable's own path as its startup program name.
That receipt is preserved as `../evidence/stdlib/stdlib-ring-interleaved-benchmark-results.json`.

The paired offset kernel retains a real `paddq` in the Native offset loop.
An experiment added private inline LLVM low/high extraction helpers to remove
three provider calls. The loop became smaller (37 to 34 calls, 56 to 40 bytes
of stack) with the same three spills/reloads, but the first whole-pipeline
timings did not show a gain. A second rotating-order run used matched ordinary
RC builds, an identical `argv[0]`, nine measured samples plus warm-up and 16
iterations. Parent archive/Cargo/doc work was held during that bounded timing
window. Its medians in milliseconds per complete pipeline were:

| Pipeline / elements | Provider extraction | Inline extraction | Rust control | Reference control |
| --- | ---: | ---: | ---: | ---: |
| Growing joins / 1,024 | 1.234 | 1.232 | 0.695 | 0.053 |
| Growing joins / 16,384 | 23.960 | 25.246 | 14.803 | 1.169 |
| Balanced joins and aliases / 1,024 | 2.875 | 2.994 | 1.350 | 0.069 |
| Balanced joins and aliases / 16,384 | 55.347 | 55.736 | 29.853 | 1.223 |

There was no repeatable extraction gain, so those helpers were reverted. The
chosen module keeps the paired SIMD arithmetic and the existing provider
extraction calls. The rejected candidate, its complete correctness corpus,
portable objects, exact source patch, and both timing receipts remain
reviewable. No speedup is claimed from the smaller assembly alone. The exact
ordinary-RC repeat is in
`../evidence/stdlib/stdlib-ring-ordinary-normalized-benchmark-results.json`; its per-artifact
profile and source hashes distinguish it from the exploratory run.

Allocation diagnostics for one balanced 16,384-node operation show about
901,234 Native header blocks / 26.6 MB cumulative allocation / 8.3 MB process
peak, versus 540,784 Rust header blocks / 16.5 MB cumulative / 8.3 MB peak.
These process totals include startup, preparation, and teardown; they are not
operation-only allocation counts. The Native loop still contains repeated
owner retain/release traffic, and this representation moves smaller arenas
and resolves forwarding aliases while the reference uses directly linked,
traced objects. Those differences remain material costs. The new scalar
Native lowering may improve ordinary machine operations in a later family;
it does not remove the three wide provider calls, and no later-family
performance result is inferred here.

The chosen numeric kernel was also re-emitted by frozen Rust `b004049b`
for AArch64 and wasm: both retain the independent vector addition in LLVM
and compile to real target objects. Exact source/IR/object hashes are in
`../evidence/stdlib/stdlib-ring-portable-kernel-validation-results.json`. The maintained six-test
`lib_ring` target covers the full traces, managed owners, numeric kernel and
this actual object emission. Runtime behavior on those cross targets is not
claimed by object compilation.
