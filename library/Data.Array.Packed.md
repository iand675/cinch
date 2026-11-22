# Data.Array.Packed

Packed scalar constructors for Array. The element format travels with the buffer, so generic reads and updates keep it.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Array.Packed`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Array.Packed.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Array](Data.Array.md), [Data.Core](Data.Core.md).

Non-embedded source closure: [Data.Array](Data.Array.md), [Data.Array.Packed](Data.Array.Packed.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Packed scalar constructors. Result is an ordinary Array: the element
format travels with the buffer, so generic reads and updates preserve it.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-61727261792d66726f6d2d6368617273"></a>

## `array-from-chars`

```cinch
array-from-chars :: (List Char -> Array Char)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 18

<a id="value-61727261792d66726f6d2d66333273"></a>

## `array-from-f32s`

```cinch
array-from-f32s :: (List F32 -> Array F32)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 16

<a id="value-61727261792d66726f6d2d66363473"></a>

## `array-from-f64s`

```cinch
array-from-f64s :: (List F64 -> Array F64)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 17

<a id="value-61727261792d66726f6d2d69313673"></a>

## `array-from-i16s`

```cinch
array-from-i16s :: (List I16 -> Array I16)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 9

<a id="value-61727261792d66726f6d2d69333273"></a>

## `array-from-i32s`

```cinch
array-from-i32s :: (List I32 -> Array I32)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 10

<a id="value-61727261792d66726f6d2d69363473"></a>

## `array-from-i64s`

```cinch
array-from-i64s :: (List I64 -> Array I64)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 11

<a id="value-61727261792d66726f6d2d693873"></a>

## `array-from-i8s`

```cinch
array-from-i8s :: (List I8 -> Array I8)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 8

<a id="value-61727261792d66726f6d2d6973697a6573"></a>

## `array-from-isizes`

```cinch
array-from-isizes :: (List Isize -> Array Isize)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 20

<a id="value-61727261792d66726f6d2d75313673"></a>

## `array-from-u16s`

```cinch
array-from-u16s :: (List U16 -> Array U16)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 13

<a id="value-61727261792d66726f6d2d75333273"></a>

## `array-from-u32s`

```cinch
array-from-u32s :: (List U32 -> Array U32)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 14

<a id="value-61727261792d66726f6d2d75363473"></a>

## `array-from-u64s`

```cinch
array-from-u64s :: (List U64 -> Array U64)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 15

<a id="value-61727261792d66726f6d2d753873"></a>

## `array-from-u8s`

```cinch
array-from-u8s :: (List U8 -> Array U8)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 12

<a id="value-61727261792d66726f6d2d7573697a6573"></a>

## `array-from-usizes`

```cinch
array-from-usizes :: (List Usize -> Array Usize)
```

[Source](/reference-sources/Data.Array.Packed.cinch) · line 19

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Array/Packed.cinch`. Source SHA-256: `799f936d2712685f6ade99dd777c79c7c039c1e7ca28b8988700fde0f104b364`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
