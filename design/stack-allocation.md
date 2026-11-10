# Stack allocation of non-escaping blocks

Cinch guarantees enough about lifetime that a large share of its
allocations do not need the heap: values are owned, borrows are scoped to
regions, effects are explicit, and tail calls are structured. This note
turns those guarantees into a frame allocation pass, because the measured
allocation volume says that is where the next large win is.

## 1. Why: the measurement

`CINCH_ALLOC_STATS=1` on the self-hosted compiler (Stage3, `--check` with
the pinned base interfaces):

| Input | Blocks | Bytes | Value blocks | Value bytes |
| --- | --- | --- | --- | --- |
| `module Main where; main :: Int; main = 0` | 89.8M | 2.17 GB | 78.5M | 2.06 GB |
| 3,965-line toolkit bundle | 464.1M | 10.97 GB | 444.8M | 10.70 GB |

Average block 24 bytes: tiny constructors, tuples, and closures. A
two-line program already costs 90M allocations, which is interface
loading; the toolkit costs about 94K allocations per line on top, which is
the checker. The runtime's size-class heap serves 96% of these in about
20 ns each, so allocation is a large constant factor on every phase, and
alternative system allocators cannot help (measured, no win; the traffic
never reaches them).

A stack block costs a `sub` on the frame pointer and no free, and it
removes the header write, the count traffic and the collector's scan. The
same measurement on a program that allocates in a loop has to be taken
again per program, but for a compiler-shaped workload the win is
proportional to the volume.

## 2. What the language already gives us

- **Ownership.** Every value carries a class (`owned`, `borrowed`, `lent`,
  `uncounted`, `immortal`), and `rc::insert` already proves which parameters
  can be borrowed and where blocks are uniquely owned.
- **Regions.** `Ref r a` and `Mut r a` are scoped to a region bound by a
  scope form, and the checker already refuses a borrow that outlives its
  region (`design/ownership-and-usage.md` section 4). A frame allocation is
  the degenerate region that is one activation.
- **Structured concurrency.** Tasks are scoped by `withScope` and joined
  before the scope ends, so a value that never crosses a `spawn` cannot
  outlive the frame that made it.
- **Tail calls.** A self tail call reuses the frame, and the runtime caps
  the stack, so a frame-allocated block inside a loop is safe exactly when
  it does not escape the iteration.

## 3. The analysis

A Core block allocation (`Ctor`, `Lam`'s environment, a tuple, a `Let`-bound
dictionary that captures) is **frame-eligible** when every use of the value
is within the allocating activation:

1. It is not returned, not stored into a field of a value that escapes, not
   stored through `RefSet`, not passed to `spawn` or a foreign call, and not
   captured by a closure that escapes. Formally: not part of the
   activation's escape set, computed bottom up with the same lattice the RC
   pass already uses for ownership.
2. Every reference is dropped or moved within the activation. A block whose
   last use is inside a loop iteration is eligible per iteration only if the
   value is not live across the back edge; the pass therefore places the
   frame slot in the entry block and treats the slot as reused, which is
   sound exactly under that condition.
3. Its size is known at emission and below a threshold (default 4 KiB),
   because a dynamic size must stay on the heap or in a dynamic frame alloc,
   and a very large object belongs on the heap regardless.

The pass runs after RC insertion, where the count operations are explicit,
so it can rewrite the pair "allocate, dup, drop, free" into "frame slot,
no count, no free" and leave every other count untouched. It is a local
analysis per unit with a summary for callees: a callee whose parameter is
never stored or returned is recorded as not-escaping in the unit summary,
so interprocedural flow does not need a whole-program fixpoint to start.

## 4. Lowering

- The block's frame slot is an `alloca` in the entry block, 16-byte aligned
  and rounded to an even word count, for the reasons `compiler/src/codegen.rs`
  already documents for dynamic allocas (the AArch64 probe-stack
  interaction).
- The block header is written once at the slot with a new
  `RC_FRAME` count (a value no correct program can produce on the heap), so
  every `dup`, `drop`, `release` and `Scan` path can treat it as immortal:
  no count write, no free, and the reclaimer skips it. That keeps the
  runtime's count paths unchanged, which matters because they are shared
  with heap blocks.
- A frame-eligible block that is nonetheless dropped on a path the analysis
  could not prove is a compile error in a checked build (the verifier's
  invariant), and a `CINCH_RC_DEBUG=1` build asserts it at run time.
- Stack depth: each unit's frame slots are counted, and a unit whose frame
  would exceed a limit (default 16 KiB) keeps its allocations on the heap.
  The runtime already reserves large stacks and reports overflow, so this is
  a bound, not a new failure mode.

## 5. Where it belongs in the two compilers

- **Rust seed** (`compiler/src`): a new `stack.rs` pass between `rc::insert`
  and `specialize::optimize`, plus codegen support for the frame slot and
  the `RC_FRAME` header. The Core text output gains a `@frame` annotation on
  the unit and an `(alloc-frame ...)` node form so the decision is
  inspectable, which is what the Core output work exists for.
- **Cinch compiler** (`bootstrap/`): the same analysis in
  `Bootstrap.RuntimeIr`, where allocation is already emitted, with the same
  `RC_FRAME` convention. The two must agree on which blocks are frame-bound
  only if the bootstrap's fixed point compares `owned` Core; it compares IR,
  so agreement is required on the emitted IR, which the fixed point already
  checks per stage but not across compilers. Land the Rust side first and
  gate the Cinch side on matching IR for the toolkit.

## 6. Interactions and risks

- **Espaced-closure check.** A frame-allocated closure's `env` holds pointers
  into the caller's frame. The whole point of the eligibility rule is to
  refuse that case, so the analysis must treat "closure stored anywhere"
  as escape even when the closure itself is local.
- **Regions and `Mut`.** A `Mut r a` inside a scope is already region-bound;
  the frame pass should produce the same answer and the two mechanisms must
  not disagree. Where they do, the region checker is authoritative.
- **Leak accounting.** Frame blocks never enter the live-bytes counter, so
  `cinch-leak: live=0` still holds and `CINCH_HEAP_LIMIT` does not count
  them. The frame bound is what keeps a program from trading heap for stack
  without limit.
- **Debugging.** A frame block's address changes per activation, so a
  pointer into one is valid only within it; that is exactly the invariant
  the analysis proves, and `CINCH_RC_DEBUG=1` checks it.

## 7. Plan

1. Measurement first: the allocation attribution lane reports blocks by
   kind, size, phase and caller. The eligible fraction is an upper bound on
   the win, so compute it from that data before writing the pass.
2. Rust seed: `RC_FRAME`, the eligibility analysis, codegen, Core text
   annotation, and a verifier rule. Acceptance: the compiler's own bundle
   self-compile keeps byte-identical IR when the pass is disabled, drops
   allocation volume by the estimated fraction when enabled, leaks stay
   zero, and the compiler's own speed on the toolkit improves.
3. Cinch compiler: the same, gated on matching the seed's emitted IR for the
   toolkit bundle.
4. Then widen the eligibility rule with the interprocedural summary, and
   measure again.

Related designs: `ownership-and-usage.md`, `reclamation.md`,
`compilation-speed.md`, `layouts-and-specialization.md`,
`core-output.md`, `parallel-evaluation.md`.

## 8. As built (first landing, 2026-11-10)

The Rust seed implements the pass, `compiler/src/stack.rs`, with the
runtime convention in `RC_FRAME`. This section records what the
implementation decides, where it departs from the sketch above, and what
it does not yet catch.

### 8.1 The header value

`RC_FRAME` is `RC_MAX`, the largest value the 40-bit count field can
hold (`compiler/runtime/src/block_kinds.rs`). No heap block can reach it:
the heap cap bounds the live words of the process far below `2^39`, and a
count only ever moves by one from `RC_FRESH`. Every count path therefore
treats a frame block exactly like an immortal one, with no second
representation and no change to the heap layout:

- `cinch_dup`, `cinch_drop` and `cinch_free` ignore it; the release walk
  never descends into it; `claim` (the spawn mark) stops at it.
- `cinch_drop_slow` refuses it under `CINCH_RC_DEBUG=1`, and `free_in`
  traps there rather than handing stack memory to the allocator: both are
  compiler-bug traps, and the compiler elides the operations that would
  reach them.
- The inline count helpers each module emits gain one `RC_FRAME` arm, and
  only in a module whose program has a frame slot. A module the pass left
  alone emits the helper it always emitted.
- A frame block never enters the live-bytes counter or the leak count, so
  `cinch-leak: live=0 immortal=0 leaked=0` still holds and
  `CINCH_HEAP_LIMIT` does not see it.

### 8.2 The rule as implemented

Only a `let`-bound allocation is a candidate (a saturated constructor
call, or a closure). The binder is the `let`'s, which the count pass
already treats as one owned reference, and the allocation is eligible
when every reference to that binder is one of:

- the scrutinee of a `case` (a borrowed read),
- the container of a `Proj`/`DictProj` (one field is read and handed out
  as an owned reference; the block keeps its own),
- an argument of a **borrowed** parameter of a saturated direct call
  (`Program::borrowed`, the pass's own borrow inference): the callee reads
  it and neither stores, returns nor drops it, so the reference stays the
  caller's own,
- a `Drop` or a `Release`, which end it.

Anything else refuses: a returned value, a constructor field, an argument
to an owned parameter or a join, a closure environment slot, a
`RefSet`/`BoxInit`/`LazyInit` store, or a call through the value itself.
The binder must also never be `Dup`ped, its class must be counted, its
constructor must not have a boundary layout (`repr(C)`, `repr(packed)`,
`repr(transparent)`), its payload must be at most 4 KiB, and the unit's
slots together at most `--stack-frame-limit` (16 KiB by default).

Rule 1 and the borrowed-argument rule are the ownership lattice's facts,
not a second analysis: after `crate::rc::insert` every owned binder is
consumed exactly once on every path, so "every reference is a read or a
death" is also "the reference never leaves the activation". A value stored
into any block at all is refused, including a frame block: that
restriction is what keeps a frame block's release a flat list of classes
(its payload fields are never themselves frame blocks), and it is the
first thing to widen.

### 8.3 Lowering

The slot is one entry-block `alloca [n x i64], align 16` with `n` rounded
up to an even count, so the payload sits at an 8-mod-16 offset exactly as
a heap block's does. The header is written once per execution, with
`RC_FRAME`, the ordinary kind, and the ordinary layout descriptor (the
constructor's layout id or the closure's shape index; an escaped
descriptor also writes its word at -16). The binder's `dup` and `drop`
are elided, and its death releases the payload: an ordinary `drop` per
counted field, in reverse payload order (D390), with no free and no scan.
`Release`'s death is the unique path of a drop specialisation without the
free: the moved fields keep the references the block held, the rest are
dropped, and the runtime `unique` test is not emitted at all, since a
frame count can never be one.

### 8.4 Inspectability

`--dump-core` prints `@frame(<n> slots, <bytes> bytes)` on a unit with
frame slots and `@frame(f<slot>)` on each lowered allocation node
(`design/core-output.md` section 10). The artifact manifest gains a
`stack` object: `enabled`, `units`, `slots`, `bytes`, `candidates` and
`refused` by reason (`layout`, `form`, `size`, `duplicated`, `escapes`,
`frame-limit`). `CINCH_STACK_TRACE=1` prints each unit's decision and
refusal reason to stderr. `--no-stack-alloc` turns the pass off, and
`--stack-frame-limit` sets the unit bound.

### 8.5 Measured reach

The pass is only as useful as the share of allocation sites whose
reference dies in its own activation, and the measurement says that share
is smaller than the "96% do not escape" figure suggests, because
"does not escape" is much weaker than "dies here".

| Workload (Rust seed, `emit-ir`) | Candidates | Slots | Bytes | Refused |
| --- | --- | --- | --- | --- |
| 3,964-line toolkit bundle, simplifier on | 128 | 5 | 160 | 122 escapes, 1 duplicated |
| 39,953-line self-hosted compiler source | 2,888 | 1,024 | 33,280 | 1,371 escapes, 493 duplicated |

The refusal breakdown on the self-hosted compiler is the useful number
for what comes next: 786 sites escape as a value position and 476 more are
`Dup`ped and then returned, against 299 stored in a block, 256 passed to an
owned parameter, 25 captured by a closure and 5 called through. Returned
values and their duplicates are 68% of the refusals, and no per-unit rule
can take them. A block a unit returns lives
in its caller's frame only if the callee is inlined at the call site,
which is the interprocedural step: a unit whose body is one saturated
constructor call over its parameters can be substituted at a saturated
call site, and then the ordinary rule applies in the caller's position.
`CINCH_STACK_TRACE=1` reports the exact reason per site, so that work
starts from data rather than a guess.

What the lowering is worth where it does fire, measured on a program that
builds and matches one three-field block per iteration, 100,000
iterations, `CINCH_ALLOC_STATS=1`, simplifier off (so the loop is real):

| Arm | Blocks | Bytes | Live-peak bytes |
| --- | --- | --- | --- |
| `--no-stack-alloc` | 400,006 | 10,400,218 | 234 |
| pass on | 300,006 | 7,200,217 | 201 |

100,000 blocks and 3.2 MB of churn gone, 25% of the blocks and 31% of the
bytes, with `live=0` either way. So the lowering pays exactly where the
rule fires, and the rule does not fire on the compiler's own shape. The
note's estimate of the eligible fraction was optimistic: "does not
escape" is much weaker than "dies in its own activation", and the lever
the volume measurement points at is the returned-block case, which needs
inlining rather than a wider escape rule.

The pass's own cost is the walk plus one clone of the borrow-mode map:
`emit-ir` on the toolkit medians 10.05 s on and 9.70 s off over five
interleaved runs (median of 5; the spread inside each arm is 1.5 to 3 s,
so treat the 3.5% as the upper bound of a small cost), and peak RSS is
unchanged at about 225 MiB. `check` is 4.09 s on and 4.18 s off: check
mode never reaches the pass, so that comparison is noise, and the honest
statement is that the pass cannot change the compiler's own frontend
speed at all. Its win is in the runtime of a compiled Cinch program,
which is why the self-hosted compiler is where it has to be measured.

### 8.6 What this pass does not do

- It does not inline a callee's allocation into the caller (section 8.5).
- It does not frame-allocate a duplicated block: a second reference would
  need a second release of the payload, and a frame count cannot make one
  of them a no-op.
- It does not frame-allocate a word block whose reference is `Dup`ped only
  to be dropped later on the same path.
- It does not lower dictionaries (`InstDict`), memo cells (`LazyAlloc`),
  knots (`BoxAlloc`), boxed numbers or byte buffers; they need either an
  inlined factory or a payload the pass does not own.
- It does not put a frame block in another block's field, so recursive
  frame structures wait for a recursive release plan.
- It does not run inside a quotation's retained body, which is emitted as
  a code unit of its own.
- The Cinch-side compiler has not implemented the same analysis yet
  (`bootstrap/Bootstrap.RuntimeIr`); until it does, a bootstrap fixed point
  compares IR the seed produced with the pass on against IR the bootstrap
  produces without it, so the bootstrap build has to either implement the
  pass or the seed has to compile the ladder with `--no-stack-alloc`.
  Nothing in this landing gates the ladder; the orchestrator decides.

## 9. The inliner (second landing)

Section 8.5 said the remaining volume needs inlining: 68% of refusals were
a callee's returned value. `compiler/src/simplify.rs` now carries the
`inline` rule, which runs in the simplifier's pass loop (so its work is
ticked and capped with the other rewrites) and therefore *before* the
count pass and before stack allocation: an inlined body's allocation is a
`let`-bound allocation at the call site, which is exactly what
`crate::stack` already understands.

What it does: a unit inlined at its single call site. The conditions are
that the unit is referenced nowhere as a value, is not a member of a
recursive strongly connected component of the call graph (Tarjan, with a
self edge counting, which is what makes the pass terminate), carries no
quotation and no join, has no unresolved parameter class, binds no local
of its own (no `let`, no pattern binder, no join: the copy then needs no
renaming), and its body is at most `inline_threshold` (40) nodes. Every
argument that is not a local is bound to a fresh local in evaluation
order, so nothing moves past an effect and a parameter read twice reads
the same word; a local argument is substituted directly. A node budget of
2% of the program caps the growth, and nothing is removed: the callee keeps
its body, because interfaces and other artifacts name it.

The local half of the occurrence analysis lives in the pass itself: a
`let` bound to a trivial value (a local, a literal, a unit name, a
primitive or a foreign symbol) whose binder has one reader is substituted,
which is safe anywhere because such a word has no effect and no
allocation to move into a branch or a loop body.

Reported: `inlined` (sites) and `inlined_nodes` in the manifest's
`simplifier` object, `@inline(<n> sites, <m> nodes: <callees>)` on a
caller in `--dump-core`, and `CINCH_INLINE_TRACE=1` per site on stderr.

Measured reach: 12 sites and 66 nodes on the toolkit bundle, and the
stack pass's reach there did not move (5 slots, 128 candidates, 160
bytes), because the toolkit's constructor wrappers are already unfolded
by the simplifier's own inlining. On the 39,953-line compiler source the
numbers are in the table below.

The multi-site rule (a body of at most `inline_small` nodes, with no call
and no allocation, copied at every site) is **not landed**: with it on,
two units' bodies came out in a shape the ownership verifier refuses
(`dup of v<N>, which is not alive here`, and a borrowed read of a binder
the verifier counts as dead), so the small-matcher case is refused and
recorded here as the next slice's first task. The single-site rule above
is what the measurements were taken with.

The honest answer to "did the reach move enough": not yet. The inliner
reaches only binder-free wrappers at their single call site, so it cannot
take the returned-value case the census pointed at whenever the callee
matches its argument (a `case` binds a local, and such a body is
refused). Fixing the multi-site path, then allowing `case`-bearing single
site bodies, is what unlocks those 786 sites.
