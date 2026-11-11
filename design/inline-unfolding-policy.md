# The unfolding inliner

Policy for `compiler/src/simplify.rs` (the seed simplifier) and
`bootstrap/Bootstrap.RuntimeIr.cinch` (the self-hosted backend's inliner).
The goal is the one a hand-written compiler meets without trying: a
dictionary method whose body is a primop, a field projection or a shallow
decision must reach the instruction it names at the site that calls it, so
no call to `Eq_I64.==`, `Bool.not` or `Nat.Add_Nat.+` survives.

No em dashes.

## 1 What the census says

The compiler's own retained-Core IR (`build/bootstrap/stage2/compiler.ll`)
called these units at the counts below before this policy. Every one of
them is a body a hand-written compiler would have emitted inline.

| residual callee | calls |
|---|---|
| `Data.List.FromList_List_List@O.mk` | 1208 |
| `Data.List.Semigroup_List@O.mk` | 856 |
| `Data.Machine.Eq_I64.==` | 726 |
| `Data.String.string-compare` | 651 |
| `Data.Nat.Eq_Nat.==` | 587 |
| `Data.Bool.not` | 412 |
| `Data.Nat.Add_Nat.+` | 327 |

The seed's own IR, which compiles the prelude into the same program, called
`Data.Machine.Eq_I64.==` 511 times, `Data.Nat.Add_Nat.+` 1077 times,
`Data.String.string-compare` 904 times and `Data.Bool.not` 44 times, with
every one of those unit bodies present in the program. So the bodies were
available and the policy refused them: every one of them was refused with
`costly`, the multi-site rule's body test, because a primop application is a
`Core::App` and the old rule read any `App` as a call.

That is the whole gap in the seed: admission, not discovery. The
self-hosted compiler has a second, structural gap, section 6.

## 2 The model: a declaration unfolds when its body is cheap by construction

The inliner answers one question per call site: may this call be replaced by
the callee's body? The existing answer is a size rule (`40` nodes at one
site, `12` at many) plus a cost test for the many-site case. The size rule
is a proxy for "the copy will not cost more than the call", and the proxy
fails exactly where the goal is: a four-node primop wrapper is refused
because of the *shape* of its body, not its size.

The new rule states the property directly.

A unit is **unfoldable** when every node of its body is one of the forms in
the table in section 3, when no node in it is a value that escapes (a
closure, a quotation, a lazy thunk), and when it is not a member of a
recursive group. Unfoldable is a unit property, computed once per program,
because the rules read only the body and the unit's own callees. It is
transitive: a body that applies another unfoldable unit is unfoldable, so a
dictionary method that delegates to a second method unfolds through the
chain, and the budget decreases per link (section 4).

Unfoldable replaces the cost test for the multi-site rule. The size caps
stay as a separate bound on duplicated code, because "cheap per copy" says
nothing about how many copies a program wants, and the rules keep their
precedence so a copy the old size rules admitted keeps its old shape:

| rule | body test | size cap | site test |
|---|---|---|---|
| single site | binds only `case` pattern binders | `inline_threshold` (40) | none |
| every site | unfoldable, no decision, pure | `inline_small` (12) | none |
| every site | unfoldable, no decision, calls a unit | `inline_unfold` (40) | none |
| every site | unfoldable, one decision | `inline_unfold_case` (24) | fold, when over `inline_unfold_small` (8) |

The `inline_unfold` cap exists because a body with a call in it is the shape
that grows: each copy carries a call that will itself be expanded, so one
copy of a 30-node chain can become 60 nodes after the chain closes. A body
of pure projections, literals and primops (the 3 to 8 node methods) is
bounded by `inline_small`, which is the same 12 nodes as before.

The two decision caps are the split between "a copy that cannot cost more
than the call" and "a copy that duplicates a branch". A decision-bearing
body of at most 8 nodes lowers to a compare and a branch over two trivial
alternatives, which is what the call's own dispatch cost; it is copied
everywhere. A larger one is copied only where the scrutinee argument is a
constructor application, which `known_constructor` folds in the same
iteration, so the copy ends up carrying one alternative. That is the
"shallow case on a known constructor" rule stated as a site test rather
than a body test, because the property is a property of the site.

The single-site rule keeps precedence because it admits a copy the class
rule would refuse at the site: `case p of Path s -> s` at one site with an
unknown `p` is a borrowed view of an argument, and it is exactly the shape
the frame pass's refusals are in.

## 3 The class: what "cheap by construction" admits

Class membership is a structural predicate over the Core body. It admits a
node when the code generator emits that node's work at the site, and admits
nothing whose copy creates a *new* scope, a *new* allocation site, or a
*site-dependent* effect.

| node | admitted | why |
|---|---|---|
| `Int`, `Bool`, `Unit`, `ConstBig`, `ConstBytes`, `ConstString`, `ConstSym`, `StrLit` | yes | a literal; the constant is emitted once per copy, which the linker merges |
| `Var` | yes | a read of a binder the copy renames |
| `Prim(op)` | yes | the intrinsic's instruction sequence, emitted at the site |
| `Foreign(sym)` | yes | a real call; the copy calls it once per execution, exactly as the callee did |
| `Global(g)`, `Ctor(g)` | yes, as an `App` head | a call to `g`; admitted when `g` is unfoldable, a constructor, or a `prim` unit |
| `App(head, args)` | yes | a call or a primop application; the head decides |
| `Proj`, `DictProj` | yes | a load from a block the caller already holds |
| `Let(x, class, v, body)` | yes | a value binder; the copy renames `x` and keeps the class |
| `Case` | yes, shallow | a tag comparison and a branch, which is what the call's own dispatch would have been; alternatives must be admitted and the case must not nest |
| `If` | yes | same as a shallow `Case` |
| `Dup`, `Drop` | yes | a count operation on a binder the copy renamed; the count pass inserted it for the callee's parameters and the copy keeps that balance |
| `InstDict` | no | a dictionary construction: one allocation per copy, and copies multiply sites |
| `Ctor` application | no, unless the unit is a constructor unit or the class rule's own budget admits it | every copy is a fresh block, so a copy at every site multiplies allocation sites |
| `Lam` | no | a closure: the copy would create one closure per site instead of one per definition |
| `Quote`, `DeclsQuote` | no | a retained body is a binding scope of its own |
| `Join`, `Jump` | no | a join's identity is per unit; a copy needs fresh join ids and a scope of its own |
| `Release`, `BoxInit`, `BoxClear`, `BoxGet`, `LazyInit`, `LazyAlloc`, `BoxAlloc`, `Demand`, `RefSet` | no | storage effects whose duplication changes when the effect happens |
| `Fault`, `SpliceVal` | no | a fault names its site; a splice hole is meaningful only in its quotation |

The constructor question is the sharp one, and the policy answers it in two
places. A *constructor unit* (the `@O.mk`-shaped wrapper) is a unit whose
body is one `cinch_alloc_ctor`; the constructor itself is already emitted
inline wherever a `case` or a literal builds a block, and the wrapper exists
so the unit can be passed as a value. Such a unit stays out of the class
because the class admits calls to constructor *units* at the head of an
`App` only when the unit is itself unfoldable *and* its body is a
projection chain rather than a block build. What the goal asks for from
`Semigroup_List@O.mk` is not a copy of `ret ptrtoint(@dict)`: it is that the
frontend stop emitting a call for a static dictionary at all (section 6).

## 4 Recursive inlining, and the budget that makes it terminate

Copying a body in one operation is not enough: the copy's own calls are the
next link of the chain, and a policy that stops after one link leaves
`Eq_Nat.==` as `icmp eq (natCmp a b) 0` with `natCmp` still a call, which is
half the goal. The rule is therefore recursive:

1. Substitute the body with fresh binder ids and the actual arguments.
2. Push the callee's name on the inline stack.
3. Inline the copy's body, with the callee on the stack and the depth
   incremented.
4. Pop the callee.

Three budgets bound the recursion, and only the third needs a decreasing
argument:

- **Nodes.** Every copy that can grow the program charges its node count
  against the program-wide budget (two percent of the program, at least a
  thousand nodes). A pure class body (no decision, no unit call) charges
  nothing: replacing `call @f` and the callee's frame with `icmp` shrinks
  the module, and charging it is what made the first measurement stop after
  5,786 copies with four thousand sites left. A decision-bearing or
  chain-bearing copy charges, because that is the shape that grows.
- **Depth.** `inline_depth` (6) caps the chain length. The class's own
  instantiations are one to three links; the cap is slack, and it bounds the
  cost of a pathological chain of admitted wrappers.
- **The stack.** A call to a unit already on the stack is refused. This is
  the rule that makes an unbounded unfolding impossible even if every other
  bound were removed: a cycle of units each in the class would otherwise
  unfold forever. Because the stack is per unfolding, two sibling calls in
  one body each get their own chain.

Units in a recursive group are refused entry to the class, so a
self-recursive function never reaches the stack rule. The stack is for
mutual chains the class admits by construction and for the second link that
closes a dictionary method's delegation.

## 5 Loops

Core has no loop form. A source loop becomes a tail worker (a unit whose
name ends `#tail`, reached by a jump), and a call inside a loop body is one
syntactic call site in that worker's body. Inlining it copies the body once
into the worker: the code is emitted once and executed per iteration, which
is exactly what an out-of-line call did, minus the call. Nothing in this
policy makes a copy per iteration, and the node budget counts each syntactic
site once. A unit in a recursive group is refused, so a worker that calls
itself is not unfolded at all.

## 6 What is deliberately not inlined, and the cross-module limit

- **Large functions.** Over the caps in section 2. `string-compare` is the
  standing example: 12 Core nodes in the seed, but a body with two
  `case`s, two `Release`s (after counting) and two runtime calls, which
  lowers to about 60 instructions. Copying it to 900 sites is a size
  decision the numbers make, not the class (measurement in section 8).
- **Recursive groups.** A member of a strongly connected component of the
  call graph, or a unit with a self edge.
- **Effects whose duplication changes semantics.** An allocation, a
  dictionary construction, a closure, a lazy thunk, a storage effect
  (section 3). A copy of a strict call is a call per execution either way,
  so a `Foreign` application is safe; a copy of an allocation is one block
  per execution instead of one block per definition site, which is why the
  class refuses it.
- **Code whose duplication multiplies allocation sites.** `InstDict` and
  constructor applications, as above.
- **A body the program does not contain.** This is the self-hosted
  compiler's limit and it is a build fact, not a policy choice. The
  compiler program is 46 modules (`Bootstrap.*`, `Main`, `Text.*`); the
  prelude (`Data.*`, `Algebra.*`) is a pinned library object, and its
  interfaces carry signatures and representation descriptors, not bodies.
  A call from the compiler's own code to `Data.Bool.not` therefore has no
  body to unfold in that compilation, and no inlining policy can remove it.
  The seed is unaffected: it compiles the prelude into the same program, so
  the same call has a body there.

  Three ways out, in increasing cost, none of them part of this change:

  1. Compile the prelude into the compiler's own program (the seed already
     does; the self-hosted chain would need the prelude sources in its
     bundle, and the pinned object would become redundant).
  2. Carry unfoldable bodies in the interface (`body` for a unit that is in
     the class, the shape the compiled-interface-metadata design already
     reserves for `total_body`) and let the inliner read them.
  3. Link with LTO so the object's `linkonce_odr` prim wrappers inline at
     link time. This is what makes the residual calls cheap today rather
     than free; it is not a substitute for the policy, because the wrapper
     still allocates a frame and blocks the caller's own folding.

## 7 Where the policy lives

- `compiler/src/simplify.rs`: `unfoldable` computes the class over
  `program.fns` with the recursive set excluded and the transitivity
  memoized. `candidate` consults it instead of `is_cheap` for the many-site
  rules. `inline_unit` carries the stack, depth and node budget and
  re-enters itself on each copy.
- `bootstrap/Bootstrap.RuntimeIr.cinch`: `unfold-shape` is the same
  predicate over `CoreTerm`, returning the shape (chain, decisions) the cap
  depends on; `inline-class-candidate` adds the cap and the direct
  self-reference refusal; `inline-class-names` appends the class to the
  size-rule names the plan already had. `inline-call` pushes the callee
  onto the plan's inline stack, emits the copy, and pops it, so the copy's
  own calls are offered the same unfolding and the next site starts from an
  empty stack. Both stay behind the existing switch until the numbers in
  section 8 say otherwise.

  Two deliberate differences. The self-hosted class refuses a decision whose
  scrutinee is a parameter, because the emitter has no site-side fold test;
  only a decision over a constructor application inside the body is
  admitted. And the self-hosted class refuses a body that names itself
  directly; an indirect cycle is broken by the emission stack, so it unfolds
  one level instead of none.

## 8 How this is measured

Per callee before and after: call count in the emitted IR, by demangled
name. Program-wide: IR lines, `cinch_alloc_ctor` sites, count operations,
and emit wall and CPU time in interleaved pairs. The seed is measured first
because its oracle is cheap (it emits the compiler in about 15 s) and
because its program contains the prelude bodies, so the class's effect is
visible there without a build change.

Seed, same tree and same binary family, only `simplify.rs` differs:

| callee | before | after |
|---|---|---|
| `Data.Machine.Eq_I64.==` | 1245 | 1 |
| `Data.Nat.Eq_Nat.==` | 1815 | 1 |
| `Data.Nat.Add_Nat.+` | 2253 | 1 |
| `Data.Bool.not` | 32 | 0 |
| `Data.String.string-compare` | 784 | 784 |

The one remaining call per unit is its own apply bridge. IR 6,203,083 to
6,241,065 lines (+0.6 %), allocations unchanged at 794. Every removed
wrapper becomes one `cinch.prim.*` call, which is `linkonce_odr` and
inlined by the backend, so `Bool.not` at a site is one `xor` after the
backend's inline, not before it. Emit wall, three interleaved pairs: base
15.04, 16.76, 16.02 s; new 17.59, 11.98, 13.60 s, with CPU medians of 12.9
and 12.5 s. The spread exceeds the effect, which is expected: the inliner
is a small pass over a 1.6 M node program.

Self-hosted, the compiler's own bundle through
`--emit-ir-with-interfaces`, switch off then on:

| policy | lines | units | calls | allocations | dup+drop |
|---|---|---|---|---|---|
| switch off | 983,827 | 6646 | 191,395 | 15,085 | 132,651 |
| old `--inline` | 988,982 | 6573 | 192,252 | 15,187 | 133,946 |
| class, cap 40, decisions, ctor uses | 1,019,467 | 6204 | 204,930 | 17,122 | 143,297 |
| class, cap 12, no ctor use, no decision | 991,124 | 6445 | 194,509 | 15,182 | 135,535 |

The wide class cost more than it saved: +3.6 % IR and 2,038 more allocation
sites, because a constructor use read as a chain link and the 40 node cap
copied twenty node bodies with calls in both branches. The narrow class is
still a loss on this module: +0.74 % lines, +97 allocation sites and 2,884
more count operations, for 201 pruned units. It therefore stays behind the
switch, off by default, and the seed keeps the class on because its program
holds the bodies the class exists for.

The named residuals do not move on the self-hosted side at any setting:
`Eq_I64.==` 726, `string-compare` 651, `Eq_Nat.==` 587, `Bool.not` 413,
`Nat.Add` 328, `Semigroup_List@O.mk` 856. They are calls into the pinned
library object, whose interfaces carry no bodies, so there is nothing for
an inliner to unfold (section 6).

## 9 The decision inputs, mapped onto our compilers

The class in section 3 is one rule. GHC's inliner is a decision procedure
with eight inputs, and this section maps each onto the seed
(`compiler/src/simplify.rs`) and the self-hosted backend
(`bootstrap/Bootstrap.RuntimeIr.cinch`), saying for each whether we have it,
lack it, or the input has no counterpart. Citations are to
`/Users/ian/Code/ghc` as it stands, where the machinery is split across
`GHC/Core/Unfold.hs` (guidance), `GHC/Core/Unfold/Make.hs` (construction),
`GHC/Core/Opt/Simplify/Inline.hs` (`callSiteInline`, `tryUnfolding`),
`GHC/Core/Opt/Simplify/Utils.hs` (`interestingCallContext`,
`interestingArg`, `preInlineUnconditionally`), and
`GHC/Core/Opt/OccurAnal.hs`.

Two corrections to the brief that the mapping depends on. The current tree
has no `CompulsoryUnfolding` or `StableUnfolding` constructor: both are
`CoreUnfolding` with `uf_src :: UnfoldingSource` being `CompulsorySrc` or
`StableUserSrc`/`StableSystemSrc` (`GHC/Types/Basic.hs:1084-1104`), and
`ug_grow` is gone, replaced by discounts (`Note [Discounts and
thresholds]`, `Unfold.hs:997`). And the thresholds are 750 for creation and
90 for use, not 90 and 60 (`Unfold.hs:94-127`). Also, a `DFunUnfolding` is
*never* unfolded at a call site: `callSiteInline` returns `Nothing` with the
comment "Never unfold a DFun" (`Inline.hs:117`), and `smallEnoughToInline`
answers `False` for every constructor except `CoreUnfolding`
(`Inline.hs:62-69`). What is certain about a DFun is narrower: it stops a
worker/wrapper split (`certainlyWillInline`, `Make.hs:409`) and it counts as
a `ValueArg` in `interestingArg`. The certain-at-a-call-site class in this
tree is `UnfWhen` (no size test, gated by `enough_args` and
`boring_ok || some_benefit`, `Inline.hs:310-318`) plus `CompulsorySrc`,
which `activeUnfolding` exempts from every switch (`Inline.hs:122-130`).

| # | input | GHC | seed | self-hosted |
|---|---|---|---|---|
| 1 | unfolding kinds | `NoUnfolding`, `BootUnfolding`, `OtherCon`, `DFunUnfolding`, `CoreUnfolding` keyed by `uf_src` (vanilla, INLINE, INLINABLE, wrapper, compulsory), each with a guidance (`Core.hs:1760-1819`) | **partial**: no record at all, but the body of every unit in the program is in `program.fns`, so "vanilla unfolding" needs no record; a body in `prim` IR is not Core and is never unfolded | **missing**: same, and the library's bodies are absent, so most dictionary methods have no unfolding in the strong sense |
| 2 | certain versus risky | `UnfWhen`: inline with no size test when saturated enough and the context is not boring; `uncondInline` mints it at `size <= 10*(arity+1)` or a trivial 0-arity RHS (`Unfold.hs:498-505`) | **present**: the unfoldable class is our certain class (primop, projection, literal, shallow decision over a known constructor, delegation), plus the zero-field constructor, whose body is a static block | **present** in `unfold-shape`, with the decision case narrower (section 7) |
| 3 | occurrence analysis | `OccInfo`: `OneOcc {occ_in_lam, occ_n_br, occ_int_cxt, occ_tail}`, `IAmDead`, `ManyOccs`, `IAmALoopBreaker` (`Basic.hs:823-858`); `preInlineUnconditionally` substitutes a once-read, not-in-lambda binder *regardless of RHS size* (`Utils.hs:1524-1528, 1653-1692`); `postInlineUnconditionally` adds trivial RHSs and dead binders (`Utils.hs:1812-1853`) | **was missing, now present**: `occurrences` per unit per pass, the two unconditional rules, and the spine condition that a strict language needs | **missing**; its inliner has no binding rules at all, only site copying |
| 4 | call context | `interestingCallContext` -> `CaseCtxt`/`ValAppCtxt`/`DiscArgCtxt`/`RuleArgCtxt`/`RhsCtxt`/`BoringCtxt` (`Utils.hs:988-1010`); `interestingArg` -> `TrivArg`/`NonTrivArg`/`ValueArg` (`Utils.hs:1134-1187`) | **now present in the argument sense**: `interesting_arg` classifies a constructor application, a literal, a lambda, a value unit, a primop; **not applicable in the continuation sense**, because the inliner is a unit rewrite with no continuation to inspect | **missing** |
| 5 | two thresholds and a size model | creation 750, use 90, sizes in tenths, allocation +10 for a boxed binder (`size_up_alloc`), case alternative +10, call `10*(1+nargs)`, constructor application 10 (0 when nullary), class method `20+10n` with a 30 discount; `small_enough = size + depth_penalty - discount <= 90` with `computeDiscount` = 10 + 10 per non-trivial argument + per-argument discounts + result discount (`Inline.hs:331-339, 709-751`) | **now present in shape** (section 10), missing the per-binder `ug_args` discount for a function-typed argument | **missing**: one flat 12-node cap |
| 6 | loop breakers and arity | SCC plus `IAmALoopBreaker` (strong and weak); a strong breaker's unfolding is trimmed to `noUnfolding` (`Types/Id/Info.hs:611-614`); eta expansion at bindings (`tryEtaExpandRhs`, `findRhsArity`) and reduction (`tryEtaReduce`, `rebuildLam`) | **present**: SCC refusal plus the inline stack; calls must be saturated (`arity == args.len()`) | **present**: SCC-free, the plan's stack is the only breaker; saturation required |
| 7 | phase control | phases, `INLINE [1]`, `activeUnfolding` per phase, `early_phase` for top-level pre-inlining | **not applicable for this corpus**: one pass to a fixed point, one monotonically decreasing budget (section 11) | same |
| 8 | cross-module unfoldings | `.hi` carries `IfCoreUnfold`/`IfDFunUnfold`, exposed when `stable \|\| profitable \|\| explicitly_requested` and never when the guidance is `UnfNever` (`Iface/Tidy.hs:709-766`); a non-stable unfolding's guidance is recomputed on load (`IfaceToCore.hs:1904-1914`) | **missing**: the seed compiles the prelude in, so it never needs one | **missing, and decisive**: the library object ships contracts, not bodies (section 12) |

Notes on the rows that are not simply "present" or "missing":

- **Row 1, what a stable unfolding would be for us.** A `.cinchif` entry
  per unit that carries (a) the body as a Core snippet, (b) its class shape
  (chain links, decisions, allocation sites) and (c) an arity, so a caller
  can run the same site test it runs for a local unit without re-checking
  anything. It belongs beside the contract in the unit's interface, keyed by
  unit name, under a schema version bump, and its identity has to follow the
  justified/assumed rule the prim design already uses: an *assumed* body is
  contract (a body edit stales every dependent), a *justified* body is
  representation (a better body does not stale anyone). That choice is the
  whole risk of the mechanism, so it is decided before the schema is
  written, not after.
- **Row 3, the spine condition.** GHC substitutes a once-read RHS wherever
  it is read, because in a lazy language a substitution cannot lose work:
  the RHS is a thunk that is forced where it is used. In a strict language
  it can: `let x = expensive in case c of True -> x; False -> 1` would drop
  the work for `False`. So our rule adds "every read is on the spine", which
  is a syntactic test over the same walk that counts the reads. GHC's
  `IsInsideLam` arm has the same character (`occ_int_cxt = IsInteresting`
  plus `canInlineInLam`, `Utils.hs:1689-1691`); ours is stricter because it
  must be.
- **Row 4, why the continuation sense is not applicable.** A GHC simplifier
  rewrites an expression inside a continuation, so it knows whether the
  value is about to be scrutinised (`CaseCtxt`), applied (`ValAppCtxt`), or
  discarded (`DiscArgCtxt`). Our inliner rewrites a whole unit body at once
  and the `pass` that follows re-examines nodes with a `Scope`; the
  information a continuation would carry is recovered a pass later by
  `known_constructor` and `case_merge`. That delay costs nothing measurable
  (section 13) because the pass runs to a fixed point anyway.
- **Row 6, why eta is not applicable.** Our lowering never produces an
  unsaturated call to a unit: a unit value is a closure (`Global` at a value
  position), and every call site the inliner sees is an `App` whose argument
  count equals the callee's arity, which is checked before any copy. GHC
  needs eta expansion because `f x` may be a partial application of a
  higher-arity `f` whose unfolding then cannot fire; we have no such shape.

## 10 The size model

GHC counts in tenths of a node and subtracts the call's cost before
comparing (`Unfold.hs:402, 560-767`; `Inline.hs:331-339`). We adopt the same
scale so the constants are comparable, with our own prices:

| item | cost | why this price |
|---|---|---|
| literal, variable, type | 0 | a word in a register; the constant is emitted once |
| `Prim` application | 10 | one instruction sequence at the site |
| `Foreign` application | 20 | a real call remains |
| unit call | 10 | one `call` |
| constructor application with fields | 60 | one `cinch_alloc_ctor` plus its descriptor load; a duplicated allocation is the worst thing a copy can do here, so it is priced six times a call |
| zero-field constructor | 0 | a static block address |
| projection | 10 | one load |
| decision (`Case`), per alternative | 10 | one compare and branch |
| `Let` of a value | 0 | the binder is eliminated by the occurrence rules |
| closure (`Lam`) | 30 | one allocation |

The site test is GHC's shape:

```
size = sum of node costs
discount = 10 (the call the copy removes)
         + 10 per argument that is not trivial
         + 10 per interesting argument
small_enough = size - discount <= use_threshold
```

with `use_threshold = 90`, `creation_threshold = 750` on the same scale, and
the certain class bypassing both. An interesting argument is one of GHC's
`ValueArg` shapes that we have: a constructor application, a literal, a
lambda, a unit whose body is a value, a primop.

The creation threshold is not decoration. It is what stops a large body from
being *considered*: a body over 750 is never in the table, so no site test
runs for it, which is the difference between a policy that costs a comparison
per site and one that costs a walk per unit.

## 11 Phase control

GHC's simplifier runs phases so that a rule can be limited to the first pass
and so that inlining decreases as the program settles (`INLINE [1]`,
`activeUnfolding`, `early_phase` at `Utils.hs:1688`). We have one pass
repeated to a fixed point with **one monotonically decreasing budget**, and
the measurement (section 13) says that is enough for this corpus, with one
correction to the original guess: the *certain* class reaches its answer in
two iterations (the named residuals and `string-compare` are identical at
two and at four), while the risky path keeps finding copies in passes three
and four, which is worth 0.2 % of lines and 1,868 calls. So the iteration cap
earns its keep, and a `INLINE [1]` style phase restriction is still not
needed: what GHC gets from phases, our budget already gets from monotone
decrease plus repetition, and no measurement shows a rule that must fire
early and not later.

## 12 Cross-module unfoldings: two mechanisms, costed

The census in section 8 is 3,561 calls into the pinned library object from
the compiler's own module (`Eq_I64.==` 726, `string-compare` 651,
`Eq_Nat.==` 587, `Bool.not` 413, `Nat.Add` 328, `Semigroup_List@O.mk` 856).
Every one of them is a body the policy would unfold if the body were in the
program, and the seed proves what that is worth: with the bodies present it
removed 4,000+ wrapper calls at +0.6 % IR and no extra allocation site.

**Mechanism A, unfoldings in the interface.** A `.cinchif` gains an
optimisation section: for each unit in the unfoldable class, the body as a
Core snippet, its class shape and its arity. The loader assembles them into
a table of "foreign bodies" that `inline_table` consults exactly as it
consults `program.fns`. Cost: a schema addition plus a loader plus about
eighty lines in the inliner; the snippets are small (a prim wrapper is one
node, a delegation two, the largest class member in the prelude is seven).
Risk: the identity question of row 1, which the existing justified/assumed
split already answers. This is the user's "serialisation for library
interfaces" and it is the only mechanism that reaches a *user program*
linking the same object.

**Mechanism B, link the library's IR.** Build the pinned object and the
runtime archive as bitcode and let `clang -flto` see every body. Cost: the
runtime archive is Rust, so it needs `-C linker-plugin-lto` and a matching
LLVM; the library object is built by the *pinned* producer, so its bitcode
schema follows the producer's LLVM version, and the stack-probe fault is
parked on this path. Benefit: none of our class rules run (a case over a
known constructor, a dictionary hop removal and the fold of a `mk` call are
Core-level rewrites); LLVM would only inline the wrapper and keep the case.

**Recommendation: build A first.** A is smaller, does not touch the pinned
toolchain, and gives the *same* policy a body to work on; B is a link-flag
complement that already helps the prim wrappers (`linkonce_odr`, inlined at
`-O2` today) and should be revisited only for what LLVM can do that Core
cannot, which the census does not show. The order also matches the evidence
we have: the seed's own numbers measure the win of A, and no number we have
measures a win for B alone.

## 13 What this round changed, and what it measured

Measured on the compiler's own bundle (46 modules plus the prelude, which the
seed compiles into the same program), `emit-ir`, one lane, one machine. The
baseline is main before this round; each row adds one change.

| stage | lines | units | calls | allocations | calls to wrappers | `string-compare` |
|---|---|---|---|---|---|---|
| baseline | 6,314,294 | 15,793 | 878,696 | 801 | 80,834 | 788 |
| + occurrence rules (wide) | 6,251,057 | 15,571 | 880,359 | 801 | 80,871 | 792 |
| + static-block constructors | 6,251,057 | 15,571 | 814,600 | 801 | 15,112 | 792 |
| + two thresholds (90) | 6,200,539 | 16,020 | 815,670 | 801 | 15,469 | 712 |
| + pre-inline restricted to movable values | 6,229,607 | 16,116 | 813,222 | 801 | 15,437 | 708 |
| use threshold 400 instead of 90 | 6,193,685 | 16,015 | 815,738 | 801 | 15,435 | 785 |

A wrapper is a function whose body is at most three instructions, which is
the shape a dictionary method or a one line helper has; the baseline's
80,834 calls to wrappers are the calls an unfolding policy exists to remove
at the use site.

What the numbers say, one change at a time:

- **Occurrence rules** (`preInline`, `DeadBinder`): 1.0 % fewer lines and
  222 fewer units for no change in calls or allocation sites. Substituting
  a one-shot binding removes its slot and lets `prune` drop the unit that
  bound it. The rules fire on the compiler's own code, not just in tests:
  the Core dump's `@ticks` moves on every module.
- **Static-block constructors**: 65,759 calls removed, 7.5 % of every call
  in the module, with the line count, the unit count and the allocation
  sites all unchanged. This is the largest single result of the round and it
  is not an inliner change: a zero-field constructor's unit is
  `ret ptrtoint @"U.blk"`, and the emitter was calling it. A hand-written
  compiler emits the address. GHC's equivalent is that a data constructor
  has no unfolding at all and Core-to-STG builds the block directly; its
  `sizeExpr` prices a nullary constructor application at zero
  (`Unfold.hs:855-864`). The rewrite is scoped to a whole-program module: an
  object build, or a module emitted without its neighbours, may not define
  the constructor's unit at all, and there the call stays (that is the
  `.blk` undefined-symbol failure the unscoped version produced in the
  package install path). The address is emitted on first use and exactly
  once, so a module that does define the constructor's unit cannot get two
  definitions.
- **The two thresholds**: the model is 0.8 % smaller than the previous flat
  caps with 0.13 % more calls, no allocation change, and 227 more units kept
  (the use threshold of 90 tenths is 9 nodes, tighter than the old 40 node
  single-site rule). The named residuals in section 1 stay at their apply
  bridges, and `string-compare` falls from 788 to 712 because the copies at
  known-constructor sites fold.
- **The pre-inline restriction** (a movable right hand side only) costs 29,068
  lines against the wide rule and buys 2,448 calls. It is not a tuning
  choice: with the wide rule a memo cell and a call were substituted at
  their one read, which changes when the work happens in a strict language.
  The comment in the code states the deviation from GHC and why.
- **The threshold is not a sensitive knob here**: 400 tenths instead of 90
  buys 6,854 lines and costs 68 calls. 90 is kept because it is GHC's
  constant and no measurement prefers another.
- **Iterations.** Two passes instead of four (the default) is 12,619 more
  lines and 1,868 more calls, with the same named residuals and the same
  `string-compare`: the certain class settles in two, the risky path keeps
  finding work. The cap of four is therefore doing something measurable, and
  no phase restriction is needed to bound it.
- **No allocation site was ever duplicated** by any decision in this round:
  `cinch_alloc_ctor` is 801 in every row. That is the property the model's
  60-tenths allocation price exists to protect, and it is worth stating
  because the previous round's wide class added 2,038 of them.

## 14 The interface optimisation artifact

Mechanism A of section 12 is implemented as a second artifact beside the
contract, not as a fatter `.cinchif` (the reason is in section 2.2 of
`design/compiled-interface-metadata.md`: a body must not move the identity a
dependent keys on):

```text
build/<Module>.cinchif   the contract (schema 10, unchanged)
build/<Module>.cinchopt  the unfoldings (schema 1)
```

`compiler/src/ifaceopt.rs` defines it. It records the module, the
`iface_hash` it was produced against, the runtime ABI, and one row per unit:
arity, parameter representation classes and the body. The body encoding is a
canonical tagged JSON form for exactly the unfoldable class (section 3): a
node outside the class makes the writer skip that body, and the reader
refuses an unknown tag. The writer runs in `write_artifacts` beside every
`.cinchif`, filters by `simplify::unfoldable_names` (one class computation
per program), and writes no artifact for a module with no class body. The
artifact's own content hash is recorded in the package execution interface
(`build/package.cinchif`) and travels with the package on install.

The consumer is `--interface-optimisations <manifest>`, where the manifest is
the interface manifest a consumer already uses. It loads each row's sibling
`.cinchopt`, refuses one whose module or `iface_hash` disagrees with the row
(inlining against a stale body is wrong code, not a missed win), and turns
the bodies into a foreign table (`ifaceopt::load_manifest`). The driver then
specializes *with the library's bodies still present*, because that is where
a dictionary projection becomes a direct call to a method unit; only then
does it drop the units the artifacts describe, and run
`simplify::simplify_with_foreign`. The dropped units are not emitted by this
compilation: the caller links the object the artifacts describe.

Measured on the compiler's own 46 module bundle, at the post-specialization
Core point (the point an artifact's bodies describe), with the library units
made external:

| measurement | no artifact | artifact |
| --- | --- | --- |
| artifact units | 0 | 359 |
| cross-object calls | 15,759 | 10,895 |
| Core nodes | 1,516,062 | 1,526,369 |
| `Data.Machine.Eq_I64.==` | 532 | 0 |
| `Data.Nat.Eq_Nat.==` | 890 | 0 |
| `Data.Bool.not` | 52 | 0 |
| `Data.Nat.Add_Nat.+` | 1,272 | 0 |
| `Data.String.string-compare` | 1,037 | 1,037 |

So the class removes 2,746 wrapper calls (4,864 cross-object calls including
the secondary chains), and the copies cost 10,307 nodes, 0.68 per cent of the
no-artifact program, while the program as a whole is 77,241 nodes smaller
than before the simplifier because the duplicates the inliner already made
are now made once per site instead of once per definition plus copies.
`string-compare` does not move: it has two decisions and is outside the
class, which is the size decision section 6 records.

The emitted IR of an external-library build is 149 calls and 49 KB smaller
than the whole-program build of the same bundle, and neither defines nor
calls a wrapper unit: the whole-program build had already inlined them, and
the artifact build reaches the same residual.

### When the artifact is written, and what it costs

Writing is default only for `--emit-object`, the mode that produces a
library for someone else: its consumer may read the artifacts, and the cost
is small against a library build. Every other mode writes them only with
`--emit-interface-optimisations`, so a check of a user program pays
nothing. Measured on the 46 module compiler bundle (105 interfaces, 65 with
a class body):

| item | value |
| --- | --- |
| `.cinchif` bytes | 3,500,263 |
| `.cinchopt` files / bytes | 65 / 202,025 (5.8 per cent of the contracts) |
| class pass (`unfoldable_names`) | 96 ms |
| build and encode all 65 | 179 ms |
| total added to a compile that writes them | 275 ms |

Reading is opt-in and cannot be default today: the Rust driver loads no
interface set of its own (it embeds the prelude and consumes packages as
sources), so there is no manifest to read implicitly. A user with a prebuilt
library passes `--interface-optimisations <manifest>`; the manifest is the
same `[module, iface_hash, path]` list the library already publishes, and
each row's sibling `.cinchopt` is read only if it is present. A row without
one behaves exactly as before.

The self-hosted compiler's inliner (`Bootstrap.RuntimeIr.cinch`) is not a
consumer yet; section 15 sizes that gap.

## 15 The self-hosted gap, sized

The stage chain's inliner is `Bootstrap.RuntimeIr.cinch`, which works on
`CoreTerm`: every node carries at least one `UType`, and some carry two
(`CApplyEff`, `CErasedApply`, `CDecision`) or a `Scheme` (`CLet`). The
artifact's bodies are untyped, so a consumer either gets types in the
artifact or infers them. Here is what each costs, counted from the base
library the Rust seed actually produces (`emit-object` on
`CinchBaseObject` with the pinned roots):

| count | value |
| --- | --- |
| modules with a class body | 31 |
| class units | 401 |
| Core nodes in those bodies | 2,721 (6.8 per unit) |
| artifact bytes, all 31 modules | 78,416 |
| `Data.Machine` alone | 216 units, 1,136 nodes, 34,653 bytes |
| `Data.Machine` source types | 820 foreign declarations, 372 distinct type texts, 1,618 distinct type nodes |

**Transport.** Every node needs an annotation, so a per-node type tree is
2,721 trees; at the class's average of about two type nodes and about 18
bytes per encoded type node that is roughly 98 KB, more than doubling the
78 KB artifact. A per-module type table does better: the distinct types
across the base library are on the order of 1,000 type nodes (18 KB), plus
a two-byte index per node (5.4 KB), so about 23 KB, a 30 per cent increase.
It also needs a `UType` codec in Cinch and, because the Rust seed's `Core`
carries no types at all, a Cinch producer that writes the section.

**Re-elaboration.** Zero artifact bytes and no producer change: the
untyped body the Rust seed already writes is enough, because the consumer
has the schemes. `Bootstrap.Contracts.interface-headers` already parses
every interface scheme with `Text.Parse.parse-scheme`, and
`Bootstrap.Infer` (6,964 lines) already instantiates and unifies. A new
entry point would walk the admitted `CoreTerm` forms (17 constructors),
look up each callee's scheme, instantiate it at the unit's concrete types
and unify the argument against the parameter, producing the annotated term
the emitter wants. That is about 300 to 500 lines, once, at load, over
401 small bodies; a body that fails to infer is skipped.

Re-elaboration is therefore the smaller of the two by every measure that
matters here: 0 bytes instead of +23 to +98 KB, no producer change instead
of a Cinch producer mode plus a type codec, at the price of a bounded
inference entry point in the consumer. The artifact's reader already skips
a body whose encoding it cannot decode rather than refusing the module, so
a future typed form and this inference form can coexist while both are
implemented.
