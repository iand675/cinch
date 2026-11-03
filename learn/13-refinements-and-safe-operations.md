# 13. Refine values and make unsafe cases explicit

A refinement is a carrier value together with checked evidence of a predicate. The evidence is erased at runtime, so the representation remains the carrier's representation. The distinction remains in the type system, where it can justify operations that would otherwise be partial.

# A bounded value

<!-- cinch:run exit=0 -->
```cinch
module Main where
  type Small = refine (n :: Nat) with n < 5

  main :: Int
  main =
    let small = Small 3 (decide (3 < 5))
    in if unrefine small == 3 then 0 else 1
```

`Small` is a named refinement of `Nat`. The constructor takes a value and evidence; `decide` discharges the concrete true predicate. `unrefine` explicitly recovers the carrier. There is no silent subtype conversion letting every `Nat` pass as `Small`.

A literal false predicate is rejected. For an unknown value, branch on a runtime decision and construct the refined value only in the successful branch. `Yes witness` carries evidence of the proposition. `No _` requires another application outcome.

# Division needs a nonzero divisor

<!-- cinch:run exit=0 -->
```cinch
module Main where
  quotient :: Nat -> Nat -> Option Nat
  quotient n d = case decide (NonZero d) of
    Yes witness -> Some (nat-div n (NonZero d witness))
    No _ -> None

  main :: Int
  main = case quotient 100 4 of
    None -> 1
    Some n -> if n == 25 then 0 else 2
```

The operation consumes a `NonZero Nat` divisor. A literal such as `4` can discharge that refinement directly at compile time. A general variable needs evidence. This is why `nat-div n d` with two unrestricted `Nat` arguments is rejected instead of introducing a divide-by-zero runtime failure.

The same principle appears in byte indexing (`index < bytes-len buffer`), `Nat` subtraction (`right <= left`), and narrowing conversions (the source lies in the destination's range). Branch facts can discharge the obligations. The proof solver checks a supported arithmetic fragment; it is not a solver for all possible mathematical truths.

# Existing refinements

`Data.NonEmpty` provides `non-empty :: forall a. List a -> Option (NonEmpty a)`. `Data.NonZero` provides total constructors for mathematical integers and naturals. `Data.Sorted` and `Data.Ascii` capture other useful invariants. Their module references explain the predicate and exported operations.

An operation on a refinement must preserve its predicate. Appending nonempty lists preserves nonemptiness. The empty-list identity does not; this is why a transferred monoid instance cannot simply pretend its identity is nonempty. A successful finite law check still carries its finite-check status, rather than becoming a universal theorem.

# Exercise and answer

Change `quotient` to return `999` when the divisor is zero. Where does the check belong?

<details><summary>Answer</summary>

Keep the decision inside `quotient`, return `nat-div n (NonZero d witness)` from `Yes`, and return `999` from `No`. Its result type becomes `Nat`. The total operation stays behind its proof requirement; only the application's zero-divisor policy changes.

</details>
