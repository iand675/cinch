# Tutorial 1 Values and dependent types

Source: <https://chatgpt.com/space/page_7e6a395e629c819195dc11bcf37cf789>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Learn the proposed notation by writing functions whose types describe their results. Prerequisite: ordinary functional programming. This is a design walkthrough, not an executable installation guide; the compiler and prelude do not yet exist.

# Start with strict functions

```haskell
twice :: Nat -> Nat
twice x = x + x

answer = twice 21
```

`answer` is `42`. `Nat` is an unbounded mathematical natural number in this draft. Function arguments are evaluated before the body. An explicitly pure arrow requires the compiler to justify termination; unsignatured local definitions may infer effects.

Use a machine integer type when bounded representation matters. Its checked addition returns an explicit overflow outcome; it does not silently inherit the semantics of `Nat`.

# Let a value appear in a type

```haskell
repeat :: forall a. using (copy :: Copy a) =>
          (n :: Nat) -> a -> Vec n a
repeat n x = case n of
  Zero -> Nil
  Succ k -> Cons x (repeat k x)
```

The constructor pattern `Succ k` matches a successor. The branches produce vectors of lengths `0` and `k + 1`. Recursion decreases `n`, so it is total. `Copy` is required because the element is reused.

`n` need not be a compile-time literal. A runtime input can determine the vector's length while the checker reasons symbolically about the relationship.

# Eliminate an impossible case

```haskell
first :: forall a n region. Ref region (Vec (n + 1) a) -> Ref region a
first xs = case xs of
  Cons x rest -> x
```

The implicit index `n` is universally quantified. `Nil` cannot have length `n + 1`, so it is impossible. The result borrows from the input, and the inferred lifetime prevents keeping the result after the input is destroyed.

Expected rejection:

```haskell
first Nil
```

There is no natural `n` satisfying `0 = n + 1`.

# Return a runtime witness

```haskell
pack :: forall a. List a -> Sigma (n :: Nat). Vec n a
pack xs = case xs of
  Nil -> (0, Nil)
  Cons x rest -> case pack rest of
    (n, tail) -> (n + 1, Cons x tail)
```

The dependent pair binds the returned length in the vector's type. The implementation must retain that length if later code reads it; dependent indices are not automatically erased.

# Exercise and answer

What should `zipExact` accept? Use two vectors with the same index:

```haskell
zipExact :: forall a b n. Vec n a -> Vec n b -> Vec n (a, b)
```

Its recursive implementation needs only the two matching constructor cases. A pair with unequal lengths is rejected at the call site or must first pass a runtime length-equality decision.

Next: Tutorial 2 establishes evidence from checks and explicit proofs.

[Next tutorial](02-proofs-and-runtime-checks.md) · [Documentation overview](../overview.md)