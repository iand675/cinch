# Attributes and pragmas: the complete reference

Cinch writes a source attribute as `#[...]`. The Rust compiler recognises exactly 22 compiler-owned names and rejects every other name that no macro declares. This page indexes them: exact spelling, admitted positions, argument validation, the effect on checking and code generation, and the current status. [Type layout](24-type-layout.md) works through the representation choices, and [safety assertions](22-safety-assertions-and-llvm.md) works through the definition checks in full.

An attribute is not an ordinary name and cannot be shadowed. An attribute never grants permission: a hint leaves the type, effect, ownership, coverage, and proof rules untouched, and a semantic assertion must pass its check. Every name in the index below is implemented, so each carries `Current`; [planned spellings](#planned-and-provisional-attribute-spellings) are listed separately and are not accepted today.

# Index of compiler-owned names

One row per name the compiler recognises. Variant spellings such as `inline(always)` are forms of one name, not separate attributes.

| Spelling | Where it may appear | What the compiler does | Status |
| --- | --- | --- | --- |
| `#[meta(...)]` | Any declaration, constructor, field, clause, alternative, guard, `if` position, and the module header | Stores opaque tooling metadata; no check and no runtime effect | Current |
| `#[weight(n)]` | Function clause, `case` alternative, guard, `if` condition, `if` branch | Sets the branch weight of that outcome | Current |
| `#[likely]` | The same decision positions as `weight` | Sets weight 2000 | Current |
| `#[unlikely]` | The same decision positions as `weight` | Sets weight 1 | Current |
| `#[cold]` | Decision positions, or a function declaration | Sets weight 0 on an outcome, or marks the function entry cold | Current |
| `#[inline]` | Function declaration | Desirable-inlining hint | Current |
| `#[noinline]` | Function declaration | Never-inline hint | Current |
| `#[repr(...)]` | `data` and `record` declarations, and fields of a `repr(C)` record | Layout contract, recorded in the interface | Current |
| `#[recursion("...")]` | Function declaration | Checks a recursion claim (`"none"`, `"tail"`, `"bounded(b)"`) | Current |
| `#[stack("...")]` | Function declaration | Checks a stack claim (`"constant"`, `"bounded(b)"`) | Current |
| `#[no-alloc]` | Function declaration | Checks that the body runs no allocation site | Current |
| `#[alloc("once")]` | Function declaration | Checks the single-allocation-site-per-call bound | Current |
| `#[max-complexity(n)]` | Function declaration | Checks that branches plus one is at most `n` | Current |
| `#[max-lines(n)]` | Function declaration | Checks that the body's source line span is at most `n` | Current |
| `#[assert-min(n)]` | Function declaration | Checks that at least `n` runtime checks run in the body | Current |
| `#[checked-params]` | Function declaration | Checks that each parameter is validated before first use | Current |
| `#[no-macros]` | Function declaration | Checks that the body carries no expansion provenance | Current |
| `#[no-raw-ptr]` | Function declaration | Checks the raw-pointer and opaque-type rule | Current |
| `#[deny-warnings]` | Function declaration, or the module header | Turns in-scope warnings into errors | Current |
| `#[profile("jpl")]` | Module header | Applies the JPL rule profile to the module | Current |
| `#[byval]` | Real `foreign import` or `foreign export` declaration | Selects the aggregate parameter-copy crossing | Current |
| `#[sret]` | Real `foreign import` or `foreign export` declaration | Selects the aggregate result-out crossing | Current |

A macro may declare further names with `macro name attributes (a, b) :: ...`; those names are admitted at attachment sites and validated by the macro. Nothing else is: an undeclared name is a check error, never inert metadata.

# Syntax and attachment

The name follows `#[` immediately. Nothing may separate the three parts of an attribute: not `#[` from the name, and not the name from `(` or `=` or `]`.

```text
#[inline]
#[inline(always)]
#[weight(2000)]
#[recursion("tail")]
#[repr(C, align(16))]
#[meta(owner = "parser", revision = 1)]
```

The name is a run of ASCII alphanumerics, `_`, `'`, `.`, and `-`, and it may not begin or end with `.` or `-`. Every form below is rejected before any name is looked up.

| Written | Compiler output |
| --- | --- |
| `#[ weight(2)]` | `cinch: Main.cinch: 2:3: an attribute needs a name` |
| `#[weight (2)]` | `cinch: Main.cinch: 2:3: unterminated attribute` |
| `#[weight = 2000]` | `cinch: Main.cinch: 3:12: unterminated attribute` |
| `#[!x]` | `cinch: Main.cinch: 2:3: an attribute needs a name` |
| `#[inline` | `cinch: Main.cinch: 2:3: unterminated attribute` |

Three argument forms exist. `#[name]` takes none. `#[name(...)]` captures the balanced text between the parentheses and keeps quoted strings opaque while scanning, so a `)` inside a string does not close the list. `#[name=literal]` captures an attached `=` tail up to `]`; the `=` must touch the name, which is why `#[weight=2000]` works and `#[weight = 2000]` does not. An unterminated list or string is a lexical error.

Each name validates its own argument text, so the same syntax produces different refusals. `#[meta]` is the only name that accepts its payload without interpreting it; a payload such as `#[meta(garbage !!! = = ,)]` checks. `#[repr(...)]` requires parentheses, and `#[repr=C]` is refused by the representation validator.

## Attachment positions

| Position | Written as | Names admitted |
| --- | --- | --- |
| Module header | before `module` | `meta`, `profile`, `deny-warnings` |
| Declaration | before the declaration, including across a line break | `meta` on any declaration; `repr` on `data`/`record`; `inline`, `noinline`, `cold`, and the safety assertions on function definitions; `byval`, `sret` on foreign declarations |
| Constructor | before a `data` constructor | `meta` |
| Record field | before the field, or before its type | `meta`; `repr(align(N))`, `repr(packed)`, `repr(packed(N))` on a `repr(C)` record |
| Function clause | between the function name and its patterns | `meta`, `weight`, `likely`, `unlikely`, `cold` |
| `case` alternative | before the alternative's pattern | `meta`, `weight`, `likely`, `unlikely`, `cold` |
| Guard | after `\|` | `meta`, `weight`, `likely`, `unlikely`, `cold` |
| `if` condition | after `if` | `meta`, `weight`, `likely`, `unlikely`, `cold` |
| `if` branch | before the branch expression | `meta`, `weight`, `likely`, `unlikely`, `cold` |

Layout of the source is insensitive here: an attribute may sit on the line above its construct, and the parser drops the layout separator that the newline inserted. Placement is enforced per position by the checker, not by the lexer, so a parsed attribute at the wrong position fails with a named placement error rather than a syntax error.

# Metadata: `meta`

`#[meta(...)]` carries tooling metadata. The checker accepts the payload as raw text, stores nothing in the contract, emits nothing to the backend, and does not interpret keys or values. Accepted positions are every position in the table above, including the module header and codata observations.

Two `meta` attributes on one construct are an error, and the message is the generic duplicate-name one:

```text
cinch: 3:3: two `meta` attributes on one construct: duplicate attribute names are an error unless a macro declares a multi-attribute form (attribute annotations specification section 1.3) (in Main.cinch)
```

Two attributes on one definition at two different positions are fine: `#[meta(a = 1)]` on a signature and `#[meta(b = 2)]` on that function's equation both check. The module header is the same: repeated `meta` there checks, while repeated `profile` and repeated `deny-warnings` are refused explicitly.

A dotted name is not automatically in the `meta` namespace. `#[meta.reader(x = 1)]` is refused like any other undeclared name:

```text
cinch: 2:3: unknown attribute `meta.reader`: outside the `meta` namespace, an attribute name must be compiler-known or declared by a macro (`macro name attributes (meta.reader) :: ...`) (in Main.cinch)
```

This program attaches metadata at every admitted position and checks cleanly:

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

## Macro-owned names

A macro signature declares the attribute names its expansions may use:

```text
macro decorate attributes (codec) :: Syntax -> Declarations
```

Registration happens when the macro signature is collected. A second macro declaring an existing name is refused at registration:

```text
cinch: 3:9: attribute name `codec` is already declared by macro `first` (declared at 2:9); macro attribute namespaces cannot collide silently (in Main.cinch)
```

Compiler-owned validation runs before ownership is consulted, so declaring a compiler-owned name does not override its rules. With `macro grab attributes (weight) :: ...` in scope, `#[weight(2)]` on a function declaration still fails with the ordinary weight placement error. Macro-emitted attributes revalidate at expansion sites, and unknown undeclared names fail rather than disappearing.

# Branch weights: `weight`, `likely`, `unlikely`, and `cold`

Weights are ratios inside one decision point, not probabilities across a program. Values run from 0 through 4294967295. Zero describes negligible likelihood; it does not make a branch unreachable and does not permit its removal.

| Spelling | Argument validation | Weight assigned |
| --- | --- | --- |
| `#[weight(n)]`, `#[weight=n]` | One non-negative integer literal representable in 32 bits | `n` |
| `#[likely]` | No arguments | 2000 |
| `#[unlikely]` | No arguments | 1 |
| `#[cold]` on an outcome | No arguments | 0 |
| `#[cold]` on a function declaration | No arguments | No weight; marks the function entry cold |

These are fragments inside a function definition, so they carry no `module` header and no verification marker:

```cinch
choose #[weight(2000)] True = 1
choose #[weight(1)] False = 0

result = case flag of
  #[likely] True -> 1
  #[unlikely] False -> 0

result = if #[likely] flag then 1 else 0
result = if flag then #[weight(2000)] 1 else #[weight(1)] 0
```

A clause weight sits between the function name and its patterns, an alternative weight before the pattern, and a guard weight after `|`. The `if #[weight(w)] condition` form is a pair: it supplies the true weight `w` and completes the false side with weight 1. An explicit `else` annotation replaces that 1. Annotating the true side again, before the condition and before `then`, is refused:

```text
cinch: 3:9: the true branch of this `if` is annotated twice (before the condition and before the `then` expression); annotate one (attribute annotations specification section 2.1) (in Main.cinch)
```

Weights are all or nothing per decision point. Partial annotation drops every weight at that decision and reports `incomplete-branch-weights`; a guard's likelihood supplies no coverage for unannotated alternatives. Under `#[deny-warnings]` or the `jpl` profile that report becomes an error. `incoherent-weights` stays informational: it reports a weighted clause counted at more than one path (wildcard, or-pattern, or guard fallthrough) and keeps the weights usable.

Pattern-column compilation marginalizes clause weights by summing the reachable clause weights for each alternative. Optimizer transformations preserve their conditional meaning, rescale merged decisions where required, or drop weights where the contract requires it; see [rewrites and optimization](12-rewrites-and-optimization.md). A weight is never evidence that an arm is unreachable.

This program exercises clause, alternative, guard, condition, and branch positions, and runs:

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

The cold branch still returns 7. `#[cold]` before a function signature instead describes the function entry, and it lowers as a function cold hint rather than as a numeric weight on every branch inside the function.

Refusals name the family and the position:

| Written | Compiler output |
| --- | --- |
| `#[weight(2)]` on a function declaration | `` `#[weight]` annotates one alternative of a decision point (attribute annotations specification sections 2 and 3); a function declaration carries `#[inline]`, `#[noinline]` and `#[cold]` (section 5), and a weight on a clause is written between the name and its patterns `` |
| `#[weight(4294967296)]` | `` a weight is a non-negative integer literal representable in 32 bits (attribute annotations specification section 2) `` |
| `#[weight(x)]` | `` `#[weight(n)]` takes one non-negative integer literal (attribute annotations specification section 2) `` |
| `#[likely(1)]` | `` `#[likely]` takes no arguments (attribute annotations specification section 3) `` |
| `#[cold(1)]` on a function declaration | `` `#[cold]` takes no arguments (attribute annotations specification sections 3 and 5) `` |
| `#[likely] #[weight(2000)]` on one outcome | `` one alternative carries one weight: `weight` and the weight attribute before it both assign one (attribute annotations specification section 2.1) `` |

# Inline hints: `inline`, `noinline`, and function `cold`

| Spelling | Meaning | LLVM attribute group word |
| --- | --- | --- |
| `#[inline]` | Ordinary discretionary inlining | `inlinehint` |
| `#[inline(always)]` | Stronger always-inline hint | `alwaysinline` |
| `#[inline(never)]` | Never inline | `noinline` |
| `#[noinline]` | The same hint as `inline(never)` | `noinline` |
| `#[cold]` | Function entry is unlikely to run | `cold` |

The words in the last column are real: `cinch emit-ir` on a module with one function per hint prints attribute groups containing `inlinehint`, `alwaysinline`, `noinline`, and `cold` respectively. The hints do not enter the interface hash, do not change whether an identical program typechecks, and do not make an illegal transformation legal.

One function carries one inline hint, and the hint may ride the signature or an equation. Using `#[noinline]` and `#[inline(never)]` on one construct, or on two positions of one definition, is refused:

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

| Written | Compiler output |
| --- | --- |
| `#[inline]` and `#[noinline]` on two positions of one definition | `` Main.f carries two inline hints: `#[inline]`, `#[inline(always)]`, `#[inline(never)]` and `#[noinline]` are one hint family (attribute annotations specification section 5) `` |
| `#[inline]` twice on one construct | `` two `inline` attributes on one construct: duplicate attribute names are an error unless a macro declares a multi-attribute form (attribute annotations specification section 1.3) `` |
| `#[inline(sometimes)]` | `` `#[inline]` takes no argument or one of `(always)` and `(never)` (attribute annotations specification section 5) `` |
| `#[noinline(always)]` | `` `#[noinline]` takes no arguments (attribute annotations specification section 5) `` |
| `#[inline]` on a `case` alternative | `` `#[inline]` is a hint on a function declaration (attribute annotations specification section 5) `` |

Every hint form checks and runs:

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

  main :: Int
  main = if a 0 == 1 && b 0 == 2 && c 0 == 3 && d 0 == 4 then 0 else 1
```

The compact `=` tail reaches the same validators, which is why `#[weight=2000]` and `#[recursion="tail"]` are accepted while the space-separated historical spellings are not:

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

# Repr: every current variant

`#[repr(...)]` is a semantic layout contract, not a performance hint. Write every compatible choice in one attribute, such as `#[repr(C, packed(2))]`; a second `repr` attribute on one declaration is a duplicate-name error. The variants are:

| Variant | Arguments and constraints | Effect |
| --- | --- | --- |
| `C` | No argument | Source-order represented fields, C-style alignment and padding, for an admitted single-constructor layout |
| `transparent` | Exactly one represented non-erased field; combines with no other choice and with no field modifier | Same recorded size, alignment, and layout as that field |
| `align(N)` | Nonzero power-of-two `u64`; may not lower natural alignment | Raises the aggregate's alignment, or an admitted field's alignment |
| `packed` | No argument; the same as `packed(1)` | Caps field placement alignment at 1 |
| `packed(N)` | Nonzero power-of-two `u64` | Caps effective field placement alignment at `N` |
| `u8`, `u16`, `u32`, `u64` | One fixed-discriminant choice on `data`, not on `record`; combines with no other choice | Records a discriminant width of 1, 2, 4, or 8 bytes |

`transparent` combines with nothing. `packed` and aggregate `align` conflict. A fixed discriminant combines with no other choice. `C` combines with an alignment floor or a packing cap. Duplicate variant items, unknown items, empty lists, empty items, zero or non-power-of-two alignments, and malformed arguments are all errors.

Current behaviour for a fixed discriminant is a recorded fact: the payload layout stays the native uniform word, and the `repr` spelling plus the discriminant width are what the interface records. Page [type layout](24-type-layout.md) shows the recorded facts, byte offsets, natural versus effective alignment, packed-borrow rules, and FFI admission.

Field modifiers are `repr(align(N))`, `repr(packed)`, and `repr(packed(N))`, admitted on fields of a `repr(C)` record only. A modifier may prefix the field declaration or the field's type; both positions merge into one attribute list, so writing the same modifier in both places is a duplicate name. `C`, `transparent`, and the discriminant choices are not field modifiers, and an erased field has no runtime slot to modify.

This program declares one aggregate per admitted shape and runs:

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

Rejections, with the exact message text:

| Written | Compiler output |
| --- | --- |
| `#[repr]` or `#[repr()]` | `` `#[repr(...)]` takes at least one representation (attribute annotations specification section 4.1) `` |
| `#[repr=C]` | `` `#[repr(...)]` takes its representations in parentheses (attribute annotations specification section 1) `` |
| `#[repr(Rust)]` | `` `Rust` is not a representation (attribute annotations specification section 4.1 expects `C`, `transparent`, `align(N)`, `packed`, `packed(N)`, or `u8`/`u16`/`u32`/`u64`) `` |
| `#[repr(align(0))]`, `#[repr(align(3))]` | `` `repr(align(N))` takes a non-zero power of two (attribute annotations specification section 4.1) `` |
| `#[repr(C, align(4))]` on a record of two `I64` fields | `` `repr(align(4))` only raises: Pair's natural alignment is 8 (attribute annotations specification section 4.2) `` |
| `#[repr(C, packed, align(8))]` | `` `repr(packed)` removes padding and `repr(align(N))` raises alignment; they conflict (attribute annotations specification section 4.1) `` |
| `#[repr(transparent)]` with two fields | `` `repr(transparent)` needs exactly one represented field, found 2 (attribute annotations specification section 4.1) `` |
| `#[repr(u16)]` on a `record` | `` a `record` has no constructor discriminant (attribute annotations specification section 4.1) `` |
| `#[repr(C)]` on a multi-constructor `data` | `` a multi-constructor `data` has no field layout to compute: `repr(C)`, `align` and `packed` apply to one constructor's source-order fields (attribute annotations specification section 4.1); use `#[repr(u8\|u16\|u32\|u64)]` for a fixed discriminant `` |
| `#[repr(C)]` on `codata` | `` `#[repr(C)]` attaches to a `data` or `record` declaration (attribute annotations specification section 4.1) `` |
| `#[repr(C)]` on a `case` alternative or a constructor | `` `#[repr(...)]` attaches to a `data` or `record` declaration (attribute annotations specification section 4.1) `` |
| field modifier on a record without `repr(C)` | `` a field's representation applies to fields of a `repr(C)` record (attribute annotations specification section 9's open decision admits them there) `` |
| `#[repr(C)]` on a field | `` a field takes `repr(align(N))` or `repr(packed)`/`repr(packed(N))`, not `C` (attribute annotations specification section 4.1) `` |
| field modifier on an erased field | `` field ok is erased and has no representation to lay out (attribute annotations specification section 4) `` |
| two `repr` attributes on one declaration | `` two `repr` attributes on one construct: duplicate attribute names are an error unless a macro declares a multi-attribute form (attribute annotations specification section 1.3) `` |

# Safety assertions

Eleven names assert a property of one definition, written in fourteen forms across the argument variants below. Each attaches to a function declaration, meaning a value signature or a top-level equation; the two positions of one definition are one attachment site set, so one definition carries at most one of each assertion. Every assertion is checked at the definition, and a satisfied assertion becomes a contract fact.

| Spelling | Argument validation | What is checked | Recorded fact |
| --- | --- | --- | --- |
| `#[recursion("none")]` | One quoted string, exactly `"none"` | No recursive call in the definition's group, self or mutual | `recursion=none` |
| `#[recursion("tail")]` | One quoted string, exactly `"tail"` | Every recursive call in the self or mutual group sits in tail position | `recursion=tail` |
| `#[recursion("bounded(b)")]` | `"bounded(n)"` with `n` a natural number, or a name | A static depth bound, proved structurally when the argument names the decreasing value or size index, else by the conservative literal call-graph ladder | `recursion=bounded(b)` |
| `#[stack("constant")]` | One quoted string, exactly `"constant"` | Tail recursion, or no recursion | `stack=constant` |
| `#[stack("bounded(b)")]` | `"bounded(n)"` with the same grammar | A static stack bound, by the same two proof procedures | `stack=bounded(b)` |
| `#[no-alloc]` | No arguments | No allocation site in the body: constructor, local function or lambda closure, memo thunk, `Mut` cell, `Bytes` buffer, foreign aggregate | `no-alloc` |
| `#[alloc("once")]` | One quoted string, exactly `"once"` | At most one allocation site runs per call, and no site sits under a closure | `alloc=once` |
| `#[max-complexity(n)]` | One natural number literal fitting `u64` | Cyclomatic complexity, counted as decision branches plus one, is at most `n` | `complexity<=n` |
| `#[max-lines(n)]` | One natural number literal fitting `u64` | The source line span of the checked body is at most `n` | `lines<=n` |
| `#[assert-min(n)]` | One natural number literal fitting `u64` | At least `n` runtime `decide` checks run in the body; folded proof obligations do not count | `assert-min=n` |
| `#[checked-params]` | No arguments | Every parameter is validated by a refutable `case` match or a runtime `decide` before its first use | `checked-params` |
| `#[no-macros]` | No arguments | The body carries no macro expansion provenance | `no-macros` |
| `#[no-raw-ptr]` | No arguments | No raw pointer or opaque type in the signature, and no foreign pointer operation in the body | `no-raw-ptr` |
| `#[deny-warnings]` | No arguments | Warnings in the definition's scope become errors | `deny-warnings` |

A `bounded(b)` argument is a natural number or a name, never an arithmetic expression. A name matches only a structural bound that the proof reports under that name; `#[stack("bounded(xs)")]` on a non-recursive function is refused because the established bound is the numeric depth 0. A bound the checker cannot establish is refused rather than trusted, and the refusal says which method it tried:

```text
cinch: 2:3: `#[recursion "bounded(3)"]` on Main.ping: the recursive group ["Main.ping", "Main.pong"] is mutual: no depth bound is established for a mutual cycle (bounds that cannot be established are refused, never trusted; decision D113)
```

The argument validators are shared, so the refusals are uniform:

| Written | Compiler output |
| --- | --- |
| `#[recursion("strict")]` | `` `strict` is not a recursion assertion: `#[recursion]` takes `"none"`, `"tail"` or `"bounded(n)"` (attribute annotations specification section 10) `` |
| `#[recursion(tail)]` | `` `#[recursion]` takes one string argument in quotes (attribute annotations specification sections 1 and 10) `` |
| `#[stack("linear")]` | `` `linear` is not a stack assertion: `#[stack]` takes `"constant"` or `"bounded(n)"` (attribute annotations specification section 10) `` |
| `#[alloc("twice")]` | `` `#[alloc]` takes `"once"` (the initialization pattern of attribute annotations specification section 10) `` |
| `#[max-lines(walrus)]` | `` `#[max-lines]` takes one natural number literal (attribute annotations specification section 10) `` |
| `#[no-alloc(1)]` | `` `#[no-alloc]` takes no arguments (attribute annotations specification section 10) `` |
| any assertion on a `data` declaration | `` `#[recursion]`, `#[stack]`, `#[no-alloc]`, `#[alloc]`, `#[max-complexity]`, `#[max-lines]`, `#[assert-min]`, `#[checked-params]`, `#[no-macros]`, `#[no-raw-ptr]` and `#[deny-warnings]` attach to a function definition (a signature or a top-level equation; attribute annotations specification section 10) `` |
| two `#[max-lines]` attachments on one definition | `` Main.f carries two `#[max-lines]` assertions: one definition carries one (attribute annotations specification sections 1.3 and 10) `` |

A violation names the attribute, the asserted rule, and the offending site:

| Assertion | Example violation text |
| --- | --- |
| `#[recursion("none")]` | `` `#[recursion "none"]` on Main.loopy asserts the no-recursion rule (JPL rule 1): the recursive call at this site (self or mutual) `` |
| `#[recursion("tail")]` | `` the call to `Main.notail` at this site is not in tail position `` |
| `#[recursion("bounded(b)")]` | `` the established bound is call-graph (the literal counter ladder: depth <= 3), which does not bound the depth by 3 `` |
| `#[no-alloc]` | `` `#[no-alloc]` on Main.cons1 asserts the no-allocation rule (JPL rule 3): the constructor `Cons` is constructed here `` |
| `#[alloc("once")]` | `` the constructor `Cons` is constructed here as well: at most 1 allocation site(s) run per call `` |
| `#[max-complexity(n)]` | `` the body's cyclomatic complexity (branches + 1) is 3, above 1 `` |
| `#[max-lines(n)]` | `` the body spans 3 source lines, above 2 `` |
| `#[assert-min(n)]` | `` the body has 1 runtime `decide` check(s) (proof obligations do not count), below 2 `` |
| `#[checked-params]` | `` the parameter `x` is used here before any validating check (a refutable `case` match or a runtime `decide`) `` |
| `#[no-macros]` | `` a macro expansion occurred at this site `` |
| `#[no-raw-ptr]` | `` a raw pointer appears in the signature `` |

One rejection, in full:

<!-- cinch:reject contains="boundary-validation rule (JPL rule 7)" -->
```cinch
module Main where
  #[checked-params]
  raw :: Int -> Int
  raw x = x + 1

  main :: Int
  main = 0
```

A `decide` whose operands are compile-time values is a folded obligation, so it does not count toward `#[assert-min]`. Only a check the emitted program runs counts.

Verified assertions are contract facts. `cinch check` writes them to `build/Main.cinchif` in an `assertions` array of definition name and normalized fact spellings, and the same facts enter `iface_hash`, so a dependent module rechecks when an assertion changes. This program asserts eleven names in thirteen of the fourteen forms and runs:

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

Those facts are readable from the interface:

```sh
cinch check Main.cinch
python3 -c 'import json; from pathlib import Path; print(json.dumps([[a["name"], a["facts"]] for a in json.loads(Path("build/Main.cinchif").read_text())["assertions"]]))'
```

```text
[["bounded", ["assert-min=1"]], ["double", ["complexity<=2", "deny-warnings", "lines<=2", "no-alloc", "no-macros", "no-raw-ptr", "recursion=none"]], ["lengthOf", ["stack=bounded(xs)"]], ["sumTo", ["recursion=tail", "stack=constant"]], ["validated", ["checked-params"]], ["wrap", ["alloc=once"]]]
```

The array is sorted by definition name, and each definition lists its facts sorted. An assertion of `#[recursion("bounded(3)")]` would add `recursion=bounded(3)` to its definition's row, with the bound spelled as written.

# Module policy: `profile` and module `deny-warnings`

A module header admits three names: `meta`, `profile`, and `deny-warnings`. Everything else is refused there:

```text
cinch: 1:1: unknown module attribute `inline`: a module header carries `#[profile "jpl"]`, `#[deny-warnings]` and `#[meta(...)]` (attribute annotations specification section 10) (in Main.cinch)
```

`#[profile("jpl")]` takes the single quoted string `"jpl"` and applies the JPL rule numbers to every definition in the module:

| Rule | Set by the profile |
| --- | --- |
| Recursion | A recursive definition without its own `recursion` or `stack` assertion must be tail recursive or establish a static bound |
| Allocation | Allocation is refused unless the definition asserts `#[alloc("once")]` or `#[no-alloc]`, so a site outside the initialization pattern fails |
| Complexity and lines | Complexity at most 10, body at most 60 source lines |
| Runtime checks | At least 2 runtime `decide` checks on each exported definition |
| Macros and pointers | No macro expansion, no raw pointer, no opaque signature |
| Warnings | Warnings in scope are errors |

A definition's own assertions may be stricter than the profile and never looser, and a looser written bound is refused. `deny-warnings` on the module header has the same effect as the profile's warnings rule, and `#[checked-params]` stays a per-definition attribute that the profile does not imply. There is no command-line profile switch: the attribute on the module header is the only way to select it.

```text
cinch: 2:3: `#[max-complexity 12]` is looser than the `jpl` profile's rule (complexity 10): a definition's own attributes may be stricter than the profile, never looser (attribute annotations specification section 10; decision D114)
```

This module runs under the profile:

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

Refusals at the header:

| Written | Compiler output |
| --- | --- |
| `#[profile("nasa")]` | `` `#[profile]` takes `"jpl"` (the JPL rule profile of attribute annotations specification section 10) `` |
| `#[profile("jpl")]` twice | `` one module carries one `#[profile "jpl"]` attribute (attribute annotations specification section 1.3) `` |
| `#[deny-warnings]` twice | `` one module carries one `#[deny-warnings]` attribute (attribute annotations specification section 1.3) `` |
| `#[profile("jpl")]` on a function declaration | `` `#[profile "jpl"]` annotates a module header (attribute annotations specification section 10) `` |

# Foreign aggregate crossing: byval and sret

These two names select an aggregate crossing convention on a real `foreign import` or `foreign export` declaration. The declarations below are fragments, because the call needs the external symbols to link:

```cinch
#[byval]
foreign import ccall "sum_big" sum-big :: Big ->{Foreign Lib, Diverge} I64

#[sret]
foreign import ccall "make_big" make-big :: I64 ->{Foreign Lib, Diverge} Big
```

`Big` is a `#[repr(C)]` record with an admitted explicit layout. Without either attribute, an aggregate parameter crosses as a pointer to a call-scoped layout copy, and an aggregate result is refused. `#[byval]` selects the admitted parameter copy and spells the declaration with `byref(<layout>) align <n>`, which states the pointee layout without silently changing the machine convention; it is not LLVM's `byval` keyword. `#[sret]` selects the result-out buffer and spells a `void` LLVM result with an `sret(<layout>)` pointer parameter. Both are visible in `cinch emit-ir` output and are asserted by `compiler/tests/llvm_contract.rs`.

Neither name takes arguments. `#[byval]` with no aggregate parameter, `#[sret]` with no aggregate result, and either name on a reserved intrinsic declaration are refused:

| Written | Compiler output |
| --- | --- |
| `#[byval]` on a `CInt -> CInt` import | `` `#[byval]` spells aggregate parameters crossing by value; this binding marshals no aggregate parameter `` |
| `#[sret]` on a `CInt -> CInt` import | `` `#[sret]` spells an aggregate result crossing by value; this binding's result is not an aggregate `` |
| an aggregate result without `#[sret]` | `` a foreign result cannot be an aggregate: `Main.Big` follows the platform's by-value struct ABI, which this slice does not guess (foreign function interface specification section 2: reject unsupported layouts); return through an `Out` cell or an `Owned` pointer, or spell the by-value crossing `#[sret]` `` |
| `#[byval(1)]` | `` `#[byval]` takes no arguments `` |
| `#[byval]` on a function declaration | `` `#[byval]` and `#[sret]` attach to a `foreign` import or export declaration (the by-value aggregate ABI spelling of design/llvm-contract-attributes.md) `` |

The one rejection that needs a module of its own:

<!-- cinch:reject contains="this binding marshals no aggregate parameter" -->
```cinch
module Main where
  #[byval]
  foreign import ccall "fix_neg" neg :: CInt ->{Foreign Fix, Fail} CInt

  main :: Int
  main = 0
```

Neither attribute proves the C code correct, establishes ownership or deallocation, or permits unwinding across the boundary. See [FFI and primitives](15-ffi-layout-and-attributes.md) and [type layout](24-type-layout.md#foreign-calls-and-layout).

# Duplicates, identity, and what is not an attribute

A repeated name on one construct is an error. The two positions of one definition, its signature and its equations, merge: the inline family and the safety assertions reject a second attachment by name (`Main.f carries two inline hints`, `Main.f carries two #[max-lines] assertions`), and `#[cold]` is likewise one hint per function, while `#[meta]` may appear at both positions. Family rules also reject different names that assign the same choice: `inline(never)` with `noinline`, or `likely` with `weight(2000)` on one outcome. Macro-owned names have the macro's repetition policy.

Layout and ABI facts and verified safety assertions are part of the interface contract: `repr` choices land in the recorded layout table, assertions in the `assertions` array, and a change to either forces dependent rechecking. Inlining and branch weights are representation hints and never change the contract hash. Metadata is neither a proof nor a runtime effect.

The following are not attributes, and the compiler rejects them in `#[...]` position:

| Form | What it is | Where it is written |
| --- | --- | --- |
| `{-# ... #-}` | A block comment, because the lexer reads `{- ... -}` as a nested comment | Anywhere a comment is allowed; its text has no effect |
| `immarg`, `uses`, `when`, `justify` | Clauses of a target-IR primitive body | After the body's `{ ... }` fragment |
| `unsafe` | A declaration modifier | `unsafe prim ...` |
| `ignore e` | A term of type `Unit` that discards its argument on purpose (`ignore` is a builtin application, not syntax) | Expression position |
| `stage`, `macro`, `corec`, `proof`, `spec` | Declaration modifiers | Before the declaration name |
| `nonnull`, `noundef`, `willreturn`, `nofree`, `captures(none)`, `!range` | LLVM output names the backend derives from checked facts | Emitted IR only |

The `#[ignore]` attribute does not exist. `#[ignore]` on a declaration is an unknown name, and `#[ignore]` in expression position is a parse error:

```text
cinch: 2:3: unknown attribute `ignore`: outside the `meta` namespace, an attribute name must be compiler-known or declared by a macro (`macro name attributes (ignore) :: ...`) (in Main.cinch)
```

# Planned and provisional attribute spellings

<span class="support-tag support-tag--planned">Coming soon</span> the selected layout and allocator designs name these spellings. None is a compiler-owned attribute today, and the exact syntax stays subject to the implementing design.

| Design spelling | Intended behaviour | Owning design |
| --- | --- | --- |
| `#[repr(unboxed)]` | Store an admitted value type inline wherever it is used | [Layouts and specialization](../design/layouts-and-specialization.md) |
| `#[unpack]`, `#[nounpack]` | Request or override inlining of a fixed field's representation into its parent | [Layouts and specialization](../design/layouts-and-specialization.md) |
| `#[global-allocator]`-style declaration | Select a user-supplied program allocator; `global_allocator` in design prose is a Rust analogy, not a Cinch alias | [Allocators](../design/allocators.md) |
| `#[fip]` | Check the planned fully-in-place update and reuse discipline | [Allocators](../design/allocators.md) |
| `#[alloc("bounded(n)")]` | Extend allocation checking to proved bounds; current `alloc` accepts only `"once"` | [Allocators](../design/allocators.md) |

`#[may-fault]` appears in the [no-faults design discussion](../design/no-faults.md) as an alternative, not as an accepted attribute and not as a committed escape hatch. Discussion alternatives are not promised syntax; the no-faults APIs use checked evidence and defined exhaustion boundaries.

# Implementation authority

Lexing and parsing: `compiler/src/lexer.rs::lex_attr`, `compiler/src/parser.rs::parse_attr_list` and `parse_sig_or_equation`. Validation and checking: `compiler/src/check_decls.rs::validate_attr_site`, `validate_module_attrs`, `weight_value`, `parse_repr_args`, `SafetySet::fill_from`, and the per-assertion checks in `verify_definition_safety`. Recording: `compiler/src/interface.rs` (`AbiLayout`, `assertions`). Code generation: `compiler/src/codegen.rs` (attribute groups, branch-weight metadata, `byref`/`sret` spellings). Behaviour suites: `compiler/tests/attributes.rs`, `safety.rs`, `llvm_contract.rs`, `ffi.rs`, `prim.rs`, `repr_classes.rs`. The [attribute specification](../specifications/attribute-annotations-specification.md) carries the intended semantics and its dated amendments; current acceptance follows the Rust paths listed here.
