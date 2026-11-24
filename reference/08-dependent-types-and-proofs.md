# Dependent types, proofs, and erasure

Cinch's type checker sits on a small, predicative dependent core: stratified universes, dependent function (Pi) and pair (`Sigma`) types, indexed algebraic families, an erased proposition universe `Prop` with `%0` multiplicity enforcement, and a term-level proof kernel (`refl`, `congr`, and structural induction). Definitional equality and proof checking are intentionally bounded and predictable—every unsupported obligation is rejected explicitly rather than guessed or silently assumed.

For predicate refinements over value carriers and flow-sensitive narrowing, see [refinements, narrowing, and proof obligations](06-refinements.md).

# Universes

Types are stratified into a predicative hierarchy `Type 0 : Type 1 : ...`, with `Prop : Type 0` sitting at the base for erased propositions. `Type` in source syntax denotes `Type 0`. Because `Type : Type` is disallowed, a parameter of kind `Type` cannot be instantiated with `Type` itself:

<!-- cinch:reject contains="the universe `Type 0` has kind `Type 1`: `Type : Type` is not admitted" -->
```cinch
module Main where
  data Box (a :: Type) where
    Box :: a -> Box a

  f :: Box Type -> Nat
  f b = 0

  main :: Int
  main = 0
```

`Nat` and `Int` inhabit `Type 0` with exact, unbounded integer semantics, so the checker reduces index arithmetic over them. Fixed-width machine integers (`I8`..`I64`, `U8`..`U64`) also inhabit `Type 0`, but arithmetic over them is opaque in type indices.

# Dependent arrows, `Sigma`, and conversion

A function parameter may be named before `->` so that the return type can refer to it. In `bump`, the input index `n` determines the result index `n + 1`, so `bump 2 (Tag 7)` has type `Tagged 3`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Tagged (n :: Nat) where
    Tag :: Nat -> Tagged n

  bump :: (n :: Nat) -> Tagged n -> Tagged (n + 1)
  bump n t = Tag 10

  readTag :: forall (n :: Nat). Tagged n -> Nat
  readTag t = case t of
    Tag k -> k

  main :: Int
  main = if readTag (bump 2 (Tag 7)) == 10 then 0 else 1
```

Implicit `forall` binders carry either a value type (`forall (n :: Nat).`) or a kind (`forall (a :: Type).`) and are solved from application context—`readTag (bump 2 (Tag 7))` infers `n = 3` from the argument's type.

`Sigma (x :: A). B` is a dependent pair pairing a runtime witness `x :: A` with a payload of type `B` that may depend on `x`. Pairs are built with `(x, y)` and eliminated with pair patterns; unlike family indices and `Prop` evidence, a `Sigma` witness is retained at runtime:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  pack :: forall (n :: Nat). Vec n Int -> Sigma (m :: Nat). Vec m Int
  pack xs = (n, xs)

  unpack :: (Sigma (m :: Nat). Vec m Int) -> Nat
  unpack p = case p of
    (m, v) -> m

  main :: Int
  main = if unpack (pack (Cons 7 (Cons 8 Nil))) == 2 then 0 else 1
```

## Definitional equality

Type conversion combines four rules:

1. **Beta reduction** at elaboration time.
2. **Numeral arithmetic reduction** (`iota`) on concrete `Nat`/`Int` index expressions.
3. **Alpha equivalence** for Pi and `Sigma` binders.
4. **Proof irrelevance** for terms inhabiting `Prop`.

Crucially, **user-defined functions do not unfold during conversion**. A named function applied inside an index stays opaque, so `Vec (plus-one 2) Int` does not convert to `Vec 3 Int`:

<!-- cinch:reject contains="(Main.plus-one 2)" -->
```cinch
module Main where
  plus-one :: Nat -> Nat
  plus-one n = n + 1

  three :: Vec (plus-one 2) Int
  three = Cons 1 (Cons 2 (Cons 3 Nil))

  main :: Int
  main = 0
```

Direct numeral arithmetic inside an index does reduce, so `Vec (2 + 1) Int` is definitionally equal to `Vec 3 Int`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  total :: Vec (2 + 1) Int -> Int
  total v = case v of
    Cons x rest -> x + sum2 rest

  sum2 :: Vec 2 Int -> Int
  sum2 v = case v of
    Cons x rest -> x + sum1 rest

  sum1 :: Vec 1 Int -> Int
  sum1 v = case v of
    Cons x rest -> x

  main :: Int
  main = if total (Cons 1 (Cons 2 (Cons 3 Nil))) == 6 then 0 else 1
```

# Indexed families

Three foundational indexed families are built in:

```text
data Vec (n :: Nat) (a :: Type) where
  Nil  :: Vec 0 a
  Cons :: forall k. a -> Vec k a -> Vec (k + 1) a

data Fin (n :: Nat) where
  Fin :: (i :: Nat) -> (bound :: i < n) -> Fin n

data Dec (p :: Prop) where
  Yes :: (h :: p) -> Dec p
  No  :: (h :: Not p) -> Dec p
```

User programs declare indexed families with the same GADT-style `data ... where` syntax—each constructor specifies the exact index it targets (as in `Tagged` above and `Signal` below).

Constructors carrying proposition fields cannot be fabricated without valid evidence. `Fin i bound` requires proof of `i < n`; supplying a false static bound fails immediately:

<!-- cinch:reject contains="obligation disproved: `7 < 3` is false" -->
```cinch
module Main where
  badIndex :: Fin 3
  badIndex = Fin 7 refl

  main :: Int
  main = 0
```

For runtime indices, obtain the proof witness through `decide`, which evaluates the proposition and returns `Yes bound` or `No _`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  safeAt :: (n :: Nat) -> Nat -> Option (Fin n)
  safeAt n i = case decide (i < n) of
    Yes bound -> Some (Fin i bound)
    No _ -> None

  finVal :: (n :: Nat) -> Fin n -> Nat
  finVal n f = case f of
    Fin i b -> i

  main :: Int
  main = case safeAt 3 1 of
    Some f -> if finVal 3 f == 1 then 0 else 1
    None -> 2
```

# Index-aware pattern coverage

Exhaustiveness checking solves constructor index equations against the scrutinee's type. For `Vec 0 Int`, `Cons` produces `Vec (k + 1) Int` and cannot unify with `0`, so matching `Nil` alone is exhaustive:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  onlyNil :: Vec 0 Int -> Int
  onlyNil v = case v of
    Nil -> 5

  main :: Int
  main = if onlyNil Nil == 5 then 0 else 1
```

Writing an arm for a constructor whose index contradicts the scrutinee is a compile-time error rather than dead code:

<!-- cinch:reject contains="impossible arm: constructor Cons cannot build Vec 0 Int" -->
```cinch
module Main where
  bad :: Vec 0 Int -> Int
  bad v = case v of
    Nil -> 5
    Cons x xs -> x

  main :: Int
  main = 0
```

User-defined indexed families follow the same rule at concrete indices—`Signal 1` is covered completely by `Mono`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Signal (channels :: Nat) where
    Mono :: Int -> Signal 1
    Stereo :: Int -> Int -> Signal 2

  observe :: Signal 1 -> Int
  observe signal = case signal of
    Mono x -> x

  main :: Int
  main = if observe (Mono 4) == 4 then 0 else 1
```

## Rigid-index boundary

Index unification succeeds when the scrutinee's index is concrete. When the index is a rigid universally quantified variable (from `forall (n :: Nat)`), the checker does not currently refine `n` per branch when matching constructors with concrete target indices:

<!-- cinch:reject contains="this slice does not refine rigid indices" -->
```cinch
module Main where
  data Signal (channels :: Nat) where
    Mono :: Int -> Signal 1
    Stereo :: Int -> Int -> Signal 2

  observe :: forall (n :: Nat). Signal n -> Int
  observe signal = case signal of
    Mono x -> x

  main :: Int
  main = 0
```

To branch over an unknown index, either match at a concrete index or dispatch via `decide` and a witness (as `safeAt` does for `Fin`).

# `Prop`, erasure, and `%0`

`Prop` is the universe of compile-time evidence. Terms in `Prop` are **proof-irrelevant** and **erased before code generation**: they occupy zero runtime space and cannot influence runtime control flow.

Record fields marked `erased` carry compile-time invariants without taking a field slot in the heap layout:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Tagged where
    erased ok :: Eq 3 3
    val :: Nat

  get :: Tagged -> Nat
  get t = t.val

  main :: Int
  main = if get (Tagged { val = 4, ok = refl }) == 4 then 0 else 1
```

Because all proofs of a proposition erase identically, any two proofs of `Eq 3 3` are definitionally interchangeable:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  proof p1 :: Eq 3 3
  proof p1 = refl
  proof p2 :: Eq 3 3
  proof p2 = refl

  use :: Eq 3 3 -> Nat -> Nat
  use h x = x + 1

  main :: Int
  main = if use p1 1 == use p2 1 then 0 else 1
```

At function binders, multiplicity `%0` enforces the same phase separation: an erased parameter may appear in types and proofs, but never in a runtime computational position:

<!-- cinch:reject contains="multiplicity %0: x is erased and must never be used, but is used 1 time(s)" -->
```cinch
module Main where
  erase :: Nat %0 -> Nat
  erase x = x

  main :: Int
  main = 0
```

Summary of what survives to runtime:

- **Retained:** `Sigma` witnesses, ordinary value fields, and any computation with observable effects or possible divergence.
- **Erased:** `Prop` evidence, `erased` record fields, `%0` parameters, family type indices, and `spec`/`proof` declarations.

# Proof terms: `refl`, `congr`, and induction

Cinch uses explicit proof terms rather than a separate tactic language. The kernel admits four proof forms:

- **`refl`** — proves `Eq x x` for any index `x`.
- **`congr f e`** (or `congr e` when the lifting function is inferred) — lifts an equality `e :: Eq a b` through a total function `f` to `Eq (f a) (f b)`.
- **Structural induction via `case`** — branches on an inductive value, proving each case with recursive calls on structurally smaller constructor fields.
- **Named evidence** — references to in-scope `proof` declarations, `Yes` bindings from `decide`, or `Prop` parameters.

Proving reflexivity over an inductive unary counter by structural induction and congruence:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Count where
    Z :: Count
    S :: Count -> Count

  same :: (value :: Count) -> Eq value value
  same value = case value of
    Z -> refl
    S inner -> congr S (same inner)

  doubled :: Eq (S (S Z)) (S (S Z))
  doubled = same (S (S Z))

  main :: Int
  main = 0
```

If the function passed to `congr` does not produce the expected equality, the checker reports both the inferred and expected propositions.

# Totality, `spec`, and strict positivity

To keep `Prop` logically consistent, every `spec` and `proof` definition must terminate, and every inductive `data` type must be strictly positive.

## `spec` and `proof` termination

A `spec` function defines pure ghost logic for specifications and type-level reasoning; a `proof` defines a proposition witness. Both must decrease structurally on an inductive argument on every recursive call:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Three where
    Z :: Three
    S :: Three -> Three

  spec size :: Three -> Nat
  size t = case t of
    Z -> 0
    S u -> 1 + size u

  main :: Int
  main = 0
```

Because `spec` definitions are erased ghost code, calling one in an executable expression is rejected:

<!-- cinch:reject contains="it is erased ghost code and cannot be used computationally" -->
```cinch
module Main where
  data Three where
    Z :: Three
    S :: Three -> Three

  spec size :: Three -> Nat
  size t = case t of
    Z -> 0
    S u -> 1 + size u

  main :: Int
  main = size (S Z)
```

Non-decreasing recursion and mutual recursion cycles in logical definitions are both rejected:

<!-- cinch:reject contains="must terminate: no argument decreases structurally" -->
```cinch
module Main where
  data Three where
    Z :: Three
    S :: Three -> Three

  spec spin :: Three -> Nat
  spin t = spin t

  main :: Int
  main = 0
```

<!-- cinch:reject contains="mutually recursive" -->
```cinch
module Main where
  proof a :: Eq 3 3
  proof a = b
  proof b :: Eq 3 3
  proof b = a

  main :: Int
  main = 0
```

## Strict positivity

Inductive `data` definitions must be strictly positive: the type being defined cannot appear to the left of an arrow in any constructor field (negative occurrence), preventing non-terminating encodings of logical falsehood:

<!-- cinch:reject contains="occurs negatively in field `$f0` of constructor BadC" -->
```cinch
module Main where
  data Bad where
    BadC :: (Bad -> Bad) -> Bad

  main :: Int
  main = 0
```

Nested recursive occurrences inside another type constructor (such as `List Bush -> Bush`) are also rejected; only direct positive occurrences are currently checked:

<!-- cinch:reject contains="occurs nested inside another type constructor" -->
```cinch
module Main where
  data Bush where
    B :: List Bush -> Bush

  main :: Int
  main = 0
```

# Current boundaries

- **Rigid-index elimination.** Pattern matching on an indexed family whose scrutinee index is an unresolved rigid variable does not refine the index per arm; use concrete indices or `decide`.
- **Universe cumulativity and eta rules.** `Type 0 : Type 1 : ...` is enforced without implicit universe cumulativity; definitional eta equivalence for records and `Unit` is not yet finalized.
- **Executable recursion and `Diverge`.** Totality is strictly checked for `spec` and `proof` declarations. Requiring explicit `Diverge` annotations on arbitrary recursive executable functions is deferred, so ordinary recursive definitions are not currently proved total by the compiler.
- **Proof terms and laws.** Proofs are explicit terms (`refl`, `congr`, structural `case`, `decide`, and named evidence) with no tactic engine. A `rewrite ... justify` citing a class law records an explicit assumption in the build manifest without extending definitional equality.
- <span class="support-tag support-tag--planned">Coming soon</span> **Expanded proof automation and verified primitives.** Broader automated decision procedures and primitive proof certificates are planned; unsupported propositions will continue to be rejected explicitly rather than assumed.
