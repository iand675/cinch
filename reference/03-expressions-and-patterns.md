# Expressions, bindings, and patterns

This page is the surface-expression reference. The [tutorial](../learn/02-values-and-layout.md) introduces the same forms through runnable programs.

# Expression forms

| Form | Meaning and constraints |
| --- | --- |
| `f x y` | Curried application, associating left. |
| `\x y -> body` | Lambda; captured values retain usage and region restrictions. |
| `let declarations in body` | Sequential local group, with inference and eligible value generalization. |
| `letrec declarations in body` | Recursive group; recursion must be guarded under lambda, memo body, or delay. |
| `if condition then a else b` | Boolean selection; branches agree in result type and participate in usage/effect checking. |
| `case value of ...` | Source-order pattern matching with coverage and index-aware impossibility checks. |
| `(expression :: Type)` | Checked ascription; no implicit numeric conversion. |
| `()` / `(a, b)` | Unit / structural tuple. |
| `[a, b]` | `FromList` construction at the expected container type; defaults to `List`. |
| `{key => value}` | `FromMap` construction; unconstrained map literals default to ordered `Map`. |
| `Record { field = value }` | Named record construction. |
| `value.field` | Field projection; ownership is checked at projection places. |
| `f @{io = capability} argument` | Explicit named capability supplies for a call. |
| `with (io = capability) in body` | Lexically scoped capability supply. |
| `do ...` | Lexically selected sequencing provider. |
| `handle expression with handler` | Interpret admitted effect operations. |
| `[| expression |]`, `$(code)` | Typed quotation and splice in admitted staging context. |
| `name!(tokens)` | Hygienic macro invocation in the supported syntax context. |
| `forget ...`, `narrow ...` | Explicit refinement manipulation; see [refinement reference](06-refinements.md). |

Finite range syntax uses `Enum`: `[start .. end]` and `[start, next .. end]`. It is not an automatically infinite lazy list. The [Data.Enum reference](../library/Data.Enum.md) gives the class and the current [range support](06-refinements.md) discusses evidence at boundaries.

# Local declarations

Local groups admit signatures, equations, pattern bindings, local function equations, and memo bindings `~name = expression`. A `where` block belongs to an equation and can use its arguments. Ordinary local groups are sequential, not an implicit mutually recursive knot.

Irrefutable tuple destructuring in local bindings is supported. A memo binding requires a variable pattern. Haskell-style `~pattern` argument syntax is not the memo facility. Locally annotated polymorphic `forall`/`using` signatures remain restricted; move such a helper to a top-level explicit signature when needed.

# Patterns

A term-position typed hole is currently rejected; `_` is supported as a pattern wildcard.

Patterns include variables, `_`, `()`, integer literals, symbol literals, constructors with nested arguments, tuples, lists, named record fields, and as-patterns `name@pattern`. Repeating a binder in one pattern is rejected; equality belongs in an expression or guard.

Guards use `pattern | condition -> body` in a `case`, or the corresponding equation form. The guard must be `Bool`; its effects join the surrounding computation. A guarded row contributes no unconditional coverage. Keep an unguarded fallback when the guard can fail. Alternatives and equations are tried in source order; a catch-all makes later rows unreachable for those values.

Coverage accounts for constructors, Boolean alternatives, tuples, literal fallback, and admitted index refinements. An omitted reachable case is an error. A refuted indexed constructor is impossible and an explicit arm for it is rejected. [Dependent types](08-dependent-types-and-proofs.md) describe the rigid-index elimination boundary.

# Evaluation and construction

Ordinary bindings and applications are strict. Explicit thunks, memo groups, and guarded codata introduce their own evaluation rules. Constant collection literals may be folded at compilation; nonconstant elements retain runtime construction. The manifest records the fold boundary. Folding must not change effect order, ownership, or the chosen instance's semantics.

Implementation: `compiler/src/ast.rs`, `parser.rs`, `check_terms.rs`, `check_decls.rs`. Behaviour suites: `surface.rs`, `guard_coverage.rs`, `pattern_field_names.rs`, `letrec.rs`, `tail_calls.rs`.
