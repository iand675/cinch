# Float refinements: NaN, infinities and the other IEEE hazards in the types

Status: feasibility analysis and proposal for review, 2026-10-30. The user:
"it'd be great if you could achieve the same thing for float/double re NaN,
dividing by 0 (other cursed things), +Inf, -Inf. Not sure how feasible it
is". Companions: `design/no-faults.md`, `design/refinement-types.md`,
`design/no-faults-evidence-map.md`, `specifications/primitive-types-specification.md`
section 5. No em dashes.

## 1 Where Cinch already stands

Floats never fault, and the specification already refuses the classic
traps (`primitive-types-specification.md` sections 5 and 7):

- `F32`/`F64` have no `Eq` and no `Ord` instance, because IEEE equality is
  not an equivalence relation. Comparisons are the named IEEE predicates
  (`feq`, `flt`, ...) or `totalOrder`.
- Float-to-integer conversion is only `checkedToT` (`None` on NaN and out of
  range) or `saturatingToT`. The codegen selects around LLVM's poisonous
  `fptosi`/`fptoui` results (`codegen.rs` ~4070), so no poison escapes.
- No transformation may reassociate, contract or re-round float code.

So the problem is not crashes. NaN, the infinities and negative zero flow
through programs silently as values, and a type cannot say "this is a real
number". The goal is to let types say it and have the compiler prove it,
with the same machinery as the integer work (refinements over a type
parameter, D413; `decide`; facts from `case` rows, D412 and its rework).

## 2 The hazards, exactly (IEEE 754 binary64, round to nearest)

| Operation | NaN when | Infinite when (operands not NaN) |
| --- | --- | --- |
| `x + y` | an operand is NaN, or `{+Inf, -Inf}` | an operand is infinite, or overflow |
| `x - y` | an operand is NaN, or both infinite with the same sign | as above |
| `x * y` | an operand is NaN, or zero times infinity | an operand is infinite, or overflow |
| `x / y` | an operand is NaN, `0 / 0`, or `Inf / Inf` | `y` is zero and `x` is not, `x` is infinite, or overflow |
| `sqrt x` | `x` is NaN or `x < 0` (`sqrt (-0) = -0`) | `x = +Inf` |
| `rem x y`, `fmod` | an operand is NaN, `x` infinite, or `y` zero | never |
| `log x` | `x` is NaN or `x < 0` | `x = +-0` gives `-Inf`; `x = +Inf` |
| `pow x y` | negative finite `x` with non-integer finite `y`, NaN operands except `pow x 0 = 1` and `pow 1 y = 1` | `pow (+-0) y` for `y < 0`, overflow |

Other hazards:

- **Negative zero.** `-0 == +0` under IEEE equality, yet they are
  distinguishable: `1 / -0 = -Inf`, `signBit`, `totalOrder`. Hashing must
  not separate values equality identifies.
- **Negated comparisons.** `not (x < y)` does not imply `x >= y`: NaN makes
  both false. Facts from float comparisons must use IEEE-correct negation.
- **Non-closure.** Even when both operands are finite, `+`, `-`, `*` and `/`
  can overflow to an infinity. `Finite` is not closed under arithmetic
  without magnitude bounds; "not NaN" is not closed under `+` either
  (`Inf + -Inf`).
- **Rounding.** `(a + b) + c` and `a + (b + c)` differ; `x + 1 == x` for
  large `x`. Already handled: no reassociation without a contract.
- **Platform differences.** Transcendental functions (`sin`, `exp`, `log`)
  are not correctly rounded in most libms, so results differ across
  platforms. That is a determinism question, not a value hazard; it belongs
  with the platform contract.
- **Flush-to-zero and denormals-are-zero modes** change semantics. Cinch
  never enables them, and generated code must never set them.

## 3 Predicates

Extending the closed refinement vocabulary, all decidable by one or two
float comparisons:

| Predicate | Meaning | Runtime test |
| --- | --- | --- |
| `NotNaN x` | `x` is not NaN | `fcmp ord x, x` |
| `Finite x` | not NaN and not infinite | `isFinite` |
| `NonZero x` | strictly below or above zero, so neither zero and not NaN | `flt x 0 || fgt x 0` |
| `Positive x`, `Negative x` | `x > 0`, `x < 0` (excludes both zeros and NaN) | one `fcmp` |
| `NonNegative x` | `x >= 0` or `x` is `-0` (the `sqrt` domain; excludes NaN) | one `fcmp` |
| `InRange lo hi x` | `lo <= x <= hi` for literal bounds (excludes NaN) | two `fcmp` |

`type Real64 = refine (x :: F64) with Finite x` (name open) is the type most
code wants: an actual real number.

Facts from `case` rows (the D412 rework) produce these predicates from the
existing IEEE tests, with IEEE-correct negation. The false row of `flt x y`
gives `not (x < y)`, which implies `x >= y` only when both are `NotNaN`. The
true row of `isNaN x` gives nothing useful, and its false row gives
`NotNaN x`.

## 4 Operations whose types state their guarantees

Because `Finite` is not closed under arithmetic, refined operands give a
weaker result, and the type says exactly what holds:

| Operation | Operands | Result |
| --- | --- | --- |
| `+`, `-`, `*` | `Finite`, `Finite` | `NotNaN` (may overflow to an infinity) |
| `/` | `Finite`, `NonZero` and `Finite` | `NotNaN` |
| `sqrt` | `NonNegative` | `NotNaN` (and `Finite` if the operand is) |
| `log` | `Positive` | `NotNaN` |
| `rem` | `Finite`, `NonZero` | `Finite` |
| conversion to an integer type | `InRange lo hi` within the target's range | the integer, total, no `Option` |

To get `Finite` back after arithmetic there are two routes:

1. **Interval reasoning (proposed).** `InRange` predicates with literal
   bounds compose: if `x` is in `[a, b]` and `y` in `[c, d]`, then `x + y` is
   in `[a + c, b + d]`, computed with outward rounding so the bound is
   sound. When the result interval is inside the finite range, the result is
   `Finite` and `InRange`, with no runtime check. The checker already has
   literal-interval implication (`types.rs`, `interval_atom`). This extends
   it from comparisons to arithmetic over refined operands: abstract
   interpretation of the expression over intervals. Physical quantities,
   probabilities, normalized coordinates and money in a bounded range all fit
   this pattern.
2. **Decide at a boundary.** Where bounds are unknown (input, accumulation
   over unbounded loops), `decide (Finite x)` once, which is one comparison,
   and the fact carries forward. This is the same discipline as validating
   input.

## 5 What becomes lawful, and what gets faster

- **`Eq`, `Ord` and `Hash` on `NotNaN F64`.** Without NaN, a total order
  exists. Two choices, which differ only on zeros:
  - IEEE equality (`-0 == +0`, as Rust's `ordered_float::NotNan` does, with
    hashing normalizing `-0` to `+0`). This is an equivalence relation, but
    not a congruence: `1 / x` separates the two zeros, so a rewrite that
    substitutes equals for equals would be unsound.
  - `totalOrder` equality (`-0 /= +0`), which IS a congruence. Proposed,
    because Cinch's rewrites and laws assume equal values are
    interchangeable. IEEE `feq` stays available by name.
- **Sound fast-math.** LLVM's `nnan` and `ninf` flags let the optimizer
  assume no NaN or infinity, and make the result poison if the assumption is
  wrong, so most languages cannot use them safely. When the refinements
  prove operands and result `NotNaN` or `Finite`, the compiler can emit them
  soundly, with the proof as the justification (the unifying justification
  rule, `design/layouts-and-specialization.md` section 7). `reassoc` and
  `contract` change results and stay forbidden without a contract.

## 6 Feasibility

| Tier | What | Cost | Builds on |
| --- | --- | --- | --- |
| 1 | The predicates of section 3, `decide`, total front doors (`finite :: F64 -> Option Real64`), facts from IEEE tests with correct negation | small: the integer machinery reused | D412, D413 |
| 2 | The refined operation signatures of section 4; `Eq`/`Ord`/`Hash` on `NotNaN F64`; total float-to-integer conversion from `InRange` | medium: a signature table and instance work | tier 1 |
| 3 | Interval reasoning over arithmetic (section 4, route 1) | medium to large: outward-rounded interval arithmetic in the checker | NF4's linear arithmetic slot in `types.rs` |
| 4 | Sound `nnan`/`ninf` emission | small once tier 3 exists | the justification rule |
| 5 | Error-bound proofs (how far a result is from the real value) | research (Flocq-level) | proof automation, last |

Tiers 1 and 2 are straightforward, and tier 3 is what makes it pleasant. None
of it needs IEEE semantics to change: plain `F64` stays exact IEEE, and the
refinements say what is known about a value.

## 7 Decisions (user, 2026-10-30)

- **`F64` (and `F32`) stay the only float types, unboxed, with exact IEEE
  semantics.** The refinements of section 3 carry the proofs. A refinement
  is representation-identical (D31: the refined value is the container's
  machine word), so `Finite`, `NotNaN` and `InRange` cost nothing at
  runtime. A name such as `Real64` is only an alias for
  `refine (x :: F64) with Finite x`, never a distinct boxed type. This
  settles "opt-in or default": there is one type, and what is known about a
  value is in its refinement.
- **The IEEE `Eq`/`Ord` instances on `F64` live in an unsafe module and are
  never implicit.** They break `Eq`'s laws (`NaN` is not equal to itself),
  so they cannot be ordinary instances. Proposed mechanism, extending the
  existing `unsafe` keyword (prim bodies, D213/D221): an `unsafe instance`
  is exempt from the law layer's checks for the laws it names as violated,
  is recorded as an assumption in the artifact manifest, may never be
  declared `implicit` (an `implicit unsafe instance` is a compile error),
  and is reachable only by explicit selection (`using`/`with`). It lives in
  a module whose name says so (`Unsafe.Float`, or `Data.Float.Ieee` with the
  unsafe marker on the instance; the name is open).

## 8 Open decisions

- Whether the LAWFUL instances on `NotNaN F64` (`totalOrder` equality, a
  congruence, section 5) are implicit, or also explicit-only.
- The module name for the unsafe IEEE instances.
- Names: `Real64`/`Real32` aliases; the `InRange` spelling.
