# Tutorial 7 Rewrites and stream fusion

Source: <https://chatgpt.com/space/page_f812b35e00ec8191ae0252ac9dda7bc7>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Learn to register an optimization only after establishing the law it needs. Prerequisite: modules and stream reasoning from Tutorials 4 and 5.

# Begin with a pure law

For finite lists and total pure reusable callbacks:

```haskell
proof mapFusion :: (f :: Int -> Int) -> (g :: Int -> Int)
                -> (xs :: List Int)
                -> Eq (List.map f (List.map g xs))
                      (List.map (\x -> f (g x)) xs)
proof mapFusion f g xs = case xs of
  Nil -> refl
  Cons x rest -> congr (\ys -> Cons (f (g x)) ys) (mapFusion f g rest)
```

The proof uses the recursive definition of `List.map` and induction on the finite list. Mathematical `Int` avoids overflow ambiguity. This law is about pure finite values under the draft's idealized resource semantics; it does not promise identical allocation behavior or out-of-memory outcomes.

# Register the direction

```haskell
rewrite mapMap forall (f :: Int -> Int) (g :: Int -> Int)
                     (xs :: List Int). where
  match List.map f (List.map g xs)
  replace List.map (\x -> f (g x)) xs
  justify mapFusion f g xs
```

The theorem establishes validity. The rewrite declaration selects a direction. A phase or cost policy decides when to use it. The compiler checks applicability, including the callback types, and limits optimization work to avoid cyclic rewriting.

This rule does not become a new definitional equality in the type checker.

# Lower a producer pipeline

```haskell
sumSquares :: forall n region. Ref region (Vec n Nat) -> Nat
sumSquares xs = Stream.fold 0 (+)
  (Stream.map (\x -> x * x) (Vec.stream xs))
```

Here `stream()` borrows the vector and yields copied natural values. Its finite producer contract guarantees `n` elements. A lowering can specialize the producer state machine to a loop with an accumulator, eliminating intermediate collection materialization.

The compiler may still allocate for mathematical integers. A no-intermediate-collections assertion does not mean zero allocation.

# Reject an invalid extension

Replacing the callbacks with logging functions changes the question. In strict eager lists, nested maps log all inner calls before outer calls. A fused loop interleaves them. The pure theorem cannot justify that transformation.

An explicit effectful stream pipeline already has a specified stepping order, so a different theorem may justify eliminating its machinery while preserving that order.

# Inspect the result

A proposed optimization report should show the rule used, its law identity, any remaining producer allocation, and the location that blocked specialization. If code is imported as replaceable, the compiler must preserve its dispatch boundary unless it records the specialization dependency required for future updates.

# Exercise and answer

Can a proof of real-number associativity justify reassociating floating-point addition? No. Use the machine semantics, or require an explicitly selected approximate relation with a proved error bound.

Next: Tutorial 8 generates definitions and proofs at compile time.

# Platform specialization

A fused producer loop can be specialized again using target-indexed laws. The next proof must cover machine lane arithmetic, memory access, tails, and capability-scoped dispatch. A correct fusion theorem alone does not justify vectorization. Tutorial 10 develops a wrapping-integer reduction with a scalar fallback.

[Previous tutorial](06-processes-and-temporal-contracts.md) · [Next tutorial](08-staging-and-schema-generation.md) · [Documentation overview](../overview.md)