# Rewrites, laws, specialization, and optimizer reports

Optimization runs on Core IR after type, effect, and ownership checking and before LLVM code generation. It combines two mechanisms:

1. **Declared rewrite rules** (`rewrite` and `target rewrite`) — pattern/replacement equations backed by definitional equality or a declared law.
2. **Automatic driving, fusion, and Core simplification** — call unfolding, constructor and constant folding, type-class dictionary specialization, `Data.Producer` stream fusion, and post-specialization Core simplification.

No optimization can change a program's types, effect rows, or exported interface contract. Every residual unit is rechecked before installation; any candidate that fails rechecking is discarded in favor of the generic definition.

The driver's `fuel` and `measure` bounds come from the selected target profile (`--target`; see [Chapter 01](01-using-the-compiler.md#targets-and-cross-compilation)). You steer optimization through program structure, declared rewrites, `#[inline]` / `#[weight(n)]` hints, and the Core simplifier CLI flags (`--max-simplifier-iterations`, `--inline-model`, `--no-inline`, and related toggles in [Chapter 01](01-using-the-compiler.md#core-simplifier-and-lowering-controls)).

# Rewrite declarations

```text
rewrite_decl  = [ "target" ], "rewrite", var_id,
                [ "forall", binder, { binder }, "." ],
                [ using_context ], "where", rewrite_block ;
rewrite_block = "{", "match", term, ";", "replace", term, ";",
                { "require", term, ";" }, "justify", term, [ ";" ], "}" ;
```

Layout syntax may be used in place of braces and semicolons. Clauses may appear in any order, and each clause may appear at most once; repeating a clause is a parse error (`duplicate `match` clause in a rewrite declaration`).

This rule cites the standard `Semigroup.assoc` law to reassociate `<>`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Algebra.Semigroup

  rewrite reassociate forall (x :: U32) (y :: U32) (z :: U32). using (sg :: Semigroup U32) where
    match ((x <> y) <> z)
    replace (x <> (y <> z))
    justify Semigroup.assoc x y z

  main :: Int
  main = 0
```

The `using` clause states constraints available to the rule's body; it does not require an instance to exist in the current module. A rule is checked in its own context.

## Static checks

The checker enforces eight invariants on every rewrite declaration before the optimizer ever sees it:

| Rule | Diagnostic on failure |
| --- | --- |
| `match` is headed by a defined function, constructor, or primitive | ``the rule's `match` pattern must be headed by a defined function, constructor, or primitive: a bare variable pattern would change definitional equality`` |
| `replace` has the same type as `match` | ``the `replace` term must have the `match` term's type: type mismatch: expected ..., got ...`` |
| `replace` references only binders from `match` or `using` | ``the `replace` term mentions `m`, which the `match` pattern does not bind`` |
| Pattern type is runtime data, not erased evidence or a type | `a rewrite over erased evidence would change definitional equality (proof irrelevance) and resurrect erased components` |
| `match`, `replace`, and `justify` are pure | ``an effectful rewrite rule needs an observational-equivalence justification, which the proof kernel does not express in this slice: the `match` term performs effects, and pure `Eq` evidence does not authorize reordering them`` |
| Linear (`%1`) and affine (`%aff`) binders respect multiplicity in `replace` | Multiplicity error (a rule cannot duplicate or drop linear values) |
| Each `require` condition has a runtime type | ``a `require` condition is operational evidence, not a proof: it must have a runtime type`` |
| Both `match` and `justify` clauses are present | ``a rewrite declaration needs a `justify` clause`` |

A bare variable pattern is rejected because it would apply to every expression of its type:

<!-- cinch:reject contains="must be headed by a defined function" -->
```cinch
module Main where
  rewrite everything forall (x :: Nat) (y :: Nat). where
    match x
    replace y
    justify refl

  main :: Int
  main = 0
```

Rewriting erased proof evidence is rejected because erasure removes proof terms before runtime:

<!-- cinch:reject contains="a rewrite over erased evidence" -->
```cinch
module Main where
  rewrite overProof forall (x :: Nat) (h :: Eq x x). where
    match h
    replace h
    justify refl

  main :: Int
  main = 0
```

Effectful patterns are rejected because pure `Eq` evidence does not govern effect ordering:

<!-- cinch:reject contains="an effectful rewrite rule needs an observational-equivalence justification" -->
```cinch
module Main where
  shout :: forall r. using (io :: Io r) => Nat ->{IO r} Nat
  shout n = let _w = ioWrite (string-to-bytes (nat-to-string n)) in n

  rewrite shoutEcho forall (n :: Nat). where
    match shout n
    replace shout n
    justify refl

  main :: Int
  main = 0
```

Omitting `justify` is rejected during parsing:

<!-- cinch:reject contains="needs a `justify` clause" -->
```cinch
module Main where
  import Algebra.Semigroup

  rewrite assocRule forall (x :: U32) (y :: U32) (z :: U32). using (sg :: Semigroup U32) where
    match ((x <> y) <> z)
    replace (x <> (y <> z))

  main :: Int
  main = 0
```

# Justifications: definitional equality and laws

Every rewrite requires a `justify` term establishing `Eq match replace`. Two forms of evidence are accepted, and the build manifest distinguishes them:

- **Definitional equality (`justify refl`).** Succeeds when both sides normalize to the same index term by computation alone. No assumption is added to `assumptions.declared`.
- **Law citation (`justify Law.name args...`).** Cites a signature or class law whose proposition unifies with `Eq match replace` when instantiated at the rule's binders. Unless an instance discharges the law, its identity is recorded in `assumptions.declared`.

This rule holds by definitional equality and leaves `assumptions.declared` empty:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  ident :: Nat -> Nat
  ident n = n

  rewrite identDrop forall (n :: Nat). where
    match ident n
    replace n
    justify refl

  main :: Int
  main = if nat-to-int (ident 41) == 41 then 0 else 1
```

Rewrite equations never extend the kernel's definitional equality. Even with a `rewrite` rule in scope, proving the same equation with `refl` fails:

<!-- cinch:reject contains="`refl` is evidence of" -->
```cinch
module Main where
  mapL :: forall a b. (a -> b) -> List a -> List b
  mapL f xs = case xs of
    Nil -> Nil
    Cons x rest -> Cons (f x) (mapL f rest)

  mapLC :: forall a b c. (b -> c) -> (a -> b) -> List a -> List c
  mapLC f g xs = mapL (\x -> f (g x)) xs

  signature OptLaws where
    mapPair :: forall a b c. (f :: b -> c) -> (g :: a -> b) -> (xs :: List a) -> Eq (mapL f (mapL g xs)) (mapLC f g xs)

  rewrite mapMap forall (f :: U32 -> U32) (g :: U32 -> U32) (xs :: List U32). where
    match mapL f (mapL g xs)
    replace mapLC f g xs
    justify OptLaws.mapPair f g xs

  bogus :: (f :: U32 -> U32) -> (g :: U32 -> U32) -> (xs :: List U32) -> Eq (mapL f (mapL g xs)) (mapLC f g xs)
  bogus f g xs = refl

  main :: Int
  main = 0
```

A law whose proposition does not unify with `Eq match replace` is rejected:

<!-- cinch:reject contains="justification does not prove" -->
```cinch
module Main where
  import Algebra.Semigroup

  rewrite wrongRule forall (x :: U32) (y :: U32). using (sg :: Semigroup U32) where
    match ((x <> y) <> y)
    replace (x <> y)
    justify Semigroup.assoc x y y

  main :: Int
  main = 0
```

A `require` clause attaches an operational precondition to the rule. Its operative form is `Available cap` for a capability binder, which holds where `cap` is in the firing unit's validity scope; a plain `Bool` expression is recorded as an undischarged condition in the manifest.

# Observing a rewrite

The rule below equates `countDown` with `tick` via `Loop.same`. The optimizer rewrites calls to `countDown` inside both `countDown` and `main`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  countDown :: Nat -> Nat
  countDown n = case n of
    0 -> 0
    _ -> countDown (nat-sub n 1)

  tick :: Nat -> Nat
  tick n = case n of
    0 -> 0
    _ -> tick (nat-sub n 1)

  signature Loop where
    same :: (n :: Nat) -> Eq (countDown n) (tick n)

  rewrite countToTick forall (n :: Nat). where
    match countDown n
    replace tick n
    justify Loop.same n

  main :: Int
  main = if nat-to-int (countDown 5) == 0 then 0 else 1
```

Inspecting the emitted LLVM IR confirms that `Main.main` calls `Main.tick` directly instead of `Main.countDown`:

```sh
cinch emit-ir Main.cinch | sed -n '/define i64 @"Main.main"()/,/^}/p' | grep 'call i64 @"Main'
#   %t0 = call i64 @"Main.tick"(i64 11)
```

In `build/Main.manifest.json`, `specialization.sites` records two completed firings of `rewrite:countToTick` (in `Main.countDown` and `Main.main`, both with `stop: null`), and `assumptions.declared` lists `Main.same`.

# Reading the optimizer report

Every `check`, `emit-ir`, `emit-object`, `build`, and `run` writes `build/<entry-stem>.manifest.json`. Optimization and verification telemetry is recorded across five keys:

| Key | Contents |
| --- | --- |
| `specialization.ran` | Whether the optimization driver ran for this artifact. |
| `specialization.fuel`, `measure` | Active unfolding and size bounds from the target profile (`8` and `8` on the built-in native and `wasm32` profiles). |
| `specialization.whistle` | Termination check in use (`repetition-or-measure`). |
| `specialization.sites[]` | Per-site trace records: `{ unit, kind, stop }`. |
| `specialization.fused[]` | Human-readable descriptions of `Data.Producer` fusion steps applied. |
| `specialization.discarded[]` | Residual candidates rejected by the post-optimization recheck, with reasons. |
| `specialization.representation_versions[]` | Representation-specialized clones emitted (`unit@classes`), alongside `representation_unrecorded`. |
| `assumptions.declared` | Law identities cited by active rewrites and declared interface assumptions. |
| `target.conditions[]` | Each `target rewrite` precondition, its rule name, and whether firing discharged it. |
| `trust` | Trust level (`2`, `full-recheck`) and the list of rechecked units. |
| `lowering` | Boundary split between `checked` invariants (Core scoping, `%1`/`%aff` linearity, memo/demand sharing, branch weights, contract identity) and `trusted` layers (LLVM codegen and the runtime). |

Each entry in `specialization.sites[]` has a `kind` from:
`unfold`, `case-fold`, `const-fold`, `dictionary-specialize`, `effect-static`, `rewrite:<name>`, `fusion`, `case-merge`, `dead-alternative`, `case-push`.

A `stop` value of `null` means the transformation completed. Otherwise `stop` names the barrier that prevented it: `fuel-exhausted`, `whistle: ...`, `effect barrier: ...`, `allocation barrier: ...`, `linear value: ...`, `opaque boundary: ...`, `capability scope: ...`, or `inline(never): ...`.

Because every unfolded call produces a site entry, query the manifest rather than reading it raw:

```sh
jq '{
  rewrites: [.specialization.sites[] | select(.kind | startswith("rewrite"))],
  fused: .specialization.fused,
  assumptions: .assumptions.declared,
  target_conditions: .target.conditions
}' build/Main.manifest.json
```

To inspect the post-optimization IR directly without parsing LLVM, dump Core with `cinch core emit Main.cinch` or `--dump-core=lowered` (see [Chapter 01](01-using-the-compiler.md#core-output)).

# Barriers and suppressed optimizations

When a checked rewrite or fusion candidate does not fire, the manifest records a site with a non-null `stop` explaining why.

**Effect barrier.** Pure `Eq` laws do not permit reordering callbacks whose purity is unknown. In `traceRun` below, `f` and `g` mutate a byte buffer; fusing `mapL f (mapL g xs)` would interleave their writes differently, so `rewrite:mapMap` stops with an effect barrier (`effect barrier: the callback's purity is unknown`) and the program preserves the unfused order:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  mapL :: forall a b. (a -> b) -> List a -> List b
  mapL f xs = case xs of
    Nil -> Nil
    Cons x rest -> Cons (f x) (mapL f rest)

  mapLC :: forall a b c. (b -> c) -> (a -> b) -> List a -> List c
  mapLC f g xs = mapL (\x -> f (g x)) xs

  signature OptLaws where
    mapPair :: forall a b c. (f :: b -> c) -> (g :: a -> b) -> (xs :: List a) -> Eq (mapL f (mapL g xs)) (mapLC f g xs)

  rewrite mapMap forall (f :: U32 -> U32) (g :: U32 -> U32) (xs :: List U32). where
    match mapL f (mapL g xs)
    replace mapLC f g xs
    justify OptLaws.mapPair f g xs

  traceRun :: Nat
  traceRun =
    let log = bytes-new 1 in
    let bump m = bytes-set log 0 (nat-rem (nat-add (nat-mul (bytes-get log 0) 4) m) 256) in
    let g x = let _q = bump 1 in wrapping-add-u32 x 2 in
    let f x = let _p = bump 2 in wrapping-add-u32 x 1 in
    let _v = mapL f (mapL g (Cons 1 (Cons 2 Nil))) in
    bytes-get log 0

  main :: Int
  main = if traceRun == 90 then 0 else 1
```

**Allocation barrier.** Stream fusion stops if a callback allocates a memo cell whose creation would be reordered relative to its demand. The `fusion` site in `Main.main` records `allocation barrier: the callback allocates a memo cell, which fusion would reorder relative to its demand`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  step3 :: Nat -> Step Nat Nat
  step3 k = case k of
    0 -> Yield 3 1
    1 -> Yield 5 2
    _ -> Done

  mem :: Nat -> Nat
  mem x = let ~y = nat-add x x in copyOut (demand y)

  main :: Int
  main =
    let s = Producer step3 0 in
    if prod-fold 0 nat-add (prod-map mem (prod-map mem s)) == 32 then 0 else 1
```

# Producer fusion and inlining hints

Pipelines built over `Data.Producer` (`prod-map`, `prod-fold`, `prod-take`, and friends) fuse into a single `Done`/`Skip`/`Yield` state machine, eliminating intermediate `Step` allocations:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  step3 :: Nat -> Step Nat Nat
  step3 k = case k of
    0 -> Yield 3 1
    1 -> Yield 5 2
    _ -> Done

  srcP :: Producer Nat Nat
  srcP = Producer step3 0

  doubleN :: Nat -> Nat
  doubleN x = nat-add x x

  tripleN :: Nat -> Nat
  tripleN x = nat-add x (nat-add x x)

  main :: Int
  main =
    let total = prod-fold 0 nat-add (prod-map doubleN (prod-map tripleN srcP)) in
    if total == 48 then 0 else 1
```

In the manifest, `specialization.fused` reports the fused stages (`fused 2 map stage(s) into the fold loop state machine`). Fusion strictly preserves pull boundaries (`prod-take 2` pulls at most two elements) and per-element callback order.

Inlining hints (`#[inline]`, `#[inline(always)]`, `#[inline(never)]` / `#[noinline]`) emit the corresponding LLVM attributes (`inlinehint`, `alwaysinline`, `noinline`) and never alter program semantics. `#[inline(never)]` and `#[noinline]` also prevent both the specialization driver from unfolding the callee and the Core simplifier from inlining it.

## Target-gated rewrites

A `target rewrite` guards firing with a `require Available <cap>` clause so that the transformation applies only in functions whose validity scope carries that capability. Below, `sumWrap32Vector` rewrites `scalar xs` inside `in-scope` (`stop: null`) and stops in `no-scope` with a `capability scope` barrier:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  scalar :: forall n region. Ref region (Vec n U32) -> U32
  scalar xs = Stream.fold U32.zero U32.wrapping-add (Vec.stream xs)

  signature SumProofs where
    scalar-to-simd :: forall t w n region. (vectors :: VecU32Target t w) -> (xs :: Ref region (Vec n U32)) -> Eq (scalar xs) (Simd.sum-wrap32 vectors xs)

  target rewrite sumWrap32Vector
    forall t w n region (xs :: Ref region (Vec n U32)).
    using (vectors :: VecU32Target t w) where
    match scalar xs
    replace Simd.sum-wrap32 vectors xs
    require Available vectors
    justify SumProofs.scalar-to-simd vectors xs

  in-scope :: forall t w n region. using (vectors :: VecU32Target t w) => Ref region (Vec n U32) -> U32
  in-scope xs = scalar xs

  no-scope :: forall n region. Ref region (Vec n U32) -> U32
  no-scope xs = scalar xs

  main :: Int
  main = 0
```

When the rule fires, `target.conditions` records the capability condition as `discharged: true` and `assumptions.declared` records `Main.scalar-to-simd`.

# Guarantees, bounds, and current limits

| Transformation | Bound / safety invariant |
| --- | --- |
| Call unfolding | Bounded by profile `fuel` and `measure` plus the whistle; stops cleanly without altering semantics. |
| Known-constructor `case` folding | Dead alternatives are dropped; surviving branch weights are not rescaled, and duplicated decisions preserve their weights. |
| Dictionary specialization | Static dictionaries lower to direct method calls while preserving capability and effect discipline. |
| Pure primitive folding | Constant-folded faults remain faults; nothing is dropped silently. |
| `Data.Producer` fusion | Preserves effect ordering and pull boundaries. |
| Declared `rewrite` / `target rewrite` | Respects all driving barriers and passes post-optimization residual rechecking. |

Driving stops immediately at any of seven barriers:

- a non-empty effect row whose result the driver needs,
- an opaque boundary such as a `foreign` call,
- a `%1` or `%aff` value that would be duplicated or dropped,
- a memo cell or thunk whose allocation would reorder relative to its demand,
- a `corec` value whose `delay` guard would be stripped,
- erased evidence (`%0`), or
- an index equation requiring an unproved refinement.

Every optimized residual is rechecked (`trust.level = 2`, `full-recheck`). A candidate that fails rechecking is logged in `specialization.discarded` and never installed. Specialization is purely representational: residual units preserve the identity and arity of the generic definition, leaving the compiled interface byte-identical so dependent modules never become stale.

The `fuel` and `measure` limits come from the active target profile (`8` and `8` across all five CLI `--target` triples: `aarch64-apple-darwin`, `x86_64-apple-darwin`, `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, and `wasm32-unknown-unknown`). Exhausting fuel can suppress an optimization but never changes program meaning.

<span class="support-tag support-tag--planned">Coming soon</span> **Handler-vtable specialization and step certificates.** Statically known `handle` clauses are not yet specialized to direct calls across `do` and task boundaries. Independently checkable per-step optimization certificates (above `trust.level = 2`) are also planned.

See [supercompilation and JIT specification](../specifications/supercompilation-and-jit-specification.md), [modules and compilation specification](../specifications/modules-and-compilation-specification.md) §5, and the [rewrites and performance tutorial](../learn/19-rewrites-and-performance.md).
