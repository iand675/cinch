# Data.Set

Ordered sets and their membership, combination, difference, and conversion operations.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Set`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Set.cinch) · [How library modules are loaded](00-index.md)

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Set: the ordered set.

A height-augmented AVL tree over `Ord a`: `set-insert` keeps the
left subtree strictly smaller and the right strictly greater (so a
repeated element keeps one copy) and rebalances on the way out, so
the operations are O(log n) and the tree is never deeper than
~1.44 log2(n). The earlier family was the documented UNBALANCED
bootstrap BST (decision D105's shape, "a balanced tree is a later
family, same surface"): this is that later family, and the
observable semantics are exactly as before (a repeated element
keeps one copy, insert/lookup round trip, `set-to-list` lists the
tree's elements in ascending order, each exactly once).

The node carries its cached height (`Bin h x l r`): the balanced
family needs O(1) balance metadata, so the bootstrap's three-field
node grows its height field (an observable shape change, reported
with P34; the operations' semantics are unchanged and no test
constructs a node by hand). A hand-built `Bin` tree is still a
correct search tree for every operation (`set-member`,
`set-to-list`, `set-size` never read heights), and `set-insert`
repairs the heights along the path it rebuilds; heights a hand-built
node lies about may cost it a rebalance, never correctness.

Named refusals (boundaries this module deliberately does not
cross):
  an arbitrary-height join: `balance` fixes subtrees that differ by
    at most two levels, so the ordered operations that drop a node
    rejoin through the right subtree's minimum (`set-glue`), and
    `set-filter`/`set-map` rebuild through `set-from-list` rather
    than a structural rejoin. That rebuild is why `set-filter`
    states `Ord a` where Data.Set's `filter` needs nothing;
  a size-augmented node: the node carries the height (AVL), so
    `set-size` stays the O(n) walk it has always been.

The bootstrap's deletion and union refusals are over: the family now
ships `set-delete`, `set-union`, `set-intersection`,
`set-difference`, `set-subset`, `set-map`, `set-filter` and
`set-fold` (the union behavior the bootstrap asserted test-locally
in compiler/tests/prelude_split.rs is now the family's own).

Module scheme (design/base-library.md): the type's home carries its
`FromList` instance (`[a, b, c]` at `Set a` folds `set-insert`).

</details>

# Types and constructors

<a id="type-536574"></a>

## `Set`

```cinch
data Set (a :: Type)
```

The ordered set: a height-augmented AVL tree over `Ord a`.

[Source](/reference-sources/Data.Set.cinch) · line 50

```cinch
Tip :: forall a . Set a
```

```cinch
Bin :: forall a . (Nat -> (a -> (Set a -> (Set a -> Set a))))
```

# Values

<a id="value-7365742d64656c657465"></a>

## `set-delete`

```cinch
set-delete :: forall a . using (ord :: Ord a) => (a -> (Set a -> Set a))
```

Removal: the search descends as `set-member` does, and a found
node's subtrees rejoin through the right subtree's smallest element
(the AVL delete), so the height invariant survives. An absent
element rebuilds the same tree.

[Source](/reference-sources/Data.Set.cinch) · line 141

<a id="value-7365742d646966666572656e6365"></a>

## `set-difference`

```cinch
set-difference :: forall a . using (ord :: Ord a) => (Set a -> (Set a -> Set a))
```

The elements of the left set that the right set does NOT hold.

[Source](/reference-sources/Data.Set.cinch) · line 208

<a id="value-7365742d656d707479"></a>

## `set-empty`

```cinch
set-empty :: forall a . Set a
```

[Source](/reference-sources/Data.Set.cinch) · line 54

<a id="value-7365742d66696c746572"></a>

## `set-filter`

```cinch
set-filter :: forall a . using (ord :: Ord a) => ((a -> Bool) -> (Set a -> Set a))
```

The elements the predicate accepts, rebuilt from the surviving
ascending listing. The rebuild is why this states `Ord a` where
Data.Set's `filter` needs nothing: `balance` only fixes subtrees
that differ by at most two levels, so a structural filter (which
can empty one whole side) has no join to use and reinserts
instead. The rebuild also means duplicates cannot appear and the
ascending order is re-derived.

[Source](/reference-sources/Data.Set.cinch) · line 243

<a id="value-7365742d666f6c64"></a>

## `set-fold`

```cinch
set-fold :: forall a b . ((a -> (b -> b)) -> (b -> (Set a -> b)))
```

A RIGHT fold in ascending order (Data.Set's `foldr`): folding with
`Cons` rebuilds `set-to-list`, and the empty set is the identity.

[Source](/reference-sources/Data.Set.cinch) · line 255

<a id="value-7365742d66726f6d2d6c697374"></a>

## `set-from-list`

```cinch
set-from-list :: forall a . using (ord :: Ord a) => (List a -> Set a)
```

[Source](/reference-sources/Data.Set.cinch) · line 132

<a id="value-7365742d696e73657274"></a>

## `set-insert`

```cinch
set-insert :: forall a . using (ord :: Ord a) => (a -> (Set a -> Set a))
```

[Source](/reference-sources/Data.Set.cinch) · line 102

<a id="value-7365742d696e74657273656374696f6e"></a>

## `set-intersection`

```cinch
set-intersection :: forall a . using (ord :: Ord a) => (Set a -> (Set a -> Set a))
```

The elements of the left set that the right set also holds.

[Source](/reference-sources/Data.Set.cinch) · line 197

<a id="value-7365742d69732d656d707479"></a>

## `set-is-empty`

```cinch
set-is-empty :: forall a . (Set a -> Bool)
```

Named `set-is-empty` because the bare `is-empty` is the list's
(`List a -> Bool`) and there is no overloading to share one
spelling; the module prefix is how the family disambiguates.

[Source](/reference-sources/Data.Set.cinch) · line 179

<a id="value-7365742d6d6170"></a>

## `set-map`

```cinch
set-map :: forall a b . using (ord :: Ord b) => ((a -> b) -> (Set a -> Set b))
```

The image of a set under a function (Data.Set's `map`): every
element is rewritten and reinserted, so two elements that collide
keep one copy and the result is ordered by the NEW values, hence
`Ord b` and never `Ord a`.

[Source](/reference-sources/Data.Set.cinch) · line 233

<a id="value-7365742d6d656d626572"></a>

## `set-member`

```cinch
set-member :: forall a . using (ord :: Ord a) => (a -> (Set a -> Bool))
```

[Source](/reference-sources/Data.Set.cinch) · line 110

<a id="value-7365742d73697a65"></a>

## `set-size`

```cinch
set-size :: forall a . (Set a -> Nat)
```

[Source](/reference-sources/Data.Set.cinch) · line 127

<a id="value-7365742d737562736574"></a>

## `set-subset`

```cinch
set-subset :: forall a . using (ord :: Ord a) => (Set a -> (Set a -> Bool))
```

Whether every element of the first set is in the second (Data.Set's
`isSubsetOf`, kept under the module prefix, which is how every
other ordered operation here names itself).

[Source](/reference-sources/Data.Set.cinch) · line 221

<a id="value-7365742d746f2d6c697374"></a>

## `set-to-list`

```cinch
set-to-list :: forall a . (Set a -> List a)
```

[Source](/reference-sources/Data.Set.cinch) · line 124

<a id="value-7365742d756e696f6e"></a>

## `set-union`

```cinch
set-union :: forall a . using (ord :: Ord a) => (Set a -> (Set a -> Set a))
```

The union inserts the left set's ascending listing into the right;
`set-insert` keeps one copy per element and rebalances, so the
result is a proper AVL tree. The result is symmetric in its
elements but the cost is not: O(m log n) for m and n elements.

[Source](/reference-sources/Data.Set.cinch) · line 188

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Set.FromList_Set` | `FromList (Set a) a` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Set.cinch`. Source SHA-256: `da051f79c03b512657f4bb1521f307cda5eaa38cc6a82ec1dc94ec1d8988fd64`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
