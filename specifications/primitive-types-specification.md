# Primitive types specification

Source: local draft in this repository (added after the documentation export)  
Added: 2026-10-22

---

[Documentation overview](../overview.md)

Draft 0.1. This chapter defines the primitive, unboxed types: the fixed-width machine scalars that executable code manipulates directly. It extends draft 0.2 and makes concrete the core specification's requirement that "machine integer types use explicit checked, wrapping, or saturating operations". MUST denotes a requirement of this draft. Proposed choices are identified explicitly; unresolved items are listed in Open decisions.

# 1 Scope and role

Three families of types must not be confused:

| Family | Examples | Semantics |
| --- | --- | --- |
| Mathematical | `Nat`, `Int` | Unbounded; exact, total arithmetic; see the core specification |
| Primitive machine scalars | `I32`, `U64`, `F64`, `Char`, `Bool` | This chapter: fixed width, unboxed, explicitly chosen overflow behavior |
| Structured and target-indexed | `Vec n U32`, closures, records, dictionaries | Other chapters; not primitives |

A primitive value is **unboxed**: it occupies its stated fixed layout, is passed, stored and compared inline, and involves no indirection, allocation or ownership transfer. Primitive types are ordinary types in `Type`; they carry no proofs and are never erased.

Dependent mathematics keeps using `Nat` and `Int`. A machine integer is not an `Int`: converting between them is explicit (section 7), and `Nat` and `Int` never acquire machine overflow behavior.

# 2 The type set

| Type | Width (bits) | Align | Values |
| --- | --- | --- | --- |
| `Bool` | 8 | 1 | `False`, `True` |
| `Char` | 32 | 4 | Unicode scalar values |
| `I8`, `I16`, `I32`, `I64` | 8, 16, 32, 64 | 1, 2, 4, 8 | Two's complement signed |
| `U8`, `U16`, `U32`, `U64` | 8, 16, 32, 64 | 1, 2, 4, 8 | Unsigned |
| `F32`, `F64` | 32, 64 | 4, 8 | IEEE 754 binary32 and binary64 |
| `Usize`, `Isize` | target word | target word | Unsigned and signed; see below |
| `()` | 0 | 1 | The single value `()` |

`Usize` and `Isize` are the target-width machine words: their width and alignment are fields of the target profile (see [Platform proofs and SIMD specification](platform-proofs-and-simd-specification.md) section 1), MUST be at least 32 bits, and MUST be equal to each other on a given target. They are the types of sizes, indices and pointer differences. A program compiled for wasm32 has 32-bit `Usize`; a program with a 64-bit memory model has 64-bit `Usize`. Cross-compilation MUST use the target's width, never the build host's.

`I128` and `U128` are an OPTIONAL extension (Open decisions). Where supported they follow every rule below with width 128.

# 3 Unboxed semantics and layout

- A primitive type's size and alignment are exactly as stated in section 2, on every target. They are part of the language, not of an implementation's discretion, because [Foreign function interface specification](foreign-function-interface-specification.md) bindings and raw memory layouts depend on them.
- Arrays and `Vec n t` of a primitive type `t` have element stride equal to `sizeof t` with no padding.
- Primitive values are `Copy`: they may be duplicated and dropped freely (see the core specification, section 7). They impose no ownership or lifetime obligations.
- Evaluation of primitive operations follows the core rules: strict, left-to-right, call-by-value.
- Byte order is not observable through primitive values. It becomes observable only through explicit serialization and raw memory APIs, which state the order they use.

# 4 Integer arithmetic

Overflow behavior is never implicit. For every integer type `t` and each of addition, subtraction, multiplication and negation, the prelude provides exactly three families, named for their behavior:

| Family | Signature shape | Behavior on overflow |
| --- | --- | --- |
| `checked` | `checkedAdd : t -> t -> Option t` | `None`; the wrapped value is never returned |
| `wrapping` | `wrappingAdd : t -> t -> t` | Two's complement wrap modulo `2^n` |
| `saturating` | `saturatingAdd : t -> t -> t` | Clamp to `minBound` or `maxBound` |

The families are `checkedAdd`, `wrappingAdd`, `saturatingAdd`, and likewise `Sub`, `Mul`, `Neg`. `U32.wrappingAdd` (used as the worked example in [Platform proofs and SIMD specification](platform-proofs-and-simd-specification.md) section 5) is this operation: the scalar model is the sum modulo `2^32`. `checkedNeg` returns `None` for `minBound`; `wrappingNeg` and `saturatingNeg` follow their family. Bounds are `minBound, maxBound : t`.

Division and remainder exist only as `checkedDiv, checkedRem : t -> t -> Option t`. They return `None` when the divisor is zero and, for signed types, when the quotient `minBound / -1` would overflow. Quotients truncate toward zero; the remainder satisfies `x = checkedDiv x y * y + checkedRem x y` when both are `Some`. Flooring variants are library functions, not primitives.

Bitwise operations are total: `bitAnd, bitOr, bitXor, bitNot : t -> t -> t` (or `t` for `bitNot`). Shifts distinguish invalid counts from defined wraparound:

- `checkedShl, checkedShr : t -> U32 -> Option t` return `None` when the count is at least the width of `t`.
- `wrappingShl, wrappingShr : t -> U32 -> t` reduce the count modulo the width.
- `rotateLeft, rotateRight : t -> U32 -> t` are total and reduce the count modulo the width.
- `shr` is arithmetic (sign-propagating) on `I*` and `Isize`, logical (zero-filling) on `U*` and `Usize`.

Bit inspection is total: `countOnes, leadingZeros, trailingZeros, reverseBits, swapBytes : t -> U32` where meaningful (`swapBytes` is identity on one-byte types).

There is **no** `Add`, `Sub`, `Mul` or `Neg` instance for machine integer types: `x + y` on `I32` MUST be rejected, because a class method has a single result type and cannot express which overflow family is meant. The prelude MAY offer opt-in wrapper newtypes whose instances commit to one family:

```haskell
-- Proposed prelude wrapper. `Wrapping I32` is a newtype over `I32` whose
-- `Add` instance commits to two's complement wrap; the compiler never
-- chooses an overflow family silently.
-- instance Add (Wrapping I32) where
--   (+) x y = wrap (wrappingAdd (unwrap x) (unwrap y))
```

# 5 Floating point

`F32` and `F64` implement IEEE 754 binary32 and binary64 arithmetic: `add, sub, mul, div, neg, abs, sqrt` with round-to-nearest ties-to-even as the default rounding direction. Signed zeros, infinities and NaNs follow IEEE 754. NaN payloads are unspecified; operations propagate a quiet NaN but which one is not observable.

Floats have **no** `Eq` and **no** `Ord` instance: `==` and `<` on `F64` MUST be rejected, because IEEE equality is not an equivalence relation. The prelude provides the IEEE comparisons `feq, fne, flt, fle, fgt, fge : F64 -> F64 -> Bool` (quiet, non-signaling) and a total order:

```haskell
totalOrder :: F64 -> F64 -> Ordering
-- IEEE 754-2019 totalOrder: -NaN < -inf < ... < -0 < +0 < ... < +inf < +NaN
```

`isNaN, isInfinite, isFinite, isNormal, signBit : F64 -> Bool` are total. `minimum, maximum : F64 -> F64 -> F64` are the IEEE 754-2019 `minimum` and `maximum`: they propagate NaN and distinguish `-0` from `+0`.

Floating-point status flags and the dynamic rounding mode are not observable in this draft (Open decisions). No transformation may reassociate, contract or re-round floating-point computation without a separate contract of the kind [Platform proofs and SIMD specification](platform-proofs-and-simd-specification.md) section 5 requires.

# 6 Bool and Char

`Bool` has exactly the values `False` and `True`. The total functions `not : Bool -> Bool` and `and, or, xor : Bool -> Bool -> Bool` are the strict boolean algebra.

The surface operators `&&` and `||` MUST short-circuit, as required by the core specification. They therefore elaborate to `if` expressions, not to method application; their right operand is not evaluated when the result is already determined. A class-method operator is always strict, so a hypothetical `(&&)` method is a different, strict operation and MUST NOT be given the short-circuit meaning. (The compiler implements this: the parser elaborates infix `&&`/`||` to `if` at parse time, while parenthesized `(&&)`/`(||)` still name the strict prelude methods.)

`Char` values are Unicode scalar values: the code points `0x0` to `0xD7FF` and `0xE000` to `0x10FFFF`. `Char.toU32 : Char -> U32` is total; `U32.checkedToChar` (section 7) rejects surrogates. `Char` has `Eq` and `Ord` in code point order. Character literals and their escapes are defined by [Surface syntax and grammar](surface-syntax-and-grammar.md).

# 7 Conversions and literals

All conversions between primitive types are explicit; there is no implicit widening, narrowing or signedness change. For every ordered pair of numeric primitive types `S` and `T` the prelude provides:

- `S.toT : S -> T` when every value of `S` is representable in `T` (widening, or `U8 -> I32` style cases). This conversion is total.
- `S.checkedToT : S -> Option T` otherwise, returning `None` when the value is not representable.
- `S.saturatingToT : S -> T` clamping to the representable range.
- `S.wrappingToT : S -> T` truncating in two's complement, defined between integer types only.

Float-to-integer conversion is never total: `checkedToT` returns `None` for NaN and out-of-range values, `saturatingToT` clamps and maps NaN to zero. Integer-to-float and float-to-float conversions are total but may round per IEEE 754; the rounding is part of the operation's meaning.

Conversion to and from the mathematical types is explicit as well: `toInt : t -> Int` and `toNat : U8 -> Nat` (and the other unsigned types) are total, `checkedFromInt : Int -> Option t` and `checkedFromNat : Nat -> Option t` are checked, matching the pattern above.

Integer literals are elaborated against their expected type. A literal outside that type's range is a compile error, and `Nat` and `Int` literals are unbounded. Hexadecimal literals (`0x` form) name the same values as decimal ones. A literal whose type is not determined by context is an error, with one narrow exception: when pending literal wants are the only thing blocking resolution, the literal defaults to the mathematical `Int`. There is no defaulting to `I64` or `F64`. Prefix `-` on a literal folds into the literal and is range-checked with it, so `-128 : I8` is valid; prefix `-` on a non-literal machine number is rejected (use `checkedNeg`, `wrappingNeg` or `saturatingNeg`).

# 8 Class instances

| Instance | Types |
| --- | --- |
| `Copy` | All primitives |
| `Eq` | `Bool`, `Char`, `I8`-`I64`, `U8`-`U64`, `Usize`, `Isize` |
| `Ord` | The same types as `Eq`, with signed numeric order on `I*`/`Isize`, unsigned on `U*`/`Usize`, `False < True`, code point order on `Char` |

`F32` and `F64` have none of these instances; use section 5's operations. Bitwise operations unify under the prelude `Bits` class (landed with the operator batches); `Wrapping`-style wrappers over the overflow families remain an open decision.

# 9 Interned symbols

`Sym` is a core runtime type: one word, a handle to a process-wide
interned name cell. It is not a machine scalar (section 2's set
unchanged): no arithmetic exists on it, and its layout is a word-sized
handle like `Bytes`/`String` values.

- **Interning**: names intern into cells; two equal names are the same
  cell, so `Eq` is identity (an $O(1)$ pointer test) and `Eq`/`Ord`
  coherence requires `Ord` to agree with name equality. `Ord` is
  lexicographic by name (deterministic across runs and builds; never
  insertion order). The prelude provides
  `intern : String -> Sym` and `sym-name : Sym -> String`.
- **Literals are constants first**: `'foo` interns at compile time and
  is emitted as a constant (the P14 constant-emission path); `intern`
  of an equal name at runtime resolves to the same cell (the runtime
  table is keyed by name). Symbol literal patterns test identity.
- **Classes**: `Eq`, `Ord`, `Show` (prints `'foo`). No numeric or
  character classes.
- **Boundaries**: `Sym` has no C counterpart; FFI code takes
  `sym-name` (or `Bytes`) at the boundary. Within Wasm components it
  is a word handle. Symbols are opaque identity: nothing in the
  language exposes a cell's address as data.

Syntax and contexts are `specifications/
surface-grammar-local-amendments.md` section 12.

# 10 Foreign, Wasm and SIMD correspondence

The layout table in section 2 is the FFI layout table. The C ABI mappings are `U8` to `uint8_t` through `U64` to `uint64_t`, `I8` to `int8_t` through `I64` to `int64_t`, `F32`/`F64` to `float`/`double` where the target ABI defines them as IEEE 754, and `Bool` to C's `_Bool`. `Char` has no C counterpart; use `U32`. FFI layer names such as `SizeT` and `CInt` in [Foreign function interface specification](foreign-function-interface-specification.md) are mapped to primitives by the target's recorded ABI metadata (commonly `SizeT` = `Usize`, `CInt` = `I32`), never by the generator host's `sizeof`.

For Wasm, `I8`-`I32`, `U8`-`U32` and `Usize`/`Isize` on wasm32 use `i32` operations, `I64`/`U64` use `i64`, and `F32`/`F64` use `f32`/`f64`. Sub-word types are value-level: their width is enforced by the operations of section 4 and section 7, not by a narrower Wasm type. A memory64 profile makes `Usize`/`Isize` 64-bit; see [WebAssembly and WASI specification](webassembly-and-wasi-specification.md) section 8 on keeping these widths a target property.

For SIMD, the lane element types of target-indexed vectors are exactly these primitives (`Vec n U32` and friends). Vector types themselves are not primitives and are not unboxed scalars. A rewrite between a scalar loop and a vectorized one must respect the arithmetic family: wrapping operations reassociate freely under their modular model, checked operations do not (a regrouping can change which operation overflows), and floating-point reassociation needs its own contract.

# 11 Required rejection cases

- `x + y` on `I32`, or any use of `Add`/`Sub`/`Mul`/`Neg` on a machine integer type, without an explicit wrapper instance.
- `x == y` or `x < y` on `F32`/`F64` (no `Eq`/`Ord` instance).
- An integer literal that does not fit its expected type, such as `300 : U8`, and a literal with no determined type.
- Implicit conversion in either direction between machine integer types, or between `Int` and a machine integer type.
- `-x` on a non-literal machine integer, and `checkedShl x 64` returning a wrapped value instead of `None` on `I64`.
- A `Char` built from a surrogate code point, or a float-to-integer conversion that is allowed to trap or wrap without saying so.
- Using the build host's `Usize` width in a cross-compiled layout or proof.

# 12 Open decisions

- Literal defaulting for unconstrained numeric literals beyond the narrow `Int` rule recorded in section 7 (pending wants only).
- Whether `I128`/`U128` belong in the core prelude or an extension module, and how targets without hardware support expose them.
- Type-level symbols: a `Sym` kind with promoted `'foo` literals for row labels, schema column names, and other static tags. The tick grammar already reserves the spelling (amendment section 12); the promotion is deferred until a consumer (likely the A6 schema vocabulary) needs it.
- Observability of IEEE status flags and the dynamic rounding mode, and the exact contract under which reassociation or contraction is permitted.
- A `Bits` class is landed (prelude, decision D90); what remains open is whether `Wrapping`-style wrappers over section 4's overflow families earn their own class.
- Encoding `Usize`'s width as a target-indexed value that dependent specifications can mention (needed before proofs may compute sizes).
- Byte order and alignment guarantees of raw memory APIs built on these types.

Resolved since draft 0.2: `&&`/`||` are surface control forms (section 6). The compiler elaborates infix `&&`/`||` to `if` at parse time; parenthesized `(&&)`/`(||)` name strict class methods and are different operations.

Resolved since draft 0.3: the `Bits` class (section 11's open decision) is implemented in the prelude: `bitAnd`/`bitOr`/`bitXor`/`bitNot`, wrapping shifts, total rotates, `testBit`/`setBit`/`clearBit`/`complementBit` over a modulo-width mask rule, and the bit inspection family, with instances for all ten machine integer types. `Wrapping`-style wrappers remain open. The traditional operator spellings `&`, `|`, `^`, `<<`, `>>` are wrappers over the class with Rust-style precedence (surface grammar local amendments section 5), and `Rem`/`Div` classes with `%`/`/` cover the mathematical `Nat` and `Int` (machine integers keep the checked families per section 8's rule).

# Implementation status

The compiler in `compiler/` implements this chapter's machine types and families (the `I8`-`I64`, `U8`-`U64`, `F32`, `F64`, `Char`, `Usize`, `Isize` names, the checked/wrapping/saturating families, explicit conversions, and literal range checking), and now has the mathematical `Nat` and `Int` on the bignum runtime with unbounded digit-string literals. `&&`/`||` short-circuit as required by section 10. Deviations still open: machine conversion names (`toInt`, `toNat`, `checkedFromInt`, `checkedFromNat`) are spelled with the machine type suffix (`intToI64` style) until the module slice lets this chapter's dotted spelling land (decision D6 in `compiler/CONFORMANCE.md`), and `Bytes` has no string literal form yet.

# Amendments

Dated notes recording landed decisions (`compiler/CONFORMANCE.md`,
D-numbers) that amend or interpret this chapter's normative text.
2026-10-28, P15 conformance sweep.

- **Literals and minus folding (D7, D10), 2026-10-28.** This amends
  section 7: integer literals are canonical decimal digit strings end
  to end (hex folds at lex, a prefix minus folds into the string);
  machine types range-check from the string, `Nat` rejects negatives,
  and `Nat`/`Int` lower to bignum constants with an `i64` fast path.
  Float literal exactness is the odd-part rule (odd part fits the
  significand and the binary exponent is in range). An ambiguous
  literal want defaults to the mathematical `Int` only when pending
  literal wants are the only thing blocking resolution; there is no
  defaulting to `I64` or `F64`.
- **`Copy` as the sharing class (D16), 2026-10-28.** This amends
  sections 3 and 8: the sharing class is spelled `Copy` (`Share` is an
  alias), with structural instances (scalars, `Nat`/`Int`/`Bool`/`Unit`,
  products/ADTs of `Copy` parts coinductively, `Ref r a`), not user-
  extensible. `Bytes`, functions, thunks, cells, and `Mut` are not
  `Copy`. Evidence is structural or from a `using (copy :: Copy a)`
  binder; `copyOut` is the only implicit-copy site plus memo bare-use.
- **Comparison derivation (D11), 2026-10-28.** `natCmp`/`intCmp` return
  the raw -1/0/1 immediate typed at `I64`; `Eq`/`Ord` instances derive
  as `eqI64 (intCmp x y) 0` style and compare the immediate against
  zero with the `I64` prims. This amends section 8's instance table.
- **`String` and `Bytes` (D13, D88-D89), 2026-10-28.** The runtime text
  type is `String`, a UTF-8 newtype over `Bytes` declared in the
  prelude; strings are built from `Bytes`, `Char`, and the formatting
  functions. 2026-10-28 (D291-D294): string literals are no longer a
  surface future item - a `"..."` literal in an ordinary term checks
  as the runtime `String` (constant `Bytes` payload, fresh per
  evaluation, escapes per the surface grammar) and allocates
  (the JPL no-allocation fact fires on it). The prelude
  `String` library is the REFERENCE SEMANTICS for UTF-8 (validation,
  decoding, encoding, code point indexing); the Rust runtime carries
  SIMD-accelerated implementations of the same rules (`cinch_utf8_*`)
  tested to agree: acceleration is a representation detail behind
  justified prim bodies (reference-then-accelerate). `Bytes` language
  signatures are `Nat`-typed with the machine-word conversion inside
  the intrinsic units (D13).
- **Operator spellings and the class stack (D2, D90-D91), 2026-10-28.**
  Operators are class methods (bare `~` is reserved for memo bindings).
  `&`, `|`, `^`, `<<`, `>>` wrap the `Bits` class; `%` and `/` are the
  `Rem` and `Div` methods with instances for the mathematical `Nat`
  and `Int` only (machine integers keep the explicit families);
  bitwise NOT has no operator spelling; division or remainder by zero
  takes the defined fault path (exit 72). Precedence and associativity:
  surface grammar local amendments section 6.
- **Interned symbols (D122-D124, D232-D237), 2026-10-28.** This amends
  section 9: `Sym` is a builtin core type with kind-tagged, leaked
  interned cells (`KIND_SYM`); `Eq` is the pointer test with kind
  validation, `Ord` compares name bytes; `cinch_sym_intern` rejects
  non-UTF-8 names so `sym-name` is total and the extern surface is
  exactly four `nm`-verified `cinch_sym_*` ops. `'foo` literals intern
  at compile time and emit as constants; the char-precedence rule is
  lexical (`'''` is `Char '\''`; a tick name is the identifier after
  the tick with dash glue and primes as ident chars). Type-position
  promotion of `'foo` is deferred with the spelling reserved (D124).
  Syntax: surface grammar local amendments section 12.
- **Layout correspondence naming (D6), 2026-10-28.** This amends
  section 10's naming: machine-type operations are prelude names of the
  form `<op><Type>` (`wrappingAddU32`, `u8ToI32`) bound from
  `foreign import ccall "cinch.prim.*"`; this chapter's dotted
  spellings (`U32.wrappingAdd`) are a presentation form, and the
  section-2 layout table is the FFI layout table (the cross-reference
  the SPEC-MAPPING wording flag records).
- **Cinch naming (D278-D281), 2026-10-28.** The language is Cinch:
  sources `.cinch`, interfaces `.cinchif`, runtime externs `cinch_*`;
  artifact names in this chapter read with those names.

[Documentation overview](../overview.md)
