# Cinch bootstrap compiler

A first compiler for the language designed in this repository. The surface
syntax is the draft 0.2 grammar in
[`specifications/surface-syntax-and-grammar.md`](../specifications/surface-syntax-and-grammar.md);
the static and dynamic goals are
[`specifications/core-language-specification.md`](../specifications/core-language-specification.md).
The language is named **Cinch** (renamed from its working name on
2026-10-27; the prose and identifier sweeps landed together). Every
identifier carries the name: the `cinch` CLI and crate, the reserved
`cinch.prim.*`/`cinch.runtime.*` namespaces, `.cinch` sources,
`.cinchif` compiled interfaces, `cinch_*` runtime externs, the
`cinchruntime` staticlib, and the `CINCH_*` environment variables.

This is a bootstrap slice: a small, honest subset implemented end to end
(source text to native executable) so the design can be exercised and
iterated on. It is not milestone 1 of the implementation plan in
[`design/open-decisions-and-implementation-plan.md`](../design/open-decisions-and-implementation-plan.md);
the dependent core is still ahead of us.

## Toolchain

The [devenv](https://devenv.sh) shell supplies Rust, LLVM, Tree-sitter and
Node. Cargo generates the parser during the build. Outside the shell, put
the `tree-sitter` CLI and its JavaScript runtime on PATH or set
`TREE_SITTER` to the path of the CLI.

```sh
devenv shell        # rustc, cargo, clang, llvm
cinch check examples/compiler/fib.cinch
cinch run examples/compiler/fib.cinch
cinch emit-ir examples/compiler/fib.cinch -o fib.ll
cinch build examples/compiler/fib.cinch -o fib
cinch-test       # cargo test
cinch doc extract compiler/prelude/Data/Option.cinch --legacy -o /tmp/option.cinchdoc
cinch doc render compiler/prelude/Data/Option.cinch --legacy
cinch repl
cinch pkg add alice/library@0.1.0 --registry http://127.0.0.1:8787
cinch pkg publish --registry http://127.0.0.1:8787 --project . --token "$CINCH_REGISTRY_TOKEN"
cinch pkg verify --project .
```

`cinch pkg add` fetches a registry release, writes it under the project's
`vendor/` tree, checks it into interfaces, emits `package.o` and its ABI
manifest, and updates `.cinchlock`. It installs exact and semver registry
dependencies recursively and accepts `--token` for private releases. `publish`
uploads `.cinchpkg` with its module files; `verify` rechecks lock identity,
source hashes, object hash, and interfaces locally. Backtracking resolution,
path/git dependencies, and provenance verification remain absent.

`CINCH_CLANG` selects the clang used for codegen and linking (devenv sets
it to the nix clang).

The optional embedded ORC backend is built with the `jit` feature:

```sh
cargo build --features jit --bin cinch
cinch jit examples/compiler/fib.cinch
```

`cinch jit` compiles the checked program to LLVM IR and executes its `main`
through LLVM's C LLJIT API. The backend loads the runtime shared library, so
JIT'd code resolves the same `cinch_*` boundary as a native build, and installs
the program's layout table before the entry runs. It requires LLVM with the C
ORC headers and library; `CINCH_LLVM_CONFIG` selects the `llvm-config` binary.
The feature is off by default, so the ordinary compiler build does not depend on
LLVM.

`cinch doc` is the documentation-comment front end. It parses `.cinchdoc`
canonical JSON, looks up symbols, indexes a source tree, runs `cinch test`
blocks, and renders Markdown. Cinch code fences in rendered output are
highlighted by the grammar in `tooling/tree-sitter-cinch`; `--plain` keeps
fenced code, and `cinch doc markdown` applies the same highlighting to an
arbitrary Markdown stream.

`cinch emit-object Library.cinch -o library.o` emits a library dependency
object, its LLVM text (`library.ll`) and physical ABI (`library.abi.json`).
The object includes the checked dependency closure. It has no process entry
and does not register a layout table. A consumer must supply `__apply` and
register one combined table containing the exact recorded layout prefix
before calling library code. Ordinary `build` does not yet consume these
objects. See [the object linkage contract](../design/runtime-object-linkage.md).
Use `--object-roots roots.json` to request concrete generic function, dictionary
and method variants needed by a separate consumer. The manifest records the
actual class method slot families used by the resulting object.

The Cinch stage compiler consumes a matching bundle through
`--emit-ir --interfaces MANIFEST --objects CONFIG SOURCE IR`. Admission checks
artifact SHA256 identities, exact configured ABI and checked interfaces, then
emission preserves the provider's layout prefix and actual method slot plan.
This flag belongs to the Cinch stage CLI; the Rust development CLI uses the
subcommands above. The stage driver builds and links its pinned library bundle
automatically, without routing compiler implementation sources through Rust.

## Pipeline

```
source -> lexer -> layout -> parser (+ fixity pass) -> checker/elaborator
       -> core IR -> LLVM IR text -> clang -> executable
```

The backend emits LLVM IR text and drives `clang`; keeping LLVM out of
process keeps the nix toolchain simple while the backend is still moving.
The closure and dictionary ABI is documented in `src/core.rs`.

## What is implemented

- **Array primitives (Native, ABI6 candidate)**: opaque `Array a`,
  row-major `Tensor a`, and composable heterogeneous `Columnar row`;
  checked reads, persistent updates, shape validation, packed scalar
  buffers, and counted object/bignum elements. The explicit source modules
  are `Data.Array`, `Data.Array.Packed`, `Data.Tensor` and `Data.Columnar`.
  See [array semantics and the self-hosting integration boundary](../design/arrays.md).
  The pinned ABI5 Cinch compiler does not yet implement these operations.
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
  operators. The prelude is ordinary source (`prelude/Prelude.cinch`) layered
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
- **Wasm and components** (webassembly and wasi specification): the
  wasm32 artifact profile with target-keyed widths and feature
  admission, the WIT subset parser with canonical-ABI component adapters
  (the codec case asserted exactly; `BoundaryError`/`DecodeError`
  adapter vocabulary), `WasiStream`/`WasiFuture` with degraded
  observations (never the productive codata `Stream`), a cooperative
  task backend preserving the memory model, and `eval : Code a -> a`
  over a build-time-reified runtime evaluator.
- **Runtime evolution** (runtime evolution specification): `upgrade`
  declarations with field-change classification and checked migrations,
  migration-free record views recording schema compatibility evidence in
  the interface, and the versioned dispatch runtime (code retention with
  live-frame counts, complete-update-set activation gating, no resuming
  old code over new state, target compatibility before reachability),
  with transparent JIT tiering and consumption-safe deopt riding the
  same machinery.
- **Concurrency** (concurrency specification): structured scopes with
  `spawn` and linear task handles (dropping cannot silently detach),
  n-ary `joinAllOrCancel` with first-failure-cancels and full
  acknowledgment awaiting, cooperative cancellation observed at defined
  points, task captures under the share-or-transfer discipline,
  `Blocking` foreigns on dedicated task threads with effect accounting,
  and the `Io r` runtime wired to real operations (`ioWrite`/`ioRead`/
  `ioSleep`/`pollCancel`) over the Rust runtime's threads.
- **FFI** (foreign function interface specification): real `foreign
  import`/`export ccall` with layout-checked binding contracts in the
  interface metadata and honest effect rows; `opaque` types, `RawPtr`
  and ownership roles with the deallocator rule and no integer-to-pointer
  roles; boundary and runtime field access over recorded `repr` layouts
  (packed fields marshal byte-exact); and the schema-introspection
  vocabulary (`SchemaSnapshot`, `introspectConsistently`,
  `CodecGenerator.derive`) over the subprocess capability.
- **Rewrites, optimizer, and target profiles** (modules spec sections 5,
  6, 13; platform spec): `rewrite`/`target rewrite` declarations with
  `justify` checking (kernel evidence or `signature` law methods,
  recorded in the assumptions when undischarged; effectful rules
  rejected; definitional equality never extended), the optimizer with
  preservation discipline (barrier rules, residual recheck, map fusion
  preserving values and effect order with the blocker named when fusion
  stops, weight reconciliation with the case-merge rescale), fuel/whistle
  driving bounds in target profiles with the specialization report, and
  target capability evidence (`Available`) with replaceable SIMD
  dispatch and the checked/trusted lowering split in the manifest.
- **Attribute annotations** (attribute annotations specification): the
  `#[...]` system across declarations, fields, clauses, alternatives,
  guards, `if` branches, and field types; branch weights `#[weight(n)]`
  with the `cold`/`likely`/`unlikely` wrappers (2000/1/0),
  all-or-nothing reporting, and LLVM `branch_weights` emission; inline
  hints including `#[noinline]`; and `repr(C)`/`transparent`/`align`/
  `packed`/`uN` with layout facts in the interface ABI table and the
  packed-borrow safety rule. Semantic attributes enter the contract
  projection; hints never do.
- **Staging and metaprogramming** (modules spec sections 7-11):
  multi-module source files, `stage` declarations with typed quotations
  `[||]`, splices `$()`, and `quoteDecls` over a compile-time
  interpreter, `generate do` with capability-refined stage I/O (file and
  subprocess), snapshot/replay build identity with redacted secrets,
  hygienic token-tree macros with expansion provenance, macro-owned
  attribute names, and stage scheduling with cycle diagnostics.
- **Guarded codata** (core spec section 9): `codata` declarations as
  delayed-observation records, `corec` with structural productivity
  checking (unguarded recursion rejected naming the call), `Later` as a
  logical modality distinct from `Thunk` under a usage-multiplicity rule
  (a `Later` binder is `%1`/`%aff` only), and productivity-loss typing
  for unguarded codata producers (`Diverge` on the last arrow).
- **Dependent core** (core spec sections 1-5, 8): stratified universes
  (`Type 0 : Type 1 : ...`, `Prop`; `Type : Type` never admitted),
  dependent function and pair types, kind-aware conversion checking
  (terminating by construction, no delta), implicit arguments with
  `forall` classification, indexed families (`Vec`/`Fin`/`Dec` builtins
  plus general user families) with index-equation matching and
  index-aware coverage, `spec`/`proof` with structural totality and
  erasure (no runtime slots for proofs or indices), `refl`/`congr` with
  proof irrelevance, and `decide` with the five obligation statuses.
  Positivity checking covers all inductive definitions.
- **Effects, capabilities, and sequencing** (runtime capabilities spec):
  effect rows with region-argument label unification, the handler
  calculus (`handle ... with`, one-shot `resume`, lexical operation
  dispatch, effect-typed handlers), `do` notation with direct/`Result`/
  `Option` sequencing providers (class-method `pure`/`bind`), runtime
  capabilities (`using (io :: Io r)`, `with (io = ...) in`, `@{...}`,
  `newIO`) with lexical-only solving and runtime identity `r` scoping.
- **Codegen**: uniform word representation (immediates for machine words/
  `Bool`/`Unit`, boxed blocks for ADTs and the bignum/`Bytes` types,
  closure records for functions and dictionaries), generic curried
  application through an apply stub (so partial application,
  over-application and closures all work), pattern matching to `switch`,
  recursion, and a C `main` returning `main :: Int` as the process exit
  code (through the exit adapter, decision D12).

## What is deliberately not implemented yet

Rejected with an explicit error naming the form:

- A constructor match against a family applied to rigid indices (decision
  D35: the error names the index equation; dispatch through `decide` or
  computed indices instead). This is a design boundary, not a gap.
- Handler-vtable specialization (the P13 follow-up; blocked on
  propagating concrete vtables across function boundaries).
- `~` irrefutable patterns in function argument positions (Haskell's
  `~p`); memo binding bodies support only variable patterns. Symbol
  operators like `~>` remain available.
- Local signatures with `forall`/`using`; recursive `let` bindings
  (the P19 guarded `letrec` lands the latter).
- Superclass bundles: a class constraint does not entail other classes;
  functions state each constraint they use (`using (add :: Add a, mul :: Mul a) => ...`).

Semantics notes for the slice:

- `Int` and `Nat` are mathematical (bignum-backed via the Rust
  `cinchruntime`); machine types are separate and explicit (primitive types
  specification). Division/remainder takes a `NonZero` divisor (a numeral
  is decided at compile time), so it never faults; `Nat`
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
  prelude/Prelude.cinch     the facade: the implicit surface and its re-exports
  prelude/cinch-base.cinchpkg  the base package's modules and check order (data)
  prelude/Data/Core.cinch   the inductive types the home modules re-export
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
    store.rs               the content-keyed artifact cache
    driver.rs              CLI pipeline
  tests/integration.rs     compile and run programs, assert exit codes
```

## Roadmap

Everything through P28 (refinement types) is landed, including the
P19 surface DX batch, the P20 safety assertions, P18's `prim`
declarations with inline target IR, P16's core requirements, and the
P21-P24 tooling (REPL, language server, tree-sitter/editor
integrations, interned symbols). Remaining before close: the P15
conformance sweep (SPEC-MAPPING upkeep, the approved D175 facade
trim, example 03's gate), the P28 lattice continuation (D256-D261:
compound predicates, predicate-level instances, narrowings,
`forget`, refinement obligations in `using` contexts), D46 `Name`
completion, and `Data.Enum` + range syntax.

Scheduled later: **P29, Blaze-style uber-fast recompilation**
(design/incremental-recompilation.md): REAPER-style on-demand
dependency graphs, an INCR-style persistent session with
function-level recompilation and type-checking confined to the
reachability closure of changed contracts, and LINK-style object
caching on declaration-level hashes. The compiler's content-addressed
artifacts, contract-only `iface_hash`, and `.cinchif` dependency chain
are the starting advantages; the acceptance is latency (unchanged
rebuild cost grows with graph traversal only) and a cache that is
advisory, never load-bearing.
