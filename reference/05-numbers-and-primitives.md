# Numbers, machine primitives, symbols, and text

Cinch separates unbounded mathematical integers (`Nat`, `Int`) from fixed-width machine scalars (`I8`–`I64`, `U8`–`U64`, `F32`, `F64`, `Isize`, `Usize`, `Char`), UTF-8 text (`String`), byte buffers (`Bytes`), and interned symbols (`Sym`). There are no implicit numeric coercions and no implicit overflow policies: every conversion is explicit, and the operation family you call determines how overflow is handled.

The normative specification is [primitive types](../specifications/primitive-types-specification.md). Full export lists live in [Data.Machine](../library/Data.Machine.md), [Data.Nat](../library/Data.Nat.md), [Data.Int](../library/Data.Int.md), [Data.NonZero](../library/Data.NonZero.md), [Data.Bytes](../library/Data.Bytes.md), [Data.String](../library/Data.String.md), [Data.Char](../library/Data.Char.md), and [Data.Sym](../library/Data.Sym.md).

# Builtin primitive types

| Type | Width (bits) | Alignment (bytes) | Semantics |
| --- | --- | --- | --- |
| `Bool` | 8 | 1 | `False` and `True`. Builtin. |
| `Char` | 32 | 4 | One Unicode scalar value. Machine scalar. |
| `I8`, `I16`, `I32`, `I64` | 8, 16, 32, 64 | 1, 2, 4, 8 | Two's-complement signed integers. Machine scalars. |
| `U8`, `U16`, `U32`, `U64` | 8, 16, 32, 64 | 1, 2, 4, 8 | Unsigned integers. Machine scalars. |
| `F32`, `F64` | 32, 64 | 4, 8 | IEEE 754 binary32 and binary64. Machine scalars. |
| `Usize`, `Isize` | target word | target word | Sizes, indices, pointer offsets. Machine scalars. |
| `Unit` (`()`) | 0 | 1 | Single value `()`. Builtin. |
| `Nat` | unbounded | word | Non-negative exact integer. Tagged immediate / bignum. |
| `Int` | unbounded | word | Signed exact integer. Tagged immediate / bignum. |
| `Bytes` | word handle | 8 | Mutable byte buffer. Reference-counted object. |
| `Sym` | word handle | 8 | Interned symbol. Reference-counted object. |
| `Array a`, `Tensor a`, `Columnar row` | element-dependent | element-dependent | Opaque runtime containers (no surface constructors). |

`Usize` and `Isize` track the **target** profile's word width, not the build host's (64-bit on native 64-bit targets, where `max-bound-usize == 18446744073709551615`; 32-bit on `wasm32`). Machine scalars are unboxed, `Copy`, and never erased. Their widths and alignments match the C ABI, except `Char`, which has no direct C scalar counterpart. See [compiler-provided vocabulary](20-compiler-vocabulary.md) for the complete set of builtin types.

# Exact mathematical numbers: Nat and Int

`Nat` and `Int` represent exact mathematical integers and never wrap or overflow. Both use a tagged single-word fast path (`v << 1 | 1`) for small values — `0` to `2^63 - 1` for `Nat` and `-2^62` to `2^62 - 1` for `Int` — promoting transparently to heap bignums outside those bounds (see [type layout](24-type-layout.md)).

| Operation | Signature | Notes |
| --- | --- | --- |
| `nat-add`, `nat-mul` | `Nat -> Nat -> Nat` | Total. |
| `nat-sub` | `using (b <= a) => (a :: Nat) -> (b :: Nat) -> Nat` | Exact difference; caller proves `b <= a`. |
| `nat-div` | `Nat -> NonZero Nat -> Nat` | Total by construction. |
| `nat-rem` | `Nat -> (d :: NonZero Nat) -> refined (r < d) Nat` | Result carries `r < d`. |
| `checked-div`, `checked-rem` | `Nat -> Nat -> Option Nat` | `None` when the divisor is `0`. |
| `saturating-sub` | `Nat -> Nat -> Nat` | `max(a - b, 0)`. |
| `checked-sub` | `Nat -> Nat -> Option Nat` | `None` when `b > a`. |
| `nat-cmp` | `Nat -> Nat -> I64` | Immediate `-1`, `0`, or `1`. |
| `int-add`, `int-sub`, `int-mul` | `Int -> Int -> Int` | Total. |
| `int-div`, `int-rem` | `Int -> NonZero Int -> Int` | Total by construction. |
| `int-neg` | `Int -> Int` | Total. |
| `int-cmp` | `Int -> Int -> I64` | Immediate `-1`, `0`, or `1`. |
| `int-mod-nat` | `Int -> NonZero Nat -> Nat` | Floored residue (`0 <= r < d`). |

`Nat` subtraction requires `b <= a` as a compile-time obligation; an impossible subtraction is rejected statically:

<!-- cinch:reject contains="the obligation `2 <= 1` of this call is false" -->
```cinch
module Main where
  main :: Int
  main = case (1 - 2 :: Nat) of
    _ -> 0
```

Branch conditions discharge the same obligation automatically (`if b <= a then a - b else 0` typechecks without an explicit `decide`), and `saturating-sub` wraps that pattern as a named function.

# Explicit numeric conversions

Every numeric conversion is explicit. Implicit widening, narrowing, or signedness changes are type errors (`type mismatch: expected U8, got I32`).

Conversion names follow a uniform scheme across all machine integer pairs (documented in full in [Data.Machine](../library/Data.Machine.md)):

- `<src>-to-<dst>` — total lossless conversion (e.g., `u8-to-i32`, `u32-to-nat`, `i8-to-f64`, `i64-to-isize`).
- `checked-<src>-to-<dst>` — returns `Option <dst>`, yielding `None` if out of range.
- `saturating-<src>-to-<dst>` — clamps out-of-range inputs to the target's bounds.
- `wrapping-<src>-to-<dst>` — truncates or reinterprets in two's complement.

Conversions between mathematical integers and machine integers carry proof obligations or refined equalities where applicable:

| From mathematical integer | To mathematical integer |
| --- | --- |
| `nat-to-i64 :: using (n <= 9223372036854775807) => (n :: Nat) -> refined (x == n) I64` | `nat-from-i64 :: using (0 <= x) => (x :: I64) -> refined (n == x) Nat` |
| `checked-nat-to-i64 :: Nat -> Option I64` | `checked-nat-from-i64 :: I64 -> Option Nat` |
| `nat-to-int :: (n :: Nat) -> refined (x == n) Int` | `nat-from-int :: using (0 <= x) => (x :: Int) -> refined (n == x) Nat` |
| `int-to-i64 :: using (-9223372036854775808 <= x, x <= 9223372036854775807) => (x :: Int) -> refined (r == x) I64` | `int-from-i64 :: (x :: I64) -> refined (r == x) Int` |
| `checked-int-to-i64 :: Int -> Option I64` | `u8-to-nat`, `u16-to-nat`, `u32-to-nat` (total) |

# Machine integer arithmetic and overflow policies

Fixed-width machine integers never silently wrap or trap on `+`, `-`, `*`, or unary `-`. Instead, every integer type provides three explicit operation families:

| Family | Examples | Return type | On overflow |
| --- | --- | --- | --- |
| Checked | `checked-add-u8`, `checked-sub-i32`, `checked-mul-u64`, `checked-neg-i8` | `Option t` | Returns `None`. |
| Wrapping | `wrapping-add-u8`, `wrapping-sub-i32`, `wrapping-mul-u16`, `wrapping-neg-i64` | `t` | Two's-complement wrap modulo `2^width`. |
| Saturating | `saturating-add-u8`, `saturating-sub-i32`, `saturating-mul-usize`, `saturating-neg-i8` | `t` | Clamps to `min-bound-*` or `max-bound-*`. |

Extremes are exported as named constants (`min-bound-i8`, `max-bound-u8`, `max-bound-usize`, etc.). Total typed comparisons (`eq-i64`, `ne-i32`, `lt-u8`, `le-isize`, `gt-u16`, ...) exist for all integer types; floats use the IEEE predicate family instead.

Because a single infix operator cannot specify an overflow policy, machine integers intentionally have no `Add`, `Sub`, `Mul`, or `Neg` instances:

<!-- cinch:reject contains="no instance for Add I32" -->
```cinch
module Main where
  main :: Int
  main = case (1 :: I32) + (2 :: I32) of
    0 -> 1
    _ -> 0
```

Prefix `-` on a literal folds into the literal itself and is range-checked statically, so `(-3 :: I8)` is valid while `-x` on a machine variable fails with `no instance for Neg I8`.

Signed division also checks the `minBound / -1` overflow case in addition to division by zero:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main = case checked-add-u8 255 1 of
    None ->
      if wrapping-add-u8 255 1 == 0 then
        if saturating-add-u8 255 1 == 255 then
          case checked-div-i32 7 0 of
            None -> case checked-div-i32 (-2147483648) (-1) of
              None -> 0
              Some q -> 1
            Some q -> 2
        else 3
      else 4
    Some n -> 5
```

# Bitwise operations and shifts

Bitwise operations (`bit-and-i16`, `bit-or-u32`, `bit-xor-i64`, `bit-not-u8`) and bit-inspection primitives (`count-ones-i32`, `leading-zeros-u16`, `trailing-zeros-i8`, `reverse-bits-u64`, `swap-bytes-i16`) are total and carry their type in their name.

Shift operations explicitly separate out-of-range shift counts from masked counts:

| Family | Examples | Semantics |
| --- | --- | --- |
| Checked shift | `checked-shl-i64`, `checked-shl-u32` | Returns `Option t`; `None` when count `>= width`. |
| Wrapping shift | `wrapping-shl-i64`, `wrapping-shr-u16` | Masks shift count modulo `width`. |
| Rotation | `rotate-left-i8`, `rotate-right-u64` | Total; count reduced modulo `width`. |

Right shift (`shr`) is arithmetic (sign-extending) on signed types (`I*`, `Isize`) and logical (zero-filling) on unsigned types (`U*`, `Usize`).

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    case (-3 :: I8) of
      n -> if n == -3 then
             case checked-shl-i64 1 64 of
               None -> case checked-add-i8 127 1 of
                 None -> case wrapping-shl-i64 1 65 of
                   r -> if r == 2 then 0 else 1
                 Some x -> 2
               Some x -> 3
           else 4
```

# Integer literals and range checking

Integer literals (decimal, `0x` hexadecimal, or negative signed literals) are elaborated against their expected type and range-checked at compile time:

<!-- cinch:reject contains="integer literal 300 is out of range for U8" -->
```cinch
module Main where
  main :: Int
  main = case (300 :: U8) of
    _ -> 0
```

`Nat` and `Int` literals are unbounded. Cinch never silently defaults an unconstrained literal to `I64` or `F64`; when no context constrains a literal (other than pure literal constraints resolvable to exact `Int`), the compiler rejects it with `the type of this integer literal is not determined by context; add a type annotation`.

# Total division and NonZero

The `Div` and `Rem` operators `/` and `%` take a `NonZero a` divisor (`type NonZero a = refine (n :: a) with NonZero n`). Because the witness is erased `Prop` evidence, `NonZero Nat` has the same runtime representation as `Nat`, and division by zero is ruled out statically.

Non-zero numeral divisors satisfy the predicate automatically (`n / 10`); literal zero is rejected at compile time:

<!-- cinch:reject contains="integer literal 0 is not a NonZero Nat" -->
```cinch
module Main where
  main :: Int
  main = case (10 / 0 :: Nat) of
    _ -> 0
```

For runtime divisors, pass a `NonZero` value, use a branch fact (`if d /= 0`), or branch on `decide (NonZero d)`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  safeDiv :: Nat -> Nat -> Option Nat
  safeDiv n d = case decide (NonZero d) of
    Yes w -> Some (nat-div n (NonZero d w))
    No f -> None

  main :: Int
  main = case safeDiv 10 2 of
    Some q -> case q of
      5 -> 0
      _ -> 1
    None -> 2
```

At input boundaries, [Data.NonZero](../library/Data.NonZero.md) exports `non-zero :: Int -> Option (NonZero Int)` and `non-zero-nat :: Nat -> Option (NonZero Nat)`; `checked-div` and `checked-rem` accept zero directly and return `Option`.

# Standard numeric and boolean operators

| Operator | Fixity | Declaring module | Notes |
| --- | --- | --- | --- |
| `*` | `infixl 9` | `Algebra.Arithmetic` | Exact on `Nat`/`Int`. |
| `/` | `infixl 9` | `Algebra.Arithmetic` | Divisor is `NonZero a`. |
| `%` | `infixl 9` | `Algebra.Arithmetic` | Divisor is `NonZero a`. |
| `+` | `infixl 8` | `Algebra.Arithmetic` | Exact on `Nat`/`Int`. |
| `-` | `infixl 8` | `Algebra.Arithmetic` | Exact on `Int`; requires `b <= a` on `Nat`. |
| `<<`, `>>` | `infixl 7` | `Algebra.Bits` | Bit shifts. |
| `&` | `infixl 6` | `Algebra.Bits` | Bitwise AND. |
| `^` | `infixl 5` | `Algebra.Bits` | Bitwise XOR. |
| `\|` | `infixl 4` | `Algebra.Bits` | Bitwise OR. |
| `==`, `/=` | `infix 3` | `Relation.Eq` | Non-associative equality. |
| `<`, `<=`, `>`, `>=` | `infix 3` | `Relation.Ord` | Non-associative ordering. |
| `&&` | `infixr 2` | `Data.Bool` | Short-circuiting boolean AND. |
| `\|\|` | `infixr 1` | `Data.Bool` | Short-circuiting boolean OR. |

Infix `&&` and `||` short-circuit by desugaring to `if`; parenthesized `(&&)` and `(||)` refer to the strict two-argument functions. Operators without a declared or imported fixity are rejected.

# Floating-point numbers

`F32` and `F64` implement IEEE 754 binary32 and binary64. Because NaN violates reflexivity, transitivity, and algebraic laws, floating-point types deliberately have no `Eq`, `Ord`, or arithmetic type class instances. Use the explicit named operations instead:

- **Arithmetic:** `add-f64`, `sub-f32`, `mul-f64`, `div-f64`, `neg-f32`, `abs-f64`, `sqrt-f64`, `minimum-f64`, `maximum-f64` (`minimum`/`maximum` propagate NaN and distinguish `-0.0` from `+0.0`).
- **Comparisons:** IEEE predicates `feq-f64`, `fne-f64`, `flt-f64`, `fle-f64`, `fgt-f64`, `fge-f64`, plus IEEE 754 total ordering `total-order-f32` and `total-order-f64`.
- **Classification:** `is-na-nf64`, `is-infinite-f64`, `is-finite-f64`, `is-normal-f64`, `sign-bit-f32`.
- **Conversions:** `f32-to-f64` and `f64-to-f32` (total, IEEE rounding); float-to-integer conversions exist only in `checked-*` and `saturating-*` forms.

Floating-point literals are not part of the surface syntax. Parse decimal text with `f64-from-decimal :: Bytes -> F64` and format with `f64-to-decimal :: F64 -> Bytes`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  zero :: F64
  zero = f64-from-decimal (string-to-bytes "0")

  main :: Int
  main =
    let a = f64-from-decimal (string-to-bytes "1.5")
        b = f64-from-decimal (string-to-bytes "2.25")
        s = add-f64 a b
    in if feq-f64 s (f64-from-decimal (string-to-bytes "3.75")) then
         if is-na-nf64 (div-f64 zero zero) then
           if is-infinite-f64 (div-f64 a zero) then 0 else 1
         else 2
       else 3
```

<span class="support-tag support-tag--planned">Coming soon</span> Refined floating-point contracts (finiteness, range proofs, and opt-in unsafe IEEE instances) are designed in [float refinements](../design/float-refinements.md).

# Characters and UTF-8 text

`Char` represents a Unicode scalar value (`0`–`0x10FFFF`, excluding UTF-16 surrogates `0xD800`–`0xDFFF`). Construction from integers is checked and returns `Option Char`:

| Function | Signature |
| --- | --- |
| `char-code` | `Char -> InRange 0 1114111 Nat` |
| `char-to-u32` | `(c :: Char) -> refined ((0 <= r && r <= 1114111) && r == c) U32` |
| `char-from-code` | `Nat -> Option Char` |
| `u32-to-char` | `U32 -> Option Char` |
| `digit-value`, `hex-digit-value` | `Char -> Option Nat` |
| `to-upper-ascii`, `to-lower-ascii` | `Char -> Char` |

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main = case char-from-code 55296 of
    None -> case u32-to-char 55296 of
      None -> case char-from-code 65 of
        Some c -> if char-code c == 65 then 0 else 1
        None -> 2
      Some c -> 3
    Some c -> 4
```

`String` is immutable UTF-8 text (a module-private wrapper over `Bytes`). String literals (`"..."`) always have type `String`; there is no separate byte-string literal.

- `string-to-bytes :: String -> Bytes` copies out the UTF-8 bytes; `string-from-bytes :: Bytes -> Option String` validates UTF-8 and copies in.
- `string-length :: String -> Nat` counts Unicode scalar values; `string-byte-length :: String -> Nat` counts UTF-8 bytes.
- `pack :: List Char -> String` and `unpack :: String -> List Char` convert to and from character lists.
- [Data.Char](../library/Data.Char.md) and [Data.String](../library/Data.String.md) also export ASCII predicates (`is-ascii-digit`, `is-ascii-space`, ...) and low-level `utf8-*` helpers.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let s = "h\u{00e9}llo"
    in if string-length s == 5 then
         case string-from-bytes (string-to-bytes s) of
           Some t -> if t == s then
                       case string-char-at s 1 of
                         Some c -> if char-code c == 233 then 0 else 1
                         None -> 2
                     else 3
           None -> 4
       else 5
```

# Byte buffers

`Bytes` is a reference-counted mutable buffer of raw bytes with fixed allocation length. Static read-only buffers fault if mutated. Every indexed access requires compile-time bounds evidence, and returned lengths and byte values carry range refinements forward:

| Function | Signature |
| --- | --- |
| `bytes-new` | `(n :: Nat) -> refined (bytes-len r == n) Bytes` |
| `bytes-len` | `Bytes -> InRange 0 9223372036854775807 Nat` |
| `bytes-get` | `using (i < bytes-len b) => (b :: Bytes) -> (i :: Nat) -> InRange 0 255 Nat` |
| `bytes-set` | `using (i < bytes-len b) => (b :: Bytes) -> (i :: Nat) -> InRange 0 255 Nat -> Unit` |
| `bytes-from-u8s` | `List U8 -> Bytes` |
| `bytes-from-list` | `List (InRange 0 255 Nat) -> Bytes` |
| `bytes-copy-range` | `using (di + n <= bytes-len dst, si + n <= bytes-len src) => (dst :: Bytes) -> (src :: Bytes) -> (di :: Nat) -> (si :: Nat) -> (n :: Nat) -> Unit` |
| `bytes-compare` | `Bytes -> Bytes -> I64` |
| `sha256-bytes` | `Bytes -> Bytes` |

Calling `bytes-get` or `bytes-set` without proving `i < bytes-len b` is rejected with a diagnostic naming the missing obligation and the ways to discharge it (`decide`, a branch condition, or a refined argument). Because `bytes-len b == 3` in the `then` branch below implies `1 < bytes-len b`, no separate bounds check is needed for `bytes-get b 1`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let b = bytes-from-u8s (Cons 65 (Cons 66 (Cons 67 Nil)))
    in if bytes-len b == 3 then
         case bytes-get b 1 of
           n -> if n == 66 then 0 else 1
       else 2
```

# Interned symbols

`Sym` is a word-sized handle to a process-wide interned name. `Eq` compares pointer identity in O(1); `Ord` compares the underlying name bytes lexicographically, making ordering deterministic across runs.

- `intern :: String -> Sym` interns a string dynamically.
- `sym-name :: Sym -> String` recovers the symbol's text.
- Symbol literals (`'foo`) are interned at compile time and emitted as constants.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    if 'foo == intern "foo" then
      if sym-name 'foo == "foo" then
        if ('bar < 'foo) then 0 else 1
      else 2
    else 3
```

Symbols are opaque identities: their addresses are never exposed, and they support no arithmetic. Type-level symbol promotion is reserved syntax; see [staging and macros](14-staging-and-macros.md) for compile-time name quotation.

# Exact Rationals and BigDecimals

[Data.Rational](../library/Data.Rational.md) and [Data.BigDecimal](../library/Data.BigDecimal.md) provide exact arbitrary-precision fractions and base-10 decimals. Both are source-distributed modules: importing them requires `Data.Rational.cinch` or `Data.BigDecimal.cinch` beside the entry module (otherwise compilation fails with `module Data.Rational is imported, but Data.Rational.cinch does not exist`).

- **`Rational`** maintains a normalized coprime `Int` pair with a strictly positive denominator behind a private constructor, so `rational 1 2` and `rational 2 4` produce equal values. Construction via `rational :: Int -> Int -> Result RationalError Rational` rejects zero denominators (`ZeroDenominator`); `div` rejects division by zero (`DivideByZero`); `add`, `sub`, `mul`, `from-int`, `numerator`, and `denominator` are total; `to-f64` explicitly rounds to `F64`.
- **`BigDecimal`** pairs an unscaled `Int` mantissa with an `Int` base-10 exponent: `big-decimal :: Int -> Int -> BigDecimal`, observed with `unscaled` and `exponent`, and parsed via `from-string :: String -> Result BigDecimalError BigDecimal`. `add`, `sub`, `mul`, `abs`, and `neg` are total; `round :: Nat -> Rounding -> BigDecimal -> BigDecimal` and `div :: Nat -> Rounding -> BigDecimal -> BigDecimal -> Option BigDecimal` require an explicit `Rounding` mode so quotients are never silently rounded.
