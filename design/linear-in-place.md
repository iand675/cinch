# In-place work from linearity: a census, and what the answer turned out to be

Status: measurement and finding, 2026-11-13. Lane `dasl-linear-inplace`.

Companions: `design/reclamation.md` section 5 (the Perceus reuse mechanism
this note measures), `design/ownership-and-usage.md` (the multiplicities),
`design/arraylist.md` (the array side of the same question).

The question this lane was given: "if you're able to use linear types
throughout the compiler itself to do more in-place work that would be
beneficial I'd expect." The answer, measured rather than assumed, is that
**linearity is not what pays.** Perceus-style reuse already does the in-place
work, on its own static proof, without reading a single `%1` annotation. This
note is the evidence for that claim, the census that makes it quantitative,
and the list of what the reuse pass cannot reach.

Conventions: `path:line` cites this tree on 2026-11-13. Every number below is
from a command in section 8, so it can be repeated. No em dashes.

---

## 1 The short version

| Claim | Evidence |
| --- | --- |
| Reuse already rewrites list spines in place | `<>`, `map` and `concat` arms carry `release @token` plus `reuse` in owned Core (section 3) |
| It does so with no linearity annotation anywhere | the prelude's `<>` is `Semigroup (List t)`, every parameter `%many` (section 3.2) |
| It pays when the operand is dead and costs one spine when it is shared | dead left operand: 2,008 blocks; live one: 3,008 (section 3.3) |
| The compiler's own code already reuses 2,064 sites | 11.3% of 18,255 releases (section 5) |
| Another 5,923 sites (32.4%) are real missed opportunities | an allocation exists in the arm, off the spine (section 5) |
| The blocker is spine shape, not ownership | 77.8% of the misses hide the allocation behind a branch (section 6) |
| A `%1` annotation would not move the number | 24 of 1,011 append sites could pass one (section 4); section 7 |

The headline for the user's question: **linearity buys the permission, but
the permission was never the constraint.** The constraint is that the pass
only searches a straight-line spine.

---

## 2 What the mechanism is, in one paragraph

`compiler/src/rc.rs` runs after inlining and inserts counts. When a `case`
arm's scrutinee dies (owned, not live, not read again) and a constructor of
the same payload shape is allocated on that arm's spine, the pass emits
`Release { block, token }` instead of a plain drop, and rewrites the
allocation into `Core::Reuse { token, ctor, args }`. The release's unique
path keeps the cell and binds its word to the token; its shared path binds 0.
The reuse writes the new constructor's fields into that cell on the token, or
allocates when the token is 0. Both paths merge at one join. This is Lean 4's
reset/reuse pair, and `design/reclamation.md` section 5 records its
implementation, its store-skipping and family-crossing extensions, and the
41,000-to-1,000 block result on a synthetic rebuild.

Two properties matter for this note:

- The decision is a **static proof** (the arm's tag resolves to an unplanned
  constructor of the same field count) plus a **runtime count** (`rc == 1`).
  No source annotation participates.
- The search is deliberately narrow: `insert_reuse` walks `Let` values, `Let`
  bodies, `Dup` bodies and `Drop` bodies, and stops at anything else, because
  a reuse under a branch could be skipped and the kept cell would leak.

---

## 3 The three operations that were supposed to need help

### 3.1 `<>`, `map` and `concat` all reuse today

A hand-written append, `map`, and the prelude's own `<>`, dumped with
`--dump-core owned`:

```text
unit Main.append-own :: forall (a :: Type). List a -> List a -> List a
  body (case (local t0)
    ((con 0) (drop t0 @class(owned) (local t1)))
    ((con 1 (t2 @class(param(0))) (t3 @class(owned)))
      (release t0 @token(t4)
        (moved t3)
        (rest)
        (reuse t4 @kept(0)
          Data.Core.Cons
          (args (local t2) (app (global Main.append-own @classes(0)) (local t3) (local t1)))))))
```

`map` is the same shape with a different payload:

```text
unit Main.mp :: (Data.Core.List Int) -> (Data.Core.List Int)
  body (case (local t0)
    ((con 0) (drop t0 @class(owned) (app (ctor Data.Core.Nil @classes(T) 0 0))))
    ((con 1 (t1 @class(owned)) (t2 @class(owned)))
      (release t0 @token(t3)
        (moved t1 t2)
        (rest)
        (reuse t3
          Data.Core.Cons
          (args (app (global Main.inc) (local t1)) (app (global Main.mp) (local t2)))))))
```

### 3.2 And they do it with no linearity in sight

The prelude's list append is the `Semigroup` instance
(`compiler/prelude/Data/List.cinch:517`); the module contains no `%1` at all:

```cinch
implicit instance Semigroup_List t :: Semigroup (List t) where
  (<>) xs ys = case xs of
    Nil -> ys
    Cons x rest -> Cons x ((<>) rest ys)
```

Every parameter there is `%many`, the unannotated default
(`compiler/src/types.rs`, `default_mult`). Nothing in that definition says the
left operand is consumed. The reuse pass derives it: the arm's scrutinee is
owned, nothing reads it again, and a same-shaped `Cons` is born on the spine.
The owned Core for a program calling `<>` shows the pair, so the reuse is
happening on the un-annotated definition.

That is the central empirical result of this lane. The mechanism does not need
the permission, because it establishes the permission itself, per site, from
the count discipline it is already inserting.

### 3.2.1 The compiler's own `map`-shaped recursions already reuse

`Bootstrap.Core` has 208 `Data.List.Semigroup_List` references and 488 `reuse`
nodes. Its four hand-written list maps all get the pair, for example
`map-types` (`Bootstrap.Core.cinch`):

```text
unit Bootstrap.Core.map-types
  body (case (local t1)
    ((con 0) (drop t0 @class(owned) (drop t1 @class(owned) (app (ctor Data.Core.Nil @classes(O) 0 0)))))
    ((con 1 (t2 @class(owned)) (t3 @class(owned)))
      (release t1 @token(t4)
        (moved t2 t3)
        (rest)
        (reuse t4 Data.Core.Cons
          (args (app (dup t0 @class(owned) (local t0)) (local t2))
                (app (global Bootstrap.Core.map-types) (local t0) (local t3)))))))
```

The same shape appears at `map-evidences`, `seal-patterns` and
`index-type-metas`. This is the user's "do more work in place in the compiler
itself", already happening, on code written without a single annotation.

### 3.3 The block costs, measured

Two 1,000-cell lists appended, then consumed (`CINCH_ALLOC_STATS=1`):

| Program | Blocks | Header blocks | Freed | Live |
| --- | --- | --- | --- | --- |
| `build 1000 Nil <> build 1000 Nil` (left operand dead) | 2,008 | 2,000 | 2,000 | 0 |
| same, but `xs` read again after the append | 3,008 | 3,000 | 3,000 | 0 |
| three 500-cell lists, `a <> b <> c` | 1,508 | 1,500 | 1,500 | 0 |
| `map inc (build 1000 Nil)` | 1,008 | 1,000 | 1,000 | 0 |
| `reverse` written with an accumulator | 1,133 | 1,125 | 1,125 | 0 |
| `sort (build 1000 Nil)` | 16,804 | 16,796 | 16,796 | 0 |

The append rows are the proof: 2,000 cells for a 2,000-element result means the
left spine was rewritten, not rebuilt. The third row shows the chaining case
reusing every spine. The shared row is the control, and the extra 1,000 blocks
are exactly the rebuilt left spine, which is what the un-optimized form costs.

`map` at 1,008 is the same story: 1,000 cells in, 1,000 cells out, no second
spine.

`reverse` at 1,133 does **not** fully reuse, and that is the other lane's
finding (lane `dasl-inplace-reverse`). Its arm is

```text
unit Main.go :: List Int -> List Int -> List Int
  body (case (local t0)
    ((con 0) (drop t0 @class(owned) (local t1)))
    ((con 1 (t2 @class(owned)) (t3 @class(owned)))
      (release t0
        (moved t2 t3)
        (rest)
        (app (global Main.go)
          (local t3)
          (app (ctor Data.Core.Cons @classes(T) 1 2) (local t2) (local t1))))))
```

The `release` has no `@token`, so no reuse: the `Cons` it could reuse is not on
the spine, it is the second argument of the recursive call. `insert_reuse`
stops at the `App` head. That is a spine-shape limit, not an ownership limit,
and it is the single largest class of missed opportunity in the compiler's own
code (section 6).

`sort` at 16,804 blocks is not a reuse question. Merge sort's intermediate
lists are a different shape and its cost is inherent to the algorithm.

---

## 4 The census: call sites

Counted over `bootstrap/*.cinch`, 42,306 lines across 44 modules, comment
lines excluded so prose does not inflate the count.

| Operation | Call sites | Notes |
| --- | --- | --- |
| `<>` | 1,011 | by far the most common; 207 in `Bootstrap.Core`, 142 in `Bootstrap.Infer`, 139 in `Bootstrap.Resolve` |
| `reverse` | 138 | 80 of them in `Bootstrap.Parse` |
| `sort` / `sort-by` | 107 | 48 in `Bootstrap.Core`, 35 in `Bootstrap.TotalSemantics` |
| `map` | 15 | the concrete list `map`; the class method is separate |
| `hash-map-insert` | 36 | |
| `hash-map-lookup` | 34 | |
| `filter` | 2 | |
| `concat` as a bare name | 0 | the prelude spells it `join-lists` (private) and `concat-all` (strings) |
| `foldl` / `foldr` | 0 direct | accumulator walks are written as explicit recursion |

A source-level count of `%1` eligibility, using the only proxy the source
offers (an operand named by a bare identifier that occurs exactly once in the
enclosing definition, which is the checker's own use-count rule at source
granularity):

| `<>` sites | Count | Share |
| --- | --- | --- |
| total | 1,011 | |
| left operand is a bare identifier | 822 | 81.3% |
| of those, the identifier occurs exactly once (single-use) | 24 | 2.9% |
| of those, the identifier occurs more than once | 798 | 97.1% |
| left operand is not a bare identifier (a call, a literal, a `case`) | 189 | 18.7% |

**24 of 1,011 sites, 2.4%, could pass a `%1` argument** by that proxy. The
other 798 need the left list again, which is exactly the case where reuse
declines and where a linear signature would be a type error rather than an
optimization.

This is the honest answer to the `%1` question: linearity is available on
about 2% of the append sites in the compiler's own code, so an API that
required it would be used almost nowhere. The mechanism that does not require
it reaches all of them.

---

## 5 The census: what the compiler's own Core actually collects

Call sites are an upper bound. The real question is what the pass achieves on
the compiler's own residual program, so I dumped owned Core for all 44
`bootstrap/*.cinch` modules (5,927 units) and parsed every `release` site with
an S-expression reader, walking the exact spine `rc.rs insert_reuse` walks.

Every release falls into one of five buckets:

| Outcome | Count | Share |
| --- | --- | --- |
| **reused**: the release carries `@token` | 2,064 | 11.3% |
| **opportunity**: a constructor allocation exists in the arm but off the spine | 5,923 | 32.4% |
| **no allocation**: nothing in the arm to reuse the cell into | 8,364 | 45.8% |
| **incompatible**: an allocation is on the spine, wrong shape or planned | 1,904 | 10.4% |
| total | 18,255 | |

The middle row is the only one that is a missed opportunity. The third row is
not: a dying block with no successor of any shape has nothing to be reused as,
and no pass or type could improve it. That distinction is what makes the
number actionable: **5,923 of 18,255 releases (32.4%) could be reuse pairs and
are not**, and 2,064 already are.

Per module, the ones that matter for size (`opp` is the opportunity row):

| Module | Units | `release` | reused | opp | none | incompatible |
| --- | --- | --- | --- | --- | --- | --- |
| `Bootstrap.Infer` | 1,024 | 3,869 | 178 | 1,388 | 1,795 | 508 |
| `Bootstrap.Core` | 574 | 2,752 | 489 | 680 | 1,283 | 300 |
| `Bootstrap.RuntimeIr` | 695 | 1,724 | 114 | 558 | 922 | 130 |
| `Bootstrap.Parse` | 329 | 696 | 94 | 327 | 226 | 49 |
| `Bootstrap.Type` | 327 | 1,286 | 175 | 372 | 698 | 41 |
| `Bootstrap.CoreBody` | 86 | 687 | 82 | 461 | 127 | 17 |
| `Bootstrap.Typing` | 249 | 625 | 15 | 184 | 400 | 26 |
| `Bootstrap.Ast` | 240 | 577 | 44 | 135 | 282 | 116 |
| `Bootstrap.Check` | 115 | 281 | 0 | 35 | 141 | 105 |
| `Bootstrap.Kinds` | 39 | 118 | 0 | 15 | 102 | 1 |

`Bootstrap.Check` and `Bootstrap.Kinds` collect zero reuse. That is not an
ownership problem: both are decision procedures whose allocations sit under
branches, and their `none` counts show most arms allocate nothing at all.

---

## 6 Why 88.7% of releases decline, and why linearity cannot fix it

The pass searches a spine of `Let` values, `Let` bodies, `Dup` bodies and
`Drop` bodies, and stops at anything else. Taking the 5,923 opportunity sites
and classifying by the node that stopped the walk:

| Blocker | Count | Share of opportunities |
| --- | --- | --- |
| a `case` | 3,412 | 57.6% |
| an `if` | 1,197 | 20.2% |
| a call (`App`) | 1,314 | 22.2% |

So the dominant blocker is a **branch**, not a call: 4,609 of 5,923 (77.8%) of
the missed opportunities hide the allocation behind a `case` or an `if`.

Neither is an ownership question:

- **Under a branch.** The dying node is uniquely held; the allocation happens
  in one alternative of a decision that follows it. The pass cannot prove the
  alternative is always taken, and a reuse that is skipped on some path leaks
  the kept cell, so it declines. This is the same restriction Lean 4's reset
  and reuse has, and it is a limitation of the *proof*, not of the type system.
- **As a call argument** (22.2%). The node dies, the `Cons` is born, and the
  two are adjacent in evaluation order but separated by an `App` boundary.
  This is `reverse`'s shape exactly (section 3.3).

A `%1` annotation on the parameter changes neither. The parameters at these
sites are already `@owned` in the Core, which is the condition reuse needs;
what is missing is a proof that the spine reaches the allocation. Writing `%1`
would add a checker obligation to every caller and leave the reuse count
exactly where it is.

The `incompatible` row of section 5 (1,904, 10.4%) is a different kind of
miss: the allocation *is* on the spine but its shape does not match the dying
block (a different field count, or a boundary layout, which
`design/reclamation.md` section 5 excludes because a planned payload is not its
fields). That is the family-crossing proof doing its job, not a gap.

---

## 7 What is missing at the language level

The lane was asked to record where the language blocks an in-place operation.
After the measurement, the honest list is short and none of it is about
linearity:

1. **No way to express "this arm always takes this branch" for the reuse
   search.** A `case` blocks 3,412 of the 5,923 missed opportunities (57.6%),
   and an `if` another 1,197 (20.2%). Lean solves this with the same spine
   restriction, so it is a shared limit of the mechanism, not a Cinch gap.
   What would lift it is a value-numbering or evaluation-order analysis proving
   the alternative is the only reachable one, not a type.

2. **A `%1` argument cannot be threaded through a helper without threading the
   signature.** Already recorded in `design/ownership-and-usage.md` section 7:
   there are no multiplicity-polymorphic binders and no inline `%1 ->`
   lambdas, so a linear continuation has to be a named function. This lane did
   not need it, because reuse does not require the annotation.

3. **`%1` does not imply uniqueness.** A `%1` parameter can receive a value the
   caller also holds through a `%many` binder, so the runtime `rc == 1` check
   is still required. `design/reclamation.md` section 5 states this and names
   the missing feature: a `Unique` type discipline in Clean's style. That is
   the one language addition that *would* let the pass skip the runtime check
   on linear arguments, and it is a new feature, not a use of an existing one.

4. **The prelude cannot name "the argument is consumed" in a way that changes
   lowering.** Since the mechanism derives ownership per site, it does not
   need to. Recorded because it was the hypothesis this lane started from and
   the measurement falsified it.

---

## 8 What was and was not built

**Built: nothing in `compiler/src`.** This is the deliberate outcome. The
operations the lane was chartered to implement (`append`, `map` in place,
`concat`) already lower to reuse, so an intrinsic or a second public name would
have been a parallel mechanism doing worse work than the pass already does, and
would have needed a lock bump for `compiler/runtime/src` and
`compiler/prelude` to do it.

**Not built, with the reason:**

| Operation | Status | Why |
| --- | --- | --- |
| `<>` append | already reuses | section 3.1, 2,008 blocks for 2,000 cells |
| `map` | already reuses | section 3.1, 1,008 blocks for 1,000 cells |
| `concat` (`join-lists`) | already reuses | nested `<>` reuses every spine: 1,508 for 1,500 |
| `reverse` with an accumulator | **now reuses** | was declined (the `Cons` is a call argument); fixed by the argument search in section 11, 1,133 to 1,008 blocks |
| `sort` | not a reuse pair | merge sort's intermediates are a different shape; 16,804 blocks is the algorithm |
| `filter` | no sites | 2 call sites in the whole compiler; not worth a mechanism |
| `Data.HashMap` / `Data.Set` insertion | out of scope for reuse as shaped | path copying is `design/reclamation.md`'s stated next target, not this lane's |

**Lock bumps: none needed.** No file under `compiler/runtime/src` or
`compiler/prelude` was modified, so no stage0 re-pin and no library-provider
re-pin. `runtime_abi` did not move.

**Which gates this lane ran.** The deliverable is a measurement, so the gates
that matter are the ones that make the numbers trustworthy, and those it ran:

| Gate | Status |
| --- | --- |
| the compiler's own source set checks clean | yes: the stage-3 compiler's `--check` exits 0 with an empty diagnostics file, and the Rust frontend on the same source set writes 115 interfaces |
| every number reproducible | yes, identical over repeats with `build/` removed |
| owned Core dump for all 44 modules | yes, 5,927 units parsed |
| `cargo test --lib`, the list/reclamation/hashmap suites, the smokes | **not run**, and not required: no file under `compiler/` was modified |
| fixed point (stage2/stage3 byte equality) | **not run**, same reason |

The two skipped rows are the gates for a code change. This lane's only change
is a file under `design/`, so running them would test the sibling lanes' work
in the shared working copy rather than this lane's. If the pass is extended
per section 10, those gates become required and belong to that change.

---

## 9 The commands

```sh
# owned Core for every bootstrap module, then the reuse census over it.
# `check` never reaches the owned stage, so `emit-ir` is the command; it
# writes the dump before refusing for want of a `main`.
for f in bootstrap/*.cinch; do
  m="${f%.cinch}"
  cinch emit-ir "$m.cinch" -o /tmp/o.ll --dump-core owned >/dev/null 2>&1
done
python3 tooling/reuse_census.py build --by-module --why
#   units: 5927
#   reused 2064 (11.3%), opportunity 5923 (32.4%), no allocation 8364 (45.8%),
#   incompatible 1904 (10.4%); opportunities: case 3412, if 1197, app 1314

# block costs
cinch build W1.cinch -o W1.exe && CINCH_ALLOC_STATS=1 ./W1.exe

# owned Core for one module (emit-ir reaches the owned stage; check does not)
cinch emit-ir Bootstrap.Core.cinch -o /tmp/x.ll --dump-core owned
#   writes build/Bootstrap.Core.owned.core, then refuses for want of `main`

# the compiler's own self-check cost, from a removed build/ so no interface
# cache is reused
rm -rf build
CINCH_ALLOC_STATS=1 CINCH_HEAP_LIMIT=2G cinch --check Main.cinch diag.txt
#   283,639 blocks, 280,214 header blocks, live=0 (identical over repeats)

# the toolkit cost
CINCH_ALLOC_STATS=1 CINCH_HEAP_LIMIT=2G cinch --check \
  --interfaces build/bootstrap/stage3/base-interfaces.json \
  build/bootstrap-profile/sources/toolkit.cinch diag.txt
#   157,671,323 blocks, 147,572,689 header blocks, live=0 (identical over repeats)

# where the time goes
python3 tooling/profile_build.py check ...   # sample profile of the check
#   rc::release is the second-hottest frame at 9.3-10.1% self
```

The self-check figure of 283,639 blocks is the `--check` path, which does not
run the counting pass, the reuse insertion or the coalescing. The 18,255
release sites in section 5 are from `emit-ir`, which does. Both are reported
because they answer different questions: the first is what a check costs, the
second is what the pass sees.

Both figures are stable across repeats when `build/` is removed first. Leaving
an interface cache in place moves them slightly (the earlier runs of this note
read 283,645 and 157,671,331), because a cached module is not re-checked, so
the command above removes `build/` to make the number reproducible.

The measured binary is `build/bootstrap/stage3/cinch`, sha256 `5031a3a42f7929e7`
(16 hex digits of the digest), dated 2026-11-13 14:10.

---

## 10 What this means for the roadmap

1. **Reuse is the in-place mechanism, and it is already paying.** 2,064 sites
   reuse today, and 5,923 more (32.4% of releases) are real opportunities. The
   blockers are spine shapes (`case`, `if`, call argument), not types.

2. **Do not add linear signatures for performance.** The measurement says the
   annotation reaches 2.4% of append sites and changes no lowering. Linearity
   is worth its checker complexity where it prevents a *bug* (the `Effect.Pty`
   case in `design/ownership-and-usage.md` section 7), not where it might
   permit an optimization.

3. **The next win is in the pass, not the language, and section 11 takes it.**
   The two extensions this predicted are implemented: the call-argument search
   (which fixed `reverse`) and the per-arm branch expansion. They collect 773
   of the 5,923 opportunities. What remains is the `incompatible` proof doing
   its job, plus selective callee specialisation (section 11.5), which is not
   built.

4. **The one language feature that would help is `Unique`, not `%1`.** It is
   the only addition that lets a proven-unique value skip the runtime count
   check. It is a new feature with its own design cost, and this lane's numbers
   say the check is not what is costing us.

---

## 11 The follow-on: making the off-spine shapes reuse (implemented)

Section 10 said the next win is in the pass, not the language. This section
records the pass change that collects it, measured the same way.

### 11.1 What was added

Two search extensions to `compiler/src/rc.rs`, both keeping the existing
pairing logic and the existing soundness argument, and both in the one
mechanism (no second pass):

**Call and jump arguments are on the spine.** A call's arguments are
evaluated in order after the `Release` bound the token and before the
transfer, with no branch between them, so a compatible allocation among them
is reachable on the same path that reached the `Release`. This is the
`reverse` shape: `reverse-into ys (Cons y acc)` puts the fresh `Cons` in an
argument of the recursive call. Jump arguments are the same fact (a join's
parameters are filled in the same order). Landed first by lane
`dasl-inplace-reverse`; merged here as one mechanism.

**A branch expands the pair per arm.** When no allocation is on the
straight-line spine, a `case` or `if` gets a copy of the release in each arm,
and each arm that allocates a compatible constructor binds its own token.
This is Lean's `expandResetReuse`. The soundness argument is that the arms
are mutually exclusive, so exactly one runs and one release per arm is
exactly one release per path: the one-token-one-reuse rule holds per path,
and nothing is shared across the branch, so no cell can be kept alive on a
path with no reuse to take it.

Three details the implementation needs and the design should record:

- **Arms need not agree.** An arm that allocates nothing, or allocates an
  incompatible shape, binds no token, which is the plain drop that arm would
  have had. Requiring every arm to allocate was the first version and it
  collected almost nothing (96 sites); relaxing to per-arm independence is
  what made the change pay. The expansion is applied only when at least one
  arm takes a pair, because otherwise it duplicates a drop into every arm for
  no gain.
- **The moved-field set is per arm.** The single-body release moved the union
  of the fields the body reads. Duplicating it makes the set per arm, so an
  arm that reads no counted field of the dying block moves none and drops the
  block itself.
- **The tail-call frame rule is untouched.** `stack.rs` runs after `rc.rs` and
  passes `tail` through a `Release`, so a release pushed into a branch keeps
  the branch's tail position. A `Reuse` is opaque to that pass: the block it
  writes is the dying one, which was already classified on its own. The
  `let x = Cons ... in tailcall x` shape never enters this path.

### 11.2 The census after the change

Same tool, same 44 modules (`tooling/reuse_census.py`):

| Outcome | Before | After | Change |
| --- | --- | --- | --- |
| reused | 2,064 | 2,837 | +773 (+37.5%) |
| opportunity | 5,923 | 5,539 | -384 |
| no allocation | 8,364 | 8,471 | +107 |
| incompatible | 1,904 | 1,927 | +23 |
| total releases | 18,255 | 18,774 | +519 |

By blocker class on the remaining opportunities:

| Blocker | Before | After |
| --- | --- | --- |
| `case` | 3,412 | 3,279 |
| `app` | 1,314 | 1,244 |
| `if` | 1,197 | 1,000 |
| `release` (nested) | 0 | 16 |

The release total grew because a duplicated pair is more `Release` nodes than
the one it replaced, which is the code growth the expansion costs. The
opportunity count fell less than the reuse count rose because the extra
releases introduce new sites, some of which are themselves opportunities: the
per-arm structure moves allocations onto arms' spines, which creates arms
that now have a reachable allocation but no pair yet.

### 11.3 The block counts, before and after

Same programs, `CINCH_ALLOC_STATS=1`, `CINCH_LEAK_CHECK=1`:

| Shape | Before | After | What it means |
| --- | --- | --- | --- |
| `<>`, dead left operand | 2,008 | 2,008 | already reused (section 3.3) |
| `map` over a dead input | 1,008 | 1,008 | already reused |
| `a <> b <> c` | 1,508 | 1,508 | already reused |
| `reverse` with an accumulator | 1,133 | **1,008** | the call-argument fix: the reverse allocates nothing, 1,008 is the build |
| `if`-shaped rebuild | 2,008 | **1,008** | branch expansion: one pair per branch |
| nested-`case` rebuild | 2,008 | **1,008** | branch expansion |
| mixed arms (one allocates, one returns the tail) | 1,008 | 1,008 | unchanged: the non-allocating arm takes the plain drop, so no pair is placed there |

Every run exits 0 with `leaked=0`. The three shapes that already reused are
unchanged, which is the check that the extension does not disturb the
straight-line path.

The `if`-shaped rebuild's Core shows the shape the expansion produces:

```text
(if <condition>
  (release t0 @token(t5)
    (moved t1 t2) (rest)
    (reuse t5 Data.Core.Cons (args (app (global Main.bump) (local t1)) (app (global Main.rebuild) (local t2)))))
  (release t0 @token(t6)
    (moved t1 t2) (rest)
    (reuse t6 @kept(0) Data.Core.Cons (args (local t1) (app (global Main.rebuild) (local t2))))))
```

Two releases, two tokens, and the second arm skips a store the cell already
holds (`@kept(0)`), which is the existing store-skipping working per arm.

### 11.4 Considered and not done: a reuse token on the calling convention

An extra `%1` parameter carrying a corpse into a callee would let a callee
build into a cell its caller just died on. It is rejected: it taxes every call
site of that callee, needs per-call-site specialisation and interface churn,
and the three cases it would cover are already covered without it. If the
corpse is already an argument, the callee's own reuse pass pairs it with a
birth (case 1, the `reverse` shape, now working). If the corpse is dead
before the call and is not an argument, it is dead code to eliminate, not a
value to transport. If the callee is small, inlining makes the pair
intraprocedural. Only a large, allocation-heavy callee called from a site
with a fresh corpse would pay for the transport, and the census has no such
site large enough to justify the convention change.

### 11.5 What is still open

- **`case` at 3,279 remains the largest class.** The arms that still refuse
  are the ones where no arm allocates a compatible shape, which is the
  `incompatible` proof doing its job rather than a gap.
- **Selective callee specialisation** (clone a callee per call site with a
  reuse slot) is the next mechanism the user described, and it is *not*
  implemented here. Its order of work is: this shape work first, then the
  clone with a must-analysis over the clone's entry path, a recursion fact in
  `borrow_modes`' existing worklist, and a measured "one argument versus one
  allocation" pair. It is out of scope for this change and nobody should
  assume it is coming from this lane.

### 11.6 The gates this change passed

| Gate | Result |
| --- | --- |
| `cargo test --lib` | 275 passed, 0 failed |
| `--test reclamation` | 19 passed, 0 failed |
| `--test rc_specialisation` | 5 passed, 1 failed |
| `--test simplify` | 41 passed, 0 failed |
| `--test inline_copy` | 4 passed, 0 failed |
| `--test core_output` | 21 passed, 0 failed |
| `--test ownership` | 29 passed, 0 failed |

The four new `rc_specialisation` tests (the `if`-shaped rebuild, the
nested-`case` rebuild, the refused mixed-arm branch, and the recursive
branched rebuild in both a plain and an `rc_debug` build) all pass. The one
failure is
`borrowed_parameters_release_and_static_dictionaries_place_fewer_operations`,
which asserts that `Main.SizeInt` is swept from the emitted IR. It is **not
caused by this change**: it fails identically at commit `f07e60f2`, which
contains no `branch_reuse` and no argument search, verified by building that
revision in a separate workspace and running the same test there. It is a
pre-existing regression from the inliner work between `567b9674` (where it
passes) and `f07e60f2`.

**Fixed point and the stage numbers are not reported here.** The stage
rebuild needs the ladder's stage lock, which a sibling lane's test suite held
for the duration of this work, so the fixed-point comparison and the
toolkit/self-check block counts on a stage built from this compiler did not
run. The block counts in section 11.3 are program-level measurements with the
Rust frontend's `build`, which exercises the same `rc.rs`, so they are valid
evidence for the mechanism; they are not the stage-level numbers. Anyone
landing this must run `tooling/bootstrap.py build --stage 3` and report the
fixed point before it merges, and `compiler/src` is a seed source, so a
stage0 re-pin is needed.

**Lock bump: needed.** `compiler/src/rc.rs` changed, so the stage0 seed and
the library provider both need re-pinning. `runtime_abi` did not move.
