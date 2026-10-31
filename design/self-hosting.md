# Self-hosting acceptance and remaining compiler work

The 2026-10-30 request resumes the bootstrap ladder and extends its target:
the Cinch compiler must compile itself. Rung 6's Fibonacci compilation is
useful evidence, but does not establish self-hosting.

`tooling/bootstrap.sh` implements the stage layout and commands. The seed
lock pins its Git source revision plus the conversion/interface/fixity patch and source
hash; the developing compiler cannot silently change stage 0. Stage 1
materializes the compiler modules and toolkit as a project. Stages 2 and 3
receive one identical frozen source bundle, with no seed frontend in their
execution path. Successful self-builds require a new executable, IR and
link report. The final check also requires matching canonical interfaces
and rebuilds a changed diagnostic label in the checker, then executes that
changed compiler and requires the new label in an actual refusal.

## Acceptance

1. The Rust seed compiles the Cinch compiler sources into stage 1.
2. Stage 1 reads and compiles those same sources into stage 2, rebuilding
   every compiler module. It may use clang, the runtime and checked base
   library artifacts, as the existing ABI design permits. It must not
   invoke the Rust compiler frontend or substitute prebuilt compiler code.
3. Stage 2 repeats the build into stage 3. Canonical interfaces and emitted
   IR agree between stages 2 and 3. Run the resulting executables as well.
4. Exercise a changed compiler source through the chain, so reusing an old
   artifact or emitting a fixed copy of the compiler cannot pass.
5. The complete compiler and runtime suites pass, including previously
   ignored tests. External compiler behavior tests must exercise the
   self-built frontend too; passing the Rust library's unit tests alone
   does not establish frontend parity.

Keep `CINCH_HEAP_LIMIT` positive and at most 2G throughout. Run gates on
frozen snapshots, with no time limit, as `next-steps.md` requires.
Run only one build or test command at a time, with one Cargo build worker
and one test thread. Rust code generation also uses one unit, including
root-launched stage builds. Separate snapshots do not make concurrent suites safe:
the heap cap is per generated program, not a machine-wide resource budget.
The previous overlapping gates were interrupted by a machine crash and
must not be reported as completed verification.

## Current boundary

The schema-5 continuation now passes its focused producer/consumer gates:
definition-site class contexts, explicit dictionary slots, private method
closure, owner consistency and portable pure instance heads. Conditional
instance method proof contracts retain positional arguments; conditional
and forward family instances remain explicitly unresolved. Native class,
kind, nominal and refinement metadata tests pass. The Cinch loader passes
14 contract cases and nine interface-set cases; the nominal/imported-class
runner passes 22 runtime cases. Inference and parser-corpus gates pass,
and typed Core passes 14 runtime-input cases including the 1,501-function
stress input. These are focused results, separate from the complete frozen
schema-4 baseline below; the complete developing-tree suite remains required.

Qualified schemes bind definition-local dictionary symbols. Calls and
method projections resolve against those lexical givens before Core is
sealed; slots are checked against defining signature rows. Qualified value
ABI arity includes dictionary parameters, and malformed arity is refused.
The regressions cover calls, shadowing, missing providers and scope leaks.
Global instance selection, source class/instance declarations and dependent
proofs remain unsupported. Source capability/effect checking is under active
verification as described below. Core closure checking is not an
independent typing or proof kernel.

The next type-engine continuation introduces distinct Region, Effect and
Row kinds, retained effect arrows and closed rows. Its compiled Cinch
runner passes four runtime-input checks covering order-independent set
unification, idempotence/coalescing, transactional backtracking, effect
inclusion, rigid-region escape, forall opening and generalization. Source
conversion and capability/effect body elaboration were not enabled by that
engine change. The subsequent Core, inference and parser regression gate
passed. Schema 6 now retains explicit Region binders, kernel Io constraints
and closed kernel effect rows in native resolved value contracts, and the
Cinch qualifier retains them. The native region/class contract tests and
loader's 14 contract and nine interface-set cases pass. A second compiled
type/Core runner passes row-union, effectful application and lexical capability
evidence checks; binding an IO provider does not mask its body's effects.
Source-level applications, lambdas, equations and lexical IO bindings now
use these forms. Their compiled Cinch checker passes 15 runtime-source cases:
IO propagation, returned closures, using/with providers, fresh providers,
implicit row regions, rejection of pure-body/callback effect erasure,
missing/mismatched providers, scope leaks, fresh-region closure escape and
wrong Region kinds. Builtin metadata is checked against kernel contracts.
All six regression targets now pass: Core, effect/types, inference, loader,
nominals/imported classes and parser corpus. ioRead's imported Option contract, foreign declarations and source
effect handlers remain unsupported.
Open row tails remain unsupported. The schema-6 seed has
been intentionally installed after verifying its applied patch against
the pinned revision and complete native source hash
`f43379c30a694ba6dd65390ddb2157fb824806dbff702007173e1cf476545c03`.
The compiler-source bundle is
`2c226bf447807e6a0b0e91818bfbe824b07ec716f81966b9262ed462450406f3`.
Stages 0 and 1 rebuilt from that seed and pass all six Fibonacci,
precedence and named-refusal executable smoke checks. Stage 2 uses the same
compiler-source hash and exits 4 before body checking, with 136 header
diagnostics and no IR or executable. The earlier IO/region/effect-arrow
refusals are gone; dependent binders, proof/refinement contexts, source
effect/foreign/instance declarations and two unresolved methods remain.
The count includes cascading diagnostics, not independent missing features.
Evidence is in `/tmp/cinch-schema6-io-results.json`; the complete
compiler/runtime run began on the intended 504-input snapshot
`5ede0e05a98647f36513e42799abad40de095520184f88ca5cf4c8950495c58b`.
Its parser test exposed a provenance defect: sharing Cargo's target cache
reused an executable with the live checkout's CARGO_MANIFEST_DIR embedded.
Its runtime corpus included the new Bootstrap.Proof module (zero unmodeled
nodes), outside the snapshot. This run cannot establish a frozen full pass. It was stopped after the
provenance defect was confirmed, and all of its child processes were
confirmed stopped before focused repair checks started. Partial results and
the cancellation reason are recorded in `/tmp/cinch-schema6-full/results.json`.
Snapshot content auditing alone does not establish the runtime inputs of a
cached test executable.

`tooling/verify_bootstrap.py` is the replacement verifier. All 16 tooling
unit tests passed initially. The first real capture found the repository's
internal parser-corpus directory link; the verifier now copies internal
directory links into ordinary frozen directories and rejects cycles and
links outside the source tree. All 19 tooling unit tests now pass. Its first
complete frozen run is still required. It copies source contents, uses private compiler/runtime Cargo
caches and private stage outputs, audits source contents between gates, and
runs compiler, runtime, tooling and stage smoke gates sequentially, followed
by the full stage-2/stage-3 fixed-point and changed-source check. A smoke-only
pass cannot make the verifier succeed. The common
bootstrap test harness now refuses an embedded manifest path that differs
from the active package root. These changes must be verified before a new
complete frozen result is claimed; historical full-run counts remain
historical execution evidence rather than proof of the new snapshot setup.

The current native regression run also found interface emission ordering
failures in higher-kinded, algebra and category library tests: embedded
consumers were emitted before source copies of their defining class homes.
The repair computes dependency order from explicit imports and
required nominal contracts, replacing each embedded home with its source
copy in that order. Family homes do not gain a Prelude facade back-edge.
Exact defining-interface hashes remain mandatory. The three source-path
checks, class-contract test and two-fixture source-override test pass. All
24 higher-kinded, 22 algebra and 12 category tests pass, including every
formerly failing case in those targets. The complete current-tree gate
remains required.

The first runtime-proof gate caught an existing native pattern checker bug:
nondependent JSON fields incorrectly had to use identical variable names
across pattern rows. The repair restricts that requirement to fields whose
names later field types actually mention. Three new regressions cover JSON
row-local names, independent evidence names and rejection of inconsistent
dependent binders. Those tests and the existing dependent-type target are
now pass alongside all 14 existing dependent-type tests and the proof gate.
Evidence is in `/tmp/cinch-proof-pattern-repair.log`.

The following dependent-proof continuation postdates the legacy snapshot.
Its compiled Cinch certificate checker and bounded search now pass their
runtime-input gate. `Bootstrap.Proof` reifies mathematical integer
arithmetic and Boolean propositions. Bounded Fourier-Motzkin and case-split
search produces only citations and positive multipliers. A separate checker
reconstructs every inequality from the actual hypotheses, goal and active
split alternative, then checks exact cancellation and a negative bound.
Certificates carry no cached goal or search-produced inequality. Runtime-input
tests pass for three-term bounds and malicious/stale certificates.
The runner also decodes complete facts, goals and certificates from JSON
files supplied after compilation, covering changed identities, negative or
zero multipliers, invalid citations and sibling-branch evidence. All four
runtime-input batches (29 checks each) and 16 complete JSON packets pass. This is the compiled-check tier, not
a reflected soundness theorem. The elaborator must still check index sorts,
pure total calls and lexical scope, retain dependent argument identities,
and connect checked certificates to Core obligations. No previously refused
source proof is accepted by this standalone module change.

A subsequent index-syntax continuation is drafted in `Bootstrap.Index`,
with distinct bound, rigid, meta and defining-global identities, structural
alpha equivalence, capture-avoiding simultaneous substitutions and explicit
scope/escape checks. Its runtime runner has 32 checks at six input nonces
and 23 complete JSON term/context packets supplied after compilation.
The compiled runner passes all 32 checks at six input nonces and all 23
complete JSON packets, including arbitrary-precision binding identities.
The current-module ladder check and parser corpus also pass with the new
module included; evidence is in `/tmp/cinch-index-runtime-3.log` and
`/tmp/cinch-index-integration-regressions.log`. Source proof acceptance is
unchanged: connecting typed index conversion/unification to source types
and actual proof contexts remains required.

The developing type engine now retains generic value-parameter carriers
in `KValue`. To keep the bootstrap language's ordered declaration rules,
carriers use complete tagged structural type trees, with checked
`value-kind`/`value-carrier` construction and decoding; display spellings
are not type identities. Normalization, unification, substitution, inference
levels and generalization visit those carriers. Quantifier kind holes remain
visible to scheme closure. Core refuses to freeze an unsolved value index
into a fabricated private runtime witness. All 33 carrier checks pass at
five runtime inputs, including malformed-tree and sequence-node rejection.
The latest checks include pure/empty-effect arrow conversion and rejection
of effectful carriers. The 523-input private snapshot
`f607381513aeccfcd7b55e39bd337f8de0b40e11379c84077abbf596f6b48074`
passed Core and both effect targets, then failed inference compilation:
generated LLVM reached 2,166,056,592 bytes and exceeded Clang's source-location
limit. Results are in `/tmp/cinch-carrier-kind-regressions/result.json`.
No source dependent kind,
binder, index expression or proof acceptance has been enabled by this type
engine work; value witnesses and typed Core obligations remain required.

The next internal type-engine slice adds `UIndex carrier expression` and
connects the structural index syntax to normalization, unification, carrier
serialization, substitution, inference levels and generalization. Stored
metavariable carriers are checked before simplification, so an Int variable
cannot masquerade as a Nat index and zero multiplication cannot hide an
unknown variable. Simultaneous bound/meta substitutions preserve capture
avoidance and reject non-value or incompatible carrier replacements. Core
refuses unsolved index witnesses and opaque computations without checked
reification. Nat subtraction currently needs a known bound or reflexivity;
symbolic bounds still require the source proof-context integration.
The first isolated slice passes 30 checks at five input nonces alongside
all 33 existing carrier checks. A subsequent frozen run passes 33 checks
at five nonces and 21 complete type/carrier/index JSON packets supplied
after compilation, plus the existing carrier checks. Evidence is in
`/tmp/cinch-value-index-runtime-1/` and `/tmp/cinch-value-index-runtime-2/`;
both final source audits pass. A later guard prevents zero multiplication
and reflexive subtraction from erasing opaque evaluation before purity or
totality has been checked. Its expanded 34-check/24-packet runner and the
broader bootstrap regressions are running in
`/tmp/cinch-typed-index-regressions/` and still need final verification.
The expanded runner, carrier checks and all broader targets pass: 13 Rust
tests in total, with a final unchanged-source audit. Core, effect types,
source effects, inference, ladder, nominals, parser, source paths and type
syntax are covered by this focused snapshot. A subsequent conformance review against D28 and the native
`ITerm::normalize` tightens conversion to arithmetic on numerals only.
Symbolic identities such as `n + 0`, `0 * n` and `n - n` need propositional
evidence; they must not silently extend conversion. This also preserves
unresolved metas and opaque evaluation. The resulting 37-check/28-packet
runner and strict JSON-input handling pass at five nonce inputs alongside
all 33 carrier checks in a subsequent private snapshot
`d5b973fdabd5202adea8e3decd23e69b872e5b7d64c01a1642884d4dda22cfa1`.
Its final source audit is unchanged; evidence is in
`/tmp/cinch-index-conversion-conformance/`. Index checks finish in 35.84
seconds and carrier checks in 23.52 seconds. The complete current-tree
compiler/runtime and self-hosting gates remain required.
These are internal conversion checks. Source dependent kinds, Pi binders,
refinements, actual bound/global typing and proof-context construction are
still required; no new source proof acceptance is enabled by this slice.

The LLVM expansion exposed a native optimizer bug: exceeding the profile's
measure stopped further rounds but still installed the oversized residual.
Later callers unfolded those already expanded bodies. The driver now keeps
the preceding bounded residual, discards helpers and report entries created
by the rejected round, and refuses to copy a callee larger than the caller's
entire allowance. A regression over 12 transitive functions, both declaration
orders and three measure bounds fails before the repair and passes after it.
The private snapshot
`53419b9d5e80031fbece5b3683dbc4300cff79c5cc320f20e20ad358d7f6d395`
passes all 24 selected tests: inference, nominal contracts, effect types,
indices, carrier kinds, source paths and the 16 optimizer tests. Inference
now completes in 103.38 seconds rather than failing after 437.17 seconds.
The final report rollback and complete index-codec round trips pass in
snapshot `667f98638ac6e477e7f255e4484f957b6d7c165c6957615f693b6a282c09d2fa`:
the growth regression, all 16 optimizer tests and index tests pass. The latter
exercise 33 checks at six runtime nonces and 23 complete JSON packets.
Both snapshots used private caches and passed their final source audits;
evidence is in `/tmp/cinch-optimizer-bootstrap-regressions/` and
`/tmp/cinch-optimizer-final-regressions/`. These are focused gates, not a new
complete compiler/runtime or self-hosting acceptance run.

The schema-6 seed was intentionally refreshed after these focused gates.
Its source hash is
`475b8887d71c8becfd9b572897717ac58fac3d3df5db8294c2ed3ef2bb81fcec`
and patch hash is
`f7c25689f80fcc6274bb70fec0da942ca1cb2001283144dd3b45ea9bb8a115fc`.
All patched source bytes were verified against the pinned Git archive;
the prior seed is backed up in `/tmp/cinch-schema6-5705-before-optimizer`.
Fresh stages 0/1 have now rebuilt from this pin, and all six executable
smoke checks pass. The frozen 523-input staged run
`6590ee7878fb34c3ca1ed4b31ce8f2e33a997f29e97f2877a1a4c62c961f043e`
also passes all 19 tooling tests and its final source audit. Its compiler
source bundle is
`f993990a3dcf5b78ed6eec1a12086335658048136ad47b379f4ffb6a7032b50a`.
Stage 2 again exits 4 with 136 header diagnostics, before body checking,
IR or an executable. The current evidence is
`/tmp/cinch-optimizer-stages/boundary.json` and `stage-summary.json` beside
it. This run predates the developing typed-index continuation.

The source resolver's dependent-scope continuation passes its isolated
runtime gate:
leading `using` conditions now preserve the names of later arguments in the
same function, while callback-local argument names do not hide outer module
globals. Three passing regression cases cover direct, later-curried and
nested callback binders. The resolver runner's older IO assertion has been updated
to check a valid lexical provider rather than expect IO elaboration to be
unsupported. These source/test edits are outside the frozen schema-6 gate.

Before the schema-6 rebuild described above, the schema-5 stage rebuild
produced the following historical results. Its patch was
applied to the pinned Git revision and all patched source bytes verified
against the new lock hash. Stages 0/1 have rebuilt, and both pass
Fibonacci, precedence and named-refusal executable smoke checks. Stage 2
uses the identical compiler-source hash
`2f1f58c6a79a07b4016449330e56fa4bc2df6e44e19ae3ad6eae1c5ac57ad10a`
and still exits 4 before body checking or IR emission. It reports 232
header diagnostics, compared with 110 before this continuation: the newer
checker reports cascading errors within contexts rather than refusing each
whole context once. Remaining unsupported areas include region/effect and
capability elaboration, dependent binders and predicates, refinements,
foreign/effect/instance declarations and two unresolved method contracts.
Three duplicate-binder diagnostics also require examining their source
predicates. No stage-2 executable exists. Evidence is recorded in
`/tmp/cinch-schema5-lexical-stage-result.json` and the stage-2 diagnostics;
the raw diagnostic count is not a count of independent missing features.

`Bootstrap.Pipeline` still compiles a checked integer fragment. Its emitter
uses `i64` values, direct calls and local slots. It does not yet implement
the uniform-word constructor, closure, dictionary and reclamation ABI
needed to compile the compiler. `Bootstrap.Check` now selects the
`Bootstrap.Infer` rank-one elaborator for quantified signatures,
parameterized data and inferred lambdas. Fresh instantiation, rigid
definition variables, occurs checks, scope levels, let generalization,
higher-kinded type parameters, regular constructor substitution, strict
positivity and constructor-specializing pattern coverage are implemented
in Cinch. Constructor-local existentials and indexed families still need
dependent elaboration. Source class/instance declarations, open effect rows
and refinements remain explicit refusals; parsing them does not discharge
their obligations. Lexical class constraints and closed kernel IO effects
are supported as described above.
Source module export aliases, definition symbols and source ranges now
survive aggregation as `DSource` metadata. Successful elaboration exposes
checked schemes and those source contexts. It now also builds pure typed
terms in `Bootstrap.Core`: applications retain type instantiations,
constructor patterns retain their field types, and let bindings retain
their generalized schemes. Quantified identities stay distinct through
nested lets. Each function is sealed before checking the next, with
unconstrained internal monotypes represented by explicit private rigid
declarations. Typed source data declarations now retain their kinds,
parameters and complete constructor schemes, including private constructors;
field types come from the checked schemes. Imported declarations remain
interface contracts. Failed elaboration exposes no bindings, data or core. This is a
typed-term producer and closure check, not an independent typing, usage or
proof kernel. Dependent data/evidence, canonical source interfaces and shared runtime
lowering still need their full implementation. The existing emitter does
not consume this core yet.
`Bootstrap.Resolve` now checks source-module dependencies, selections,
renames, aliases and export surfaces before assigning distinct global
symbols. Private declarations neither collide nor leak across modules.
The pipeline passes the resolved aggregate to the checker and emitter.
Imports without a source module now resolve through a closed checked-interface
set supplied with `--interfaces`. Public names and inherited fixities come
from those contracts; private names are absent from the importing scope.
The checker and emitter also refuse raw imports or multiple modules if
called without resolution.

`Bootstrap.Builtin` carries the IO kernel's value contracts independently
of Prelude exports: `newIO`, its `Runtime.newIO` alias, `ioWrite`, `ioRead`,
`ioSleep` and `pollCancel`. Region kinds, capability contexts, effect rows,
runtime ABI and fresh identity creation remain attached to their canonical
bindings. The checker now accepts the verified newIO/ioWrite/ioSleep/pollCancel
contracts with lexical providers and retained closed rows. ioRead still
requires a defining Option contract. This is the IO slice; concurrency,
staging and other kernel APIs remain outside this builtin environment.
The decision vocabulary also has explicit contracts: `decide` consumes
predicate syntax and is not first-class; `Yes` and `No` quantify a Prop
parameter and have erased positive/negative evidence fields. The checker
refuses these until dependent predicate and erasure checking exists.
Capability supplies have structured `EWith` trees and lexical scope;
handler operations and continuations resolve in their respective scopes.
Effect declarations and foreign contracts retain their defining names and
external symbols while their checking and lowering remain explicit refusals.
Instance heads and bodies also resolve: method labels stay attached to the
class contract, instance parameters scope their bodies, and an instance
identifier is not introduced as an ordinary module value. Coherence and
dictionary elaboration are still required before acceptance.
Effect arrows now contain structured rows with label applications and an
optional row tail, so resolution can qualify labels without losing regions.

`Bootstrap.Load` now loads the seed's emitted interfaces as a closed
dependency set, checking schema, runtime ABI, contract identities, missing
or stale dependencies, duplicate module names and cycles. The loader
retains the complete JSON contract, including assumptions, instances and
layout facts. Selected value schemes parse through `parse-scheme` directly
into type trees, with full consumption and retained quantifiers and proof
preconditions. The pipeline now loads these contracts before parsing,
retains them in the resolved AST, and checks imported monomorphic
integer/Boolean value schemes. Schema 4 adds pure quantified and nominal
value and constructor contracts captured at definition-site lookup. Private
type kinds and complete constructor identities survive without publishing
their source names. The loader checks the defining modules' dependency
closure and rejects inconsistent nominal contracts. The checker preserves
their identities through instantiation and pattern coverage. Overloaded
selectors retain their candidate identities and are refused until dictionary
elaboration exists. Effectful, constrained, dependent, record and existential
imports still need their full checker support.
Imported code is explicitly refused by the emitter until it can use the
shared runtime representation and layout registry. Envelope validation
does not establish kind, coherence or ABI compatibility.

The native backend currently assigns constructor and pointer-map layout
IDs for one complete program (`compiler/src/codegen.rs`, `Layouts::finish`)
and the runtime installs one program table
(`compiler/runtime/src/block.rs`, `register_layouts`). Library and compiler
code must use one shared registry. The current size/alignment interface
facts do not establish compatible IDs between independently emitted
objects. The bootstrap backend must preserve that whole-program invariant
when incorporating base implementations.

The current continuation fixes the constant evaluator's integer conversions
and extends the Cinch frontend's syntax contract:

- Structured index expressions, predicate contexts and refined results.
- Named refinement declarations and quantified parameter kinds retained
  in the AST, with explicit checker refusal until proof checking exists.
- Complete `using` lists, including the final constraint.
- Tuple binding patterns, guarded alternatives and handler clauses.
- Declaration parameter names, kinds and implicitness; foreign direction,
  ABI, external symbol and source name; import modifiers, selections,
  renamings, aliases and arguments.
- Implicit Prelude fixities, comma-separated fixity declarations and
  precedence climbing that resumes at the caller's binding power.
- Operator-owner fixities: importers cannot redeclare them, even
  identically. Selections, renames and re-exports inherit the defining
  fact; an operator without a fixity remains usable in prefix form.

Interface schema 4 retains signature preconditions, named refinements,
declaration parameter kinds and implicitness, constructor schemes with
result indices and linker symbols, defining type/class identities, and
explicit public class/method names. Higher-kinded function quantifiers
retain their resolved kind. This does not yet make interface-only checking
complete: rich contextual identities, coherence, evidence and ABI consumption
still need their full implementation. Pure resolved schemes use canonical
definition identities; unbound type names also route through kind checking
instead of the legacy monomorphic checker's permissive nominal spelling.

The parser corpus now includes every `bootstrap/*.cinch` source and its
toolkit. `bootstrap_type_syntax` inspects actual syntax trees: an unmodeled
count of zero cannot detect a discarded field. `Run.Eval.cinch` checks
tuple destructuring, guard fall-through and operator precedence against
observable values.

After the crash, serial focused checks pass for the source resolver,
operator interface publication, evaluator, IR emitter and end-to-end
pipeline. The pipeline now also compiles a multi-module program with a
renamed operator re-export, a global integer constant and a Boolean value,
then runs the produced native binary. It refuses importer fixity
redeclarations during parsing and function values that still require
closure lowering before emitting IR. This evidence is for the current
fragment, not a self-build or a complete-suite pass.

The refreshed pinned seed and stage 1 both built and passed their Fibonacci,
precedence and named-refusal executable smoke checks. The schema-3 refresh
repeated these checks. The subsequent
connected-loader sources pass native checking and rebuild stage 1 with
the loader, contract adapter and manifest reader reachable from its entry.
The connected pipeline integration test passes: real checked imports,
renamed re-exports, imported type mismatches, private-name rejection,
operator ownership, unused contracts and invalid manifest schemas are
exercised through the compiled Cinch pipeline. Imported calls reach the
explicit shared-ABI refusal; no incompatible library calls are emitted.
The full parser corpus and source-scope regression runner pass after this
integration, and the rebuilt stage 1 passes Fibonacci, precedence and
named-refusal smoke checks. The orchestration suite has 11 passing tests.
The schema-4 seed and stage 1 have rebuilt. Stage 1 includes the rank-one
checker, resolved nominal contracts and retained source-module contexts,
and passes Fibonacci, precedence and named-refusal executable smoke checks.
The earlier self-build exhausted the stack during reference collection:
the declaration walker reserved about 200 KiB per recursive list frame.
Separating the single-declaration match from the traversal fixes that
failure. The resolver regression exercises more than 6,000 declarations
and reference order through nested instances and foreign exports.

The fresh stage-2 attempt passes parsing, checked-interface loading and
source namespace resolution, then exits 4 with 110 checker diagnostics,
down from the prior 189. Regular imported constructor refusals are gone.
There are 56 dictionary/proof-context diagnostics and 12 method-contract
refusals. The five residual nominal refusals name `NonZero` or `InRange`;
these need their refinement/proof semantics. Dependent arguments, effects,
regions/capabilities, foreign and instance declarations remain incomplete.
Header failures prevent body checking, so 110 is not a count of all
remaining source or elaboration problems. There is no stage-2 compiler or
emitted IR. Evidence is in `build/bootstrap/stage2/compiler.ll.diagnostics`
and its `stage.json`; its source identity matches the rebuilt stage 1.

The seed patch was deliberately refreshed for schema 4, with its application
and resulting source hash verified. All six affected serial regressions
pass: checker, inference, loader, nominal imports, parser corpus and pipeline.
The nominal runner checks aliases, private kinds, hidden constructors,
same-spelling types, literal string identity and rejection of unresolved
display schemes. The orchestration suite has 11 passing tests. The pipeline
retains explicit IO, decision, effect, foreign and instance refusals. The
refreshed module-scope, resolver and syntax-tree gates pass too, including
the long reference walk. Verification stays one command at a time.
The interface reader regression also passes all twelve altered-artifact
cases with a single compiled reader. Its artifact, source and digest inputs
now come from files, so testing each refusal no longer recompiles the same
program. A process killed by a signal fails the test instead of counting
as the malformed-JSON refusal. This gate completes in about 35 seconds.
The runner takes base contracts only from the verified stage-1 manifest
and the pinned base-package allowlist. Compiler interfaces cannot substitute
for rebuilding compiler sources. Manifest and interface bytes participate
in stage-cache verification.

The first completed serial full baseline on 2026-10-31 records 1102 native
compiler tests passed, two failed, and zero ignored; the runtime suite
records 165 passed and no failures or ignored tests. All three stage-0
smoke checks pass. The original bytes-literal fold and the signed/unsigned
conversion regressions pass in that run. The failures are the ladder's
duplicate `interface-global` export and a Prelude probe that uses the
unordered class-contract table to choose a sequencing method scheme.
Their repairs pass the focused ladder/Prelude gates. An allocation
probe's removed heap guard is also repaired: generated programs retain an
explicit cap, while a runtime unit test checks default/fallback selection.
That private runtime refactor preserves behavior but changes the pinned
source inputs. The eight allocation checks and three heap unit tests pass.
The seed patch and resulting source hash have been refreshed and verified
against the pinned revision. Stages 0 and 1 have rebuilt from that lock and
the typed-data sources. The fresh stage-2 attempt again exits 4 with the
same 110 header diagnostics; its source hash matches stage 1
(`57dc0ee99c0a0cface79342f34241acbd7c42290e3bc3e3eac2332549d4fc914`).
There is still no stage-2 IR or executable.
The baseline precedes these repairs, and it does not establish self-built
frontend parity. Evidence is in `/tmp/cinch-schema4-full/results.json` and
the compiler/runtime/smoke logs beside it.

The first core regression passes twelve runtime-input cases, including
distinct polymorphic instantiations, typed constructor fields, nested
generalization without capture, private type declarations, guarded
equations, Boolean short circuit branches and rejection with no core.
Its large input checks 1,501 function bodies and their solved annotations.
The parser now retains a guarded function equation as `EGuarded`, and
normalizes infix Boolean syntax to conditionals before name resolution;
prefix operators remain ordinary function values. Broader checker,
inference and parser-corpus regressions pass. All nine subsequent integration
targets pass too: evaluator, IR, ladder, loader, module scopes, nominal
imports, pipeline, resolver and retained syntax. A further focused core
run passes thirteen runtime-input cases, adding empty source data, private
constructors and higher-kinded declaration parameters. The nominal
runner now shares each case's diagnostic result between its report and
assertion; all twelve snapshot cases remain. The full frozen rerun completed
on 2026-10-31 at 13:57 UTC: 1105 native compiler tests and 166 runtime tests
pass, with zero failures or ignored tests. Both stage-0 and stage-1 pass
Fibonacci, precedence and named-refusal executable smokes, six checks in
all. The snapshot SHA256 is
`fc21e0d1be1c5a53a1cd36d4b743e83c1ee38e3e6ba190f5acd4f1a319ad8dd3`;
its source audit confirms all 497 captured inputs are unchanged. The full
logs, results and source audit are in `/tmp/cinch-typed-core-full/`.
These results belong to that schema-4 snapshot. The later schema-5 and
lexical dictionary continuation has a separate focused gate; its first
Rust build found a class lookup argument type mismatch, now repaired for
that rerun. None of this establishes a stage-2 self-build.

## Next implementation sequence

1. Preserve the connected loader, kernel contracts and module scope tests.
   Extend the kernel environment as further compiler features require it;
   keep capabilities, proposition evidence and erasure rules explicit.
2. Extend the resolved pure nominal contracts to rich imported schemes, full
   kind inference, dictionaries, effect rows and the compiler sources'
   refinement obligations. Imported schemes must use their defining type
   identities; do not guess a private or aliased type from a short name.
   Constructor-local existentials require scoped skolemization and indexed
   constructors require dependent checking. Carry
   evidence through elaboration; do not erase an unproved obligation to
   make a source check. Use the retained `DSource` contexts and checked
   schemes to produce typed modules and canonical source interfaces.
3. Complete typed core's evidence and module contracts and lower constructors, tuples, cases, closures,
   partial applications and dictionaries under the runtime ABI. Implement
   the same representation and reference-counting contract as the seed.
4. Read checked base interfaces and link the library/runtime artifacts.
   Preserve their hashes, symbols and assumptions rather than guessing
   them from source names.
5. Build and gate stages 1, 2 and 3 as above. Keep the fragment programs as
   regression tests throughout this port.

Related designs: `bootstrap-ladder.md`, `compiled-interface-metadata.md`,
`dependent-core-kernel.md`, `ownership-and-usage.md`,
`effects-capabilities-sequencing.md`, `no-faults.md`, `proof-automation.md`
and `reclamation.md`.
