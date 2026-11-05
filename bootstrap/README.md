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
compiler source bundle and use only their preceding Cinch compiler as frontend.
That compiler emits typed shared-runtime LLVM and checked interfaces with
`--emit-ir-with-interfaces --interfaces MANIFEST --objects CONFIG`;
the platform linker links its IR, the checked library dependency object and
the matching runtime archive. The library-only producer is independently
pinned in `bootstrap/library-provider.lock.json`; it never compiles
`Bootstrap.*` implementations. Object, metadata, interface and runtime hashes
participate in later-stage cache identity. Missing concrete dependency variants
are reported in the stage's `requested-roots.json` for the producer's explicit
root protocol. The produced compiler must also
run and return its bad-arguments status before the build is accepted.
Cache hits verify source/builder identities and output hashes.
The fixed-point check requires the actual compiler modules' canonical
interfaces, so base-library interfaces cannot stand in for compiler output.
The final check also rebuilds a changed checker diagnostic and requires
that change in the resulting compiler's observable behavior.
Each command freezes its compiler inputs once, so edits made during a
stage build are picked up by the next command and cannot mix versions
inside a bootstrap chain.

The current ABI6 array/effect compiler builds all 43 compiler modules through
Stages 0, 1, 2 and 3. All stage smokes pass. Stages 2 and 3 match in LLVM,
executable bytes and all 43 canonical interfaces; interface-only entry
consumption and an actual changed-source rebuild also pass independently.
See [current ABI6 chain evidence](current-abi6-bootstrap-results.json).
This proves Acceptance 1-4 for compiler source `31ac09d621b8f8bc7b13f64f3eb6f61965f96507d7f8683b8512d4156ff40fa0`.
The complete matching Native and final-Stage3 suites, auxiliary checks,
original API coverage, proof soundness and Acceptance 5 remain open.
All 14 original external checks now pass on that exact ABI6 Stage3; see
[current external evidence](current-abi6-stage3-external-results.json).
The complete tooling suite passes all 41 tests, including both supported
stage ABIs and their mismatch refusals; see
[current tooling evidence](current-abi6-tooling-results.json).
The complete matching auxiliary audit covers all 171 runtime, 41 tooling
and 39 editor checks; see [auxiliary evidence](current-abi6-auxiliary-results.json).
Each component records its actual execution source and checked input identity.

Portable client tests use the separate explicit
[test library roots](test-library-object-roots.json), including concrete
variants outside the compiler source's own dependency closure. The pinned
Native producer builds only the generated `CinchBaseObject` library root
and its admitted base modules with these roots. Its checked interfaces
and runtime must match the compiler's library dependencies. See
[test library provenance](current-abi6-test-library-results.json).
Client parsing, checking and lowering remain with the exact Stage3 compiler.
The older [repaired chain](final-repaired-bootstrap-results.json) and
[external checks](final-stage3-external-results.json) retain their original
source scope; they do not establish the current compiler's full acceptance.
A compiler refusal is recorded with
its diagnostics and never falls back to the Rust frontend.
The runner applies a positive heap cap of at most 2G to every program.
Each stage chain runs one command at a time, with one build worker and one
test thread. Independent validation jobs share a limit of five heavy jobs:
four shared slots and one explicitly assigned probe lane. The heap cap
applies to each generated process separately.

The complete verification entry point is:

    python3 tooling/verify_bootstrap.py --evidence /tmp/cinch-verification-NAME

Choose a fresh evidence directory. This verifier copies the source files,
uses private build caches, checks the test executables' source paths, and
runs compiler, runtime, tooling and bootstrap checks sequentially at reduced
priority. Its final gate requires stage 2, stage 3, their fixed point and
the changed-source probe. That complete stage chain now passes on the repaired
snapshot. All41 tooling checks pass. Full compiler test and external acceptance
results remain separate requirements; a partial sweep cannot establish them.

The driver also has separate frontend and build modes:

    cinch --check [--interfaces MANIFEST] SOURCE DIAGNOSTICS_BASE
    cinch --emit-ir [--interfaces MANIFEST] SOURCE IR
    cinch --emit-interfaces [--interfaces MANIFEST] SOURCE PREFIX
    cinch --emit-ir-with-interfaces --interfaces MANIFEST --objects CONFIG SOURCE IR
    cinch --emit-ir --interfaces MANIFEST --objects CONFIG SOURCE IR
    cinch --compile-only [--interfaces MANIFEST] SOURCE IR EXECUTABLE REPORT

Check-only runs parsing, resolution and the current source/Core checks, then
writes `DIAGNOSTICS_BASE.diagnostics` (empty on success). It does not run the
emitter or linker. Compile-only emits and links without executing the output;
its report contains the linker exit alone. Omitting a mode preserves the
original compile/link/run behavior. Mode and interface options may appear in
either order; duplicate or conflicting mode options are refused. These modes
prepare the external Cinch test harness; they do not establish complete
independent correctness checking or a self-built frontend suite.

The pipeline also accepts `--interfaces MANIFEST` before its four paths.
The manifest is JSON: `{"schema":1,"interfaces":[["Module","hash","path"]]}`.
Paths are absolute or relative to the process's working directory. The
loader verifies the entire dependency set before source parsing, so
operator fixities come from checked public interfaces. Resolution retains
full contracts and defining identities. Schema8 carries definition-site
schemes, private nominal kinds, class contracts, Region and closed effect
contexts, exact value-index carriers, refinements and byte observations.
Pure polymorphic values and
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
ioRead's imported Option contract, foreign declarations outside checked
runtime intrinsics, and effect handlers still need elaboration. Source
effect declarations retain checked operation schemes and dispatch slots;
operation calls retain their declared effect rows.
The decision contracts keep `decide` as predicate syntax and retain
`Yes`/`No`'s erased evidence fields. Source elaboration and independent Core
checks validate their scoped predicates and separate source/runtime arities.

`Bootstrap.Type` and `Bootstrap.Infer` add rank-one checking in Cinch:
fresh instantiation, rigid quantified variables, kind checks, occurs and
scope-escape checks, let generalization, regular parameterized constructors,
positivity and pattern coverage. Pure polymorphic programs now reach the
emitter's explicit refusal for typed lowering. Lexical dictionary evidence
and kernel IO effects are retained. Coherent implicit instance selection
and named explicit dictionary values are supported. Source instances of
checked defining class interfaces retain specialized method schemes,
captured contexts and typed dictionary fields. Only implicit instances
participate in automatic search. A named dictionary may be supplied with
`with`; its member labels do not become new public functions. Source
class declarations, existential/indexed constructor elaboration and
decision/proof erasure remain required.
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

`Bootstrap.Global` independently rechecks Core references against retained
global declarations: schemes and type argument kinds, roles, method
owners/slots, constructor schemes, dictionary factories and lexical
providers. Universal openings and declared dictionary binders are checked
too, including within source dictionary method bodies. Its validated
context constructor is private. This reference checker is not the full
independent kind/term/proof kernel. Numeric reflection still needs actual
checked primitive or instance-body semantics; a canonical class name and
slot do not establish those semantics. Current evidence and remaining
gates are recorded in
[the global-reference report](global-reference-checkpoint-results.json)
and [the self-hosting design](../design/self-hosting.md).

`Bootstrap.Kinds` and `Bootstrap.Typing` separately check the nominal and
lexical kind contexts and the types/effects of retained Core terms. This
includes application and branch results, patterns and constructor fields,
primitive operands, strict lets and latent rows. They run after source
elaboration and global-reference checking. Their focused tests and the
full pipeline pass. Fresh stage0/1 pass six smokes; actual stage2 refuses
four headers before body checking and produces no compiler. That preceding
kind/term implementation passed all164+5 Native-assisted gates. It has been
superseded locally by the validated169+5 constructor snapshot. The later
source-decision implementation has separate validation. Full declaration/coverage,
semantic proof authority, totality and usage remain required; see
[the Core typing report](core-typing-checkpoint-results.json).

`UProofFun` retains a Prop domain, latent effect row and proper result type.
Source and runtime arity differ explicitly; ordinary runtime arguments cannot
stand in for erased evidence. The preceding type-only checkpoint passes all
nine focused targets; see [its report](proof-arrow-checkpoint-results.json).
Current source-proof validation and runtime limits are described below.

`CPProof`, `CProofRef` and `CErasedApply` retain scoped proof identities and
call effects. Source proof forwarding and lambda checks pass; wrong evidence,
runtime proof inspection and value-index confusion are refused. These checks
are included in the completed169+5 constructor suite. General proof computation,
totality and typed runtime erasure remain required; see [the source proof report](source-proof-checkpoint-results.json).

Kernel-owned `Yes`/`No` retain one erased source evidence field and no runtime
payload. Their source patterns, proposition scopes and exact metadata checks
pass, as do the separate fixed-semantics Core decision checks. That constructor
snapshot now passes the complete169+5 Native-assisted suite and its exact code
is installed locally. Its stage2 attempt stopped at two headers before bodies;
the newer source-decision attempt below has its own evidence. See
[the decision report](decision-checkpoint-results.json).

Source `decide` now retains checked predicates and executable Core decisions,
including short-circuit logical forms and stable strict-let value identity.
Its source/Core and dependent-function checks pass. Fresh stage0/1 pass six
smokes; actual stage2 stops before bodies at one imported measure-contract
header and produces no IR or compiler executable. Broader regressions and its
complete170+5 matrix are running. Unknown method bodies cannot define arithmetic aliases. Typed
runtime lowering and genuine self-hosting remain required. See
[the source decision report](source-decision-checkpoint-results.json).

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


## Selecting an external frontend for portable tests

The shared Rust test helpers accept an explicit Cinch compiler and checked base
manifest through `CINCH_TEST_COMPILER` and `CINCH_TEST_INTERFACES`. Both variables
must be set together. A selected compiler's failure is reported directly; it
never falls back to the Rust frontend. With neither variable set, the existing
Native path remains the default.

The external path bundles the project's owned source files, invokes `--check`
or `--compile-only`, and records inputs, arguments, outputs and status beneath
`.cinch-self-tests/`. Build does not execute the artifact; run executes the
successfully built artifact and retains its exit code. Only distinct base-family
interfaces are accepted; Native-produced compiler implementation interfaces are
refused. EmitIr is explicitly unsupported on this path for now.

`compiler/examples/bootstrap_external_probe.rs` exercises all three supported
modes through a compiled Cinch frontend, including multi-module input, a filename
containing a newline, a separately runnable binary and invalid source. This is an
adapter probe, not a complete self-built frontend suite. Tests that call Native
APIs directly still require separate coverage and classification.


## Retained checked programs

The pipeline now calls `check-and-retain`, which preserves the resolved source
and its typed elaboration together in `CheckedProgram`. Its constructor is
private. The typed emitter boundary receives this checked value; scalar emission
still uses its associated source while shared-runtime Core lowering is built.
The existing diagnostic-only `check-program` API remains available.

A runtime-input probe checks retention of polymorphic definitions and data,
rejects an actual type error, and verifies that outside code cannot call the
checked constructor. The complete scalar IR and pipeline targets pass. This
boundary is not a claim of complete usage/totality checking, typed runtime
lowering or successful self-hosting. See [the focused report](checked-program-results.json).


## Typed shared-runtime lowering

`Bootstrap.RuntimeIr.emit-runtime-program` consumes the retained checked Core
and returns either a complete LLVM module or named diagnostics. The first
implemented path covers closed scalar workers, exact mathematical Int/Nat
arithmetic, direct calls, branches, lexical lets and recursion. Values use
ABI5 runtime words, with duplication on local reads and cleanup at scope exits.
The native entry uses the runtime's reserved stack and exit leak check.

`programs/Run.RuntimeIr.cinch` reads source after it has been built. Eight
runtime-fed programs emit LLVM, link to the existing runtime, return their exact
expected results and report zero leaks, including bignums above2^128. Unsupported
polymorphism returns named diagnostics and clears stale IR. The complete emitter
source closure also passes through Cinch. The real compiler CLI route is being
validated separately; the default pipeline still uses its standalone scalar
renderer. Aggregates, closures, dictionaries, effects and the complete base ABI
remain required. See [the runtime IR checkpoint](runtime-ir-results.json).


The `--emit-ir` mode routes retained checked Core through the typed shared-runtime
backend, writes LLVM without linking or execution, and clears stale IR if the
emitter refuses a checked program. Mode and interface flags work in either order.
The generated module must be linked with the matching `libcinchruntime.a`.
`Bootstrap.RuntimeAbi` selects the exact interface/kernel ABI for a build:
live Native builds and the current pinned stage builder select ABI6. The
builder renders that choice from its verified seed lock before hashing and
exporting the complete source snapshot; the separate library producer and
runtime must match. Each build rejects other ABI versions and mixed interface
sets. Cinch now lowers the checked ABI6 array intrinsics, including element
class/layout and defining-interface authority checks. The previous ABI5 chain
receipts remain historical; current ABI6 Stage0/1 and positive array clients
pass, while Stage2/3 and full final acceptance remain in progress. See
[current array validation](current-array-kernel-effects-results.json),
[development stages](current-abi6-development-stage-results.json) and
[actual Stage1 array clients](current-abi6-development-array-results.json).
The real CLI test links and runs an exact bignum calculation, with zero leaks;
the existing whole pipeline still passes. The complete updated compiler source
also passes through Cinch in check-only mode. See [the emit-mode report](runtime-emit-results.json).


Typed runtime lowering also supports closed tuples and irrefutable tuple
patterns in parameters and single-alternative cases. One module-wide ABI5
registry stores the actual Raw/Obj/TObj field maps. Nested fields are borrowed
while their owning root stays live; returning a field retains it before the
root is dropped. Thirteen executable cases pass with zero leaks, including a
full16-field inline map. A17-field layout gets a named refusal until extended
descriptors are implemented. The whole backend source closure passes through
Cinch. Source ADTs and polymorphic/closure/dictionary/effect lowering remain
unfinished. See [the tuple report](runtime-tuple-results.json).


The protocol3 stage driver passes all27 tooling checks, six fresh stage0/1
smokes and a real numeric source-to-Cinch-IR-to-pinned-runtime executable probe.
The actual complete compiler still exits5 on the typed emitter's named
polymorphic/dictionary/aggregate refusals; no Stage2 compiler or usable IR is
produced. The empty IR file confirms stale output was cleared. That stage
snapshot precedes the tuple checkpoint. The numeric probe proves the linkage
path, not a self-built compiler. See [the exact driver results](typed-stage-driver-results.json).
