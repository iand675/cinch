# Data.HashMap

Hash maps with explicit key constraints, lookup/update, and map-style combination operations.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.HashMap`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.HashMap.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Hash](Data.Hash.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.HashMap: the hash map over `Data.Hash`.

A tree-indexed bucket table: the key's hash digest indexes a
height-augmented AVL spine (Data.Set's balanced shape), and each
spine node holds the association list of the keys that hash to that
digest. A hash collision degrades the bucket to a scan without
losing either key, exactly as the bootstrap's fixed 8-bucket table
documented; what grows is the index: the spine rebalances as new
digests arrive, so the operations are O(log d) plus the bucket scan
for d distinct digests (the earlier family was the documented
fixed-size stub: "no resizing", now replaced). Keys need `Hash` and
`Eq`: hashing alone cannot decide a collision, and the `Hash` law
(Data.Hash) is what makes the bucket choice sound.

The map wraps its private spine (`HashMap :: Spine k v -> HashMap k
v`): the bootstrap's one-field node over a bucket-list spine grows
the balanced index behind the same constructor name (an observable
shape change, reported with P34; every operation's semantics is
unchanged and no test constructs a map by hand).

Named refusals (boundaries this module deliberately does not
cross):
  bucket-order observation: the buckets' entry order is an
    implementation detail (insertion order within a bucket), so
    `hash-map-to-list`/`hash-map-keys`/`hash-map-values` list the
    entries in that unstable order and nothing may depend on it (the
    ordered views live at Data.Map);
  a fixed bucket count: the digest, not `rem` of a constant, is the
    index (there is no `hash-buckets`);
  adversarial digests: the `Hash` law is the caller's contract (the
    spine balances whatever digests arrive; only bucket scans
    degrade).

Measured limits (bootstrap/programs/Run.Maps.cinch, asserted by
compiler/tests/lib_hashmap_real.rs; spans are the program's own
monotonic clock, and the ranges below span runs on a loaded machine):
32000 distinct-digest keys insert in 0.2-0.4s and look up in
0.05-0.1s, where 8x the keys costs about 10x the time (n log n; an
8x jump to 64x is what a quadratic index or a fixed table with
unbounded buckets would show). There is no growth or rehash step to
trigger, because the spine holds one node per digest PRESENT and its
height is the only thing that grows. 256 distinct keys that share ONE
digest insert and look up in 0.7-0.9s each, which is the whole cost of
the named boundary above (the bucket scan, linear in the collision
class: 300 keys in one bucket measure at 0.8-0.9s, and correctness
does not move). The key count itself has no ceiling short of memory:
100000 distinct-digest keys insert in ~0.8s and look up in ~0.2s.

Module scheme (design/base-library.md): the type's home carries its
`FromMap` instance (`{k => v}` at `HashMap k v`).

</details>

# Types and constructors

<a id="type-486173684d6170"></a>

## `HashMap`

```cinch
data HashMap (k :: Type) (v :: Type)
```

[Source](/reference-sources/Data.HashMap.cinch) · line 65

```cinch
HashMap :: forall k v . (Spine k v -> HashMap k v)
```

# Values

<a id="value-686173682d6d61702d64656c657465"></a>

## `hash-map-delete`

```cinch
hash-map-delete :: forall k v . using (h :: Hash k, eq :: Eq k) => (k -> (HashMap k v -> HashMap k v))
```

Removal: the bucket loses the key's entry, and a bucket left empty
drops its digest node from the spine, so deleting every key leaves
the same `STip` spine `hash-map-empty` has (nothing accumulates).

[Source](/reference-sources/Data.HashMap.cinch) · line 164

<a id="value-686173682d6d61702d656d707479"></a>

## `hash-map-empty`

```cinch
hash-map-empty :: forall k v . HashMap k v
```

[Source](/reference-sources/Data.HashMap.cinch) · line 139

<a id="value-686173682d6d61702d66696c746572"></a>

## `hash-map-filter`

```cinch
hash-map-filter :: forall k v . using (h :: Hash k, eq :: Eq k) => ((k -> (v -> Bool)) -> (HashMap k v -> HashMap k v))
```

Keep the entries the predicate accepts, rebuilt through
`hash-map-from-pairs`. The rebuild, not a structural filter, is why
the constraints are stated: an emptied bucket would leave a digest
node to drop, and dropping one is Data.Set's arbitrary-height
rejoin problem. The predicate sees the key as well as the value.

[Source](/reference-sources/Data.HashMap.cinch) · line 325

<a id="value-686173682d6d61702d666f6c64"></a>

## `hash-map-fold`

```cinch
hash-map-fold :: forall k v b . ((k -> (v -> (b -> b))) -> (b -> (HashMap k v -> b)))
```

A RIGHT fold in `hash-map-to-list`'s order: the fold visits every
entry exactly once, and that order is the buckets' unstable order,
so an order-dependent combining function must not be relied on
(folding with `Cons` rebuilds `hash-map-to-list` itself, and the
empty map is the identity). The right direction is what makes the
`Cons` rebuild agree with the view.

[Source](/reference-sources/Data.HashMap.cinch) · line 342

<a id="value-686173682d6d61702d66726f6d2d7061697273"></a>

## `hash-map-from-pairs`

```cinch
hash-map-from-pairs :: forall k v . using (h :: Hash k, eq :: Eq k) => (List (k, v) -> HashMap k v)
```

Pairs fold in from the head, so an EARLIER pair wins a repeated
key (the head is inserted last into the fold of the tail), like
Data.Map's `map-from-pairs`.

[Source](/reference-sources/Data.HashMap.cinch) · line 155

<a id="value-686173682d6d61702d696e73657274"></a>

## `hash-map-insert`

```cinch
hash-map-insert :: forall k v . using (h :: Hash k, eq :: Eq k) => (k -> (v -> (HashMap k v -> HashMap k v)))
```

[Source](/reference-sources/Data.HashMap.cinch) · line 148

<a id="value-686173682d6d61702d69732d656d707479"></a>

## `hash-map-is-empty`

```cinch
hash-map-is-empty :: forall k v . (HashMap k v -> Bool)
```

Named `hash-map-is-empty` because the bare `is-empty` is the
list's (`List a -> Bool`) and there is no overloading to share one
spelling; the module prefix is how the family disambiguates.

[Source](/reference-sources/Data.HashMap.cinch) · line 264

<a id="value-686173682d6d61702d6b657973"></a>

## `hash-map-keys`

```cinch
hash-map-keys :: forall k v . (HashMap k v -> List k)
```

[Source](/reference-sources/Data.HashMap.cinch) · line 233

<a id="value-686173682d6d61702d6c6f6f6b7570"></a>

## `hash-map-lookup`

```cinch
hash-map-lookup :: forall k v . using (h :: Hash k, eq :: Eq k) => (k -> (HashMap k v -> Option v))
```

[Source](/reference-sources/Data.HashMap.cinch) · line 142

<a id="value-686173682d6d61702d6d6170"></a>

## `hash-map-map`

```cinch
hash-map-map :: forall k v w . ((v -> w) -> (HashMap k v -> HashMap k w))
```

Rewrite the values: the keys and their digests are unchanged, so
the spine's shape and cached heights carry over and no `Hash`/`Eq`
is needed (Data.Map's `map`).

[Source](/reference-sources/Data.HashMap.cinch) · line 305

<a id="value-686173682d6d61702d73697a65"></a>

## `hash-map-size`

```cinch
hash-map-size :: forall k v . (HashMap k v -> Nat)
```

The entry count: the buckets' lengths summed over the spine.

[Source](/reference-sources/Data.HashMap.cinch) · line 252

<a id="value-686173682d6d61702d746f2d6c697374"></a>

## `hash-map-to-list`

```cinch
hash-map-to-list :: forall k v . (HashMap k v -> List (k, v))
```

The entries in the buckets' unstable order (see the refusal in the
header): the ordered views live at Data.Map.

[Source](/reference-sources/Data.HashMap.cinch) · line 219

<a id="value-686173682d6d61702d756e696f6e"></a>

## `hash-map-union`

```cinch
hash-map-union :: forall k v . using (h :: Hash k, eq :: Eq k) => (HashMap k v -> (HashMap k v -> HashMap k v))
```

The LEFT map's value wins a shared key (Data.Map's `union`): the
left map's entries are inserted into the right, and
`hash-map-insert` replaces the stored value in place. The listing
that drives the walk is in the buckets' order, which cannot matter
here: the result is a map, and the walk visits each left entry
exactly once.

[Source](/reference-sources/Data.HashMap.cinch) · line 279

<a id="value-686173682d6d61702d756e696f6e2d77697468"></a>

## `hash-map-union-with`

```cinch
hash-map-union-with :: forall k v . using (h :: Hash k, eq :: Eq k) => ((v -> (v -> v)) -> (HashMap k v -> (HashMap k v -> HashMap k v)))
```

The union of `hash-map-union` with a shared key combined instead
of chosen: `f` sees the LEFT value first and the right value
second (Data.Map's `unionWith`).

[Source](/reference-sources/Data.HashMap.cinch) · line 285

<a id="value-686173682d6d61702d76616c756573"></a>

## `hash-map-values`

```cinch
hash-map-values :: forall k v . (HashMap k v -> List v)
```

[Source](/reference-sources/Data.HashMap.cinch) · line 236

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.HashMap.FromMap_HashMap` | `FromMap (HashMap k v) k v` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/HashMap.cinch`. Source SHA-256: `bd0ff3ffdb9066a8a0b7518641b38683a994d8e9ee9538f79e99def3d452a565`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
