# Refinements, narrowing, and proof obligations

A refinement constrains a carrier type with a decidable predicate. `NonZero Nat` is a `Nat` at run time: the witness is `Prop` evidence, erasure removes it, and the value costs what its carrier costs. What the refinement changes is which operations apply. Every operation with a precondition demands evidence, and the checker either finds it, refuses naming the fact it is missing, or refuses naming the fact it disproved.

This page gives the spelling, the discharge rules, and the compiler's exact responses. [Dependent types and proofs](08-dependent-types-and-proofs.md) covers `Prop`, `refl` and indexed families. [Safety assertions and LLVM contracts](22-safety-assertions-and-llvm.md) covers `#[checked-params]`, which turns a boundary decision into an asserted contract. The normative statement is the [core language specification](../specifications/core-language-specification.md) section 5, as amended by the refinement entry in its decisions list; the reasoning behind the design is [refinement types](../design/refinement-types.md).

# Declaring a refinement

A `type` declaration whose right-hand side is a `refine` form registers a refinement. The binder takes `::` or `:`. These are declarations in a module body, each adding one name:

```cinch
type Small = refine (n :: Nat) with n < 5
type SmallColon = refine (n : Nat) with n < 5
type NonEmpty a = refine (xs :: List a) with NotNull xs
type Ascii = refine (s :: String) with Ascii s
type Sorted = refine (xs :: List Nat) with Ordered xs
```

A refinement may take index parameters before the carrier, which makes one name serve as both a type former and a predicate over a value. Inside a module body, that declaration is:

```cinch
type InRange (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi
```

`InRange` is declared in the prelude, in [Algebra.Arithmetic](../library/Algebra.Arithmetic.md), beside `NonZero`. The bounds are inclusive index parameters, so a value of type `InRange 0 255 Nat` contributes `0 <= x && x <= 255` as a fact, and `InRange 0 255 r` in predicate position is the same predicate over `r`. Library results are declared at refinements of this shape: `bytes-len :: Bytes -> InRange 0 9223372036854775807 Nat`, `char-code :: Char -> InRange 0 1114111 Nat`, `u32-to-i64 :: U32 -> InRange 0 4294967295 I64`.

The predicate must speak about the refined binder and must be decidable over the carrier. The four structural predicates are the ones the checker reads:

| Predicate | Carrier it decides | Meaning |
| --- | --- | --- |
| `NotNull x` | `List a` | `x` is not the empty constructor |
| `NonZero x` | `Nat`, `Int`, machine integers | `x` is not zero |
| `Ascii x` | `String`, `List Char` | every byte is below 128 |
| `Ordered x` | `List Nat` | non-decreasing order |

Everything else is a comparison (`<`, `<=`, `>`, `>=`, `==`, `/=`) over those carriers, index equality, or a conjunction (`&&`) or disjunction (`||`) of those. A declaration outside the vocabulary is refused when it is checked, not when it is used.

Each of these declarations is a complete module that the checker refuses.

<!-- cinch:reject contains="`NonZero` decides numeric values" -->
```cinch
module Main where
  type Bad = refine (xs :: List Nat) with NonZero xs

  main :: Int
  main = 0
```

<!-- cinch:reject contains="comparison or index equality" -->
```cinch
module Main where
  type Bad = refine (xs :: List Nat) with xs < 5

  main :: Int
  main = 0
```

<!-- cinch:reject contains="must mention the refined binder" -->
```cinch
module Main where
  type Bad = refine (xs :: List Nat) with NotNull (Cons 1 Nil)

  main :: Int
  main = 0
```

<!-- cinch:reject contains="a `type` declaration is a refinement" -->
```cinch
module Main where
  type T = Int

  main :: Int
  main = 0
```

# Constructing a refined value

A refined type has one transparent constructor, spelled with the type's own name, taking the carrier and evidence for the predicate:

```cinch
Small 3 (decide (3 < 5))
NonEmpty xs w
```

`decide` yields `Yes` with a witness or `No` with a refutation. At a numeral the checker decides the proposition itself, so `Small 3 (decide (3 < 5))` needs nothing at run time. For a value known only at run time, the witness exists only inside the `Yes` branch.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  type Small = refine (n :: Nat) with n < 5

  checked-small :: Nat -> Option Small
  checked-small n = case decide (n < 5) of
    Yes w -> Some (Small n w)
    No _ -> None

  main :: Int
  main = case checked-small 3 of
    Some s -> if unrefine s == 3 then 0 else 1
    None -> 2
```

`unrefine value` recovers the carrier. It is the identity at run time, because the representation is the carrier. A refined value whose predicate is comparisons only also flows into a plain expectation without `unrefine`, which is how `byte-at-or-zero` below returns an `InRange 0 255 Nat` as a `Nat`; `NonEmpty`, `Sorted` and `Ascii` do not, and project explicitly.

A library usually ships a total constructor that returns `Option` and builds the transparent one internally. [Data.NonEmpty](../library/Data.NonEmpty.md) exports `non-empty :: forall a. List a -> Option (NonEmpty a)`; [Data.NonZero](../library/Data.NonZero.md) exports `non-zero :: Int -> Option (NonZero Int)` and `non-zero-nat :: Nat -> Option (NonZero Nat)`. Use these at input boundaries. The `NonZero` type itself lives in [Algebra.Arithmetic](../library/Algebra.Arithmetic.md).

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.NonEmpty

  head-of :: List Nat -> Option Nat
  head-of xs = case non-empty xs of
    Some ne -> case unrefine ne of
      Cons x rest -> Some x
      Nil -> None
    None -> None

  main :: Int
  main = case head-of (Cons 7 Nil) of
    Some h -> if h == 7 then 0 else 1
    None -> 2
```

A construction that fails to establish its predicate is refused in one of three ways. A predicate that is false is disproved. A predicate over a run-time value is not decided at the construction. A witness of some other proposition is evidence for the wrong thing.

<!-- cinch:reject contains="the discharge `9 < 5` is disproved" -->
```cinch
module Main where
  type Small = refine (n :: Nat) with n < 5

  bad :: Small
  bad = Small 9 (decide (9 < 5))

  main :: Int
  main = 0
```

<!-- cinch:reject contains="the discharge `k < 5` is not decided at this construction" -->
```cinch
module Main where
  type Small = refine (n :: Nat) with n < 5

  bad :: Nat -> Small
  bad k = Small k (decide (k < 5))

  main :: Int
  main = 0
```

The third case reports an index mismatch and names both propositions:

<!-- cinch:reject contains="is evidence of `3 < 9`, not of `k < 5`" -->
```cinch
module Main where
  type Small = refine (n :: Nat) with n < 5

  bad :: Nat -> Small
  bad k = case decide (3 < 9) of
    Yes w -> Small k w
    No _ -> Small 0 (decide (0 < 5))

  main :: Int
  main = 0
```

# `decide` and its outcomes

`decide` takes a proposition from the decidable fragment and returns `Dec p`. `Yes` carries a proof of `p` and `No` a refutation, both erased. The checker runs its own decision procedure first, and its verdict is one of five statuses:

| Status | What `decide` returns or does | Evidence reaches the program |
| --- | --- | --- |
| Checked | The proposition folds to true: `decide` is `Yes`, at no run-time cost | Always, on the path that holds the `Yes` binder |
| Disproved | The proposition folds to false: `decide` is `No` | Never, since `Yes` is unreachable |
| Unresolved | The proposition mentions values only known at run time: `decide` compiles to the comparison and a branch | Only in the `Yes` branch at run time |
| Unsupported | No decision procedure applies (a negated proposition, a proposition variable, an unsupported operand type) | Refused at compile time |
| Timeout | Static evaluation ran out of reduction steps | Refused at compile time |

The three run-time outcomes are visible in one program. `runtime` compares at run time, `staticYes` and `staticNo` fold, and both branches exist in the compiled code only for `runtime`.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  runtime :: Nat -> Nat
  runtime x = case decide (x < 5) of
    Yes _ -> 1
    No _ -> 0

  static-yes :: Nat
  static-yes = case decide (2 < 3) of
    Yes _ -> 1
    No _ -> 0

  static-no :: Nat
  static-no = case decide (3 < 2) of
    Yes _ -> 1
    No _ -> 0

  main :: Int
  main = if runtime 3 == 1 && runtime 9 == 0 && static-yes == 1 && static-no == 0
         then 0 else 1
```

The two refusals are compile errors, and neither is a proof. A negated proposition has no decision procedure here:

<!-- cinch:reject contains="obligation status: unsupported: `decide` has no decision procedure for a negated proposition" -->
```cinch
module Main where
  f :: Nat -> Nat
  f x = case decide (Not (x < 5)) of
    Yes _ -> 1
    No _ -> 0

  main :: Int
  main = 0
```

Static evaluation has a step budget. A term deep enough to exhaust it reports the timeout instead of guessing. The message repeats the whole proposition, which is why the source line below is long:

<!-- cinch:reject contains="obligation status: timeout: deciding" -->
```cinch
module Main where
  f :: Nat -> Nat
  f x = case decide (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x)))))))))))))))))))))))))))))))) < 5) of
    Yes _ -> 1
    No _ -> 0

  main :: Int
  main = 0
```

That program produces:

```text
cinch: 3:14: obligation status: timeout: deciding `(x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + (x + (1 + x)))))))))))))))))))))))))))))))) < 5` ran out of reduction steps (in Main.cinch)
```

A `Yes` binder carries evidence only when it is named. `Yes _` binds nothing, and the obligation it would have discharged is refused:

<!-- cinch:reject contains="no evidence for `NonZero d` is in scope here" -->
```cinch
module Main where
  bad :: Nat -> (d :: Nat) -> Option Nat
  bad n d = case decide (NonZero d) of
    Yes _ -> Some (n / d)
    No _ -> None

  main :: Int
  main = 0
```

# Facts that a branch adds

A condition in the source is a fact in its branches, with no `Dec` value built and no obligation to write. `if c then a else b` is `case c of True -> a; False -> b`: the `True` branch knows `c`, the `False` branch knows its refutation. Four forms add the same fact, and the program below relies on all four to supply the `NonZero` divisor that `/` demands.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  by-if :: Nat -> (d :: Nat) -> Nat
  by-if n d = if d /= 0 then n / d else 0

  by-literal :: Nat -> (d :: Nat) -> Nat
  by-literal n d = case d of
    0 -> 0
    _ -> n / d

  by-guard :: Nat -> (d :: Nat) -> Nat
  by-guard n d = case n of
    0 -> 0
    _ | d > 0 -> n / d
    _ -> 0

  by-both :: Nat -> (d :: Nat) -> Nat
  by-both n d = if d /= 0 && n > 3 then n / d else 0

  main :: Int
  main = if by-if 9 3 == 3 && by-if 9 0 == 0
         && by-literal 9 3 == 3 && by-guard 9 3 == 3 && by-guard 9 0 == 0
         && by-both 9 3 == 3 && by-both 2 3 == 0
         then 0 else 1
```

The readings, in full:

- A `Bool` scrutinee read as a predicate gives its `True` branch the predicate and its `False` branch the refutation. The readable predicates are a comparison whose operator is the prelude's `Ord`/`Eq` method and whose operands are `Nat`, `Int`, a machine integer or a `Char`; a structural predicate written as the condition (`if NonZero d then`); and `&&`, `||` and `not` over those. Floats are excluded, because under IEEE a NaN makes `not (x < y)` differ from `x >= y`.
- A numeric scrutinee gives a literal branch `x == k` and the default branch `x /= k` for every literal, which is what `by-literal` uses.
- `comparing k x y` gives `LT`, `EQ` and `GT` the three comparisons at a numeric key.
- A list-shaped scrutinee gives a non-empty constructor's branch `NotNull x` and the empty one's branch `not (NotNull x)`.
- A guard is a `Bool` column over its row: the row's body is the `True` branch and the fall-through to later rows is the `False` branch, and it reaches only rows whose names denote the same values.
- `p && q` checks `q` in the `True` branch of `p` and holds both; `p || q` checks `q` in the `False` branch of `p` and refutes both. A `Bool` the checker cannot read carries no fact.

Facts combine, and the combination is decided by difference logic, so `i < n` entails `i + 1 <= n` without a second test:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  at-most :: using (k <= n) => (n :: Nat) -> (k :: Nat) -> Nat
  at-most n k = n - k

  advance :: (n :: Nat) -> (i :: Nat) -> Nat
  advance n i = if i < n then at-most n (i + 1) else 0

  main :: Int
  main = if advance 5 3 == 1 && advance 5 5 == 0 then 0 else 1
```

# Obligations at a use

An operation states its precondition as a `using` context on its signature. The operands are named positionally when the declaration does not name them (`arg0` is the first argument), or by the name the declaration gives them. [Data.Bytes](../library/Data.Bytes.md) declares:

```text
bytes-get :: using (arg1 < (Data.Bytes.bytes-len arg0)) => ((arg0 :: Bytes) -> ((arg1 :: Nat) -> InRange 0 255 Nat))
```

An index obligation is discharged by a decision over the same names. The result is a refined value, so it flows straight back out as `InRange 0 255 Nat` or as the plain `Nat` it is:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Bytes

  byte-at-or-zero :: (b :: Bytes) -> (i :: Nat) -> Nat
  byte-at-or-zero b i = case decide (i < bytes-len b) of
    Yes w -> bytes-get b i
    No _ -> 0

  main :: Int
  main = if byte-at-or-zero (bytes-from-list (Cons 65 Nil)) 0 == 65 then 0 else 1
```

Without the fact, the obligation is refused, and the message restates the obligation in the checker's own spelling and lists the ways to discharge it:

<!-- cinch:reject contains="the obligation `0 < (Data.Bytes.bytes-len b)` needs `Dec` evidence or a branch fact at this use" -->
```cinch
module Main where
  import Data.Bytes

  first :: (b :: Bytes) -> Nat
  first b = bytes-get b 0

  main :: Int
  main = 0
```

Vector indexing carries the same obligation in the type: `Vec.getCopy` takes its index as a `Fin n`, so the bound arrives as part of the index rather than as a `using` clause, and the caller builds the `Fin` with a decision ([dependent types](08-dependent-types-and-proofs.md) shows that construction).

Division works the same way. `(/)` and `(%)` are declared as `a -> NonZero a -> a`, so no instance can reach a zero divisor. A numeral divisor needs nothing written (`n / 2`), because its predicate is decided at compile time. Any other divisor is a `NonZero a` passed in, or a plain `a` under a fact.

A refined result is a fact wherever the value is used. `nat-from-i64 :: using (0 <= arg0) => ((arg0 :: I64) -> refined (r == arg0) Nat)` states both its precondition and its result, and a caller supplies the precondition from a decision:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Nat

  magnitude :: (x :: I64) -> Option Nat
  magnitude x = case decide (0 <= x) of
    Yes w -> Some (nat-from-i64 x)
    No _ -> None

  exact :: (x :: I64) -> Option (refined (r == x) Nat)
  exact x = case decide (0 <= x) of
    Yes w -> Some (nat-from-i64 x)
    No _ -> None

  main :: Int
  main = case magnitude 42 of
    Some n -> if n == 42 then 0 else 1
    None -> 2
```

`bytes-set` shows obligations stacked on one call: it requires the index bound `arg1 < bytes-len arg0` and `InRange 0 255 Nat` for the byte written, and each is refused separately when its evidence is absent.

# Narrowing and forgetting

The reorder along the lattice is one way by default. Weakening needs no evidence: a value at a stronger point is a value at any point the implication table derives, which includes interval containment (`> 100 && < 200` implies `> 0`), conjunct projection, and the vocabulary rows (`length xs > 0` implies `NotNull xs`; a range that misses zero implies `NonZero x`). Strengthening needs a witness, which is what `narrow value to Target with witness` carries.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  type Window = refine (n :: Nat) with n > 100 && n < 200
  type Positive = refine (n :: Nat) with n > 0

  up :: Positive -> Option Window
  up p = case decide (unrefine p > 100 && unrefine p < 200) of
    Yes w -> Some (narrow p to Window with w)
    No _ -> None

  main :: Int
  main = case up (Positive 150 (decide (150 > 0))) of
    Some w -> if unrefine w == 150 then 0 else 1
    None -> 2
```

The same cast without a witness is refused, and the message names the predicate it needs and points at the two directions:

<!-- cinch:reject contains="`narrow` carries a `Dec` of the stronger predicate" -->
```cinch
module Main where
  type Window = refine (n :: Nat) with n > 100 && n < 200
  type Positive = refine (n :: Nat) with n > 0

  up :: Positive -> Window
  up p = narrow p to Window

  main :: Int
  main = 0
```

Type ascription weakens, and it is the everyday spelling: `let small : refined (n > 0) Nat = value` keeps what the table derives. Ascribing to a point the table cannot derive is refused, with the obligation named:

<!-- cinch:reject contains="the obligation `(p > 100 && p < 200)` needs `Dec` evidence" -->
```cinch
module Main where
  type Window = refine (n :: Nat) with n > 100 && n < 200
  type Positive = refine (n :: Nat) with n > 0

  up :: Positive -> Int
  up p =
    let w = p :: Window in
    0

  main :: Int
  main = 0
```

`forget` names what is given up, for the case where the boundary should be visible in the source. It drops top-level conjuncts only, spells the atoms with the refinement declaration's own binder, and costs nothing at run time:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  type Window = refine (n :: Nat) with n > 100 && n < 200
  type Positive = refine (n :: Nat) with n > 0

  down :: Window -> Positive
  down w = forget (n < 200) from w

  widen :: Window -> Nat
  widen w = unrefine (down w)

  main :: Int
  main = if widen (Window 150 (decide (150 > 100 && 150 < 200))) == 150 then 0 else 1
```

The bracketed form drops several atoms: `forget [n > 100, n < 200] from w`. Forgetting an atom that is not a conjunct is an error, not a silent weakening:

<!-- cinch:reject contains="which is not a top-level conjunct of" -->
```cinch
module Main where
  type Window = refine (n :: Nat) with n > 100 && n < 200

  down :: Window -> Nat
  down w =
    let b = forget (n > 200) from w in
    0

  main :: Int
  main = 0
```

# Mutation invalidates evidence

A fact is about a value and a place, not about a spelling. Evidence derived from a mutable place is scoped to that borrow, and a write to the place invalidates it. The refusal names both the derivation and the write, so the fix is a fresh decision after the mutation.

<!-- cinch:reject contains="stale evidence" -->
```cinch
module Main where
  prog :: forall region. Mut region Nat -> Option (Fin 5)
  prog m =
    case decide (copyOut m < 5) of
      Yes bound ->
        let u = refSet m 9 in
        Some (Fin (copyOut m) bound)
      No _ -> None

  main :: Int
  main = 0
```

The triggers are `refSet m v`, a call passing a `Mut` argument, a call whose effect row names the region with a write-capable head, and opaque writes (foreign calls, `eval`, the mutating `Bytes` primitives), which conservatively invalidate every `Mut`-sourced fact. Rebinding a name has the same effect: a fact over a rebound name is out of scope. Region escape and capability rules apply to a refined value's carrier unchanged.

# Instances and preservation

An instance transfers from the container to the refinement exactly when the method preserves the predicate. [Data.NonEmpty](../library/Data.NonEmpty.md) carries `Semigroup` because appending two non-empty lists is non-empty, and cannot carry `Monoid` because its identity is the empty list.

<!-- cinch:reject contains="breaks preservation law `NonEmpty.preserve-mempty`" -->
```cinch
module Main where
  import Data.NonEmpty
  import Algebra.Monoid

  implicit instance Monoid_NonEmpty x :: Monoid (NonEmpty x) where
    mempty = Nil

  main :: Int
  main = 0
```

The checker reports the law's name, a sample, and the predicate that failed. [Data.NonZero](../library/Data.NonZero.md) is the contrast: its `Monoid (NonZero Int)` instance exists, because 1 is non-zero and a product of non-zeros is non-zero. [Data.Sorted](../library/Data.Sorted.md) has a `Semigroup` (merge) and no instance for a method that produces unsorted output from sorted input.

A law that the sample layer cannot settle reports `not checked` rather than claiming a proof. The instance exists either way; the manifest's law row states which it is, and a definition is never described as universally proved because its declaration parsed.

# What is refused or still planned

- No inference and no SMT solver. A proposition outside the fragment needs evidence you write, or a program shaped so that the fragment applies. Nonlinear arithmetic is not decided; a true nonlinear proposition is refused rather than assumed.
- Machine integer arithmetic is opaque to the arithmetic procedure except for the value each variable holds: `i + 1 < n` does not follow from `i < n` over a wrapping type, because at the maximum the sum wraps. `Nat` and `Int` arithmetic is exact and does linearize.
- A float carrier is accepted in a declaration, as `type Probability = refine (x :: F64) with 0 <= x && x <= 1` checks. No float literal exists in the bootstrap slice, so nothing can be constructed: `Probability 0.5 (...)` reports `float literals are not supported in the bootstrap slice (there is no AST node for them)`. The NaN and infinity predicates are the [float refinements](../design/float-refinements.md) design, not a feature you can use today.
- <span class="support-tag support-tag--planned">Coming soon</span> **Broader proof automation.** More solver coverage, a tactic surface, and certificate formats are in [proof automation](../design/proof-automation.md). Today the discharge is the table in `types.rs` plus difference logic and a bounded linear pass, each producing a certificate a separate checker re-reads.

Implementation: `compiler/src/check.rs`, `check_terms.rs`, `check_decls.rs`, `types.rs`, the prelude modules `Algebra/Arithmetic.cinch`, `Data/NonEmpty.cinch`, `Data/NonZero.cinch`, `Data/Sorted.cinch`, `Data/Ascii.cinch`. Suites: `refinements.rs`, `lattice.rs`, `linear_facts.rs`, `case_evidence.rs`, `evidence_invalidation.rs`.
