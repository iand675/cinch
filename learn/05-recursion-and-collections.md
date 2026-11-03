# 5. Recursion, lists, and maps

A recursive function calls itself on a smaller part of its input. For a list, the natural smaller part is its tail. A base case stops the recursion. This makes list processing a useful place to learn recursion without a mutable loop counter.

# Take a list apart

<!-- cinch:run exit=0 -->
```cinch
module Main where
  total :: List Nat -> Nat
  total xs = case xs of
    Nil -> 0
    Cons x rest -> x + total rest

  main :: Int
  main = if total [3, 4, 5] == 12 then 0 else 1
```

`Nil` is the empty list. `Cons x rest` stores an element and the remaining list. The empty sum is zero; the nonempty sum adds one element to the result for the rest. `[3, 4, 5]` constructs a list through the `FromList` mechanism; with the expected `List Nat` type it has the same elements as `Cons 3 (Cons 4 (Cons 5 Nil))`.

A recursive call that does not decrease in an accepted way may require a `Diverge` effect. Giving a function an effect-free arrow makes a stronger claim than saying that it happened to terminate for one test input. [Totality and proofs](../reference/08-dependent-types-and-proofs.md) explain the supported checks and their limits.

# Transform and fold

Use library combinators when their meaning matches your operation. `map (\x -> x * 2) xs` transforms every element. `filter (\x -> x > 3) xs` keeps the elements satisfying a predicate. `foldr` replaces `Nil` with an initial result and `Cons` with a combining function. For example, `foldr (\x acc -> x + acc) 0 xs` calculates a sum.

These are strict computations over finite lists. Mapping a finite list does not create an automatically lazy stream. A productive infinite sequence uses `codata` and `Later`, introduced in [chapter 15](15-streams-and-producers.md).

# Tail recursion and local recursion

A tail-recursive helper carries the result accumulated so far. Its recursive call is the final action, leaving no addition waiting on return. The compiler supports self and mutual tail calls where the calling convention permits them; the compiler report records fallback cases.

Use `letrec` for local recursive functions. Occurrences in a recursive group must be guarded by a lambda, memo body, or `delay`. `letrec x = x + 1 in x` is an eager cycle and is rejected. Guarding admission is separate from proving termination: a lambda can still loop when called.

# Choose a map when keys matter

<!-- cinch:run exit=0 -->
```cinch
module Main where
  prices :: Map Nat Nat
  prices = {1 => 7, 2 => 12}

  main :: Int
  main = case map-lookup 2 prices of
    None -> 1
    Some price -> if price == 12 then 0 else 2
```

The map is ordered and lookup returns an `Option`, because a key can be missing. `{k => v}` uses `FromMap`; it does not denote a record. `Data.Set` and `Data.HashMap` provide other collection choices with explicit imports. Use the [library directory](../library/00-index.md) to find their exact constraints and operations.

# Exercise and answer

Write a function counting elements in a `List a`. Does the implementation need to know anything about `a`?

<details><summary>Answer</summary>

Give it `forall a. List a -> Nat`. Return `0` for `Nil` and `1 + count rest` for `Cons _ rest`. It does not inspect the element, so it needs no constraint on `a`.

</details>
