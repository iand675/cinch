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

Related designs: `ownership-and-usage.md`, `reclamation.md`,
`compilation-speed.md`, `layouts-and-specialization.md`,
`core-output.md`, `parallel-evaluation.md`.
