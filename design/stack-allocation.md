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
  it does not escape the iteration. The same frame reuse is why no frame
  block may cross a call in tail position at all (section 7.6): the callee's
  frame, not the caller's, owns that memory once the call is made.

## 3. The analysis

A Core block allocation (`Ctor`, `Lam`'s environment, a tuple, a `Let`-bound
dictionary that captures) is **frame-eligible** when every use of the value
is within the allocating activation:

1. It is not returned, not stored into a field of a value that escapes, not
   stored through `RefSet`, not passed to `spawn` or a foreign call, not
   passed as an argument of a call in tail position (the callee's frame
   replaces this activation's, section 7.6), and not captured by a closure
   that escapes. Formally: not part of the activation's escape set, computed
   bottom up with the same lattice the RC pass already uses for ownership.
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
  and rounded to an even word count, so every slot keeps the stack a
  multiple of 16 bytes (the AArch64 stack-probe requirement that retired
  the apply stub's dynamic alloca, `compiler/src/codegen.rs`).
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

Implemented (2026-11-10):

- `RC_FRAME` (the count field's `RC_MAX`, unreachable for a heap block because
  the heap cap bounds live words far below it) and every count path treating
  it as immortal, with `CINCH_RC_DEBUG=1` traps for a frame free or a slow
  drop of a frame block.
- The eligibility pass (`compiler/src/stack.rs`) after RC insertion, reusing
  the count pass's own postconditions as the lattice facts, with entry-block
  `alloca` lowering, `@frame` in Core output, `--no-stack-alloc`,
  `--stack-frame-limit`, and refusal reasons in the manifest.
- The inliner (`compiler/src/simplify.rs`) with both rules: a single site for
  a body of at most 40 nodes, and every site for a body of at most 12 nodes
  with no call and no allocation. A body that binds locals is admitted when
  every binder it makes is a `case` pattern binder, which is the shape the
  refusals are in. `admit` names the check that refused each candidate,
  `CINCH_INLINE_TRACE` prints one line per candidate (nodes, sites, free
  locals), `CINCH_INLINE_CHECK` re-derives the copy's closure in a release
  build, and `--no-inline` takes the measurement the other way.
- The duplicated rule: a second reference is admissible when the payload
  holds no counted word, because neither the dup nor the drop can act on
  anything then. `CINCH_STACK_TRACE` prints the counted field count, the dup
  count and the slot's bytes for every candidate.

### 7.1 The inliner copy bug (item 1, closed)

`rename_binders` gave every binder of a copied body a fresh id, and renamed
every count, cell and `release` operation that named one, but its first match
arm was `Core::Var(_) => {}`: it never renamed a *use*. A copy of
`case p of Path s -> s` therefore came out as `case Var(0) of Path Var(f) ->
Var(1)`, reading the original unit's binder id from a unit that does not bind
it, and `crate::rc`'s verifier refused the caller (`dup of v2, which is not
alive here`). A body that binds nothing has no binder to rename, which is why
the rule only ever reached constructor wrappers, and why the bug looked like
a multi-copy or substitution problem rather than a traversal gap.

Two diagnostics found it in seconds, and stay in: the copy's free locals must
be a subset of its parameters after the binder and parameter renaming, and
two binders of one body may not share an id. On the toolkit the first one
printed

```
inline: the copy of Data.Path.path-to-string at Main.f reads free locals
[LocalId(1), LocalId(3)] outside its parameters [LocalId(3)]:
Case { scrut: Var(LocalId(3)),
       alts: [Con { tag: 0, fields: [LocalId(2)], ... } -> Var(LocalId(1))] }
```

which is the missing `Var` arm, named.

The same review found a second, latent bug: the parameters were substituted
one at a time in the callee's parameter id space (`LocalId(0)`, `LocalId(1)`,
...), while the substituted terms are the caller's, whose locals use the same
numbers. `first b 7` inside a two-parameter caller substituted parameter 0
with `Var(1)`, and substituting parameter 1 then rewrote that argument,
so the copy returned the constant `7`. It verified, so nothing caught it.
The parameters are now renamed to fresh ids first and the substitution
replaces those: no caller local can be mistaken for a parameter still to be
substituted, and the copy's closure is the invariant the diagnostic checks.

### 7.3 The frame payload's release, exactly once (item 4)

A frame block has no count that can make one of its drops the last, so the
release cannot be driven by a zero crossing. It is owned instead by the
first path that reaches it, and the header records that it happened:

- The count field holds `RC_FRAME` while the payload is live and
  `RC_FRAME_RELEASED = RC_MAX - 1` once it has been released. Both are
  counts no heap block can hold (the heap cap bounds live words far below
  `2^39`, and a count only moves by one from `RC_FRESH`), so the two states
  cost no extra field and no extra word.
- `cinch_frame_release(word)` reads the header, returns when it is already
  marked, otherwise writes the marker *before* touching a child and then
  walks the block's real kind and descriptor, dropping each counted child in
  reverse payload order with an ordinary `cinch_drop`. It never frees the
  block and never touches the allocator.
- The invariant is not "one path releases" but **the payload's counted
  children are dropped exactly once on every path**. The three paths are
  the binder's static death (code generation's own field-by-field release,
  which marks first too), a callee that took the block as an owned
  parameter and drops it, and a parent frame block whose own release walks
  the payload it was stored in. Marking first makes the second and later
  paths no-ops, which also covers a child that points back into a block
  still being released.
- `cinch_drop` on a frame block calls the release. The inline count helpers
  exclude both markers with one signed compare (`top < RC_FRAME_RELEASED`,
  top-shifted) and route them to a frame arm: the drop arm calls
  `cinch_frame_release`, the dup arm is a no-op, because a frame dup is a
  second static reference and the field is not a count. The arm is present
  only in a module that can hold or receive a frame block, so a program the
  pass left alone (and `--no-stack-alloc`) still emits the helper it always
  emitted.
- `cinch_free`, the spawn mark and the heap release walk leave both markers
  alone rather than decrementing a field that is not a count, and
  `cinch_drop_slow` releases the payload instead of handing stack memory to
  the allocator (under `CINCH_RC_DEBUG=1` it traps, because the debug paths
  go through `cinch_drop`).

What the marker buys is that a frame block may be released from a place its
allocating unit cannot see, which is what the two widenings below need. What
it does not buy is a second *live* reference: a `Dup` of a frame block with a
counted payload is still refused, because the reference that outlives the
release would read dropped children.

The pass's static release stays: the code generator knows the counted byte
offsets at the binder's own death, so that path is a fixed list of drops with
no call and no scan. The runtime walk exists for the block whose origin the
unit cannot see.

Two transfers become admissible, and each needs a fact about a place the
pass cannot see:

- **A frame block passed to a callee's owned parameter** (`escapes:call`).
  Admissible when the callee consumes the parameter: every reference to it
  inside the callee is a read or a death, none is `Dup`ped, and nothing
  stores it, returns it or passes it on (`Facts::consumed`, one scan per
  unit). The callee's own drop is then the release. A parameter the callee
  forwards is not consumed by this rule, so forwarding stays refused: that
  would need a fixpoint over summaries. The call must not be in tail
  position, because the callee's frame replaces the caller's there (section
  7.6): the argument would arrive describing a slot the callee has already
  spilled over.
- **A frame block stored in a field of another frame block**
  (`escapes:field`). Admissible when the enclosing block is itself
  frame-bound, which makes the decision a fixpoint: start with no frame
  binder and repeat, each round taking the candidates the last round's set
  makes eligible. Adding a frame binder can only turn a store from an escape
  into a consumption, so the set grows and the iteration terminates. The
  frame limit is applied as a *shrink* of the fixpoint and the fixpoint is
  re-run under the shrunk set, because a limit inside the iteration can
  oscillate (an earlier slot pushes a later one out, and dropping it
  re-admits the earlier one).

`CINCH_STACK_WIDEN=none|consume|store|all` selects the transfers, so one
widening can be measured against the other; `CINCH_STACK_TRACE` now prints,
on an accepted candidate, how many consumptions it had and how many were
static deaths, stores in a frame block, or passes to a consuming callee.

### 7.4 Dead-unit elimination (item 2)

Inlining moves an allocation into its caller's activation, which is what the
frame pass needs, but a copy leaves the callee's body behind: a unit whose
every call site was inlined is never called again. Those units are code the
artifact still pays for, and they are what keeps the inliner's node budget
(two percent of the program) the binding constraint.

`simplify::prune` is a reachability sweep over the units, run after inlining
and before reference counting, with the program's real roots: `program.main`,
every `foreign export` target and Wasm world binding implementation, every
instance's dictionary unit and its methods, every constructor unit (the
layout table's ids and the tags a `case` compares name them, and another
module's block carries one of those ids), every `prim` declaration's unit,
every boundary layout plan's unit, and every unit of a module an object or
package build emits. A unit is live when a live unit mentions it, directly or
as a `Lam`'s code pointer, in any position including a quotation's retained
body; everything else is removed. The removed unit count and the Core nodes
they held go in the artifact manifest as `simplifier.pruned` and
`simplifier.pruned_nodes`, and `CINCH_NO_PRUNE=1` takes the measurement the
other way.

### 7.5 Measured: which widening paid

`core emit` over the compiler workload (the bootstrapped compiler: 46 files,
40,266 lines, the `Run.Check` entry) with `CINCH_STACK_TRACE=1` and
`--no-inline`, so the population is the pre-inliner units:

| `CINCH_STACK_WIDEN` | Slots | `escapes:value` | `escapes:call` | `escapes:field` | `duplicated` |
| --- | --- | --- | --- | --- | --- |
| `none` (the rule before section 7.3) | 694 | 460 | 160 | 231 | 439 |
| `consume` | 711 | 460 | 129 | 245 | 440 |
| `store` | 694 | 460 | 160 | 231 | 439 |
| `all` | 711 | 460 | 129 | 245 | 440 |

**The owned-parameter widening paid; the store widening did not.**

- A frame block passed to a callee's consuming owned parameter recovered 31
  of the 160 `escapes:call` refusals and 17 slots (694 to 711). The 31 are
  calls whose callee was not unfolded and whose parameter the callee drops
  itself: the compiler's own units pass a freshly built block to a helper
  that pattern matches it once.
- The store widening moved no slot on this workload, and the refusal counts
  are identical to `none` in every column: here a block that would hold
  another block is itself returned or stored, so the holder is never
  frame-bound (the 460 `escapes:value` are mostly returned blocks, which
  item 3's worker/wrapper rule is the route to). It does fire on the shapes
  it is for: `stored.cinch` (a `Box` stored in a `Holder` that is matched in
  the same unit) goes from 0 slots to 2, and a leak-checked run of it is
  `live=0 immortal=0 leaked=0` under both `CINCH_LEAK_CHECK=1` and
  `CINCH_RC_DEBUG=1`.
- `escapes:field` *rose* by 14 under `consume` (231 to 245), which is the
  fixpoint working: a block whose callee now consumes it becomes a frame
  site, so stores into that block are re-decided and the ones whose holder
  is not frame-bound are refused for the true reason.
- The counts are lower than section 7.2's because dead-unit elimination now
  runs first: 1,101 units and 122,878 Core nodes the inliner or the
  specializer left with no caller are gone before the frame pass walks the
  program, so it considers fewer candidates (1,994 here).

The IR the sweep removes, on the same workload, is the direct measure of
item 2: the emitted module is 5,067,823 lines with `CINCH_NO_PRUNE=1` and
4,455,074 without, 12.1% of the artifact, and the manifest records
`simplifier.pruned` (1,101) and `simplifier.pruned_nodes` (122,878) so the
removal is visible in the artifact rather than inferred from IR size.

What is not measured here: the compiler's own speed (the toolkit check and a
self-compile) with interleaved medians. The first measurement was built to
answer "does the release remove volume", and the frame payload's release
costs nothing at run time on the paths the pass already lowered (the static
release is a fixed list of drops plus one header store per block, where it
was a fixed list of drops). The runtime walk only runs for a block whose
origin the unit cannot see, which is the population `consume` opened (31
refusals).

### 7.6 A tail call's frame replaces the caller's (item 5, closed)

The consume transfer was unsound in tail position, and it broke the
bootstrap. `CINCH_STACK_WIDEN=consume` alone reproduces it; `store` alone and
`none` both produce byte-identical IR for stage2 and converge, which
isolated the transfer without a bisect over the pass.

The trigger is the compiler's own `Bootstrap.Type.type-text`, compiled by the
seed with both widenings on. It builds a two-word `Cons` cell in a frame slot
and passes it to `Data.String.string-concat-all`, a call in tail position
with a matching signature, so code generation emits `musttail`:

```
  %t1521 = alloca [4 x i64], align 16
  ...
  store i64 %t1514, ptr %t1524          ; the cell's payload
  store i64 %t1520, ptr %t1525
  %t1528 = load i64, ptr %t1527
  %t1529 = musttail call i64 @"Data.String.string-concat-all"(i64 %t1528)
```

`string-concat-all` consumes its parameter: it patterns it, releases the
cell and forwards the tail to `concat-from`. But `musttail` means the callee's
frame *replaces* the caller's, so the callee's own spills and slots land in
`type-text`'s slot memory, which is where the cell it was handed lives. It
reads a clobbered header and payload, and the release walk reaches a word
that is not a block:

```
cinchruntime: compiler bug: reference count operation on a freed or corrupt block
  cinchruntime::rc::release
  cinch_drop_slow
  Data.String.string-concat-all
  Bootstrap.Type.mismatch-message
  Bootstrap.Type.unify-pi#tail
  Bootstrap.Dictionary.check-instance-pairs
```

A self tail call is the same hazard by a different route: code generation
turns it into a jump to the unit's loop header, which re-executes the unit's
own slot writes, so an argument handed across the back edge is overwritten by
the next iteration's allocation.

The fix is in the scan, not the lowering: `Scan` now threads tail position
(`Let`, `Dup`, `Drop`, `Release`, an `If` branch, a `case` alternative body
and a `Join`'s two bodies preserve it, exactly the forms
`CoreEmitter::emit_to` settles into one destination) and classifies every
argument of a call in that position as `Role::TailCall`, an escape. That
covers the lent case too: a borrowed parameter is a read, and a read of a
slot the callee has already spilled over is no better than a transfer.
Neither the ownership lattice nor `Facts::consumed` can see the frame
substitution, because both are properties of the callee's body, not of where
the call sits.

Threading the flag through every tail-preserving form is the part that has to
be exhaustive, and the first version of the fix was not: `Dup` and `Drop` had
an arm that recorded the count and then fell through to the generic child
walk, which resets the position to an ordinary value, and `If` had no arm at
all. Rebuilding the chain from that version moved every call site in
`type-text` out of the frame except one, which then crashed the same way at
`Data.String.concat-from`: the slot was a cell allocated inside a count
operation's body, and the argument read as a consumption because the scan
never saw the tail. The flag is therefore threaded from the unit body through
each form's body explicitly, and the regression test covers the plain body,
a `Dup` body and an `If` branch, so a form that stops preserving the flag
fails the test rather than the bootstrap.

The regression test is `a_frame_block_passed_to_a_tail_call_stays_on_the_heap`
in `compiler/tests/stack_allocation.rs`: the same Core program the widening's
test uses, with the call as the unit's result rather than a `let` value, under
each of those forms, for a consumed and for a lent parameter. It takes a slot
before this change and is refused as `escapes:tail-call` after it.

### 7.7 Measured: what the pass buys after the tail-call fix

Both binaries below are Stage1, the one compiler in the chain the pass lowers
(the seed builds it; Stage2 and later are built by the Cinch compiler, which
does not implement the pass yet). `--no-stack-alloc` in `CINCH_SEED_FLAGS`
builds the same source without frame slots, so the comparison is the
compiler's own code:

| Workload (Stage1) | Pass | Blocks | Bytes | CPU (user+sys) |
| --- | --- | --- | --- | --- |
| toolkit check, 3,966 lines | on | 336,133,037 | 7.38 GB | 8.04 s, 7.55 s |
| toolkit check | off | 350,516,475 | 7.73 GB | 7.73 s, 7.26 s |
| self-compile, 40,266 lines | on | 3,089,819,009 | 71.16 GB | 106.9 s |
| self-compile | off | 3,133,383,640 | 72.24 GB | 110.1 s |

The pass removes 4.1% of the blocks and 4.6% of the bytes on the toolkit
check, and 1.4% and 1.5% on the self-compile, all with `live=0` and
`immortal=0` in the allocation summary, so the volume leaves the heap rather
than becoming a leak. The CPU it buys is not established: the toolkit check
came out 4% slower and 4% faster in its two interleaved pairs (the on binary
is 26.8 MB against 22.8 MB, so it pays for its fewer allocations in instruction
footprint), and the self-compile's single pair is 3% faster on. Volume moved;
speed did not, at this reach. Raising the reach (the returned-block
population, section 7.2's 42% of refusals) is what would make the volume
matter, and that is the worker/wrapper queue item, not this pass.

### 7.8 One apply stub per function (2026-11-11)

The fixed-frame stub that retired the dynamic `alloca` (section 1, and
`compilation-speed.md` section 8) kept the `caml_applyN` shape: a
dispatcher that switched on `%argc`, one `app.<N>` entry per supplied count
from zero to the arity, and an `app.many` default. On the compiler's own
module that is 196 dispatchers, 973 per-supplied entries and 196 `many`
bodies, 1,365 functions and 2.90 MB of `__TEXT`.

An A/B bisect of the chain measured what it cost. Every arm is a full chain
build at its revision with PGO on, shard count 3, and one byte-identical
3,965-line toolkit file (SHA-256 `e29fb240...`), round-robin interleaved, 15
rounds each:

| comparison | CPU delta | wall |
| --- | --- | --- |
| `12c31ad9` to `9f4e4030` (the apply block) | **+0.470 s, +7.45%** | +7.37% |
| `9f4e4030` to `6e9f9122` (reuse and the later pins) | -0.15%, inside spread | 0% |
| `6e9f9122` to `c31cb754` (the dense body module) | -0.91% | +0.06% |
| two-line control, same apply block | **+7.14%** | +6.98% |

The cause is footprint, not instructions. The workload's own emitted IR is
unchanged (193 to 195 lines). The PGO profile the stage link already trains
records 54 of those 1,169 apply entries executing at all, 1.16M of 672M
calls, 0.17%. What the compiler pays for is the IR it writes and retains:
3.4 to 5.2 s of its own CPU and 44 to 45 MB of peak RSS (90.0 s / 428 MB at
`12c31ad9` against 93.4 s / 472 MB at `9f4e4030`).

So the cross product goes, and both properties it was introduced for stay:
no dynamic `alloca`, and a direct saturated call. `@"U.app"` is one entry
per function, whatever count it is handed:

- read `nf` (the closure's filled count), compute `total = nf + %argc`,
  branch once on `total >= arity`;
- **saturated** (the common path, and the only path a fully applied closure
  takes): merge the `nf` saved words and the `%argc` incoming ones into a
  static `[arity x i64]` frame, one `select` per slot, drop or free the
  closure, call the function directly, and push any excess at the result
  through `__apply` exactly as before;
- **grow** (`total < arity`): allocate a closure whose payload is the saved
  words followed by the incoming ones and whose code word is this same stub,
  and copy both directly with `memcpy`.

Two things fall out of that arrangement and are deliberate. The merge is
inside the saturated block, because only there does every frame slot have a
word behind it: the earlier entries merged all `arity` slots before
branching, and a grow-path slot above `nf + supplied` computed a pointer
past the end of `argv` (never loaded, but the bounds argument is better made
than relied on). And the grow path copies the two sources rather than a
frame, so the frame's only role is the saturated call's argument list. A
nullary unit needs no frame and no branch at all, so its stub is the
saturated path alone.

Cost: one `add` and one `icmp` per application, in place of the
dispatcher's `switch`, and no constant-supplied specialisation. The
specialisation was the thing that did not pay: 54 of 1,169 entries ever
executed.

Measured, and the result contradicts the attribution above. The bisect
compared whole chain builds at two revisions, and `12c31ad9..9f4e4030`
contains eleven commits, of which the stub is one (`5e8115a7`). The other
ten are the interface optimisation artifact (`compiler/src/ifaceopt.rs`,
957 new lines), the simplifier, specialisation, the driver and the seed
arena. To separate them, two arms were built from the identical base tree
(`b4854641`, which is `main`) differing only in the two emitter files, so
the only variable is this collapse. Round-robin, 5 pairs each, one
byte-identical toolkit file:

| arm | toolkit check CPU (median) | spread | delta from `12c31ad9` |
| --- | --- | --- | --- |
| `12c31ad9` (before the fan-out) | 6.05 s | 0.07 | |
| `ctl` = `b4854641`, the fan-out intact | 6.39 s | 0.16 | +5.62% |
| `col` = `b4854641` + this collapse | 6.46 s | 0.28 | +5.68% |

`ctl` to `col` is **+0.16%, inside the spread**. The collapse is real and
exactly as large as intended in the emitted module, and it buys nothing:
the stage-3 module goes from 7,990 `define`s and 47.66 MB to 6,814 and
42.44 MB, 1,176 fewer functions and 5.22 MB less text, with stage2 and
stage3 `compiler.ll` byte equal on both arms. The 5.6% regression that the
bisect charged to "the apply block" is therefore in the other ten commits,
not in the stub fan-out: the stub is 1,169 functions of code that the
profile says executes 54 times out of 672M calls, and removing it moves
neither the toolkit check nor the compiler's own compile.

That is worth keeping on its own terms. The fan-out was introduced to
retire the last dynamic `alloca` (section 1), and one entry per function
retires it too, with 1,176 fewer functions and 5.2 MB less `__TEXT`, no
per-count specialisation and one `add` plus one `icmp` per application. It
is also the smaller IR to compile, which is the direction section 7.7 says
the pass pays in. What it is not is the fix for the 5.6%, and the
attribution to check next is the interface optimisation artifact and the
simplifier changes in the same range.

Both emitters carry the change, `compiler/src/codegen.rs` (`stub_def`,
`stub_grow`) and `bootstrap/Bootstrap.RuntimeIr.cinch` (`app-stub`,
`saturated-stub`, `app-partial`), and `design/reclamation.md` section 4
states the ownership rule for the single entry.

Gates, all on the lane chain built into `/tmp/ab/collapse`: the fixed point
holds (stage2 and stage3 `compiler.ll` byte equal, `cinch` byte equal, every
`.cinchif` byte equal), `smoke --stage 3` passes all three cases, and the
toolkit check reports `cinch-leak: live=0 immortal=0 leaked=0`. Of the five
required test targets, `bootstrap_runtime_emit`,
`bootstrap_runtime_library_abi`, `bootstrap_runtime_ir` and
`stack_allocation` pass; `core_output` loses two golden-file tests on a
`@ticks` difference that the unmodified base tree reproduces, and so do the
four suite targets that lose a test (`bootstrap_manifest`,
`bootstrap_parse`, `bootstrap_pipeline`, `syntax_index_evidence`), all
measured on the `ctl` arm and identical there. The full target suite is 140
of 233 passing against the recorded 145 of 231, and the five that differ are
those same pre-existing failures.

### 7.2 Measured reach

`core emit` with the rules on and with `--no-inline`, refusals by reason
(`CINCH_STACK_TRACE`, the first escape's role after the reason):

| Source | Inline | Sites / nodes | Slots | Bytes | escapes:value | escapes:call | escapes:field | escapes:capture | escapes:head | duplicated |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 3,964-line toolkit | on | 133 / 497 | 5 | 160 | 66 | 15 | 18 | 23 | 0 | 1 |
| 3,964-line toolkit | off | 0 / 0 | 5 | 160 | 66 | 15 | 18 | 23 | 0 | 1 |
| 40,052-line compiler source | on | 7,253 / 17,326 | 1,014 | 33,072 | 3,220 | 257 | 300 | 25 | 5 | 491 |
| 40,052-line compiler source | off | 0 / 0 | 1,024 | 33,280 | 791 | 254 | 299 | 25 | 5 | 490 |

The mechanism is unblocked and the reach it was expected to move has not
moved: 133 sites on the toolkit and 7,253 on the compiler source, and the
frame pass's slots are unchanged on the toolkit and ten lower on the
compiler source. The reason is in the reason column. A copied callee body
that returns a block returns it from every unit it was copied into, so
inlining multiplies `escapes:value` (791 to 3,220) instead of consuming it.
The population it was supposed to reach is blocks that *leave* a callee
(791 of the 1,864 refusals measured here, 42%), and copying the callee into
its callers does not make those blocks local, it only makes more of them:
each copy is a new unit that returns a block. Item 4 of the queue is the
route to that population, not the copy bug. The per-unit picture is mixed in
both directions, which is what a rule that moves code between units does:
six units gain slots because a copied callee brought its allocation with it
(`Bootstrap.RuntimeIr.scheme-classes` 1 to 3, `Bootstrap.ExportType.quantifiers`
0 to 2), and `Bootstrap.RuntimeIr.scheme-classes.shared0` loses its only slot
because the inlined body removed the subterm that had it.

What inlining buys is volume, in two ways that a block count separates
(`CINCH_ALLOC_STATS=1`, `CINCH_LEAK_CHECK=1`, `live=0` in every row):

| Program (200,000-iteration loop unless noted) | Inliner | Frame pass | Blocks | Bytes |
| --- | --- | --- | --- | --- |
| selector with a primitive call in its body (`pick p = case p of MkPair a b -> a + b`) | on | on | 100,006 | 2,400,213 |
| the same, `--no-inline` | off | on | 100,006 | 2,400,213 |
| selector without the call (`pick p = case p of MkPair a b -> a`) | on | on | 6 | 213 |
| the same, `--no-inline` | off | on | 100,006 | 2,400,213 |
| helper returns a block, caller matches it at once (`case tick i of Step a b -> ...`, tag not statically known) | on | on | 6 | 213 |
| the same, `--no-stack-alloc` | on | off | 200,006 | 4,800,213 |
| the same, `--no-inline` | off | on | 100,005 | 2,400,189 |
| 1,000 iterations, `open (Cell i)` | on | on | 6 | 213 |
| the same, `--no-inline` | off | on | 506 | 12,144 |

The first row is the landed rule's boundary: `a + b` is an application, so the
multi-site rule refuses the body as `costly` and nothing moves. The second
block is `known-constructor` folding the case once the call that hid it is
gone (`case (pick (MkPair i 2)) of` becomes `case (MkPair i 2) of`), which
deletes the block rather than framing it. The third is the case the whole
note is about: the block is *returned* by `tick`, so without the inliner the
frame pass can only refuse it (`escapes:value`, and the manifest shows it);
with the inliner the block is born and matched in `drive`'s activation and
takes one of the unit's two 32-byte slots, 200,006 blocks down to 6. So the
reach numbers in the table above understate what the rule is for: the
workloads that race it are the ones where the callee's block is returned,
and the compiler source's own loop bodies are `let`-heavy enough that
`pattern_binders_only` refuses them (the single-site rule admits a `let`
body only when it binds nothing but `case` pattern binders).

The `escapes:call` and `escapes:field` populations move by 3 and 1, because
the inlined selectors are cheap: they read a field, they do not build blocks.

The duplicated rule (item 3) moved three refusals on the compiler source and
no slots. Against the same measurement with the widened rule disabled
(1,024 slots, `escapes:value` 788, `duplicated` 493), `duplicated` fell to 490
and `escapes:value` rose to 791: those three blocks are duplicated *and*
returned, so the escape check the widened rule now reaches refuses them for
the true reason. The widening costs nothing and makes the report honest, but
it is not a reach win on this workload, and a source program rarely produces
the shape at all: borrow inference lends a parameter that is only read, and
drop specialization turns a second scrutinee read into a `Drop`, so the
`Dup` has to come from two *owned* uses. The rule is therefore pinned by a
Core-level test that states the reference shape exactly.

The queue, in expected-value order, with the numbers that justify each:

1. **Let the runtime release a frame payload.** `cinch_drop_slow` on
   `RC_FRAME` scans the block's real kind and descriptor and drops each
   counted child without freeing the block. Unlocks `escapes:call` (257) and
   `escapes:field` (300) and removes the static release plan from codegen.
   It costs the zero-cost frame drop, and it needs a rule for which drop
   releases the payload when no count makes one of them last. This is now
   the first item: item 1 of the previous queue (the inliner copy bug) is
   closed, and its measurement says the returned-value population is not
   reachable by copying callees.
2. **Dead-unit elimination**, so inlining shrinks IR instead of spending a
   node budget: 7,253 sites and 17,326 added nodes on the compiler source,
   against a budget of two percent of the program. Roots: `program.main`,
   `foreign export` targets, instance methods, constructor units named by a
   case tag or the layout table, and every unit an interface exports.
3. **Worker/wrapper for a returned single-field block**, which is the shapes
   the 3,220 `escapes:value` refusals are in: `case p of Path s -> s` is a
   selector that hands back its field, and a caller that reads it twice pays
   for the block once. `known-constructor` already deletes the ones whose
   scrutinee is a literal constructor; the rest need the block to not be
   built at all.
4. **Measure volume on the ladder, not reach**: build Stage2 with the rules
   on and off and compare `cinch-alloc: blocks=` from `CINCH_ALLOC_STATS=1`
   on the toolkit check. The two microbenchmarks above are not the workload.
5. **Dead-source lookahead for the duplicated rule**: a duplicated block
   whose every counted field is dead on every path needs no release either,
   which is a larger population than "no counted field at all" (three
   candidates measured, so measure the refined rule before building it).
6. Small items: `case_merge` fires zero times on both workloads (delete or
   fix); extend `known-constructor` to non-value arguments by binding them,
   which deletes the allocation instead of framing it; pin the Tarjan SCC
   analysis with a test.

## 8. The escape census (2026-11-13)

Section 7 measured the pass's reach as slots and refusals by reason. That is
not enough to decide what to do next: "escapes" covers six different
positions, and the rule's own checks (`duplicated`, `layout`, `size`) are
refused before any reference is looked at, so a candidate that is both
duplicated and returned appears in one column and hides the other. Section 8
classifies every allocation the pass considers, reports the classes with
their block counts and payload bytes, and asks of each class what relaxing
it would buy.

### 8.1 The instrument

`--escape-report` (`CINCH_ESCAPE_REPORT=1` does the same for a caller that
cannot pass flags) prints the census after frame allocation, and the
artifact manifest's `stack` row carries it:

```
$ cinch emit-ir compiler.cinch -o /tmp/compiler.ll --escape-report
cinch escape census: compiler.cinch (EmitIr): 2389 let-bound allocation(s), ...
```

`stack.classes` is the binding reason per class (the first check that
failed, in the rule's order), so its counts sum to `stack.candidates`;
`stack.positions` counts escape positions independently, so a candidate that
escapes in two positions appears under both. `stack.payload` is the payload
bytes the frame slots hold, `stack.largest` and `stack.largest_class` are the
largest candidate payload and the class that refused it, and
`stack.partial` is the partial-escape population of section 8.5.

The classes are `Refusal::ALL` in `compiler/src/stack.rs`:

| Class | What it is |
| --- | --- |
| `frame` | lowered into the unit's frame |
| `layout` | a boundary layout (`repr(C)`, `packed`, `transparent`) |
| `form` | a site the emitter will not lower as one block (an unknown constructor unit, a closure over a unit that does not exist) |
| `uncounted` | the binder's class is `Raw`, so no `Drop` hangs on the payload's release |
| `duplicated` | `Dup`ped with a counted payload word |
| `size` | above the block limit and otherwise eligible |
| `frame-limit` | the unit's frame would pass its limit and otherwise eligible |
| `return` | the unit's own result |
| `value` | a `let` value, an `If` condition, another non-result value position |
| `field` | a store into a field of a block that is not frame-bound |
| `call` | an argument of a call the pass cannot see into |
| `tail-call` | an argument of a call in tail position |
| `spawn` | an argument of `spawn` |
| `jump` | an argument of a join's `Jump` |
| `capture` | a closure's saved word |
| `write` | a `RefSet`/`BoxInit`/`LazyInit` store, or the cell written through |
| `head` | the callee of an application |

Reproducing it: `python3 tooling/bootstrap.py build --stage 3` builds the
seed, then `compiler/target/release/cinch emit-ir <bundle> --escape-report`.
The two bundles this section reports are `tooling/pgo.py`'s training bundle
(the five toolkit modules, 3,966 lines, `main = 0`) and
`tooling/bootstrap.py`'s `collect_compiler_sources` bundle (51 files,
40,266 lines) for the compiler's own build. `CINCH_NO_PRUNE=1` is the
measurement the toolkit bundle needs: its `main = 0` reaches nothing, so the
dead-unit sweep removes the toolkit before the pass sees it.

### 8.2 The table

`emit-ir`, the shipped rules (`CINCH_STACK_WIDEN=all`), pruning as the build
does it, `--escape-report`:

| Workload | Candidates | Frame | `duplicated` | `return` | `value` | `field` | `call` | `tail-call` | `capture` | `head` | Slots | Payload |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| toolkit bundle, `CINCH_NO_PRUNE=1` | 61 | 1 (1.6%) | 1 (1.6%) | 6 (9.8%) | 8 (13.1%) | 10 (16.4%) | 2 (3.3%) | 10 (16.4%) | 23 (37.7%) | 0 | 1 | 16 B |
| toolkit driver (the lexer over a source string) | 9 | 0 (0.0%) | 1 (11.1%) | 1 (11.1%) | 0 | 6 (66.7%) | 0 | 1 (11.1%) | 0 | 0 | 0 | 0 B |
| compiler bundle, 40,266 lines | 2,389 | 1,165 (48.8%) | 494 (20.7%) | 55 (2.3%) | 184 (7.7%) | 326 (13.6%) | 89 (3.7%) | 60 (2.5%) | 4 (0.2%) | 12 (0.5%) | 1,165 | 21,040 B |

Payload bytes per class, same runs:

| Workload | `frame` | `duplicated` | `return` | `value` | `field` | `call` | `tail-call` | `capture` | `head` | Largest |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| toolkit bundle | 16 | 32 | 88 | 144 | 184 | 32 | 240 | 640 | 0 | 56 B |
| toolkit driver | 0 | 32 | 24 | 0 | 120 | 0 | 56 | 0 | 0 | 56 B |
| compiler bundle | 21,040 | 8,096 | 864 | 3,056 | 7,464 | 2,472 | 1,176 | 112 | 528 | 88 B |

Escape positions, counted independently (a candidate may hold several), on
both workloads:

| Position | Toolkit bundle | % | Bytes | Compiler bundle | % | Bytes |
| --- | --- | --- | --- | --- | --- | --- |
| `field` | 23 | 37.7% | 392 | 847 | 35.5% | 15,808 |
| `tail-call` | 17 | 27.9% | 344 | 102 | 4.3% | 1,904 |
| `value` | 8 | 13.1% | 144 | 188 | 7.9% | 3,216 |
| `return` | 6 | 9.8% | 88 | 67 | 2.8% | 1,056 |
| `call` | 4 | 6.6% | 64 | 260 | 10.9% | 5,328 |
| `capture` | 23 | 37.7% | 640 | 11 | 0.5% | 488 |
| `head` | 1 | 1.6% | 32 | 15 | 0.6% | 616 |

`spawn`, `jump` and `write` are zero on both workloads: the compiler builds
no task closures, its joins are all lowered to branches the scan already
walks, and no `let`-bound block is stored through a `RefSet`, `BoxInit` or
`LazyInit`.

The independent column is why the class column cannot answer the queue's
questions alone. `field` is 326 candidates by binding reason and 847
positions: most field stores sit in candidates that some other check
refused first (a duplicated payload, a returned holder), so a change to the
store rule would reach 847 stores but only the 326 candidates whose binding
reason it is. Both numbers are real, and they answer different questions:
"how many allocations would this rule move" (binding) and "how many
references would it admit" (position).

### 8.3 What the classes say

**The size class is empty because nothing is near the limit, not because the
check is unreachable.** The largest candidate payload on either workload is
88 bytes (11 words) on the compiler bundle and 56 bytes (7 words) on the
toolkit, against a 4 KiB block limit and a 16 KiB frame limit.
Raising `DEFAULT_BLOCK_LIMIT` would buy nothing on either: there is no
candidate between 88 bytes and 4 KiB, and the `size` class is empty because
of that, not because the check is unreachable (the `largest` column is what
says so, which is why the manifest carries it).

The frame limit is slack too, and measured both ways. Raising it to 1 MiB
leaves the compiler bundle's census identical, class for class, to the
16 KiB run (1,165 frame, 494 duplicated, and every other row unchanged), so
no unit's frame is near the limit. Lowering it to 0 moves the compiler
bundle's 1,165 slots into `frame-limit` (48.8%, 21,040 bytes) and the
toolkit bundle's single slot likewise, which is the check working and the
column being the population a limit would buy. `frame-limit` is zero at the
default on both workloads. Neither limit is a lever on these two programs;
both are worth keeping as bounds (they are what keeps a pathological unit
off the stack) but there is nothing to raise, and a per-unit block limit
would have no population to apply to either.

**The layout class is empty.** No candidate on either workload is a
`repr(C)`, `packed` or `transparent` constructor. Reproducing recorded
offsets would be worth zero bytes here, which is why this lane did not
attempt it: it is a change with a real cost (the code generator's field
access would need the layout plan at the allocation site, and the frame
release would need the recorded offsets rather than payload order) against a
population of zero.

**`duplicated` is the largest single rule-level class on the compiler
bundle: 494 candidates, 20.7%, 8,096 bytes.** That is the population section
7.2's queue item 5 named, and it is bigger than the three it guessed at. The
rule refuses a `Dup`ped binder whose payload holds a counted word, because
two references need two releases and a frame block has no count to make one
of them a no-op. The reachable refinement is not "a larger limit" but
dead-field lookahead: a duplicated block whose counted fields are *dead* on
every path needs no release at all, so both references can drop it as a
no-op. This census does not measure how many of the 494 have that shape (it
would need a liveness pass over the payload), so the next step is to measure
it before building it, exactly as item 5 said, but now with the size known:
at most 494 candidates and 8,096 bytes.

**`capture` dominates the toolkit bundle (23 of 61 candidates, 640 bytes)
and is nearly absent from the compiler bundle (4 binding, 11 positions).**
A captured word is a closure environment slot, and a frame block's address
cannot go there: the closure outlives the activation as soon as anything
stores it. The toolkit's
combinators are closures over freshly built blocks, which is why the class is
large there. No widening reaches it: a closure that captures a block is the
definition of the block escaping.

**`field` is the largest escape position on both workloads** (847
positions on the compiler bundle, 23 on the toolkit bundle, 6 of 9 on the
driver). A store into a block that is not itself frame-bound keeps the child
on the heap. The `store` widening already admits the case where the holder
is frame-bound, and section 7.5 measured it firing on the shapes it is for
but not on the compiler workload, because there the holder is returned or
stored in turn. The population is therefore
the same one `return` names, seen from the other side: the holder has to
stop escaping before its fields can be framed.

**`return` is small on the compiler bundle (55 binding, 67 independent) and
is the class section 7.2 spent its effort on.** The difference between the
two columns is the point of counting positions independently: 12 of the
`return` positions are in candidates whose *binding* reason is `duplicated`
or a store, so a change to the return rule alone would not reach them. The
worker/wrapper queue item is worth at most 67 candidates and 1,056 bytes on
this workload, not the 3,220 that section 7.2's first-escape column reported
(that column counted a candidate once, under whichever escape came first in
the walk, so it could not separate a returned block from a block that is
duplicated *and* returned).

### 8.4 What this lane changed

Nothing in the rule. The census is the deliverable, and every class that
could have justified a change came out either empty (`layout`, `size`,
`frame-limit`, `spawn`, `jump`, `write`, `uncounted`) or too small to be
worth a rule (`return` at 2.3% binding, `call` at 3.7%, `head` at 0.5%,
`capture` at 0.2%). The two classes worth acting on are `duplicated`
(section 7.2's item 5, now sized at 494 candidates and 8,096 bytes) and the
partial-escape population below, which is the next lane by the task's own
ordering.

The size limits are worth keeping and not worth raising. The largest
candidate is 88 bytes on the compiler bundle and 56 on the toolkit bundle,
against a 4 KiB block limit and a 16 KiB per-unit frame limit. Measured both
ways on the compiler bundle: at `--stack-frame-limit 1048576` the census and
the emitted IR are identical to the 16 KiB run, so no unit is near the
limit; at `--stack-frame-limit 0` all 1,165 slots move into `frame-limit`
(48.8%, 21,040 bytes), which is the check working and the column being the
population a limit would buy. A per-unit block limit would be a rule with no
population to apply to.

The emitted IR is byte-identical before and after the instrumentation on
both workloads (`cmp` on the two `emit-ir` outputs), the manifest's `stack`
row gained `payload`, `classes`, `positions`, `partial`, `largest` and
`largest_class`, and `refused` keeps its old shape so every existing reader
(the `stack_allocation` tests included) sees what it saw.

Two caveats for whoever runs the target suite against
`bootstrap/stage3-suite-baseline.json`. That baseline is dated from commit
`5fc2d4ee`, which predates `bootstrap/Bootstrap.Expand.cinch`; two targets
therefore fail for a reason this pass cannot reach, and neither is a
regression of the census. `bootstrap_parse` reports
`Bootstrap.Expand` in its measured unmodeled table without the constant
listing it (the test enumerates modules by hand), and `bootstrap_pipeline`
fails to build its fixture because the fixture ships no
`Bootstrap.Expand.cinch`. Re-deriving the baseline against a current
commit is the fix; it is not this lane's change to make.

### 8.5 The partial-escape population, sized for the next lane

A value whose references are consumed on one branch and escape on another is
the partial-escape case: the value could stay virtual (a frame slot) on the
consuming path and be materialised as a heap block only on the escaping
path. The census counts a candidate as partial when its first escape sits at
branch path `E` and some path `C` that ends the value does not start with
`E`, which is exactly "the escape and the consumption are on different
sides of a decision point".

| Workload | Partial | % of candidates | Bytes |
| --- | --- | --- | --- |
| toolkit bundle | 14 | 23.0% | 312 |
| toolkit driver | 2 | 22.2% | 80 |
| compiler bundle | 161 | 6.7% | 3,600 |

161 candidates and 3,600 bytes on the compiler bundle, 14 and 312 bytes on
the toolkit bundle. That is the size of the next lane: a split that keeps
the value in the frame on the consuming path would move at most those 161
allocations off the heap, and it costs a copy on every escaping path plus a
rule for which path owns the release. It is smaller than the `duplicated` population and it
is a much larger change (the value becomes two values with different
lifetimes), which is why it is the next lane and not this one.

**Dynamic promotion is not needed and the census says so.** Allocate on the
stack and copy to the heap if the value escapes at run time needs a barrier
on every store of a frame pointer into a heap object, and the population it
would reach is the 161 partial candidates above. The rule-level classes
(`duplicated` 494, `layout` 0, `size` 0, `uncounted` 0) are not reachable by
promotion at all: a duplicated payload's problem is a second *release*, not a
late escape. 6.7% of the candidates, at the price of a write barrier on every
heap store in the program, is not a trade this census recommends.

What a promotion would have to be, to be complete: the barrier has to catch
a frame pointer stored into any heap object, which is every `field` store,
every `capture` and every `write` in the census's position column (847, 11
and 0 on the compiler bundle). The three positions are already refused by
the rule, so the promotion would have to add a check to the store path for
a population the static rule already refuses: it converts a compile-time
refusal into a run-time copy, and it cannot help the candidates whose
binding reason is `duplicated` (the majority) at all.

Then the Cinch-side implementation, gated on matching the seed's emitted IR
for the toolkit bundle.

Related designs: `ownership-and-usage.md`, `reclamation.md`,
`compilation-speed.md`, `layouts-and-specialization.md`,
`core-output.md`, `parallel-evaluation.md`.
