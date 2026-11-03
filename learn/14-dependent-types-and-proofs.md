# 14. Dependent types, finite indices, and proofs

A dependent result type can mention an input value. Instead of saying only that a value is a vector, you can state its length. A dependent pair packages a witness with a value whose type uses that witness.

# Check an index before constructing it

<!-- cinch:run exit=0 -->
```cinch
module Main where
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

The binder `(n :: Nat)` gives the argument a name inside the remaining type. `Fin n` represents an index below that bound. The function does not fabricate the bound evidence: it obtains it from `decide (i < n)`. A request for index 3 at bound 3 takes the `No` branch because the inequality is strict.

# Vectors retain a length

`Vec n a` represents a vector of length `n`. `Nil` has length zero; `Cons` adds one element. The expected type distinguishes the indexed constructors from the ordinary list vocabulary. Indices participate in coverage: a function on `Vec 0 a` need only handle `Nil`, because a nonempty constructor is impossible at that index.

<span class="support-tag support-tag--planned">Coming soon</span> **general elimination at rigid indices.** Some matches on a family at an unresolved rigid index are deliberately refused by the current Rust checker. The intended dependent language has broader indexed elimination. Today the diagnostic names the index equation; use a supported decision or computed-index formulation rather than assuming every textbook GADT match is accepted.

# Package an unknown length

`Sigma (n :: Nat). Vec n Int` means a pair whose first component is a natural number and whose second is a vector with that exact length. A filter can return such a pair because its output length is known only after it has examined the input. Pattern matching on `(n, xs)` brings both the witness and its relationship to `xs` into scope.

A regular tuple `(Nat, Vec 3 Int)` does not state that relationship: the first component can be any natural number, independently of the fixed three-element vector.

# Proofs and computation

The logical fragment uses `Prop`, proof irrelevance, and erasure. `refl` proves a definitional equality; it is not a universal escape hatch. Conversion reduces supported computations in types. `congr` transports equality through a function, while explicit proof terms and proof declarations supply evidence for more structured claims. Logical computation must satisfy the required purity and totality checks.

Rewrites in the optimizer do not extend definitional equality. An assumption recorded for an optimization is also not a new kernel theorem. See [dependent core reference](../reference/08-dependent-types-and-proofs.md) for universes, positivity, erasure, obligation statuses, and the supported proof boundary.

# Exercise and answer

What should `checked-index 0 0` return? Why can't a caller manufacture a `Fin 0` by choosing a larger numeral?

<details><summary>Answer</summary>

It returns `None`. No natural number is less than zero; choosing another value cannot make the bound predicate true. `Fin` construction requires checked evidence, so an invalid numeral and fabricated witness are rejected.

</details>
