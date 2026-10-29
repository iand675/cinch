# The bootstrap ladder

The artifacts behind the P35 milestones in `design/stdlib-and-bootstrap.md`:
a compiler written in Cinch, built rung by rung, each rung a runnable Cinch
artifact with an acceptance command rather than a claim.

## The rungs

| Rung | Artifact | Acceptance |
|---|---|---|
| 1 parse the language's own grammar | `Bootstrap.Parse.cinch` (lexing is `Text.Lex`, so the parser is a token-level grammar) | `examples/compiler/fib.cinch` and `classes.cinch` parse with zero unmodeled nodes; the wider corpus parses with per-module unmodeled counts it reports |
| 2 read and write a manifest canonically | `Bootstrap.Manifest.cinch` | a real `.cinchif` and `cinch-base.cinchpkg` round-trip byte-identically, and manifest questions (module count, facade index, order permutation) are answered in Cinch |
| 3 type-check a fragment | `Bootstrap.Check.cinch` | a hand-built AST of `fib.cinch` is accepted; arity, unbound-name, type and unmodeled-node failures are refused with named diagnostics |
| 4 emit LLVM IR text | `Bootstrap.Ir.cinch` | the IR for the fragment's `fib` module, assembled by `clang` and run, exits 0 |
| 5 drive clang | `Bootstrap.Driver.cinch` | a fixed IR module is written, linked by a spawned `clang` and executed, and its exit code is reported |
| 6 compile an example with itself | `Bootstrap.Pipeline.cinch` (integration, lead-owned) | `examples/compiler/fib.cinch` goes source to native executable through rungs 1-5 and the executable exits 0 |

Prerequisites, because the ladder cannot stand without them: `Text.Lex`
lexing what the compiler's own source contains (attributes, `prim` bodies,
tick names), real `HashMap`/`Set` (the current map is a documented bucket
bootstrap), and a Cinch test runner so the bootstrap can run its own checks.

## Layout and naming

Artifacts are flat files whose name is their dotted module name:
`bootstrap/Bootstrap.Parse.cinch` declares `module Bootstrap.Parse where`.
The compiler resolves a project's modules by each file's `module`
declaration, so a flat directory is a valid project. Modules use the
`Bootstrap.` namespace so they are never confused with the standard
library.

`Bootstrap.Ast.cinch` and `Bootstrap.Report.cinch` are the shared
contract every rung codes against, and they are LEAD-OWNED: a stream that
needs a node, a field or a helper asks for it rather than editing it, so
six streams cannot disagree about the tree. `Bootstrap.Parse` fills the
tree, `Bootstrap.Check` and `Bootstrap.Ir` consume it, and every rung
reports through `Bootstrap.Report`'s `Diagnostic`.

## How a rung is verified

Every rung has one Rust test, `compiler/tests/bootstrap_<rung>.rs`, built
on the shared harness `compiler/tests/common/mod.rs`:

- `common::project(tag, files)` writes the toolkit plus the rung's own
  sources into a fresh directory. The toolkit (`Text.Lex`, `Text.Scan`,
  `Text.Parse`, `Text.Json`) is NOT embedded in the compiler, so a project
  ships it; a rung that needs another non-embedded module (`Text.Args`,
  `Effect.Log`) ships it in its own `files` list.
- `common::run`, `common::check`, `common::build` call the real driver
  (`cinch::driver::driver`) on a large-stack thread and return the
  program's exit code, and `common::read_output` reads a file the program
  wrote into its project.
- A Cinch program has one exit code and no assertion macro, so a rung's
  program accumulates its answer in the exit code (0 = all checks passed,
  a distinct number per failed check) and the test asserts that number.
  Evidence that is a file (emitted IR, a canonical manifest) is written
  into the project and read back by the test.

`cargo test --test bootstrap_contract` is the contract's own test: it
proves the shared modules compile, that the tree can be built and walked,
that the unmodeled counter counts, and that a diagnostic renders as
`rung:line:col: message`.

## The Cinch discipline these artifacts follow

The compiler refuses a lot on purpose (`compiler/README.md`'s
"deliberately not implemented yet", `compiler/CONFORMANCE.md`), and each
refusal below has cost a stream a cycle when it was discovered late:

- Every definition's equation arity must equal its signature's arrow
  count. A partial application returning a function must be written
  eta-expanded, at full arity.
- No local signatures with `forall` or `using`: lift the binding to a
  top-level definition that can carry them.
- One flat module per file; no nested modules; no import cycles. Shared
  declarations go in a module ordered ahead of both users, and the users
  re-export them by name (`Data.Core` is the standard-library precedent).
- `case` cannot dispatch on a string, character or float literal pattern.
  Match constructors and compare with `string-compare`, `eq-char` or
  `char-code` chains, or intern to `Sym` and match by identity.
- No raw strings, and the escape set is fixed (`\n`, `\r`, `\t`, `\\`,
  `\"`, `\'`, `\u{HEX}`). Target IR text is built from escaped literals
  and `string-concat`.
- A keyword is never a binder: `as`, `implicit`, `target`, `data`, `type`,
  `to`, `stage`, `spec`, `macro`, `law`, `prim`, `using`, `forall`,
  `import`, `module`, `if`, `then`, `else`, `where`, `in`, `of`, `let`,
  `case` all fail at parse when used as one.
- A data type cannot mention one declared later, and it cannot nest itself
  inside another type constructor: a sequence of a recursive type is an
  in-type spine with an end marker, exactly as `Bootstrap.Ast` and
  `Data.Json` do. The `*-spine` builders and `*-of` walkers in
  `Bootstrap.Ast` are the convention to follow.
- A definition that would be an eager reference cycle is refused
  (unguarded recursion); a recursive descent parser recurses through
  top-level functions, which is fine, and local recursive values need
  `letrec` or a lambda.
- No em dashes or en dashes anywhere, in Cinch or in comments (a standing
  house rule).

## Ownership while a wave runs

A stream owns its own `bootstrap/Bootstrap.<Rung>.cinch` and its own
`compiler/tests/bootstrap_<rung>.rs`, and nothing else. It must not edit
`compiler/src/**`, `compiler/prelude/Prelude.cinch`,
`compiler/CONFORMANCE.md`, `design/**`, the READMEs, the shared contract
modules, or another stream's files: report what is needed to the lead and
keep working on what is reachable. Two streams do own prelude files by
assignment (`Text/Lex.cinch` for lexing coverage, `Data/HashMap.cinch` and
`Data/Set.cinch` for the real maps) and their changes must be additive.

## The honest boundary

`Bootstrap.Ast`'s `*Unmodeled` nodes are how the ladder states its
coverage instead of implying it: the parser records the span and the
source text of a form it recognizes but does not model, the checker
refuses such a node BY NAME, and `program-unmodeled-count` turns coverage
into a number a test can assert and a report can print. A rung that
"works" only because a form was silently dropped is not a working rung.

## Running a rung by hand

    tooling/run-bootstrap-ladder.sh                          # type-check every ladder module
    tooling/run-bootstrap-ladder.sh --all-programs           # build and run every rung program
    tooling/run-bootstrap-ladder.sh programs/Run.Parse.cinch # run one rung program

The compiler does not embed the P30/P31 toolkit, so the script assembles a
project (the toolkit sources, every `bootstrap/*.cinch` module, and the
program) in a temp directory and drives the real `cinch` CLI against it.
That is the same assembly `compiler/tests/common/mod.rs` performs, which is
why a rung that passes its test also runs here.

A rung's driver program is `bootstrap/programs/Run.<Rung>.cinch` (declaring
`module Main where`), and the rung's Rust test ships that same file as its
entry, so the program the test runs and the program a human runs are one
artifact rather than two copies of one program.
