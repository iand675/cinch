# The bootstrap ladder

The artifacts behind the P35 milestones in `design/stdlib-and-bootstrap.md`:
a compiler written in Cinch, built rung by rung, each rung a runnable Cinch
artifact with an acceptance command rather than a claim.

## Compiler stages

The self-hosting continuation uses explicit stages, with separate outputs:

    tooling/bootstrap.sh build --stage 0  # fixed Rust seed
    tooling/bootstrap.sh build --stage 1  # Cinch compiler built by stage 0
    tooling/bootstrap.sh build --stage 2  # same sources compiled by stage 1
    tooling/bootstrap.sh smoke --stage 1  # valid programs and a named refusal
    tooling/bootstrap.sh check            # stage 3, fixed point and changed-source behavior
    tooling/bootstrap.sh status

Artifacts and logs live in `build/bootstrap/stageN/`. Stage 0 is pinned by
`bootstrap/stage0.lock.json`: a Git revision, a reviewed patch and hashes
of the patch and compiler sources. Editing the development Rust compiler
does not silently replace the seed. Stages 2 and 3 read the same frozen
compiler source bundle and invoke only their preceding Cinch compiler.
Cache hits verify source/builder identities and output hashes.
The final check also rebuilds a changed checker diagnostic and requires
that change in the resulting compiler's observable behavior.
Each command freezes its compiler inputs once, so edits made during a
stage build are picked up by the next command and cannot mix versions
inside a bootstrap chain.

The current stage-1 frontend is the ladder's fragment compiler. A stage-2
refusal is an actual bootstrap blocker, recorded with its diagnostics;
there is no fallback to the Rust frontend. `design/self-hosting.md` states
what remains before that build and the complete bootstrap check can pass.
The runner applies a positive heap cap of at most 2G to every program.
Builds and tests run one command at a time, with one build worker and one
test thread. The heap cap applies to each generated process separately.

The complete verification entry point is:

    python3 tooling/verify_bootstrap.py --evidence /tmp/cinch-verification-NAME

Choose a fresh evidence directory. This verifier copies the source files,
uses private build caches, checks the test executables' source paths, and
runs compiler, runtime, tooling and bootstrap checks sequentially at reduced
priority. Its final gate requires stage 2, stage 3, their fixed point and
the changed-source probe; the current stage-2 refusal therefore prevents a
complete pass. The verifier's tooling unit tests pass; its first complete
frozen run is still required.

The pipeline also accepts `--interfaces MANIFEST` before its four paths.
The manifest is JSON: `{"schema":1,"interfaces":[["Module","hash","path"]]}`.
Paths are absolute or relative to the process's working directory. The
loader verifies the entire dependency set before source parsing, so
operator fixities come from checked public interfaces. Resolution retains
full contracts and defining identities. Schema 4 carries definition-site
pure schemes and private nominal kinds. Pure polymorphic values and
regular imported constructors enter checking under those contracts;
display spellings do not establish a nominal identity. Private constructor
identities constrain coverage without exposing source names. The loader
checks their dependency closure and rejects conflicting contracts.
Imported calls still require the shared runtime
representation and layout registry and are refused before IR emission.
The stage runner supplies only checked base-library contracts from the
verified stage-1 artifacts, never prebuilt compiler modules.

`Bootstrap.Builtin` keeps the IO kernel contracts separate from Prelude's
public surface, including explicit region kinds, capability requirements,
effect rows and `newIO`'s fresh identity rule. `with` supplies and effect
rows are structured syntax trees. Resolution keeps capability bindings
local and qualifies effect operations and labels. Source checking now
accepts the kernel newIO/ioWrite/ioSleep/pollCancel contracts, lexical Io
providers, Region binders and closed kernel effect arrows. Function bodies
must fit their declared rows; a with binding does not remove effects.
Fresh regions cannot escape in inferred returned closures. Open row tails,
ioRead's imported Option contract, foreign declarations and effect handlers
still need elaboration.
The decision contracts keep `decide` as predicate syntax and retain
`Yes`/`No`'s erased evidence fields; they require dependent checking before
acceptance.

`Bootstrap.Type` and `Bootstrap.Infer` add rank-one checking in Cinch:
fresh instantiation, rigid quantified variables, kind checks, occurs and
scope-escape checks, let generalization, regular parameterized constructors,
positivity and pattern coverage. Pure polymorphic programs now reach the
emitter's explicit refusal for typed lowering. Lexical dictionary evidence
and kernel IO effects are retained; global instance selection, existential
and dependent obligations remain checker refusals.
`Bootstrap.Core` retains the checked terms, effect rows, capability scopes, constructor field types,
type instantiations and generalized let schemes. Failed elaboration exposes
no core. Source data retain their kinds, parameters and complete constructor
schemes, including private constructors. Fourteen runtime-input Core cases
include empty data, higher-kinded parameters and a 1,501-function body sweep;
the emitter's typed/shared-runtime path still needs implementation. The
source-effects runner reads changed programs after compilation and passes
15 positive/negative cases, including effect propagation, missing providers,
region mismatches, scope leaks and fresh-region closure escape.
The resolver keeps public aliases and source-module provenance in `DSource`
nodes so the later typed interface writer can retain the original scopes.

The orchestration checks run with:

    python3 -m unittest discover -s tooling/tests -v

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

`Bootstrap.Load` loads interface dependency sets from checked artifact
identities. `programs/Run.Load.cinch` accepts a JSON manifest path (default
`load-cases.json`) and checks real contracts plus damaged variants with one
compiled runner. `compiler/tests/bootstrap_load.rs` produces those inputs
from fresh seed-emitted interfaces. Loading retains the full contract;
scheme parsing and later type/coherence/ABI checking remain separate steps.
The manifest's `cases` rows are `[module, hash, path, expected-error]`;
an empty error means success. Its `sets` rows are `[inputs, expected-error]`,
where each input is `[module, hash, path]` and a successful set includes
every transitive dependency. Paths are read at runtime, so damaged inputs
do not require rebuilding the runner.

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
- Only an operator's declaring module may declare its fixity. Consumers
  inherit it through the imported binding; identical redeclarations are
  refused too. An operator without a fixity is usable in prefix form.
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
