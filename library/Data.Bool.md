# Data.Bool

Boolean operations and their comparison instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Bool`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bool.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Hash](Data.Hash.md), [Text.Codec](Text.Codec.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Bool: the booleans and their strict operations.

`Bool` is a core runtime type; the `cinch.prim.*Bool` foreign
imports are the machine operations. `&&` and `||` are strict
functions in the bootstrap slice; the surface short-circuiting form
lands with the effects correction (primitive types specification
section 6).

Module scheme (design/base-library.md): the type's home carries its
instance family, including the `Text.Codec` instance (the A6
codec generator's BOOLEAN column is exactly one byte, 0 or 1).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-2626"></a>

## `&&`

```cinch
&& :: (Bool -> (Bool -> Bool))
```

These are strict functions in the bootstrap slice; the surface `&&`
and `||` short-circuit once that deviation is corrected (primitive
types specification, section 6).

[Source](/reference-sources/Data.Bool.cinch) · line 32

<a id="value-616e64"></a>

## `and`

```cinch
and :: (Bool -> (Bool -> Bool))
```

The strict boolean algebra (primitive types specification, section 6).

[Source](/reference-sources/Data.Bool.cinch) · line 42

<a id="value-626f6f6c"></a>

## `bool`

```cinch
bool :: forall a . (a -> (a -> (Bool -> a)))
```

The boolean fold: `bool f t b` is `t` when `b` is `True` and `f`
otherwise (`Data.Bool.bool`), the elimination form matching the
constructors' order.

[Source](/reference-sources/Data.Bool.cinch) · line 54

<a id="value-6e6f74"></a>

## `not`

```cinch
not :: (Bool -> Bool)
```

[Source](/reference-sources/Data.Bool.cinch) · line 38

<a id="value-6f72"></a>

## `or`

```cinch
or :: (Bool -> (Bool -> Bool))
```

[Source](/reference-sources/Data.Bool.cinch) · line 45

<a id="value-7072696d2d616e642d626f6f6c"></a>

## `prim-and-bool`

```cinch
prim-and-bool :: (Bool -> (Bool -> Bool))
```

[Source](/reference-sources/Data.Bool.cinch) · line 26

<a id="value-7072696d2d65712d626f6f6c"></a>

## `prim-eq-bool`

```cinch
prim-eq-bool :: (Bool -> (Bool -> Bool))
```

[Source](/reference-sources/Data.Bool.cinch) · line 23

<a id="value-7072696d2d6e652d626f6f6c"></a>

## `prim-ne-bool`

```cinch
prim-ne-bool :: (Bool -> (Bool -> Bool))
```

[Source](/reference-sources/Data.Bool.cinch) · line 24

<a id="value-7072696d2d6e6f742d626f6f6c"></a>

## `prim-not-bool`

```cinch
prim-not-bool :: (Bool -> Bool)
```

[Source](/reference-sources/Data.Bool.cinch) · line 25

<a id="value-7072696d2d6f722d626f6f6c"></a>

## `prim-or-bool`

```cinch
prim-or-bool :: (Bool -> (Bool -> Bool))
```

[Source](/reference-sources/Data.Bool.cinch) · line 27

<a id="value-786f72"></a>

## `xor`

```cinch
xor :: (Bool -> (Bool -> Bool))
```

[Source](/reference-sources/Data.Bool.cinch) · line 48

<a id="value-7c7c"></a>

## `||`

```cinch
|| :: (Bool -> (Bool -> Bool))
```

[Source](/reference-sources/Data.Bool.cinch) · line 35

# Operator fixities

```cinch
infixr 2 &&
infixr 1 ||
```

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Bool.Codec_Bool` | `Codec Bool` | Implicit |
| `Data.Bool.Eq_Bool` | `Eq Bool` | Implicit |
| `Data.Bool.Hash_Bool` | `Hash Bool` | Implicit |
| `Data.Bool.JsonCodec_Bool` | `JsonCodec Bool` | Implicit |
| `Data.Bool.Ord_Bool` | `Ord Bool` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Bool.cinch`. Source SHA-256: `b5d776700b7c67f7ad33736bfa1d78edc8d8e94070852c59b22175b637fd81f5`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
