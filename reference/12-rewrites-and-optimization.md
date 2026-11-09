# Rewrites, laws, specialization, and optimizer reports

A rewrite is a declared transformation: a pattern, a replacement with the same type, optional applicability conditions, and evidence under a relation the kernel can check. Rules run in the optimizer, after checking and before code generation, so a rule can change how a program runs. It cannot change the program's types, its effect rows, or its exported contract, and every residual unit is rechecked before it is installed.

Automatic specialization is separate from rules. The driver unfolds calls, folds known constructors and constants, specializes dictionaries, and fuses `Data.Producer` pipelines. You direct it with the program's shape, with `#[inline]` and `#[weight(n)]` hints, and with rewrites. There is no flag that names a function to drive, and no flag that changes fuel or the whistle.

# The declaration form

```text
rewrite_decl = [ "target" ], "rewrite", var_id,
               [ "forall", binder, { binder }, "." ],
               [ using_context ], "where", rewrite_block ;
rewrite_block = "{", "match", term, ";", "replace", term, ";",
                { "require", term, ";" }, "justify", term, [ ";" ], "}" ;
```

That production is [the surface grammar](../specifications/surface-syntax-and-grammar.md). The bootstrap requires the `forall` binders and also accepts the block written with the module's layout instead of braces and semicolons, which is the spelling used below. The clauses may appear in any order and each appears at most once; a repeated clause is refused with `duplicate `match` clause in a rewrite declaration`.

This rule cites a shipped class law, `Semigroup.assoc`, and states exactly the equation the law proves:

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

`using (sg :: Semigroup U32)` states the class constraint the rule's terms may use; it does not assert that an instance exists. A rule is checked in its own context, so the citation works whether or not the program ever instantiates `Semigroup U32`.

## What the checker enforces

| Condition | Refusal from `cinch check` |
| --- | --- |
| `match` is headed by a defined function, constructor, or primitive | ``the rule's `match` pattern must be headed by a defined function, constructor, or primitive: a bare variable pattern would change definitional equality`` |
| `replace` has the type of `match` | ``the `replace` term must have the `match` term's type: type mismatch: expected Nat, got Bool`` |
| `replace` mentions only names the pattern binds, or `using` dictionaries | ``the `replace` term mentions `m`, which the `match` pattern does not bind`` |
| the pattern's type is runtime data, not evidence or a type | `a rewrite over erased evidence would change definitional equality (proof irrelevance) and resurrect erased components` |
| all three terms are pure | ``an effectful rewrite rule needs an observational-equivalence justification, which the proof kernel does not express in this slice: the `match` term performs effects, and pure `Eq` evidence does not authorize reordering them`` |
| a linear binder is used exactly once in `replace` | the rule must not duplicate or drop a `%1` or `%aff` value; the checker counts the binder's occurrences |
| a `require` condition has a runtime type | ``a `require` condition is operational evidence, not a proof: it must have a runtime type`` |
| both `match` and `justify` exist | ``a rewrite declaration needs a `justify` clause`` |

A bare variable pattern is refused because it would rewrite every term of its type:

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

Erased evidence is refused because rewriting it would resurrect data that erasure removes:

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

An effectful rule is refused, because pure `Eq` evidence says nothing about observation order:

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

A missing clause is a parse error, reported before anything is checked:

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

`cinch check` reports the position first for parse errors, then the message: ``cinch: Main.cinch: 4:3: a rewrite declaration needs a `justify` clause``. Checker diagnostics print the position first and the file name last, as in the rows above.

# Justification: definitional equality and laws

There are two kinds of justification, and the difference is visible in the artifact manifest.

**Definitional equality.** `justify refl` works when the two sides reify to the same index term, which means the equation holds by computation. Nothing is assumed, so nothing is recorded.

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

The manifest records the firing and an empty assumption list:

```text
$ python3 -c "import json;d=json.load(open('build/Main.manifest.json'));print([s for s in d['specialization']['sites'] if s['kind'].startswith('rewrite')]);print(d['assumptions']['declared'])"
[{'kind': 'rewrite:identDrop', 'stop': None, 'unit': 'Main.main'}]
[]
```

**A law.** A signature law, or a shipped class law, has a proposition type. The checker instantiates its binders with the rule's terms and unifies the resulting proposition with the rule's equation. When no instance discharges the law, its identity becomes a declared assumption in the manifest.

`refl` does not prove a law. The kernel's conversion is fixed and rule equations do not extend it, so a theorem stated as an equation must be cited as evidence:

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

`cinch check` reports ``cinch: 19:18: index mismatch: (Main.mapL @(U32) @(U32) f (Main.mapL @(U32) @(U32) g xs)) vs (Main.mapLC @(U32) @(U32) @(U32) f g xs): `refl` is evidence of `Eq ?i547 ?i547`, not of `Eq (Main.mapL @(U32) @(U32) f (Main.mapL @(U32) @(U32) g xs)) (Main.mapLC @(U32) @(U32) @(U32) f g xs)) (in Main.cinch)``. The number after `?i` identifies a generated metavariable and differs between compilations.

A law that does not prove the rule's equation is refused the same way a bad proof is:

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

`cinch check` reports ``cinch: 5:3: the justification does not prove the rule's equation: it proves `p`, and the rule states `Eq` over its `match` and `replace` terms (index mismatch: (Algebra.Semigroup.<> @(U32) @{@dictionary[Local(4058)]} y y) vs y)``. The dictionary identity number inside `Local(...)` differs between compilations.

A `require` clause adds a runtime condition. Its operative form is `Available v` over a capability binder, which holds exactly where that capability is in the firing unit's validity scope; a plain `Bool` term becomes an unknown condition, which the manifest records without claiming it.

# Watching a rule fire

The rule below states that two counting functions agree, and the driver applies it inside `countDown` itself and inside `main`.

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

Emit the IR and read the body of `Main.main`. The call is to the replacement's unit:

```text
$ cinch emit-ir Main.cinch | sed -n '/define i64 @"Main.main"()/,/^}/p' | grep 'call i64 @"Main'
  %t0 = call i64 @"Main.tick"(i64 11)
```

Save the same program as `Plain.cinch` with the `signature` and `rewrite` declarations deleted, emit that, and the body calls the original unit:

```text
$ cinch emit-ir Plain.cinch | sed -n '/define i64 @"Main.main"()/,/^}/p' | grep 'call i64 @"Main'
  %t0 = call i64 @"Main.countDown"(i64 11)
```

The same run writes the report. Both firings are recorded, and the law is a declared assumption:

```text
$ python3 -c "import json;d=json.load(open('build/Main.manifest.json'));print([s for s in d['specialization']['sites'] if s['kind'].startswith('rewrite')]);print(d['assumptions']['declared'])"
[{'kind': 'rewrite:countToTick', 'stop': None, 'unit': 'Main.countDown'}, {'kind': 'rewrite:countToTick', 'stop': None, 'unit': 'Main.main'}]
['Main.same']
```

# Reading the optimizer report

Every `check`, `emit-ir`, `build` and `run` writes `build/<entry-file-stem>.manifest.json`. The optimizer's part of it is `specialization`:

| Key | Meaning |
| --- | --- |
| `ran` | the driver ran for this artifact |
| `fuel`, `measure` | the bounds of this build: 8 and 8 for the native profile |
| `whistle` | the repetition measure in use: `repetition-or-measure` |
| `sites[]` | every transformation site, with `unit`, `kind` and `stop` |
| `fused[]` | the fusion steps that ran, in words |
| `discarded[]` | residual candidates the recheck refused, with the reason |
| `representation_versions[]` | the class-vector copies the collector emitted, as `unit@letters` |
| `representation_unrecorded` | instantiation sites with no concrete class vector |

`kind` takes one of `unfold`, `case-fold`, `const-fold`, `dictionary-specialize`, `effect-static`, `rewrite:<name>`, `fusion`, `case-merge`, `dead-alternative`, `case-push`. `stop` is `null` when the transformation completed, and otherwise names what stopped it: `fuel-exhausted`, `whistle: ...`, `effect barrier: ...`, `allocation barrier: ...`, `linear value: ...`, `opaque boundary: ...`, or `capability scope: ...`.

A small program commonly records thousands of sites, since every unfolded call is one. Query the report instead of reading it. For the program above, this counts all sites and the ones that stopped early:

```text
$ python3 -c "import json;d=json.load(open('build/Main.manifest.json'));s=d['specialization']['sites'];print(len(s), sum(1 for x in s if x['stop']))"
2174 472
```

Three further sections record what the build assumed:

| Section | Contents |
| --- | --- |
| `assumptions.declared` | law identities rules relied on, plus declared interface assumptions |
| `target.conditions[]` | each `target rewrite` condition, its rule, and whether a firing discharged it |
| `trust` | the trust level (`2`, `full-recheck`) and the units it rechecked |

`lowering` states the split between what the recheck covers and what the build trusts:

```text
$ python3 -c "import json;d=json.load(open('build/Main.manifest.json'));print(json.dumps(d['lowering']))"
{"checked": ["core structure and scoping", "usage of %1/%aff parameters", "memo and demand sharing", "decision-point weights", "contract identity"], "trusted": ["LLVM code generation", "the runtime and its instruction model"]}
```

# When the optimizer did not fire

A rule that is declared, checks, and still does nothing leaves a `stop` behind. `stop` on a `rewrite:<name>` site means the site was considered and refused, so a program that runs correctly yet shows no speedup has an answer in the manifest.

An effectful callback blocks the eager map rule, because the rule's pure justification does not authorize interleaving callbacks that perform effects:

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

The program still computes the unfused order, and the report says why:

```text
$ python3 -c "import json;d=json.load(open('build/Main.manifest.json'));print([s for s in d['specialization']['sites'] if s['kind'].startswith('rewrite') and s['stop']])"
[{'kind': 'rewrite:mapMap', 'stop': "effect barrier: the callback's purity is unknown (an effectful argument would change effect order)", 'unit': 'Main.traceRun'}]
```

The same shape of answer covers specialization. The program below still returns 32, and fusion is refused because the callback allocates a memo cell:

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

```text
$ python3 -c "import json;d=json.load(open('build/Main.manifest.json'));print([s for s in d['specialization']['sites'] if s['kind']=='fusion' and s['stop']])"
[{'kind': 'fusion', 'stop': 'allocation barrier: the callback allocates a memo cell, which fusion would reorder relative to its demand', 'unit': 'Main.main'}]
```

# Specialization and fusion you can trigger

Write the pipeline over the producer step form and the driver fuses it. `prod-map` stages compose into the fold loop, and the intermediate step allocation disappears:

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

The report's `fused` entry names what happened:

```text
$ python3 -c "import json;d=json.load(open('build/Main.manifest.json'));print(d['specialization']['fused'])"
['Main.main: fused 2 map stage(s) into the fold loop state machine (Done/Skip/Yield); eliminated the intermediate step allocation']
```

Fusion keeps the pull boundary. `prod-take 2` pulls exactly two steps, so a producer whose third step records that it ran leaves the log at 0. Fusion also keeps callback order per element, so an effectful callback observes the same sequence it would observe unfused.

`#[inline]`, `#[inline(always)]` and `#[inline(never)]` are hints to code generation (`inlinehint`, `alwaysinline`, `noinline` on the LLVM definition) and never change program meaning. They do not control the driver: a `#[inline(never)]` wrapper called once is still unfolded and folded into its caller in `emit-ir` output, with the hint left on the definition for the LLVM inliner.

## Target rules

A `target rewrite` adds a `require` clause and fires only where the firing unit carries the matching capability parameter. Both functions below call the same scalar sum; the rule replaces it only inside the one that takes the vector capability:

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

```text
$ python3 -c "import json;d=json.load(open('build/Main.manifest.json'));print([s for s in d['specialization']['sites'] if s['kind'].startswith('rewrite')])"
[{'kind': 'rewrite:sumWrap32Vector', 'stop': None, 'unit': 'Main.in-scope'}, {'kind': 'rewrite:sumWrap32Vector', 'stop': 'capability scope: the rule applies only where a `VecU32Target` capability is valid', 'unit': 'Main.no-scope'}]
```

and the manifest records the condition next to the rule:

```text
$ python3 -c "import json;d=json.load(open('build/Main.manifest.json'));print(json.dumps(d['target']['conditions']));print(d['assumptions']['declared'])"
[{"condition": "capability VecU32Target valid in the firing region's validity scope", "discharged": true, "rule": "sumWrap32Vector"}]
['Main.scalar-to-simd']
```

# What the optimizer may assume

| Allowed | Bounded by |
| --- | --- |
| unfold a call with a known definition and known arguments | fuel and the measure bound; the site records where driving stopped |
| fold a known constructor's `case`, drop the dead alternatives | weights of the surviving alternatives are not rescaled, duplicated decisions keep theirs |
| specialize a known dictionary to direct method calls | the capability and effect rules continue to hold of the residual |
| fold pure total primitives on known arguments | a folded fault stays a fault; nothing is dropped silently |
| fuse `Data.Producer` pipelines into loops | the selected observation model, including effect order and the pull boundary |
| apply a declared rewrite where its pattern matches | the barrier list below; the residual recheck |

Driving stops at these barriers, and the site names the one that stopped it: a non-empty effect row whose result the driver needs, a `foreign` call or other opaque boundary, a `%1` or `%aff` value that would be duplicated or dropped, a memo cell or thunk that would be reordered relative to its demand, a `corec` construction whose `delay` guard would be removed, erased evidence, and an index equation that would need an invented refinement.

The recheck is what makes the report trustworthy. `trust.level` is `2` with the name `full-recheck`, and a candidate residual that fails the recheck goes into `discarded` instead of being installed, so the generic body keeps running. Specialization is representation-only: every residual unit has the identity and arity of the generic form, and the compiled interface is byte-identical, so a specialized build never stales a dependent.

Two knobs are not in the driver's gift: `fuel` and `measure` come from the target profile, which the `cinch` CLI fixes at `native64` with 8 and 8. Running out of fuel can lose an optimization and cannot change what the program computes.

<span class="support-tag support-tag--planned">Coming soon</span> **Handler-vtable specialization and step certificates.** A `handle` with a statically known handler is not yet specialized to direct clause calls, because dispatch sites can live inside callees reached through `do` and task bodies. The higher trust level, where each driving step carries an independently checkable certificate, is planned; the current residual recheck is not that certificate format.

The normative text is [the supercompilation and JIT specification](../specifications/supercompilation-and-jit-specification.md), with the rewrite rules themselves in [the modules and compilation specification](../specifications/modules-and-compilation-specification.md) section 5. The [practice chapter](../learn/19-rewrites-and-performance.md) walks through tuning a program.

Implementation: `core.rs` (`optimize`, the driver, barriers, whistle, weight reconciliation), `specialize.rs` (representation copies), `check_decls.rs` (`check_rewrite`, `check_justify`), `codegen.rs`. Suites: `compiler/tests/optimizer.rs`, `compiler/tests/laws.rs`.
