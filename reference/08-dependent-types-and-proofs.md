# Dependent types, the logical fragment, and evidence

The checker has a dependent core: dependent function and pair types, indexed families, a universe hierarchy, erased propositions, and a small proof kernel with `refl`, `congr` and structural induction. Its elimination and proof procedures are bounded on purpose, and the bounds are named in the diagnostics rather than left to be discovered. This page gives each construct with a program that compiles, and each boundary with the text the checker prints.

[Refinements, narrowing, and proof obligations](06-refinements.md) covers predicates over a carrier and the evidence that discharges them. The normative statement is the [core language specification](../specifications/core-language-specification.md) sections 2 to 4; the kernel's shape is [the dependent core kernel](../design/dependent-core-kernel.md); what the checker decides on its own is [proof automation](../design/proof-automation.md).

# Universes

`Type` is stratified, `Type 0 : Type 1 : ...`, and `Prop` sits below at `Prop : Type 0`. `Type : Type` is never admitted, so a type parameter at `Type 0` cannot be filled with `Type` itself.

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

`Nat` and `Int` are in `Type 0` with exact, unbounded semantics. Machine integers are primitives in `Type 0` with fixed representations, which is why index arithmetic over them is opaque to the checker while `Nat` and `Int` arithmetic is not.

# Dependent arrows and `Sigma`

A binder before an arrow may be named, and the result type may mention it. `bump` computes its result index from its argument, and the call `bump 2 (Tag 7)` therefore builds a `Tagged 3`:

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

`forall` binders carry a value annotation (`(n :: Nat)`) or a kind annotation (`(a :: Type)`). An implicit binding is solved from the application's context: `readTag (bump 2 (Tag 7))` supplies the index from the argument's type.

`Sigma (x :: A). B` is a dependent pair. Construction is the pair notation, elimination is a pair pattern, and the witness survives into run time:

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

Conversion is beta at elaboration, iota on numerals, alpha for Pi and `Sigma`, and proof irrelevance. User definitions do not unfold: a named function in an index position stays an opaque application, so `Vec (plus-one 2)` and `Vec 3` are different types and the refusal shows both sides. The program below is refused with `index mismatch` and the unreduced form `(Main.plus-one 2)` on one side.

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

Arithmetic *inside* an index does reduce on numerals, which is what makes the next program's type legal: `Vec (2 + 1) Int` is `Vec 3 Int`.

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

`Vec`, `Fin` and `Dec` are builtin families, because their runtime helpers need code the surface cannot express before the borrow recursion slice, and because `Vec`'s constructors are spelled `Nil` and `Cons`, which `List` already owns. Their declarations are these:

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

The indexed-declaration mechanism is general, and a program declares its own families the same way: `data Tagged (n :: Nat) where Tag :: Nat -> Tagged n` above, and `data Signal (channels :: Nat)` with constructors at fixed indices below.

`Fin` cannot be fabricated. Its second field is the erased evidence of `i < n`, and a bound that is false cannot produce evidence:

<!-- cinch:reject contains="obligation disproved: `7 < 3` is false" -->
```cinch
module Main where
  badIndex :: Fin 3
  badIndex = Fin 7 refl

  main :: Int
  main = 0
```

The accepted construction takes the bound from a static decision, and the index itself stays in the value:

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

# Constructor coverage with index equations

Coverage accounts for the indices of the family at the scrutinee's type. `Vec 0 Int` has no `Cons`, so matching `Nil` alone is exhaustive:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  onlyNil :: Vec 0 Int -> Int
  onlyNil v = case v of
    Nil -> 5

  main :: Int
  main = if onlyNil Nil == 5 then 0 else 1
```

The same rule makes an arm for a refuted constructor an error, not dead code to be ignored:

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

A user family behaves the same at a concrete index. `Signal 1` has only `Mono`, so one arm covers it:

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

# The limit on rigid-index elimination

The index equation is solvable when the scrutinee's index is a concrete term. When it is a rigid variable, from a `forall (n :: Nat)` parameter, the checker does not refine the family by the arm's index: every constructor remains possible and the match is refused. This is the boundary as it stands.

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

The full text is

```text
cinch: 8:5: cannot match constructor Mono against Main.Signal @value[4056]: the index equation 1 vs @value[4056] has no solution here (this slice does not refine rigid indices; reshape the type or dispatch through `decide`) (in Main.cinch)
```

`@value[4056]` is an internal name for the rigid index, not something you can write in source. Until that slice lands, index dispatch over an unknown index goes through `decide` and a witness, as `safeAt` does for `Fin`, or through a family whose scrutinee has a known index.

# `Prop`, erasure, and `%0`

`Prop` is the universe of erased evidence. A declaration may mark a record field `erased`, and the field then occupies no run-time slot, so a value whose first declared field is erased reads its second field first in the block:

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

Two proofs of one proposition are interchangeable, because both erase to nothing:

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

The `%0` multiplicity is the enforcement of the same separation at a binding: an erased parameter must never be read. Use it for evidence-shaped arguments that must not leak into output.

<!-- cinch:reject contains="multiplicity %0: x is erased and must never be used, but is used 1 time(s)" -->
```cinch
module Main where
  erase :: Nat %0 -> Nat
  erase x = x

  main :: Int
  main = 0
```

`Sigma` retains its witness (`unpack` above returns `m`), while family indices are not stored in blocks and erased evidence never is. Erasure may not remove computation: a term with an observable effect or a divergence is not erased, and a proof of a total fact vanishes entirely.

# `refl`, `congr`, and induction

`refl` proves `Eq x x` for any index `x`. `congr` lifts an equality through a total function. It takes the lifting function as an explicit index argument before the evidence, which is what `congr S (same inner)` does; the checker describes that form as `congr [f] e`, and the one-argument form `congr e` leaves the function implicit. A proof of a general statement is a `case` on the induction variable: each branch is a proof term, and the recursive call is on a structurally smaller value.

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

A proof term is `refl`, `congr [f] e`, a `case` on the induction variable, or the name of evidence. When a `congr` carries a function that does not prove the expected proposition, the message names both.

# Termination and positivity

A `spec` or `proof` definition must terminate: a looping function cannot inhabit a proposition. Structural recursion over a constructor field is accepted:

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

A `spec` definition is ghost code: it exists to be reasoned about, and it has no executable form, so calling one from `main` is refused.

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

A definition with no structural decrease is refused, and the message says which obligation is unsupported and what to supply:

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

Mutual recursion between logical definitions is a cycle, not a termination argument:

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

Recursive occurrences must be positive. A constructor field that mentions the type to the left of an arrow is refused:

<!-- cinch:reject contains="occurs negatively in field `$f0` of constructor BadC" -->
```cinch
module Main where
  data Bad where
    BadC :: (Bad -> Bad) -> Bad

  main :: Int
  main = 0
```

Nesting inside another type constructor is refused too: this slice checks direct occurrences only.

<!-- cinch:reject contains="occurs nested inside another type constructor" -->
```cinch
module Main where
  data Bush where
    B :: List Bush -> Bush

  main :: Int
  main = 0
```

# What the checker does not do yet

- Rigid-index elimination, above: a match on an unresolved index is refused, not accepted conditionally.
- The universe hierarchy holds for `Type 0`, and its cumulativity is still an open decision in the kernel design, as is an eta rule for records and units.
- Totality is required of `spec` and `proof` definitions. Requiring the `Diverge` effect of a recursive executable definition is a named deferral (decision D32 in the [core language specification](../specifications/core-language-specification.md)), so a recursive executable function is not proved total by the compiler today.
- No tactic layer. Proofs are terms: `refl`, `congr`, `case` on the induction variable, `decide` and named evidence. A `rewrite ... justify` may cite a class law and record an undischarged assumption; that does not extend definitional equality.
- <span class="support-tag support-tag--planned">Coming soon</span> **Broader proof automation and primitive proofs.** The [proof automation](../design/proof-automation.md) and [verified primitives](../design/verified-primitives.md) designs describe the intended solver and certificate work. A future checker has to keep the rule these examples show: an unsupported proposition is refused, never accepted because it was written.

Implementation: `compiler/src/check.rs`, `check_decls.rs`, `check_terms.rs`, `types.rs`. Suites: `dependent.rs`, `case_evidence.rs`, `linear_facts.rs`, `syntax_proof_patterns.rs`, `bootstrap_source_constructors.rs`, `evidence_invalidation.rs`.
