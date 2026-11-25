# Refinements, narrowing, and proof obligations

A refinement constrains a carrier type with a decidable predicate. Because the proof witness lives in `Prop` and is erased before code generation, a refined value has the exact runtime representation and cost of its carrier: `NonZero Nat` is a `Nat` at runtime. Refinements restrict which operations may be called. Any operation with a `using` precondition demands evidence at each call site, and the checker either discharges it automatically, accepts evidence from control flow or `decide`, or rejects the program naming the missing or disproved fact.

For general `Prop` proofs, `refl`, and indexed families, see [dependent types and proofs](08-dependent-types-and-proofs.md). For turning boundary checks into asserted LLVM contracts with `#[checked-params]`, see [safety assertions and LLVM contracts](22-safety-assertions-and-llvm.md).

# Declaring a refinement

Refinements are declared with `type Name params = refine (x :: T) with P` (a single `:` is also accepted inside the binder, though `::` is standard):

```cinch
type Small = refine (n :: Nat) with n < 5
type NonEmpty a = refine (xs :: List a) with NotNull xs
type Ascii = refine (s :: String) with Ascii s
type Sorted = refine (xs :: List Nat) with Ordered xs
```

A refinement may also accept index parameters before the carrier type, allowing the same name to act as both a refined type constructor and a predicate:

```cinch
type InRange (lo :: Int) (hi :: Int) a = refine (x :: a) with lo <= x && x <= hi
```

`InRange` and `NonZero` are defined in [Algebra.Arithmetic](../library/Algebra.Arithmetic.md) and available in the prelude. Bounds are inclusive, so a value of type `InRange 0 255 Nat` supplies `0 <= x && x <= 255` as a fact, and `InRange 0 255 r` is valid in predicate position. Standard library functions return refined types directly: `bytes-len :: Bytes -> InRange 0 9223372036854775807 Nat`, `char-code :: Char -> InRange 0 1114111 Nat`, and `u32-to-i64 :: U32 -> InRange 0 4294967295 I64`.

A refinement predicate must reference the refined binder and belong to the decidable fragment: the four builtin structural predicates, comparisons (`<`, `<=`, `>`, `>=`, `==`, `/=`), index equality, and conjunctions (`&&`) or disjunctions (`||`) of those.

| Structural predicate | Applicable carriers | Meaning |
| --- | --- | --- |
| `NotNull x` | `List a` | `x` is a `Cons` cell (non-empty). |
| `NonZero x` | `Nat`, `Int`, machine integers | `x /= 0`. |
| `Ascii x` | `String`, `List Char` | Every byte is strictly below `128`. |
| `Ordered x` | `List Nat` | Elements are non-decreasing. |

Invalid refinement declarations are rejected immediately at declaration time:

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

# Constructing and unrefining values

Every refinement type defines a transparent constructor sharing the type's name, taking the carrier value and a witness of its predicate:

```cinch
Small 3 (decide (3 < 5))
NonEmpty xs w
```

On compile-time constants such as `3 < 5`, the checker discharges the predicate statically with zero runtime code. On runtime values, branch on `decide` so the witness exists inside the `Yes` branch:

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

`unrefine value` extracts the underlying carrier and compiles to the identity. Comparison-only refinements (such as `InRange 0 255 Nat`) also weaken implicitly to their carrier type without `unrefine`; structural refinements (`NonEmpty`, `Sorted`, `Ascii`) require an explicit `unrefine`.

Library modules typically wrap transparent construction in smart constructors returning `Option`: [Data.NonEmpty](../library/Data.NonEmpty.md) exports `non-empty :: forall a. List a -> Option (NonEmpty a)`, and [Data.NonZero](../library/Data.NonZero.md) exports `non-zero :: Int -> Option (NonZero Int)` and `non-zero-nat :: Nat -> Option (NonZero Nat)`.

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

Invalid construction fails in one of three ways: the predicate is statically disproved, the predicate depends on an unbranched runtime value, or the supplied witness proves a different proposition:

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

# Deciding propositions with `decide`

`decide` evaluates a proposition in the decidable fragment and yields `Dec p` (`Yes` carrying an erased proof of `p`, or `No` carrying an erased refutation). During checking, the decision procedure classifies each proposition into one of five statuses:

| Status | Behavior | Evidence availability |
| --- | --- | --- |
| `Checked` | Statically folded to `Yes` with no runtime check. | Always available in the `Yes` branch. |
| `Disproved` | Statically folded to `No`. | Never (`Yes` is unreachable). |
| `Unresolved` | Compiled to a runtime test and branch. | Available inside the `Yes` branch. |
| `Unsupported` | Outside the decidable fragment (e.g., an abstract `Prop` variable or an unsupported predicate/operand; boolean combinations `And`, `Or`, and `Not` over decidable propositions are supported). | Compile error. |
| `Timeout` | Static reduction exceeded its step budget. | Compile error. |

In the module below, `runtime` compiles to a runtime comparison, while `static-yes` and `static-no` fold away at compile time:

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

Unsupported propositions (such as an abstract `Prop` variable) and expressions exceeding the static reduction step budget are rejected rather than guessed:

<!-- cinch:reject contains="obligation status: unsupported: there is no decision procedure for the proposition variable `p`" -->
```cinch
module Main where
  f :: forall (p :: Prop). Nat -> Nat
  f x = case decide p of
    Yes _ -> 1
    No _ -> 0

  main :: Int
  main = 0
```

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

Binding a `Yes` witness to `_` discards the evidence instead of bringing it into scope:

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

# Control-flow facts

Ordinary branching expressions automatically introduce facts into their branches without allocating `Dec` or naming witnesses. Because `if c then a else b` elaborates to `case c of True -> a; False -> b`, the `True` branch gains `c` and the `False` branch gains its negation. All four patterns below discharge the `NonZero` requirement of `/`:

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

Specifically, the checker extracts facts from:

- **Boolean conditions:** comparisons via prelude `Ord`/`Eq` on `Nat`, `Int`, machine integers, or `Char`; structural predicates (`if NonZero d then ...`); and boolean combinations with `&&`, `||`, and `not`. Floating-point comparisons are excluded because IEEE NaN breaks `not (x < y) <=> x >= y`.
- **Numeric `case` literals:** a literal branch `k -> ...` adds `x == k`, and fallback branches add `x /= k` for every preceding literal.
- **`comparing k x y`:** `LT`, `EQ`, and `GT` branches receive the corresponding numeric comparison.
- **List patterns:** `Cons` branches add `NotNull x`; `Nil` branches add `not (NotNull x)`.
- **Guards:** the guarded body assumes the guard holds; subsequent rows with matching bindings assume the guard failed.
- **Short-circuit connectives:** `p && q` checks `q` assuming `p` and asserts both when true; `p || q` checks `q` assuming `not p` and refutes both when false. Opaque `Bool` expressions contribute no facts.

In-scope facts combine via difference logic, so `i < n` automatically entails `i + 1 <= n`:

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

# Proof obligations at call sites

Functions state preconditions via `using (...) => ...` clauses referencing their named parameters (diagnostics display positional names `arg0`, `arg1`, ... when a signature omits parameter names). For example, [Data.Bytes](../library/Data.Bytes.md) declares:

```cinch
bytes-get :: using (i < bytes-len b) => (b :: Bytes) -> (i :: Nat) -> InRange 0 255 Nat
```

A `decide` branch (or `if i < bytes-len b`) discharges the obligation, and the refined `InRange 0 255 Nat` result flows directly into a `Nat` return type:

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

Calling `bytes-get` without bounds evidence fails with a diagnostic naming the unsatisfied obligation and the ways to discharge it:

<!-- cinch:reject contains="the obligation `0 < (Data.Bytes.bytes-len b)` needs `Dec` evidence or a branch fact at this use" -->
```cinch
module Main where
  import Data.Bytes

  first :: (b :: Bytes) -> Nat
  first b = bytes-get b 0

  main :: Int
  main = 0
```

Related patterns across the standard library:

- **Type-indexed bounds:** `Vec.getCopy` takes a `Fin n` index, embedding the bound in the index type rather than a `using` clause (see [dependent types and proofs](08-dependent-types-and-proofs.md)).
- **Total division:** `(/)` and `(%)` require `NonZero a`, satisfied automatically for non-zero literals or via branch/witness facts for variables.
- **Refined postconditions:** `nat-from-i64 :: using (0 <= x) => (x :: I64) -> refined (r == x) Nat` both demands a precondition and propagates an equality fact on its result:

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

Multiple obligations on one function are checked independently: `bytes-set` requires both `i < bytes-len b` and `InRange 0 255 Nat` for the byte value, reporting each missing fact separately.

# Narrowing and forgetting

Movement along the refinement lattice is asymmetric:

- **Weakening** (stronger to weaker) is implicit wherever the implication rules derive the target predicate — including interval containment (`n > 100 && n < 200` implies `n > 0`), conjunct projection, `length xs > 0` implying `NotNull xs`, and zero-excluding intervals implying `NonZero x`.
- **Strengthening** (weaker to stronger) requires explicit evidence via `narrow value to Target with witness`:

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

Omitting `with witness` on `narrow` is rejected:

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

Type ascription (`let small = (value :: refined (n > 0) Nat)`) weakens along derivable implications and rejects strengthening without evidence:

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

To make weakening explicit in source code, `forget (pred) from value` drops a named top-level conjunct (spelled with the refinement declaration's binder) at zero runtime cost. Multiple conjuncts may be dropped at once with `forget [p1, p2] from value`:

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

Attempting to `forget` a predicate that is not a top-level conjunct of the source refinement is a compile error:

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

Evidence tracks values and mutable places, not variable spellings. Any fact derived from reading a `Mut` reference is invalidated as soon as that place may have been written; re-decide after the mutation to obtain fresh evidence.

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

Facts derived from mutable places are invalidated by:

- Direct mutation (`refSet m v`).
- Passing a `Mut` reference to a function call.
- Calling a function whose effect row names the reference's region with a write-capable effect.
- Opaque writes (foreign calls, `eval`, or mutating `Bytes` primitives), which conservatively invalidate all `Mut`-derived facts.
- Shadowing or rebinding a variable name (facts over the old binding go out of scope).

Region escape and capability rules apply unchanged to a refined value's carrier.

# Type class instances and predicate preservation

A type class instance transfers to a refinement only when every method preserves the refinement predicate. For example, [Data.NonEmpty](../library/Data.NonEmpty.md) provides `Semigroup` (appending two non-empty lists is non-empty), whereas `Monoid` is rejected because `mempty = Nil` violates `NotNull`:

<!-- cinch:reject contains="breaks preservation law `Data.NonEmpty.NonEmpty.preserve-mempty`" -->
```cinch
module Main where
  import Data.NonEmpty
  import Algebra.Monoid

  implicit instance Monoid_NonEmpty x :: Monoid (NonEmpty x) where
    mempty = Nil

  main :: Int
  main = 0
```

By contrast, [Data.NonZero](../library/Data.NonZero.md) defines `Monoid (NonZero Int)` under multiplication (`1` is non-zero, and the product of non-zero integers is non-zero), and [Data.Sorted](../library/Data.Sorted.md) defines `Semigroup` via ordered merge. Laws that cannot be settled by sample checking are marked `not checked` in the module manifest rather than claimed as proved.

# Scope and limitations

- **No general SMT solver or refinement inference.** Automated discharge covers the implication table, difference logic, and bounded linear arithmetic over exact `Nat` and `Int` (each emitting a certificate re-checked by an independent kernel checker). Nonlinear arithmetic and propositions outside this fragment require explicit proof terms or restructuring.
- **Wrapping machine integers do not linearize.** Because fixed-width arithmetic can wrap, `i < n` does not entail `i + 1 <= n` over machine integers; exact `Nat` and `Int` arithmetic does linearize.
- **Floating-point refinements.** Declarations such as `type Probability = refine (x :: F64) with 0 <= x && x <= 1` are accepted, though floating-point literals are not part of surface syntax and NaN/finiteness predicates are planned.
- <span class="support-tag support-tag--planned">Coming soon</span> **Broader proof automation.** Expanded solver coverage, a tactic surface, and richer certificate formats are planned.
