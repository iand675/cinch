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

Then the Cinch-side implementation, gated on matching the seed's emitted IR
for the toolkit bundle.

### 7.9 Continuations: application is a consumption (2026-11-14)

A continuation handed to a callee that calls it once and returns was
refused three ways. The changes are part of design/cps-optimisation.md,
rule 5:

- **Naming.** A closure that is a direct argument of a non-tail call to a
  consuming parameter, with captures that are locals, literals or dups of
  a local, is `let`-bound before the frame rule runs
  (`stack::name_closure_args`). Building it performs only count
  increments, so moving it ahead of its siblings changes no order.
- **`Role::Head`.** A non-tail application of a frame binder is a
  consumption (`Uses::applied`). In a parameter summary, a tail
  application (`Role::TailHead`) and a self-call that forwards the
  parameter to its own position (`Role::SelfForward`) are consumptions
  too: the block sits in a caller's frame, which outlives the summarised
  unit's tail calls. In the allocating unit, a tail application of its own
  frame closure stays an escape (section 7.6).
- **The apply stub's frame arm.** The stub's shared path read `RC_FRAME`
  as a positive count and decremented it in place. That would have left
  the payload unreleased and the marker corrupt, had a frame closure ever
  been applied; the old rule refused every application, so none was. The
  shared path now duplicates the saved words for the call and then
  releases a frame closure's payload once (`cinch_frame_release`). The tail
  twin shares the code. The closure's own unit joins
  `Report::receives_frame`.

Applying a closure is never a borrowed position (`rc.rs` borrow
inference), so a frame closure is never lent to a callee that applies it,
and the stub's release is the only one on its path. A `Dup` of a frame
closure with a counted capture is still refused.

Measured on a ten-million-iteration loop whose continuation captures a
counted word, with specialisation and inlining off so the closure
survives: 1,155 ms on the heap and 922 ms in the frame.
`tests/cps.rs::a_consumed_continuation_takes_a_frame_slot` covers a
non-tail application and a tail application in the callee, at `-O0` and
`-O2`, under the leak check.

Related designs: `ownership-and-usage.md`, `reclamation.md`,
`compilation-speed.md`, `layouts-and-specialization.md`,
`core-output.md`, `parallel-evaluation.md`.
