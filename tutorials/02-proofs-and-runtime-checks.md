# Tutorial 2 Proofs and runtime checks

Source: <https://chatgpt.com/space/page_811f5c451ea481919981aba5cd276baf>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Learn how static evidence, runtime decisions, and unfinished proofs differ. Prerequisite: Tutorial 1. The examples use proposed prelude operations with the contracts described here.

# Build a bounded index

`Fin n` contains a natural number `i` and erased evidence that `i < n`. Its constructor is available only when that evidence exists.

```haskell
checkedIndex :: (n :: Nat) -> Nat -> Option (Fin n)
checkedIndex n i = case decide (i < n) of
  Yes bound -> Some (Fin i bound)
  No _ -> None
```

`decide` uses a total decision procedure for natural-number comparison. The successful result carries evidence about this particular `i` and `n`. It does not prove that all caller-supplied indices are valid.

```haskell
tryGet :: forall a n region. using (copy :: Copy a) =>
          Ref region (Vec n a) -> Nat -> Option a
tryGet xs i = case checkedIndex n i of
  Some index -> Some (Vec.getCopy xs index)
  None -> None
```

Here the length is available from the vector representation or a retained index. `Vec.getCopy` requires `Fin n` and returns the element without a bounds-error path. `tryGet` exposes invalid input as `None`.

# Prove a general statement

```haskell
proof plusZero :: (n :: Nat) -> Eq (n + 0) n
proof plusZero n = case n of
  Zero -> refl
  Succ k -> congr Succ (plusZero k)
```

This example uses addition defined recursively on its first argument. `refl` proves definitional equality. `congr` transports equality through a total function. The recursive proof is smaller because it uses `k`.

Proof execution is erased. Its recursive structure is checked before erasure, so infinite recursion cannot be used to manufacture evidence.

# Strengthen an interface gradually

A function initially returning `List a` can later return a package proving its result is sorted and a permutation of the input. Existing weaker code may still use the list projection. Code relying on sorting must request the stronger package from the beginning.

The compiler cannot silently upgrade the original return type because tests passed. Nor can it reinterpret a proof-search timeout as success.

# Diagnose three different failures

| Situation | Required response |
| --- | --- |
| `Fin(7, proof)` for a bound of `3` | Refute the obligation or report a concrete contradiction |
| A valid theorem exceeds the automation budget | Report unresolved evidence and allow an explicit lemma |
| A runtime index is unknown statically | Use a decision procedure and handle its failure branch |

An editor hole such as `?bound` can show the current context, but it cannot appear in an accepted artifact whose contract requires that proof.

# Exercise and answer

Can `decide(eventuallyResponds(server))` prove liveness at startup? No general decision procedure exists for that claim. Use a static temporal argument with explicit environment assumptions, or expose a bounded timeout outcome without calling it a proof of eventual response.

Next: Tutorial 3 explains how mutation and effects constrain evidence.

[Previous tutorial](01-values-and-dependent-types.md) · [Next tutorial](03-ownership-and-effects.md) · [Documentation overview](../overview.md)