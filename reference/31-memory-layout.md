# How memory layout works

Cinch separates a declaration's **source type** from its **physical memory layout**. Programs write ordinary algebraic data types, records, refinements, and polymorphic functions; the compiler chooses the most compact native representation that preserves soundness, reference counting, and module interface boundaries.

This chapter explains the full layout pipeline as implemented in both the Rust seed (`seed/`) and the self-hosted Stage 3 compiler (`compiler/`): representation classes, automatic niche fusion and bit-packing, 3-bit pointer tagging, NaN-sentinel float sums, polymorphic niche specialization, inline `#[repr(value)]` structs and `FixedArray`, automatic record unpacking, register-level calling optimizations (CPR, CSR, and worker-wrapper), stack-frame allocation, and self-describing heap headers.

For C ABI layout rules, padding, alignment, `#[repr(transparent)]`, and foreign boundary marshaling (`#[byval]` / `#[sret]`), see [Type layout and runtime representation](24-type-layout.md). For the complete `#[repr(...)]` and `#[unpack]` attribute index, see [Attributes and pragmas](23-attributes-and-pragmas.md).

# Layout pipeline at a glance

When the compiler checks a module and lowers it to LLVM IR, each type passes through a fixed hierarchy of layout decisions:

| Stage | Applies to | Result | Controlled by |
| --- | --- | --- | --- |
| 1. Explicit representation | Types carrying `#[repr(...)]` | C struct, `#[repr(value)]` POD struct, `#[repr(transparent)]`, `#[repr(unboxed)]`, `#[repr(niche)]`, or fixed discriminant | `#[repr(...)]` |
| 2. Default niche fusion & bit-packing | Unannotated closed `data` sums and small bit-packable `record`s | 1-word `Raw` or `TObj` value with zero heap allocation | Automatic (opt-in error checking via `#[repr(niche)]`) |
| 3. Polymorphic niche specialization | Unannotated polymorphic sums whose payload arms are type parameters or boxed objects | `TObj` pointer-tagged word at `Obj` instantiations; ordinary boxed layout elsewhere | Automatic (demand-driven specialization) |
| 4. Default record unpacking | Unannotated monomorphic records with small leaf product fields | Nested product fields stored inline in the enclosing block | Automatic (`#[unpack]` forces; `#[nounpack]` disables) |
| 5. Uniform boxed representation | Recursive types, open families, large products, remaining sums | 8-byte-aligned heap or static block behind a 1-word `Obj` pointer | Default fallback |
| 6. Call & lifetime elision | Functions returning/taking products or small sums, non-escaping blocks | Register returns (CPR/CSR), unboxed workers (`#ww`), stack frames (`RC_FRAME`), in-place Perceus reuse | Automatic in simplifier, ownership, stack, and codegen |

All six stages preserve source-level semantics, type identity, instance resolution, and refinement proofs. Layout choices that affect cross-module calling conventions are recorded in `.cinchif` interfaces and hashed into `iface_hash`.

# Representation classes

Every runtime slot — function parameter, local binder, constructor field, and return value — is assigned a representation class (`seed/src/repr.rs` and `compiler/Cinch.RuntimeIr.cinch`). Reference counting and pointer maps depend only on this class:

| Class | ABI token | Width | Counted | LLVM type | Used for |
| --- | --- | --- | --- | --- | --- |
| `Raw` | `R` | 1 word | Never | `i64` (narrowed at direct boundaries) | Machine scalars (`Bool`, `I8`..`I64`, `U8`..`U64`, `F32`, `F64`, `Char`, `Unit`, `Sym`), opaque handles, `Raw` niche/bit-packed types, refinement-proved immediate `Int`/`Nat` |
| `Obj` | `O` | 1 word | Always | `i64` / `ptr` | Heap and static blocks: ordinary records/ADTs, closures, dictionaries, `String`, `Bytes`, arrays, tuples |
| `TObj` | `T` | 1 word | If bit 0 is `0` | `i64` | Arbitrary-precision `Nat`/`Int` (immediate when odd, boxed bignum when even), pointer-tagged `Obj` niche sums, active polymorphic niche sums |
| `Wide` | `W` | 2 words | Never | `i128` | `I128`, `U128` (boxed at polymorphic word-only edges via `WideBox`) |
| `Val` | `V<sz>a<al>` | `sz` bytes | Never | `i64`, `{ i64, i64 }`, or indirect `ptr` / `sret` | `#[repr(value)]` POD structs and `FixedArray n a` |
| `Param(i)` | — | Specialized | Per call vector | Resolved on demand | Generic type parameters, specialized into `unit@RO`, `unit@OO`, etc. |
| `PolyNiche(mask)` | — | Specialized | Per call vector | `TObj` if all masked params are `Obj`, else `Obj` | Polymorphic sums eligible for pointer-tagged niche specialization |

## Direct-boundary narrowing

Polymorphic slots stay uniform 64-bit words so one specialized copy works for all `Raw` types or all `Obj` types. At direct monomorphic calls, the code generator narrows arguments and results to their proved width:

- `U8`, `I8`, `Bool` cross as `i8`; `U16`, `I16` as `i16`; `U32`, `I32`, `F32`, `Char` as `i32`.
- Counted pointers, `Nat`, and `Int` cross at target pointer width (`i64` on 64-bit native targets, `i32` on `wasm32`).
- Indirect apply stubs (`unit.app`) marshal between uniform word frames and narrowed direct entries.

## Refinement-proved `TObj` to `Raw` RC elision

Unrefined `Nat` and `Int` have class `TObj`: values that fit in a tagged immediate encode as `(n << 1) | 1`, while larger magnitudes allocate a bignum block. Every `TObj` copy or drop must test bit 0 at runtime.

When a refinement predicate proves that a `Nat` or `Int` always fits inside the target's immediate range, the compiler downgrades its representation class from `TObj` to **`Raw`**:

- On 64-bit targets: `Nat` with proved upper bound `<= 2^62 - 1` (`4_611_686_018_427_387_903`), or `Int` with proved bounds in `[-(2^62 - 1), 2^62 - 1]`.
- On `wasm32`: `Nat` with upper bound `<= 2^31 - 1`, or `Int` in `[-(2^30 - 1), 2^30 - 1]`.

The tagged immediate encoding is preserved, so conversions between refined and unrefined `Nat`/`Int` stay zero-cost, while all reference-counting checks on the refined type are statically elided:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  type SmallNat = refine (n :: Nat) with n <= 1000

  add-small :: SmallNat -> SmallNat -> Nat
  add-small a b = a + b

  main :: Int
  main =
    if add-small 40 2 == 42 then 0 else 1
```

# Automatic niche fusion and `#[repr(niche)]`

By default, the compiler attempts **niche fusion** on every unannotated closed `data` declaration and every small unannotated single-constructor `record` whose fields can be bit-packed into a single 64-bit word.

When niche fusion succeeds, the type occupies **one 64-bit word** (`Raw` or `TObj`) with zero heap wrapper allocation. You do not need to write `#[repr(niche)]` to get this layout; writing `#[repr(niche)]` on a `data` type makes niche fusion a checked requirement so the compiler reports an error if no valid single-word niche plan exists.

An unannotated declaration is skipped by default niche fusion (and falls back to its ordinary layout) when:

- It already carries an explicit `#[repr(...)]` attribute or `#[unpack]` field.
- It is an open data family, opaque type, or `codata` declaration.
- The type is exported from the module while one or more of its constructors are kept private (preventing layout leakage across an abstraction boundary).
- It is `Data.Core.Option` or `Data.Core.Result` (which use uniform layout plus polymorphic CSR so generic `Option a` / `Result e a` stay uniform across `Raw` and `Obj` instantiations).

## How niche domains work

Every payload type contributes an **occupied value domain** that tells the layout planner which 64-bit bit patterns are used and which are unused ("niches"):

| Payload type | Occupied domain | Available niches |
| --- | --- | --- |
| `Unit`, `Bool` | Unsigned `[0, 0]`, `[0, 1]` | All other 64-bit patterns |
| `U8`, `U16`, `U32`, `Char` | Unsigned `[0, max]` (`0x10FFFF` for `Char`) | Values above `max` |
| `I8`, `I16`, `I32` | Signed `[min, max]` | Values outside `[min, max]` |
| Refined `I64` / `U64` / scalars | Narrowed `[lo, hi]` minus proved `!= k` exclusions | Excluded values and out-of-range values |
| Refined immediate `Nat` / `Int` | Odd integers (`bit 0 == 1`) | All even integers (`0, 2, 4, ...`) |
| Unrefined `Nat` / `Int` | Non-zero `TObj` words (odd immediates + non-null aligned pointers) | `0` (`null`) |
| `F32` | Low 32 bits (`0 ..= 0xFFFF_FFFF`) | Values `>= 0x1_0000_0000` |
| `F64` | Canonicalized IEEE-754 bits (NaNs canonicalized to `0x7FF8_0000_0000_0000`) | Quiet-NaN bit patterns `0x7FF8_0000_0000_0001..` |
| Boxed `Obj` (`String`, `Bytes`, records, closures, etc.) | Non-null 8-byte-aligned pointers (`w != 0 && (w & 7) == 0`) | `0`, odd immediates (`bit 0 == 1`), and even low tags `2`, `4` |
| Nested niche `data` type | Union of its arms' occupied values and sentinels | Remaining unused patterns, avoiding sentinel collision |
| `I128`, `U128`, `Sym`, `FixedArray`, `#[repr(value)]`, opaque | Not single-word nichable | None |

## Refinement-driven scalar niches and nested sums

Refinements feed directly into layout planning. A `data` type carrying `refine (x :: I64) with x /= 0` places its nullary sentinel at `0`; a type with `refine (x :: I64) with x >= 0 && x <= 100` can fit multiple nullary variants outside `[0, 100]`; disjoint refined payload arms (such as negative vs. non-negative `I64`) share one `Raw` word without any tag bits; and a nested sum around another niche type allocates fresh sentinels that never collide with the inner type's sentinels:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  type NonZeroI64 = refine (x :: I64) with x /= 0

  data MaybeNonZero where
    NoVal :: MaybeNonZero
    SomeVal :: NonZeroI64 -> MaybeNonZero

  data NestedMaybe where
    OuterNone :: NestedMaybe
    OuterSome :: MaybeNonZero -> NestedMaybe

  read-nested :: NestedMaybe -> I64
  read-nested m = case m of
    OuterNone -> 1
    OuterSome NoVal -> 2
    OuterSome (SomeVal n) -> n

  main :: Int
  main =
    let a = read-nested OuterNone
    in let b = read-nested (OuterSome NoVal)
    in let c = read-nested (OuterSome (SomeVal 39))
    in if wrapping-add-i64 (wrapping-add-i64 a b) c == 42 then 0 else 1
```

## 3-bit pointer tagging for multi-pointer and mixed sums

Every heap and static Cinch block is 8-byte aligned, so the low 3 bits (`bits 0..3`) of any `Obj` pointer are always `000`. The layout planner exploits those bits for sums with boxed payloads:

- **Up to 3 boxed `Obj` payload arms** receive distinct even low pointer tags: **`0`**, **`2`**, and **`4`**. (Tag `6` is reserved; odd tags have bit `0 == 1` and denote uncounted immediates.)
- **Nullary variants** take `0` (`null`) first, then odd immediate sentinels (`1`, `3`, `5`, ...).
- **Small raw payload arms** (such as `Bool` or `I32`) can share the same 1-word `TObj` representation alongside up to 3 `Obj` arms by shifting their payload above an odd tag word (`bit 0 == 1`).

The resulting sum has class **`TObj`**. Reference-counting operations (`cinch_dup`, `cinch_drop`, `cinch_mark_mt`) test bit `0` (skipping odd immediates and `0`), clear the tag bits (`ptr & ~6`) to locate the underlying block header, and preserve the exact tag bits stored in container slots during multi-threaded sharing (`cinch_mark_mt`).

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data MiniJson where
    JNull :: MiniJson
    JBool :: Bool -> MiniJson
    JCode :: I32 -> MiniJson
    JStr :: String -> MiniJson
    JArr :: List String -> MiniJson
    JObj :: List (String, String) -> MiniJson

  score :: MiniJson -> I64
  score j = case j of
    JNull -> 1
    JBool b -> if b then 2 else 3
    JCode n -> if n == 7 then 4 else 0
    JStr _ -> 5
    JArr _ -> 10
    JObj _ -> 20

  main :: Int
  main =
    let total = wrapping-add-i64
          (wrapping-add-i64 (score JNull) (wrapping-add-i64 (score (JBool True)) (score (JCode 7))))
          (wrapping-add-i64 (score (JStr "a")) (wrapping-add-i64 (score (JArr ["b"])) (score (JObj [("k", "v")]))))
    in if total == 42 then 0 else 1
```

`MiniJson` packs all six variants into a single 64-bit `TObj` word (`JStr` = tag 0, `JArr` = tag 2, `JObj` = tag 4, `JNull` = `0`, and `JBool`/`JCode` in the odd immediate subspace).

## Bit-packed multi-field products and sums

When every field of a closed monomorphic `record` or multi-field `data` constructor has a small raw domain (such as `Bool`, `U8`, `U16`, `I8`, `I16`, `U32`, `I32`, or a bounded refinement like `refine (n :: I64) with n >= 0 && n <= 1023`) and the total width of all fields plus any constructor tag fits in **64 bits**, the compiler bit-packs all fields into a single **`Raw`** 64-bit word.

- Single-constructor bit-packed records (for example `Rgba { r :: U8, g :: U8, b :: U8, a :: U8 }` or a refined bitfield `SourceLoc`) use zero tag bits and pack fields sequentially with bias subtraction and masking.
- Multi-constructor sums mixing bit-packed arms, nullaries, and up to 3 `Obj` pointer arms either pack into one `Raw` word (tag in the low bits, packed fields above) or into one `TObj` word (odd tag word with packed fields shifted above, alongside tagged pointers).
- Bit-packed records still carry unoccupied bit patterns when their total width is below 64 bits, so an enclosing sum such as `OptionRgba` can fuse into the remaining bits of the same 64-bit word.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Rgba where
    r :: U8
    g :: U8
    b :: U8
    a :: U8

  data MaybeRgba where
    NoColor :: MaybeRgba
    Color :: Rgba -> MaybeRgba

  channel-sum :: MaybeRgba -> I64
  channel-sum c = case c of
    NoColor -> 0
    Color (Rgba r g b a) ->
      wrapping-add-i64
        (wrapping-add-i64 (u8-to-i64 r) (u8-to-i64 g))
        (wrapping-add-i64 (u8-to-i64 b) (u8-to-i64 a))

  main :: Int
  main =
    let s = channel-sum (Color (Rgba 10 12 15 5))
    in if s == 42 then 0 else 1
```

## NaN-sentinel `F64` and `F32` sums

IEEE-754 floating-point sums also collapse into a single `Raw` word:

- **`F64` sums:** When constructing a payload arm carrying `F64`, any NaN input (signaling or quiet, with any payload) is canonicalized to quiet canonical NaN `0x7FF8_0000_0000_0000`. Nullary variants receive distinct quiet-NaN sentinels starting at `0x7FF8_0000_0000_0001`, leaving every finite value, `±0.0`, `±Infinity`, and canonical `NaN` untouched and distinguishable.
- **`F32` sums:** `F32` occupies only the low 32 bits (`0 ..= 0xFFFF_FFFF`) of a 64-bit word, so nullary sentinels sit cleanly above `2^32 - 1` (`0x1_0000_0000`, `0x1_0000_0001`, ...).
- Nested `F64`/`F32` option sums draw consecutive unused sentinels so outer and inner nullaries never collide.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data MaybeF64 where
    NoF64 :: MaybeF64
    HasF64 :: F64 -> MaybeF64

  read-f64 :: MaybeF64 -> F64
  read-f64 x = case x of
    NoF64 -> 0.0
    HasF64 v -> v

  main :: Int
  main =
    if feq-f64 (read-f64 (HasF64 42.0)) 42.0 && feq-f64 (read-f64 NoF64) 0.0 then 0 else 1
```

## Polymorphic niche specialization

Polymorphic sums such as `Outcome e a` or `Maybe a` cannot commit to one concrete representation at declaration time because `a` might be instantiated with `I64` (`Raw`) at one call site and `String` (`Obj`) at another.

When an unannotated polymorphic `data` type has:

- At least two constructors,
- Between `1` and `3` single-field payload constructors whose payloads are either type parameters or concrete non-null `Obj` types, and
- Any number of nullary constructors,

the compiler attaches a **polymorphic niche plan** (`Class::PolyNiche(mask)`, where `mask` records which type-parameter positions must be `Obj`).

During demand-driven class specialization:

- At any instantiation where every required type parameter has class `Obj` (for example `Outcome String (List Int)`), the sum activates its 1-word pointer-tagged **`TObj`** layout with zero wrapper allocation.
- At instantiations where a required parameter is `Raw`, `TObj`, `Wide`, or `Val` (for example `Outcome String I64`), it falls back seamlessly to ordinary boxed heap representation (where CSR can still unbox return values in registers).

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Outcome e a where
    Pending :: Outcome e a
    Failed :: e -> Outcome e a
    Ready :: a -> Outcome e a

  score :: Outcome String String -> I64
  score o = case o of
    Pending -> 1
    Failed _ -> 10
    Ready _ -> 31

  main :: Int
  main =
    let total = wrapping-add-i64 (score Pending) (wrapping-add-i64 (score (Failed "err")) (score (Ready "ok")))
    in if total == 42 then 0 else 1
```

## Explicit `#[repr(niche)]` checks

Adding `#[repr(niche)]` explicitly requires the compiler to prove a monomorphic single-word niche layout and reject the declaration otherwise. `#[repr(niche)]` applies only to `data` declarations (not `record`s), and fails when no unused bit pattern exists (for example, an unrefined `I64` paired with a nullary variant):

<!-- cinch:reject contains="repr(niche)" -->
```cinch
module Main where
  #[repr(niche)]
  data FullI64 where
    Empty :: FullI64
    Full :: I64 -> FullI64

  main :: Int
  main = 0
```

<!-- cinch:reject contains="applies only to data declarations" -->
```cinch
module Main where
  #[repr(niche)]
  record NotAData where
    a :: U8
    b :: U8

  main :: Int
  main = 0
```

# Inline POD value types: `#[repr(value)]` and `FixedArray`

When you want a multi-field aggregate stored **inline by value** (never heap-allocated, with deterministic C-compatible field offsets and alignment), use `#[repr(value)]` on a `record` and/or `FixedArray n a`.

All fields of a `#[repr(value)]` record or `FixedArray n a` must be **POD (plain old data)** — non-refcounted scalars (`Unit`, `Bool`, `I8`..`I128`, `U8`..`U128`, `Isize`, `Usize`, `F32`, `F64`, `Char`), foreign pointers, other `#[repr(value)]` records, or `FixedArray n a`. Combining with `C`, `align(N)`, or `packed(N)` is supported (`#[repr(C, value, packed(2))]`, `#[repr(value, align(64))]`).

`#[repr(value)]` records and `FixedArray n a` have representation class `Val { size, align }` (`V<size>a<align>` in ABI tables) and follow a four-tier native value calling convention (`ValueAbi`):

| Byte size & alignment | Native LLVM parameter & return ABI |
| --- | --- |
| `size == 0` | Zero-sized (`i64 0` dummy register, no memory traffic) |
| `1 <= size <= 8` and `align <= 16` | Passed and returned directly in one `i64` register |
| `9 <= size <= 16` and `align <= 16` | Passed and returned directly in `{ i64, i64 }` registers |
| `size > 16` or `align > 16` | Passed by `ptr readonly align <al> dereferenceable(<sz>)`; returned via caller stack `sret([<sz> x i8])` |

Because `#[repr(value)]` values carry no heap header and no reference counts, functions operating on them satisfy `#[no-alloc]` directly:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[repr(value)]
  record Vec2 where
    x :: U64
    y :: U64

  #[repr(value, align(64))]
  record CacheLine where
    val :: U64

  #[no-alloc]
  add-vec2 :: Vec2 -> Vec2 -> Vec2
  add-vec2 a b = Vec2 (wrapping-add-u64 a.x b.x) (wrapping-add-u64 a.y b.y)

  #[no-alloc]
  bump-line :: CacheLine -> CacheLine
  bump-line c = CacheLine (wrapping-add-u64 c.val 1)

  main :: Int
  main =
    let v = add-vec2 (Vec2 10 20) (Vec2 5 7)
    in let c = bump-line (CacheLine 41)
    in if v.x == 15 && v.y == 27 && c.val == 42 then 0 else 1
```

# Record field unpacking: automatic, `#[unpack]`, and `#[nounpack]`

For ordinary heap-allocated monomorphic records (those not using `#[repr(C)]`, `#[repr(value)]`, `#[repr(unboxed)]`, or bit-packed niche layout), Cinch automatically inlines small nested product fields into the enclosing record's payload so outer records do not chase an extra heap pointer.

## Default auto-unpacking rules

A field `f :: T` of a monomorphic native record `R` is unpacked automatically when all of the following hold:

1. `T` is a monomorphic single-constructor native product (`record` or single-constructor `data`) defined in the same module, without explicit `#[repr(...)]` or niche layout.
2. `T` has between **2 and 4** non-erased represented fields, none of which are `I128`/`U128`.
3. `T` is a **leaf product** (none of `T`'s own fields is a single-constructor uniform product, guaranteeing non-recursiveness and declaration-order independence).
4. If `R` is exported, `T`'s constructor is also exported.
5. The field is not marked `#[nounpack]`, and the resulting flattened record has at most **12** total physical slots.

When `R` is constructed directly from a nested constructor expression such as `Ray (Vec3 1.0 2.0 3.0) (Vec3 4.0 5.0 6.0)`, the compiler flattens the six `F64` fields directly into one 6-word `Ray` block without ever allocating intermediate `Vec3` blocks. Pattern matching (`case ray of Ray (Vec3 x y z) _ -> ...`) reads the inline slots directly.

- Use **`#[unpack]`** on a field to unpack a monomorphic single-constructor product explicitly (even across module boundaries or above the automatic thresholds).
- Use **`#[nounpack]`** on a field to opt out of automatic unpacking and keep a separate boxed pointer (for example when the inner value is frequently shared across many outer records).

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Vec3 where
    x :: F64
    y :: F64
    z :: F64

  -- Both Vec3 fields auto-unpack inline into one 6-word Ray block.
  record Ray where
    origin :: Vec3
    dir :: Vec3

  -- Explicit opt-out keeps `shared` as a single pointer slot.
  record SharedRay where
    #[nounpack] shared :: Vec3
    scale :: F64

  ray-x :: Ray -> F64
  ray-x r = case r of
    Ray (Vec3 ox _ _) (Vec3 dx _ _) -> add-f64 ox dx

  main :: Int
  main =
    let r = Ray (Vec3 12.0 2.0 3.0) (Vec3 30.0 5.0 6.0)
    in let sr = SharedRay (Vec3 1.0 2.0 3.0) 2.0
    in if feq-f64 (ray-x r) 42.0 && feq-f64 sr.scale 2.0 then 0 else 1
```

# Eliminating temporary allocations across calls

Even when a type's canonical representation is a boxed `Obj` block (such as `Vec3`, `Option I64`, or `Result String I64`), the compiler eliminates short-lived allocations across function boundaries through five cooperating passes:

1. **Constructed Product Results (CPR):** If every return path of a function constructs the same 1–3 field product (or tail-calls a function that does), its internal body returns the fields directly in registers as `{ i64, ... }`. Callers that immediately `case`-destructure the result (`virtual_let`) never allocate a heap block.
2. **Constructed Sum Results (CSR):** If every return path of a function constructs one of two distinct uniform constructors (tags `0..=255`, each with `0..=2` non-wide fields — including `Option` and `Result`), its body returns `{ i8, i64 }` or `{ i8, i64, i64 }` in registers. Chained functions that match a CSR result and forward or construct arms of the same sum (such as `checked-step` calling `safe-div`) stay completely unboxed across `#[noinline]` boundaries.
3. **Multi-parameter product worker-wrapper (`#ww` / `#cprww`):** When a function pattern-matches one or more small product parameters (`<= 4` fields each, up to `8` total flattened parameters), the compiler emits an unboxed worker taking the fields directly in registers, rewrites direct constructor-argument call sites to the worker, and composes with CPR (`#cprww`) when the function also returns a product.
4. **Stack-frame allocation (`RC_FRAME`):** Non-escaping heap blocks whose lifetime is statically bounded by the current call frame are placed in stack `alloca` slots (`seed/src/stack.rs`) with `rc = RC_FRAME`, avoiding heap allocator traffic while remaining compatible with runtime release paths.
5. **Perceus in-place reuse & static uniqueness:** When a `case` arm consumes a dying block and allocates a same-size constructor, Perceus reuses the dying cell in place (including across different constructors of the same size). When static uniqueness analysis proves `rc == 1`, the runtime uniqueness branch is elided completely and the cell is overwritten unconditionally.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Vec3 where
    x :: F64
    y :: F64
    z :: F64

  #[noinline]
  safe-div :: I64 -> I64 -> Result String I64
  safe-div a b =
    if b == 0 then Err "div0"
    else Ok (wrapping-add-i64 a b)

  #[noinline]
  checked-step :: I64 -> I64 -> Result String I64
  checked-step x d = case safe-div x d of
    Err e -> Err e
    Ok y -> safe-div y 3

  #[noinline]
  add-vec3 :: Vec3 -> Vec3 -> Vec3
  add-vec3 a b = case a of
    Vec3 x1 y1 z1 -> case b of
      Vec3 x2 y2 z2 -> Vec3 (add-f64 x1 x2) (add-f64 y1 y2) (add-f64 z1 z2)

  main :: Int
  main =
    let r = case checked-step 10 2 of Err _ -> 0; Ok v -> v in
    let s = case add-vec3 (Vec3 1.0 2.0 3.0) (Vec3 4.0 5.0 6.0) of
          Vec3 x y z -> add-f64 (add-f64 x y) z in
    if r == 15 && feq-f64 s 21.0 then 0 else 1
```

# Heap block layout and self-describing headers (ABI 10)

Every heap-allocated, stack-frame, and static block places a 64-bit header word at byte offset **`-8`** from its payload pointer. Every payload is rounded up to whole 64-bit words (minimum 8 bytes) so payload addresses are always 8-byte aligned — the invariant that 1-bit `Nat`/`Int` immediates and 3-bit pointer tags rely on.

```text
  offset -16 (optional)   offset -8 (header, 64 bits)            offset 0.. (8-byte aligned payload)
  +---------------------+--------------------------------------+------------------------------------+
  | descriptor ptr /    | layout (24b) | kind (5b) | rc (35b)  | word 0 | word 1 | ...              |
  | extended byte size  | bits 40..64  | bits 35..40 | bits 0..35 |                                  |
  +---------------------+--------------------------------------+------------------------------------+
```

| Bit range | Width | Field | Encoding |
| --- | --- | --- | --- |
| `0..35` | 35 bits | `rc` | Signed count: `0` = immortal/static (`RC_IMMORTAL`), `1` = freshly allocated (`RC_FRESH`), `> 1` = single-threaded count, `-n` = atomic multi-threaded count of `n`, `2^34 - 1` = live stack-frame block (`RC_FRAME`), `2^34 - 2` = released stack-frame block (`RC_FRAME_RELEASED`) |
| `35..40` | 5 bits | `kind` | Runtime block kind (`1` = constructor, `2` = closure, `3` = dictionary, `4` = memo, `6`/`7` = boxed `Nat`/`Int`, `8`/`9`/`19`/`20` = `Bytes` variants, `14` = `Array`, `17` = `TVar`, `18` = `MVar`, `21` = `Atomic`, `22` = `RwLock`, `31` = debug freed poison) |
| `40..64` | 24 bits | `layout` | Self-describing shape for shaped blocks (`KIND_CTOR`, `KIND_CLOSURE`, `KIND_DICT`), or payload word count for fixed/unshaped kinds (`0xFF_FFFF` = `LAYOUT_BIG`, placing byte size at offset `-16`) |

## Self-describing shaped blocks

Constructors, closures, and dictionaries carry no global layout table and no runtime layout ID:

- **Inline shape:** Bits `40..48` (`8` bits) hold the constructor tag (`0..=254`, or `0` for closures/dictionaries). Bits `48..64` (`16` bits) hold a 1-bit-per-field counted mask over payload fields (after the fixed prefix of a closure or dictionary). Each counted field is scanned as `SCAN_TOBJ` (skipping odd immediates and `0`, untagging `~6`, and releasing/marking the target block).
- **Escaped shape:** When a constructor tag is `>= 255`, a counted field sits at index `>= 16`, or a constructor/dictionary exceeds the small-object heap limit (`31` payload words / 256 bytes including header), bits `40..48` hold `255` (`SHAPE_ESCAPED`) and offset **`-16`** stores a pointer to a static descriptor `{ tag, payload_words, counted_mask_words... }`.
- **Small-object heap:** Blocks up to 256 bytes (header + up to 31 payload words) are served from thread-local size-classed slabs; larger blocks fall back to the counting system allocator.

Runtime headers and descriptors are internal ABI details; user and foreign code must never read or persist them directly.

# Inspecting layouts and verifying zero allocation

Three tools let you verify the exact layout and allocation behavior the compiler chose for any module:

1. **Interface layout table (`.cinchif`):** Run `cinch check Main.cinch` and inspect `abi.layouts` in `build/Main.cinchif` to see recorded `repr`, byte `size`, `align`, `fields`, and `discriminant` for every type.
2. **Emitted LLVM IR:** Run `cinch emit-ir Main.cinch -o Main.ll` to inspect function signatures (`i64`, `{ i64, i64 }`, `{ i8, i64 }`, `sret`, `#csr`, `#ww`, `#cpr`) and confirm the absence of `@cinch_alloc_ctor` inside hot loops.
3. **Runtime allocation and RC diagnostics:** Run the compiled binary with environment flags (see [Allocators and heap tuning](25-allocators.md)):

```sh
CINCH_ALLOC_STATS=1 CINCH_LEAK_CHECK=1 ./main
CINCH_RC_DEBUG=1 ./main
```

- `CINCH_ALLOC_STATS=1` prints total allocated blocks and bytes on exit.
- `CINCH_LEAK_CHECK=1` asserts zero live heap blocks at process exit.
- `CINCH_RC_DEBUG=1` quarantines freed blocks and poisons their payloads with `0xdeadbeefdeadbeef` and `KIND_FREED` to trap any reference-counting bug immediately.
