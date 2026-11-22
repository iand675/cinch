# Data.Array

Immutable contiguous arrays with optional failed reads, updates, slices, and ranges, plus list conversions.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Array`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Array.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.FromList](Data.FromList.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.Array](Data.Array.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Immutable contiguous arrays. Failed reads/updates/ranges return None.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-61727261792d617070656e64"></a>

## `array-append`

```cinch
array-append :: forall a . (Array a -> (Array a -> Array a))
```

[Source](/reference-sources/Data.Array.cinch) · line 19

<a id="value-61727261792d66726f6d2d6c697374"></a>

## `array-from-list`

```cinch
array-from-list :: forall a . (List a -> Array a)
```

[Source](/reference-sources/Data.Array.cinch) · line 13

<a id="value-61727261792d676574"></a>

## `array-get`

```cinch
array-get :: forall a . (Array a -> (Nat -> Option a))
```

[Source](/reference-sources/Data.Array.cinch) · line 16

<a id="value-61727261792d6c656e677468"></a>

## `array-length`

```cinch
array-length :: forall a . (Array a -> Nat)
```

[Source](/reference-sources/Data.Array.cinch) · line 15

<a id="value-61727261792d7265706c6963617465"></a>

## `array-replicate`

```cinch
array-replicate :: forall a . (Nat -> (a -> Array a))
```

[Source](/reference-sources/Data.Array.cinch) · line 14

<a id="value-61727261792d736574"></a>

## `array-set`

```cinch
array-set :: forall a . (Array a -> (Nat -> (a -> Option (Array a))))
```

[Source](/reference-sources/Data.Array.cinch) · line 17

<a id="value-61727261792d736c696365"></a>

## `array-slice`

```cinch
array-slice :: forall a . (Array a -> (Nat -> (Nat -> Option (Array a))))
```

[Source](/reference-sources/Data.Array.cinch) · line 18

<a id="value-61727261792d746162756c617465"></a>

## `array-tabulate`

```cinch
array-tabulate :: forall a . (Nat -> ((Nat -> a) -> Array a))
```

[Source](/reference-sources/Data.Array.cinch) · line 21

<a id="value-61727261792d746f2d6c697374"></a>

## `array-to-list`

```cinch
array-to-list :: forall a . (Array a -> List a)
```

[Source](/reference-sources/Data.Array.cinch) · line 27

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Array.FromList_Array` | `FromList (Array a) a` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Array.cinch`. Source SHA-256: `3c90379a58326e1c86b3b5c0a6fbdbbb95374acc26ab0adca138633f7caa87ee`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
