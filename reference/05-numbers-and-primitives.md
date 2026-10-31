# Numbers, machine primitives, symbols, and text

Mathematical numbers, machine scalars, text, and byte buffers have distinct representations and safety obligations. Operator spellings are library vocabulary; explicit operation families decide the overflow policy.

# Integer types

| Family | Values / width | Arithmetic policy |
| --- | --- | --- |
| `Nat` | Nonnegative mathematical integers | Exact arithmetic; subtraction requires a nonnegative result. |
| `Int` | Signed mathematical integers | Exact arithmetic, subject to exhaustion. |
| `U8`, `U16`, `U32`, `U64` | Unsigned fixed width | Named checked, wrapping, or saturating operations. |
| `I8`, `I16`, `I32`, `I64` | Signed two's-complement width | Named checked, wrapping, or saturating operations. |
| `Usize`, `Isize` | Width from target profile | Same explicit policy, target-dependent bounds. |

For unsigned width `w`, the range is 0 through `2^w - 1`; for signed width `w`, it is `-2^(w-1)` through `2^(w-1)-1`. Literals outside the expected type's range are rejected. Mathematical numbers use immediate values when possible and bignums outside that range; the optimization does not change the mathematical range.

`checked-add-u8` returns `Option U8`. `wrapping-add-u8` returns the modular result. `saturating-add-u8` clamps at the bound. The exact shipped families, bounds, shifts, comparisons, and conversions appear in [Data.Machine](../library/Data.Machine.md). Machine scalars do not receive implicit arithmetic instances that hide the overflow policy.

# Partial arithmetic becomes a type obligation

`nat-div`, `nat-rem`, `int-div`, `int-rem`, `/`, and `%` take a `NonZero` divisor. A nonzero numeral discharges this at compile time. A variable needs a refined value or checked branch evidence. Checked division/remainder return `None` when the divisor is zero; machine signed division also checks its overflow case.

`Nat` subtraction requires `right <= left`. Byte access requires `index < bytes-len buffer`. Narrowing mathematical-to-machine conversions require the destination range, or use checked conversions returning `Option`. `InRange lo hi a` records inclusive bounds, allowing range facts to pass between operations. Do not replace a failing proof obligation with an `unsafe` assertion without identifying the new assumption.

# Floating point

`F32` and `F64` expose the current machine floating vocabulary and explicit conversions. IEEE arithmetic is not mathematical real arithmetic: NaN, infinities, signed zero, rounding, and nonassociativity affect laws. Use the named conversion and parser/formatter operations for fractional values; ordinary expression syntax does not provide a general decimal fractional literal form. Floating types have no implicit Eq/Ord/arithmetic instances that pretend IEEE laws are mathematical laws; use the named machine operations.

<span class="support-tag support-tag--planned">Coming soon</span> **the unified refined-float design.** The [float design](../design/float-refinements.md) plans a revised representation and proof-driven range/finite contracts, with explicitly selected unsafe IEEE instances. That design must not be read as a claim that every planned predicate or algebraic instance is already available. Current operations are listed from the Rust compiler's checked library interfaces.

# Strings, characters, and bytes

`Char` is a Unicode scalar, distinct from bytes. Its code conversion carries range information. `String` is text; `string-to-bytes` encodes UTF-8, and `string-from-bytes` returns `Option String` on validation. `Bytes` stores arbitrary byte data. Byte construction requires values in 0–255 and indexing needs bounds evidence. [Data.String](../library/Data.String.md), [Data.Char](../library/Data.Char.md), and [Data.Bytes](../library/Data.Bytes.md) list exact signatures.

# Interned symbols

`Sym` has literal syntax `'name`, identity-based equality, and name-based deterministic ordering. `intern` converts a string to a symbol; `sym-name` recovers its spelling. A symbol is not a numeric address or a machine scalar. FFI passes a string/byte representation rather than exposing the intern cell.

<span class="support-tag support-tag--planned">Coming soon</span> **type-level symbol consumers.** The spelling is reserved for promotion, but general type-level labels/schema consumers are deferred. Staging `Name` values are a separate name-quotation context, described in [staging](14-staging-and-macros.md).

Implementation: numeric registrations in `check_decls.rs`, lowering in `codegen.rs`, runtime in `compiler/runtime/src/`; suites: `primitives.rs`, `safe_division.rs`, `safe_subtraction.rs`, `safe_bytes.rs`, `range_facts.rs`, `small_nats.rs`, `string_literals.rs`, `sym.rs`.
