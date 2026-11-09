# 13. Refine values and make unsafe cases explicit

A refinement is a carrier value together with checked evidence of a predicate about that value. The evidence is erased at runtime, so a refined `Nat` is a `Nat` with the same representation, no wrapper, and no allocation. The difference lives in the type, where that evidence justifies operations that would otherwise be partial: division with a divisor that cannot be zero, subscripting inside a buffer's length, a byte whose value is known to fit in eight bits.

Prerequisites: [options and validation](06-options-results-and-validation.md) for `Option` and `case`, and [numbers, text, and bytes](07-numbers-text-and-bytes.md) for `Nat` and `I64`. The full predicate vocabulary is in [refinements, narrowing, and proof obligations](../reference/06-refinements.md).

# Checking a value and proving a fact are different

Two questions can be asked about any predicate: is it true, and does the compiler know that it is true. The compiler acts only on the second.

| Predicate | What the checker knows | What you may write |
| --- | --- | --- |
| `3 < 5` over literals | Decided while checking | `Small 3 (decide (3 < 5))`, with no comparison in the running program |
| `k < 5` for a parameter `k` | Nothing, until you branch on `decide` | The refined construction only inside the branch that received evidence |
| `k < 5` for a parameter you never test | Nothing | Nothing that requires the evidence |

`decide proposition` reaches the second row from the third. It produces a value of type `Dec proposition` with two outcomes: `Yes witness` carries evidence that the proposition holds, and `No _` carries the other case. Evidence obtained from `Yes` is valid only inside that branch, and only for the values the decision examined.

# Declare a refinement

A `type` declaration names a refinement. The binder after `refine` names the carrier value inside the predicate, and the predicate must mention that binder.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  type Small = refine (n :: Nat) with n < 5

  main :: Int
  main =
    let small = Small 3 (decide (3 < 5))
    in if unrefine small == 3 then 0 else 1
```

`type Small = refine (n :: Nat) with n < 5` introduces `Small` as a `Nat` whose value is below five, without declaring a data type or allocating a wrapper. `Small 3 (decide (3 < 5))` is the transparent constructor: the carrier first, then the discharge, which settles during checking at a literal, so the produced value is the numeral `3`. `unrefine small` recovers the carrier. The other direction has no implicit form, so a `Nat` does not become a `Small` because a comment claims it is small.

# Evidence from a decision at runtime

A parameter's predicate is settled only when the program examines the value. `decide` builds that examination, and the `Yes` branch receives the evidence that the examination succeeded.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  type Small = refine (n :: Nat) with n < 5

  small-or-zero :: Nat -> Option Small
  small-or-zero k = case decide (k < 5) of
    Yes witness -> Some (Small k witness)
    No _ -> None

  main :: Int
  main = case small-or-zero 3 of
    None -> 1
    Some s -> if unrefine s == 3 then 0 else 2
```

The `Yes` branch is the only place where `Small k witness` typechecks, because it is the only place where evidence of `k < 5` exists. `No _` is an ordinary alternative that the function must handle. The comparison is code for every caller, including `small-or-zero 3`, which is what separates a decision about a value from a proof about literals.

# Division needs a nonzero divisor

`nat-div :: Nat -> NonZero Nat -> Nat` demands a divisor with evidence that it is not zero. The demand is a refinement on the second argument, so the caller supplies a value that already carries the evidence or produces one by decision.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Nat

  quotient :: Nat -> Nat -> Option Nat
  quotient n d = case decide (NonZero d) of
    Yes witness -> Some (nat-div n (NonZero d witness))
    No _ -> None

  main :: Int
  main = case quotient 100 4 of
    None -> 1
    Some n -> if n == 25 then 0 else 2
```

A numeral divisor needs nothing written. The predicate `NonZero 2` holds of the literal, so the checker discharges it before the program runs.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Nat
  import Algebra.Arithmetic

  halve :: Nat -> Nat
  halve n = nat-div n 2

  tenth :: Nat -> Nat
  tenth n = n / 10

  main :: Int
  main = if halve 100 == 50 then (if tenth 250 == 25 then 0 else 1) else 2
```

`n / 10` is the `Div Nat` method, whose second argument is also a `NonZero Nat`. Both definitions are total, and the literal discharges its obligation during checking rather than at the call. A zero literal fails at the literal:

<!-- cinch:reject contains="integer literal 0 is not a NonZero Nat" -->
```cinch
module Main where
  import Data.Nat
  import Algebra.Arithmetic

  bad :: Nat -> Nat
  bad n = n / 0

  main :: Int
  main = 0
```

# The obligation you cannot establish

Writing the division without the evidence produces the report that names the obligation, the use that needs it, and the ways to discharge it.

<!-- cinch:reject contains="the obligation `NonZero d` needs `Dec` evidence or a branch fact at this use" -->
```cinch
module Main where
  import Data.Nat

  quotient :: Nat -> Nat -> Nat
  quotient n d = nat-div n d

  main :: Int
  main = 0
```

The same refusal covers subtraction. `nat-sub` carries `using (b <= a)`, so `nat-sub a b` needs evidence that `b` does not exceed `a`, and `difference a b = nat-sub a b` is refused with ``the obligation `b <= a` needs `Dec` evidence or a branch fact at this use``. Writing `difference a b = if b <= a then Some (nat-sub a b) else None` supplies the fact from the branch condition. A refinement construction has its own version of this report, because the transparent constructor records a discharge rather than a general obligation.

<!-- cinch:reject contains="is not decided at this construction" -->
```cinch
module Main where
  type Small = refine (n :: Nat) with n < 5

  bad :: Nat -> Small
  bad k = Small k (decide (k < 5))

  main :: Int
  main = 0
```

Passing `decide (k < 5)` looks like evidence, and it is a `Dec` value, but the construction erases its witness. The checker requires the decision to have been taken, so the accepted form is the `case` in `small-or-zero`, where `Yes` hands over the witness it extracted.

# Subscripts need a bounded index

`bytes-get` demands `using (i < bytes-len b)`, so an unchecked index is refused and a guard supplies the fact. The result type is the refinement `InRange 0 255 Nat`, which a byte satisfies by construction.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Nat
  import Data.Bytes
  import Algebra.Arithmetic

  as-nat :: I64 -> Option Nat
  as-nat x = if 0 <= x then Some (nat-from-i64 x) else None

  byte-at :: Nat -> Bytes -> Option (InRange 0 255 Nat)
  byte-at i b = if i < bytes-len b then Some (bytes-get b i) else None

  main :: Int
  main = case as-nat 42 of
    None -> 1
    Some n -> case byte-at 1 (bytes-from-list (Cons 65 (Cons 66 Nil))) of
      None -> 2
      Some v -> if n == 42 && unrefine v == 66 then 0 else 3
```

`as-nat` shows a second precondition of the same kind: `nat-from-i64` requires `0 <= x`, and the `if` condition supplies exactly that. Both definitions return `Option`, because both decisions can go either way for a caller's input.

Written without the guard, `bytes-get b 0` is refused with the obligation `0 < (Data.Bytes.bytes-len b)` and the same list of ways to discharge it.

Vector access follows the same rule through a different type: `Vec.getCopy` takes a `Fin n` instead of a `Nat` index, so the bound travels in the index type rather than in a `using` row. [Chapter 14](14-dependent-types-and-proofs.md) builds those indices.

# Ranges the checker can compute

`InRange lo hi a` is a refinement whose predicate is `lo <= x && x <= hi`. A returned arithmetic expression satisfies it when the checker's difference logic can derive the bounds from the refinements already in scope, which is why the low byte of a UTF-8 encoding can be built without evidence.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Nat
  import Algebra.Arithmetic

  lead :: Nat -> InRange 0 255 Nat
  lead n = 192 + nat-rem n 32

  main :: Int
  main = if unrefine (lead 70000) == 208 then 0 else 1
```

`nat-rem n 32` returns `refined (r < d) Nat`, so the checker knows the remainder is below 32. Adding 192 keeps the result inside `0` to `255`, and the obligation on `InRange` is discharged at compile time. Move the constant out of that window and the same obligation fails:

<!-- cinch:reject contains="needs `Dec` evidence or a branch fact at this use" -->
```cinch
module Main where
  import Data.Nat
  import Algebra.Arithmetic

  wide :: Nat -> InRange 0 255 Nat
  wide n = 900 + nat-rem n 32

  main :: Int
  main = 0
```

The report for `wide` prints the whole conjunct it could not establish, so the missing arithmetic is visible rather than guessed at.

# Refinements the library ships

| Type | Carrier | Total constructor | Transparent constructor |
| --- | --- | --- | --- |
| `NonEmpty` | `List a` | `non-empty` | `NonEmpty xs witness` |
| `NonZero` | `Int`, `Nat`, machine integers | `non-zero`, `non-zero-nat` | `NonZero n witness` |
| `Sorted` | `List Nat` | `to-sorted` | `Sorted xs witness` |
| `Ascii` | `String` | `to-ascii` | `Ascii text witness` |

The total constructors return `Option`, so an input that fails the predicate is an ordinary value. Use one at every boundary where data arrives from outside: `ratio` decides once, and `nat-div n divisor` inside it needs no second test, because the refined divisor carries the evidence for every later use.
<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Nat
  import Data.NonZero
  import Algebra.Arithmetic

  ratio :: Nat -> Nat -> Option Nat
  ratio n d = case non-zero-nat d of
    None -> None
    Some divisor -> Some (nat-div n divisor)

  main :: Int
  main = case ratio 100 4 of
    None -> 1
    Some q -> case ratio 100 0 of
      Some _ -> 2
      None -> if q == 25 then 0 else 3
```

An instance for a refined type reuses the carrier's methods only when they preserve the predicate. Multiplication preserves `NonZero`, so instances exist at `NonZero Int`. A `Monoid` instance at `NonEmpty a` cannot exist, because `mempty` is the empty list. The checker refuses the instance with `breaks preservation law`, names `NonEmpty.preserve-mempty`, and prints a sample value that fails `NotNull xs`.

# What the compiler enforces

- A refinement's predicate must decide over its carrier and mention the binder, so `refine (xs :: List Nat) with NonZero xs` is refused with `NonZero` decides numeric values.
- A refined value does not stand in for its carrier: the projection is `unrefine`, and strengthening needs evidence or `narrow`.
- Evidence has a scope and a lifetime: the `Yes` branch of the decision that produced it, and until the values it describes are overwritten. A failed check produces no evidence, so a disproof cannot be turned into a value.
- A precondition in a `using` row, a refinement on an argument, or a bound in a type is an obligation at every use, discharged by an argument that carries the evidence, a branch fact, or compile-time arithmetic.
- An instance transfer must preserve the refinement, checked by the law layer against the named preservation law.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| `nat-div n d` with two `Nat` parameters | ``the obligation `NonZero d` needs `Dec` evidence or a branch fact at this use`` |
| `Small k (decide (k < 5))` | ``the discharge `k < 5` is not decided at this construction`` |
| `Small 9 (decide (9 < 5))` | ``the discharge `9 < 5` is disproved: no refined value can be constructed here`` |
| `n / 0` | ``integer literal 0 is not a NonZero Nat: its predicate `NonZero 0` is false`` |
| `decide (Not (x < 5))` | ``obligation status: unsupported: `decide` has no decision procedure for a negated proposition`` |
| `Monoid` instance at `NonEmpty a` | `breaks preservation law` followed by `NonEmpty.preserve-mempty` |

The `disproved` message and the `not decided` message are different failures of one constructor. The first says the predicate is false, so the evidence can never exist. The second says the predicate is undecided here, so the witness you passed was never extracted by a branch.

# Exercises

**Exercise 1.** Write a definition that divides `100` by a runtime `Nat`, returning `None` when the divisor is zero, and answer with the divisor's evidence rather than a second test.

<details><summary>Answer</summary>

Three shapes are accepted, and they differ in where the evidence comes from. `case non-zero-nat d of None -> None; Some divisor -> Some (nat-div 100 divisor)` uses the library constructor that `ratio` uses. `case decide (NonZero d) of Yes witness -> Some (nat-div 100 (NonZero d witness)); No _ -> None` reaches the same witness directly and needs no import. `if d == 0 then None else Some (nat-div 100 d)` leans on the branch fact: the `else` branch carries `d /= 0`, which implies `NonZero d`. What fails is passing `d` with no test at all, which is the `quotient n d = nat-div n d` report above.

</details>

**Exercise 2.** `clamp-byte` returns the value below 256 and `0` otherwise, at type `Nat -> InRange 0 255 Nat`. Which obligation does each branch discharge?

<details><summary>Answer</summary>

Write `clamp-byte n = if n < 256 then n else 0`. The `then` branch must show `0 <= n && n <= 255`: the lower bound holds because `n` is a `Nat`, and the upper bound follows from the branch condition. The `else` branch returns the literal `0`, which the checker evaluates against both bounds. Neither branch needs a `decide`, because the `if` condition already supplied the fact.

</details>

Next: [dependent types, finite indices, and proofs](14-dependent-types-and-proofs.md).
