# 19. Laws, rewrites, and performance

An optimization is a claim about meaning: the new program must compute what the old one computed, in the same order, with the same effects and the same use of resources. Cinch makes that claim a declaration. A law states an equation, a `rewrite` names the equation it replaces and the evidence that entitles it, and the optimizer records, per site, whether the rule fired or what stopped it. Nothing about a rule is invisible: you read the emitted IR and the artifact manifest, not the optimizer's good intentions.

Prerequisites: [signatures and instances](09-signatures-and-instances.md) for the law vocabulary, [recursion and collections](05-recursion-and-collections.md) for `List`, and [modules](08-modules.md) for imports. Run every complete example with `cinch run Main.cinch` or `cinch build Main.cinch -o example`.

# A law is an equation you can cite

A `signature` may declare laws. Each law is a method with a proposition as its result, so it is an obligation an instance discharges and a fact a rewrite may cite. These declarations are a fragment, meant to sit inside a module:

```text
  mapL :: forall a b. (a -> b) -> List a -> List b
  mapL f xs = case xs of
    Nil -> Nil
    Cons x rest -> Cons (f x) (mapL f rest)

  mapLC :: forall a b c. (b -> c) -> (a -> b) -> List a -> List c
  mapLC f g xs = mapL (\x -> f (g x)) xs

  signature MapLaws where
    mapPair :: forall a b c. (f :: b -> c) -> (g :: a -> b) -> (xs :: List a) ->
      Eq (mapL f (mapL g xs)) (mapLC f g xs)
```

`mapLC` is the single-traversal form of two chained maps: it composes the functions first. The law says the two terms are equal for every `f`, `g`, and `xs`. Library laws are checked against instances on finite samples, and the manifest reports the status rather than claiming a theorem. A real row from the artifact manifest of the program in this chapter, for the prelude's `Ordering` instance:

```text
{"class": "Semigroup", "instance": "Data.Ordering.Semigroup_Ordering", "law": "assoc", "status": "checked (27 samples)"}
```

The status distinguishes a sample check from an assumption and from a check that could not run. `checked (27 samples)` is a finite sample result, not a universal proof, and the manifest keeps that distinction.

# Write the rule

A rewrite binds its pattern variables, states the term to find and the term to produce, and supplies evidence:

```text
  rewrite mapMap forall (f :: U32 -> U32) (g :: U32 -> U32) (xs :: List U32). where
    match mapL f (mapL g xs)
    replace mapLC f g xs
    justify MapLaws.mapPair f g xs
```

| Clause | Requirement |
| --- | --- |
| `forall (...)` | At least one binder; the rule's type variables and value variables. |
| `using (sg :: Class a)` | Optional; a capability the rule needs, valid where it fires. |
| `match` | Required. The pattern. It must be headed by a defined function, a constructor, or a primitive. |
| `replace` | Required. Terms may only mention binders the pattern binds or a `using` capability. |
| `justify` | Required. Evidence under the equation the rule states. |
| `require` | Optional. An applicability condition checked at each candidate site. |

The `justify` term is checked against the rule's equation, and a law name is not permission for an unrelated simplification: the rule `match mapL f (mapL g xs) / replace mapL f xs` with the same `justify MapLaws.mapPair f g xs` is refused, because the law proves a different equation.

# A program with the rule in scope

The rule and both definitions, plus a comparison that proves the answer:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  mapL :: forall a b. (a -> b) -> List a -> List b
  mapL f xs = case xs of
    Nil -> Nil
    Cons x rest -> Cons (f x) (mapL f rest)

  mapLC :: forall a b c. (b -> c) -> (a -> b) -> List a -> List c
  mapLC f g xs = mapL (\x -> f (g x)) xs

  signature MapLaws where
    mapPair :: forall a b c. (f :: b -> c) -> (g :: a -> b) -> (xs :: List a) ->
      Eq (mapL f (mapL g xs)) (mapLC f g xs)

  rewrite mapMap forall (f :: U32 -> U32) (g :: U32 -> U32) (xs :: List U32). where
    match mapL f (mapL g xs)
    replace mapLC f g xs
    justify MapLaws.mapPair f g xs

  double :: U32 -> U32
  double x = wrapping-add-u32 x x

  incr :: U32 -> U32
  incr x = wrapping-add-u32 x 1

  sumL :: List U32 -> U32
  sumL xs = case xs of
    Nil -> 0
    Cons x rest -> wrapping-add-u32 x (sumL rest)

  main :: Int
  main =
    let xs = Cons 1 (Cons 2 Nil)
    in if sumL (mapL double (mapL incr xs)) == 10 then 0 else 1
```

`(1 + 1) * 2 + (2 + 1) * 2` is `10`, so the program exits `0` whether the rule fires or not. That is the point: a rewrite must not change what you observe, so the check you can run is the program's own answer, and the evidence that the rule fired comes from the artifact.

# Read the difference in the emitted IR

Emit the IR, then look at the entry function:

```text
$ cinch emit-ir Main.cinch -o Main.ll
$ grep 'call i64 @"Main\.mapL@RR"' Main.ll
323827:  %t31 = call i64 @"Main.mapL@RR"(i64 %t29, i64 %t30)
323828:  %t32 = call i64 @"Main.mapL@RR"(i64 %t28, i64 %t31)
```

Delete the `rewrite` declaration and repeat. The same two lines are absent and the fused call appears instead:

```text
324287:  %t7 = call i64 @"Main.mapLC@RRR"(i64 %t4, i64 %t5, i64 %t6)
```

The two chained `mapL@RR` calls became one `mapLC@RRR` call, so the list is walked once instead of twice. `Main.mapL@RR` and `Main.mapLC@RRR` are the specialized representation variants the optimizer emitted for those units: `Main.mapLC@RRR` is defined in the rewritten program's IR and absent from the other. The line numbers differ between the two files because the specializer also unfolds `sumL` in the rewritten program; compare call sites, not file sizes.

# Confirm it in the manifest

Every build writes `build/Main.manifest.json` beside the source. Its `specialization.sites` array names each transformation site and the blocker, if any. With the rule in scope:

```text
$ python3 -c 'import json; d=json.load(open("build/Main.manifest.json"));
print([s for s in d["specialization"]["sites"] if s["kind"].startswith("rewrite")])'
[{'kind': 'rewrite:mapMap', 'stop': None, 'unit': 'Main.main'}]
```

`stop: None` means the rule applied. The same artifact records the justification identity as a declared assumption, because no instance discharges `MapLaws.mapPair`:

```text
"declared": ["Main.mapPair"]
```

Without the `rewrite` declaration the array is empty and `declared` holds no such name. Those two facts, the site row and the assumption row, are what "the optimizer did something" means in this compiler.

# How to tell the optimizer did not fire

Rules have barriers. A rule justified by a pure equation cannot be applied under a callback whose purity is unknown, because changing traversal order would change the order of effects. Here the rule is in scope and the pattern matches, and the optimizer refuses it:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  mapL :: forall a b. (a -> b) -> List a -> List b
  mapL f xs = case xs of
    Nil -> Nil
    Cons x rest -> Cons (f x) (mapL f rest)

  mapLC :: forall a b c. (b -> c) -> (a -> b) -> List a -> List c
  mapLC f g xs = mapL (\x -> f (g x)) xs

  signature MapLaws where
    mapPair :: forall a b c. (f :: b -> c) -> (g :: a -> b) -> (xs :: List a) ->
      Eq (mapL f (mapL g xs)) (mapLC f g xs)

  rewrite mapMap forall (f :: U32 -> U32) (g :: U32 -> U32) (xs :: List U32). where
    match mapL f (mapL g xs)
    replace mapLC f g xs
    justify MapLaws.mapPair f g xs

  traceRun :: Nat
  traceRun =
    let log = bytes-new 1
    in let bump m = bytes-set log 0 (nat-rem (nat-add (nat-mul (bytes-get log 0) 4) m) 256)
    in let g x = let _q = bump 1 in wrapping-add-u32 x 2
    in let f x = let _p = bump 2 in wrapping-add-u32 x 1
    in let _v = mapL f (mapL g (Cons 1 (Cons 2 Nil)))
    in bytes-get log 0

  main :: Int
  main = if traceRun == 90 then 0 else 1
```

The accumulator `*4 + m` makes the order visible. The unfused order runs the inner map to completion and then the outer map, which is `90`; the fused order interleaves and produces a different number. The program exits `0`, and the manifest says why the rule stayed out:

```text
{"kind": "rewrite:mapMap", "stop": "effect barrier: the callback's purity is unknown (an effectful argument would change effect order)", "unit": "Main.traceRun"}
```

Three ways to tell that a rule did not fire, in increasing order of specificity: the pattern is not in the IR, the `stop` field is not `None`, or the `rewrite:<name>` row is absent entirely (the pattern never matched at that site). Never conclude from a faster build or a smaller binary that the rule fired; read the row.

# Hints do not replace the contract

`#[inline(always)]`, `#[inline(never)]`, `#[cold]`, `#[weight(n)]`, `#[likely]` and `#[unlikely]` are representation hints with defined placements and validation rules. They change code generation, and they do not change what the checker requires: an `#[unlikely]` branch is still checked for types, effects, ownership, and coverage, and an unlikely branch is not dead code. The complete list is in [attributes and pragmas](../reference/23-attributes-and-pragmas.md).

# Measure, do not assume

Check the output and the exit status before the clock. Use dynamic inputs so constant folding cannot replace the computation you meant to measure, and separate compile time from run time. Two switches report what the program actually did. `CINCH_ALLOC_STATS=1` prints one line at exit through the runtime's `atexit` handler:

```text
$ cinch build Main.cinch -o example && CINCH_ALLOC_STATS=1 ./example
cinch-alloc: blocks=13 bytes=394 peak=210 header-blocks=7 header-bytes=176 immortal=0 freed=7 live=0
```

`CINCH_RECLAIM=0` builds the leak-everything baseline the same code compares against, and the counters show the difference:

```text
$ CINCH_RECLAIM=0 cinch build Main.cinch -o leaky && CINCH_ALLOC_STATS=1 ./leaky
cinch-alloc: blocks=13 bytes=394 peak=322 header-blocks=7 header-bytes=176 immortal=0 freed=0 live=7
```

Both runs exit `0` and compute `10`; `freed=0 live=7` says the second build never returned its value blocks. `CINCH_HEAP_LIMIT` and `CINCH_STACK_SIZE` set the runtime limits an allocation test runs under.

<span class="support-tag support-tag--planned">Coming soon</span> **stronger proof automation and automatic representation selection.** Richer law automation and specialization strategies are designed; today's rule application is the one described here, bounded by fuel and the measure whistle, with residual rechecking. A planned optimizer feature is not a performance guarantee for the compiler you have.

# What the compiler enforces

- A `rewrite` needs `match`, `replace`, and `justify`, each at most once, and the parser names the missing clause.
- A `match` pattern must be headed by a defined function, constructor, or primitive. A bare variable pattern would change definitional equality and is refused.
- Every binder the `replace` term mentions must be bound by the pattern or by a `using` capability.
- `justify` must prove the equation the rule states. Definitional equality, and therefore `refl`, is not extended by a rule.
- Effects and linear values are barriers. A rule cannot reorder visible effects, duplicate a `%1` value, or drop a required use.
- The optimizer records one site per attempt with its blocker, and rechecks the residual program.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| A `rewrite` with no `justify` clause | ``a rewrite declaration needs a `justify` clause`` |
| `justify MapLaws.mapPair f g xs` for a different `replace` | ``the justification does not prove the rule's equation: it proves `p`, and the rule states `Eq` over its `match` and `replace` terms`` |
| `match x` on a bare binder | ``a bare variable pattern would change definitional equality`` |

The second message continues with the equation it compared, for example `(Main.mapLC @(U32) @(U32) @(U32) f g xs) vs (Main.mapL @(U32) @(U32) f xs)`, which tells you which side of the rule to change.

# Exercises

**Exercise 1.** Add a rule that turns `mapL identity xs` into `xs`, justified by a law you declare. What must the law's type be, and what does the manifest show after the rule fires?

<details><summary>Answer</summary>

Import `Function.Combinators` for `identity`, declare the law as `signature IdentityLaws where mapIdentity :: forall a. (xs :: List a) -> Eq (mapL identity xs) xs` (the `.` after the binders is required), then write:

```text
  rewrite mapId forall (xs :: List U32). where
    match mapL identity xs
    replace xs
    justify IdentityLaws.mapIdentity xs
```

`identity` is a defined function, so the pattern is headed by a defined function and is accepted. With a reachable call such as `sumL (mapL identity (Cons 1 (Cons 2 Nil)))`, the manifest gains `{"kind": "rewrite:mapId", "stop": null, "unit": "Main.main"}` and `assumptions.declared` holds `Main.mapIdentity`. A call whose list argument is produced by an effectful computation gets the `effect barrier` blocker instead, because evaluating `xs` unchanged and evaluating `mapL identity xs` are not ordered the same way.

</details>

**Exercise 2.** A rule is in scope, its pattern appears in the source, and `specialization.sites` has no `rewrite:<name>` row. What are the possible explanations, and which one can you rule out by looking at the IR?

<details><summary>Answer</summary>

The pattern must occur in optimized code for a site to be attempted: the term may have been inlined, folded, or dead-code eliminated before the driver reached it, which leaves no row. Alternatively the declaration was rejected in a way you ignored, or the rule was not in the module the code came from. Looking at the IR rules out "the rule fired silently": a firing replaces the matched structure, so the fused call or the substituted term would be visible at the site. Absence of both the row and the replacement means the site was never reached, not that the rule was applied and hidden.

</details>

Next: [foreign code, layouts, and target profiles](20-foreign-code-and-targets.md).
