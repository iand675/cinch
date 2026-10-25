# Tutorial 4 Modules and laws

Source: <https://chatgpt.com/space/page_c16fa5ea6e148191831f925724d373f2>  
Exported: 2026-10-22T18:59:36Z

---

[Documentation overview](../overview.md)

Learn how one operation name can support several abstractions without erasing their differences. Prerequisite: dependent types, proofs, and effects from Tutorials 1 through 3.

# Package an operation and its model

For a reusable pure finite sequence, start with:

```haskell
signature FiniteSequence (c :: Type -> Type) where
  spec contents :: forall a. c a -> List a
  filter :: forall a. (a -> Bool) -> c a -> c a
  proof filterCorrect :: forall a. (p :: a -> Bool) -> (xs :: c a)
                      -> Eq (contents (filter p xs))
                            (List.filter p (contents xs))
```

The law says exactly which elements remain, including order and multiplicity. `contents` is a mathematical view; runtime code does not allocate that list just because a proof mentions it. This simplified interface assumes reusable immutable inputs. A resource-owning implementation needs a contract over views and ownership transfer.

# Choose a dictionary

```haskell
keepPositive :: forall c. using (sequence :: FiniteSequence c) => c Int -> c Int
keepPositive xs = sequence.filter (\x -> x > 0) xs
```

Callers can supply `PackedSequence` or `LinkedSequence` explicitly. An unambiguous implicit dictionary permits the shorter call. Laws are tied to that dictionary; a theorem about one equality or ordering implementation cannot justify an optimization using another.

# Allow the result type to change

Vectors need a more dependent signature:

```haskell
filterVec :: forall a n region. using (copy :: Copy a) =>
             (a -> Bool) -> Ref region (Vec n a)
             -> Sigma (m :: Nat). (Vec m a, Proof (m <= n))
```

`Proof` denotes an erased wrapper here. A complete correctness contract also establishes that the output equals the accepted subsequence. The size bound alone would allow an incorrect implementation that always returns the empty vector.

A general `Filtering` dictionary can supply an associated output family indexed by predicate and input view. Generic clients must additionally request the observation and laws they need; an arbitrary output family offers little useful reasoning by itself.

# Keep stream guarantees distinct

`filter` on a productive stream returns a search process unless there is evidence supporting total next-match search. A richer module can provide productive filtering when such evidence is supplied. An effectful source additionally specifies effect order, early termination, and cleanup.

These interfaces can share names and combinators while exposing different types. Implicit resolution should not silently switch between runtime algorithms based on a newly available proof.

# Exercise and answer

Would `all(output, p)` be a sufficient filter law? No. Returning nothing satisfies it. Require the exact accepted subsequence, or an equivalent combination of preservation, completeness, order, and multiplicity properties.

Next: Tutorial 5 constructs and proves properties of infinite streams.

[Previous tutorial](03-ownership-and-effects.md) · [Next tutorial](05-streams-and-coinduction.md) · [Documentation overview](../overview.md)