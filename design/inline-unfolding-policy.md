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
