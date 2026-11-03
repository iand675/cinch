# Attributes and pragmas: the complete reference

Cinch writes source attributes as `#[...]`. This page indexes **every compiler-owned attribute name currently accepted by the Rust implementation**, its arguments, valid attachment points, and meaning. The [type-layout chapter](24-type-layout.md) explains representation choices in detail, and [safety assertions](22-safety-assertions-and-llvm.md) explains the definition checks and emitted LLVM facts.

Attributes are not ordinary function names and cannot be shadowed. They do not grant permission to violate types, effects, ownership, coverage, or proof rules. A performance hint leaves those rules intact; a semantic assertion must pass its check.

# Syntax and attachment

Use these current spellings:

```text
#[inline]
#[inline(always)]
#[weight(2000)]
#[recursion("tail")]
#[repr(C, align(16))]
#[meta(owner = "parser", revision = 1)]
```

The name immediately follows `#[`. The argument opener `(`, an admitted `=`, or closing `]` immediately follows the name. Spaces inside parentheses are fine; `#[ weight(2)]` and `#[weight (2)]` are rejected by the current lexer. Parenthesized arguments are balanced and quoted strings inside them are kept opaque while scanning. An unterminated list/string is a lexical error.

The lexer also accepts a compact `=...` tail for attributes whose validator admits it, such as `#[weight=2000]` and `#[recursion="tail"]`. `repr` specifically requires parentheses. Prefer the parenthesized form throughout this book. Historical design examples such as `#[recursion "tail"]` describe the argument concept, but the space-separated spelling is not accepted by the current lexer.

Prefix a function's signature or top-level equation for a function attribute. Put a **clause** weight between the function name and its patterns: `choose #[likely] True = ...`. A `case` weight precedes the pattern; a guard weight follows `|`. `if` has both condition-position and branch-position forms. Prefix a module header for a module attribute:

```text
#[deny-warnings]
module Checked where
  ...
```

Do not assume a parsed attribute is legal in every position. Function inline hints cannot annotate a constructor or `case` alternative. Module headers admit only `meta`, `profile`, and `deny-warnings`. Field layout modifiers have the additional containing-record restriction described below.

# All compiler-owned attribute names

The Rust validator recognizes these **22 names**. Spellings such as `repr(C)` and `inline(always)` are variants of one name, rather than separate attributes.

| Name and accepted arguments | Valid placement | Meaning |
| --- | --- | --- |
| `#[meta(...)]` | Attribute-bearing declarations, constructors, fields, clauses/alternatives/guards/if positions; also module headers | Opaque nonsemantic metadata for tooling. |
| `#[weight(n)]` | Function clause, `case` alternative, guard, `if` condition or branch | Relative nonnegative 32-bit branch weight. |
| `#[likely]` | Same decision positions as `weight` | Exactly weight 2000. No arguments. |
| `#[unlikely]` | Same decision positions as `weight` | Exactly weight 1. No arguments. |
| `#[cold]` | Decision positions, or a function signature/top-level equation | Weight 0 on an alternative; function-level LLVM cold hint on a definition. No arguments. |
| `#[inline]`, `#[inline(always)]`, `#[inline(never)]` | Function signature/top-level equation | Desirable, always, or never inline hint. |
| `#[noinline]` | Function signature/top-level equation | Alias of `inline(never)`. No arguments. |
| `#[repr(...)]` | Admitted data/record/codata type declarations; restricted field positions | Layout contract. Complete variant table below. |
| `#[recursion("none")]`, `#[recursion("tail")]`, `#[recursion("bounded(n)")]` | Function signature/top-level equation | Check absence, tail position, or a supported recursion-depth bound. Pass one quoted string. |
| `#[stack("constant")]`, `#[stack("bounded(n)")]` | Function signature/top-level equation | Check the corresponding stack-growth discipline. Pass one quoted string. |
| `#[no-alloc]` | Function signature/top-level equation | Check the analysis's no-allocation contract. No arguments. |
| `#[alloc("once")]` | Function signature/top-level equation | Check the admitted single-allocation-site-per-call bound. Other strings are rejected. |
| `#[max-complexity(n)]` | Function signature/top-level equation | Cyclomatic complexity, branches plus one, is at most `n`. |
| `#[max-lines(n)]` | Function signature/top-level equation | Checked source-body line count is at most `n`. |
| `#[assert-min(n)]` | Function signature/top-level equation | At least `n` runtime decision/assertion checks; erased proof obligations do not count. |
| `#[checked-params]` | Function signature/top-level equation | Check validation before the parameter uses covered by the analysis. No arguments. |
| `#[no-macros]` | Function signature/top-level equation | Check expansion provenance for absence of macros. No arguments. |
| `#[no-raw-ptr]` | Function signature/top-level equation | Check the raw-pointer/opaque-signature and operation restriction. No arguments. |
| `#[deny-warnings]` | Function signature/top-level equation, or module header | Warnings in its checked scope are errors. No arguments. |
| `#[profile("jpl")]` | Module header only | Apply the implemented JPL rule profile. Other strings are rejected. |
| `#[byval]` | Real `foreign import`/`foreign export` declaration | Select the admitted aggregate parameter-copy crossing. No arguments. |
| `#[sret]` | Real `foreign import`/`foreign export` declaration | Select an admitted aggregate result-out crossing. No arguments. |

For complexity, line, and assertion counts, `n` is a natural-number literal fitting `u64`. In `bounded(n)`, it is a natural-number literal fitting `u64`, or an admitted argument/size-index name such as `bounded(xs)`. It is **not** an arbitrary arithmetic expression. Named structural bounds and literal call-ladder bounds have different proof procedures; unsupported claims are refused.

Without an attribute, no corresponding user assertion is requested. Ordinary type/effect/resource checking and the backend's independently established facts still apply. Inlining/weights use the optimizer's ordinary policy when unspecified; unannotated layout uses the compiler's native representation.

# Branch weights, likely, unlikely, and cold

Weights are ratios within **one decision point**, not probabilities measured across unrelated functions. Values range from 0 through 4,294,967,295. Zero describes negligible likelihood; it does not make a branch impossible or permit its removal.

These are fragments inside a function definition:

```cinch
choose #[weight(2000)] True = 1
choose #[weight(1)] False = 0

result = case flag of
  #[likely] True -> 1
  #[unlikely] False -> 0

result = if #[likely] flag then 1 else 0
result = if flag then #[weight(2000)] 1 else #[weight(1)] 0
```

A condition-position `if #[weight(w)] condition` is a pair form: it supplies true weight `w` and completes false weight 1. An explicit false-branch annotation can replace that implicit 1. Annotating the true side again is a duplicate outcome weight. A guard annotation similarly weights the pass outcome, with fallthrough weight 1. A guard's likelihood still supplies no unconditional coverage.

For ordinary alternative/branch lists, partial annotation drops **all** weights at that decision and emits `incomplete-branch-weights`. It does not mix an explicit weight with an invented estimate for unannotated alternatives. Under `deny-warnings`, the warning policy can make this a failure. `incoherent-weights` is informational: it reports the relevant wildcard/marginalization inconsistency while retaining weights.

Pattern-column compilation marginalizes clause weights by summing reachable clause weights. Optimizer transformations preserve their conditional meaning, rescale merged decisions where required, or drop weights where the contract requires it. See [rewrites and optimization](12-rewrites-and-optimization.md). A hint is never proof that an arm is unreachable.

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

Notice that the cold branch still returns 7 correctly. `#[cold]` before a function signature instead describes the function entry and lowers as a function cold hint; it is not a numeric weight on every branch inside that function.

# Inline and noinline

`inline` requests ordinary discretionary inlining; `inline(always)` selects the stronger always-inline hint; `inline(never)` and `noinline` select the same never-inline hint. The emitted LLVM spellings include `inlinehint`, `alwaysinline`, and `noinline`.

These are hints on a checked definition, not textual macros. `always` does not make an impossible or semantics-changing transformation legal. A recursive function does not become terminating through inlining. Hints do not enter the public contract hash, and should not change whether an otherwise identical program typechecks.

One function carries one inline-family choice. Using `#[noinline]` and `#[inline(never)]` together is an error even though their names differ. The compiler also rejects repeated `#[inline]` on the same construct.

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

# Repr: every current variant

Combine compatible choices in **one** attribute, such as `#[repr(C, packed(2))]`. Two `repr` attributes on one declaration are rejected. These variants are semantic layout contracts, not performance hints.

| Variant | Arguments and constraints | Effect |
| --- | --- | --- |
| `C` | No argument | Source-order represented fields, C-style alignment/padding for admitted single-constructor layouts. |
| `transparent` | Exactly one non-erased represented field; no other repr choice or field layout modifier | Same recorded size/alignment/layout as that field. |
| `align(N)` | Nonzero power-of-two `u64`; may not lower natural alignment | Raise aggregate alignment, or an admitted field's alignment. |
| `packed` | No argument; equivalent to `packed(1)` | Cap field placement alignment at 1. |
| `packed(N)` | Nonzero power-of-two `u64` | Cap effective field placement alignment at `N`. |
| `u8`, `u16`, `u32`, `u64` | One fixed-discriminant choice on `data`; not a record | Record discriminant width 1, 2, 4, or 8 bytes. Current native representation remains a uniform word; this is not full packed-enum lowering. |

`transparent` combines with nothing else. `packed` and aggregate `align` conflict. A fixed discriminant combines with no other repr choice. `C` can combine with an alignment floor or packing cap. Duplicate variant items, unknown items, empty lists/items, zero/non-power-of-two alignments, and malformed arguments are errors.

Field `repr(align(N))` and `repr(packed(N))` modifiers are admitted on fields of a **repr(C) record**. They can prefix the field declaration or its type; the two positions merge and the duplicate rules still apply. `C`, `transparent`, and discriminant choices are not field modifiers. Erased fields have no runtime slot to modify.

Read [type layout](24-type-layout.md) for byte offsets, natural versus effective alignment, packed-copy/borrow rules, fixed-field restrictions, FFI admission, and future unboxed/unpacked forms.

# Safety assertions and module policy

Every safety attribute in the index above has its own checked definition contract. [The safety reference](22-safety-assertions-and-llvm.md) explains each assertion, the Result must-use rule, and the distinction between source assertions and generated LLVM annotations.

`#[profile("jpl")]` prefixes a module header. It enforces the implemented recursion/allocation restrictions, complexity 10 and lines 60, the relevant exported-function runtime assertion minimum of 2, no macros/raw pointers, and warnings policy. A definition can strengthen the profile, not weaken it. There is no CLI `--profile` switch implied by this attribute.

`deny-warnings` is also valid on a module header. Other function assertions cannot simply be moved there to create a blanket module policy. Metadata is permitted there; arbitrary macro-owned module-header attributes are not.

# Foreign aggregate crossing: byval and sret

These **declaration fragments require external symbols** and their complete ABI/effect contracts:

```cinch
#[byval]
foreign import ccall "sum_big" sum-big :: Big ->{Foreign Lib, Diverge} I64

#[sret]
foreign import ccall "make_big" make-big :: I64 ->{Foreign Lib, Diverge} Big
```

`Big` must have an admitted explicit aggregate layout. The unannotated aggregate-parameter default is a pointer to a call-scoped layout copy, not a general by-value C argument convention. `byval` selects the admitted copy crossing, whose current LLVM declaration uses `byref(layout)` to state the pointee layout without silently changing the machine convention. It is not simply a request to paste LLVM's `byval` keyword.

`sret` admits an aggregate result through an explicit result-out buffer and a void LLVM result. A plain aggregate result without that admitted crossing is rejected. Scalars do not need these attributes; `byval` with no aggregate parameter and `sret` with no aggregate result are errors. Reserved intrinsic declarations are not real foreign boundaries and cannot acquire this convention through the annotation.

Neither attribute proves C code correct, establishes ownership/deallocation, or allows unwinding. The external declaration/object must agree with the recorded calling convention. See [FFI and primitives](15-ffi-layout-and-attributes.md) and [type layout](24-type-layout.md#foreign-calls-and-layout).

# Metadata and macro-owned attributes

`#[meta(key = "value")]` is nonsemantic metadata: the current compiler accepts its raw payload without interpreting it as executable arguments or a proof. The current built-in accepted name is exactly **`meta`**. A dotted name such as `meta.reader` is not automatically accepted merely because the design speaks of a metadata namespace; it requires an applicable registered macro-owned name or is rejected. Duplicate `meta` at an ordinary attachment site is rejected like other compiler-owned names. The current module-header validator does not apply that repeat check to `meta`; it does explicitly reject repeated `profile` and `deny-warnings`.

A macro signature declares its attribute names explicitly:

```text
macro decorate attributes (reader-note) :: Syntax -> Declarations
```

Registered macro-owned names are admitted through the macro attribute mechanism and revalidated at expansion attachment sites. The macro owns its argument/repetition policy. Registration is not a way to redefine `repr` or bypass compiler-owned validation. Generated attributes retain expansion provenance; unknown undeclared attributes fail rather than disappearing silently.

<!-- cinch:run exit=0 -->
```cinch
#[meta(owner = "examples")]
module Main where
  #[meta(reason = "format demonstration")]
  #[inline(always)]
  answer :: Int
  answer = 42

  main :: Int
  main = if #[weight=2000] answer == 42 then 0 else 1
```

# Duplicates, identity, and common mistakes

At ordinary attachment sites, repeated compiler-owned names are errors, subject to the module-metadata exception above. Family rules additionally reject different names assigning the same choice: `inline(never)` plus `noinline`, or `likely` plus `weight(2000)` on one outcome. Macro-owned names have their own repetition policy. Separate allowed attributes can be written on successive lines.

Layout/ABI facts and verified safety assertions participate in interface contracts. Changing them can require dependent rechecking or an upgrade/migration compatibility decision. Inlining and branch weights are representation hints and do not change the contract-only interface hash. Nonsemantic metadata is not a proof or a runtime effect.

LLVM output names such as `nonnull`, `noundef`, `willreturn`, `nofree`, `captures(none)`, or `!range` are **not accepted general source attributes**. The compiler derives them from checked facts. `immarg`, `uses`, `when`, `justify`, and `unsafe` in a primitive body are distinct syntax, not `#[...]` attributes. `ignore result` is an expression; `#[ignore]` is not the Result-discard mechanism.

# Planned and provisional attribute spellings

<span class="support-tag support-tag--planned">Coming soon</span> the selected layout/allocator designs include the following proposed spellings. They are **not accepted compiler-owned attributes today**, and their exact syntax remains subject to the implementing design.

| Design spelling | Intended behaviour | Owning design |
| --- | --- | --- |
| `#[repr(unboxed)]` | Store an admitted value type inline; proposed spelling for the value-type direction. | [Layouts and specialization](../design/layouts-and-specialization.md) |
| `#[unpack]`, `#[nounpack]` | Request or override inlining of a fixed field's representation into its parent. | [Layouts and specialization](../design/layouts-and-specialization.md) |
| `#[global-allocator]`-style declaration | Select a user-supplied program allocator. `global_allocator` in design prose is a Rust analogy, not an accepted Cinch alias. | [Allocators](../design/allocators.md) |
| `#[fip]` | Check the planned fully-in-place update/reuse discipline. | [Allocators](../design/allocators.md) |
| `#[alloc("bounded(n)")]` | Extend allocation checking to proved bounds; current `alloc` accepts only `"once"`. | [Allocators](../design/allocators.md) |

`#[may-fault]` appears in the [no-faults design discussion](../design/no-faults.md) as an alternative, not an accepted attribute or a committed escape hatch. Do not mark arbitrary discussion alternatives as promised syntax. The current no-faults APIs use checked evidence and defined exhaustion boundaries.

Implementation authority: `compiler/src/lexer.rs::lex_attr`, `parser.rs::parse_attr_list`, `check_decls.rs::validate_attr_site`, `validate_module_attrs`, `weight_value`, `parse_repr_args`, and `SafetySet::fill_from`. Behaviour suites: `compiler/tests/attributes.rs`, `safety.rs`, `llvm_contract.rs`, `ffi.rs`, and `prim.rs`. The [attribute specification](../specifications/attribute-annotations-specification.md) preserves intended semantics and dated amendments; current acceptance follows the Rust paths listed here.
