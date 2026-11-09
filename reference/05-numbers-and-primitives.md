# Numbers, machine primitives, symbols, and text

Mathematical numbers, machine scalars, text, and byte buffers are different types with different representations and different proof obligations. Operator spellings are library vocabulary; the operation family you call decides the overflow policy. The normative chapter is [primitive types](../specifications/primitive-types-specification.md), amended by the dated notes in `compiler/CONFORMANCE.md`. Signatures below are the checked exports listed under [Data.Machine](../library/Data.Machine.md), [Data.Nat](../library/Data.Nat.md), [Data.Int](../library/Data.Int.md), [Data.NonZero](../library/Data.NonZero.md), [Data.Bytes](../library/Data.Bytes.md), [Data.String](../library/Data.String.md), [Data.Char](../library/Data.Char.md), and [Data.Sym](../library/Data.Sym.md).

# The types the checker knows

| Type | Width | Align | Kind of value |
| --- | --- | --- | --- |
| `Bool` | 8 | 1 | `False` and `True`. Builtin. |
| `Char` | 32 | 4 | One Unicode scalar value. Machine scalar. |
| `I8`, `I16`, `I32`, `I64` | 8, 16, 32, 64 | 1, 2, 4, 8 | Two's complement signed. Machine scalars. |
| `U8`, `U16`, `U32`, `U64` | 8, 16, 32, 64 | 1, 2, 4, 8 | Unsigned. Machine scalars. |
| `F32`, `F64` | 32, 64 | 4, 8 | IEEE 754 binary32 and binary64. Machine scalars. |
| `Usize`, `Isize` | target word | target word | Sizes, indices, pointer differences. Machine scalars. |
| `()` | 0 | 1 | The single value `()`. |
| `Nat` | unbounded | word | Nonnegative exact integer. Builtin, bignum. |
| `Int` | unbounded | word | Signed exact integer. Builtin, bignum. |
| `Bytes` | word handle | 8 | Arbitrary byte buffer. Builtin, counted object. |
| `Sym` | word handle | 8 | Interned name. Builtin, counted object. |
| `Array a`, `Tensor a`, `Columnar row` | element-dependent | element-dependent | Opaque runtime containers. No source constructor. |

`Usize` and `Isize` are the target profile's word width, not the build host's. The native profile is 64-bit, and `max-bound-usize == 18446744073709551615` there; a wasm32 profile makes both 32-bit. Machine scalars are unboxed, `Copy`, and never erased. The width table is also the C ABI table, with `Char` having no C counterpart.

Machine scalars are builtin types the checker installs. So are `Nat`, `Int`, `Bool`, `Unit`, `Bytes`, and `Sym`; [compiler vocabulary](20-compiler-vocabulary.md) lists the whole installed set.

# Mathematical Nat and Int

`Nat` and `Int` are exact. They never take machine overflow behaviour, and arithmetic does not wrap. Both are boxed bignums with an immediate fast path: a `Nat` in `0` through `2^63 - 1` and an `Int` in `-2^62` through `2^62 - 1` travel as one word (`v << 1 | 1`). Those are representation thresholds, not the types' limits. [Type layout](24-type-layout.md) gives the representation details.

| Operation | Signature | Note |
| --- | --- | --- |
| `nat-add`, `nat-mul` | `Nat -> Nat -> Nat` | Total. |
| `nat-sub` | `using (b <= a) => (a :: Nat) -> (b :: Nat) -> Nat` | Exact difference; the caller proves the result is a `Nat`. |
| `nat-div` | `Nat -> NonZero Nat -> Nat` | Total by construction. |
| `nat-rem` | `Nat -> (d :: NonZero Nat) -> refined (r < d) Nat` | Result carries its bound. |
| `checked-div`, `checked-rem` | `Nat -> Nat -> Option Nat` | `None` exactly when the divisor is zero. |
| `saturating-sub` | `Nat -> Nat -> Nat` | `max(a - b, 0)`. |
| `checked-sub` | `Nat -> Nat -> Option Nat` | `None` exactly when `b > a`. |
| `nat-cmp` | `Nat -> Nat -> I64` | Immediate `-1`, `0`, or `1`. |
| `int-add`, `int-sub`, `int-mul` | `Int -> Int -> Int` | Total. |
| `int-div`, `int-rem` | `Int -> NonZero Int -> Int` | Total by construction. |
| `int-neg` | `Int -> Int` | Total. |
| `int-cmp` | `Int -> Int -> I64` | Immediate `-1`, `0`, or `1`. |
| `int-mod-nat` | `Int -> NonZero Nat -> Nat` | Floored residue, always `0 <= r < d`. |

Subtraction at `Nat` is a checked obligation, and an impossible one is refuted at the call rather than at run time:

<!-- cinch:reject contains="the obligation `2 <= 1` of this call is false" -->
```cinch
module Main where
  main :: Int
  main = case (1 - 2 :: Nat) of
    _ -> 0
```

A branch condition supplies the same fact. `if b <= a then a - b else 0` typechecks, and `saturating-sub` is that function under its own name.

# Conversions between the integer families

Every conversion is explicit. There is no implicit widening, narrowing, or signedness change, and the attempt is a type error:

```text
cinch: 4:13: type mismatch: expected U8, got I32
```

The four families follow one naming rule. A total conversion is `<src>-to-<dst>` with both sides lowercase: `u8-to-i32`, `u32-to-nat`, `i8-to-f64`, `i64-to-isize`. A conversion that can fail is `checked-<src>-to-<dst>` and returns `Option`; one that clamps is `saturating-<src>-to-<dst>`; one that truncates in two's complement is `wrapping-<src>-to-<dst>`. Every ordered pair of machine integer types has all four where they are defined, which is the 10 by 10 family in [Data.Machine](../library/Data.Machine.md).

| From a mathematical number | To a mathematical number |
| --- | --- |
| `nat-to-i64` : `using (n <= 9223372036854775807) => (n :: Nat) -> refined (x == n) I64` | `nat-from-i64` : `using (0 <= x) => (x :: I64) -> refined (n == x) Nat` |
| `checked-nat-to-i64` : `Nat -> Option I64` | `checked-nat-from-i64` : `I64 -> Option Nat` |
| `nat-to-int` : `(n :: Nat) -> refined (x == n) Int` | `nat-from-int` : `using (0 <= x) => (x :: Int) -> refined (n == x) Nat` |
| `int-to-i64` : `using (-9223372036854775808 <= x, x <= 9223372036854775807) => (x :: Int) -> refined (r == x) I64` | `int-from-i64` : `(x :: I64) -> refined (r == x) Int` |
| `checked-int-to-i64` : `Int -> Option I64` | `u8-to-nat`, `u16-to-nat`, `u32-to-nat` : total |

# Overflow: three named families

Overflow behaviour is never implicit. For addition, subtraction, multiplication, and negation each machine integer type has three families, and the family is in the name:

| Family | Result | Behaviour on overflow |
| --- | --- | --- |
| `checked-add-u8`, `checked-sub-i32`, `checked-mul-u64`, `checked-neg-i8` | `Option t` | `None`. The wrapped value is never returned. |
| `wrapping-add-u8`, `wrapping-sub-i32`, `wrapping-mul-u16`, `wrapping-neg-i64` | `t` | Two's complement wrap modulo the width. |
| `saturating-add-u8`, `saturating-sub-i32`, `saturating-mul-usize`, `saturating-neg-i8` | `t` | Clamp to `min-bound-*` or `max-bound-*`. |

Bounds are named values: `min-bound-i8`, `max-bound-u8`, `max-bound-usize`, and their siblings. Comparison has one name per type and is total: `eq-i64`, `ne-i32`, `lt-u8`, `le-isize`, `gt-u16`. The comparison family exists for the integer types only; `F32` and `F64` use the IEEE predicates instead.

Machine integer types have no `Add`, `Sub`, `Mul`, or `Neg` instance, because one class method cannot say which family is meant. The operator call is refused by class resolution:

<!-- cinch:reject contains="no instance for Add I32" -->
```cinch
module Main where
  main :: Int
  main = case (1 :: I32) + (2 :: I32) of
    0 -> 1
    _ -> 0
```

Negation of a machine value is the same refusal, reported as `no instance for Neg I8`. A prefix minus on an integer literal is not an operator: it folds into the literal and is range checked with it, so `(-3 :: I8)` is valid and `-x` on a machine variable is not.

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

The last arm is the signed division overflow case: `minBound / -1` has no `I32` quotient, so `checked-div-i32` answers `None` for it as well as for a zero divisor.

# Bit operations and shifts

Bitwise operations are total and carry the type in the name: `bit-and-i16`, `bit-or-u32`, `bit-xor-i64`, `bit-not-u8`. Bit inspection is total and returns a width-typed count where meaningful: `count-ones-i32`, `leading-zeros-u16`, `trailing-zeros-i8`, `reverse-bits-u64`, `swap-bytes-i16`.

Shifts separate the invalid count from the defined wrap:

| Form | Result |
| --- | --- |
| `checked-shl-i64`, `checked-shl-u32` | `Option t`, `None` when the count is at least the width. |
| `wrapping-shl-i64`, `wrapping-shr-u16` | Reduced count modulo the width. |
| `rotate-left-i8`, `rotate-right-u64` | Total, count reduced modulo the width. |

`shr` is arithmetic on `I*` and `Isize` and logical on `U*` and `Usize`. `checked-shl-i64 1 64` is `None` while `wrapping-shl-i64 1 65` is `2`.

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

# Literals and range checking

An integer literal is a decimal digit string, with `0x` folded at lexing and a prefix minus folded into the string. It is elaborated against its expected type. A literal outside the expected type's range is a compile error, and the message names both the literal and the type:

<!-- cinch:reject contains="integer literal 300 is out of range for U8" -->
```cinch
module Main where
  main :: Int
  main = case (300 :: U8) of
    _ -> 0
```

`Nat` and `Int` literals are unbounded. A literal whose type no constraint determines is an error rather than a default, with one narrow exception: when pending literal wants are the only thing blocking resolution it takes the mathematical `Int`. There is no defaulting to `I64` or `F64`, and a literal whose type stays open reports `the type of this integer literal is not determined by context; add a type annotation`.

# Division and NonZero

`/` and `%` are the `Div` and `Rem` class methods, and their second argument is a `NonZero a`, so division is total by construction. `NonZero` is a refinement declared beside the classes: `type NonZero a = refine (n :: a) with NonZero n`. A `NonZero Nat` is its `Nat` at run time, because the witness is erased.

A numeral divisor decides its own predicate, so `n / 10` needs nothing written. Zero is refuted:

<!-- cinch:reject contains="integer literal 0 is not a NonZero Nat" -->
```cinch
module Main where
  main :: Int
  main = case (10 / 0 :: Nat) of
    _ -> 0
```

A variable divisor needs evidence. Either it arrives as a `NonZero` argument, or it comes from a runtime decision whose `Yes` branch carries the witness:

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

`Data.NonZero`'s `non-zero` and `non-zero-nat` are the total constructors returning `Option`, for the same reason. `checked-div` and `checked-rem` are the operations where zero is an ordinary input.

# Operators and fixities

Operators are class methods, and only the declaring module may declare a fixity. These are the declared facts:

| Operator | Fixity | Declaring module | Second argument |
| --- | --- | --- | --- |
| `*` | `infixl 9` | `Algebra.Arithmetic` | `a` |
| `/` | `infixl 9` | `Algebra.Arithmetic` | `NonZero a` |
| `%` | `infixl 9` | `Algebra.Arithmetic` | `NonZero a` |
| `+` | `infixl 8` | `Algebra.Arithmetic` | `a` |
| `-` | `infixl 8` | `Algebra.Arithmetic` | `a`, with `b <= a` at `Nat` |
| `<<`, `>>` | `infixl 7` | `Algebra.Bits` | count |
| `&` | `infixl 6` | `Algebra.Bits` | `a` |
| `^` | `infixl 5` | `Algebra.Bits` | `a` |
| `\|` | `infixl 4` | `Algebra.Bits` | `a` |
| `==`, `/=` | `infix 3` | `Relation.Eq` | `a` |
| `<`, `<=`, `>`, `>=` | `infix 3` | `Relation.Ord` | `a` |
| `&&` | `infixr 2` | `Data.Bool` | `Bool`, short-circuiting |
| `\|\|` | `infixr 1` | `Data.Bool` | `Bool`, short-circuiting |

`&&` and `||` short-circuit because the parser elaborates them to `if`; the parenthesized `(&&)` and `(||)` name the strict methods and are different operations. An operator with no declared fixity and no import is refused with `there is no fixity declared for the operator ... and there is no default for an undeclared operator`.

# Floating point

`F32` and `F64` are IEEE 754 binary32 and binary64. They have no `Eq` and no `Ord` instance and no arithmetic class instance, because those laws do not hold for them. The named operations are the interface: `add-f64`, `sub-f32`, `mul-f64`, `div-f64`, `neg-f32`, `abs-f64`, `sqrt-f64`.

Comparison is the IEEE predicate family, `feq-f64`, `fne-f64`, `flt-f64`, `fle-f64`, `fgt-f64`, `fge-f64`, plus the total order `total-order-f32` and `total-order-f64`. Inspection is total: `is-na-nf64`, `is-infinite-f64`, `is-finite-f64`, `is-normal-f64`, `sign-bit-f32`. `minimum-f64` and `maximum-f64` propagate NaN and distinguish the two zeros.

There is no fractional literal form in the surface syntax:

```text
cinch: 4:13: float literals are not supported in the bootstrap slice (there is no AST node for them)
```

A decimal comes from bytes through `f64-from-decimal :: Bytes -> F64`, and `f64-to-decimal :: F64 -> Bytes` renders one. Both `f32-to-f64` and `f64-to-f32` are total and round per IEEE 754; float to integer is `checked-*` or `saturating-*` only.

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

<span class="support-tag support-tag--planned">Coming soon</span> the unified refined-float design, which plans a revised representation and proof-driven range and finiteness contracts with explicitly selected unsafe IEEE instances. [Float refinements](../design/float-refinements.md) records it. It is not a description of the operations above.

# Char and text

`Char` is a Unicode scalar value, distinct from a byte. `char-code :: Char -> InRange 0 1114111 Nat` is total, and both directions of construction return `Option` because a surrogate is not a scalar value:

| Function | Signature |
| --- | --- |
| `char-code` | `Char -> InRange 0 1114111 Nat` |
| `char-to-u32` | `(a0 :: Char) -> refined ((0 <= r && r <= 1114111) && r == a0) U32` |
| `char-from-code` | `Nat -> Option Char` |
| `u32-to-char` | `U32 -> Option Char` |
| `digit-value`, `hex-digit-value` | `Char -> Option Nat` |
| `to-upper-ascii`, `to-lower-ascii` | `Char -> Char` |

`char-from-code 55296` is `None`, and `char-code 'A'` is 65.

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

`String` is immutable UTF-8 text, a module-private newtype over `Bytes`. `string-to-bytes` encodes and copies out, `string-from-bytes` validates and copies in, and returns `Option`. `string-length` counts characters while `string-byte-length` counts bytes. `pack` and `unpack` convert to and from `List Char`. There is no fractional literal form and no byte-string literal form; a `"..."` literal is a `String`.

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

`Data.Char` and `Data.String` also export the ASCII classification predicates (`is-ascii-digit`, `is-ascii-space`, and the rest) and the byte-level helpers `utf8-*`, which are the reference semantics for the UTF-8 rules.

# Bytes

`Bytes` is arbitrary byte data in a counted object. Read-only static buffers fault on write.

| Function | Signature |
| --- | --- |
| `bytes-new` | `(n :: Nat) -> refined ((b.bytesLen cinch-result) == n) Bytes` |
| `bytes-len` | `Bytes -> InRange 0 9223372036854775807 Nat` |
| `bytes-get` | `using (arg1 < (bytes-len arg0)) => (arg0 :: Bytes) -> (arg1 :: Nat) -> InRange 0 255 Nat` |
| `bytes-set` | `using (arg1 < (bytes-len arg0)) => Bytes -> Nat -> InRange 0 255 Nat -> Unit` |
| `bytes-from-u8s` | `List U8 -> Bytes` |
| `bytes-from-list` | `List (InRange 0 255 Nat) -> Bytes` |
| `bytes-copy-range` | `using ((arg2 + arg4) <= (bytes-len arg0), (arg3 + arg4) <= (bytes-len arg1)) => Bytes -> Bytes -> Nat -> Nat -> Nat -> Unit` |
| `bytes-compare` | `Bytes -> Bytes -> I64` |
| `sha256-bytes` | `Bytes -> Bytes` |

An access needs its bound as evidence. With the buffer's length unknown to the checker, the obligation is unsatisfied and named in full:

```text
cinch: 4:15: the obligation `9 < (Data.Bytes.bytes-len (Data.Bytes.bytes-new 4))` needs `Dec` evidence or a branch fact at this use (discharge it in the `Dec`/`decide` world: `case decide (9 < (Data.Bytes.bytes-len $call-argument:4057)) of Yes w -> ...`, under a branch condition that implies it (`if ... then`, a guard, or the left operand of `&&`), or pass an argument whose refinement implies it): no evidence for `9 < (Data.Bytes.bytes-len (Data.Bytes.bytes-new 4))` is in scope here
```

`bytes-len` and `bytes-get` carry their range as a refinement, so the fact travels to the next operation.

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

`Sym` is a word-sized handle to a process-wide interned name cell. `Eq` is identity, so it is one pointer test; `Ord` compares the name bytes, so the order is deterministic across runs. `intern : String -> Sym` and `sym-name : Sym -> String` are the conversions, and a `'name` literal interns at compile time and is emitted as a constant.

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

A symbol is opaque identity: nothing in the language exposes a cell address, and there is no arithmetic on `Sym`. Type-level promotion of `'foo` is reserved spelling and not implemented; [staging](14-staging-and-macros.md) covers the separate name-quotation context.

# Rational and BigDecimal

Both are exact and both are source-distributed: [Data.Rational](../library/Data.Rational.md) and [Data.BigDecimal](../library/Data.BigDecimal.md) are marked **Ship source**, so an `import` needs the module file beside the entry file under its dotted name, and an import without it reports `module Data.Rational is imported, but Data.Rational.cinch does not exist`.

`Rational` is a normalised integer pair: the denominator is positive, the two are coprime, and the raw constructor is private, so `rational 1 2` and `rational 2 4` produce equal values. `rational : Int -> Int -> Result RationalError Rational` reports `ZeroDenominator`; `div` reports `DivideByZero` and is the only partial operation. `add`, `sub`, and `mul` are total, `from-int` embeds integers, `numerator` and `denominator` observe the pair, and `to-f64` is the documented rounded boundary.

`BigDecimal` is an unscaled `Int` times ten to an `Int` exponent: `big-decimal : Int -> Int -> BigDecimal`, with `unscaled` and `exponent` as observations, `from-string : String -> Result BigDecimalError BigDecimal`, and `add`, `sub`, `mul`, `abs`, `neg` total. `round : Nat -> Rounding -> BigDecimal -> BigDecimal` and `div : Nat -> Rounding -> BigDecimal -> BigDecimal -> Option BigDecimal` carry the rounding mode explicitly, so no quotient is silently rounded.

# Implementation

Machine scalar registration and the conversion families are in `compiler/src/check_decls.rs` and the prelude's `Data/Machine.cinch`; literal range checking and the arithmetic primitives are in `compiler/src/check_terms.rs`, `core.rs`, and `codegen.rs`; the bignum, bytes, and symbol runtimes are in `compiler/runtime/src/`. Suites: `primitives.rs`, `bootstrap_machine_values.rs`, `safe_division.rs`, `safe_subtraction.rs`, `safe_bytes.rs`, `range_facts.rs`, `small_nats.rs`, `string_literals.rs`, `sym.rs`, `lib_rational.rs`, `lib_bigdecimal.rs`.
