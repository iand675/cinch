# Attributes and pragmas: the complete reference

Source attributes in Cinch are written `#[...]`. Every attribute name must be either compiler-owned (listed below) or declared by a macro in scope; unknown names are compile errors, never silently ignored metadata.

Attributes cannot be shadowed and never bypass checking:

- **Hints** (`#[inline]`, `#[noinline]`, `#[unroll]`, `#[cold]`, branch weights) leave typing, effects, ownership, and interface hashes unchanged.
- **Contracts** (`#[repr]`, `#[byval]`, `#[sret]`, and safety assertions) are verified by the checker and hashed into the module's interface identity.

For detailed semantics beyond syntax and validation, see [Chapter 15](15-ffi-layout-and-attributes.md) (FFI & primitives), [Chapter 22](22-safety-assertions-and-llvm.md) (safety verification & LLVM lowering), and [Chapter 24](24-type-layout.md) (memory layout & field unpacking).

# Index of compiler-owned names

Every compiler-owned attribute accepted today is listed here. Variant arguments (such as `inline(always)` or `unroll(disable)`) belong to a single attribute name. Unimplemented design spellings are listed separately under [planned and provisional attribute spellings](#planned-and-provisional-attribute-spellings).

| Attribute | Admitted positions | Effect | Interface contract? | Status |
| --- | --- | --- | --- | --- |
| `#[meta(...)]` | Module header, any declaration, constructor, field, clause, alternative, guard, `if` condition/branch | Opaque tooling metadata; no check or runtime effect | No | Current |
| `#[weight(n)]` | Function clause, `case` alternative, guard, `if` condition, `if` branch | Assigns decision branch weight `n` (`0..=4294967295`) | No | Current |
| `#[likely]` | Same decision positions as `weight` | Shorthand for weight `2000` | No | Current |
| `#[unlikely]` | Same decision positions as `weight` | Shorthand for weight `1` | No | Current |
| `#[cold]` | Decision positions, or function declaration | Weight `0` on an outcome; `cold` entry hint on a function | No | Current |
| `#[inline]`, `#[inline(always\|never)]` | Function declaration | Discretionary, always, or never-inline hint | No | Current |
| `#[noinline]` | Function declaration | Equivalent to `#[inline(never)]` | No | Current |
| `#[unroll(N\|disable\|0)]` | Function declaration | Loop-unrolling hint (`N` in `1..=2147483647`, or `disable`/`0`) | No | Current |
| `#[repr(...)]` | `data` and `record` declarations; fields of a `repr(C)` record | Explicit layout contract | Yes | Current |
| `#[unpack]` | Native record field | Unpacks a monomorphic single-constructor product inline | Yes | Current |
| `#[nounpack]` | Native record field | Retains ordinary boxed/word field representation | Yes | Current |
| `#[recursion("...")]` | Function declaration | Verifies `"none"`, `"tail"`, or `"bounded(b)"` recursion | Yes | Current |
| `#[stack("...")]` | Function declaration | Verifies `"constant"` or `"bounded(b)"` stack growth | Yes | Current |
| `#[no-alloc]` | Function declaration | Verifies zero allocation sites in body | Yes | Current |
| `#[alloc("once")]` | Function declaration | Verifies at most one allocation site per call (none under closures) | Yes | Current |
| `#[max-complexity(n)]` | Function declaration | Verifies cyclomatic complexity (branches + 1) ≤ `n` | Yes | Current |
| `#[max-lines(n)]` | Function declaration | Verifies body source-line span ≤ `n` | Yes | Current |
| `#[assert-min(n)]` | Function declaration | Verifies at least `n` runtime `decide` checks | Yes | Current |
| `#[checked-params]` | Function declaration | Verifies every parameter is validated before first use | Yes | Current |
| `#[no-macros]` | Function declaration | Verifies no macro expansion provenance in body | Yes | Current |
| `#[no-raw-ptr]` | Function declaration | Verifies no raw pointers/opaque types in signature or body | Yes | Current |
| `#[deny-warnings]` | Module header, or function declaration | Promotes in-scope warnings to compile errors | Yes | Current |
| `#[profile("jpl")]` | Module header | Enforces the module-wide JPL safety profile | Yes | Current |
| `#[byval]` | `foreign import` or `foreign export` | Passes aggregate parameter by value per target C ABI | Yes | Current |
| `#[sret]` | `foreign import` or `foreign export` | Returns aggregate result by value per target C ABI | Yes | Current |

# Syntax and attachment

The attribute name must immediately follow `#[`, and any argument delimiter (`(` or `=`) or closing `]` must immediately follow the name with no intervening whitespace:

```text
#[inline]
#[inline(always)]
#[unroll(4)]
#[weight(2000)]
#[weight=2000]
#[recursion("tail")]
#[repr(C, align(16))]
#[meta(owner = "parser", revision = 1)]
```

An attribute name consists of ASCII alphanumerics, `_`, `'`, `.`, and `-`, and may not begin or end with `.` or `-`. Malformed syntax is rejected during lexing:

| Written | Lexer diagnostic |
| --- | --- |
| `#[ weight(2)]` | `an attribute needs a name` |
| `#[weight (2)]` | `unterminated attribute` |
| `#[weight = 2000]` | `unterminated attribute` |
| `#[!x]` | `an attribute needs a name` |
| `#[inline` | `unterminated attribute` |

Three argument forms are lexed:

1. **Bare:** `#[name]` — no arguments.
2. **Parenthesized:** `#[name(...)]` — balanced text inside parentheses; quoted strings are scanned opaquely so `)` inside a string does not close the attribute.
3. **Compact `=`:** `#[name=literal]` — attached `=` tail up to `]` (no spaces around `=`). Reaches the same validators as parenthesized arguments, except where parentheses are explicitly required (such as `#[repr(...)]`).

## Attachment positions

Attributes may sit on the line immediately above their target; layout separators introduced by the newline are ignored. Placement rules are enforced by the checker:

| Position | Placement | Admitted compiler-owned names |
| --- | --- | --- |
| Module header | Before `module` | `meta`, `profile`, `deny-warnings` |
| Declaration | Before signature or declaration | `meta` (any); `repr` (`data`/`record`); `inline`, `noinline`, `unroll`, `cold`, safety assertions (functions); `byval`, `sret` (`foreign`) |
| Constructor | Before a `data` constructor | `meta` |
| Record field | Before the field name or before its type | `meta`, `unpack`, `nounpack`; `repr(align(N))`, `repr(packed)`, `repr(packed(N))` (on `repr(C)` records only) |
| Function clause | Between function name and patterns | `meta`, `weight`, `likely`, `unlikely`, `cold` |
| `case` alternative | Before the pattern | `meta`, `weight`, `likely`, `unlikely`, `cold` |
| Guard | Immediately after `\|` | `meta`, `weight`, `likely`, `unlikely`, `cold` |
| `if` condition | Immediately after `if` | `meta`, `weight`, `likely`, `unlikely`, `cold` |
| `if` branch | Before the `then` or `else` expression | `meta`, `weight`, `likely`, `unlikely`, `cold` |

# Metadata and macro-owned attributes

## `#[meta(...)]`

`#[meta(...)]` accepts arbitrary balanced payload text at every attachment position (including codata observations), stores nothing in the interface contract, and emits nothing to code generation.

- Two `#[meta]` attributes on the **same construct** are rejected as a duplicate attribute name.
- Separate positions on one definition (for example, on the signature and on an equation, or repeated on the module header) are distinct attachment sites.
- Dotted names like `#[meta.reader(...)]` are **not** implicitly inside `meta`; they are rejected unless declared by a macro.

<!-- cinch:run exit=0 -->
```cinch
#[meta(tool = "indexer")]
module Main where
  #[meta(kind = "sum")]
  data T where
    #[meta(tag = "a")]
    A :: T
    B :: T

  #[meta(record = true)]
  record P where
    #[meta(unit = "bytes")]
    x :: I64
    y :: I64

  #[meta(signature = "size")]
  signature Size a where
    size :: a -> Int

  codata Stream where
    #[meta(observation = "head")]
    head :: I64

  g :: T -> Nat
  g #[meta(clause = "a")] A = 0
  g B = 1

  f :: T -> Nat -> Nat
  f t n =
    case t of
      #[meta(note = "first")] A ->
        if #[meta(note = "cond")] n == 0 then #[meta(note = "then")] 1 else 2
      B -> case n of
        #[meta(guard = "pass")] k | #[meta(guardnote = "nz")] k > 0 -> k
        _ -> 0

  main :: Int
  main = 0
```

## Macro-owned attributes

Macros declare custom attribute names in their signature:

```cinch
macro decorate attributes (codec) :: Syntax -> Declarations
```

- Two macros declaring the same attribute name collide at registration time (`attribute name 'codec' is already declared by macro '...'`).
- Compiler-owned validation always runs first, so declaring a compiler-owned name in a macro does not weaken or override its built-in placement rules.
- Attributes emitted by macro expansion are revalidated at the expansion site.

# Branch weights: `weight`, `likely`, `unlikely`, and `cold`

Branch weights express relative ratios within a single decision point (`0..=4294967295`). Weight `0` means negligible likelihood, never unreachability—the compiler never deletes a `0`-weighted branch.

| Spelling | Arguments | Assigned weight |
| --- | --- | --- |
| `#[weight(n)]`, `#[weight=n]` | Unsigned 32-bit integer literal | `n` |
| `#[likely]` | None | `2000` |
| `#[unlikely]` | None | `1` |
| `#[cold]` (decision position) | None | `0` |
| `#[cold]` (function declaration) | None | Marks function entry `cold` in LLVM |

Placement forms inside a function:

```cinch
choose #[weight(2000)] True = 1
choose #[weight(1)] False = 0

result = case flag of
  #[likely] True -> 1
  #[unlikely] False -> 0

result = if #[likely] flag then 1 else 0
result = if flag then #[weight(2000)] 1 else #[weight(1)] 0
```

Key rules:

- **`if` shorthand:** `if #[weight(w)] cond then a else b` assigns weight `w` to the true branch and defaults the false branch to `1` (unless `else` carries its own weight). Annotating both the condition and the `then` branch is rejected.
- **All-or-nothing per decision point:** Partially annotating a decision drops all weights at that decision and emits `incomplete-branch-weights` (promoted to an error under `#[deny-warnings]` or `#[profile("jpl")]`). A guard weight does not cover unannotated outer alternatives.
- **Overlapping clauses:** When a weighted clause reaches multiple decision paths (via wildcards, or-patterns, or guard fallthrough), the compiler reports an informational `incoherent-weights` notice and sums reachable clause weights per compiled column. See [Chapter 12](12-rewrites-and-optimization.md) for how optimizer passes preserve or rescale weights.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  classify :: Bool -> Int
  classify flag = case flag of
    #[likely] True -> 42
    #[cold] False -> 7

  choose :: Bool -> Int
  choose #[weight(2000)] True = 42
  choose #[weight(1)] False = 7

  guarded :: Bool -> Nat -> Int
  guarded flag n = case flag of
    #[weight(2000)] True | #[likely] n == 0 -> 42
    #[weight(2000)] True -> 7
    #[weight(1)] False -> 7

  main :: Int
  main = if classify False == 7 && choose True == 42 && guarded True 0 == 42 then 0 else 1
```

Common rejections:

| Written | Diagnostic summary |
| --- | --- |
| `#[weight(2)]` on a function declaration | `#[weight]` annotates decision alternatives, not function declarations (write clause weights between name and patterns). |
| `#[weight(4294967296)]` or `#[weight(x)]` | Weight must be a non-negative 32-bit integer literal. |
| `#[likely(1)]`, `#[unlikely(1)]`, `#[cold(1)]` | Takes no arguments. |
| `#[likely] #[weight(2000)]` on one outcome | One alternative carries at most one weight. |

# Function optimization hints: `inline`, `noinline`, `unroll`, and `cold`

All function hints are purely advisory: they do not enter `iface_hash`, do not change typechecking, and are rejected on `foreign` declarations.

| Spelling | Meaning | LLVM IR lowering |
| --- | --- | --- |
| `#[inline]` | Discretionary inline hint | `inlinehint` |
| `#[inline(always)]` | Force/strong inline hint | `alwaysinline` |
| `#[inline(never)]`, `#[noinline]` | Prevent inlining | `noinline` |
| `#[unroll(N)]` (`1 <= N <= 2147483647`) | Unroll loop by factor `N` | `!{!"llvm.loop.unroll.count", i32 N}` on loop back-edges |
| `#[unroll(disable)]`, `#[unroll(0)]` | Disable loop unrolling | `!{!"llvm.loop.unroll.disable"}` on loop back-edges |
| `#[cold]` | Mark function entry unlikely | `cold` |

Rules:

- **One hint per family per function** across its signature and equations combined: combining `#[noinline]` with `#[inline(never)]`, or `#[unroll(4)]` with `#[unroll(disable)]`, is an error.
- **`#[unroll]` details:** Accepts a non-negative integer literal up to `2147483647` (`i32::MAX`) or the keyword `disable`. `#[unroll(0)]` normalizes to `disable` (since `llvm.loop.unroll.count` of `0` is invalid). It never unrolls at the Core level and is accepted as a no-op on loop-free functions.

<!-- cinch:reject contains="one function carries one inline hint" -->
```cinch
module Main where
  #[noinline]
  #[inline(never)]
  keep :: Int -> Int
  keep x = x

  main :: Int
  main = 0
```

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[inline]
  a :: Int -> Int
  a x = x + 1

  #[inline(always)]
  b :: Int -> Int
  b x = x + 2

  #[noinline]
  c :: Int -> Int
  c x = x + 3

  #[cold]
  d :: Int -> Int
  d x = x + 4

  #[unroll(4)]
  sumTo :: Int -> Int -> Int
  sumTo acc n = if n == 0 then acc else sumTo (acc + n) (n - 1)

  #[unroll(disable)]
  e :: Int -> Int
  e x = x + 5

  main :: Int
  main = if a 0 == 1 && b 0 == 2 && c 0 == 3 && d 0 == 4 && sumTo 0 3 == 6 && e 0 == 5 then 0 else 1
```

Compact `=` syntax reaches the same validators for single-argument hints and assertions:

<!-- cinch:run exit=0 -->
```cinch
#[meta(owner = "examples")]
module Main where
  #[inline(always)]
  #[meta(reason = "compact argument form")]
  answer :: Int
  answer = 42

  #[recursion="none"]
  double :: Int -> Int
  double x = x + x

  main :: Int
  main = if #[weight=2000] double answer == 84 then 0 else 1
```

Hint argument and placement rejections:

| Written | Diagnostic summary |
| --- | --- |
| `#[inline(sometimes)]` | `#[inline]` takes no argument, `(always)`, or `(never)`. |
| `#[noinline(always)]` | `#[noinline]` takes no arguments. |
| `#[unroll(2147483648)]` | Count exceeds `2147483647` (`i32::MAX`). |
| `#[unroll(always)]` | Takes a natural number or `disable`. |
| `#[unroll(4)]` and `#[unroll(disable)]` on one definition | One function carries at most one unroll hint. |
| `#[inline]` or `#[unroll(4)]` on a `case` alternative or `foreign` declaration | Function-declaration hint only. |

# Repr: every current variant

`#[repr(...)]` is a semantic layout contract recorded in the module interface. Combine compatible modifiers inside one attribute (`#[repr(C, packed(2))]`); writing two `#[repr]` attributes on one declaration is a duplicate-name error.

| Variant | Admitted target & constraints | Effect |
| --- | --- | --- |
| `unboxed` | Single-constructor type with one supported runtime field | Construction and matching operate directly on the payload word |
| `niche` | Closed monomorphic `data` declaration whose constructors fit a single-word niche/bit-packed/tagged-pointer plan | Requires single-word niche fusion (also applied automatically by default when eligible); see [How memory layout works](31-memory-layout.md#automatic-niche-fusion-and-reprniche) |
| `value` | Single-constructor `record` whose fields are all POD (scalars, pointers, `#[repr(value)]`, `FixedArray`) | Inline non-refcounted multi-word value layout (`i64`, `{ i64, i64 }`, or indirect/`sret`); combines with `C`, `align(N)`, and `packed(N)` |
| `C` | Single-constructor `data` or `record`; no arguments | Source-order represented fields with C alignment and padding |
| `transparent` | Single-constructor type with exactly one represented non-erased field; combines with no other `repr` choice or field modifier | Identical size, alignment, and ABI layout to that field |
| `align(N)` | Nonzero power-of-two `u64`; cannot lower natural alignment | Raises aggregate alignment or field alignment (on `repr(C)` records) |
| `packed` | No arguments (alias for `packed(1)`) | Caps field placement alignment at `1` |
| `packed(N)` | Nonzero power-of-two `u64` | Caps effective field placement alignment at `N` |
| `u8`, `u16`, `u32`, `u64` | `data` declarations only (not `record`); combines with no other choice | Records a fixed discriminant width of 1, 2, 4, or 8 bytes in the interface (native payload stays uniform-word) |

Combination rules:

- `C` combines with `align(N)` or `packed`/`packed(N)`.
- `packed` and aggregate `align(N)` conflict.
- `transparent` and fixed discriminants (`u8`..`u64`) combine with nothing.
- Field-level `repr(align(N))`, `repr(packed)`, and `repr(packed(N))` are admitted only on non-erased fields of `#[repr(C)]` records. Writing a field modifier before the field name and before its type merges into one attribute list, so repeating it in both positions is a duplicate error.

On native records, `#[unpack]` stores the fields of a monomorphic single-constructor product inline, while `#[nounpack]` keeps the ordinary field representation. See [Type layout](24-type-layout.md#unpacking-a-record-field) for eligibility rules, byte offsets, packed-borrow restrictions, and FFI admission.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[repr(C)]
  record Aligned where
    #[repr(align(16))] a :: I64
    b :: I64

  #[repr(C, packed(2))]
  record Packed where
    a :: I64
    b :: I64

  #[repr(transparent)]
  record Wrapper where
    value :: I64

  #[repr(u8)]
  data Kind where
    First :: Kind
    Second :: Kind

  #[repr(C, packed)]
  data Word where
    Word :: I64 -> I64 -> Word

  main :: Int
  main = 0
```

Common `#[repr]` rejections:

| Written | Diagnostic summary |
| --- | --- |
| `#[repr]` or `#[repr()]` | Requires at least one representation. |
| `#[repr=C]` | Must use parentheses: `#[repr(...)]`. |
| `#[repr(Rust)]` | Unknown representation name. |
| `#[repr(align(0))]`, `#[repr(align(3))]` | Alignment must be a nonzero power of two. |
| `#[repr(C, align(4))]` on `I64` fields | `align(N)` can only raise natural alignment (here `8`). |
| `#[repr(C, packed, align(8))]` | `packed` and `align(N)` conflict. |
| `#[repr(transparent)]` with 2 represented fields | Requires exactly one represented field. |
| `#[repr(u16)]` on a `record` | Records have no constructor discriminant. |
| `#[repr(C)]` on multi-constructor `data` | `C`/`align`/`packed` apply to single-constructor layouts; use `u8`/`u16`/`u32`/`u64` for fixed discriminants. |
| `#[repr(C)]` on `codata`, constructor, or `case` arm | Only attaches to `data` or `record` declarations. |
| Field `repr(...)` on non-`repr(C)` record or erased field | Only non-erased fields of `repr(C)` records admit field `repr` modifiers (`align`/`packed` only). |

# Safety assertions

Eleven assertion names (fourteen argument forms) attach to function declarations (signature or top-level equations, at most one of each per definition). Every verified assertion is recorded in the module's interface `assertions` table and hashed into `iface_hash`. See [Chapter 22](22-safety-assertions-and-llvm.md) for the proof procedures, manifest format, and LLVM IR lowering.

| Spelling | Argument grammar | Recorded interface fact |
| --- | --- | --- |
| `#[recursion("none")]` | Exact string `"none"` | `recursion=none` |
| `#[recursion("tail")]` | Exact string `"tail"` | `recursion=tail` |
| `#[recursion("bounded(b)")]` | `"bounded(n)"` (`n` natural number or parameter/index name) | `recursion=bounded(b)` |
| `#[stack("constant")]` | Exact string `"constant"` | `stack=constant` |
| `#[stack("bounded(b)")]` | `"bounded(n)"` (same grammar) | `stack=bounded(b)` |
| `#[no-alloc]` | None | `no-alloc` |
| `#[alloc("once")]` | Exact string `"once"` | `alloc=once` |
| `#[max-complexity(n)]` | Natural number fitting `u64` | `complexity<=n` |
| `#[max-lines(n)]` | Natural number fitting `u64` | `lines<=n` |
| `#[assert-min(n)]` | Natural number fitting `u64` | `assert-min=n` |
| `#[checked-params]` | None | `checked-params` |
| `#[no-macros]` | None | `no-macros` |
| `#[no-raw-ptr]` | None | `no-raw-ptr` |
| `#[deny-warnings]` | None | `deny-warnings` |

A `bounded(b)` argument must be a literal natural number or an identifier (never an arithmetic expression). A named bound matches only a structural descent under that name (`#[stack("bounded(xs)")]` on a non-recursive function is rejected because the established bound is numeric `0`).

Argument and placement rejections:

| Written | Diagnostic summary |
| --- | --- |
| `#[recursion("strict")]` or `#[recursion(tail)]` | Requires quoted `"none"`, `"tail"`, or `"bounded(n)"`. |
| `#[stack("linear")]` | Requires quoted `"constant"` or `"bounded(n)"`. |
| `#[alloc("twice")]` | Currently accepts only `"once"`. |
| `#[max-lines(walrus)]` | Requires a natural number literal. |
| `#[no-alloc(1)]` | Takes no arguments. |
| Any safety assertion on `data` / `record` | Function declarations only. |
| Duplicate assertion across signature and equation | At most one per assertion name per definition. |

Representative verification failures:

| Assertion | Example violation diagnostic |
| --- | --- |
| `#[recursion("none")]` | `` `#[recursion "none"]` on Main.loopy asserts the no-recursion rule (JPL rule 1): the recursive call at this site `` |
| `#[recursion("tail")]` | `` the call to `Main.notail` at this site is not in tail position `` |
| `#[recursion("bounded(3)")]` | `` the established bound is call-graph (the literal counter ladder: depth <= 4), which does not bound the depth by 3 `` |
| `#[no-alloc]` | `` `#[no-alloc]` on Main.cons1 asserts the no-allocation rule (JPL rule 3): the constructor `Cons` is constructed here `` |
| `#[alloc("once")]` | `` the constructor `Cons` is constructed here as well: at most 1 allocation site(s) run per call `` |
| `#[max-complexity(1)]` | `` the body's cyclomatic complexity (branches + 1) is 3, above 1 `` |
| `#[max-lines(2)]` | `` the body spans 3 source lines, above 2 `` |
| `#[assert-min(2)]` | `` the body has 1 runtime `decide` check(s) (proof obligations do not count), below 2 `` |
| `#[checked-params]` | `` the parameter `x` is used here before any validating check (a refutable `case` match or a runtime `decide`) `` |
| `#[no-macros]` | `` a macro expansion occurred at this site `` |
| `#[no-raw-ptr]` | `` a raw pointer appears in the signature `` |

<!-- cinch:reject contains="boundary-validation rule (JPL rule 7)" -->
```cinch
module Main where
  #[checked-params]
  raw :: Int -> Int
  raw x = x + 1

  main :: Int
  main = 0
```

All eleven safety assertion names in one checked program:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  #[recursion("none")]
  #[no-alloc]
  #[no-macros]
  #[no-raw-ptr]
  #[deny-warnings]
  #[max-complexity(2)]
  #[max-lines(2)]
  double :: Int -> Int
  double x = x + x

  #[recursion("tail")]
  #[stack("constant")]
  sumTo :: Int -> Int -> Int
  sumTo acc n = if n == 0 then acc else sumTo (acc + n) (n - 1)

  #[stack("bounded(xs)")]
  lengthOf :: List Int -> Int
  lengthOf xs = case xs of
    Nil -> 0
    Cons x rest -> 1 + lengthOf rest

  #[alloc("once")]
  wrap :: List Nat -> List Nat
  wrap xs = Cons 1 xs

  #[assert-min(1)]
  bounded :: Nat -> Nat
  bounded x = case decide (x < 5) of
    Yes _ -> x
    No _ -> 0

  #[checked-params]
  validated :: Int -> Int
  validated x = case x of
    0 -> 1
    _ -> x

  main :: Int
  main = if double 2 == 4 && sumTo 0 3 == 6 && validated 0 == 1 then 0 else 1
```

# Module policy: `profile` and module `deny-warnings`

Only three compiler-owned attributes may appear before `module`: `#[meta(...)]`, `#[profile("jpl")]`, and `#[deny-warnings]`. Any other attribute on a module header is rejected (`unknown module attribute ...`).

- `#[profile("jpl")]` accepts only `"jpl"` and may appear at most once per module. See [Chapter 22](22-safety-assertions-and-llvm.md#the-jpl-profile) for the exact rule baseline and stricter-only override rules.
- `#[deny-warnings]` takes no arguments, may appear at most once on the module header, and promotes all warnings in the module to errors.

<!-- cinch:run exit=0 -->
```cinch
#[profile("jpl")]
module Main where
  classify :: Nat -> Nat
  classify x = case decide (x < 5) of
    Yes _ -> case decide (x < 3) of
      Yes _ -> 1
      No _ -> 2
    No _ -> 3

  main :: Int
  main = case 2 + 2 of
    n -> case decide (n < 5) of
      Yes _ -> case decide (3 < n) of
        Yes _ -> 0
        No _ -> 1
      No _ -> 2
```

Header rejections: `#[profile("nasa")]` (only `"jpl"` is accepted), duplicate `#[profile("jpl")]` or `#[deny-warnings]`, and `#[profile("jpl")]` on a function declaration.

# Foreign aggregate crossing: byval and sret

`#[byval]` and `#[sret]` select by-value C ABI aggregate crossing on real `foreign import` and `foreign export` declarations (never on functions or reserved intrinsics):

```cinch
#[byval]
foreign import ccall "sum_big" sum-big :: Big ->{Foreign Lib, Diverge} I64

#[sret]
foreign import ccall "make_big" make-big :: I64 ->{Foreign Lib, Diverge} Big
```

Here `Big` is a `#[repr(C)]` record with an admitted explicit layout. Without these attributes, an aggregate parameter crosses as a pointer to a call-scoped copy and an aggregate result is refused. With `#[byval]` or `#[sret]`, the backend emits the target C ABI's aggregate lowering (register pieces, `byval(<layout>)`, caller-copy pointer, or `sret(<layout>)`). See [Chapter 15](15-ffi-layout-and-attributes.md#aggregate-crossings-byval-and-sret) for per-target ABI rules and [Chapter 24](24-type-layout.md#foreign-calls-and-layout) for layout admission.

Neither attribute takes arguments, and each requires a matching aggregate in the foreign signature:

<!-- cinch:reject contains="this binding marshals no aggregate parameter" -->
```cinch
module Main where
  #[byval]
  foreign import ccall "fix_neg" neg :: CInt ->{Foreign Fix, Fail} CInt

  main :: Int
  main = 0
```

| Written | Diagnostic summary |
| --- | --- |
| `#[byval]` without an aggregate parameter | `this binding marshals no aggregate parameter`. |
| `#[sret]` without an aggregate result | `this binding's result is not an aggregate`. |
| Aggregate foreign result without `#[sret]` | Return through an `Out` cell / `Owned` pointer, or annotate with `#[sret]`. |
| `#[byval(1)]` or `#[sret(1)]` | Takes no arguments. |
| `#[byval]` or `#[sret]` on a regular function | Attaches only to `foreign` import/export declarations. |

# Duplicates, contract identity, and non-attributes

## Duplicate and family conflict rules

Repeating an attribute name on one construct is an error (unless a macro explicitly allows repetition for its own attribute). For functions, attributes on the signature and on top-level equations merge into one set, so the following are all rejected:

- Repeating any safety assertion, `#[cold]`, `#[inline]`, `#[noinline]`, or `#[unroll]` across signature and equations.
- Mixing two hints from the same family: `#[inline(never)]` with `#[noinline]`, `#[unroll(4)]` with `#[unroll(disable)]`, or `#[likely]` with `#[weight(2000)]` on one decision arm.

## Contract identity

Layout contracts (`#[repr]`, `#[unpack]`/`#[nounpack]`), foreign ABI contracts (`#[byval]`, `#[sret]`), and verified safety assertions enter the module's interface and `iface_hash`; changing them invalidates dependents. Optimization hints (`#[inline]`, `#[noinline]`, `#[unroll]`, `#[cold]`, branch weights) and `#[meta(...)]` never alter `iface_hash`.

## Forms that are not attributes

These constructs look similar to attributes in other languages (Haskell pragmas, Rust attributes, LLVM IR) and are rejected in `#[...]` position:

| Form | What it actually is in Cinch |
| --- | --- |
| `{-# ... #-}` | An ordinary nested block comment (`{- ... -}`); ignored by the compiler. |
| `immarg`, `uses`, `when`, `justify` | Clauses following a `prim` target-IR block (`{ ... }`). |
| `unsafe` | Declaration modifier (`unsafe prim ...`). |
| `ignore e` | Special-form expression returning `Unit` (`#[ignore]` does not exist). |
| `stage`, `macro`, `corec`, `proof`, `spec` | Declaration keywords before a declaration name. |
| `nonnull`, `noundef`, `willreturn`, `nofree`, `nocapture`, `!range` | LLVM IR attributes derived by the backend from verified analyses, never source attributes. |

# Planned and provisional attribute spellings

<span class="support-tag support-tag--planned">Coming soon</span> The following spellings appear in design documents and are **not** accepted by the compiler today; their final syntax remains subject to the implementing design:

| Design spelling | Intended role | Design document |
| --- | --- | --- |
| `#[global-allocator]` | Program-wide custom allocator selection | [Allocators](25-allocators.md) |
| `#[fip]` | Fully-in-place update and reuse verification | [Allocators](25-allocators.md) |
| `#[alloc("bounded(n)")]` | Proved allocation-count bounds beyond `"once"` | [Allocators](25-allocators.md) |

`#[may-fault]` is not a planned attribute.
