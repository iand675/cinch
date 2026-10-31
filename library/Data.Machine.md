# Data.Machine

Fixed-width and target-sized scalar operations. Names make checked, wrapping, or saturating overflow policy explicit.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Machine`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Machine.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Text.Codec](Text.Codec.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Machine: the machine scalar types and their operations.

The machine scalar types follow the primitive types specification:
`I8`-`I64`, `U8`-`U64`, `Usize`, `Isize`, `F32` and `F64` are core
runtime types. Names follow decision D6: an operation is a prelude
name of the form `<op><Type>` (`wrapping-add-u32`) and a conversion
is `<src>To<dst>` (`u8-to-i32`). The `checked*`, `saturating*` and
`wrapping*` conversion families capitalize the source type
(`checked-u8-to-i64`); the total family lowercases it (`u8-to-i32`).
The specification's dotted `U32.wrappingAdd` spelling is a
presentation form (the presentation aliases in the checker).

Overflow behavior is never implicit: `checked` returns Option and
never the wrapped value, `wrapping` is two's complement wrap,
`saturating` clamps to the bounds. There is no Add/Sub/Mul/Neg
instance for machine types: a class method has a single result type
and cannot express which family is meant (primitive types
specification section 8). `F32` and `F64` have no Eq/Ord instances.

Module scheme (design/base-library.md): this is the machine
scalars' vocabulary home (the scheme names the mathematical types;
the machine family groups here), carrying its instance family
(Eq/Ord per integer type, `Bits` over the integers, and the `I64`
`Text.Codec` instance that matches the A6 codec generator's INTEGER
column: 8 bytes little-endian).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-6162732d663332"></a>

## `abs-f32`

```cinch
abs-f32 :: (F32 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 450

<a id="value-6162732d663634"></a>

## `abs-f64`

```cinch
abs-f64 :: (F64 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 473

<a id="value-6164642d663332"></a>

## `add-f32`

```cinch
add-f32 :: (F32 -> (F32 -> F32))
```

Primitive types specification, section 5: IEEE 754 binary32 and
binary64 arithmetic with round-to-nearest ties-to-even. Floats have
no Eq and no Ord instance (`==` and `<` on F64 are rejected); the
comparisons below are the quiet, non-signaling IEEE ones.

F32.

[Source](/reference-sources/Data.Machine.cinch) · line 445

<a id="value-6164642d663634"></a>

## `add-f64`

```cinch
add-f64 :: (F64 -> (F64 -> F64))
```

F64.

[Source](/reference-sources/Data.Machine.cinch) · line 468

<a id="value-6269742d616e642d693136"></a>

## `bit-and-i16`

```cinch
bit-and-i16 :: (I16 -> (I16 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 111

<a id="value-6269742d616e642d693332"></a>

## `bit-and-i32`

```cinch
bit-and-i32 :: (I32 -> (I32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 150

<a id="value-6269742d616e642d693634"></a>

## `bit-and-i64`

```cinch
bit-and-i64 :: (I64 -> (I64 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 189

<a id="value-6269742d616e642d6938"></a>

## `bit-and-i8`

```cinch
bit-and-i8 :: (I8 -> (I8 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 72

<a id="value-6269742d616e642d6973697a65"></a>

## `bit-and-isize`

```cinch
bit-and-isize :: (Isize -> (Isize -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 423

<a id="value-6269742d616e642d753136"></a>

## `bit-and-u16`

```cinch
bit-and-u16 :: (U16 -> (U16 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 267

<a id="value-6269742d616e642d753332"></a>

## `bit-and-u32`

```cinch
bit-and-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 306

<a id="value-6269742d616e642d753634"></a>

## `bit-and-u64`

```cinch
bit-and-u64 :: (U64 -> (U64 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 345

<a id="value-6269742d616e642d7538"></a>

## `bit-and-u8`

```cinch
bit-and-u8 :: (U8 -> (U8 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 228

<a id="value-6269742d616e642d7573697a65"></a>

## `bit-and-usize`

```cinch
bit-and-usize :: (Usize -> (Usize -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 384

<a id="value-6269742d6e6f742d693136"></a>

## `bit-not-i16`

```cinch
bit-not-i16 :: (I16 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 114

<a id="value-6269742d6e6f742d693332"></a>

## `bit-not-i32`

```cinch
bit-not-i32 :: (I32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 153

<a id="value-6269742d6e6f742d693634"></a>

## `bit-not-i64`

```cinch
bit-not-i64 :: (I64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 192

<a id="value-6269742d6e6f742d6938"></a>

## `bit-not-i8`

```cinch
bit-not-i8 :: (I8 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 75

<a id="value-6269742d6e6f742d6973697a65"></a>

## `bit-not-isize`

```cinch
bit-not-isize :: (Isize -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 426

<a id="value-6269742d6e6f742d753136"></a>

## `bit-not-u16`

```cinch
bit-not-u16 :: (U16 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 270

<a id="value-6269742d6e6f742d753332"></a>

## `bit-not-u32`

```cinch
bit-not-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 309

<a id="value-6269742d6e6f742d753634"></a>

## `bit-not-u64`

```cinch
bit-not-u64 :: (U64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 348

<a id="value-6269742d6e6f742d7538"></a>

## `bit-not-u8`

```cinch
bit-not-u8 :: (U8 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 231

<a id="value-6269742d6e6f742d7573697a65"></a>

## `bit-not-usize`

```cinch
bit-not-usize :: (Usize -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 387

<a id="value-6269742d6f722d693136"></a>

## `bit-or-i16`

```cinch
bit-or-i16 :: (I16 -> (I16 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 112

<a id="value-6269742d6f722d693332"></a>

## `bit-or-i32`

```cinch
bit-or-i32 :: (I32 -> (I32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 151

<a id="value-6269742d6f722d693634"></a>

## `bit-or-i64`

```cinch
bit-or-i64 :: (I64 -> (I64 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 190

<a id="value-6269742d6f722d6938"></a>

## `bit-or-i8`

```cinch
bit-or-i8 :: (I8 -> (I8 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 73

<a id="value-6269742d6f722d6973697a65"></a>

## `bit-or-isize`

```cinch
bit-or-isize :: (Isize -> (Isize -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 424

<a id="value-6269742d6f722d753136"></a>

## `bit-or-u16`

```cinch
bit-or-u16 :: (U16 -> (U16 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 268

<a id="value-6269742d6f722d753332"></a>

## `bit-or-u32`

```cinch
bit-or-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 307

<a id="value-6269742d6f722d753634"></a>

## `bit-or-u64`

```cinch
bit-or-u64 :: (U64 -> (U64 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 346

<a id="value-6269742d6f722d7538"></a>

## `bit-or-u8`

```cinch
bit-or-u8 :: (U8 -> (U8 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 229

<a id="value-6269742d6f722d7573697a65"></a>

## `bit-or-usize`

```cinch
bit-or-usize :: (Usize -> (Usize -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 385

<a id="value-6269742d786f722d693136"></a>

## `bit-xor-i16`

```cinch
bit-xor-i16 :: (I16 -> (I16 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 113

<a id="value-6269742d786f722d693332"></a>

## `bit-xor-i32`

```cinch
bit-xor-i32 :: (I32 -> (I32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 152

<a id="value-6269742d786f722d693634"></a>

## `bit-xor-i64`

```cinch
bit-xor-i64 :: (I64 -> (I64 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 191

<a id="value-6269742d786f722d6938"></a>

## `bit-xor-i8`

```cinch
bit-xor-i8 :: (I8 -> (I8 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 74

<a id="value-6269742d786f722d6973697a65"></a>

## `bit-xor-isize`

```cinch
bit-xor-isize :: (Isize -> (Isize -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 425

<a id="value-6269742d786f722d753136"></a>

## `bit-xor-u16`

```cinch
bit-xor-u16 :: (U16 -> (U16 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 269

<a id="value-6269742d786f722d753332"></a>

## `bit-xor-u32`

```cinch
bit-xor-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 308

<a id="value-6269742d786f722d753634"></a>

## `bit-xor-u64`

```cinch
bit-xor-u64 :: (U64 -> (U64 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 347

<a id="value-6269742d786f722d7538"></a>

## `bit-xor-u8`

```cinch
bit-xor-u8 :: (U8 -> (U8 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 230

<a id="value-6269742d786f722d7573697a65"></a>

## `bit-xor-usize`

```cinch
bit-xor-usize :: (Usize -> (Usize -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 386

<a id="value-636865636b65642d6164642d693136"></a>

## `checked-add-i16`

```cinch
checked-add-i16 :: (I16 -> (I16 -> Option I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 91

<a id="value-636865636b65642d6164642d693332"></a>

## `checked-add-i32`

```cinch
checked-add-i32 :: (I32 -> (I32 -> Option I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 130

<a id="value-636865636b65642d6164642d693634"></a>

## `checked-add-i64`

```cinch
checked-add-i64 :: (I64 -> (I64 -> Option I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 169

<a id="value-636865636b65642d6164642d6938"></a>

## `checked-add-i8`

```cinch
checked-add-i8 :: (I8 -> (I8 -> Option I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 52

<a id="value-636865636b65642d6164642d6973697a65"></a>

## `checked-add-isize`

```cinch
checked-add-isize :: (Isize -> (Isize -> Option Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 403

<a id="value-636865636b65642d6164642d753136"></a>

## `checked-add-u16`

```cinch
checked-add-u16 :: (U16 -> (U16 -> Option U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 247

<a id="value-636865636b65642d6164642d753332"></a>

## `checked-add-u32`

```cinch
checked-add-u32 :: (U32 -> (U32 -> Option U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 286

<a id="value-636865636b65642d6164642d753634"></a>

## `checked-add-u64`

```cinch
checked-add-u64 :: (U64 -> (U64 -> Option U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 325

<a id="value-636865636b65642d6164642d7538"></a>

## `checked-add-u8`

```cinch
checked-add-u8 :: (U8 -> (U8 -> Option U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 208

<a id="value-636865636b65642d6164642d7573697a65"></a>

## `checked-add-usize`

```cinch
checked-add-usize :: (Usize -> (Usize -> Option Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 364

<a id="value-636865636b65642d6469762d693136"></a>

## `checked-div-i16`

```cinch
checked-div-i16 :: (I16 -> (I16 -> Option I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 103

<a id="value-636865636b65642d6469762d693332"></a>

## `checked-div-i32`

```cinch
checked-div-i32 :: (I32 -> (I32 -> Option I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 142

<a id="value-636865636b65642d6469762d693634"></a>

## `checked-div-i64`

```cinch
checked-div-i64 :: (I64 -> (I64 -> Option I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 181

<a id="value-636865636b65642d6469762d6938"></a>

## `checked-div-i8`

```cinch
checked-div-i8 :: (I8 -> (I8 -> Option I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 64

<a id="value-636865636b65642d6469762d6973697a65"></a>

## `checked-div-isize`

```cinch
checked-div-isize :: (Isize -> (Isize -> Option Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 415

<a id="value-636865636b65642d6469762d753136"></a>

## `checked-div-u16`

```cinch
checked-div-u16 :: (U16 -> (U16 -> Option U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 259

<a id="value-636865636b65642d6469762d753332"></a>

## `checked-div-u32`

```cinch
checked-div-u32 :: (U32 -> (U32 -> Option U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 298

<a id="value-636865636b65642d6469762d753634"></a>

## `checked-div-u64`

```cinch
checked-div-u64 :: (U64 -> (U64 -> Option U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 337

<a id="value-636865636b65642d6469762d7538"></a>

## `checked-div-u8`

```cinch
checked-div-u8 :: (U8 -> (U8 -> Option U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 220

<a id="value-636865636b65642d6469762d7573697a65"></a>

## `checked-div-usize`

```cinch
checked-div-usize :: (Usize -> (Usize -> Option Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 376

<a id="value-636865636b65642d6633322d746f2d693136"></a>

## `checked-f32-to-i16`

```cinch
checked-f32-to-i16 :: (F32 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 874

<a id="value-636865636b65642d6633322d746f2d693332"></a>

## `checked-f32-to-i32`

```cinch
checked-f32-to-i32 :: (F32 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 876

<a id="value-636865636b65642d6633322d746f2d693634"></a>

## `checked-f32-to-i64`

```cinch
checked-f32-to-i64 :: (F32 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 878

<a id="value-636865636b65642d6633322d746f2d6938"></a>

## `checked-f32-to-i8`

```cinch
checked-f32-to-i8 :: (F32 -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 872

<a id="value-636865636b65642d6633322d746f2d6973697a65"></a>

## `checked-f32-to-isize`

```cinch
checked-f32-to-isize :: (F32 -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 890

<a id="value-636865636b65642d6633322d746f2d753136"></a>

## `checked-f32-to-u16`

```cinch
checked-f32-to-u16 :: (F32 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 882

<a id="value-636865636b65642d6633322d746f2d753332"></a>

## `checked-f32-to-u32`

```cinch
checked-f32-to-u32 :: (F32 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 884

<a id="value-636865636b65642d6633322d746f2d753634"></a>

## `checked-f32-to-u64`

```cinch
checked-f32-to-u64 :: (F32 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 886

<a id="value-636865636b65642d6633322d746f2d7538"></a>

## `checked-f32-to-u8`

```cinch
checked-f32-to-u8 :: (F32 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 880

<a id="value-636865636b65642d6633322d746f2d7573697a65"></a>

## `checked-f32-to-usize`

```cinch
checked-f32-to-usize :: (F32 -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 888

<a id="value-636865636b65642d6636342d746f2d693136"></a>

## `checked-f64-to-i16`

```cinch
checked-f64-to-i16 :: (F64 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 894

<a id="value-636865636b65642d6636342d746f2d693332"></a>

## `checked-f64-to-i32`

```cinch
checked-f64-to-i32 :: (F64 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 896

<a id="value-636865636b65642d6636342d746f2d693634"></a>

## `checked-f64-to-i64`

```cinch
checked-f64-to-i64 :: (F64 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 898

<a id="value-636865636b65642d6636342d746f2d6938"></a>

## `checked-f64-to-i8`

```cinch
checked-f64-to-i8 :: (F64 -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 892

<a id="value-636865636b65642d6636342d746f2d6973697a65"></a>

## `checked-f64-to-isize`

```cinch
checked-f64-to-isize :: (F64 -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 910

<a id="value-636865636b65642d6636342d746f2d753136"></a>

## `checked-f64-to-u16`

```cinch
checked-f64-to-u16 :: (F64 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 902

<a id="value-636865636b65642d6636342d746f2d753332"></a>

## `checked-f64-to-u32`

```cinch
checked-f64-to-u32 :: (F64 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 904

<a id="value-636865636b65642d6636342d746f2d753634"></a>

## `checked-f64-to-u64`

```cinch
checked-f64-to-u64 :: (F64 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 906

<a id="value-636865636b65642d6636342d746f2d7538"></a>

## `checked-f64-to-u8`

```cinch
checked-f64-to-u8 :: (F64 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 900

<a id="value-636865636b65642d6636342d746f2d7573697a65"></a>

## `checked-f64-to-usize`

```cinch
checked-f64-to-usize :: (F64 -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 908

<a id="value-636865636b65642d6931362d746f2d693136"></a>

## `checked-i16-to-i16`

```cinch
checked-i16-to-i16 :: (I16 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 605

<a id="value-636865636b65642d6931362d746f2d693332"></a>

## `checked-i16-to-i32`

```cinch
checked-i16-to-i32 :: (I16 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 608

<a id="value-636865636b65642d6931362d746f2d693634"></a>

## `checked-i16-to-i64`

```cinch
checked-i16-to-i64 :: (I16 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 611

<a id="value-636865636b65642d6931362d746f2d6938"></a>

## `checked-i16-to-i8`

```cinch
checked-i16-to-i8 :: (I16 -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 602

<a id="value-636865636b65642d6931362d746f2d6973697a65"></a>

## `checked-i16-to-isize`

```cinch
checked-i16-to-isize :: (I16 -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 629

<a id="value-636865636b65642d6931362d746f2d753136"></a>

## `checked-i16-to-u16`

```cinch
checked-i16-to-u16 :: (I16 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 617

<a id="value-636865636b65642d6931362d746f2d753332"></a>

## `checked-i16-to-u32`

```cinch
checked-i16-to-u32 :: (I16 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 620

<a id="value-636865636b65642d6931362d746f2d753634"></a>

## `checked-i16-to-u64`

```cinch
checked-i16-to-u64 :: (I16 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 623

<a id="value-636865636b65642d6931362d746f2d7538"></a>

## `checked-i16-to-u8`

```cinch
checked-i16-to-u8 :: (I16 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 614

<a id="value-636865636b65642d6931362d746f2d7573697a65"></a>

## `checked-i16-to-usize`

```cinch
checked-i16-to-usize :: (I16 -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 626

<a id="value-636865636b65642d6933322d746f2d693136"></a>

## `checked-i32-to-i16`

```cinch
checked-i32-to-i16 :: (I32 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 635

<a id="value-636865636b65642d6933322d746f2d693332"></a>

## `checked-i32-to-i32`

```cinch
checked-i32-to-i32 :: (I32 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 638

<a id="value-636865636b65642d6933322d746f2d693634"></a>

## `checked-i32-to-i64`

```cinch
checked-i32-to-i64 :: (I32 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 641

<a id="value-636865636b65642d6933322d746f2d6938"></a>

## `checked-i32-to-i8`

```cinch
checked-i32-to-i8 :: (I32 -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 632

<a id="value-636865636b65642d6933322d746f2d6973697a65"></a>

## `checked-i32-to-isize`

```cinch
checked-i32-to-isize :: (I32 -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 659

<a id="value-636865636b65642d6933322d746f2d753136"></a>

## `checked-i32-to-u16`

```cinch
checked-i32-to-u16 :: (I32 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 647

<a id="value-636865636b65642d6933322d746f2d753332"></a>

## `checked-i32-to-u32`

```cinch
checked-i32-to-u32 :: (I32 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 650

<a id="value-636865636b65642d6933322d746f2d753634"></a>

## `checked-i32-to-u64`

```cinch
checked-i32-to-u64 :: (I32 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 653

<a id="value-636865636b65642d6933322d746f2d7538"></a>

## `checked-i32-to-u8`

```cinch
checked-i32-to-u8 :: (I32 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 644

<a id="value-636865636b65642d6933322d746f2d7573697a65"></a>

## `checked-i32-to-usize`

```cinch
checked-i32-to-usize :: (I32 -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 656

<a id="value-636865636b65642d6936342d746f2d693136"></a>

## `checked-i64-to-i16`

```cinch
checked-i64-to-i16 :: (I64 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 665

<a id="value-636865636b65642d6936342d746f2d693332"></a>

## `checked-i64-to-i32`

```cinch
checked-i64-to-i32 :: (I64 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 668

<a id="value-636865636b65642d6936342d746f2d693634"></a>

## `checked-i64-to-i64`

```cinch
checked-i64-to-i64 :: (I64 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 671

<a id="value-636865636b65642d6936342d746f2d6938"></a>

## `checked-i64-to-i8`

```cinch
checked-i64-to-i8 :: (I64 -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 662

<a id="value-636865636b65642d6936342d746f2d6973697a65"></a>

## `checked-i64-to-isize`

```cinch
checked-i64-to-isize :: (I64 -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 689

<a id="value-636865636b65642d6936342d746f2d753136"></a>

## `checked-i64-to-u16`

```cinch
checked-i64-to-u16 :: (I64 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 677

<a id="value-636865636b65642d6936342d746f2d753332"></a>

## `checked-i64-to-u32`

```cinch
checked-i64-to-u32 :: (I64 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 680

<a id="value-636865636b65642d6936342d746f2d753634"></a>

## `checked-i64-to-u64`

```cinch
checked-i64-to-u64 :: (I64 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 683

<a id="value-636865636b65642d6936342d746f2d7538"></a>

## `checked-i64-to-u8`

```cinch
checked-i64-to-u8 :: (I64 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 674

<a id="value-636865636b65642d6936342d746f2d7573697a65"></a>

## `checked-i64-to-usize`

```cinch
checked-i64-to-usize :: (I64 -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 686

<a id="value-636865636b65642d69382d746f2d693136"></a>

## `checked-i8-to-i16`

```cinch
checked-i8-to-i16 :: (I8 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 575

<a id="value-636865636b65642d69382d746f2d693332"></a>

## `checked-i8-to-i32`

```cinch
checked-i8-to-i32 :: (I8 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 578

<a id="value-636865636b65642d69382d746f2d693634"></a>

## `checked-i8-to-i64`

```cinch
checked-i8-to-i64 :: (I8 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 581

<a id="value-636865636b65642d69382d746f2d6938"></a>

## `checked-i8-to-i8`

```cinch
checked-i8-to-i8 :: (I8 -> Option I8)
```

Checked, saturating and wrapping integer conversions; float to
integer is checked or saturating (it is never total).

[Source](/reference-sources/Data.Machine.cinch) · line 572

<a id="value-636865636b65642d69382d746f2d6973697a65"></a>

## `checked-i8-to-isize`

```cinch
checked-i8-to-isize :: (I8 -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 599

<a id="value-636865636b65642d69382d746f2d753136"></a>

## `checked-i8-to-u16`

```cinch
checked-i8-to-u16 :: (I8 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 587

<a id="value-636865636b65642d69382d746f2d753332"></a>

## `checked-i8-to-u32`

```cinch
checked-i8-to-u32 :: (I8 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 590

<a id="value-636865636b65642d69382d746f2d753634"></a>

## `checked-i8-to-u64`

```cinch
checked-i8-to-u64 :: (I8 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 593

<a id="value-636865636b65642d69382d746f2d7538"></a>

## `checked-i8-to-u8`

```cinch
checked-i8-to-u8 :: (I8 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 584

<a id="value-636865636b65642d69382d746f2d7573697a65"></a>

## `checked-i8-to-usize`

```cinch
checked-i8-to-usize :: (I8 -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 596

<a id="value-636865636b65642d6973697a652d746f2d693136"></a>

## `checked-isize-to-i16`

```cinch
checked-isize-to-i16 :: (Isize -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 845

<a id="value-636865636b65642d6973697a652d746f2d693332"></a>

## `checked-isize-to-i32`

```cinch
checked-isize-to-i32 :: (Isize -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 848

<a id="value-636865636b65642d6973697a652d746f2d693634"></a>

## `checked-isize-to-i64`

```cinch
checked-isize-to-i64 :: (Isize -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 851

<a id="value-636865636b65642d6973697a652d746f2d6938"></a>

## `checked-isize-to-i8`

```cinch
checked-isize-to-i8 :: (Isize -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 842

<a id="value-636865636b65642d6973697a652d746f2d6973697a65"></a>

## `checked-isize-to-isize`

```cinch
checked-isize-to-isize :: (Isize -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 869

<a id="value-636865636b65642d6973697a652d746f2d753136"></a>

## `checked-isize-to-u16`

```cinch
checked-isize-to-u16 :: (Isize -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 857

<a id="value-636865636b65642d6973697a652d746f2d753332"></a>

## `checked-isize-to-u32`

```cinch
checked-isize-to-u32 :: (Isize -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 860

<a id="value-636865636b65642d6973697a652d746f2d753634"></a>

## `checked-isize-to-u64`

```cinch
checked-isize-to-u64 :: (Isize -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 863

<a id="value-636865636b65642d6973697a652d746f2d7538"></a>

## `checked-isize-to-u8`

```cinch
checked-isize-to-u8 :: (Isize -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 854

<a id="value-636865636b65642d6973697a652d746f2d7573697a65"></a>

## `checked-isize-to-usize`

```cinch
checked-isize-to-usize :: (Isize -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 866

<a id="value-636865636b65642d6d756c2d693136"></a>

## `checked-mul-i16`

```cinch
checked-mul-i16 :: (I16 -> (I16 -> Option I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 93

<a id="value-636865636b65642d6d756c2d693332"></a>

## `checked-mul-i32`

```cinch
checked-mul-i32 :: (I32 -> (I32 -> Option I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 132

<a id="value-636865636b65642d6d756c2d693634"></a>

## `checked-mul-i64`

```cinch
checked-mul-i64 :: (I64 -> (I64 -> Option I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 171

<a id="value-636865636b65642d6d756c2d6938"></a>

## `checked-mul-i8`

```cinch
checked-mul-i8 :: (I8 -> (I8 -> Option I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 54

<a id="value-636865636b65642d6d756c2d6973697a65"></a>

## `checked-mul-isize`

```cinch
checked-mul-isize :: (Isize -> (Isize -> Option Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 405

<a id="value-636865636b65642d6d756c2d753136"></a>

## `checked-mul-u16`

```cinch
checked-mul-u16 :: (U16 -> (U16 -> Option U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 249

<a id="value-636865636b65642d6d756c2d753332"></a>

## `checked-mul-u32`

```cinch
checked-mul-u32 :: (U32 -> (U32 -> Option U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 288

<a id="value-636865636b65642d6d756c2d753634"></a>

## `checked-mul-u64`

```cinch
checked-mul-u64 :: (U64 -> (U64 -> Option U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 327

<a id="value-636865636b65642d6d756c2d7538"></a>

## `checked-mul-u8`

```cinch
checked-mul-u8 :: (U8 -> (U8 -> Option U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 210

<a id="value-636865636b65642d6d756c2d7573697a65"></a>

## `checked-mul-usize`

```cinch
checked-mul-usize :: (Usize -> (Usize -> Option Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 366

<a id="value-636865636b65642d6e65672d693136"></a>

## `checked-neg-i16`

```cinch
checked-neg-i16 :: (I16 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 100

<a id="value-636865636b65642d6e65672d693332"></a>

## `checked-neg-i32`

```cinch
checked-neg-i32 :: (I32 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 139

<a id="value-636865636b65642d6e65672d693634"></a>

## `checked-neg-i64`

```cinch
checked-neg-i64 :: (I64 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 178

<a id="value-636865636b65642d6e65672d6938"></a>

## `checked-neg-i8`

```cinch
checked-neg-i8 :: (I8 -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 61

<a id="value-636865636b65642d6e65672d6973697a65"></a>

## `checked-neg-isize`

```cinch
checked-neg-isize :: (Isize -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 412

<a id="value-636865636b65642d6e65672d753136"></a>

## `checked-neg-u16`

```cinch
checked-neg-u16 :: (U16 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 256

<a id="value-636865636b65642d6e65672d753332"></a>

## `checked-neg-u32`

```cinch
checked-neg-u32 :: (U32 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 295

<a id="value-636865636b65642d6e65672d753634"></a>

## `checked-neg-u64`

```cinch
checked-neg-u64 :: (U64 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 334

<a id="value-636865636b65642d6e65672d7538"></a>

## `checked-neg-u8`

```cinch
checked-neg-u8 :: (U8 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 217

<a id="value-636865636b65642d6e65672d7573697a65"></a>

## `checked-neg-usize`

```cinch
checked-neg-usize :: (Usize -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 373

<a id="value-636865636b65642d72656d2d693136"></a>

## `checked-rem-i16`

```cinch
checked-rem-i16 :: (I16 -> (I16 -> Option I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 104

<a id="value-636865636b65642d72656d2d693332"></a>

## `checked-rem-i32`

```cinch
checked-rem-i32 :: (I32 -> (I32 -> Option I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 143

<a id="value-636865636b65642d72656d2d693634"></a>

## `checked-rem-i64`

```cinch
checked-rem-i64 :: (I64 -> (I64 -> Option I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 182

<a id="value-636865636b65642d72656d2d6938"></a>

## `checked-rem-i8`

```cinch
checked-rem-i8 :: (I8 -> (I8 -> Option I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 65

<a id="value-636865636b65642d72656d2d6973697a65"></a>

## `checked-rem-isize`

```cinch
checked-rem-isize :: (Isize -> (Isize -> Option Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 416

<a id="value-636865636b65642d72656d2d753136"></a>

## `checked-rem-u16`

```cinch
checked-rem-u16 :: (U16 -> (U16 -> Option U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 260

<a id="value-636865636b65642d72656d2d753332"></a>

## `checked-rem-u32`

```cinch
checked-rem-u32 :: (U32 -> (U32 -> Option U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 299

<a id="value-636865636b65642d72656d2d753634"></a>

## `checked-rem-u64`

```cinch
checked-rem-u64 :: (U64 -> (U64 -> Option U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 338

<a id="value-636865636b65642d72656d2d7538"></a>

## `checked-rem-u8`

```cinch
checked-rem-u8 :: (U8 -> (U8 -> Option U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 221

<a id="value-636865636b65642d72656d2d7573697a65"></a>

## `checked-rem-usize`

```cinch
checked-rem-usize :: (Usize -> (Usize -> Option Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 377

<a id="value-636865636b65642d73686c2d693136"></a>

## `checked-shl-i16`

```cinch
checked-shl-i16 :: (I16 -> (U32 -> Option I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 105

<a id="value-636865636b65642d73686c2d693332"></a>

## `checked-shl-i32`

```cinch
checked-shl-i32 :: (I32 -> (U32 -> Option I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 144

<a id="value-636865636b65642d73686c2d693634"></a>

## `checked-shl-i64`

```cinch
checked-shl-i64 :: (I64 -> (U32 -> Option I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 183

<a id="value-636865636b65642d73686c2d6938"></a>

## `checked-shl-i8`

```cinch
checked-shl-i8 :: (I8 -> (U32 -> Option I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 66

<a id="value-636865636b65642d73686c2d6973697a65"></a>

## `checked-shl-isize`

```cinch
checked-shl-isize :: (Isize -> (U32 -> Option Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 417

<a id="value-636865636b65642d73686c2d753136"></a>

## `checked-shl-u16`

```cinch
checked-shl-u16 :: (U16 -> (U32 -> Option U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 261

<a id="value-636865636b65642d73686c2d753332"></a>

## `checked-shl-u32`

```cinch
checked-shl-u32 :: (U32 -> (U32 -> Option U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 300

<a id="value-636865636b65642d73686c2d753634"></a>

## `checked-shl-u64`

```cinch
checked-shl-u64 :: (U64 -> (U32 -> Option U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 339

<a id="value-636865636b65642d73686c2d7538"></a>

## `checked-shl-u8`

```cinch
checked-shl-u8 :: (U8 -> (U32 -> Option U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 222

<a id="value-636865636b65642d73686c2d7573697a65"></a>

## `checked-shl-usize`

```cinch
checked-shl-usize :: (Usize -> (U32 -> Option Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 378

<a id="value-636865636b65642d7368722d693136"></a>

## `checked-shr-i16`

```cinch
checked-shr-i16 :: (I16 -> (U32 -> Option I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 107

<a id="value-636865636b65642d7368722d693332"></a>

## `checked-shr-i32`

```cinch
checked-shr-i32 :: (I32 -> (U32 -> Option I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 146

<a id="value-636865636b65642d7368722d693634"></a>

## `checked-shr-i64`

```cinch
checked-shr-i64 :: (I64 -> (U32 -> Option I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 185

<a id="value-636865636b65642d7368722d6938"></a>

## `checked-shr-i8`

```cinch
checked-shr-i8 :: (I8 -> (U32 -> Option I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 68

<a id="value-636865636b65642d7368722d6973697a65"></a>

## `checked-shr-isize`

```cinch
checked-shr-isize :: (Isize -> (U32 -> Option Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 419

<a id="value-636865636b65642d7368722d753136"></a>

## `checked-shr-u16`

```cinch
checked-shr-u16 :: (U16 -> (U32 -> Option U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 263

<a id="value-636865636b65642d7368722d753332"></a>

## `checked-shr-u32`

```cinch
checked-shr-u32 :: (U32 -> (U32 -> Option U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 302

<a id="value-636865636b65642d7368722d753634"></a>

## `checked-shr-u64`

```cinch
checked-shr-u64 :: (U64 -> (U32 -> Option U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 341

<a id="value-636865636b65642d7368722d7538"></a>

## `checked-shr-u8`

```cinch
checked-shr-u8 :: (U8 -> (U32 -> Option U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 224

<a id="value-636865636b65642d7368722d7573697a65"></a>

## `checked-shr-usize`

```cinch
checked-shr-usize :: (Usize -> (U32 -> Option Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 380

<a id="value-636865636b65642d7375622d693136"></a>

## `checked-sub-i16`

```cinch
checked-sub-i16 :: (I16 -> (I16 -> Option I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 92

<a id="value-636865636b65642d7375622d693332"></a>

## `checked-sub-i32`

```cinch
checked-sub-i32 :: (I32 -> (I32 -> Option I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 131

<a id="value-636865636b65642d7375622d693634"></a>

## `checked-sub-i64`

```cinch
checked-sub-i64 :: (I64 -> (I64 -> Option I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 170

<a id="value-636865636b65642d7375622d6938"></a>

## `checked-sub-i8`

```cinch
checked-sub-i8 :: (I8 -> (I8 -> Option I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 53

<a id="value-636865636b65642d7375622d6973697a65"></a>

## `checked-sub-isize`

```cinch
checked-sub-isize :: (Isize -> (Isize -> Option Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 404

<a id="value-636865636b65642d7375622d753136"></a>

## `checked-sub-u16`

```cinch
checked-sub-u16 :: (U16 -> (U16 -> Option U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 248

<a id="value-636865636b65642d7375622d753332"></a>

## `checked-sub-u32`

```cinch
checked-sub-u32 :: (U32 -> (U32 -> Option U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 287

<a id="value-636865636b65642d7375622d753634"></a>

## `checked-sub-u64`

```cinch
checked-sub-u64 :: (U64 -> (U64 -> Option U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 326

<a id="value-636865636b65642d7375622d7538"></a>

## `checked-sub-u8`

```cinch
checked-sub-u8 :: (U8 -> (U8 -> Option U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 209

<a id="value-636865636b65642d7375622d7573697a65"></a>

## `checked-sub-usize`

```cinch
checked-sub-usize :: (Usize -> (Usize -> Option Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 365

<a id="value-636865636b65642d7531362d746f2d693136"></a>

## `checked-u16-to-i16`

```cinch
checked-u16-to-i16 :: (U16 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 725

<a id="value-636865636b65642d7531362d746f2d693332"></a>

## `checked-u16-to-i32`

```cinch
checked-u16-to-i32 :: (U16 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 728

<a id="value-636865636b65642d7531362d746f2d693634"></a>

## `checked-u16-to-i64`

```cinch
checked-u16-to-i64 :: (U16 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 731

<a id="value-636865636b65642d7531362d746f2d6938"></a>

## `checked-u16-to-i8`

```cinch
checked-u16-to-i8 :: (U16 -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 722

<a id="value-636865636b65642d7531362d746f2d6973697a65"></a>

## `checked-u16-to-isize`

```cinch
checked-u16-to-isize :: (U16 -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 749

<a id="value-636865636b65642d7531362d746f2d753136"></a>

## `checked-u16-to-u16`

```cinch
checked-u16-to-u16 :: (U16 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 737

<a id="value-636865636b65642d7531362d746f2d753332"></a>

## `checked-u16-to-u32`

```cinch
checked-u16-to-u32 :: (U16 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 740

<a id="value-636865636b65642d7531362d746f2d753634"></a>

## `checked-u16-to-u64`

```cinch
checked-u16-to-u64 :: (U16 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 743

<a id="value-636865636b65642d7531362d746f2d7538"></a>

## `checked-u16-to-u8`

```cinch
checked-u16-to-u8 :: (U16 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 734

<a id="value-636865636b65642d7531362d746f2d7573697a65"></a>

## `checked-u16-to-usize`

```cinch
checked-u16-to-usize :: (U16 -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 746

<a id="value-636865636b65642d7533322d746f2d693136"></a>

## `checked-u32-to-i16`

```cinch
checked-u32-to-i16 :: (U32 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 755

<a id="value-636865636b65642d7533322d746f2d693332"></a>

## `checked-u32-to-i32`

```cinch
checked-u32-to-i32 :: (U32 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 758

<a id="value-636865636b65642d7533322d746f2d693634"></a>

## `checked-u32-to-i64`

```cinch
checked-u32-to-i64 :: (U32 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 761

<a id="value-636865636b65642d7533322d746f2d6938"></a>

## `checked-u32-to-i8`

```cinch
checked-u32-to-i8 :: (U32 -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 752

<a id="value-636865636b65642d7533322d746f2d6973697a65"></a>

## `checked-u32-to-isize`

```cinch
checked-u32-to-isize :: (U32 -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 779

<a id="value-636865636b65642d7533322d746f2d753136"></a>

## `checked-u32-to-u16`

```cinch
checked-u32-to-u16 :: (U32 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 767

<a id="value-636865636b65642d7533322d746f2d753332"></a>

## `checked-u32-to-u32`

```cinch
checked-u32-to-u32 :: (U32 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 770

<a id="value-636865636b65642d7533322d746f2d753634"></a>

## `checked-u32-to-u64`

```cinch
checked-u32-to-u64 :: (U32 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 773

<a id="value-636865636b65642d7533322d746f2d7538"></a>

## `checked-u32-to-u8`

```cinch
checked-u32-to-u8 :: (U32 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 764

<a id="value-636865636b65642d7533322d746f2d7573697a65"></a>

## `checked-u32-to-usize`

```cinch
checked-u32-to-usize :: (U32 -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 776

<a id="value-636865636b65642d7536342d746f2d693136"></a>

## `checked-u64-to-i16`

```cinch
checked-u64-to-i16 :: (U64 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 785

<a id="value-636865636b65642d7536342d746f2d693332"></a>

## `checked-u64-to-i32`

```cinch
checked-u64-to-i32 :: (U64 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 788

<a id="value-636865636b65642d7536342d746f2d693634"></a>

## `checked-u64-to-i64`

```cinch
checked-u64-to-i64 :: (U64 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 791

<a id="value-636865636b65642d7536342d746f2d6938"></a>

## `checked-u64-to-i8`

```cinch
checked-u64-to-i8 :: (U64 -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 782

<a id="value-636865636b65642d7536342d746f2d6973697a65"></a>

## `checked-u64-to-isize`

```cinch
checked-u64-to-isize :: (U64 -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 809

<a id="value-636865636b65642d7536342d746f2d753136"></a>

## `checked-u64-to-u16`

```cinch
checked-u64-to-u16 :: (U64 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 797

<a id="value-636865636b65642d7536342d746f2d753332"></a>

## `checked-u64-to-u32`

```cinch
checked-u64-to-u32 :: (U64 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 800

<a id="value-636865636b65642d7536342d746f2d753634"></a>

## `checked-u64-to-u64`

```cinch
checked-u64-to-u64 :: (U64 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 803

<a id="value-636865636b65642d7536342d746f2d7538"></a>

## `checked-u64-to-u8`

```cinch
checked-u64-to-u8 :: (U64 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 794

<a id="value-636865636b65642d7536342d746f2d7573697a65"></a>

## `checked-u64-to-usize`

```cinch
checked-u64-to-usize :: (U64 -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 806

<a id="value-636865636b65642d75382d746f2d693136"></a>

## `checked-u8-to-i16`

```cinch
checked-u8-to-i16 :: (U8 -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 695

<a id="value-636865636b65642d75382d746f2d693332"></a>

## `checked-u8-to-i32`

```cinch
checked-u8-to-i32 :: (U8 -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 698

<a id="value-636865636b65642d75382d746f2d693634"></a>

## `checked-u8-to-i64`

```cinch
checked-u8-to-i64 :: (U8 -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 701

<a id="value-636865636b65642d75382d746f2d6938"></a>

## `checked-u8-to-i8`

```cinch
checked-u8-to-i8 :: (U8 -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 692

<a id="value-636865636b65642d75382d746f2d6973697a65"></a>

## `checked-u8-to-isize`

```cinch
checked-u8-to-isize :: (U8 -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 719

<a id="value-636865636b65642d75382d746f2d753136"></a>

## `checked-u8-to-u16`

```cinch
checked-u8-to-u16 :: (U8 -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 707

<a id="value-636865636b65642d75382d746f2d753332"></a>

## `checked-u8-to-u32`

```cinch
checked-u8-to-u32 :: (U8 -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 710

<a id="value-636865636b65642d75382d746f2d753634"></a>

## `checked-u8-to-u64`

```cinch
checked-u8-to-u64 :: (U8 -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 713

<a id="value-636865636b65642d75382d746f2d7538"></a>

## `checked-u8-to-u8`

```cinch
checked-u8-to-u8 :: (U8 -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 704

<a id="value-636865636b65642d75382d746f2d7573697a65"></a>

## `checked-u8-to-usize`

```cinch
checked-u8-to-usize :: (U8 -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 716

<a id="value-636865636b65642d7573697a652d746f2d693136"></a>

## `checked-usize-to-i16`

```cinch
checked-usize-to-i16 :: (Usize -> Option I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 815

<a id="value-636865636b65642d7573697a652d746f2d693332"></a>

## `checked-usize-to-i32`

```cinch
checked-usize-to-i32 :: (Usize -> Option I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 818

<a id="value-636865636b65642d7573697a652d746f2d693634"></a>

## `checked-usize-to-i64`

```cinch
checked-usize-to-i64 :: (Usize -> Option I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 821

<a id="value-636865636b65642d7573697a652d746f2d6938"></a>

## `checked-usize-to-i8`

```cinch
checked-usize-to-i8 :: (Usize -> Option I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 812

<a id="value-636865636b65642d7573697a652d746f2d6973697a65"></a>

## `checked-usize-to-isize`

```cinch
checked-usize-to-isize :: (Usize -> Option Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 839

<a id="value-636865636b65642d7573697a652d746f2d753136"></a>

## `checked-usize-to-u16`

```cinch
checked-usize-to-u16 :: (Usize -> Option U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 827

<a id="value-636865636b65642d7573697a652d746f2d753332"></a>

## `checked-usize-to-u32`

```cinch
checked-usize-to-u32 :: (Usize -> Option U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 830

<a id="value-636865636b65642d7573697a652d746f2d753634"></a>

## `checked-usize-to-u64`

```cinch
checked-usize-to-u64 :: (Usize -> Option U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 833

<a id="value-636865636b65642d7573697a652d746f2d7538"></a>

## `checked-usize-to-u8`

```cinch
checked-usize-to-u8 :: (Usize -> Option U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 824

<a id="value-636865636b65642d7573697a652d746f2d7573697a65"></a>

## `checked-usize-to-usize`

```cinch
checked-usize-to-usize :: (Usize -> Option Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 836

<a id="value-636f756e742d6f6e65732d693136"></a>

## `count-ones-i16`

```cinch
count-ones-i16 :: (I16 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 115

<a id="value-636f756e742d6f6e65732d693332"></a>

## `count-ones-i32`

```cinch
count-ones-i32 :: (I32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 154

<a id="value-636f756e742d6f6e65732d693634"></a>

## `count-ones-i64`

```cinch
count-ones-i64 :: (I64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 193

<a id="value-636f756e742d6f6e65732d6938"></a>

## `count-ones-i8`

```cinch
count-ones-i8 :: (I8 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 76

<a id="value-636f756e742d6f6e65732d6973697a65"></a>

## `count-ones-isize`

```cinch
count-ones-isize :: (Isize -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 427

<a id="value-636f756e742d6f6e65732d753136"></a>

## `count-ones-u16`

```cinch
count-ones-u16 :: (U16 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 271

<a id="value-636f756e742d6f6e65732d753332"></a>

## `count-ones-u32`

```cinch
count-ones-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 310

<a id="value-636f756e742d6f6e65732d753634"></a>

## `count-ones-u64`

```cinch
count-ones-u64 :: (U64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 349

<a id="value-636f756e742d6f6e65732d7538"></a>

## `count-ones-u8`

```cinch
count-ones-u8 :: (U8 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 232

<a id="value-636f756e742d6f6e65732d7573697a65"></a>

## `count-ones-usize`

```cinch
count-ones-usize :: (Usize -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 388

<a id="value-6469762d663332"></a>

## `div-f32`

```cinch
div-f32 :: (F32 -> (F32 -> F32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 448

<a id="value-6469762d663634"></a>

## `div-f64`

```cinch
div-f64 :: (F64 -> (F64 -> F64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 471

<a id="value-65712d693136"></a>

## `eq-i16`

```cinch
eq-i16 :: (I16 -> (I16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 120

<a id="value-65712d693332"></a>

## `eq-i32`

```cinch
eq-i32 :: (I32 -> (I32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 159

<a id="value-65712d693634"></a>

## `eq-i64`

```cinch
eq-i64 :: (I64 -> (I64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 198

<a id="value-65712d6938"></a>

## `eq-i8`

```cinch
eq-i8 :: (I8 -> (I8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 81

<a id="value-65712d6973697a65"></a>

## `eq-isize`

```cinch
eq-isize :: (Isize -> (Isize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 432

<a id="value-65712d753136"></a>

## `eq-u16`

```cinch
eq-u16 :: (U16 -> (U16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 276

<a id="value-65712d753332"></a>

## `eq-u32`

```cinch
eq-u32 :: (U32 -> (U32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 315

<a id="value-65712d753634"></a>

## `eq-u64`

```cinch
eq-u64 :: (U64 -> (U64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 354

<a id="value-65712d7538"></a>

## `eq-u8`

```cinch
eq-u8 :: (U8 -> (U8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 237

<a id="value-65712d7573697a65"></a>

## `eq-usize`

```cinch
eq-usize :: (Usize -> (Usize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 393

<a id="value-6633322d746f2d663332"></a>

## `f32-to-f32`

```cinch
f32-to-f32 :: (F32 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 565

<a id="value-6633322d746f2d663634"></a>

## `f32-to-f64`

```cinch
f32-to-f64 :: (F32 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 566

<a id="value-6636342d66726f6d2d646563696d616c"></a>

## `f64-from-decimal`

```cinch
f64-from-decimal :: (Bytes -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 1220

<a id="value-6636342d746f2d646563696d616c"></a>

## `f64-to-decimal`

```cinch
f64-to-decimal :: (F64 -> Bytes)
```

The P31 text conversions of `F64` (compiler/runtime/src/text.rs;
the `cinch.runtime.textF64*` intrinsics, nm-verified there):
one float's shortest round-trip decimal text (scientific form)
and the float one decimal spelling stands for (correctly
rounded). A spelling that is not a decimal number takes the
defined `MalformedDecimal` fault (exit 71).

[Source](/reference-sources/Data.Machine.cinch) · line 1219

<a id="value-6636342d746f2d663332"></a>

## `f64-to-f32`

```cinch
f64-to-f32 :: (F64 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 567

<a id="value-6636342d746f2d663634"></a>

## `f64-to-f64`

```cinch
f64-to-f64 :: (F64 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 568

<a id="value-6665712d663332"></a>

## `feq-f32`

```cinch
feq-f32 :: (F32 -> (F32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 454

<a id="value-6665712d663634"></a>

## `feq-f64`

```cinch
feq-f64 :: (F64 -> (F64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 477

<a id="value-6667652d663332"></a>

## `fge-f32`

```cinch
fge-f32 :: (F32 -> (F32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 459

<a id="value-6667652d663634"></a>

## `fge-f64`

```cinch
fge-f64 :: (F64 -> (F64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 482

<a id="value-6667742d663332"></a>

## `fgt-f32`

```cinch
fgt-f32 :: (F32 -> (F32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 458

<a id="value-6667742d663634"></a>

## `fgt-f64`

```cinch
fgt-f64 :: (F64 -> (F64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 481

<a id="value-666c652d663332"></a>

## `fle-f32`

```cinch
fle-f32 :: (F32 -> (F32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 457

<a id="value-666c652d663634"></a>

## `fle-f64`

```cinch
fle-f64 :: (F64 -> (F64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 480

<a id="value-666c742d663332"></a>

## `flt-f32`

```cinch
flt-f32 :: (F32 -> (F32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 456

<a id="value-666c742d663634"></a>

## `flt-f64`

```cinch
flt-f64 :: (F64 -> (F64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 479

<a id="value-666e652d663332"></a>

## `fne-f32`

```cinch
fne-f32 :: (F32 -> (F32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 455

<a id="value-666e652d663634"></a>

## `fne-f64`

```cinch
fne-f64 :: (F64 -> (F64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 478

<a id="value-67652d693136"></a>

## `ge-i16`

```cinch
ge-i16 :: (I16 -> (I16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 125

<a id="value-67652d693332"></a>

## `ge-i32`

```cinch
ge-i32 :: (I32 -> (I32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 164

<a id="value-67652d693634"></a>

## `ge-i64`

```cinch
ge-i64 :: (I64 -> (I64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 203

<a id="value-67652d6938"></a>

## `ge-i8`

```cinch
ge-i8 :: (I8 -> (I8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 86

<a id="value-67652d6973697a65"></a>

## `ge-isize`

```cinch
ge-isize :: (Isize -> (Isize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 437

<a id="value-67652d753136"></a>

## `ge-u16`

```cinch
ge-u16 :: (U16 -> (U16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 281

<a id="value-67652d753332"></a>

## `ge-u32`

```cinch
ge-u32 :: (U32 -> (U32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 320

<a id="value-67652d753634"></a>

## `ge-u64`

```cinch
ge-u64 :: (U64 -> (U64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 359

<a id="value-67652d7538"></a>

## `ge-u8`

```cinch
ge-u8 :: (U8 -> (U8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 242

<a id="value-67652d7573697a65"></a>

## `ge-usize`

```cinch
ge-usize :: (Usize -> (Usize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 398

<a id="value-67742d693136"></a>

## `gt-i16`

```cinch
gt-i16 :: (I16 -> (I16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 124

<a id="value-67742d693332"></a>

## `gt-i32`

```cinch
gt-i32 :: (I32 -> (I32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 163

<a id="value-67742d693634"></a>

## `gt-i64`

```cinch
gt-i64 :: (I64 -> (I64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 202

<a id="value-67742d6938"></a>

## `gt-i8`

```cinch
gt-i8 :: (I8 -> (I8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 85

<a id="value-67742d6973697a65"></a>

## `gt-isize`

```cinch
gt-isize :: (Isize -> (Isize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 436

<a id="value-67742d753136"></a>

## `gt-u16`

```cinch
gt-u16 :: (U16 -> (U16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 280

<a id="value-67742d753332"></a>

## `gt-u32`

```cinch
gt-u32 :: (U32 -> (U32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 319

<a id="value-67742d753634"></a>

## `gt-u64`

```cinch
gt-u64 :: (U64 -> (U64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 358

<a id="value-67742d7538"></a>

## `gt-u8`

```cinch
gt-u8 :: (U8 -> (U8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 241

<a id="value-67742d7573697a65"></a>

## `gt-usize`

```cinch
gt-usize :: (Usize -> (Usize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 397

<a id="value-6931362d746f2d663332"></a>

## `i16-to-f32`

```cinch
i16-to-f32 :: (I16 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 547

<a id="value-6931362d746f2d663634"></a>

## `i16-to-f64`

```cinch
i16-to-f64 :: (I16 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 548

<a id="value-6931362d746f2d693136"></a>

## `i16-to-i16`

```cinch
i16-to-i16 :: (I16 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 509

<a id="value-6931362d746f2d693332"></a>

## `i16-to-i32`

```cinch
i16-to-i32 :: ((a0 :: I16) -> refined (((-32768 <= r && r <= 32767) && r == a0)) I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 510

<a id="value-6931362d746f2d693634"></a>

## `i16-to-i64`

```cinch
i16-to-i64 :: ((a0 :: I16) -> refined (((-32768 <= r && r <= 32767) && r == a0)) I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 511

<a id="value-6931362d746f2d6973697a65"></a>

## `i16-to-isize`

```cinch
i16-to-isize :: ((a0 :: I16) -> refined (((-32768 <= r && r <= 32767) && r == a0)) Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 512

<a id="value-6933322d746f2d663332"></a>

## `i32-to-f32`

```cinch
i32-to-f32 :: (I32 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 549

<a id="value-6933322d746f2d663634"></a>

## `i32-to-f64`

```cinch
i32-to-f64 :: (I32 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 550

<a id="value-6933322d746f2d693332"></a>

## `i32-to-i32`

```cinch
i32-to-i32 :: (I32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 513

<a id="value-6933322d746f2d693634"></a>

## `i32-to-i64`

```cinch
i32-to-i64 :: ((a0 :: I32) -> refined (((-2147483648 <= r && r <= 2147483647) && r == a0)) I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 514

<a id="value-6933322d746f2d6973697a65"></a>

## `i32-to-isize`

```cinch
i32-to-isize :: ((a0 :: I32) -> refined (((-2147483648 <= r && r <= 2147483647) && r == a0)) Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 515

<a id="value-6936342d6c652d62797465"></a>

## `i64-le-byte`

```cinch
i64-le-byte :: (I64 -> (U32 -> InRange 0 255 Nat))
```

One little-endian byte of an `I64` at shift `k` bits (the A6
INTEGER column format, modules specification section 11).

[Source](/reference-sources/Data.Machine.cinch) · line 1210

<a id="value-6936342d746f2d663332"></a>

## `i64-to-f32`

```cinch
i64-to-f32 :: (I64 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 551

<a id="value-6936342d746f2d663634"></a>

## `i64-to-f64`

```cinch
i64-to-f64 :: (I64 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 552

<a id="value-6936342d746f2d693634"></a>

## `i64-to-i64`

```cinch
i64-to-i64 :: (I64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 516

<a id="value-6936342d746f2d6973697a65"></a>

## `i64-to-isize`

```cinch
i64-to-isize :: ((a0 :: I64) -> refined (r == a0) Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 517

<a id="value-69382d746f2d663332"></a>

## `i8-to-f32`

```cinch
i8-to-f32 :: (I8 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 545

<a id="value-69382d746f2d663634"></a>

## `i8-to-f64`

```cinch
i8-to-f64 :: (I8 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 546

<a id="value-69382d746f2d693136"></a>

## `i8-to-i16`

```cinch
i8-to-i16 :: ((a0 :: I8) -> refined (((-128 <= r && r <= 127) && r == a0)) I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 505

<a id="value-69382d746f2d693332"></a>

## `i8-to-i32`

```cinch
i8-to-i32 :: ((a0 :: I8) -> refined (((-128 <= r && r <= 127) && r == a0)) I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 506

<a id="value-69382d746f2d693634"></a>

## `i8-to-i64`

```cinch
i8-to-i64 :: ((a0 :: I8) -> refined (((-128 <= r && r <= 127) && r == a0)) I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 507

<a id="value-69382d746f2d6938"></a>

## `i8-to-i8`

```cinch
i8-to-i8 :: (I8 -> I8)
```

Primitive types specification, section 7: explicit conversions for
every ordered pair of numeric machine types. The total family
(`u8-to-i32`) exists when the conversion preserves every value, and
for integer-to-float and float-to-float, which round per IEEE 754.
`checked` returns Option (None on NaN and out-of-range floats),
`saturating` clamps (NaN maps to zero), `wrapping` truncates in
two's complement and is defined between integer types only. A
total integer conversion's result is its source's value: it states
that (`r == x`, a fact over the argument wherever the argument has
a reading) and the source's range (`InRange 0 255 r` for a `U8`,
which holds even where it has none), so a converted byte is a
non-negative `I64` for `nat-from-i64` and below 256 wherever that
matters.

[Source](/reference-sources/Data.Machine.cinch) · line 504

<a id="value-69382d746f2d6973697a65"></a>

## `i8-to-isize`

```cinch
i8-to-isize :: ((a0 :: I8) -> refined (((-128 <= r && r <= 127) && r == a0)) Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 508

<a id="value-69732d66696e6974652d663332"></a>

## `is-finite-f32`

```cinch
is-finite-f32 :: (F32 -> Bool)
```

[Source](/reference-sources/Data.Machine.cinch) · line 462

<a id="value-69732d66696e6974652d663634"></a>

## `is-finite-f64`

```cinch
is-finite-f64 :: (F64 -> Bool)
```

[Source](/reference-sources/Data.Machine.cinch) · line 485

<a id="value-69732d696e66696e6974652d663332"></a>

## `is-infinite-f32`

```cinch
is-infinite-f32 :: (F32 -> Bool)
```

[Source](/reference-sources/Data.Machine.cinch) · line 461

<a id="value-69732d696e66696e6974652d663634"></a>

## `is-infinite-f64`

```cinch
is-infinite-f64 :: (F64 -> Bool)
```

[Source](/reference-sources/Data.Machine.cinch) · line 484

<a id="value-69732d6e612d6e663332"></a>

## `is-na-nf32`

```cinch
is-na-nf32 :: (F32 -> Bool)
```

[Source](/reference-sources/Data.Machine.cinch) · line 460

<a id="value-69732d6e612d6e663634"></a>

## `is-na-nf64`

```cinch
is-na-nf64 :: (F64 -> Bool)
```

[Source](/reference-sources/Data.Machine.cinch) · line 483

<a id="value-69732d6e6f726d616c2d663332"></a>

## `is-normal-f32`

```cinch
is-normal-f32 :: (F32 -> Bool)
```

[Source](/reference-sources/Data.Machine.cinch) · line 463

<a id="value-69732d6e6f726d616c2d663634"></a>

## `is-normal-f64`

```cinch
is-normal-f64 :: (F64 -> Bool)
```

[Source](/reference-sources/Data.Machine.cinch) · line 486

<a id="value-6973697a652d746f2d663332"></a>

## `isize-to-f32`

```cinch
isize-to-f32 :: (Isize -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 563

<a id="value-6973697a652d746f2d663634"></a>

## `isize-to-f64`

```cinch
isize-to-f64 :: (Isize -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 564

<a id="value-6973697a652d746f2d693634"></a>

## `isize-to-i64`

```cinch
isize-to-i64 :: ((a0 :: Isize) -> refined (r == a0) I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 543

<a id="value-6973697a652d746f2d6973697a65"></a>

## `isize-to-isize`

```cinch
isize-to-isize :: (Isize -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 544

<a id="value-6c652d693136"></a>

## `le-i16`

```cinch
le-i16 :: (I16 -> (I16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 123

<a id="value-6c652d693332"></a>

## `le-i32`

```cinch
le-i32 :: (I32 -> (I32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 162

<a id="value-6c652d693634"></a>

## `le-i64`

```cinch
le-i64 :: (I64 -> (I64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 201

<a id="value-6c652d6938"></a>

## `le-i8`

```cinch
le-i8 :: (I8 -> (I8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 84

<a id="value-6c652d6973697a65"></a>

## `le-isize`

```cinch
le-isize :: (Isize -> (Isize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 435

<a id="value-6c652d753136"></a>

## `le-u16`

```cinch
le-u16 :: (U16 -> (U16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 279

<a id="value-6c652d753332"></a>

## `le-u32`

```cinch
le-u32 :: (U32 -> (U32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 318

<a id="value-6c652d753634"></a>

## `le-u64`

```cinch
le-u64 :: (U64 -> (U64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 357

<a id="value-6c652d7538"></a>

## `le-u8`

```cinch
le-u8 :: (U8 -> (U8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 240

<a id="value-6c652d7573697a65"></a>

## `le-usize`

```cinch
le-usize :: (Usize -> (Usize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 396

<a id="value-6c656164696e672d7a65726f732d693136"></a>

## `leading-zeros-i16`

```cinch
leading-zeros-i16 :: (I16 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 116

<a id="value-6c656164696e672d7a65726f732d693332"></a>

## `leading-zeros-i32`

```cinch
leading-zeros-i32 :: (I32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 155

<a id="value-6c656164696e672d7a65726f732d693634"></a>

## `leading-zeros-i64`

```cinch
leading-zeros-i64 :: (I64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 194

<a id="value-6c656164696e672d7a65726f732d6938"></a>

## `leading-zeros-i8`

```cinch
leading-zeros-i8 :: (I8 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 77

<a id="value-6c656164696e672d7a65726f732d6973697a65"></a>

## `leading-zeros-isize`

```cinch
leading-zeros-isize :: (Isize -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 428

<a id="value-6c656164696e672d7a65726f732d753136"></a>

## `leading-zeros-u16`

```cinch
leading-zeros-u16 :: (U16 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 272

<a id="value-6c656164696e672d7a65726f732d753332"></a>

## `leading-zeros-u32`

```cinch
leading-zeros-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 311

<a id="value-6c656164696e672d7a65726f732d753634"></a>

## `leading-zeros-u64`

```cinch
leading-zeros-u64 :: (U64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 350

<a id="value-6c656164696e672d7a65726f732d7538"></a>

## `leading-zeros-u8`

```cinch
leading-zeros-u8 :: (U8 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 233

<a id="value-6c656164696e672d7a65726f732d7573697a65"></a>

## `leading-zeros-usize`

```cinch
leading-zeros-usize :: (Usize -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 389

<a id="value-6c742d693136"></a>

## `lt-i16`

```cinch
lt-i16 :: (I16 -> (I16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 122

<a id="value-6c742d693332"></a>

## `lt-i32`

```cinch
lt-i32 :: (I32 -> (I32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 161

<a id="value-6c742d693634"></a>

## `lt-i64`

```cinch
lt-i64 :: (I64 -> (I64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 200

<a id="value-6c742d6938"></a>

## `lt-i8`

```cinch
lt-i8 :: (I8 -> (I8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 83

<a id="value-6c742d6973697a65"></a>

## `lt-isize`

```cinch
lt-isize :: (Isize -> (Isize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 434

<a id="value-6c742d753136"></a>

## `lt-u16`

```cinch
lt-u16 :: (U16 -> (U16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 278

<a id="value-6c742d753332"></a>

## `lt-u32`

```cinch
lt-u32 :: (U32 -> (U32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 317

<a id="value-6c742d753634"></a>

## `lt-u64`

```cinch
lt-u64 :: (U64 -> (U64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 356

<a id="value-6c742d7538"></a>

## `lt-u8`

```cinch
lt-u8 :: (U8 -> (U8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 239

<a id="value-6c742d7573697a65"></a>

## `lt-usize`

```cinch
lt-usize :: (Usize -> (Usize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 395

<a id="value-6d61736b2d693634"></a>

## `mask-i64`

```cinch
mask-i64 :: (I64 -> ((a1 :: InRange 0 9223372036854775807 I64) -> refined ((0 <= r && r <= a1)) I64))
```

A mask with a non-negative mask word: the `bit-and` of any word
with `m >= 0` lies in `[0, m]` (the and clears the sign bit and
every bit `m` lacks), so `mask-i64 x 255` is a byte. It is the same
intrinsic as `bit-and-i64`; only the declared range differs, which
`bit-and-i64` itself cannot state because a negative mask (`-1`,
the high bit) is an ordinary use of it.

[Source](/reference-sources/Data.Machine.cinch) · line 1194

<a id="value-6d61782d626f756e642d693136"></a>

## `max-bound-i16`

```cinch
max-bound-i16 :: refined (r == 32767) I16
```

[Source](/reference-sources/Data.Machine.cinch) · line 90

<a id="value-6d61782d626f756e642d693332"></a>

## `max-bound-i32`

```cinch
max-bound-i32 :: refined (r == 2147483647) I32
```

[Source](/reference-sources/Data.Machine.cinch) · line 129

<a id="value-6d61782d626f756e642d693634"></a>

## `max-bound-i64`

```cinch
max-bound-i64 :: refined (r == 9223372036854775807) I64
```

[Source](/reference-sources/Data.Machine.cinch) · line 168

<a id="value-6d61782d626f756e642d6938"></a>

## `max-bound-i8`

```cinch
max-bound-i8 :: refined (r == 127) I8
```

[Source](/reference-sources/Data.Machine.cinch) · line 51

<a id="value-6d61782d626f756e642d6973697a65"></a>

## `max-bound-isize`

```cinch
max-bound-isize :: Isize
```

[Source](/reference-sources/Data.Machine.cinch) · line 402

<a id="value-6d61782d626f756e642d753136"></a>

## `max-bound-u16`

```cinch
max-bound-u16 :: refined (r == 65535) U16
```

[Source](/reference-sources/Data.Machine.cinch) · line 246

<a id="value-6d61782d626f756e642d753332"></a>

## `max-bound-u32`

```cinch
max-bound-u32 :: refined (r == 4294967295) U32
```

[Source](/reference-sources/Data.Machine.cinch) · line 285

<a id="value-6d61782d626f756e642d753634"></a>

## `max-bound-u64`

```cinch
max-bound-u64 :: refined (r == 18446744073709551615) U64
```

[Source](/reference-sources/Data.Machine.cinch) · line 324

<a id="value-6d61782d626f756e642d7538"></a>

## `max-bound-u8`

```cinch
max-bound-u8 :: refined (r == 255) U8
```

[Source](/reference-sources/Data.Machine.cinch) · line 207

<a id="value-6d61782d626f756e642d7573697a65"></a>

## `max-bound-usize`

```cinch
max-bound-usize :: Usize
```

[Source](/reference-sources/Data.Machine.cinch) · line 363

<a id="value-6d6178696d756d2d663332"></a>

## `maximum-f32`

```cinch
maximum-f32 :: (F32 -> (F32 -> F32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 453

<a id="value-6d6178696d756d2d663634"></a>

## `maximum-f64`

```cinch
maximum-f64 :: (F64 -> (F64 -> F64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 476

<a id="value-6d696e2d626f756e642d693136"></a>

## `min-bound-i16`

```cinch
min-bound-i16 :: refined (r == -32768) I16
```

I16 (16 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 89

<a id="value-6d696e2d626f756e642d693332"></a>

## `min-bound-i32`

```cinch
min-bound-i32 :: refined (r == -2147483648) I32
```

I32 (32 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 128

<a id="value-6d696e2d626f756e642d693634"></a>

## `min-bound-i64`

```cinch
min-bound-i64 :: refined (r == -9223372036854775808) I64
```

I64 (64 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 167

<a id="value-6d696e2d626f756e642d6938"></a>

## `min-bound-i8`

```cinch
min-bound-i8 :: refined (r == -128) I8
```

Primitive types specification, section 4: integer arithmetic for
every machine integer type. Overflow behavior is never implicit:
`checked` returns Option and never the wrapped value, `wrapping` is
two's complement wrap, `saturating` clamps to the bounds. There is
no Add/Sub/Mul/Neg instance for machine types: a class method has a
single result type and cannot express which family is meant.

I8 (8 bits).
A bound states its value, so `nat-from-i64 max-bound-i64` and the
like discharge their range with nothing written.

[Source](/reference-sources/Data.Machine.cinch) · line 50

<a id="value-6d696e2d626f756e642d6973697a65"></a>

## `min-bound-isize`

```cinch
min-bound-isize :: Isize
```

Isize (64 bits, target word).

[Source](/reference-sources/Data.Machine.cinch) · line 401

<a id="value-6d696e2d626f756e642d753136"></a>

## `min-bound-u16`

```cinch
min-bound-u16 :: refined (r == 0) U16
```

U16 (16 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 245

<a id="value-6d696e2d626f756e642d753332"></a>

## `min-bound-u32`

```cinch
min-bound-u32 :: refined (r == 0) U32
```

U32 (32 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 284

<a id="value-6d696e2d626f756e642d753634"></a>

## `min-bound-u64`

```cinch
min-bound-u64 :: refined (r == 0) U64
```

U64 (64 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 323

<a id="value-6d696e2d626f756e642d7538"></a>

## `min-bound-u8`

```cinch
min-bound-u8 :: refined (r == 0) U8
```

U8 (8 bits).

[Source](/reference-sources/Data.Machine.cinch) · line 206

<a id="value-6d696e2d626f756e642d7573697a65"></a>

## `min-bound-usize`

```cinch
min-bound-usize :: Usize
```

Usize (64 bits, target word).

[Source](/reference-sources/Data.Machine.cinch) · line 362

<a id="value-6d696e696d756d2d663332"></a>

## `minimum-f32`

```cinch
minimum-f32 :: (F32 -> (F32 -> F32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 452

<a id="value-6d696e696d756d2d663634"></a>

## `minimum-f64`

```cinch
minimum-f64 :: (F64 -> (F64 -> F64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 475

<a id="value-6d756c2d663332"></a>

## `mul-f32`

```cinch
mul-f32 :: (F32 -> (F32 -> F32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 447

<a id="value-6d756c2d663634"></a>

## `mul-f64`

```cinch
mul-f64 :: (F64 -> (F64 -> F64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 470

<a id="value-6e652d693136"></a>

## `ne-i16`

```cinch
ne-i16 :: (I16 -> (I16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 121

<a id="value-6e652d693332"></a>

## `ne-i32`

```cinch
ne-i32 :: (I32 -> (I32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 160

<a id="value-6e652d693634"></a>

## `ne-i64`

```cinch
ne-i64 :: (I64 -> (I64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 199

<a id="value-6e652d6938"></a>

## `ne-i8`

```cinch
ne-i8 :: (I8 -> (I8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 82

<a id="value-6e652d6973697a65"></a>

## `ne-isize`

```cinch
ne-isize :: (Isize -> (Isize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 433

<a id="value-6e652d753136"></a>

## `ne-u16`

```cinch
ne-u16 :: (U16 -> (U16 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 277

<a id="value-6e652d753332"></a>

## `ne-u32`

```cinch
ne-u32 :: (U32 -> (U32 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 316

<a id="value-6e652d753634"></a>

## `ne-u64`

```cinch
ne-u64 :: (U64 -> (U64 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 355

<a id="value-6e652d7538"></a>

## `ne-u8`

```cinch
ne-u8 :: (U8 -> (U8 -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 238

<a id="value-6e652d7573697a65"></a>

## `ne-usize`

```cinch
ne-usize :: (Usize -> (Usize -> Bool))
```

[Source](/reference-sources/Data.Machine.cinch) · line 394

<a id="value-6e65672d663332"></a>

## `neg-f32`

```cinch
neg-f32 :: (F32 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 449

<a id="value-6e65672d663634"></a>

## `neg-f64`

```cinch
neg-f64 :: (F64 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 472

<a id="value-726576657273652d626974732d693136"></a>

## `reverse-bits-i16`

```cinch
reverse-bits-i16 :: (I16 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 118

<a id="value-726576657273652d626974732d693332"></a>

## `reverse-bits-i32`

```cinch
reverse-bits-i32 :: (I32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 157

<a id="value-726576657273652d626974732d693634"></a>

## `reverse-bits-i64`

```cinch
reverse-bits-i64 :: (I64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 196

<a id="value-726576657273652d626974732d6938"></a>

## `reverse-bits-i8`

```cinch
reverse-bits-i8 :: (I8 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 79

<a id="value-726576657273652d626974732d6973697a65"></a>

## `reverse-bits-isize`

```cinch
reverse-bits-isize :: (Isize -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 430

<a id="value-726576657273652d626974732d753136"></a>

## `reverse-bits-u16`

```cinch
reverse-bits-u16 :: (U16 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 274

<a id="value-726576657273652d626974732d753332"></a>

## `reverse-bits-u32`

```cinch
reverse-bits-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 313

<a id="value-726576657273652d626974732d753634"></a>

## `reverse-bits-u64`

```cinch
reverse-bits-u64 :: (U64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 352

<a id="value-726576657273652d626974732d7538"></a>

## `reverse-bits-u8`

```cinch
reverse-bits-u8 :: (U8 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 235

<a id="value-726576657273652d626974732d7573697a65"></a>

## `reverse-bits-usize`

```cinch
reverse-bits-usize :: (Usize -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 391

<a id="value-726f746174652d6c6566742d693136"></a>

## `rotate-left-i16`

```cinch
rotate-left-i16 :: (I16 -> (U32 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 109

<a id="value-726f746174652d6c6566742d693332"></a>

## `rotate-left-i32`

```cinch
rotate-left-i32 :: (I32 -> (U32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 148

<a id="value-726f746174652d6c6566742d693634"></a>

## `rotate-left-i64`

```cinch
rotate-left-i64 :: (I64 -> (U32 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 187

<a id="value-726f746174652d6c6566742d6938"></a>

## `rotate-left-i8`

```cinch
rotate-left-i8 :: (I8 -> (U32 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 70

<a id="value-726f746174652d6c6566742d6973697a65"></a>

## `rotate-left-isize`

```cinch
rotate-left-isize :: (Isize -> (U32 -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 421

<a id="value-726f746174652d6c6566742d753136"></a>

## `rotate-left-u16`

```cinch
rotate-left-u16 :: (U16 -> (U32 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 265

<a id="value-726f746174652d6c6566742d753332"></a>

## `rotate-left-u32`

```cinch
rotate-left-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 304

<a id="value-726f746174652d6c6566742d753634"></a>

## `rotate-left-u64`

```cinch
rotate-left-u64 :: (U64 -> (U32 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 343

<a id="value-726f746174652d6c6566742d7538"></a>

## `rotate-left-u8`

```cinch
rotate-left-u8 :: (U8 -> (U32 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 226

<a id="value-726f746174652d6c6566742d7573697a65"></a>

## `rotate-left-usize`

```cinch
rotate-left-usize :: (Usize -> (U32 -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 382

<a id="value-726f746174652d72696768742d693136"></a>

## `rotate-right-i16`

```cinch
rotate-right-i16 :: (I16 -> (U32 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 110

<a id="value-726f746174652d72696768742d693332"></a>

## `rotate-right-i32`

```cinch
rotate-right-i32 :: (I32 -> (U32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 149

<a id="value-726f746174652d72696768742d693634"></a>

## `rotate-right-i64`

```cinch
rotate-right-i64 :: (I64 -> (U32 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 188

<a id="value-726f746174652d72696768742d6938"></a>

## `rotate-right-i8`

```cinch
rotate-right-i8 :: (I8 -> (U32 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 71

<a id="value-726f746174652d72696768742d6973697a65"></a>

## `rotate-right-isize`

```cinch
rotate-right-isize :: (Isize -> (U32 -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 422

<a id="value-726f746174652d72696768742d753136"></a>

## `rotate-right-u16`

```cinch
rotate-right-u16 :: (U16 -> (U32 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 266

<a id="value-726f746174652d72696768742d753332"></a>

## `rotate-right-u32`

```cinch
rotate-right-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 305

<a id="value-726f746174652d72696768742d753634"></a>

## `rotate-right-u64`

```cinch
rotate-right-u64 :: (U64 -> (U32 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 344

<a id="value-726f746174652d72696768742d7538"></a>

## `rotate-right-u8`

```cinch
rotate-right-u8 :: (U8 -> (U32 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 227

<a id="value-726f746174652d72696768742d7573697a65"></a>

## `rotate-right-usize`

```cinch
rotate-right-usize :: (Usize -> (U32 -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 383

<a id="value-73617475726174696e672d6164642d693136"></a>

## `saturating-add-i16`

```cinch
saturating-add-i16 :: (I16 -> (I16 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 97

<a id="value-73617475726174696e672d6164642d693332"></a>

## `saturating-add-i32`

```cinch
saturating-add-i32 :: (I32 -> (I32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 136

<a id="value-73617475726174696e672d6164642d693634"></a>

## `saturating-add-i64`

```cinch
saturating-add-i64 :: (I64 -> (I64 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 175

<a id="value-73617475726174696e672d6164642d6938"></a>

## `saturating-add-i8`

```cinch
saturating-add-i8 :: (I8 -> (I8 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 58

<a id="value-73617475726174696e672d6164642d6973697a65"></a>

## `saturating-add-isize`

```cinch
saturating-add-isize :: (Isize -> (Isize -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 409

<a id="value-73617475726174696e672d6164642d753136"></a>

## `saturating-add-u16`

```cinch
saturating-add-u16 :: (U16 -> (U16 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 253

<a id="value-73617475726174696e672d6164642d753332"></a>

## `saturating-add-u32`

```cinch
saturating-add-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 292

<a id="value-73617475726174696e672d6164642d753634"></a>

## `saturating-add-u64`

```cinch
saturating-add-u64 :: (U64 -> (U64 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 331

<a id="value-73617475726174696e672d6164642d7538"></a>

## `saturating-add-u8`

```cinch
saturating-add-u8 :: (U8 -> (U8 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 214

<a id="value-73617475726174696e672d6164642d7573697a65"></a>

## `saturating-add-usize`

```cinch
saturating-add-usize :: (Usize -> (Usize -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 370

<a id="value-73617475726174696e672d6633322d746f2d693136"></a>

## `saturating-f32-to-i16`

```cinch
saturating-f32-to-i16 :: (F32 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 875

<a id="value-73617475726174696e672d6633322d746f2d693332"></a>

## `saturating-f32-to-i32`

```cinch
saturating-f32-to-i32 :: (F32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 877

<a id="value-73617475726174696e672d6633322d746f2d693634"></a>

## `saturating-f32-to-i64`

```cinch
saturating-f32-to-i64 :: (F32 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 879

<a id="value-73617475726174696e672d6633322d746f2d6938"></a>

## `saturating-f32-to-i8`

```cinch
saturating-f32-to-i8 :: (F32 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 873

<a id="value-73617475726174696e672d6633322d746f2d6973697a65"></a>

## `saturating-f32-to-isize`

```cinch
saturating-f32-to-isize :: (F32 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 891

<a id="value-73617475726174696e672d6633322d746f2d753136"></a>

## `saturating-f32-to-u16`

```cinch
saturating-f32-to-u16 :: (F32 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 883

<a id="value-73617475726174696e672d6633322d746f2d753332"></a>

## `saturating-f32-to-u32`

```cinch
saturating-f32-to-u32 :: (F32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 885

<a id="value-73617475726174696e672d6633322d746f2d753634"></a>

## `saturating-f32-to-u64`

```cinch
saturating-f32-to-u64 :: (F32 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 887

<a id="value-73617475726174696e672d6633322d746f2d7538"></a>

## `saturating-f32-to-u8`

```cinch
saturating-f32-to-u8 :: (F32 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 881

<a id="value-73617475726174696e672d6633322d746f2d7573697a65"></a>

## `saturating-f32-to-usize`

```cinch
saturating-f32-to-usize :: (F32 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 889

<a id="value-73617475726174696e672d6636342d746f2d693136"></a>

## `saturating-f64-to-i16`

```cinch
saturating-f64-to-i16 :: (F64 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 895

<a id="value-73617475726174696e672d6636342d746f2d693332"></a>

## `saturating-f64-to-i32`

```cinch
saturating-f64-to-i32 :: (F64 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 897

<a id="value-73617475726174696e672d6636342d746f2d693634"></a>

## `saturating-f64-to-i64`

```cinch
saturating-f64-to-i64 :: (F64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 899

<a id="value-73617475726174696e672d6636342d746f2d6938"></a>

## `saturating-f64-to-i8`

```cinch
saturating-f64-to-i8 :: (F64 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 893

<a id="value-73617475726174696e672d6636342d746f2d6973697a65"></a>

## `saturating-f64-to-isize`

```cinch
saturating-f64-to-isize :: (F64 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 911

<a id="value-73617475726174696e672d6636342d746f2d753136"></a>

## `saturating-f64-to-u16`

```cinch
saturating-f64-to-u16 :: (F64 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 903

<a id="value-73617475726174696e672d6636342d746f2d753332"></a>

## `saturating-f64-to-u32`

```cinch
saturating-f64-to-u32 :: (F64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 905

<a id="value-73617475726174696e672d6636342d746f2d753634"></a>

## `saturating-f64-to-u64`

```cinch
saturating-f64-to-u64 :: (F64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 907

<a id="value-73617475726174696e672d6636342d746f2d7538"></a>

## `saturating-f64-to-u8`

```cinch
saturating-f64-to-u8 :: (F64 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 901

<a id="value-73617475726174696e672d6636342d746f2d7573697a65"></a>

## `saturating-f64-to-usize`

```cinch
saturating-f64-to-usize :: (F64 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 909

<a id="value-73617475726174696e672d6931362d746f2d693136"></a>

## `saturating-i16-to-i16`

```cinch
saturating-i16-to-i16 :: (I16 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 606

<a id="value-73617475726174696e672d6931362d746f2d693332"></a>

## `saturating-i16-to-i32`

```cinch
saturating-i16-to-i32 :: (I16 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 609

<a id="value-73617475726174696e672d6931362d746f2d693634"></a>

## `saturating-i16-to-i64`

```cinch
saturating-i16-to-i64 :: (I16 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 612

<a id="value-73617475726174696e672d6931362d746f2d6938"></a>

## `saturating-i16-to-i8`

```cinch
saturating-i16-to-i8 :: (I16 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 603

<a id="value-73617475726174696e672d6931362d746f2d6973697a65"></a>

## `saturating-i16-to-isize`

```cinch
saturating-i16-to-isize :: (I16 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 630

<a id="value-73617475726174696e672d6931362d746f2d753136"></a>

## `saturating-i16-to-u16`

```cinch
saturating-i16-to-u16 :: (I16 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 618

<a id="value-73617475726174696e672d6931362d746f2d753332"></a>

## `saturating-i16-to-u32`

```cinch
saturating-i16-to-u32 :: (I16 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 621

<a id="value-73617475726174696e672d6931362d746f2d753634"></a>

## `saturating-i16-to-u64`

```cinch
saturating-i16-to-u64 :: (I16 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 624

<a id="value-73617475726174696e672d6931362d746f2d7538"></a>

## `saturating-i16-to-u8`

```cinch
saturating-i16-to-u8 :: (I16 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 615

<a id="value-73617475726174696e672d6931362d746f2d7573697a65"></a>

## `saturating-i16-to-usize`

```cinch
saturating-i16-to-usize :: (I16 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 627

<a id="value-73617475726174696e672d6933322d746f2d693136"></a>

## `saturating-i32-to-i16`

```cinch
saturating-i32-to-i16 :: (I32 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 636

<a id="value-73617475726174696e672d6933322d746f2d693332"></a>

## `saturating-i32-to-i32`

```cinch
saturating-i32-to-i32 :: (I32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 639

<a id="value-73617475726174696e672d6933322d746f2d693634"></a>

## `saturating-i32-to-i64`

```cinch
saturating-i32-to-i64 :: (I32 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 642

<a id="value-73617475726174696e672d6933322d746f2d6938"></a>

## `saturating-i32-to-i8`

```cinch
saturating-i32-to-i8 :: (I32 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 633

<a id="value-73617475726174696e672d6933322d746f2d6973697a65"></a>

## `saturating-i32-to-isize`

```cinch
saturating-i32-to-isize :: (I32 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 660

<a id="value-73617475726174696e672d6933322d746f2d753136"></a>

## `saturating-i32-to-u16`

```cinch
saturating-i32-to-u16 :: (I32 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 648

<a id="value-73617475726174696e672d6933322d746f2d753332"></a>

## `saturating-i32-to-u32`

```cinch
saturating-i32-to-u32 :: (I32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 651

<a id="value-73617475726174696e672d6933322d746f2d753634"></a>

## `saturating-i32-to-u64`

```cinch
saturating-i32-to-u64 :: (I32 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 654

<a id="value-73617475726174696e672d6933322d746f2d7538"></a>

## `saturating-i32-to-u8`

```cinch
saturating-i32-to-u8 :: (I32 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 645

<a id="value-73617475726174696e672d6933322d746f2d7573697a65"></a>

## `saturating-i32-to-usize`

```cinch
saturating-i32-to-usize :: (I32 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 657

<a id="value-73617475726174696e672d6936342d746f2d693136"></a>

## `saturating-i64-to-i16`

```cinch
saturating-i64-to-i16 :: (I64 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 666

<a id="value-73617475726174696e672d6936342d746f2d693332"></a>

## `saturating-i64-to-i32`

```cinch
saturating-i64-to-i32 :: (I64 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 669

<a id="value-73617475726174696e672d6936342d746f2d693634"></a>

## `saturating-i64-to-i64`

```cinch
saturating-i64-to-i64 :: (I64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 672

<a id="value-73617475726174696e672d6936342d746f2d6938"></a>

## `saturating-i64-to-i8`

```cinch
saturating-i64-to-i8 :: (I64 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 663

<a id="value-73617475726174696e672d6936342d746f2d6973697a65"></a>

## `saturating-i64-to-isize`

```cinch
saturating-i64-to-isize :: (I64 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 690

<a id="value-73617475726174696e672d6936342d746f2d753136"></a>

## `saturating-i64-to-u16`

```cinch
saturating-i64-to-u16 :: (I64 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 678

<a id="value-73617475726174696e672d6936342d746f2d753332"></a>

## `saturating-i64-to-u32`

```cinch
saturating-i64-to-u32 :: (I64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 681

<a id="value-73617475726174696e672d6936342d746f2d753634"></a>

## `saturating-i64-to-u64`

```cinch
saturating-i64-to-u64 :: (I64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 684

<a id="value-73617475726174696e672d6936342d746f2d7538"></a>

## `saturating-i64-to-u8`

```cinch
saturating-i64-to-u8 :: (I64 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 675

<a id="value-73617475726174696e672d6936342d746f2d7573697a65"></a>

## `saturating-i64-to-usize`

```cinch
saturating-i64-to-usize :: (I64 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 687

<a id="value-73617475726174696e672d69382d746f2d693136"></a>

## `saturating-i8-to-i16`

```cinch
saturating-i8-to-i16 :: (I8 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 576

<a id="value-73617475726174696e672d69382d746f2d693332"></a>

## `saturating-i8-to-i32`

```cinch
saturating-i8-to-i32 :: (I8 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 579

<a id="value-73617475726174696e672d69382d746f2d693634"></a>

## `saturating-i8-to-i64`

```cinch
saturating-i8-to-i64 :: (I8 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 582

<a id="value-73617475726174696e672d69382d746f2d6938"></a>

## `saturating-i8-to-i8`

```cinch
saturating-i8-to-i8 :: (I8 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 573

<a id="value-73617475726174696e672d69382d746f2d6973697a65"></a>

## `saturating-i8-to-isize`

```cinch
saturating-i8-to-isize :: (I8 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 600

<a id="value-73617475726174696e672d69382d746f2d753136"></a>

## `saturating-i8-to-u16`

```cinch
saturating-i8-to-u16 :: (I8 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 588

<a id="value-73617475726174696e672d69382d746f2d753332"></a>

## `saturating-i8-to-u32`

```cinch
saturating-i8-to-u32 :: (I8 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 591

<a id="value-73617475726174696e672d69382d746f2d753634"></a>

## `saturating-i8-to-u64`

```cinch
saturating-i8-to-u64 :: (I8 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 594

<a id="value-73617475726174696e672d69382d746f2d7538"></a>

## `saturating-i8-to-u8`

```cinch
saturating-i8-to-u8 :: (I8 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 585

<a id="value-73617475726174696e672d69382d746f2d7573697a65"></a>

## `saturating-i8-to-usize`

```cinch
saturating-i8-to-usize :: (I8 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 597

<a id="value-73617475726174696e672d6973697a652d746f2d693136"></a>

## `saturating-isize-to-i16`

```cinch
saturating-isize-to-i16 :: (Isize -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 846

<a id="value-73617475726174696e672d6973697a652d746f2d693332"></a>

## `saturating-isize-to-i32`

```cinch
saturating-isize-to-i32 :: (Isize -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 849

<a id="value-73617475726174696e672d6973697a652d746f2d693634"></a>

## `saturating-isize-to-i64`

```cinch
saturating-isize-to-i64 :: (Isize -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 852

<a id="value-73617475726174696e672d6973697a652d746f2d6938"></a>

## `saturating-isize-to-i8`

```cinch
saturating-isize-to-i8 :: (Isize -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 843

<a id="value-73617475726174696e672d6973697a652d746f2d6973697a65"></a>

## `saturating-isize-to-isize`

```cinch
saturating-isize-to-isize :: (Isize -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 870

<a id="value-73617475726174696e672d6973697a652d746f2d753136"></a>

## `saturating-isize-to-u16`

```cinch
saturating-isize-to-u16 :: (Isize -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 858

<a id="value-73617475726174696e672d6973697a652d746f2d753332"></a>

## `saturating-isize-to-u32`

```cinch
saturating-isize-to-u32 :: (Isize -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 861

<a id="value-73617475726174696e672d6973697a652d746f2d753634"></a>

## `saturating-isize-to-u64`

```cinch
saturating-isize-to-u64 :: (Isize -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 864

<a id="value-73617475726174696e672d6973697a652d746f2d7538"></a>

## `saturating-isize-to-u8`

```cinch
saturating-isize-to-u8 :: (Isize -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 855

<a id="value-73617475726174696e672d6973697a652d746f2d7573697a65"></a>

## `saturating-isize-to-usize`

```cinch
saturating-isize-to-usize :: (Isize -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 867

<a id="value-73617475726174696e672d6d756c2d693136"></a>

## `saturating-mul-i16`

```cinch
saturating-mul-i16 :: (I16 -> (I16 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 99

<a id="value-73617475726174696e672d6d756c2d693332"></a>

## `saturating-mul-i32`

```cinch
saturating-mul-i32 :: (I32 -> (I32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 138

<a id="value-73617475726174696e672d6d756c2d693634"></a>

## `saturating-mul-i64`

```cinch
saturating-mul-i64 :: (I64 -> (I64 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 177

<a id="value-73617475726174696e672d6d756c2d6938"></a>

## `saturating-mul-i8`

```cinch
saturating-mul-i8 :: (I8 -> (I8 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 60

<a id="value-73617475726174696e672d6d756c2d6973697a65"></a>

## `saturating-mul-isize`

```cinch
saturating-mul-isize :: (Isize -> (Isize -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 411

<a id="value-73617475726174696e672d6d756c2d753136"></a>

## `saturating-mul-u16`

```cinch
saturating-mul-u16 :: (U16 -> (U16 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 255

<a id="value-73617475726174696e672d6d756c2d753332"></a>

## `saturating-mul-u32`

```cinch
saturating-mul-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 294

<a id="value-73617475726174696e672d6d756c2d753634"></a>

## `saturating-mul-u64`

```cinch
saturating-mul-u64 :: (U64 -> (U64 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 333

<a id="value-73617475726174696e672d6d756c2d7538"></a>

## `saturating-mul-u8`

```cinch
saturating-mul-u8 :: (U8 -> (U8 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 216

<a id="value-73617475726174696e672d6d756c2d7573697a65"></a>

## `saturating-mul-usize`

```cinch
saturating-mul-usize :: (Usize -> (Usize -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 372

<a id="value-73617475726174696e672d6e65672d693136"></a>

## `saturating-neg-i16`

```cinch
saturating-neg-i16 :: (I16 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 102

<a id="value-73617475726174696e672d6e65672d693332"></a>

## `saturating-neg-i32`

```cinch
saturating-neg-i32 :: (I32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 141

<a id="value-73617475726174696e672d6e65672d693634"></a>

## `saturating-neg-i64`

```cinch
saturating-neg-i64 :: (I64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 180

<a id="value-73617475726174696e672d6e65672d6938"></a>

## `saturating-neg-i8`

```cinch
saturating-neg-i8 :: (I8 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 63

<a id="value-73617475726174696e672d6e65672d6973697a65"></a>

## `saturating-neg-isize`

```cinch
saturating-neg-isize :: (Isize -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 414

<a id="value-73617475726174696e672d6e65672d753136"></a>

## `saturating-neg-u16`

```cinch
saturating-neg-u16 :: (U16 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 258

<a id="value-73617475726174696e672d6e65672d753332"></a>

## `saturating-neg-u32`

```cinch
saturating-neg-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 297

<a id="value-73617475726174696e672d6e65672d753634"></a>

## `saturating-neg-u64`

```cinch
saturating-neg-u64 :: (U64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 336

<a id="value-73617475726174696e672d6e65672d7538"></a>

## `saturating-neg-u8`

```cinch
saturating-neg-u8 :: (U8 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 219

<a id="value-73617475726174696e672d6e65672d7573697a65"></a>

## `saturating-neg-usize`

```cinch
saturating-neg-usize :: (Usize -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 375

<a id="value-73617475726174696e672d7375622d693136"></a>

## `saturating-sub-i16`

```cinch
saturating-sub-i16 :: (I16 -> (I16 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 98

<a id="value-73617475726174696e672d7375622d693332"></a>

## `saturating-sub-i32`

```cinch
saturating-sub-i32 :: (I32 -> (I32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 137

<a id="value-73617475726174696e672d7375622d693634"></a>

## `saturating-sub-i64`

```cinch
saturating-sub-i64 :: (I64 -> (I64 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 176

<a id="value-73617475726174696e672d7375622d6938"></a>

## `saturating-sub-i8`

```cinch
saturating-sub-i8 :: (I8 -> (I8 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 59

<a id="value-73617475726174696e672d7375622d6973697a65"></a>

## `saturating-sub-isize`

```cinch
saturating-sub-isize :: (Isize -> (Isize -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 410

<a id="value-73617475726174696e672d7375622d753136"></a>

## `saturating-sub-u16`

```cinch
saturating-sub-u16 :: (U16 -> (U16 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 254

<a id="value-73617475726174696e672d7375622d753332"></a>

## `saturating-sub-u32`

```cinch
saturating-sub-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 293

<a id="value-73617475726174696e672d7375622d753634"></a>

## `saturating-sub-u64`

```cinch
saturating-sub-u64 :: (U64 -> (U64 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 332

<a id="value-73617475726174696e672d7375622d7538"></a>

## `saturating-sub-u8`

```cinch
saturating-sub-u8 :: (U8 -> (U8 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 215

<a id="value-73617475726174696e672d7375622d7573697a65"></a>

## `saturating-sub-usize`

```cinch
saturating-sub-usize :: (Usize -> (Usize -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 371

<a id="value-73617475726174696e672d7531362d746f2d693136"></a>

## `saturating-u16-to-i16`

```cinch
saturating-u16-to-i16 :: (U16 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 726

<a id="value-73617475726174696e672d7531362d746f2d693332"></a>

## `saturating-u16-to-i32`

```cinch
saturating-u16-to-i32 :: (U16 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 729

<a id="value-73617475726174696e672d7531362d746f2d693634"></a>

## `saturating-u16-to-i64`

```cinch
saturating-u16-to-i64 :: (U16 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 732

<a id="value-73617475726174696e672d7531362d746f2d6938"></a>

## `saturating-u16-to-i8`

```cinch
saturating-u16-to-i8 :: (U16 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 723

<a id="value-73617475726174696e672d7531362d746f2d6973697a65"></a>

## `saturating-u16-to-isize`

```cinch
saturating-u16-to-isize :: (U16 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 750

<a id="value-73617475726174696e672d7531362d746f2d753136"></a>

## `saturating-u16-to-u16`

```cinch
saturating-u16-to-u16 :: (U16 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 738

<a id="value-73617475726174696e672d7531362d746f2d753332"></a>

## `saturating-u16-to-u32`

```cinch
saturating-u16-to-u32 :: (U16 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 741

<a id="value-73617475726174696e672d7531362d746f2d753634"></a>

## `saturating-u16-to-u64`

```cinch
saturating-u16-to-u64 :: (U16 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 744

<a id="value-73617475726174696e672d7531362d746f2d7538"></a>

## `saturating-u16-to-u8`

```cinch
saturating-u16-to-u8 :: (U16 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 735

<a id="value-73617475726174696e672d7531362d746f2d7573697a65"></a>

## `saturating-u16-to-usize`

```cinch
saturating-u16-to-usize :: (U16 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 747

<a id="value-73617475726174696e672d7533322d746f2d693136"></a>

## `saturating-u32-to-i16`

```cinch
saturating-u32-to-i16 :: (U32 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 756

<a id="value-73617475726174696e672d7533322d746f2d693332"></a>

## `saturating-u32-to-i32`

```cinch
saturating-u32-to-i32 :: (U32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 759

<a id="value-73617475726174696e672d7533322d746f2d693634"></a>

## `saturating-u32-to-i64`

```cinch
saturating-u32-to-i64 :: (U32 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 762

<a id="value-73617475726174696e672d7533322d746f2d6938"></a>

## `saturating-u32-to-i8`

```cinch
saturating-u32-to-i8 :: (U32 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 753

<a id="value-73617475726174696e672d7533322d746f2d6973697a65"></a>

## `saturating-u32-to-isize`

```cinch
saturating-u32-to-isize :: (U32 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 780

<a id="value-73617475726174696e672d7533322d746f2d753136"></a>

## `saturating-u32-to-u16`

```cinch
saturating-u32-to-u16 :: (U32 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 768

<a id="value-73617475726174696e672d7533322d746f2d753332"></a>

## `saturating-u32-to-u32`

```cinch
saturating-u32-to-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 771

<a id="value-73617475726174696e672d7533322d746f2d753634"></a>

## `saturating-u32-to-u64`

```cinch
saturating-u32-to-u64 :: (U32 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 774

<a id="value-73617475726174696e672d7533322d746f2d7538"></a>

## `saturating-u32-to-u8`

```cinch
saturating-u32-to-u8 :: (U32 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 765

<a id="value-73617475726174696e672d7533322d746f2d7573697a65"></a>

## `saturating-u32-to-usize`

```cinch
saturating-u32-to-usize :: (U32 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 777

<a id="value-73617475726174696e672d7536342d746f2d693136"></a>

## `saturating-u64-to-i16`

```cinch
saturating-u64-to-i16 :: (U64 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 786

<a id="value-73617475726174696e672d7536342d746f2d693332"></a>

## `saturating-u64-to-i32`

```cinch
saturating-u64-to-i32 :: (U64 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 789

<a id="value-73617475726174696e672d7536342d746f2d693634"></a>

## `saturating-u64-to-i64`

```cinch
saturating-u64-to-i64 :: (U64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 792

<a id="value-73617475726174696e672d7536342d746f2d6938"></a>

## `saturating-u64-to-i8`

```cinch
saturating-u64-to-i8 :: (U64 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 783

<a id="value-73617475726174696e672d7536342d746f2d6973697a65"></a>

## `saturating-u64-to-isize`

```cinch
saturating-u64-to-isize :: (U64 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 810

<a id="value-73617475726174696e672d7536342d746f2d753136"></a>

## `saturating-u64-to-u16`

```cinch
saturating-u64-to-u16 :: (U64 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 798

<a id="value-73617475726174696e672d7536342d746f2d753332"></a>

## `saturating-u64-to-u32`

```cinch
saturating-u64-to-u32 :: (U64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 801

<a id="value-73617475726174696e672d7536342d746f2d753634"></a>

## `saturating-u64-to-u64`

```cinch
saturating-u64-to-u64 :: (U64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 804

<a id="value-73617475726174696e672d7536342d746f2d7538"></a>

## `saturating-u64-to-u8`

```cinch
saturating-u64-to-u8 :: (U64 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 795

<a id="value-73617475726174696e672d7536342d746f2d7573697a65"></a>

## `saturating-u64-to-usize`

```cinch
saturating-u64-to-usize :: (U64 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 807

<a id="value-73617475726174696e672d75382d746f2d693136"></a>

## `saturating-u8-to-i16`

```cinch
saturating-u8-to-i16 :: (U8 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 696

<a id="value-73617475726174696e672d75382d746f2d693332"></a>

## `saturating-u8-to-i32`

```cinch
saturating-u8-to-i32 :: (U8 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 699

<a id="value-73617475726174696e672d75382d746f2d693634"></a>

## `saturating-u8-to-i64`

```cinch
saturating-u8-to-i64 :: (U8 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 702

<a id="value-73617475726174696e672d75382d746f2d6938"></a>

## `saturating-u8-to-i8`

```cinch
saturating-u8-to-i8 :: (U8 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 693

<a id="value-73617475726174696e672d75382d746f2d6973697a65"></a>

## `saturating-u8-to-isize`

```cinch
saturating-u8-to-isize :: (U8 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 720

<a id="value-73617475726174696e672d75382d746f2d753136"></a>

## `saturating-u8-to-u16`

```cinch
saturating-u8-to-u16 :: (U8 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 708

<a id="value-73617475726174696e672d75382d746f2d753332"></a>

## `saturating-u8-to-u32`

```cinch
saturating-u8-to-u32 :: (U8 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 711

<a id="value-73617475726174696e672d75382d746f2d753634"></a>

## `saturating-u8-to-u64`

```cinch
saturating-u8-to-u64 :: (U8 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 714

<a id="value-73617475726174696e672d75382d746f2d7538"></a>

## `saturating-u8-to-u8`

```cinch
saturating-u8-to-u8 :: (U8 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 705

<a id="value-73617475726174696e672d75382d746f2d7573697a65"></a>

## `saturating-u8-to-usize`

```cinch
saturating-u8-to-usize :: (U8 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 717

<a id="value-73617475726174696e672d7573697a652d746f2d693136"></a>

## `saturating-usize-to-i16`

```cinch
saturating-usize-to-i16 :: (Usize -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 816

<a id="value-73617475726174696e672d7573697a652d746f2d693332"></a>

## `saturating-usize-to-i32`

```cinch
saturating-usize-to-i32 :: (Usize -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 819

<a id="value-73617475726174696e672d7573697a652d746f2d693634"></a>

## `saturating-usize-to-i64`

```cinch
saturating-usize-to-i64 :: (Usize -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 822

<a id="value-73617475726174696e672d7573697a652d746f2d6938"></a>

## `saturating-usize-to-i8`

```cinch
saturating-usize-to-i8 :: (Usize -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 813

<a id="value-73617475726174696e672d7573697a652d746f2d6973697a65"></a>

## `saturating-usize-to-isize`

```cinch
saturating-usize-to-isize :: (Usize -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 840

<a id="value-73617475726174696e672d7573697a652d746f2d753136"></a>

## `saturating-usize-to-u16`

```cinch
saturating-usize-to-u16 :: (Usize -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 828

<a id="value-73617475726174696e672d7573697a652d746f2d753332"></a>

## `saturating-usize-to-u32`

```cinch
saturating-usize-to-u32 :: (Usize -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 831

<a id="value-73617475726174696e672d7573697a652d746f2d753634"></a>

## `saturating-usize-to-u64`

```cinch
saturating-usize-to-u64 :: (Usize -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 834

<a id="value-73617475726174696e672d7573697a652d746f2d7538"></a>

## `saturating-usize-to-u8`

```cinch
saturating-usize-to-u8 :: (Usize -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 825

<a id="value-73617475726174696e672d7573697a652d746f2d7573697a65"></a>

## `saturating-usize-to-usize`

```cinch
saturating-usize-to-usize :: (Usize -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 837

<a id="value-7369676e2d6269742d663332"></a>

## `sign-bit-f32`

```cinch
sign-bit-f32 :: (F32 -> Bool)
```

[Source](/reference-sources/Data.Machine.cinch) · line 464

<a id="value-7369676e2d6269742d663634"></a>

## `sign-bit-f64`

```cinch
sign-bit-f64 :: (F64 -> Bool)
```

[Source](/reference-sources/Data.Machine.cinch) · line 487

<a id="value-737172742d663332"></a>

## `sqrt-f32`

```cinch
sqrt-f32 :: (F32 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 451

<a id="value-737172742d663634"></a>

## `sqrt-f64`

```cinch
sqrt-f64 :: (F64 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 474

<a id="value-7375622d663332"></a>

## `sub-f32`

```cinch
sub-f32 :: (F32 -> (F32 -> F32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 446

<a id="value-7375622d663634"></a>

## `sub-f64`

```cinch
sub-f64 :: (F64 -> (F64 -> F64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 469

<a id="value-737761702d62797465732d693136"></a>

## `swap-bytes-i16`

```cinch
swap-bytes-i16 :: (I16 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 119

<a id="value-737761702d62797465732d693332"></a>

## `swap-bytes-i32`

```cinch
swap-bytes-i32 :: (I32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 158

<a id="value-737761702d62797465732d693634"></a>

## `swap-bytes-i64`

```cinch
swap-bytes-i64 :: (I64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 197

<a id="value-737761702d62797465732d6938"></a>

## `swap-bytes-i8`

```cinch
swap-bytes-i8 :: (I8 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 80

<a id="value-737761702d62797465732d6973697a65"></a>

## `swap-bytes-isize`

```cinch
swap-bytes-isize :: (Isize -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 431

<a id="value-737761702d62797465732d753136"></a>

## `swap-bytes-u16`

```cinch
swap-bytes-u16 :: (U16 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 275

<a id="value-737761702d62797465732d753332"></a>

## `swap-bytes-u32`

```cinch
swap-bytes-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 314

<a id="value-737761702d62797465732d753634"></a>

## `swap-bytes-u64`

```cinch
swap-bytes-u64 :: (U64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 353

<a id="value-737761702d62797465732d7538"></a>

## `swap-bytes-u8`

```cinch
swap-bytes-u8 :: (U8 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 236

<a id="value-737761702d62797465732d7573697a65"></a>

## `swap-bytes-usize`

```cinch
swap-bytes-usize :: (Usize -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 392

<a id="value-746f74616c2d6f726465722d663332"></a>

## `total-order-f32`

```cinch
total-order-f32 :: (F32 -> (F32 -> Ordering))
```

[Source](/reference-sources/Data.Machine.cinch) · line 465

<a id="value-746f74616c2d6f726465722d663634"></a>

## `total-order-f64`

```cinch
total-order-f64 :: (F64 -> (F64 -> Ordering))
```

[Source](/reference-sources/Data.Machine.cinch) · line 488

<a id="value-747261696c696e672d7a65726f732d693136"></a>

## `trailing-zeros-i16`

```cinch
trailing-zeros-i16 :: (I16 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 117

<a id="value-747261696c696e672d7a65726f732d693332"></a>

## `trailing-zeros-i32`

```cinch
trailing-zeros-i32 :: (I32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 156

<a id="value-747261696c696e672d7a65726f732d693634"></a>

## `trailing-zeros-i64`

```cinch
trailing-zeros-i64 :: (I64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 195

<a id="value-747261696c696e672d7a65726f732d6938"></a>

## `trailing-zeros-i8`

```cinch
trailing-zeros-i8 :: (I8 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 78

<a id="value-747261696c696e672d7a65726f732d6973697a65"></a>

## `trailing-zeros-isize`

```cinch
trailing-zeros-isize :: (Isize -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 429

<a id="value-747261696c696e672d7a65726f732d753136"></a>

## `trailing-zeros-u16`

```cinch
trailing-zeros-u16 :: (U16 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 273

<a id="value-747261696c696e672d7a65726f732d753332"></a>

## `trailing-zeros-u32`

```cinch
trailing-zeros-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 312

<a id="value-747261696c696e672d7a65726f732d753634"></a>

## `trailing-zeros-u64`

```cinch
trailing-zeros-u64 :: (U64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 351

<a id="value-747261696c696e672d7a65726f732d7538"></a>

## `trailing-zeros-u8`

```cinch
trailing-zeros-u8 :: (U8 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 234

<a id="value-747261696c696e672d7a65726f732d7573697a65"></a>

## `trailing-zeros-usize`

```cinch
trailing-zeros-usize :: (Usize -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 390

<a id="value-7531362d746f2d663332"></a>

## `u16-to-f32`

```cinch
u16-to-f32 :: (U16 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 555

<a id="value-7531362d746f2d663634"></a>

## `u16-to-f64`

```cinch
u16-to-f64 :: (U16 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 556

<a id="value-7531362d746f2d693332"></a>

## `u16-to-i32`

```cinch
u16-to-i32 :: ((a0 :: U16) -> refined (((0 <= r && r <= 65535) && r == a0)) I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 527

<a id="value-7531362d746f2d693634"></a>

## `u16-to-i64`

```cinch
u16-to-i64 :: ((a0 :: U16) -> refined (((0 <= r && r <= 65535) && r == a0)) I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 528

<a id="value-7531362d746f2d6973697a65"></a>

## `u16-to-isize`

```cinch
u16-to-isize :: ((a0 :: U16) -> refined (((0 <= r && r <= 65535) && r == a0)) Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 533

<a id="value-7531362d746f2d6e6174"></a>

## `u16-to-nat`

```cinch
u16-to-nat :: (U16 -> InRange 0 65535 Nat)
```

[Source](/reference-sources/Data.Machine.cinch) · line 1202

<a id="value-7531362d746f2d753136"></a>

## `u16-to-u16`

```cinch
u16-to-u16 :: (U16 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 529

<a id="value-7531362d746f2d753332"></a>

## `u16-to-u32`

```cinch
u16-to-u32 :: ((a0 :: U16) -> refined (((0 <= r && r <= 65535) && r == a0)) U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 530

<a id="value-7531362d746f2d753634"></a>

## `u16-to-u64`

```cinch
u16-to-u64 :: ((a0 :: U16) -> refined (((0 <= r && r <= 65535) && r == a0)) U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 531

<a id="value-7531362d746f2d7573697a65"></a>

## `u16-to-usize`

```cinch
u16-to-usize :: ((a0 :: U16) -> refined (((0 <= r && r <= 65535) && r == a0)) Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 532

<a id="value-7533322d746f2d663332"></a>

## `u32-to-f32`

```cinch
u32-to-f32 :: (U32 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 557

<a id="value-7533322d746f2d663634"></a>

## `u32-to-f64`

```cinch
u32-to-f64 :: (U32 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 558

<a id="value-7533322d746f2d693634"></a>

## `u32-to-i64`

```cinch
u32-to-i64 :: ((a0 :: U32) -> refined (((0 <= r && r <= 4294967295) && r == a0)) I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 534

<a id="value-7533322d746f2d6973697a65"></a>

## `u32-to-isize`

```cinch
u32-to-isize :: ((a0 :: U32) -> refined (((0 <= r && r <= 4294967295) && r == a0)) Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 538

<a id="value-7533322d746f2d6e6174"></a>

## `u32-to-nat`

```cinch
u32-to-nat :: (U32 -> InRange 0 4294967295 Nat)
```

[Source](/reference-sources/Data.Machine.cinch) · line 1205

<a id="value-7533322d746f2d753332"></a>

## `u32-to-u32`

```cinch
u32-to-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 535

<a id="value-7533322d746f2d753634"></a>

## `u32-to-u64`

```cinch
u32-to-u64 :: ((a0 :: U32) -> refined (((0 <= r && r <= 4294967295) && r == a0)) U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 536

<a id="value-7533322d746f2d7573697a65"></a>

## `u32-to-usize`

```cinch
u32-to-usize :: ((a0 :: U32) -> refined (((0 <= r && r <= 4294967295) && r == a0)) Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 537

<a id="value-7536342d746f2d663332"></a>

## `u64-to-f32`

```cinch
u64-to-f32 :: (U64 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 559

<a id="value-7536342d746f2d663634"></a>

## `u64-to-f64`

```cinch
u64-to-f64 :: (U64 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 560

<a id="value-7536342d746f2d753634"></a>

## `u64-to-u64`

```cinch
u64-to-u64 :: (U64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 539

<a id="value-7536342d746f2d7573697a65"></a>

## `u64-to-usize`

```cinch
u64-to-usize :: ((a0 :: U64) -> refined (r == a0) Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 540

<a id="value-75382d746f2d663332"></a>

## `u8-to-f32`

```cinch
u8-to-f32 :: (U8 -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 553

<a id="value-75382d746f2d663634"></a>

## `u8-to-f64`

```cinch
u8-to-f64 :: (U8 -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 554

<a id="value-75382d746f2d693136"></a>

## `u8-to-i16`

```cinch
u8-to-i16 :: ((a0 :: U8) -> refined (((0 <= r && r <= 255) && r == a0)) I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 518

<a id="value-75382d746f2d693332"></a>

## `u8-to-i32`

```cinch
u8-to-i32 :: ((a0 :: U8) -> refined (((0 <= r && r <= 255) && r == a0)) I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 519

<a id="value-75382d746f2d693634"></a>

## `u8-to-i64`

```cinch
u8-to-i64 :: ((a0 :: U8) -> refined (((0 <= r && r <= 255) && r == a0)) I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 520

<a id="value-75382d746f2d6973697a65"></a>

## `u8-to-isize`

```cinch
u8-to-isize :: ((a0 :: U8) -> refined (((0 <= r && r <= 255) && r == a0)) Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 526

<a id="value-75382d746f2d6e6174"></a>

## `u8-to-nat`

```cinch
u8-to-nat :: (U8 -> InRange 0 255 Nat)
```

The unsigned machine integers as the `Nat` of the same value: the
widening conversion's range is the evidence `nat-from-i64` needs,
and the result keeps it.

[Source](/reference-sources/Data.Machine.cinch) · line 1199

<a id="value-75382d746f2d753136"></a>

## `u8-to-u16`

```cinch
u8-to-u16 :: ((a0 :: U8) -> refined (((0 <= r && r <= 255) && r == a0)) U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 522

<a id="value-75382d746f2d753332"></a>

## `u8-to-u32`

```cinch
u8-to-u32 :: ((a0 :: U8) -> refined (((0 <= r && r <= 255) && r == a0)) U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 523

<a id="value-75382d746f2d753634"></a>

## `u8-to-u64`

```cinch
u8-to-u64 :: ((a0 :: U8) -> refined (((0 <= r && r <= 255) && r == a0)) U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 524

<a id="value-75382d746f2d7538"></a>

## `u8-to-u8`

```cinch
u8-to-u8 :: (U8 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 521

<a id="value-75382d746f2d7573697a65"></a>

## `u8-to-usize`

```cinch
u8-to-usize :: ((a0 :: U8) -> refined (((0 <= r && r <= 255) && r == a0)) Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 525

<a id="value-7573697a652d746f2d663332"></a>

## `usize-to-f32`

```cinch
usize-to-f32 :: (Usize -> F32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 561

<a id="value-7573697a652d746f2d663634"></a>

## `usize-to-f64`

```cinch
usize-to-f64 :: (Usize -> F64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 562

<a id="value-7573697a652d746f2d753634"></a>

## `usize-to-u64`

```cinch
usize-to-u64 :: ((a0 :: Usize) -> refined (r == a0) U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 541

<a id="value-7573697a652d746f2d7573697a65"></a>

## `usize-to-usize`

```cinch
usize-to-usize :: (Usize -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 542

<a id="value-7772617070696e672d6164642d693136"></a>

## `wrapping-add-i16`

```cinch
wrapping-add-i16 :: (I16 -> (I16 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 94

<a id="value-7772617070696e672d6164642d693332"></a>

## `wrapping-add-i32`

```cinch
wrapping-add-i32 :: (I32 -> (I32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 133

<a id="value-7772617070696e672d6164642d693634"></a>

## `wrapping-add-i64`

```cinch
wrapping-add-i64 :: (I64 -> (I64 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 172

<a id="value-7772617070696e672d6164642d6938"></a>

## `wrapping-add-i8`

```cinch
wrapping-add-i8 :: (I8 -> (I8 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 55

<a id="value-7772617070696e672d6164642d6973697a65"></a>

## `wrapping-add-isize`

```cinch
wrapping-add-isize :: (Isize -> (Isize -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 406

<a id="value-7772617070696e672d6164642d753136"></a>

## `wrapping-add-u16`

```cinch
wrapping-add-u16 :: (U16 -> (U16 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 250

<a id="value-7772617070696e672d6164642d753332"></a>

## `wrapping-add-u32`

```cinch
wrapping-add-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 289

<a id="value-7772617070696e672d6164642d753634"></a>

## `wrapping-add-u64`

```cinch
wrapping-add-u64 :: (U64 -> (U64 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 328

<a id="value-7772617070696e672d6164642d7538"></a>

## `wrapping-add-u8`

```cinch
wrapping-add-u8 :: (U8 -> (U8 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 211

<a id="value-7772617070696e672d6164642d7573697a65"></a>

## `wrapping-add-usize`

```cinch
wrapping-add-usize :: (Usize -> (Usize -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 367

<a id="value-7772617070696e672d6931362d746f2d693136"></a>

## `wrapping-i16-to-i16`

```cinch
wrapping-i16-to-i16 :: (I16 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 607

<a id="value-7772617070696e672d6931362d746f2d693332"></a>

## `wrapping-i16-to-i32`

```cinch
wrapping-i16-to-i32 :: (I16 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 610

<a id="value-7772617070696e672d6931362d746f2d693634"></a>

## `wrapping-i16-to-i64`

```cinch
wrapping-i16-to-i64 :: (I16 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 613

<a id="value-7772617070696e672d6931362d746f2d6938"></a>

## `wrapping-i16-to-i8`

```cinch
wrapping-i16-to-i8 :: (I16 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 604

<a id="value-7772617070696e672d6931362d746f2d6973697a65"></a>

## `wrapping-i16-to-isize`

```cinch
wrapping-i16-to-isize :: (I16 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 631

<a id="value-7772617070696e672d6931362d746f2d753136"></a>

## `wrapping-i16-to-u16`

```cinch
wrapping-i16-to-u16 :: (I16 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 619

<a id="value-7772617070696e672d6931362d746f2d753332"></a>

## `wrapping-i16-to-u32`

```cinch
wrapping-i16-to-u32 :: (I16 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 622

<a id="value-7772617070696e672d6931362d746f2d753634"></a>

## `wrapping-i16-to-u64`

```cinch
wrapping-i16-to-u64 :: (I16 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 625

<a id="value-7772617070696e672d6931362d746f2d7538"></a>

## `wrapping-i16-to-u8`

```cinch
wrapping-i16-to-u8 :: (I16 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 616

<a id="value-7772617070696e672d6931362d746f2d7573697a65"></a>

## `wrapping-i16-to-usize`

```cinch
wrapping-i16-to-usize :: (I16 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 628

<a id="value-7772617070696e672d6933322d746f2d693136"></a>

## `wrapping-i32-to-i16`

```cinch
wrapping-i32-to-i16 :: (I32 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 637

<a id="value-7772617070696e672d6933322d746f2d693332"></a>

## `wrapping-i32-to-i32`

```cinch
wrapping-i32-to-i32 :: (I32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 640

<a id="value-7772617070696e672d6933322d746f2d693634"></a>

## `wrapping-i32-to-i64`

```cinch
wrapping-i32-to-i64 :: (I32 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 643

<a id="value-7772617070696e672d6933322d746f2d6938"></a>

## `wrapping-i32-to-i8`

```cinch
wrapping-i32-to-i8 :: (I32 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 634

<a id="value-7772617070696e672d6933322d746f2d6973697a65"></a>

## `wrapping-i32-to-isize`

```cinch
wrapping-i32-to-isize :: (I32 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 661

<a id="value-7772617070696e672d6933322d746f2d753136"></a>

## `wrapping-i32-to-u16`

```cinch
wrapping-i32-to-u16 :: (I32 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 649

<a id="value-7772617070696e672d6933322d746f2d753332"></a>

## `wrapping-i32-to-u32`

```cinch
wrapping-i32-to-u32 :: (I32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 652

<a id="value-7772617070696e672d6933322d746f2d753634"></a>

## `wrapping-i32-to-u64`

```cinch
wrapping-i32-to-u64 :: (I32 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 655

<a id="value-7772617070696e672d6933322d746f2d7538"></a>

## `wrapping-i32-to-u8`

```cinch
wrapping-i32-to-u8 :: (I32 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 646

<a id="value-7772617070696e672d6933322d746f2d7573697a65"></a>

## `wrapping-i32-to-usize`

```cinch
wrapping-i32-to-usize :: (I32 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 658

<a id="value-7772617070696e672d6936342d746f2d693136"></a>

## `wrapping-i64-to-i16`

```cinch
wrapping-i64-to-i16 :: (I64 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 667

<a id="value-7772617070696e672d6936342d746f2d693332"></a>

## `wrapping-i64-to-i32`

```cinch
wrapping-i64-to-i32 :: (I64 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 670

<a id="value-7772617070696e672d6936342d746f2d693634"></a>

## `wrapping-i64-to-i64`

```cinch
wrapping-i64-to-i64 :: (I64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 673

<a id="value-7772617070696e672d6936342d746f2d6938"></a>

## `wrapping-i64-to-i8`

```cinch
wrapping-i64-to-i8 :: (I64 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 664

<a id="value-7772617070696e672d6936342d746f2d6973697a65"></a>

## `wrapping-i64-to-isize`

```cinch
wrapping-i64-to-isize :: (I64 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 691

<a id="value-7772617070696e672d6936342d746f2d753136"></a>

## `wrapping-i64-to-u16`

```cinch
wrapping-i64-to-u16 :: (I64 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 679

<a id="value-7772617070696e672d6936342d746f2d753332"></a>

## `wrapping-i64-to-u32`

```cinch
wrapping-i64-to-u32 :: (I64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 682

<a id="value-7772617070696e672d6936342d746f2d753634"></a>

## `wrapping-i64-to-u64`

```cinch
wrapping-i64-to-u64 :: (I64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 685

<a id="value-7772617070696e672d6936342d746f2d7538"></a>

## `wrapping-i64-to-u8`

```cinch
wrapping-i64-to-u8 :: (I64 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 676

<a id="value-7772617070696e672d6936342d746f2d7573697a65"></a>

## `wrapping-i64-to-usize`

```cinch
wrapping-i64-to-usize :: (I64 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 688

<a id="value-7772617070696e672d69382d746f2d693136"></a>

## `wrapping-i8-to-i16`

```cinch
wrapping-i8-to-i16 :: (I8 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 577

<a id="value-7772617070696e672d69382d746f2d693332"></a>

## `wrapping-i8-to-i32`

```cinch
wrapping-i8-to-i32 :: (I8 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 580

<a id="value-7772617070696e672d69382d746f2d693634"></a>

## `wrapping-i8-to-i64`

```cinch
wrapping-i8-to-i64 :: (I8 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 583

<a id="value-7772617070696e672d69382d746f2d6938"></a>

## `wrapping-i8-to-i8`

```cinch
wrapping-i8-to-i8 :: (I8 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 574

<a id="value-7772617070696e672d69382d746f2d6973697a65"></a>

## `wrapping-i8-to-isize`

```cinch
wrapping-i8-to-isize :: (I8 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 601

<a id="value-7772617070696e672d69382d746f2d753136"></a>

## `wrapping-i8-to-u16`

```cinch
wrapping-i8-to-u16 :: (I8 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 589

<a id="value-7772617070696e672d69382d746f2d753332"></a>

## `wrapping-i8-to-u32`

```cinch
wrapping-i8-to-u32 :: (I8 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 592

<a id="value-7772617070696e672d69382d746f2d753634"></a>

## `wrapping-i8-to-u64`

```cinch
wrapping-i8-to-u64 :: (I8 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 595

<a id="value-7772617070696e672d69382d746f2d7538"></a>

## `wrapping-i8-to-u8`

```cinch
wrapping-i8-to-u8 :: (I8 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 586

<a id="value-7772617070696e672d69382d746f2d7573697a65"></a>

## `wrapping-i8-to-usize`

```cinch
wrapping-i8-to-usize :: (I8 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 598

<a id="value-7772617070696e672d6973697a652d746f2d693136"></a>

## `wrapping-isize-to-i16`

```cinch
wrapping-isize-to-i16 :: (Isize -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 847

<a id="value-7772617070696e672d6973697a652d746f2d693332"></a>

## `wrapping-isize-to-i32`

```cinch
wrapping-isize-to-i32 :: (Isize -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 850

<a id="value-7772617070696e672d6973697a652d746f2d693634"></a>

## `wrapping-isize-to-i64`

```cinch
wrapping-isize-to-i64 :: (Isize -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 853

<a id="value-7772617070696e672d6973697a652d746f2d6938"></a>

## `wrapping-isize-to-i8`

```cinch
wrapping-isize-to-i8 :: (Isize -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 844

<a id="value-7772617070696e672d6973697a652d746f2d6973697a65"></a>

## `wrapping-isize-to-isize`

```cinch
wrapping-isize-to-isize :: (Isize -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 871

<a id="value-7772617070696e672d6973697a652d746f2d753136"></a>

## `wrapping-isize-to-u16`

```cinch
wrapping-isize-to-u16 :: (Isize -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 859

<a id="value-7772617070696e672d6973697a652d746f2d753332"></a>

## `wrapping-isize-to-u32`

```cinch
wrapping-isize-to-u32 :: (Isize -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 862

<a id="value-7772617070696e672d6973697a652d746f2d753634"></a>

## `wrapping-isize-to-u64`

```cinch
wrapping-isize-to-u64 :: (Isize -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 865

<a id="value-7772617070696e672d6973697a652d746f2d7538"></a>

## `wrapping-isize-to-u8`

```cinch
wrapping-isize-to-u8 :: (Isize -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 856

<a id="value-7772617070696e672d6973697a652d746f2d7573697a65"></a>

## `wrapping-isize-to-usize`

```cinch
wrapping-isize-to-usize :: (Isize -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 868

<a id="value-7772617070696e672d6d756c2d693136"></a>

## `wrapping-mul-i16`

```cinch
wrapping-mul-i16 :: (I16 -> (I16 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 96

<a id="value-7772617070696e672d6d756c2d693332"></a>

## `wrapping-mul-i32`

```cinch
wrapping-mul-i32 :: (I32 -> (I32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 135

<a id="value-7772617070696e672d6d756c2d693634"></a>

## `wrapping-mul-i64`

```cinch
wrapping-mul-i64 :: (I64 -> (I64 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 174

<a id="value-7772617070696e672d6d756c2d6938"></a>

## `wrapping-mul-i8`

```cinch
wrapping-mul-i8 :: (I8 -> (I8 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 57

<a id="value-7772617070696e672d6d756c2d6973697a65"></a>

## `wrapping-mul-isize`

```cinch
wrapping-mul-isize :: (Isize -> (Isize -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 408

<a id="value-7772617070696e672d6d756c2d753136"></a>

## `wrapping-mul-u16`

```cinch
wrapping-mul-u16 :: (U16 -> (U16 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 252

<a id="value-7772617070696e672d6d756c2d753332"></a>

## `wrapping-mul-u32`

```cinch
wrapping-mul-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 291

<a id="value-7772617070696e672d6d756c2d753634"></a>

## `wrapping-mul-u64`

```cinch
wrapping-mul-u64 :: (U64 -> (U64 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 330

<a id="value-7772617070696e672d6d756c2d7538"></a>

## `wrapping-mul-u8`

```cinch
wrapping-mul-u8 :: (U8 -> (U8 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 213

<a id="value-7772617070696e672d6d756c2d7573697a65"></a>

## `wrapping-mul-usize`

```cinch
wrapping-mul-usize :: (Usize -> (Usize -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 369

<a id="value-7772617070696e672d6e65672d693136"></a>

## `wrapping-neg-i16`

```cinch
wrapping-neg-i16 :: (I16 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 101

<a id="value-7772617070696e672d6e65672d693332"></a>

## `wrapping-neg-i32`

```cinch
wrapping-neg-i32 :: (I32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 140

<a id="value-7772617070696e672d6e65672d693634"></a>

## `wrapping-neg-i64`

```cinch
wrapping-neg-i64 :: (I64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 179

<a id="value-7772617070696e672d6e65672d6938"></a>

## `wrapping-neg-i8`

```cinch
wrapping-neg-i8 :: (I8 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 62

<a id="value-7772617070696e672d6e65672d6973697a65"></a>

## `wrapping-neg-isize`

```cinch
wrapping-neg-isize :: (Isize -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 413

<a id="value-7772617070696e672d6e65672d753136"></a>

## `wrapping-neg-u16`

```cinch
wrapping-neg-u16 :: (U16 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 257

<a id="value-7772617070696e672d6e65672d753332"></a>

## `wrapping-neg-u32`

```cinch
wrapping-neg-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 296

<a id="value-7772617070696e672d6e65672d753634"></a>

## `wrapping-neg-u64`

```cinch
wrapping-neg-u64 :: (U64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 335

<a id="value-7772617070696e672d6e65672d7538"></a>

## `wrapping-neg-u8`

```cinch
wrapping-neg-u8 :: (U8 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 218

<a id="value-7772617070696e672d6e65672d7573697a65"></a>

## `wrapping-neg-usize`

```cinch
wrapping-neg-usize :: (Usize -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 374

<a id="value-7772617070696e672d73686c2d693136"></a>

## `wrapping-shl-i16`

```cinch
wrapping-shl-i16 :: (I16 -> (U32 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 106

<a id="value-7772617070696e672d73686c2d693332"></a>

## `wrapping-shl-i32`

```cinch
wrapping-shl-i32 :: (I32 -> (U32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 145

<a id="value-7772617070696e672d73686c2d693634"></a>

## `wrapping-shl-i64`

```cinch
wrapping-shl-i64 :: (I64 -> (U32 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 184

<a id="value-7772617070696e672d73686c2d6938"></a>

## `wrapping-shl-i8`

```cinch
wrapping-shl-i8 :: (I8 -> (U32 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 67

<a id="value-7772617070696e672d73686c2d6973697a65"></a>

## `wrapping-shl-isize`

```cinch
wrapping-shl-isize :: (Isize -> (U32 -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 418

<a id="value-7772617070696e672d73686c2d753136"></a>

## `wrapping-shl-u16`

```cinch
wrapping-shl-u16 :: (U16 -> (U32 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 262

<a id="value-7772617070696e672d73686c2d753332"></a>

## `wrapping-shl-u32`

```cinch
wrapping-shl-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 301

<a id="value-7772617070696e672d73686c2d753634"></a>

## `wrapping-shl-u64`

```cinch
wrapping-shl-u64 :: (U64 -> (U32 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 340

<a id="value-7772617070696e672d73686c2d7538"></a>

## `wrapping-shl-u8`

```cinch
wrapping-shl-u8 :: (U8 -> (U32 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 223

<a id="value-7772617070696e672d73686c2d7573697a65"></a>

## `wrapping-shl-usize`

```cinch
wrapping-shl-usize :: (Usize -> (U32 -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 379

<a id="value-7772617070696e672d7368722d693136"></a>

## `wrapping-shr-i16`

```cinch
wrapping-shr-i16 :: (I16 -> (U32 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 108

<a id="value-7772617070696e672d7368722d693332"></a>

## `wrapping-shr-i32`

```cinch
wrapping-shr-i32 :: (I32 -> (U32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 147

<a id="value-7772617070696e672d7368722d693634"></a>

## `wrapping-shr-i64`

```cinch
wrapping-shr-i64 :: (I64 -> (U32 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 186

<a id="value-7772617070696e672d7368722d6938"></a>

## `wrapping-shr-i8`

```cinch
wrapping-shr-i8 :: (I8 -> (U32 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 69

<a id="value-7772617070696e672d7368722d6973697a65"></a>

## `wrapping-shr-isize`

```cinch
wrapping-shr-isize :: (Isize -> (U32 -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 420

<a id="value-7772617070696e672d7368722d753136"></a>

## `wrapping-shr-u16`

```cinch
wrapping-shr-u16 :: (U16 -> (U32 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 264

<a id="value-7772617070696e672d7368722d753332"></a>

## `wrapping-shr-u32`

```cinch
wrapping-shr-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 303

<a id="value-7772617070696e672d7368722d753634"></a>

## `wrapping-shr-u64`

```cinch
wrapping-shr-u64 :: (U64 -> (U32 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 342

<a id="value-7772617070696e672d7368722d7538"></a>

## `wrapping-shr-u8`

```cinch
wrapping-shr-u8 :: (U8 -> (U32 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 225

<a id="value-7772617070696e672d7368722d7573697a65"></a>

## `wrapping-shr-usize`

```cinch
wrapping-shr-usize :: (Usize -> (U32 -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 381

<a id="value-7772617070696e672d7375622d693136"></a>

## `wrapping-sub-i16`

```cinch
wrapping-sub-i16 :: (I16 -> (I16 -> I16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 95

<a id="value-7772617070696e672d7375622d693332"></a>

## `wrapping-sub-i32`

```cinch
wrapping-sub-i32 :: (I32 -> (I32 -> I32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 134

<a id="value-7772617070696e672d7375622d693634"></a>

## `wrapping-sub-i64`

```cinch
wrapping-sub-i64 :: (I64 -> (I64 -> I64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 173

<a id="value-7772617070696e672d7375622d6938"></a>

## `wrapping-sub-i8`

```cinch
wrapping-sub-i8 :: (I8 -> (I8 -> I8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 56

<a id="value-7772617070696e672d7375622d6973697a65"></a>

## `wrapping-sub-isize`

```cinch
wrapping-sub-isize :: (Isize -> (Isize -> Isize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 407

<a id="value-7772617070696e672d7375622d753136"></a>

## `wrapping-sub-u16`

```cinch
wrapping-sub-u16 :: (U16 -> (U16 -> U16))
```

[Source](/reference-sources/Data.Machine.cinch) · line 251

<a id="value-7772617070696e672d7375622d753332"></a>

## `wrapping-sub-u32`

```cinch
wrapping-sub-u32 :: (U32 -> (U32 -> U32))
```

[Source](/reference-sources/Data.Machine.cinch) · line 290

<a id="value-7772617070696e672d7375622d753634"></a>

## `wrapping-sub-u64`

```cinch
wrapping-sub-u64 :: (U64 -> (U64 -> U64))
```

[Source](/reference-sources/Data.Machine.cinch) · line 329

<a id="value-7772617070696e672d7375622d7538"></a>

## `wrapping-sub-u8`

```cinch
wrapping-sub-u8 :: (U8 -> (U8 -> U8))
```

[Source](/reference-sources/Data.Machine.cinch) · line 212

<a id="value-7772617070696e672d7375622d7573697a65"></a>

## `wrapping-sub-usize`

```cinch
wrapping-sub-usize :: (Usize -> (Usize -> Usize))
```

[Source](/reference-sources/Data.Machine.cinch) · line 368

<a id="value-7772617070696e672d7531362d746f2d693136"></a>

## `wrapping-u16-to-i16`

```cinch
wrapping-u16-to-i16 :: (U16 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 727

<a id="value-7772617070696e672d7531362d746f2d693332"></a>

## `wrapping-u16-to-i32`

```cinch
wrapping-u16-to-i32 :: (U16 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 730

<a id="value-7772617070696e672d7531362d746f2d693634"></a>

## `wrapping-u16-to-i64`

```cinch
wrapping-u16-to-i64 :: (U16 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 733

<a id="value-7772617070696e672d7531362d746f2d6938"></a>

## `wrapping-u16-to-i8`

```cinch
wrapping-u16-to-i8 :: (U16 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 724

<a id="value-7772617070696e672d7531362d746f2d6973697a65"></a>

## `wrapping-u16-to-isize`

```cinch
wrapping-u16-to-isize :: (U16 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 751

<a id="value-7772617070696e672d7531362d746f2d753136"></a>

## `wrapping-u16-to-u16`

```cinch
wrapping-u16-to-u16 :: (U16 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 739

<a id="value-7772617070696e672d7531362d746f2d753332"></a>

## `wrapping-u16-to-u32`

```cinch
wrapping-u16-to-u32 :: (U16 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 742

<a id="value-7772617070696e672d7531362d746f2d753634"></a>

## `wrapping-u16-to-u64`

```cinch
wrapping-u16-to-u64 :: (U16 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 745

<a id="value-7772617070696e672d7531362d746f2d7538"></a>

## `wrapping-u16-to-u8`

```cinch
wrapping-u16-to-u8 :: (U16 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 736

<a id="value-7772617070696e672d7531362d746f2d7573697a65"></a>

## `wrapping-u16-to-usize`

```cinch
wrapping-u16-to-usize :: (U16 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 748

<a id="value-7772617070696e672d7533322d746f2d693136"></a>

## `wrapping-u32-to-i16`

```cinch
wrapping-u32-to-i16 :: (U32 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 757

<a id="value-7772617070696e672d7533322d746f2d693332"></a>

## `wrapping-u32-to-i32`

```cinch
wrapping-u32-to-i32 :: (U32 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 760

<a id="value-7772617070696e672d7533322d746f2d693634"></a>

## `wrapping-u32-to-i64`

```cinch
wrapping-u32-to-i64 :: (U32 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 763

<a id="value-7772617070696e672d7533322d746f2d6938"></a>

## `wrapping-u32-to-i8`

```cinch
wrapping-u32-to-i8 :: (U32 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 754

<a id="value-7772617070696e672d7533322d746f2d6973697a65"></a>

## `wrapping-u32-to-isize`

```cinch
wrapping-u32-to-isize :: (U32 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 781

<a id="value-7772617070696e672d7533322d746f2d753136"></a>

## `wrapping-u32-to-u16`

```cinch
wrapping-u32-to-u16 :: (U32 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 769

<a id="value-7772617070696e672d7533322d746f2d753332"></a>

## `wrapping-u32-to-u32`

```cinch
wrapping-u32-to-u32 :: (U32 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 772

<a id="value-7772617070696e672d7533322d746f2d753634"></a>

## `wrapping-u32-to-u64`

```cinch
wrapping-u32-to-u64 :: (U32 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 775

<a id="value-7772617070696e672d7533322d746f2d7538"></a>

## `wrapping-u32-to-u8`

```cinch
wrapping-u32-to-u8 :: (U32 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 766

<a id="value-7772617070696e672d7533322d746f2d7573697a65"></a>

## `wrapping-u32-to-usize`

```cinch
wrapping-u32-to-usize :: (U32 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 778

<a id="value-7772617070696e672d7536342d746f2d693136"></a>

## `wrapping-u64-to-i16`

```cinch
wrapping-u64-to-i16 :: (U64 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 787

<a id="value-7772617070696e672d7536342d746f2d693332"></a>

## `wrapping-u64-to-i32`

```cinch
wrapping-u64-to-i32 :: (U64 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 790

<a id="value-7772617070696e672d7536342d746f2d693634"></a>

## `wrapping-u64-to-i64`

```cinch
wrapping-u64-to-i64 :: (U64 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 793

<a id="value-7772617070696e672d7536342d746f2d6938"></a>

## `wrapping-u64-to-i8`

```cinch
wrapping-u64-to-i8 :: (U64 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 784

<a id="value-7772617070696e672d7536342d746f2d6973697a65"></a>

## `wrapping-u64-to-isize`

```cinch
wrapping-u64-to-isize :: (U64 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 811

<a id="value-7772617070696e672d7536342d746f2d753136"></a>

## `wrapping-u64-to-u16`

```cinch
wrapping-u64-to-u16 :: (U64 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 799

<a id="value-7772617070696e672d7536342d746f2d753332"></a>

## `wrapping-u64-to-u32`

```cinch
wrapping-u64-to-u32 :: (U64 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 802

<a id="value-7772617070696e672d7536342d746f2d753634"></a>

## `wrapping-u64-to-u64`

```cinch
wrapping-u64-to-u64 :: (U64 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 805

<a id="value-7772617070696e672d7536342d746f2d7538"></a>

## `wrapping-u64-to-u8`

```cinch
wrapping-u64-to-u8 :: (U64 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 796

<a id="value-7772617070696e672d7536342d746f2d7573697a65"></a>

## `wrapping-u64-to-usize`

```cinch
wrapping-u64-to-usize :: (U64 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 808

<a id="value-7772617070696e672d75382d746f2d693136"></a>

## `wrapping-u8-to-i16`

```cinch
wrapping-u8-to-i16 :: (U8 -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 697

<a id="value-7772617070696e672d75382d746f2d693332"></a>

## `wrapping-u8-to-i32`

```cinch
wrapping-u8-to-i32 :: (U8 -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 700

<a id="value-7772617070696e672d75382d746f2d693634"></a>

## `wrapping-u8-to-i64`

```cinch
wrapping-u8-to-i64 :: (U8 -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 703

<a id="value-7772617070696e672d75382d746f2d6938"></a>

## `wrapping-u8-to-i8`

```cinch
wrapping-u8-to-i8 :: (U8 -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 694

<a id="value-7772617070696e672d75382d746f2d6973697a65"></a>

## `wrapping-u8-to-isize`

```cinch
wrapping-u8-to-isize :: (U8 -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 721

<a id="value-7772617070696e672d75382d746f2d753136"></a>

## `wrapping-u8-to-u16`

```cinch
wrapping-u8-to-u16 :: (U8 -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 709

<a id="value-7772617070696e672d75382d746f2d753332"></a>

## `wrapping-u8-to-u32`

```cinch
wrapping-u8-to-u32 :: (U8 -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 712

<a id="value-7772617070696e672d75382d746f2d753634"></a>

## `wrapping-u8-to-u64`

```cinch
wrapping-u8-to-u64 :: (U8 -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 715

<a id="value-7772617070696e672d75382d746f2d7538"></a>

## `wrapping-u8-to-u8`

```cinch
wrapping-u8-to-u8 :: (U8 -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 706

<a id="value-7772617070696e672d75382d746f2d7573697a65"></a>

## `wrapping-u8-to-usize`

```cinch
wrapping-u8-to-usize :: (U8 -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 718

<a id="value-7772617070696e672d7573697a652d746f2d693136"></a>

## `wrapping-usize-to-i16`

```cinch
wrapping-usize-to-i16 :: (Usize -> I16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 817

<a id="value-7772617070696e672d7573697a652d746f2d693332"></a>

## `wrapping-usize-to-i32`

```cinch
wrapping-usize-to-i32 :: (Usize -> I32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 820

<a id="value-7772617070696e672d7573697a652d746f2d693634"></a>

## `wrapping-usize-to-i64`

```cinch
wrapping-usize-to-i64 :: (Usize -> I64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 823

<a id="value-7772617070696e672d7573697a652d746f2d6938"></a>

## `wrapping-usize-to-i8`

```cinch
wrapping-usize-to-i8 :: (Usize -> I8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 814

<a id="value-7772617070696e672d7573697a652d746f2d6973697a65"></a>

## `wrapping-usize-to-isize`

```cinch
wrapping-usize-to-isize :: (Usize -> Isize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 841

<a id="value-7772617070696e672d7573697a652d746f2d753136"></a>

## `wrapping-usize-to-u16`

```cinch
wrapping-usize-to-u16 :: (Usize -> U16)
```

[Source](/reference-sources/Data.Machine.cinch) · line 829

<a id="value-7772617070696e672d7573697a652d746f2d753332"></a>

## `wrapping-usize-to-u32`

```cinch
wrapping-usize-to-u32 :: (Usize -> U32)
```

[Source](/reference-sources/Data.Machine.cinch) · line 832

<a id="value-7772617070696e672d7573697a652d746f2d753634"></a>

## `wrapping-usize-to-u64`

```cinch
wrapping-usize-to-u64 :: (Usize -> U64)
```

[Source](/reference-sources/Data.Machine.cinch) · line 835

<a id="value-7772617070696e672d7573697a652d746f2d7538"></a>

## `wrapping-usize-to-u8`

```cinch
wrapping-usize-to-u8 :: (Usize -> U8)
```

[Source](/reference-sources/Data.Machine.cinch) · line 826

<a id="value-7772617070696e672d7573697a652d746f2d7573697a65"></a>

## `wrapping-usize-to-usize`

```cinch
wrapping-usize-to-usize :: (Usize -> Usize)
```

[Source](/reference-sources/Data.Machine.cinch) · line 838

<a id="value-7a65726f2d753332"></a>

## `zero-u32`

```cinch
zero-u32 :: U32
```

The scalar model (platform section 5): the sum modulo 2^32 over the
borrowed vector.

[Source](/reference-sources/Data.Machine.cinch) · line 1015

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Machine.Bits_I16` | `Bits I16` | Implicit |
| `Data.Machine.Bits_I32` | `Bits I32` | Implicit |
| `Data.Machine.Bits_I64` | `Bits I64` | Implicit |
| `Data.Machine.Bits_I8` | `Bits I8` | Implicit |
| `Data.Machine.Bits_Isize` | `Bits Isize` | Implicit |
| `Data.Machine.Bits_U16` | `Bits U16` | Implicit |
| `Data.Machine.Bits_U32` | `Bits U32` | Implicit |
| `Data.Machine.Bits_U64` | `Bits U64` | Implicit |
| `Data.Machine.Bits_U8` | `Bits U8` | Implicit |
| `Data.Machine.Bits_Usize` | `Bits Usize` | Implicit |
| `Data.Machine.Codec_I64` | `Codec I64` | Implicit |
| `Data.Machine.Eq_I16` | `Eq I16` | Implicit |
| `Data.Machine.Eq_I32` | `Eq I32` | Implicit |
| `Data.Machine.Eq_I64` | `Eq I64` | Implicit |
| `Data.Machine.Eq_I8` | `Eq I8` | Implicit |
| `Data.Machine.Eq_Isize` | `Eq Isize` | Implicit |
| `Data.Machine.Eq_U16` | `Eq U16` | Implicit |
| `Data.Machine.Eq_U32` | `Eq U32` | Implicit |
| `Data.Machine.Eq_U64` | `Eq U64` | Implicit |
| `Data.Machine.Eq_U8` | `Eq U8` | Implicit |
| `Data.Machine.Eq_Usize` | `Eq Usize` | Implicit |
| `Data.Machine.JsonCodec_I64` | `JsonCodec I64` | Implicit |
| `Data.Machine.Ord_I16` | `Ord I16` | Implicit |
| `Data.Machine.Ord_I32` | `Ord I32` | Implicit |
| `Data.Machine.Ord_I64` | `Ord I64` | Implicit |
| `Data.Machine.Ord_I8` | `Ord I8` | Implicit |
| `Data.Machine.Ord_Isize` | `Ord Isize` | Implicit |
| `Data.Machine.Ord_U16` | `Ord U16` | Implicit |
| `Data.Machine.Ord_U32` | `Ord U32` | Implicit |
| `Data.Machine.Ord_U64` | `Ord U64` | Implicit |
| `Data.Machine.Ord_U8` | `Ord U8` | Implicit |
| `Data.Machine.Ord_Usize` | `Ord Usize` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Machine.cinch`. Source SHA-256: `c06a8e2d260d6f8bab809c4fda390c7f713b2719c46e3ca170097f0b1e2c7032`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
