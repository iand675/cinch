# Data.Map

Ordered maps with comparison constraints, lookup, insertion, removal, traversal, and collection-literal support.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Map`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Map.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.List](Data.List.md), [Data.Option](Data.Option.md), [Relation.Ord](Relation.Ord.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Map: the ordered map.

An unbalanced binary search tree over `Ord k` (decision D105).
`map-insert` keeps the left subtree strictly smaller and the right
strictly greater, so a repeated key replaces the stored value in
place.

Module scheme (design/base-library.md): the type's home carries its
`FromMap` instance (map literals at `Map`).

</details>

# Types and constructors

<a id="type-4d6170"></a>

## `Map`

```cinch
data Map (k :: Type) (v :: Type)
```

The ordered map: an unbalanced binary search tree over `Ord k`
(decision D105). `map-insert` keeps the left subtree strictly
smaller and the right strictly greater, so a repeated key replaces
the stored value in place.

[Source](/reference-sources/Data.Map.cinch) · line 24

```cinch
Tip :: forall k v . Map k v
```

```cinch
Bin :: forall k v . (k -> (v -> (Map k v -> (Map k v -> Map k v))))
```

# Values

<a id="value-6d61702d64656c657465"></a>

## `map-delete`

```cinch
map-delete :: forall k v . using (ord :: Ord k) => (k -> (Map k v -> Map k v))
```

Removal: the search descends as `map-lookup` does, and the found
node's subtrees rejoin through `map-glue`. An absent key rebuilds
the same tree, so no view can tell a delete of an absent key from
the identity.

[Source](/reference-sources/Data.Map.cinch) · line 60

<a id="value-6d61702d656d707479"></a>

## `map-empty`

```cinch
map-empty :: forall k v . Map k v
```

The empty map, as a name (the family's other homes already spell
theirs `set-empty`/`hash-map-empty`; the bootstrap exposed `Tip`
only through the data declaration).

[Source](/reference-sources/Data.Map.cinch) · line 53

<a id="value-6d61702d66696c746572"></a>

## `map-filter`

```cinch
map-filter :: forall k v . ((k -> (v -> Bool)) -> (Map k v -> Map k v))
```

Keep the entries the predicate accepts; a dropped node's children
rejoin with `map-glue`, so no `Ord` is needed either (Data.Map's
`filterWithKey`). The predicate sees the KEY as well as the value:
the entry, not the bare value, is what this module lists.

[Source](/reference-sources/Data.Map.cinch) · line 165

<a id="value-6d61702d666f6c64"></a>

## `map-fold`

```cinch
map-fold :: forall k v b . ((k -> (v -> (b -> b))) -> (b -> (Map k v -> b)))
```

A RIGHT fold in ascending key order (Data.Map's `foldrWithKey`).
The right direction is the one that agrees with the view: folding
with `Cons` rebuilds `map-to-list` exactly, and the empty map is
the identity. The accumulator's role is the right (second) value.

[Source](/reference-sources/Data.Map.cinch) · line 176

<a id="value-6d61702d66726f6d2d7061697273"></a>

## `map-from-pairs`

```cinch
map-from-pairs :: forall k v . using (ord :: Ord k) => (List (k, v) -> Map k v)
```

[Source](/reference-sources/Data.Map.cinch) · line 44

<a id="value-6d61702d696e73657274"></a>

## `map-insert`

```cinch
map-insert :: forall k v . using (ord :: Ord k) => (k -> (v -> (Map k v -> Map k v)))
```

[Source](/reference-sources/Data.Map.cinch) · line 28

<a id="value-6d61702d69732d656d707479"></a>

## `map-is-empty`

```cinch
map-is-empty :: forall k v . (Map k v -> Bool)
```

Named `map-is-empty` because the bare `is-empty` is the list's
(`List a -> Bool`) and this language has no overloading to share
one spelling across carriers; the module prefix is how the family
disambiguates. (`map-null` was the alternative; `null` is the
`Foldable` class method's name and would suggest that class.)

[Source](/reference-sources/Data.Map.cinch) · line 149

<a id="value-6d61702d6b657973"></a>

## `map-keys`

```cinch
map-keys :: forall k v . (Map k v -> List k)
```

The keys in ascending order.

[Source](/reference-sources/Data.Map.cinch) · line 119

<a id="value-6d61702d6c6f6f6b7570"></a>

## `map-lookup`

```cinch
map-lookup :: forall k v . using (ord :: Ord k) => (k -> (Map k v -> Option v))
```

[Source](/reference-sources/Data.Map.cinch) · line 36

<a id="value-6d61702d6d6170"></a>

## `map-map`

```cinch
map-map :: forall k v w . ((v -> w) -> (Map k v -> Map k w))
```

Rewrite the values, leave the keys where they are. The shape is
preserved, so no `Ord` is needed (Data.Map's `map`).

[Source](/reference-sources/Data.Map.cinch) · line 156

<a id="value-6d61702d73697a65"></a>

## `map-size`

```cinch
map-size :: forall k v . (Map k v -> Nat)
```

The entry count: the O(n) walk (the node carries no size, only the
key/value pair).

[Source](/reference-sources/Data.Map.cinch) · line 139

<a id="value-6d61702d746f2d6c697374"></a>

## `map-to-list`

```cinch
map-to-list :: forall k v . (Map k v -> List (k, v))
```

The entries in ascending key order. The tree IS an ordered search
tree, so this order is observable and stable: the tests pin it
against a known construction, and a caller may rely on it.

[Source](/reference-sources/Data.Map.cinch) · line 110

<a id="value-6d61702d756e696f6e"></a>

## `map-union`

```cinch
map-union :: forall k v . using (ord :: Ord k) => (Map k v -> (Map k v -> Map k v))
```

The LEFT map's value wins a shared key (Data.Map's `union`): the
left map's entries are inserted into the right, and `map-insert`
replaces the stored value in place. Two n-entry maps cost
O(n log n).

[Source](/reference-sources/Data.Map.cinch) · line 84

<a id="value-6d61702d756e696f6e2d77697468"></a>

## `map-union-with`

```cinch
map-union-with :: forall k v . using (ord :: Ord k) => ((v -> (v -> v)) -> (Map k v -> (Map k v -> Map k v)))
```

The union of `map-union` with a shared key combined instead of
chosen: `f` sees the LEFT value first and the right value second
(Data.Map's `unionWith`).

[Source](/reference-sources/Data.Map.cinch) · line 90

<a id="value-6d61702d76616c756573"></a>

## `map-values`

```cinch
map-values :: forall k v . (Map k v -> List v)
```

The values in ascending KEY order (Data.Map's `elems`), never in
insertion order.

[Source](/reference-sources/Data.Map.cinch) · line 129

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Map.FromMap_Map` | `FromMap (Map k v) k v` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Map.cinch`. Source SHA-256: `0fa31267bf3bc94a2d17c9c2f8b454bdf7860c41e164576b81639263d16e0a32`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
