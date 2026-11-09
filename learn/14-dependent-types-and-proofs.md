# 14. Dependent types, finite indices, and proofs

A dependent result type mentions an input value. Instead of saying that a function returns a vector, you can state its length: an index below `n` is a `Fin n`, a vector of length `n` is a `Vec n a`, and a pair that carries its own length is a `Sigma (n :: Nat). Vec n a`. The checker carries those indices through construction, coverage, and conversion, and the evidence it needs for them is erased from the program it runs.

Prerequisites: [refine values and make unsafe cases explicit](13-refinements-and-safe-operations.md) for `decide` and refinements, and [data and matching](04-data-and-matching.md) for constructors and coverage. The full account of universes, positivity, and obligation statuses is in [dependent types, the logical fragment, and evidence](../reference/08-dependent-types-and-proofs.md).

# A result type that mentions an argument

A binder written `(n :: Nat)` gives the argument a name that the rest of the type may use. The function below returns an index into a vector of length `n`, and only `n` and the value decide whether one exists.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Nat

  checked-index :: (n :: Nat) -> Nat -> Option (Fin n)
  checked-index n i = case decide (i < n) of
    Yes bound -> Some (Fin i bound)
    No _ -> None

  index-value :: (n :: Nat) -> Fin n -> Nat
  index-value n f = case f of
    Fin i bound -> i

  main :: Int
  main = case checked-index 3 1 of
    None -> 1
    Some f -> if index-value 3 f == 1 then 0 else 2
```

- The arrow `(n :: Nat) -> Option (Fin n)` is a dependent function type. The binder is written with `::` after a parenthesized name, and the rest of the type may mention it.
- `Fin n` is the builtin type of indices below `n`. Its constructor is `Fin i bound`, and `bound` is evidence of `i < n`. The evidence is a proof, so it is erased from the value.
- `checked-index` does not fabricate that evidence. It obtains it from `decide (i < n)`, which is why the result is an `Option` rather than a `Fin n`.
- `index-value` is the projection: matching `Fin i bound` returns `i` and discards the proof.

A request for index `3` at bound `3` returns `None`, because the comparison is strict. A caller cannot talk its way past that: constructing `Fin 3` with a false discharge is refused before anything runs.

# Vectors carry their length

`Vec n a` is a vector of length `n`. `Nil` has index zero and `Cons` adds one element, so the constructors themselves constrain the index. The expected type selects which constructor can appear.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Nat
  import Algebra.Arithmetic

  three :: Vec (2 + 1) Int
  three = Cons 10 (Cons 20 (Cons 30 Nil))

  sum3 :: Vec 3 Int -> Int
  sum3 v = case v of
    Cons x rest -> x + sum2 rest

  sum2 :: Vec 2 Int -> Int
  sum2 v = case v of
    Cons x rest -> x + sum1 rest

  sum1 :: Vec 1 Int -> Int
  sum1 v = case v of
    Cons x rest -> x

  only-empty :: Vec 0 Int -> Int
  only-empty v = case v of
    Nil -> 0

  pack :: forall (n :: Nat). Vec n Int -> Sigma (m :: Nat). Vec m Int
  pack xs = (n, xs)

  packed-length :: (Sigma (m :: Nat). Vec m Int) -> Nat
  packed-length p = case p of
    (m, v) -> m

  main :: Int
  main =
    if sum3 three == 60
       && only-empty Nil == 0
       && packed-length (pack (Cons 7 Nil)) == 1
    then 0 else 1
```

- `three` is declared at index `2 + 1` and passed to `sum3`, which demands `Vec 3 Int`. Conversion reduces the numeral arithmetic, so the two indices agree.
- `only-empty` matches `Nil` alone and is exhaustive. At index zero there is no `Cons`, so the checker does not ask for that arm.
- Each `sum1`/`sum2`/`sum3` step changes the expected index, so the pattern `Cons x rest` is usable with a `rest` of the smaller vector type. The compiler reports a mismatch when the count and the index disagree.
- `Sigma (m :: Nat). Vec m Int` packages a length with a vector of that length. `pack` returns the pair `(n, xs)`, and `packed-length` eliminates it with `(m, v)`, keeping the relationship between the two components. A plain tuple type `(Nat, Vec 3 Int)` would not state it, because its first component could be any number at all.

# Index arithmetic is structural

Conversion compares indices as terms, after reducing the arithmetic it supports. The two directions behave differently today, and the difference is visible in which indices you may write.

An index written `2 + 1` may be used where `3` is expected, as `three` is in `sum3` above. Building the other way is stricter: `Cons` produces index `n + 1`, so the written index must present its `+ 1` at the outside for the decomposition to succeed.

<!-- cinch:reject contains="index mismatch" -->
```cinch
module Main where
  wrong :: Vec (1 + 2) Int
  wrong = Cons 1 (Cons 2 (Cons 3 Nil))

  main :: Int
  main = 0
```

The report is `constructor Cons cannot build this value: index mismatch: 1 vs 2`. `2 + 1` builds three elements and `1 + 2` does not, although both describe the same number, because only the first matches the constructor's `n + 1` shape. Write the numeral when an index is known: `Vec 3 Int` accepts any three-element construction.

# Proofs: refl, congr, and total definitions

`refl` proves an equality that holds by computation. `congr` lifts an equality through a function on indices. Both are proof terms, and both are erased.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Count where
    Z :: Count
    S :: Count -> Count

  same :: (value :: Count) -> Eq value value
  same value = case value of
    Z -> refl
    S Z -> refl
    S (S tail) -> congr S (congr S (same tail))

  main :: Int
  main = 0
```

- `same` proves `Eq value value` for every `Count`, and its branches show the two ways a proof term is built: `refl` closes a goal whose sides are already the same term, and `congr S` turns evidence about a smaller index into evidence about `S` applied to it.
- `refl` fits when both sides reduce to the same term, as in `Eq 3 3` and `Eq value value`. At `Eq 3 4` it is refused with ``obligation disproved: `Eq 3 4` is false, so no evidence for it can exist here``.
- `congr S p` takes evidence of `Eq a b` to evidence of `Eq (S a) (S b)`. The first argument is a function on indices, and `S` is one. A lambda over runtime arithmetic is not: `congr (\x -> x + 1) refl` is refused with `runtime arithmetic needs an actual checked mathematical implementation before it can appear in a dependent type or refinement fact`.
- The recursion in `same` decreases through the constructor, which is what lets the checker accept it as a proof. One `refl` also closes this goal, because both sides are the same term; the recursive form is the shape a proof needs once the two sides differ.

A `proof` or `spec` definition is held to termination. `spec spin t = spin t` is refused with ``spin is a `spec`/`proof` definition and must terminate: no argument decreases structurally``, and two proofs that call each other are reported as mutually recursive. A definition that loops or performs an effect cannot be evidence.

# Proofs are erased

`Prop`-typed evidence has no runtime representation. A record can hold an erased field, and the field contributes nothing to the value the program manipulates.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Pair where
    left :: Nat
    right :: Nat

  record Tagged where
    erased ok :: Eq 3 3
    val :: Nat

  get :: Tagged -> Nat
  get t = t.val

  main :: Int
  main = if get (Tagged { val = 4, ok = refl }) == 4 then 0 else 1
```

The difference is visible in the emitted code. Running `cinch emit-ir Main.cinch -o out.ll` on that module prints one constructor function per record, and their arity follows the represented fields:

```text
define i64 @"Main.Pair"(i64 %p0, i64 %p1) #3 {
define i64 @"Main.Tagged"(i64 %p0) #3 {
```

`Pair` carries two runtime words and `Tagged` carries one. The `ok` field exists in the type, is checked at the construction, and occupies no space, which is why `t.val` is field zero.

# The rigidity limit

Matching a family at an unresolved index is where the current checker stops. The definition below is the standard append on length-indexed vectors, and it is refused.

<!-- cinch:reject contains="does not refine rigid indices" -->
```cinch
module Main where
  append :: forall (n :: Nat) (m :: Nat). Vec n Int -> Vec m Int -> Vec (n + m) Int
  append xs ys = case xs of
    Nil -> ys
    Cons x rest -> Cons x (append rest ys)

  main :: Int
  main = 0
```

<span class="support-tag support-tag--planned">Coming soon</span> **elimination at rigid indices.** The checker reports ``cannot match constructor Nil against Vec @value[4056] Int: the index equation 0 vs @value[4056] has no solution here (this slice does not refine rigid indices; reshape the type or dispatch through `decide`)``, where the bracketed number names the unsolved index. Refining a rigid index with an equation learned from a constructor match is the missing step; the design keeps the refusal conservative rather than guessing (decision D35).

Today, an index that must vary is written in one of three ways: at concrete indices, as in `sum3`, where each step has its own type; through `Fin` and a decision, as in `checked-index`; or with the length as an explicit parameter whose value the caller supplies, as in `pack`.

# What the compiler enforces

- `Type` is stratified, so `Type : Type` is refused rather than collapsing the hierarchy.
- Coverage accounts for indices. An impossible constructor needs no arm, and an arm for one is refused as impossible.
- `Fin n` can be built only with evidence of the bound, so a fabricated index is refused at the construction.
- Conversion uses definitional equality over supported computations on indices, which is what makes `Vec (2 + 1)` usable at `Vec 3`. Optimizer rewrite rules are not conversion axioms.
- Recursive datatype occurrences must be positive, and `spec`/`proof` definitions must terminate.
- Erased evidence cannot supply a runtime value, and proof irrelevance makes two proofs of one proposition interchangeable.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| `Fin 7 refl` at type `Fin 3` | ``obligation disproved: `7 < 3` is false, so no evidence for it can exist here`` |
| A `Cons` arm at `Vec 0` | `impossible arm: constructor Cons cannot build Vec 0 Int (the indices refute it), so this arm can never match` |
| `proof lie :: 3 < 2` proved by `refl` | ``obligation disproved: `3 < 2` is false, so no evidence for it can exist here`` |
| `spec spin t = spin t` | ``spin is a `spec`/`proof` definition and must terminate: no argument decreases structurally`` |
| `data Bad where BadC :: (Bad -> Bad) -> Bad` | ``Main.Bad occurs negatively in field `$f0` of constructor BadC`` |
| `append` matching at a `forall` index | `this slice does not refine rigid indices` |
| `(n : Nat) -> ...` | ``there is no fixity declared for the operator `:`, and there is no default for an undeclared operator`` |

The `impossible arm` message and the rigid-index message are different cases of index-aware coverage. The first means the equation is decided and the arm cannot match. The second means the equation is not solved at all, and the checker declines to assume the answer.

# Exercises

**Exercise 1.** `checked-index 0 0` returns `None`, and `Fin 0` cannot be constructed at all. What does each fact depend on?

<details><summary>Answer</summary>

Both depend on the same obligation. `Fin i bound` needs evidence of `i < n`, and no natural number is below zero, so the comparison `decide (0 < 0)` yields `No` and `checked-index` answers `None`. A caller that writes `Fin 0 refl` or `Fin 7 refl` at an unsatisfiable bound is refused with `obligation disproved`, because a failed check produces no evidence. The type keeps its invariant through every route into it, not through the constructor's good manners.

</details>

**Exercise 2.** Add a fourth element to `three`, which is declared at index `2 + 1`. Predict the report before running the compiler.

<details><summary>Answer</summary>

Each `Cons` consumes one from the declared index, and three elements consume `2 + 1` exactly, so the fourth has nothing left to spend. The report comes from the innermost `Nil`: `constructor Nil cannot build this value: index mismatch: 0 vs -1`. Change the declaration to `Vec 4 Int` and the same construction is accepted, because a spelled-out numeral is compared against the count directly.

</details>

Next: [productive streams and partial producers](15-streams-and-producers.md).
