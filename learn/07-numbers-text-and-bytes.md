# 7. Numbers, text, and bytes

Numeric and text types encode different meanings. `Int` is not a machine word, a `String` is not an arbitrary byte buffer, and an unchecked narrowing conversion is not an ascription. Choosing the right type at a boundary avoids surprises later.

# Mathematical and machine arithmetic

`Nat` has no negative values; `Int` has both signs. Their representation grows for large values, subject to resource limits. Fixed-width integers are separate: `I8` through `I64`, `U8` through `U64`, plus target-sized `Isize` and `Usize`. The signed range of `I8` is −128 through 127; `U8` ranges from 0 through 255.

Machine arithmetic makes the overflow policy explicit in the operation name:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main = case checked-add-u8 255 1 of
    Some _ -> 1
    None -> if wrapping-add-u8 255 1 == 0 then 0 else 2
```

The checked addition returns `None` because 256 is outside `U8`. The wrapping addition returns 0 modulo 256. Saturating operations clamp at the relevant bound. Choose the policy that your data model needs; the compiler does not silently choose one for ordinary machine overflow. See [numeric primitives](../reference/05-numbers-and-primitives.md) for families, division, conversions, and floating-point status.

`Nat` subtraction also needs attention: `a - b` needs evidence that `b <= a`. The branch `if n == 0 then ... else n - 1` supplies information the checker can use. For an input whose relationship is unknown, check the relationship or use an operation that returns a failure value.

# Unicode and buffers

`Char` denotes a Unicode scalar value, not a single UTF-8 byte. A `String` is text. `Bytes` can hold any bytes, including invalid UTF-8. `string-to-bytes` encodes a string as UTF-8; `string-from-bytes` returns `Option String`, so invalid input does not turn into invented text.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main = case string-from-bytes (string-to-bytes "Cinch") of
    None -> 1
    Some text -> if text == "Cinch" then 0 else 2
```

Byte indexing needs a proof that the index is below the buffer length. A dynamic reader should compare its index against `bytes-len` and read only in the successful branch. Byte construction also enforces the 0–255 range. These checks make invalid buffer operations a typechecking concern at their use sites.

# Formatting is a conversion

Use `int-to-string` and `nat-to-string` to format mathematical numbers. Import `Text.Show` to use its generic `Show` vocabulary. Other codecs, builders, parsers, and structured formats have their own modules. `Text.Builder` is useful when producing substantial text from many fragments; repeated concatenation can copy growing strings repeatedly.

# Exercise and answer

Why should a network payload first have type `Bytes`, even if you expect it to contain text?

<details><summary>Answer</summary>

The peer can send invalid UTF-8. Keep the bytes until decoding succeeds, handle `None` explicitly, and only then give the result a text interpretation. A type ascription cannot validate the encoding.

</details>
