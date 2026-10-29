# The bootstrap ladder: implementation decisions

`bootstrap/README.md` is the ladder's contract (the rungs, the corpus, the
discipline, the ownership rules). This note records the decisions behind it
and the reasoning that is too long for a README, so a later rung does not
have to rediscover them. The plan it implements is
`design/stdlib-and-bootstrap.md` (P30-P35).

## Where the artifacts live and what runs them

`bootstrap/Bootstrap.<Name>.cinch` declares `module Bootstrap.<Name> where`,
one flat file per module (the compiler resolves a project's modules by each
file's `module` declaration, so a flat directory is a valid project).
`Bootstrap.Ast` and `Bootstrap.Report` are the shared contract and are
lead-owned, deliberately: every rung builds or consumes the same tree, and a
wave of parallel streams that each edited it would disagree about the tree
instead of about their rung. A stream that needs a node, a field or a helper
asks for it.

A rung's DRIVER PROGRAM is an artifact too, `bootstrap/programs/Run.<Rung>.cinch`
declaring `module Main where`, and the rung's Rust test ships that same file
as its entry. The alternative (a program inline in the Rust test as a string
literal) makes the rung unrunnable by a human and by the CLI, which is
exactly what the design document forbids: "each milestone is a runnable Cinch
artifact, not a claim".

## A rung runs as an assembled project

The compiler does not embed the P30/P31 toolkit: `Text.Lex`, `Text.Scan`,
`Text.Parse` and `Text.Json` are on disk and absent from
`compiler/prelude/cinch-base.cinchpkg` (D309/D321 and the check-recursion
budget: an embedded family module is checked as part of every compile). Since
every bootstrap artifact imports that toolkit, a rung cannot run as a bare
file. Two paths exist and they are the same assembly:

- the Rust harness, `compiler/tests/common/mod.rs`: `project()` writes the
  toolkit plus the rung's sources and program into a temp directory;
- the CLI, `tooling/run-bootstrap-ladder.sh`: the same copy, then the real
  `cinch` binary against the assembled directory (and with no argument it
  type-checks every ladder module as a project).

Embedding the toolkit into the base package is the alternative and stays
open; it is a package decision (the store keys on the embedded source set)
rather than a ladder decision, so the ladder does not depend on it.

## Sequences in the tree are in-type spines

A Cinch data type cannot mention one declared later, and it cannot nest
itself inside another type constructor: `TApp :: Span -> TypeExpr -> List
TypeExpr -> TypeExpr` is refused by the positivity discipline
("positivity-preserving nesting is not supported in this slice"). So a
sequence whose element is the type being declared is written as an IN-TYPE
SPINE with an end marker (`TArg`/`TArgsEnd`, `PArg`/`PArgsEnd`,
`EArg`/`EArgsEnd`, `EBinding`/`EBindingsEnd`, `EAlt`/`EAltsEnd`,
`DDecl`/`DDeclsEnd`), which is the same answer `Data.Json`'s
`JArr`/`JArrEnd` gives. `Bootstrap.Ast` provides both directions:
`type-arg-spine`/`pat-spine`/`expr-spine`/`binding-spine`/`alt-spine`/`decl-spine`
to build one from a `List`, and `type-args-of`/`pats-of`/`exprs-of`/
`bindings-of`/`alts-of`/`decls-of` to read it back (a walker on a node that is
not a spine yields the empty list, so a malformed spine cannot fault a
consumer). A sequence of a type that is NOT the declared one needs no spine:
`ELam :: Span -> List Pat -> Expr -> Expr` and
`DFun :: Span -> String -> List (List Pat, Expr) -> Decl` are legal.

## Coverage is a number, not a claim

The parser fills the contract tree for a fragment and records every form it
recognizes but does not model as the matching `*Unmodeled` node carrying its
span and source text. The checker refuses such a node BY NAME, and
`program-unmodeled-count` turns coverage into a number a test asserts and a
report prints. The reason for the ceremony: the previous wave's most
expensive mistake was a rung that appeared to work because a form was
silently dropped, so incompleteness here is data.

Deliberately out of the first checker rung: classes and instances
(`DClass`/`DInstance`), which need dictionary passing, and are refused by
name rather than half-implemented. The parser covers them structurally
because the corpus is full of them; the checker does not claim them yet.

## Content hashing stays on the platform layer

The design document lists content-addressing-strength hashing as a
prerequisite for bootstrapping. It is met through the runtime:
`sha256-bytes` is declared in `Data.Bytes` (the type's home, for the P31
content-addressing use) and `Text.Digest` binds the runtime's digest
primitives as `foreign import ccall`, with their vectors pinned in
`compiler/runtime/src/text.rs`'s tests and `compiler/tests/runtime_prims.rs`.
Nothing re-derives a digest in language: the algorithms belong to the
platform layer, and a bootstrap compiler is entitled to call it. A
pure-Cinch SHA-256 would be a different decision (no runtime dependency) with
a different cost, and it is not this one.

## What each rung's evidence has to be

A rung is accepted when a real program, compiled and run through the driver,
produces the observable result the rung claims: parsed facts for rung 1, a
byte-identical round trip for rung 2, an accepted fragment plus named
refusals for rung 3, IR text that `clang` assembles into a binary that
returns what the fragment's `main` means for rung 4, a spawned `clang` whose
product runs for rung 5, and the whole chain on `examples/compiler/fib.cinch`
for rung 6. Source-text assertions are not evidence at any rung.
