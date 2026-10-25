# SafeLang bootstrap compiler

A first compiler for the language designed in this repository. The surface
syntax is the draft 0.2 grammar in
[`specifications/surface-syntax-and-grammar.md`](../specifications/surface-syntax-and-grammar.md);
the static and dynamic goals are
[`specifications/core-language-specification.md`](../specifications/core-language-specification.md).
The language has no final name yet; `safelang` and the `.lang` extension are
placeholders.

This is a bootstrap slice: a small, honest subset implemented end to end
(source text to native executable) so the design can be exercised and
iterated on. It is not milestone 1 of the implementation plan in
[`design/open-decisions-and-implementation-plan.md`](../design/open-decisions-and-implementation-plan.md);
the dependent core is still ahead of us.

## Toolchain

Everything runs inside [devenv](https://devenv.sh) (nix):

```sh
devenv shell        # rustc, cargo, clang, llvm
safelang check examples/compiler/fib.lang
safelang run examples/compiler/fib.lang
safelang emit-ir examples/compiler/fib.lang -o fib.ll
safelang build examples/compiler/fib.lang -o fib
safelang-test       # cargo test
```

`SAFELANG_CLANG` selects the clang used for codegen and linking (devenv sets
it to the nix clang).

## Pipeline

```
source -> lexer -> layout -> parser (+ fixity pass) -> checker/elaborator
       -> core IR -> LLVM IR text -> clang -> executable
```

The backend emits LLVM IR text and drives `clang`; keeping LLVM out of
process keeps the nix toolchain simple while the backend is still moving.
The closure and dictionary ABI is documented in `src/core.rs`.

## What is implemented

- **Layout syntax** per the draft 0.2 rules (`where`, `of`, `let`, `in`,
  explicit-brace shielding, tab rejection, nested `{- -}` comments).
- **Declarations**: `module`, `import` (prelude only), `data`, `record`
  (single-constructor products with named fields), `signature`,
  `instance` / `implicit instance`, value signatures, equations (including
  `where` blocks), `fixity`, `foreign import ccall` for compiler
  intrinsics.
- **Terms**: variables and qualified names, constructors, integer literals,
  `()`, tuple and list literals (`[e1, e2]` desugars to prelude `Cons`
  chains), application, lambdas, `let ... in` (with irrefutable tuple
  destructuring), `if`, `case` (with nested constructor, tuple, list,
  literal and record patterns, and guards `p | e -> e`), record
  construction `R { x = e, ... }` and projection `e.x`, type ascription,
  infix chains with a real fixity pass (precedence 0-9, associativity,
  non-associative operators, `$`), prefix negation.
- **Types**: `Int` and `Nat` (mathematical, bignum-backed), the machine
  scalars `I8`-`I64`, `U8`-`U64`, `F32`, `F64`, `Char`, `Usize`, `Isize`,
  `Bytes`, `Bool`, `Unit`, algebraic data types with parameters,
  structural tuple types `(T1, ..., Tn)` (decision D1 in
  `CONFORMANCE.md`: this extends the draft EBNF type grammar), arrows,
  `forall`, `using (...) =>` constraints. Bidirectional checking with
  unification; this draft does not silently generalize, so top-level
  signatures state their quantifiers. Integer literals are unbounded
  decimal digit strings elaborated against their expected type (decision
  D10).
- **Type classes as the module stack**: `signature` declares a class,
  `instance` implements one, `implicit instance` enables unique dictionary
  resolution. Constraints elaborate to explicit dictionary parameters;
  methods elaborate to dictionary projection and application, with real
  dictionary passing at runtime. Operators (`+`, `==`, ...) are ordinary
  method names with declared fixities; the compiler has no builtin
  operators. The prelude is ordinary source (`prelude/Prelude.lang`) layered
  as `Eq`, `Ord`, `Add`, `Sub`, `Mul`, `Neg`, with `Nat`/`Int`, machine
  scalar and `Bool` instances and generic helpers (`map`, `foldr`,
  `sumList`, ...).
- **Deferral and memoization** (laziness and reclamation specification):
  affine thunks (`delay`/`force`, a thunk is a closure over `Unit`), `~ name
  = term` memo bindings in `let` and `where` blocks (one memo group per
  block, per-call lifetime in `where`, demand-on-first-use), `demand`, and
  the black-hole rule: `Eval r` effect labels per memo region, enforced by
  row lacks constraints, discharged at scope end, with `BlackHole` (exit 70)
  as defense in depth at the runtime boundary.
- **Hindley-Milner style inference with row typing**: unification with
  occurs check, let-generalization under the value restriction (locals may
  be polymorphic; top-level signatures stay explicit per the draft),
  effect rows as row-typed parts of function and thunk types (row
  metavariables, extension through tails, structural unification), and
  effect honesty: a body's performed effects must match its declared row.
- **Ownership, multiplicities, and regions** (core spec sections 3 and 7):
  `%0`/`%1`/`%aff`/`%many` usage checking with path-sensitive counts
  (disjoint projection places split, repeated places stack), the `Copy`
  class (`Share` alias; structural, non-extensible) with `copyOut`,
  `Ref r a`/`Mut r a` borrows with region escape checking and lexical `Mut`
  uniqueness, and thunk force discipline (`%1` exactly once, `%aff` at
  most once; unannotated thunks default `%aff`).
- **Codegen**: uniform word representation (immediates for machine words/
  `Bool`/`Unit`, boxed blocks for ADTs and the bignum/`Bytes` types,
  closure records for functions and dictionaries), generic curried
  application through an apply stub (so partial application,
  over-application and closures all work), pattern matching to `switch`,
  recursion, and a C `main` returning `main :: Int` as the process exit
  code (through the exit adapter, decision D12).

## What is deliberately not implemented yet

Rejected with an explicit error naming the form:

- Dependent types: computed indices, `Sigma`, dependent function types,
  `Type`/`Prop`/`Nat` universes, proofs, erasure, `spec`/`proof`/`stage`
  modifiers. `erased` record fields parse and are rejected until the
  erasure slice lands.
- Effects as capabilities: handlers, `do` notation, capabilities (`using`
  as a term, `with`, `@{...}`). Effect *rows* on arrows and multiplicity
  annotations (`%0`, `%1`, `%aff`, `%many`) are parsed and checked; what is
  missing is the handler calculus, `do` sequencing, and a richer effect
  vocabulary.
- Guarded codata (rejected at `codata` with a pointer to the codata slice),
  `rewrite`, `upgrade`, staging and macros, quotations.
- Strings (string literals and string types; `Bytes` buffers and `Char`
  values are implemented).
- `~` irrefutable patterns in function argument positions (Haskell's
  `~p`); memo binding bodies support only variable patterns. Symbol
  operators like `~>` remain available.
- First-class `foreign import` of real C symbols; module parameters and
  signatures as module arguments; local
  signatures with `forall`/`using`; recursive `let` bindings.
- Superclass bundles: a class constraint does not entail other classes;
  functions state each constraint they use (`using (add :: Add a, mul :: Mul a) => ...`).

Semantics notes for the slice:

- `Int` and `Nat` are mathematical (bignum-backed via the Rust
  `slruntime`); machine types are separate and explicit (primitive types
  specification). Division/remainder by zero faults (exit 72); `Nat`
  subtraction below zero and `Bytes` bounds faults exit 71.
- Top-level functions are monomorphic in their runtime code only in the
  sense that values are uniform words; polymorphism is real (type variables
  are erased, dictionaries are passed).
- Matching is source-order with fall-through (equations and `case` alike);
  equations after a catch-all are ignored. An uncovered constructor, `Bool`
  or tuple branch is a compile error naming what is missing. A match that
  runs out at runtime (integer literal patterns without a catch-all, or
  every applicable row failing its guard) takes the runtime fault path
  (exit 70): the fallback is never a word, because a word would crash on
  boxed result types. A row's guard is a `Bool` expression in the row's
  scope and its effects join the enclosing performed scope. A row's
  patterns bind each variable at most once.

## Layout

```
compiler/
  Cargo.toml
  prelude/Prelude.lang     the class stack and generic helpers, in-language
  src/
    lexer.rs               tokens and lexical rules
    layout.rs              indentation to { ; }
    parser.rs              EBNF subset + fixity pass
    ast.rs                 surface syntax
    check.rs               types, unification, elaboration
    check_terms.rs         term checking and pattern-match compilation
    check_decls.rs         declarations and program assembly
    core.rs                core IR and the runtime ABI
    codegen.rs             LLVM IR text
    driver.rs              CLI pipeline
  tests/integration.rs     compile and run programs, assert exit codes
```

## Roadmap

The next slices follow the implementation plan's milestones: the dependent
core (universes, indexed families, definitional equality), erasure with
retained witnesses, effect rows and capabilities over the same dictionary
mechanism, then modules-as-signatures, guarded codata, and the checked
optimizer rules. Each slice should land with its positive and negative cases
from `examples/`.
