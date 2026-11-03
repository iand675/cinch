# Relation.Ord

Ordering vocabulary and comparison helpers. State all required dictionaries explicitly; superclass entailment is not implicit.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Relation.Ord`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Relation.Ord.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Ordering](Data.Ordering.md), [Relation.Eq](Relation.Eq.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Relation.Ord: value-level ordering.

The `Ord` class of the base library scheme (design/base-library.md).
The order is numeric on the integer types and code point order on
`Char` (primitive types specification section 8); `F32`/`F64` have
no `Eq`/`Ord` instances. A class constraint entails nothing: `Ord`
does not extend `Eq`. Instances live with their type.

`comparing` and `Down` are the keyed-comparison vocabulary. The
`Ord` methods are the four booleans, so the class never returns
`Ordering`; `comparing` assembles the three-way result from those
booleans. `Down` is the order-reversing newtype, and its instances
live here with the type.

</details>

# Types and constructors

<a id="type-446f776e"></a>

## `Down`

```cinch
data Down (a :: Type)
```

The order-reversing newtype: `Down x` is `x` with the order
swapped, so `comparing (\p -> Down (key p))` sorts by `key`
descending. Equality is unaffected by the flip (only the four
ordering methods reverse).

[Source](/reference-sources/Relation.Ord.cinch) · line 46

```cinch
Down :: forall a . (a -> Down a)
```

The order-reversing newtype: `Down x` is `x` with the order
swapped, so `comparing (\p -> Down (key p))` sorts by `key`
descending. Equality is unaffected by the flip (only the four
ordering methods reverse).

# Signatures and methods

<a id="class-4f7264"></a>

## `Ord`

```cinch
signature Ord a
```

[Source](/reference-sources/Relation.Ord.cinch) · line 25

<a id="method-4f72642e3c"></a>

## `Ord.<`

```cinch
< :: forall a . using (dict :: Ord a) => (a -> (a -> Bool))
```

[Source](/reference-sources/Relation.Ord.cinch) · line 26

<a id="method-4f72642e3c3d"></a>

## `Ord.<=`

```cinch
<= :: forall a . using (dict :: Ord a) => (a -> (a -> Bool))
```

[Source](/reference-sources/Relation.Ord.cinch) · line 27

<a id="method-4f72642e3e"></a>

## `Ord.>`

```cinch
> :: forall a . using (dict :: Ord a) => (a -> (a -> Bool))
```

[Source](/reference-sources/Relation.Ord.cinch) · line 28

<a id="method-4f72642e3e3d"></a>

## `Ord.>=`

```cinch
>= :: forall a . using (dict :: Ord a) => (a -> (a -> Bool))
```

[Source](/reference-sources/Relation.Ord.cinch) · line 29

# Values

<a id="value-636f6d706172696e67"></a>

## `comparing`

```cinch
comparing :: forall a b . using (ord :: Ord b) => ((a -> b) -> (a -> (a -> Ordering)))
```

Keyed comparison: `comparing k x y` orders `x` and `y` by the
projections of `k`. The `Ord` class carries only the four boolean
methods, so the three-way result is assembled here: `LT` when the
key is `<`, `GT` when it is `>`, `EQ` otherwise.

[Source](/reference-sources/Relation.Ord.cinch) · line 35

# Operator fixities

```cinch
infix 3 <
infix 3 <=
infix 3 >
infix 3 >=
```

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Relation.Ord.Eq_Down` | `Eq (Down a)` | Implicit |
| `Relation.Ord.Ord_Down` | `Ord (Down a)` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Relation/Ord.cinch`. Source SHA-256: `7e0ea9784097052c8be5dd9a30151adeb1d05d4923878facb09d983e1777edfd`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
