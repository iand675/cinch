# Next steps (as of 2026-11-05)

## Library ABI facts describe real public entries (2026-11-05)

`emit-runtime-library-artifact-with-object` now returns actual LLVM public
bridges and closure accessors, their checked parameter/result ownership classes,
and the exact final descriptor words. Hidden value indices, runtime dictionary
and value dependencies, returned functions and erased proof continuations use
the same physical plan as the generated workers. Duplicate roots coalesce;
conflicting exports, accessor-name collisions and typed-worker names refuse.
Existing executable and library-root functions are byte-identical in this step.

The complete new runtime-fed group passes: eight physical units and all sixteen
public/accessor definitions match, including Raw/Obj/TObj specialization,
bignum/ADT ownership, partial application, proof-stage closures and hidden Nat
arguments. An independently linked caller runs at 32M and both exposed leak
reports are zero. All 43 compiler modules check. See
`bootstrap/current-runtime-library-abi-results.json` for the actual scope.
The corrected assertion decodes escaped LLVM names. Its explicit Native-fixture
cache checks every source byte and cannot replace a selected self-built frontend.

The published compiler source is now ad6733. These unit/layout facts still need
closed-object profile/content/interface metadata, checked export-root selection
and CLI/adapter wiring. No object-output parity or complete-suite/bootstrap
claim follows from this component. The full Rust run and the preserved 74f0
staged chain retain their own frozen inputs; matching final suites, original
100-target/API coverage and Acceptance 5 remain open.


## Checked library roots and refreshed bootstrap inputs (2026-11-05)

The typed LLVM backend can now emit a library from explicit checked definition
roots and representation vectors without requiring `main`. Only selected roots
have external linkage; reached helpers, constructor/lambda workers, dictionary
workers and closure dispatchers stay local. The executable path preserves its
existing linkage. Public object ABI metadata and CLI/adapter wiring remain open.

The new complete library group and all three original runtime groups pass.
The actual library test assembles two compatible objects, links an independent
caller, checks exact bignum/ADT values and captured closures, and runs at 32M
with ownership diagnostics. All 166 exposed reports have zero mortal leaks;
three existing symbol tests retain their expected immortal interned handles.
All 43 compiler modules check. See
`bootstrap/current-runtime-library-root-results.json` for exact inputs/scopes.

The preceding source repair passes all nineteen complete proof/interface groups:
24 Rust tests, including every original imported-precondition caller case and
conditional-method control. Those groups do not load RuntimeIr; their tested
modules, fixture sources and producer are unchanged by the library-root step.
Their original 74f0 receipt is retained in
`bootstrap/current-cinch-proof-interface-results.json`. Exact polymorphic
intrinsic aliases deduplicate; incompatible globals still refuse. Body owner
lookup uses the longest UTF-8 byte prefix followed by a dot, and the reserved
array names are refused in record declarations as well as data declarations.

Seed and separate library-provider locks now explicitly pin 85f00ca/ABI6 with
Native 22c91 sources and the unchanged empty seed patch. Their fresh archive,
provider and Stage0 were independently verified. The current development Rust
alias fix remains6b5675; changing these explicit roles never silently selects it.
The fresh 74f0 chain has passed Stage1 and its smokes; later stages and its audit
continue on that preserved snapshot. This new 2a23 compiler still needs its own
staged/fixed-point evidence, complete matching suites, original 100-target/API
coverage and Acceptance 5. Newer private ABI bridges are not included here.


## Ordinary aliases retain their source index spelling (2026-11-05)

The full Native sweep exposed a diagnostic regression: an ordinary alias of
`n` made the existing failed bound print `(i + 1) <= i` rather than the source
`(i + 1) <= n`. Index normalization now preserves ordinary source variables
while retaining checked identity for captured bindings and quantified values.
The original `linear_facts` test and its exact assertion are unchanged.

All twelve complete affected groups pass: 150 tests, none ignored or filtered.
All 43 current compiler modules check and every Native test target compiles.
See `bootstrap/current-native-alias-index-results.json`. The full Native sweep
continues on these exact inputs; its earlier failed snapshot remains recorded.
The separate private Cinch loader/array candidate and its explicit 85f00ca
seed/provider retain their own immutable inputs. Matching staged suites,
original API/100-target coverage and Acceptance 5 remain open.


## Quantified proofs keep value identities; checked serializer follows binder scope (2026-11-05)

The published Rust checker admits quantified values in signature predicates only
after checking their kinds. Function types, result promises and body indices
share the quantified parameter's actual local identity. Pattern names cannot
capture that identity. Named proofs and refined argument facts use the same
lexical scope when discharging obligations, including through strict aliases.

All eleven complete affected groups pass:141 tests, no ignored or filtered
cases, all Native test targets compiling and all43 current compiler modules
checking. Positive quantified, refined-input, alias and shadow cases remain;
unknown names, wrong kinds, stronger promises and unproved pattern capture are
refused. See `bootstrap/current-native-quantified-value-results.json`.

The current Cinch source is5cea. Its serializer preserves source binder names
in resolved contracts and uses checked binder positions in display schemes.
Open row contracts stay explicitly unresolved; closed region callbacks retain
their checked contracts. The actual5cea exporter separately passes43 controls
and seven original metadata assertion blocks, including ownership, refinements,
preconditions and region effects. See `bootstrap/current-cinch-serializer-results.json`.
The exact TypeSyntax diagnostic assertion includes its existing obligation
suffix. Original tests and declarations remain; test library roots use their
exact tested ordering.

The complete Native57 sweep is running on this exact production code and source.
Earlier29775 nineteen-group and c8d6 fixed-point results retain their source
scope. Explicit seed and library-provider pins remain25b7e82/ABI6 until a deliberate
refresh; they do not silently select the development Rust compiler. A fresh
matching staged chain, complete final-Stage3 suites, original API coverage and
Acceptance5 remain open. Separate intrinsic-alias and faster loader candidates
are still private and do not form part of this checkpoint.

## Tuple-layout Option tests receive their observed library variant (2026-11-05)

The explicit test library roots now include `Data.Option.is-none[T]` as well
as its object-layout variant. Three original array and shape tests requested
that tuple-layout variant and previously stopped before runtime execution.

A fresh library-only object from the exact earlier ABI6 provider retains all
55 checked interface identities and the same runtime. Replaying all fifteen
original array tests through the same c8d6 Stage3 now yields ten passing and
five failing tests, with no ignored or filtered cases. All three missing-variant
cases pass; positive packed and float programs and original assertions remain.
The Option source is byte-identical to the current published library. See
`bootstrap/current-array-test-library-variant-results.json`.

This corrects the test library inputs. The remaining copy/staging syntax,
intrinsic alias and diagnostic, and object-emission failures remain open.
This earlier Stage3 replay does not establish a current-source full-suite pass.

## Cinch consumes actual method bodies; full nineteen-group gate passes (2026-11-05)

The published 29775 compiler retains actual checked dictionary-method bodies
through loading, source projection, export and proof checking. It follows the
selected implementation's own slot, type, private dependency closure and closed
self dictionary. Names and class identities supply no arithmetic assumptions.
Real self calls are reified through their checked fields; cyclic bodies remain
opaque. Independent Core checking receives the same actual body declarations.

All43 modules and all nineteen complete proof/interface groups pass:23 Rust
harness tests, including both original method-proof matrices. Every original
caller case remains, with explicit body-context removal/refusal and restoration
controls. All28 fresh actual-exporter controls pass with13 zero-leak reports;
independent decoding and bignum execution confirm changed private arithmetic,
identity, chained methods, closure ownership and cycle refusal. All251 matching
auxiliary tests pass. See `bootstrap/current-cinch-method-body-results.json`.

The explicit seed and separate library-provider pins now select published
25b7e82, with their exact archived source hash and ABI6. Fresh Stage0 passes its
smokes and the library-only producer's55 interfaces and all62 artifacts pass
independent audit. A genuine new Stage1/2/3 chain and full current suites remain
required; the c8d6 fixed-point evidence keeps its earlier source scope.

A newer private serializer candidate passes43 exporter controls and seven
unchanged original metadata assertion blocks. Its new collision case exposed a
Native false refusal of quantified values in signature predicates. That Rust
repair is under validation, and the original failing case remains. Original
GADT/local-forall/foreign/source-overlay API parity and Acceptance5 remain open.

## Current Cinch method-body candidate: exporter and auxiliary gates (2026-11-05)

The private 29775 compiler now passes its all-43 module check and all 28 fresh
actual-exporter controls. The original interface metadata assertions and proof
clients remain intact. Independent decoding and execution of the emitted method
graphs confirm private helper closure, changed arithmetic, identity and real
calls between methods; cyclic bodies stay opaque.

All 251 matching auxiliary tests pass: 171 runtime, 41 tooling and 39 editor
checks, with no skipped cases. Source identities, executable identity and complete
logs are independently audited. See
`bootstrap/current-cinch-method-body-progress.json` for the exact private source
scope and the recorded full proof/interface gate status.

A deliberate refresh to the published 25b7e82 Native seed and separate library
producer is prepared privately. The fresh Stage0 passes all three smoke checks.
The separate library-only object passes its ABI, artifact, package-closure and
55-interface decoder audit and contains no Bootstrap implementation modules.
The genuine Stage1/2/3 rebuild waits for all nineteen matching proof/interface
groups. A fresh complete Native sweep is queued on the same source and pins.

This receipt publishes evidence, not the private compiler or refreshed pins.
Main still contains c8d6 and its explicit b64710b pins. Full matching suites,
the original interface/API coverage and Acceptance 5 remain open. Retained prior
runs keep their original source scope and do not substitute for those gates.

## ABI-checked primitive values keep their own body roots (2026-11-05)

Inlining removed call edges to primitives such as `Data.Nat.nat-add`, leaving
those actual public values without executable body contracts. Native now
retains a root for each supported checked primitive whose complete callable
scheme exactly matches the pure plain graph tier. Actual primitive identity,
effects, hidden arguments, ownership and refinements govern admission; source
names do not. Partial Nat operations, wrapping arithmetic and arbitrary foreign
calls remain opaque.

All twelve complete affected groups pass: 234 tests, no ignored or filtered
cases, every test target compiling, all 43 published compiler modules and 65
zero-leak reports. The original seven `total_bodies` tests remain, with their
altered-ordering observation fixture isolated from unrelated string consumers.
An eighth explicitly verifies that both constant ordering replacements reject
the complete library's dependent string proofs. See
`bootstrap/current-native-primitive-body-results.json`.

The private Cinch compiler passes all 43 modules with the new Native producer.
An identical actual Cinch exporter, given the corrected primitive metadata,
now retains private helper bodies, changed arithmetic, identity methods and
real calls between methods in a closed self dictionary, while omitting cyclic
method bodies. Independent decoder and bignum semantic checks pass. A fresh
full exporter build and the complete original nineteen proof/interface groups
remain required before source publication. The original imported-precondition
and conditional-method regressions pass on the preceding source, including
all twenty caller cases and explicit body-context removal/refusal controls.

The published c8d6 compiler and its genuine fixed-point evidence are unchanged.
The earlier full Native run retains 145 passing groups and its original fixture
failure; all remaining 84 groups continue diagnostically on those exact frozen
inputs. The exact c8d6 Stage3 suite also continues. These results retain their
original source scope. The b64710b ABI6 seed/provider pins remain explicit and
unchanged. The new source compiler still needs fresh pins, full matching gates,
a genuine bootstrap and the original API/Acceptance 5 audit.

## Native interfaces retain actual instance-method bodies (2026-11-05)

Checked method implementations were absent from portable interfaces unless an
unrelated exported value happened to call them. That dropped the actual Nat
`<=`, `>`, `>=` and addition roots needed by the Cinch actual-body proof
consumer. Native now emits separate optional instance `method_bodies`, tied to
the actual dictionary slot, checked plain declaration and exact typed body
closure. Private method and helper identities grant no public source names.

All ten complete affected Native groups pass: 224 tests, with no ignored or
filtered cases, plus all test targets compiling and all 43 published Cinch
compiler modules checking. The seven new controls cover all five required Nat
roots, private dependency closure, canonical serialization, malformed payloads,
shared-definition disagreement and nested-module ownership. Every pre-existing
integration test file is byte-identical. See
`bootstrap/current-native-method-body-results.json`.

The matching Cinch loader, exporter and proof consumer are still private and
under validation; original imported-precondition and conditional-method tests
remain required. The c8d6 compiler source and its genuine fixed-point evidence
are unchanged. Supported method records change interface hashes intentionally;
the explicit b64710b ABI6 seed/provider pins remain unchanged in this repair.
A new source compiler and refreshed pins need their own full gates and fresh
bootstrap. Complete matching suites, original API parity and Acceptance 5 remain
open.

## Native runtime arithmetic retains actual implementation authority (2026-11-05)

Runtime expressions crossing into dependent indices or refinement facts now
retain their checked syntax identity, lexical scope, selected callable and
numeric carrier. Only the actual checked mathematical implementation supplies
an arithmetic equation. Type-position operators keep their mathematical meaning.
Named helper reduction reads checked Core instead of treating overloaded source
syntax as an equation. Retained call records store the callable alone, avoiding
copies of large nested argument trees.

A forward operator can be read by a dependent argument or cited proof fact only
with a deferred certificate, verified against its completed actual body before
acceptance. Optional plain runtime argument readings introduce no such assumption.
Arbitrary ordinary dispatch remains valid. Machine arithmetic remains opaque to
mathematical proofs; all original overflow assertions and source-shaped failed
obligation diagnostics are preserved.

All 16 complete affected groups pass: 333 tests, all test targets typecheck,
all 43 unchanged c8d6 compiler modules check, and 135 leak reports are zero.
The 185-test library replay enables debug ownership verification and 32M RC/leak
checks. Eight new controls cover actual arithmetic bodies, annotations/nesting,
dependent arguments, lexical dictionaries, ordinary runtime dispatch, mathematical
type syntax, contextual Int literals and checked helper equations. All nine
original comparison controls and every other existing assertion remain.
The original larger-input counterexample is replayed: normal addition exits 0
with zero leaks; altered addition is rejected at the first affected byte-buffer
proof, and produces no accepted executable. See
`bootstrap/current-native-arithmetic-results.json`.

The separate unpublished Cinch candidate now also checks actual method owner,
name, slot, class arguments and factory target. Its all-43 source gate, all three
semantic controls and original instance-resolution guard matrix pass. Remaining
complete groups and a fresh exporter run continue. Imported branch-precondition
failures expose missing portable method roots: the current library interfaces
carry Nat `<`, but not `<=`, `>`, `>=` or addition. Their actual method bodies must
be exported with validated dependency/slot association before those facts can
be used; class identities or slots cannot stand in for an implementation.

Fresh complete matching Native/self-built/auxiliary suites, original API and
100-target coverage, remaining language features and Acceptance 5 stay open.
The c8d6 four-stage fixed point and explicit seed/provider pins are unchanged.

## Stage3 test inputs follow the entry import closure (2026-11-05)

The portable test adapter now inventories source headers and bundles complete
files reachable from the entry's imports. Lexer sample data such as attrs.cinch
and ticks.cinch stays available to the test program without being compiled as
modules. Imported modules in shared files and nested directories participate.
Every duplicate provider of a requested module still reaches the selected
compiler for its own diagnostic. Invalid entry text also reaches it; there is
no Native program-checking fallback.

All six complete adapter tests pass, retaining the four original cases and
adding two dependency-input controls. All three original bootstrap lexer tests
pass unchanged through actual c8d6 Stage3 executable 645dd0bb, including the
whole-prelude corpus and the previously failing carrying-token forms. The
original full-suite failure remains recorded. See
`bootstrap/current-stage3-import-closure-results.json`.

This is a scoped test-input repair; the complete current matching Native,
self-built and API/Acceptance 5 gates remain open. The Cinch compiler source,
bootstrap fixed point and seed/provider pins are unchanged.

## Native list and byte measure carriers (2026-11-05)

The Rust checker now gives the baked `length` observation its actual checked
operand carrier: Bytes or a list spine, including polymorphic and refined
operands. Runtime decisions infer the operand once, and constructor index
readings preserve their checked type instantiation. Static observations inspect
that same typed constructor form. An ordinary source function named `length`
cannot replace the baked observation. List layouts must have exactly the
empty/cons constructors and an actual recursive tail; a two-field non-list or
an extra constructor is refused.

All 17 original lattice tests now pass unchanged, together with six new carrier,
name-authority and malformed-layout controls. All 15 complete affected groups
pass: 312 tests, all test targets typecheck, all 43 unchanged c8d6 compiler
modules check, and 129 runtime leak reports are zero. The library group runs
all 185 tests with debug ownership assertions, 32M heaps, RC debugging and leak
checks. See `bootstrap/current-native-list-measure-results.json`.

The old b30 full sweep remains preserved as 58 groups passed, one failed and
170 pending; this repair closes its known lattice failure but does not claim a
fresh complete-suite pass. The selected arithmetic operand counterexample is
still open. The unpublished Cinch body-context candidate passes all 43 module
checks, 16 fresh exporter controls and every original bounded-width metadata
assertion; its complete runtime groups continue. Its unchanged 1,500-function
whole group improves from 3870.50 to 396.02 seconds, including compiler build
in both, while retaining the closure and authority checks.

The c8d6 four-stage bootstrap fixed point and explicit seed/provider pins are
unchanged. Complete matching suites, original API/100-target coverage and
Acceptance 5 remain open.

## Native selected comparison facts and typed ownership fixtures (2026-11-05)

The Rust checker now resolves the actual comparison dictionary and reads its
checked implementation, following bounded pure aliases, calls and lets.
Class names and method slots grant no numeric equations. Lexical dictionaries
without implementation authority still dispatch normally but supply no numeric
facts. A body may be forward: only facts actually cited by a proof are recorded,
and their selected-body certificates are verified before accepting the program.
This retains the standard library's valid forward references while rejecting
the changed-order dependent-result counterexample (previously accepted with
runtime exit 42 instead of 0).

All 20 complete affected groups pass independently: 447 unique tests, all 43
unchanged c8d6 compiler modules, and all test targets typecheck. The 185-test
library replay explicitly enables the debug ownership verifier, 32M heaps,
RC poisoning and leak checks. Five handwritten Core fixtures now describe their
allocated objects/dictionaries with the correct ownership class. The whole-value
pattern fixture reads its actual payload rather than comparing an object as an
I64; its expected result and every other assertion remain. The previous compiler
reproduces all five original opt-in leak failures. Successful replays produce
242 zero-leak reports. See `bootstrap/current-native-comparison-results.json`.
The compiler's code-generation implementation is unchanged; its fixture updates
are entirely under `cfg(test)`, with the compiled production prefix audited equal.

These scoped checks do not establish the full outcome. A separate larger-input
probe confirms that runtime arithmetic operands can still be mistaken for
mathematical index arithmetic: an altered Nat addition passes the small class-law
samples but accepts a false result claim and exits 42 instead of 0. This remains
open. The preserved full b30 Native49 sweep stops at lattice: 58 groups pass,
one group fails (seven original List Char measure cases resolve length as Bytes),
and 170 remain pending. Fresh complete matching suites are required after repair.

The separate unpublished Cinch source-body candidate now passes all 43 module
checks, 16 actual exporter controls and every original bounded-width metadata
assertion on Cinch-produced A/Facade contracts. Its complete runtime groups are
still running. Profiling the retained 1,500-function case identifies repeated
quadratic body-set validation inside per-definition/per-expression checks; cache
validated opaque contexts while retaining every closure/authority check.
The canonical c8d6 four-stage fixed point and explicit seed/provider pins remain
unchanged. Original API/100-target coverage and Acceptance 5 remain open.

## Native generic measures and empty collection generalization (2026-11-05)

The full Native sweep found the original loop-measure failure, while the
final-stage sweep exposed its Native oracle's empty-list generalization bug.
Preconditions now read the checked argument telescope, retain the callee's
actual type/index/row/region/evidence instantiation, and normalize solved
metadata before matching scoped facts. Branch facts retain the same generic
measure identity. Numeric predicates still read borrowed contents and reject
stale evidence after writes; compile-time call expectations remain intact.

A copied syntactic value also quantifies private variables in its deferred
dictionaries. One empty collection can consequently be used at both List Nat
and List Bool. Captured, literal and code variables retain the existing value
restriction. Every original case and assertion is retained.

All 24 complete affected groups pass independently: 497 tests, all test
targets, and all 43 unchanged c8d6 compiler modules. This includes the original
32-case collection oracle and its actual compiled Cinch consumer, proof and
syntax cases, the complete concurrency/ownership groups and all 185 library
units. Whole-group low-heap replays pass with 32M heaps and RC/leak checks:
9 linear-fact tests (10 zero-leak executable reports) and 19 surface tests
(25 zero-leak reports). See `bootstrap/current-native-measure-collection-results.json`.

The current four-stage c8d6 fixed point and explicit seed/provider pins are
unchanged. Complete matching Native49 and final-Stage3 suite50 remain required;
their predecessors and failures are preserved. Source total-body contracts,
selected-method proof semantics, original API/100-target coverage and
Acceptance 5 remain open. These scoped results do not establish completion.

## Current projection/proof-runtime compiler bootstraps from Cinch (2026-11-05)

The current c8d6 compiler has a fresh independently audited Stage0/1/2/3
bootstrap. Stage1 rebuilds all 43 compiler modules into Stage2, which
rebuilds Stage3. Every stage smoke passes. Stage2 and Stage3 have identical
LLVM, executables and all 43 canonical compiler interfaces. The actual
interface-only entry consumer passes without compiler source bodies, and
rebuilding changed source produces the expected changed diagnostic.

Both final compilers hash to
`645dd0bbef9bfaec24666c171c2f1b39bd900533e72cb5ec8e7c5800b7bae902`.
The canonical 43-module source remains
`c8d6d87a8d1a698009b1ed820427a7a2bb67dab0f34d170accb41e42678ba985`.
See `bootstrap/current-projection-proof-bootstrap-results.json`.
The explicit b64710b ABI6 seed/provider pins remain unchanged; the separate
Native checked-type scope fix changes neither these inputs nor Cinch source.

This proves Acceptance 1-4 for the current Cinch implementation. Acceptance 5
and the full requested outcome remain open: the complete matching Native48
sweep, all 93 exact Stage3 groups in49, matching auxiliary/external checks,
original 100-target/API scope and the remaining proof/type/FFI/measure work.
Earlier failing and superseded sweeps remain preserved with their own inputs.

## Native checked-type scope inspection (2026-11-05)

The complete Native sweep found an actual regression in its original
NumberBox positive: a helper returning plain data was rejected as though
its result might hide a scoped closure. The structural inspection was
resolving an already checked nominal through the caller's source imports.
It now uses the exact defining type identity when inspecting constructor
fields. Source visibility remains checked by the ordinary source resolver;
opaque/unknown data and data containing closures retain the escape guard.

All seven complete affected groups pass independently: 249 tests including
all 185 library units and all 43 current compiler modules. Existing hidden
closure escape refusals and the original ordinary-data case remain intact.
New positive clients cover private data shadowed in the caller and recursive
plain data. Replaying the complete 17-test concurrency group with 32M heaps,
RC checks and leak reporting yields 14 zero-leak executable reports. See
`bootstrap/current-native-checked-scope-results.json`.

The failing Native47 sweep is preserved. A complete matching Native sweep
is still required; these scoped checks do not close it. Cinch source c8d6
and the explicit b64710b ABI6 seed/provider pins are unchanged, so its active
fresh bootstrap retains its actual input scope. Source total-body export,
remaining original API work and Acceptance 5 remain open.

## Checked projection and decision/proof runtime lowering (2026-11-05)

The Cinch compiler lowers checked Dec constructors and matches, evaluates
runtime decision conditions once, and erases checked proof witnesses while
retaining the runtime value. Explicit `unrefine` consumes an actual checked
refinement through its defining kernel contract and retains its carrier.
Proof lambdas use delayed zero-runtime-argument closures; applying erased
evidence performs the call. Named proof workers preserve owned captures,
alternating runtime/proof stages and every late equation alternative.

All 14 complete affected groups pass independently: 28 tests, 134 runtime
clients with zero leaks, all 15 array tests (31 actual clients), and 20 exact
F32/F64 payload patterns. All 43 compiler modules and every test target also
check. The complete evaluator, Core, independent typing, source proof,
constructor, refinement, decision, interface and projection groups remain
intact. See `bootstrap/current-projection-proof-runtime-results.json`.

The new canonical 43-module source is
`c8d6d87a8d1a698009b1ed820427a7a2bb67dab0f34d170accb41e42678ba985`.
It requires a fresh Stage0/1/2/3 chain and all matching complete suites;
the earlier 31ac fixed-point receipt proves only that earlier compiler.
The explicit b64710b ABI6 seed/provider pins remain unchanged. This runtime
repair does not establish the remaining source proof API, GADT/localforall,
general FFI, total-measure export or safe source-module replacement work.
Original 100-target coverage and Acceptance 5 remain open.

## Source Pi runtime binding expectation (2026-11-05)

The complete source-Pi group passes with every original source case retained.
The implicit `forall (n :: Nat)` case now expects its specified runtime value
binding. Its retained Core must connect the actual opened Nat slot to the
same local identity and carrier; merely accepting the text cannot pass.
Additional negative cases keep unrelated variables, type parameters and
proposition parameters unavailable as runtime values. See
`bootstrap/current-source-pi-runtime-results.json`. Production compiler
modules are unchanged. The earlier failing sweeps remain preserved; full
matching Native/self-built suites and Acceptance 5 remain required.

## Native proof, syntax hygiene and exact float repair (2026-11-05)

The current Rust implementation rejects the original recursive, partial,
cyclic and wrong-carrier proof counterexamples while retaining both valid
induction programs. Staged syntax keeps defining identities, caller scope
and typed namespaces; exact F32 literals parse directly at their declared
precision, including decimal carry boundaries. Proof erasure checks the
termination of discarded computations and finalizes erased value indices.

All 43 complete affected Native groups pass independently: 691 tests without
ignored or filtered cases, the original 16 proof/hygiene CLI commands,
and all 43 current published Cinch compiler modules. Actual portable syntax
clients also execute with 32M heaps and zero leaks. See
`bootstrap/current-native-proof-repair-results.json`; it preserves the tested
source, binary and input hashes and the actual scope of each execution.

This closes the recorded Native proof counterexample for this implementation.
It does not port that proof API to the Cinch compiler. The full matching
Native suite, exact final self-built suites and original API/Acceptance 5
remain required. The 43 Cinch compiler modules and explicit b64710b ABI6
seed/provider pins are unchanged. Earlier failed runs remain preserved.
The old source-Pi ghost-index expectation also needs correction to the
specified runtime value-index behavior, with erasure/identity controls.

## ABI6 tooling and original external checks (2026-11-05)

All 41 tooling tests pass on the current explicit ABI6 pin. Two integration
fixtures now follow the selected pin, while source preparation and final
interface consumption test both ABI5 and ABI6 explicitly. A canonical
wrong-ABI contract still receives the precise ABI refusal. Production
compiler, stage, loader and provider validation behavior is unchanged.
See `bootstrap/current-abi6-tooling-results.json`.

The complete auxiliary gates now pass independently: 171 runtime, 41
tooling and 39 editor checks, with no skipped or filtered cases. Runtime
evidence retains its actual earlier execution source identity and is reused
only after exact equality of every runtime input, including all tests.
Corrected tooling and editor checks execute freshly on the frozen repair.
The Native producer and all 43 compiler module bytes match the published
ABI6 compiler. See `bootstrap/current-abi6-auxiliary-results.json`.

All 14 original external checks also pass independently through the exact
current final ABI6 Stage3: both runner programs with exact reports and 32M
zero leaks; defining-module-only fixities and separate renamed facades;
identical and changed importer redeclaration refusals; and the complete
class-identity group. Its direct Native API negative remains classified
separately. See `bootstrap/current-abi6-stage3-external-results.json`.

The compiler-only library object lacked the `Data.List.length [T]` variant
used by the original runner and allocation tests. The separate library-only
test object is freshly rebuilt from the same pinned b64710b ABI6 producer
and runtime using `bootstrap/test-library-object-roots.json`. All 55 checked
library identities and the runtime archive match the audited stage inputs;
no ABI5 object or client frontend fallback is used. All eight original
allocation tests now pass through final Stage3. The initial failing sweeps
remain preserved. See `bootstrap/current-abi6-test-library-results.json`.
The remaining complete suites, original API scope and proof/Acceptance 5
work remain required; these receipts do not establish full acceptance.

## Previous ABI6 compiler bootstraps from Cinch (2026-11-05)

The published array/effect compiler now has an independently audited
Stage0/1/2/3 bootstrap. Stage1 rebuilds all 43 compiler modules into Stage2,
which rebuilds Stage3. Every stage smoke passes. Stage2 and Stage3 have
byte-identical LLVM, executables and all 43 checked compiler interfaces.
The final compiler consumes its entry through those interfaces without
compiler source bodies. Rebuilding changed compiler source also produces
a changed executable with the expected changed diagnostic.

See `bootstrap/current-abi6-bootstrap-results.json`, for canonical compiler
`31ac09d621b8f8bc7b13f64f3eb6f61965f96507d7f8683b8512d4156ff40fa0`.
Both final executables hash to
`2a7db4640f3fd8ebb272ecd55a9ce9e680f768f555d0953d70ca11ff7544f5f8`.
The seed and separate library-only provider use the explicitly pinned
b64710b ABI6 source; previous ABI5 stages and build caches were not reused.
This proves Acceptance 1-4 for the current compiler.

Acceptance 5 and the complete requested outcome remain open. The full
209-group Native suite and the 92-group exact final-Stage3 sweep are running.
The latter retains every failure and includes all positive ABI6 arrays and
the exact float payload cases. Fresh matching auxiliary checks, the original
API scope and proof soundness remain required. The independently tested
Native proof/float, explicit projection and decision/proof runtime repairs
are separate unpublished candidates and are not claimed by this receipt.

## ABI6 arrays and closed kernel effects (2026-11-05)

The Cinch implementation now lowers immutable Array, Tensor and Columnar
operations with checked element classes, physical layouts, ownership and
constructor ABI guards. Closed Global, Fail, Reentrant and ThreadAffine rows
survive source elaboration, independent Core checking, interface export/import
and renamed facades. Defining intrinsic identity and exact polymorphic schemes
remain mandatory. Machine-word literals and promotion follow the explicit
64-bit bootstrap target, with signed and unsigned boundary refusals.

All 24 complete affected groups pass independently: 104 unfiltered tests,
all 43 compiler modules, 31 actual Cinch array programs with zero leaks,
12 invalid scheme/kind/range controls, three malformed constructor layouts,
and 20 exact F32/F64 payload patterns through updates, slices and mixed-format
append. See `bootstrap/current-array-kernel-effects-results.json`. That source
check used the prior ABI5 stage configuration; its implementation matches the
new ABI6 compiler apart from the explicit RuntimeAbi configuration module.

The seed and separate library-only provider are intentionally pinned to the
published b64710b Rust source at ABI6, with verified source hashes and no patch.
Fresh Stage0 and Stage1 build all 43 modules and pass all six smoke checks.
Independent input, executable and source audits are in
`bootstrap/current-abi6-development-stage-results.json`. Actual Stage1 also
emits, links and executes generic and packed-U8 array clients; both return 42
with 32M zero leaks and actual array runtime calls. See
`bootstrap/current-abi6-development-array-results.json`.

The current canonical compiler identity is
`31ac09d621b8f8bc7b13f64f3eb6f61965f96507d7f8683b8512d4156ff40fa0`.
Its ABI6 Stage2/3 rebuilds and fixed-point proof now pass independently, as
recorded above. Earlier e2f/ABI5 bootstrap and complete-suite receipts retain
their original scope. Full matching suites,
all positive final-Stage3 arrays, original API parity, proof soundness and
the remaining Acceptance gates remain required. The separate proof and decision-runtime
repairs remain unpublished.

## Original Stage3 fixture gaps and evaluator imports (2026-11-05)

The exact e2f Stage3 replay of all original interface fixture programs is
complete: six emit structurally valid checked contracts and seven fail.
The failures establish remaining indexed/GADT constructor and local-forall
elaboration, general foreign declarations, source-defined total-body
provenance for index calls, and safe replacement of source family homes
already present in the supplied checked interface set. The original fixtures
are unchanged. Structural successes do not prove their complete metadata,
consumer or mutation semantics. See
`bootstrap/current-original-interface-fixture-results.json`.

`Run.Eval` now explicitly imports the public `VStr` and `VChr` constructors it
uses. The complete original Native evaluator/oracle target passes. Exact
Stage3 now resolves those names and reaches code generation, which still
refuses missing decision/proof runtime layouts and lowering. That remaining
failure is preserved in `bootstrap/current-eval-source-import-results.json`;
this is a scoped source-import correction, not a passing Stage3 oracle.
Native's expected-family constructor selection is an intentional source rule.
The stopped source45 Stage3 sweep retains its original 26 passes and evaluator
failure; a separate diagnostic continuation runs every remaining original
group without hiding failures. Final complete matching suites remain required.

Only the evaluator program's selective import changed. The 43-module e2f
compiler source, pinned seed, runtime and Prelude are unchanged, so its genuine
Stage0/1/2/3 fixed-point evidence remains valid. The isolated ABI6 compiler,
proof repair and explicit projection candidates remain unpublished and need
their own full gates. Original API coverage and Acceptance 5 remain open.

## Rust source family replacements keep their manifest position (2026-11-05)

A byte-identical source copy of `Data.Core` previously made valid programs
fail in embedded `Data.Nat`: every source replacement was deferred until after
the Prelude facade, so early consumers could not see `Option`. Replacements
now occupy their original manifest slot. Early Core/Ordering homes precede
their consumers; late category/algebra homes retain the facade's aliases.
Checked source hashes and interface dependency chains remain authoritative.

All nine complete affected Native groups pass independently: 87 tests with
no ignored or filtered cases, plus all 43 Cinch compiler modules. The original
array program now checks and emits with the unchanged Core source copy.
Changed Option payloads and reversed List fields are refused by real embedded
consumers; reversed Option tags are refused by the existing codegen layout
guard. None of the malformed programs is executed. See
`bootstrap/current-rust-family-home-results.json` and
`bootstrap/current-native-array-abi-controls.json`.

Cinch source, ABI and the pinned seed are unchanged, so the e2f bootstrap
fixed-point proof below remains scoped to the same compiler and pinned inputs.
Fresh matching auxiliary checks pass independently on this Rust producer:
171 runtime, 41 tooling and 39 editor tests. All 14 original external checks
also pass through the unchanged exact e2f Stage3, including the separately
classified Native API negative on the new host. Source snapshots, complete
logs and executable hashes are independently audited in the current
auxiliary and Stage3 external receipts. The complete Native and Stage3
sweeps continue; preceding-producer results retain their original scope.
The recursive-proof defect, unpublished array/effect work,
positive ABI6 stage transition and original API/Acceptance 5 remain open.

## Current row compiler reaches a genuine fixed point (2026-11-05)

The published row/import compiler e2f now has its own independently audited
Stage0/1/2/3 bootstrap. Stage1 freshly rebuilds all 43 compiler modules into
Stage2, which rebuilds Stage3. All stage smokes pass; Stage2 and Stage3 have
byte-identical IR, executable and 43 checked interfaces. Stage3 checks its
entry using those interfaces without compiler source bodies. Stage2 also
rebuilds changed compiler source into a changed executable whose diagnostic
proves the changed behavior. The unchanged pinned seed and admitted base
artifacts are recorded explicitly; no previous Cinch compiler stage was reused.
See `bootstrap/current-open-row-bootstrap-results.json`. This proves
Acceptance 1-4 for compiler
`e2f15c51f24f38e74ddf61943d96d828cade2e60f2cdc7db67c6eafdefd3f0a5`.

All matching auxiliary gates also pass independently: 171 runtime, 41 tooling
and 39 editor tests, with no failures, ignored cases or filtering. Complete
logs, frozen source and actual executable identities are audited in
`bootstrap/current-open-row-auxiliary-results.json`.

The source-decisions regression now expects opaque propositions to resolve
successfully before the checker rejects their missing executable decision
procedure. It demands that precise diagnostic and forbids a generic resolver
failure. The complete group and all 43 compiler modules pass independently;
production compiler inputs are unchanged. See
`bootstrap/current-source-decision-phase-results.json`. The complete corrected
group also passes through exact current Stage3, independently audited in
`bootstrap/current-source-decision-stage3-results.json`. The complete Native
and final-Stage3 sweeps remain in progress. Older frozen runs retain their
original expected-phase failure rather than being relabelled.

All 14 original external checks also pass through exact current Stage3:
runner emit/link/execution, exact full reports and 32M zero-leak runs,
defining-module-only fixities, renamed and separate facades, identical and
changed importer redeclaration refusals, and the complete class-identity
group. Its two portable builds use Stage3; its direct Native API negative
stays classified separately. Artifacts, source, compiler identities and full
logs are independently audited in
`bootstrap/current-open-row-stage3-external-results.json`.

Acceptance 5 remains open. The exact published Rust compiler still accepts the
recursive false-proof counterexample; the isolated repair is not integrated.
Positive ABI6 array lowering, original API/target coverage and complete matching
final suites remain required. The separate array candidate has executed all
31 runtime programs with expected results and zero leaks, but its corrected
ABI fixtures and defining intrinsic interface roundtrips still need full
validation. None of that unpublished candidate is claimed by this bootstrap.


## Checked open rows, imported tails and operation values (2026-11-04)

The Cinch compiler now elaborates implicit and explicit Row parameters,
retains each abstract tail's identity, and normalizes joined rows without
losing known labels. Imported resolved schemes preserve their checked bound
row tails. Actual separately emitted interfaces and a renamed reexport accept
safe propagation and reject erasure and substitution of a different tail.
Wrong-kind and unbound imported tails remain refused.

Rust now accepts a bare effect operation as a function value using a checked
forwarding lambda. Pure construction stays pure, including renamed facade
aliases; the existing D23 refusal for handling effects through a called
computation is retained. This does not add dynamic handler dispatch or direct
partial-operation application.

All nine complete matching suites pass independently: 57 Rust test cases,
all 43 compiler modules, six writer/import/canonical-readback checks and three
actual renamed-facade consumer checks. See
`bootstrap/current-open-row-operation-results.json`, compiler
`e2f15c51f24f38e74ddf61943d96d828cade2e60f2cdc7db67c6eafdefd3f0a5`.
The broader 33-group, 139 zero-leak runtime program and 60 row-semantic
assertion receipt is retained separately as source5 evidence in
`bootstrap/current-open-row-source-results.json`; it was not rerun on this
combined nominal/Rust-operation source. Rust's independent four-group,
51-case and 15-oracle repair is in
`bootstrap/current-rust-operation-value-results.json`.

The direct recursive-evidence counterexample is still independently accepted
by this exact Rust candidate. Its receipt is updated; proof soundness remains
a completion blocker. The e861 stage-chain and complete-suite receipts below
belong to the preceding source. This row compiler needs its own fresh chain
and complete matching suites, original API/positive ABI6 array behavior and
full Acceptance 1-5 audit. The array implementation remains isolated and
unpublished while actual runtime failures are corrected.

## Current proof audit and complete auxiliary gates (2026-11-04)

The exact published Rust row-kind compiler still accepts four invalid proof
programs: direct recursion, a partial proof match, recursion hidden behind a
pattern branch, and Boolean patterns over a natural-number proof subject.
The unchanged valid induction control checks, while its captured-induction
variant is refused. These are current checks, independently reproduced;
no invalid proof executable was run. The actual e861 Stage3 compiler refuses
all six inputs during name/namespace resolution, including both valid controls.
Those unsupported-source refusals do not establish proof soundness.
See `bootstrap/current-proof-frontends-audit.json`. The isolated ongoing proof
repair must pass all these negatives and retain the valid controls before
integration; green existing tests cannot close this blocker.

All complete matching auxiliary gates independently pass: 171 runtime tests,
41 tooling tests and 39 syntax/editor tests, with zero failures, ignored tests,
skips or filtering. Source, producer and complete log identities are checked in
`bootstrap/current-opaque-auxiliary-results.json`. This does not close the full
compiler/self-built suites, original API/array behavior or Acceptance 5.

The Stage3 report-directory harness failure is repaired. The external artifact
now inherits the caller's working directory, matching the Native driver.
All four complete harness tests and the original checker test pass under
Native; that original complete checker also passes through actual e861 Stage3,
including its exact full diagnostic report. See
`bootstrap/current-stage3-harness-cwd-results.json`. Compiler, prelude, runtime
and bootstrap sources are unchanged. The previous selected sweep remains
recorded as 16 passing groups followed by one failed harness group; this scoped
rerun does not replace the full remaining Stage3 suite or Acceptance 5.

## Verified opaque/HK bootstrap chain and Stage3 acceptance (2026-11-04)

Compiler e861 now has an independently verified full Stage0/1/2/3 chain.
Stage1 rebuilds all 43 compiler modules into Stage2; Stage2 rebuilds Stage3.
All stage smokes pass. Stage2 and Stage3 have byte-identical IR, executable
and 43 checked interfaces. Stage3 checks its entry against those interfaces
without compiler bodies. Stage2 also rebuilds changed compiler source into
a different executable that exhibits the changed diagnostic. This proves
Acceptance 1-4 for the published opaque/HK snapshot, with the unchanged
pinned seed and admitted base dependencies. The locked ABI5 chain alone
does not prove positive live-ABI array lowering.

All 14 original final-Stage3 external checks pass independently: exact mixed
and passing runner reports, 32M zero leaks, defining-module-only fixities,
renamed and separate facades, identical and changed importer redeclaration
refusals, and the full class_identity group with its explicit Native fixture.
Evidence is `bootstrap/current-opaque-bootstrap-results.json` and
`bootstrap/current-opaque-stage3-external-results.json`.

Acceptance 5 remains open: the complete matching suites, original target
coverage and the confirmed recursive-proof soundness defect still require
completion. The joined-row source and its resolved-contract follow-up are
separate unpublished candidates; these receipts do not prove their bootstrap.

## Genuine Stage2 build; recursive-evidence blocker (2026-11-04)

The published opaque/HK Cinch compiler now builds a genuine Stage2 compiler
through Stage1, rebuilding all 43 compiler modules. Stage2 passes all three
smoke checks. Independent checks verify the actual Stage1 builder hash,
Stage2 executable hash, frozen source and all 43 emitted compiler interfaces.
See `bootstrap/current-opaque-stage2-results.json`. Stage3, fixed-point code
and interfaces, API consumption and changed-source checks remain open.

A separate check against the exact current published Rust implementation
confirms that an ordinary recursive proposition-returning function can be
accepted as evidence for the false refinement Zero at value 9. The complete
source and compiler/input identities are in
`bootstrap/current-recursive-evidence-blocker.json`. This current reproduction
checks acceptance only; it does not execute the unsafe program. A related
isolated repair is in progress. Existing passing suites do not close this
soundness defect, which must be repaired and verified before goal completion.
The complete matching suites, remaining original API/array behavior and the
full Acceptance 1-5 audit are still required.

## Verified Rust row-parameter annotations (2026-11-04)

Rust now retains explicit `forall (e :: Row)` binders and rejects a row tail
annotated as a proper type or value. A binder cannot simultaneously name a row
and a type, value or region. Ordinary unannotated rows remain inferred; explicit
Region annotations retain their existing refusal. Nominal kinds and ABI are unchanged.

Four complete Native suites pass all 28 tests, including all original effects,
parameter and interface cases. All 43 compiler modules and ten original CLI
reproductions pass independently. Evidence is
`bootstrap/current-rust-row-kind-results.json`; this is scoped Rust evidence.
The separate Cinch open-row implementation, matching complete final suites and
the full Acceptance 1-5 audit remain open.

## Verified opaque declarations and inferred constructor kinds (2026-11-04)

The Cinch checker now retains opaque declarations as abstract nominal identities
with their checked parameter and result kinds. Public aliases and private value
closures export the defining opaque identity; no constructors, layouts or empty
closed data families are introduced. Builtin replacement, forged literals,
empty-case elimination, wrong argument kinds and distinct-identity confusion
remain rejected. The complete actual producer is also accepted by Native Rust.

Unannotated forall parameters infer ordinary constructor arity from actual type
applications and checked class argument kinds. Explicit kinds remain authoritative;
conflicting arities and dictionary/application kinds remain rejected. Six new
source cases exercise both accepted inference and these refusals.

All 43 compiler modules and 30 complete affected groups pass independently, including
all 139 runtime programs with expected results and zero leaks, plus 335 independent
Core assertions. The original parser corpus now models its opaque declaration.
The proof is `bootstrap/current-opaque-kind-inference-results.json`, for compiler
`e861abb04f804a94b1dfe6f65fcddab57005fdc24837b3abfedb5482dfde0208`.
Its exact Rust producer is recorded; later repairs need matching final suites.
Fresh Stage1 built and passed all three smokes. Stage2/3 fixed-point verification,
complete matching suites, remaining original API fixtures, positive live-ABI array
lowering and the full 100-target/Acceptance 1-5 audit remain open.

## Rust numeral refinements use checked lexical facts

Numeric literals and folded negative numerals retain their static predicate and
carrier-range checks. If a closed numeric refinement instead needs the actual
lexical facts, checking uses the same scoped subsumption as an ascription.
This admits `0 <= limit` for a Nat argument and declared ordering assumptions;
false or unproved promises, ignored invalid literals and overflow still fail.
Failed scoped checks retain the original literal diagnostic.

All83 tests across six complete affected suites pass, with none ignored or
filtered. The rebuilt compiler checks all43 Cinch compiler modules. Independent
checking of the complete actual opaque-interface producer source also passes,
including the direct numeral that exposed this bug. See [the scoped audit](../bootstrap/current-rust-scoped-literal-results.json).
Cinch source identity stays
`80413b5dccb1f43943acec2b0c38b8764481417cb2663289d3eee6e75810b470`.
The Cinch opaque/constructor-kind follow-up and matching final complete suites
remain separate work; Acceptance5 and the full goal stay open.

## Cinch refinement interfaces preserve defining contracts

The Cinch interface writer now retains public refinement templates and the
private defining aliases used by exported values, constructors, methods and
instance heads. Renamed and separately emitted facades retain those identities
without publishing private names. Independent schema, canonical serialization,
hash, malformed-contract and safe/unsafe consumer checks pass.

Named `refined NonZero Nat` syntax now reaches the existing checked predicate
path. Dependent exports preserve argument names, and result binders avoid
capturing an argument named `cinch-result`. The pipeline requires actual public
refinement output instead of its former blanket refusal; unsupported foreign
assumptions remain explicit errors.

All43 compiler modules,27 complete focused groups,139 runtime programs with
expected results and zero leaks, and335 independent Core assertions pass.
Compiler identity: `80413b5dccb1f43943acec2b0c38b8764481417cb2663289d3eee6e75810b470`.
See [the independent scoped audit](../bootstrap/current-defining-refinement-interface-results.json).
The frozen executions identify their exact Native producer; the later verified
Rust opaque-kind repair is separate evidence. A final matching bootstrap and
complete suites are still required.

Eleven original interface-source fixtures were also exercised through the actual
ABI6 diagnostic Cinch frontend: five emit structurally valid artifacts, six
still refuse. This is diagnostic evidence, not final Stage3 or full semantic
parity proof. Opaque declarations, constructor-local quantifiers, total-call
contracts, open effect rows and prelude source overrides remain work. The opaque
follow-up is isolated; Acceptance5 and the full goal remain open.

## Rust opaque parameter kinds retain checked contracts

Opaque declarations now retain their checked value and higher-kinded parameter
kinds. Portable closed value binders use their actual Nat, Int or Bytes carrier;
opaque nominal closures preserve those kinds through checked interfaces and
renamed facades. Opaque types still have no constructors or projected fields,
and wrong argument kinds remain errors.

All89 tests across ten complete affected groups pass, including the existing FFI
runtime cases. The rebuilt Native compiler checks all43 Cinch compiler modules;
all seven original opaque CLI cases now agree. See [the independent scoped audit](../bootstrap/current-rust-opaque-kind-results.json).
Cinch sources retain identity
`ffee20f54e9b9b34b039104b4142b0454f28ead43adfec29753f3a8977fa2d11`.
The Cinch opaque/interface implementation remains an isolated candidate.
Matching final complete suites and Acceptance1-5 remain open.

## Rust anonymous refinement literals repaired

The Rust compiler now checks numeric literals using the binder in the actual
refinement view. Anonymous forms such as `refined NonZero Nat` no longer depend
on a named-alias table entry. Carrier range checks and the static predicate
proof remain required: zero, the wrong carrier, negative naturals and machine
overflow are still rejected.

All82 tests across six complete affected groups pass, with none ignored or
filtered. The newly built Native compiler checks all43 Cinch compiler modules;
all six original CLI reproductions agree. See [the independent scoped audit](../bootstrap/current-rust-anonymous-refinement-literal-results.json).
Cinch compiler sources retain identity
`ffee20f54e9b9b34b039104b4142b0454f28ead43adfec29753f3a8977fa2d11`.
A complete matching Native sweep and final Cinch-built suites remain required;
the preceding compiler's stage-chain proof does not close Acceptance5.

## Refinement obligation diagnostics repaired

Failed refinement checks now name the undischarged obligation while preserving
the existing diagnostic prefix. Later definitions are not checked against the
failed recovery state, avoiding false coverage and type errors. The invalid
program is still rejected; proof rules and successful checking are unchanged.

All43 compiler modules and21 complete affected groups pass, including139 runtime
programs with expected outcomes and zero leaks and335 independent Core assertions.
Compiler identity: `ffee20f54e9b9b34b039104b4142b0454f28ead43adfec29753f3a8977fa2d11`.
See [the independent scoped audit](../bootstrap/current-refinement-obligation-results.json).
The [nine unchanged original byte tests](../bootstrap/current-original-byte-diagnostic-results.json)
also pass through the genuine ABI6 diagnostic Cinch frontend. That frontend is
explicitly distinct from the final pinned-ABI5 Stage3 compiler.

The preceding ef4 compiler completed its genuine Stage0/1/2/3 chain, but its final
Stage3 suite confirms the known byte-reader diagnostic failure (8/9 pass).
This repair still requires its own matching final chain and full suites.
Refinement interface repairs remain isolated candidates. Acceptance5 and the
full goal remain open.

## Value-index compiler completes the genuine bootstrap

The published value-index/byte-contract compiler completes a fresh
Stage0/1/2/3 chain. Every stage passes all three smoke tests. Stage2 and Stage3
have identical compiler IR, executables and all43 canonical checked interfaces.
The final compiler checks its entry through the emitted APIs without compiler
bodies. Stage2 also rebuilds actually changed compiler source; that executable
produces the changed diagnostic. Only the unchanged pinned seed and admitted
base dependencies were reused.

[The independent bootstrap audit](../bootstrap/current-value-index-bootstrap-results.json)
proves Acceptance1-4 for compiler
`ef4f399642a225f45acf5bc95dc6b41816390b8b00393446870349c2752431d4`.
Acceptance5 remains open: complete matching suites38 and the100-target behavior
and API coverage audit must finish. The diagnostic follow-up passes all9
unchanged original byte tests through its diagnostic Cinch frontend, but that
later source needs its own matching final chain and suites. Interface repair
work remains isolated; none of its results proves the published source.

## Executable value indices and byte contracts

Implicit value parameters now have checked lexical/type/proof/task scopes and
real owned runtime argument slots. Byte-length witnesses execute the actual
operation; partial applications and closures retain their owned values.
Constructor indices stay erased, with normalized bound indices required to
match the same binder and checked carrier. Byte-read refinement declarations
are proved from actual runtime bounds, admitting Int-promoted range aliases
and refusing stronger promises without granting immutable-read authority.

All43 compiler modules and twelve complete focused groups pass. All139 runtime
programs return their expected results with zero leaks, including the indexed
constructor with an owned field. All335 independent Core assertions pass.
Compiler identity: `ef4f399642a225f45acf5bc95dc6b41816390b8b00393446870349c2752431d4`.
See [the independently verified checkpoint](../bootstrap/current-value-index-contract-results.json).

The unchanged original byte target passes8/9 through the diagnostic frontend.
Its unsafe mutable-reader case is rejected, but the error wording lacks the
expected obligation marker and produces secondary diagnostics. That fix,
source interface gaps, the matching fresh bootstrap and complete suites38,
and the full Acceptance1-5 audit remain open. Earlier chains prove their own
source identities only. No full bootstrap completion is claimed here.

## Typed byte helpers and allocator proof contracts

Checked pure identity helpers now preserve byte operand carriers in source
indices, including refined operands and nested helper calls. Allocation
refinement contracts are proved from the actual kernel fact that the returned
buffer length equals its input. False promises, effectful or wrong inputs and
forged unnamed refinements are refused.

All43 compiler modules and eight complete focused groups pass, with135 runtime
programs returning their expected outcomes and zero leaks. Seven new allocation
checks run at five identifiers alongside all existing Core checks. Four source
helper cases retain precise acceptance and refusal checks. Compiler identity:
`1b2eb541e7712c3defff20e8a0b81b651457050019a0bbe5502d2111a24583d4`. See [the scoped evidence](../bootstrap/current-byte-helper-allocation-results.json).

The unchanged original nine-test diagnostic passes six tests on this source.
Three failures remain: local application carrier diagnostics, executable value
parameters and the mutable-reader obligation diagnostic. Value-parameter work
is in a separate validation snapshot. Source opaque declarations, a fresh final
matching bootstrap, all complete suites and the Acceptance1-5 audit remain open.

## Checked byte-read primitive

The Cinch compiler now checks the actual bytesGet runtime contract: Bytes and
Nat operands produce a Nat byte in the closed range 0..255. Runtime lowering
converts the index through the checked runtime ABI and releases both owned
operands. This does not grant mutable reads the immutable length primitive's
proof authority.

All43 compiler modules and seven complete focused groups pass. All134 runtime
programs return their expected exits with zero leaks, including the new
byte-read alias program (exit221). Compiler identity: `b4b6367f93e165500d4589348684135f83c4746c66f91a2c9d1e683b98cfc3a7`.
[The scoped proof](../bootstrap/current-byte-read-primitive-results.json)
records exact inputs, executables, full logs and the actual runtime case.

Typed helper normalization, allocator refinement proofs, executable index
witnesses and source opaque declarations remain separate work. The running
chain11 and suites36 concern preceding API sourcea93. A final matching bootstrap,
all complete suites and the full Acceptance1-5 audit remain required.

## Checked intrinsic type views

A runtime intrinsic now retains every exact checked source/interface type view
under its one operation identity. Each view is closed, well-kinded and physically
compatible with the others. A reference must match a declared view. Ordinary
duplicate definitions and role collisions remain errors. This lets a plain
primitive alias coexist with the base library's stronger checked refinement.
Actual carrier failures now name the expected carrier, including Bytes.

All43 compiler modules and seven complete focused groups pass. All133 runtime
programs return their expected exits with zero leaks; six new independent Core
checks execute at five large identifiers (30 assertions). Compiler identity:
`445dbe70b656f15c42d45c1c17ba9d0de02b24a6bfa5fd2c503b860dd07ac169`. [The scoped proof](../bootstrap/current-intrinsic-view-results.json)
records exact source, executable and complete-log hashes.

This does not close the original nine-test byte-length parity target. Byte reads,
typed helper reification, allocator promises and executable index witnesses still
need their matching validation. The running chain11 and suites36 concern the
preceding API compiler a93, not this new source. A new final matching bootstrap,
all complete suites and the full Acceptance1-5 audit remain required.

## Scope and Task checked interfaces repaired

The Cinch writer now exports the actual builtin Scope and Task types, preserving
Region binder kinds and defining-module ownership through reexports. It adds no
fabricated nominal owner. Independent Native checked schemes, schema readback,
canonical serialization, and actual Cinch import clients agree. Existing private
nominal and declaring-module-only fixity checks remain intact.

All 43 compiler modules and nine complete focused groups pass, including all
133 runtime programs with expected exits and zero leaks. The compiler identity
is `a93c6a7eef3a95be2d1fbac87794cb08eb6b0fcc00d293a521c9fb568186cc27`. See [the independently verified API repair](../bootstrap/current-task-interface-export-results.json).

This supersedes the prior task compiler's source, so its successful four-stage
chain does not prove this new source. A fresh matching chain and complete suites
remain required. The unchanged full self-built HashMap target exposed 13 missing
library-object representation variants in test support. The checked library
roots now include those actual requests; all checked interface documents are
byte-identical to the previous library artifact. Both original HashMap tests
pass through the exact preceding compiler's Stage3, preserving the 32,000-key,
collision, correctness and growth assertions. The new compiler repeats those
tests in its own matching suites. See [the scoped library proof](../bootstrap/current-hashmap-library-support-results.json)
and [the reproducible library roots](../bootstrap/self-built-test-library-roots.json).
Compiler stage roots and pins are unchanged. Acceptance 5 and the full goal remain open.

## Published task compiler rebuilds itself; API export follow-up open

The published structured-task compiler completes a fresh Stage0/1/2/3 chain.
All stage smoke tests pass. Stage2 and Stage3 have identical compiler IR,
executables and all43 checked interfaces. The final compiler consumes those
actual emitted APIs without compiler bodies; Stage2 rebuilds changed compiler
source and the rebuilt executable exhibits the changed diagnostic.
See [the independent chain proof](../bootstrap/current-structured-task-bootstrap-results.json).
The original four-test exhaustion group also passes through fresh Stage1,
including actual task stack exhaustion. Complete final Stage3 tests remain open.

A separate module-boundary probe found Scope/Task interface export refusals.
The local Cinch repair passes real writer/reexport/reader/client and pipeline
checks. Its type-export oracle exposed a Rust producer gap: checked Scope/Task
signatures omitted resolved metadata. That Rust repair now passes nine tests
across six complete groups, including source Shadow.Scope retaining its owner.
See [the scoped Rust proof](../bootstrap/current-rust-task-interface-results.json).
All43 compiler sources check through the repaired host. The combined Cinch/API
repair is in a new nine-group run; it requires a fresh final source chain and
all matching suites. Acceptance1-4 above applies to published6b task source;
Acceptance5 and the full goal remain open.

## Structured task execution verified

The Cinch compiler now elaborates withScope, spawn and joinAllOrCancel into
checked Core and real shared-runtime threads. Scope identity, Io evidence,
effects, exact-once handle use, capture sharing and hidden lifetimes are checked
independently before emission. Only the declaring kernel owns these forms;
ordinary source shadows retain their own behavior. Joined Result layouts use
the actual checked Data.Core declaration.

All 43 compiler sources and six complete focused groups pass. All 133 runtime
programs return their expected results with zero leaks. Eight new programs cover
single/pair joins, owned values, immutable captures, nested scopes, cancellation,
a saturated scope helper, empty scopes and ordinary builtin-name shadows.
Twelve task source refusals remain; ownership/lifetime cases must reach the
actual safety diagnostic. Fourteen new independent task-node checks execute at
five large identifiers alongside all 23 existing scoped-region checks.
See [the complete focused task proof](../bootstrap/current-structured-task-results.json).

The repair also discovers imported dependencies inside do blocks and infers
fresh callback effect rows. Task error types left unconstrained after complete
body checking and dictionary/numeric resolution instantiate Unit. Public and
rigid types and ordinary private-type sealing retain their existing discipline.

The compiler source identity is 6b5e776bb196631c6c42476ec08e0f8e801a60e5fd95ea7fb21f7dfb9d776620. This does not establish full
ownership/provider parity or close Acceptance 1-5. The unchanged original
self-built exhaustion test, a fresh Stage0-3 chain and all matching complete
suites and the final coverage audit remain required. The separate Rust aggregate
scope-escape repair passes all60 focused tests; the full matching suite remains required.

## Rust aggregate scope capture repaired

The Rust frontend now rejects an application that hides a live structured-scope
token inside a returned function, tuple or data wrapper. Previously, partially applying a helper could
return a closure whose type omitted the captured scope, bypassing the visible
result-type escape check. The application now follows the existing explicit
lambda capture rule and inspects actual checked constructor fields. Saturating
the helper inside the scope and returning ordinary data still work.

All 60 tests in the five complete concurrency, exhaustion, ownership, effects
and region-interface groups pass, with none ignored or filtered. The new
regression covers direct, aliased and tuple-wrapped escapes and executes a
valid saturated helper. See [the scoped Rust proof](../bootstrap/current-rust-hidden-scope-capture-results.json).

The separate Cinch task implementation passes its six complete focused groups.
A fresh full bootstrap and complete matching suites remain required.
Acceptance 1-5 and the full goal remain open.

## Rust hidden scope capture repaired

The Rust frontend now rejects an application that hides a live structured-scope
token inside a returned function. Previously, partially applying a helper could
return a closure whose type omitted the captured scope, bypassing the visible
result-type escape check. The application now follows the existing explicit
lambda capture rule. Saturating the helper inside the scope still works.

All 59 tests in the five complete concurrency, exhaustion, ownership, effects
and region-interface groups pass, with none ignored or filtered. The new
regression covers direct, aliased and tuple-wrapped escapes and executes a
valid saturated helper. See [the scoped Rust proof](../bootstrap/current-rust-hidden-scope-capture-results.json).

The separate Cinch task implementation passes its six complete focused groups.
A fresh full bootstrap and complete matching suites remain required.
Acceptance 1-5 and the full goal remain open.

## Scoped-region Core foundation verified

Core now has an explicit fresh-region binder for structured scopes. The
independent checker binds that Region only inside the callback, requires
matching lexical Io evidence, checks owner/result/effect identity and rejects
reused binders and visible region escape. All Core substitution, proof,
reference, dictionary and runtime capture/dependency traversals preserve it.

All 43 compiler sources and six complete focused groups pass on identical
compiler inputs. The corrected scope fixture executes 23 checks at each of
five large identifier sizes, retaining all original typing tests. All 125
existing runtime programs return their expected values with zero leaks.
See [the scoped Core proof](../bootstrap/current-scoped-region-core-results.json).

This is a Core foundation. Source task forms, linear handle consumption and
capture/lifetime checking are still unfinished. Scoped nodes explicitly refuse
runtime emission until that safety work and physical task lowering are ready.
No new bootstrap or task execution is claimed. The current compiler identity
is 6cb4a980e5d7a90ffdb621d456092233ae4e3d8c113c3ea953d37b45b08441c5; Acceptance 1-5 remains open.

## Direct sequencing validated; structured tasks remain open

Direct `do` now parses explicit and layout blocks, scopes bindings in order,
retains effects and keeps statement binds monomorphic. Pure local let groups
retain checked polymorphism. The runtime backend specializes generalized local
function values at their checked type arguments, including owned captures and
aliases inside another closure. Strict computed initializers retain their
once-evaluated path and existing refusals.

All 43 compiler modules and eight complete focused groups pass. All 125 runtime
programs return their expected values with zero leaks. Twelve new source
cases agree with the Rust frontend. An observable effect-order case writes
exactly three bytes, evaluates each dependent statement once and returns 193;
both frontends execute it with zero leaks. Original tests and object
ABI/refusal/allocation checks remain. See [sequencing evidence](../bootstrap/current-direct-sequencing-results.json).

The preceding Rust-witness/Cinch 018a snapshot also completes all 208 compiler
groups, totaling 1,274 tests, with no ignored or filtered cases. Every test
executable and full log was independently audited. See [the complete Native
sweep](../bootstrap/current-native-witness-sweep-results.json). Its Rust,
prelude and runtime inputs match this sequencing snapshot; its tests do not
replace full verification of the changed Cinch compiler.

The new Cinch compiler identity is 85f7cb2b0a6f7bd6e8989659be5ee5071b44a74d775bf7510010a76ee9e45bc8. A fresh final
bootstrap is still required. The previous 018a chain below remains historical
evidence for its exact sources. The recorded self-built task-exhaustion test
still requires structured task typing, lifetime and handle-consumption checks,
and actual task runtime lowering. Result/Option provider selection and broader
ownership parity are not established here. Acceptance 1-5 is not complete.

## Current acceptance blocker: structured tasks

The exact final Stage3 compiler passes the complete original argv, allocation,
case-evidence and class-identity groups. The recorded 018a exhaustion group passes
three tests and fails its task stack-exhaustion case because that compiler
lacks `withScope` and direct `do`. The newer sequencing repair is documented
above; structured tasks still need implementation. The original
assertion remains unchanged; the fix must create a real runtime task and
retain scope lifetimes, handle consumption, effects and cancellation.
See [the recorded failure](../bootstrap/current-self-built-task-blocker.json).

Matching final-source auxiliary checks pass all 171 runtime, 41 tooling and
39 editor tests. All 14 exact Stage3 external acceptance checks also pass.
See [auxiliary evidence](../bootstrap/current-source-auxiliary-results.json)
and [external evidence](../bootstrap/current-final-external-results.json).
The full Native sweep remains running. The final self-built sweep stopped
after its exhaustion failure, with 86 groups still pending. Acceptance5 is
open. Sequencing changes are local and under focused validation; they require
a fresh bootstrap chain after validation.

## Previous018a case-repaired bootstrap checkpoint

The repaired Cinch compiler now completes a fresh Stage0/1/2/3 chain.
All stages pass their three smoke checks. Stage2 and Stage3 have identical
compiler IR, executable bytes and all 43 canonical checked interfaces.
Stage3 checks its actual entry against the emitted APIs without compiler
bodies; Stage2 rebuilds changed compiler source and the rebuilt executable
exhibits the changed diagnostic. See [the current chain proof](../bootstrap/current-case-dispatch-bootstrap-results.json).

The complete original case-evidence target now passes all 17 tests through
both fresh Stage1 and final Stage3, including the original allocation
comparison. The Rust diagnostic-only follow-up leaves every Cinch, prelude
and pinned-bootstrap input unchanged, so this chain remains current.
Acceptance 5 is still open: matching full Native and final self-built suites,
external/auxiliary checks and the final coverage audit remain required.

## Rust witness diagnostics preserve source expressions

Failed call obligations now display the user's original source expression
instead of the internal once-evaluated witness name. Proof checking keeps its
actual private identity; rendering happens only after failed discharge.
The original FFI diagnostic assertion is unchanged.

All 109 tests in seven complete affected Native groups pass, with no ignored
or filtered cases, and the repaired Rust CLI checks all 43 compiler modules.
See [the scoped diagnostic proof](../bootstrap/current-rust-witness-diagnostics-results.json).
All Cinch, prelude and pinned-bootstrap inputs are byte-identical to the
validated case/dispatch repair, so its fresh chain9 remains valid. The matching
full Native and exact final self-built suites are rerunning on this Rust snapshot.

## Current repair: checked method dispatch and scalar case facts

The current source now removes the allocation regression in the original
case-evidence loop. Fully applied closed methods use the actual private worker
selected by the checked declaring class, slot and exported instance metadata.
ABI checks and strict dictionary-value initialization remain enforced.

Boolean and numeric case rows retain facts about their actual scrutinee.
Later rows receive earlier-pattern refutations only when those earlier rows
cover their match; a failed guard does not refute its pattern. Pattern aliases
retain value identity. Elaboration, Core certificates and independent typing
use the same scoped rule, including literal True guards.

All 43 compiler modules and all 12 complete focused groups pass, with no ignored
or filtered cases. All 119 runtime programs execute as expected with zero leaks.
The four 100,000-iteration plain/if/Boolean-case/literal-case loops each allocate
exactly 6 startup blocks, replacing the previous 200,006-block evidence loop.
All original object ABI/refusal checks and 21 Boolean/case source inputs pass.
See [the independently checked repair evidence](../bootstrap/current-case-dispatch-repair-results.json).

The new Cinch compiler identity is 018a2e3ca7546d60a2ba4c62b200b6a446ce0545d4956c2a3f9362cf6f581bf1.
Its fresh four-stage bootstrap and complete matching tests are the next gates;
the earlier 2ea bootstrap proof below does not prove this changed compiler.
The preceding checkpoint's 208-group Native sweep completed successfully,
while its self-built case-evidence failure motivated this repair. Full matching
Native, final Stage3 and auxiliary execution plus Acceptance 1-5 remain open.

## Rust inline refined arguments repaired

The Rust frontend now retains the actual checked result of an inline computed
argument under a unique strict value identity. Its erased predicate carries
write provenance. The function prefix evaluates first, and the argument
evaluates once; repeated effectful calls receive distinct identities. Builtin
borrow forms retain their expected-type checking path. The Native guarded-call
oracle now checks the original inline source without rewriting it into a let.

All97 tests in the six complete affected Native groups pass with no ignored or
filtered cases, including actual runtime order, fresh mutable evidence and
later-write refusal. The complete original guarded-call target passes too.
See [the scoped Rust repair evidence](../bootstrap/current-rust-inline-argument-results.json).
These results do not replace the full matching Native sweep.

The preceding Cinch frontend still has a confirmed allocation failure in its
original case-evidence target. A separate unpublished closed-method dispatch
and scalar case-fact repair is under twelve-group verification. Its first
object probe already reduces the proof-bearing if from200006 allocations to6;
the Boolean and literal case probes remain required. The complete bootstrap
and external results below describe their recorded Cinch compiler identity.

## Previous 2ea compiler bootstrap checkpoint (2026-11-04)

The current Cinch compiler (`2ea1460a5a1f426cf65b16767ed96e97dff1ffb2d3ee022b4ca1724bf2e6075c`)
now completes the fresh Stage0/1/2/3 chain. Every stage passes all three smoke
checks. Stage2 and Stage3 produce byte-identical compiler IR, executables and
all43 checked compiler interfaces. Stage3 consumes those actual emitted APIs
without compiler-module bodies. Stage2 also rebuilds changed compiler source,
and the resulting executable exhibits the changed diagnostic. The independently
checked [current bootstrap receipt](../bootstrap/current-opaque-witness-bootstrap-results.json)
proves Acceptance1-4 for this source.

All14 [exact final Stage3 external acceptance rows](../bootstrap/current-opaque-witness-stage3-external-results.json)
now pass: original runner reports and zero leaks, definition-owned fixities
through separate facades and renames, and complete class identity checks.
Acceptance5 remains open: the complete208-group Native sweep,91-group final
Stage3 suite and final100-target coverage audit must finish. The matching171 runtime,41 tooling and39 editor checks already
pass. A separately reproduced Rust inline-refined-argument bug has an
unpublished repair under regression testing; it does not change these43 Cinch
compiler sources. Neither the working self-build nor these scoped checks claim
that the full goal is complete.

## Latest follow-up: dependent arguments and Rust unfolding

The Rust optimizer now borrows a callee definition from its read-only table
instead of cloning the entire Core body before deciding whether to unfold it.
All16 tests in the complete optimizer target pass, including the fuel, effect
order, linearity and rewrite barriers; none were ignored or filtered. See
[the scoped Rust optimizer evidence](../bootstrap/current-rust-unfold-borrow-results.json).
This result does not claim that the complete compiler suite has passed.

The Cinch dependent-argument repair now passes all ten complete focused groups
on frozen source `96d9b6692f40a517c3b3172da6ee0cb39e0e05d4deaa442b79476998ce5465be`
(Cinch compiler identity `2ea1460a5a1f426cf65b16767ed96e97dff1ffb2d3ee022b4ca1724bf2e6075c`).
All 43 actual compiler modules pass Native checking. Complete source Pi,
guarded calls, source refinements/effects, runtime lowering, fragment checker,
optimizer, inference, Core, independent typing and typed-index targets pass,
with no ignored or filtered cases. All93 runtime programs execute with expected
results and zero leaked blocks. Seven new lexical-scope checks cover quantified
let RHSs, nested schemes, lambda patterns and rejected escapes across five
large runtime identities. See [the complete focused evidence](../bootstrap/current-dependent-argument-repair-results.json).

An ordinary checked computation can supply a dependent runtime argument through
a retained strict witness. Its identity does not become an unchecked symbolic
equation. A generic consumer is bound before its argument at an independent
scheme, then instantiated within the argument's private scopes; evaluation
remains function, argument, call. Local value quantifiers scope over their
own RHS only. Runtime locals and bound type indices keep separate namespaces,
and the independent Core checker remains authoritative.

Matching auxiliary checks also pass: all171 runtime tests,41 tooling checks
and39 syntax/editor checks, with no skips or ignored cases. See
[the current auxiliary evidence](../bootstrap/current-source-auxiliary-results.json).
The complete original argv target now passes all three cases through the
previous repaired Stage1 prototype (`39ad`): both portable builds use that
actual Cinch compiler, while CLI forwarding remains a Native API case. This
[scoped recovery](../bootstrap/current-stage1-argv-recovery.json) does not replace
final acceptance through the exact new Stage3.

A fresh Stage0/1/2/3 chain and complete208-group Native sweep now use the exact
current source. Stage0 has passed its three smokes and fresh Stage1 is building.
The other stages, all43 canonical interfaces and code fixed point, actual API
consumer, changed-source rebuilding, exact final Stage3's91 selected groups and
external checks, complete100-target coverage audit and Acceptance 1-5 remain
required. The original array ABI boundary and all original refusal assertions
remain in force. Passing focused tests is not a whole-bootstrap claim.

## Constructor and symbol repair checkpoint

This change publishes the escaped-constructor and symbol-word repairs from
frozen source `72a62361a93da731ea5fe5becb9dc9828db00dda3f9b87cfcb7caf67d3177dee`.
The normalized Cinch compiler identity is
`33f4dde5688c94aeb1f04dc6498a8ceb013ee51193dd471aca9c36f3c3b675e1`.
All 43 actual compiler modules pass Native source checking. The complete
runtime-lowering, checked-object, parser and checker targets pass, with no ignored or filtered
cases. All92 runtime programs execute with the expected results and zero leaks,
including65-tag families, recursive/first-class70-constructor families, generic
Raw/Obj/TObj layouts and a partially applied constructor retaining an owned
integer. The complete checked-object target also passes all22 positive programs
and every original ABI/admission refusal. See
[the scoped repair evidence](../bootstrap/escaped-constructor-symbol-repair-results.json).
This checkpoint does not claim that bootstrap or complete-suite acceptance has finished.

The backend uses the runtime's existing escaped-descriptor format and reads
escaped tags safely. Direct and first-class constructor allocation share that
format. Existing pointer-map and shared-table capacity guards remain. Primitive
`Sym` values use the Raw class required by checked library metadata. The legacy
checker now returns the remaining function type for a partially applied
constructor expression; over-application is refused, and constructor patterns
still require every field. New wrong-prefix-type and incomplete-pattern
refusals retain those boundaries. The original allocation assertions are unchanged.

The fresh Stage0/1/2/3 attempt uses this exact source. Stage0 and all three smoke
checks pass; fresh Stage1 and all three of its smoke checks pass too. Only unchanged pinned seed and admitted
library/runtime dependency artifacts are reused after revalidation. Every Cinch
compiler stage is freshly rebuilt. All four complete focused gates now pass,
unblocking Stage2 and the complete208-group Native sweep. Its full inventory has
compiled on the same source and private cache. Matching auxiliary checks all pass:
171 runtime tests,41 tooling checks and39 editor/syntax checks, none ignored or
skipped; see [the current auxiliary result](../bootstrap/current-source-auxiliary-results.json).
The [current coverage audit](../bootstrap/current-external-coverage-audit.json)
recomputes all100 harness targets and remains open until their actual matching
executions and final requirement audit complete. The eight original ordinary
behavior targets are now running early through fresh Stage1.
The complete original allocation target now passes all eight cases through this
fresh Stage1, including the two earlier failures. Its custom C-probe case remains
a direct Native API check; portable compiler builds select Stage1. See
[the scoped allocation recovery](../bootstrap/current-stage1-allocation-recovery.json).
The next complete early target, command-line arguments, finished with one pass
and two failures. The first diagnostic rejects a computed Nat passed to the
dependent result contract of `nat-to-int`; later diagnostics cascade. The
[original complete argv failure](../bootstrap/current-stage1-argv-failure.json)
records this exact Stage1 and log identity. A separate unpublished candidate
extends strict private witness binding to ordinary computed arguments. Its
source and full proof/effect/ownership gates are still under validation.
These diagnostics do not replace the required91-group exact final Stage3 suite:
79 bootstrap groups, eight ordinary targets, three complete mixed consumers and
the designed ABI5 array refusal. Exact final Stage3 runner reports, zero-leak
execution, definition-owned fixities and class behavior remain required too.
Stage2/3 IR, executable and all43 canonical interfaces must agree; actual API
consumption and rebuilding changed compiler source remain separate gates.
The [current acceptance audit](../bootstrap/current-bootstrap-acceptance-audit.json)
verifies Acceptance1 against the actual current Stage0/1 source and artifacts;
Acceptance2-5 remain open. The coverage audit also records complete passes for
all nine direct Native interface-API targets on this source; all91 selected
final-Stage3 groups and the complete execution audit remain required.

The earlier0e28 source successfully built Stage2 and passed its smokes, but its
original eight-case allocation target finished with six passes and two failures:
large constructor families and symbol-word classification. Its Native sweep
retained138 passing groups and deferred70 after the necessary source repair was
identified. Its unfinished Stage3 build and unstarted final-consumer queues were
stopped; all completed artifacts, logs and original failures remain recorded.
See [the complete original allocation result](../bootstrap/current-stage2-allocation-failure.json).
An intermediate repair's runtime target executed65-tag and generic70-tag
programs with zero leaks, then failed when the legacy checker rejected a bare
constructor as a function. The current complete runtime target passes that case
and the added owned partial-application case. Earlier results retain their
source scope and do not count as completion on the current source.

The [progress dashboard](https://cinch-bootstrap-status.ian92821.chatgpt.site)
tracks this current attempt, including each focused gate and remaining blockers.
Its percentage is a rough engineering estimate, not a test pass percentage.
The unchanged array boundary is specified in [the array design](arrays.md):
all14 Native ABI6 array tests must execute; the pinned ABI5 self-built compiler
must refuse the unimplemented operations. No Native array assertion is removed.

## Earlier repaired-source bootstrap and remaining gates

The earlier repaired-source Stage0/1/2/3 chain passes. Stage2 and Stage3 have
byte-identical compiler IR, executable bytes and all43 canonical compiler
interfaces. Stage3 checks its actual entry using those emitted interfaces with
compiler-module bodies absent. Stage2 rebuilds changed compiler source, and the
resulting executable produces the requested changed diagnostic behavior.
See [the complete fresh-chain evidence](../bootstrap/final-repaired-bootstrap-results.json).

The chain and complete suite27 use frozen source
`c6260e01e2fdab4397cd2d8087f835510fd80b4ad52d8729a6222a349b733703`;
the rendered compiler source identity is
`e85a5d0ad1875429759da1619b9c60cf86c72e5d6213f1d3dac2430ebd9fd9dd`.
This snapshot includes the reexport-fixity, Rust class/data-name and isolated
runtime-artifact repairs. Only the operator's declaring module may declare its
fixity; reexports carry that admitted fact without introducing declarations.

All127 ordinary compiler-facing target groups pass, including previously
ignored tests. The same snapshot passes all171 runtime tests,41 tooling checks
and39 syntax/editor tests, with none skipped or ignored. See
[ordinary target evidence](../bootstrap/ordinary-compiler-groups-results.json)
and [auxiliary gates](../bootstrap/final-source-auxiliary-results.json).
The81-group bootstrap-specific sweep continues as part of the complete208-group
compiler run. Exact final-Stage3 external acceptance now passes: both original
test-runner programs produce exact reports with zero leaks at32M, all seven
fixity/facade probes pass, and the portable class build/run cases return37 and43.
The class target's separate wrong-evidence case remains a Native API check.
See [exact final-executable evidence](../bootstrap/final-stage3-external-results.json).
The complete208-group sweep and final acceptance audit remain required.

The driver uses a separately pinned library-only producer and matching runtime,
records artifact/interface hashes, invokes the preceding Cinch compiler with
`--objects`, and links its actual emitted IR. The provider closure excludes
compiler implementation modules. Prepared ABI5 stages remain separate from live
ABI6 artifacts; the Stage0 seed is unchanged.

The [progress dashboard](https://cinch-bootstrap-status.ian92821.chatgpt.site)
reads the committed [status checkpoint](../bootstrap/progress.json). It follows
active verification results and publishes meaningful count, stage and failure
changes. Its estimated percentage is not a measured test pass rate.

## Self-built test coverage in progress

The portable harness now names Native library fixture production explicitly in
14 bootstrap targets. Check/build/run calls still select the configured Cinch
frontend and never fall back after a selected compiler failure. The fixture
helper rejects compiler implementation interfaces. All four harness checks pass;
see [the scoped result](../bootstrap/explicit-native-fixture-harness-results.json).
The complete79-group bootstrap run through final Stage3 is queued separately
from the complete208-group Native run.

The [external coverage inventory](../bootstrap/external-coverage-inventory.json)
also finds21 ordinary harness targets. Native interface serialization tests,
portable program behavior and mixed targets need explicit classification and
execution evidence. Eight ordinary portable targets are queued through final
Stage3; mixed consumer coverage and the final audit remain open. The pinned ABI5
bootstrap retains the array refusal boundary specified in [the array design](arrays.md).
This inventory is an open audit, not a passing result.

## Additional Stage3 behavior checks

The mixed consumer tests now explicitly name their Native library fixture stage;
the consumer builds and executions continue through the selected Cinch compiler.
All four runners compile on their frozen snapshot. The three complete mixed
consumer targets and the designed ABI5 array-refusal case now pass through the
same exact earlier Stage3 compiler. Their source, binary, object-provider and
log hashes are independently verified in [the scoped consumer result](../bootstrap/stage3-mixed-consumer-results.json).
These results apply to the earlier e85a5d0 compiler, not the newer0e28 candidate.

The first array-refusal consumer stopped because its test-only object bundle
lacked `Data.Option.is-none[O]`. The replacement bundle passes independent
artifact validation and contains that exact requested unit. All55 interface
documents match the previous bundle byte for byte. Its
[test-only roots](../bootstrap/test-support-object-roots.json) add only the
previously requested list-length and Option variants, plus the separately
requested raw-list factory described below; compiler roots, Stage0 and library
producer pins remain unchanged. See
[the scoped artifact result](../bootstrap/test-library-variant-results.json).
The exact array-refusal consumer rerun passes on the same Stage3 compiler and
unchanged test runner. The complete canonical-artifact, byte-observation and machine-contract consumer
targets also pass on that same unchanged Stage3 compiler.

The ordinary Stage3 allocation target also exposed missing checked I64 equality
and wrapping-arithmetic declarations, and rejection of pure `Unit -> Int`
entries. The first candidate passes its complete intrinsic gate, then fails the
runtime target at a function-valued entry: the legacy checker incorrectly
requires every signature argument to have a clause pattern. The next working
candidate checks a clause body against the function type remaining after its
actual patterns. Too many arguments, inconsistent clause arities and wrong body
types retain refusal checks. Its complete checker, intrinsic, runtime and parser
gates are tracked on the frozen current-source repair snapshot. The checker,
intrinsic, parser and complete runtime-lowering targets now pass, along with
all16 Rust optimizer tests. The Cinch implementation and its regressions are now
published; the fresh staged compiler build is running. The previously failing
function-valued entry, pure Unit entry, I64 overflow, first-class operation and
owned-data cases all return the expected results with zero leaks at32M. See
[the complete focused repair evidence](../bootstrap/current-compiler-repair-results.json). The earlier green bootstrap remains evidence for its original
e85a5d0 source.

The portable bootstrap sweep separately passes type-syntax and abstract NonZero
methods, then stops in the abstract NonZero type/Core target. Its retained
diagnostic requests the missing test-library factory `Data.List.FromList_List[R]`.
The library-only rebuild and independent artifact checks pass for that exact
variant. All55 admitted base interfaces, plus the trivial library root interface,
match the previous bundle byte for byte. The unchanged complete target now
passes through the same Stage3 compiler and runner, exercising all five nonce
sizes. The original failed sweep remains recorded; this isolated recovery does
not establish the complete79-group suite. See [the unchanged target recovery](../bootstrap/stage3-nonzero-types-recovery-results.json). See [the raw-list library artifact receipt](../bootstrap/test-library-list-variant-results.json). See [the retained failures and candidate scope](../bootstrap/additional-stage3-verification-results.json).

## Rust optimizer definition-table borrowing

A live profile of the complete checker target exposed repeated deep copies of
all function bodies: each per-function optimizer driver cloned the complete
definition table, although it only reads that table. The driver now borrows the
current table; the existing updates between optimized functions are preserved.
All16 optimizer tests and the complete exact-report checker test pass on the
frozen repair snapshot, with no failures, ignored tests or filtered cases. See
[the focused Rust result](../bootstrap/native-optimizer-borrow-results.json).
This is a Rust-only implementation checkpoint. The test snapshot also contains
the Cinch candidate whose complete intrinsic, parser and runtime-lowering
targets now also pass. Its fresh staged build and full suites remain required. Earlier full-suite and bootstrap evidence retains its
original source scope. The new Rust source requires the final full Native sweep.

## Earlier combined bootstrap checkpoint

The fresh pinned Stage0/1/2/3 chain on frozen source
`6c6644d69b3b32aa04992f6793de634278c07fe717bce59d1d0dc05d24cd3b69`
passes all stage smoke checks, exact Stage2/Stage3 compiler IR and executable
comparison, and all43 canonical compiler-interface comparisons. Stage3 checks
its actual entry through those emitted interfaces with compiler-module bodies
absent. Stage2 also rebuilds a changed compiler whose diagnostic behavior changes
as requested. See [complete chain evidence](../bootstrap/fresh-combined-bootstrap-results.json).

This establishes the combined bootstrap on that snapshot. It predates the later
reexport-fixity and Rust local-data/class-name repairs. The complete test sweep
has exposed a compression-library class-name collision; its original failure is
retained. Those repairs require independent validation and a matching final
bootstrap/test run before the full goal is complete.

## Rust local data and class-name collision

The complete frozen-source sweep exposed four compression-library failures:
its private data type `Bits` was incorrectly looked up as the legacy
`Algebra.Bits` signature. Ordinary lexical data lookup now precedes the semantic
class-key fallback. Qualified declaring-class identities remain distinct.
All three class-identity tests, four compression tests and seven module-parameter
tests pass on the repair snapshot. See [focused evidence](../bootstrap/native-class-data-collision-results.json).
The original full-sweep failure is retained; complete final-source verification
belongs to the new sweep.

## Isolated runtime export verification

A clean nondefault Cargo target directory exposed four test failures: export
checks searched `compiler/runtime/target` instead of the actual Cargo artifact
directory. A shared test-only helper now discovers runtime archives beside the
running test executable. The existing `nm` checks still require every expected
global text symbol. All171 runtime tests pass, with zero failures and zero
ignored tests, in a fresh isolated directory. See
[runtime evidence](../bootstrap/runtime-isolated-artifact-results.json).

The fresh passing stage chain and the running full suite capture this repair
together with the operator reexport and Rust class/data-name repairs. The two superseded waiting
runners had no test or stage children; their earlier results remain recorded.

## Definition-owned reexport fixities and imported value forwarding

The actual self-built compiler now preserves defining fixities through ordinary
and renamed reexports, including a facade compiled separately against checked
interfaces. Resolve records the same exported fixity facts used by Parse.
Interface production forwards an admitted imported value's complete defining row,
changing only its public spelling, so schemes, symbols, arity, nominal and
optional executable contracts survive separate compilation.

The actual rebuilt compiler emits and starts successfully, then passes all seven
CLI probes. Direct, renamed and separately compiled uses inherit precedence;
identical and changed importer declarations are both refused. Independent schema,
ABI and dependency-chain validation passes for the three actual artifacts.
Both complete initial and expanded writer/parser/resolver/fixity runs pass.
The matching repaired-source stage chain now passes. The complete suite27
and repeats through its exact final Stage3 executable are still running. See [before/after evidence](../bootstrap/reexport-fixity-results.json).

## Checked interface type printing

`Bootstrap.ExportType` now prints portable checked schemes with definition-owned
nominal identities and canonical binder names. The actual self-built Stage3
compiles and runs its exporter. Independent Native readback accepts the ordinary,
polymorphic, dependent, refined and nominal examples; the region/effect example
matches Native's own resolved interface production. Unsolved, escaped and malformed
identities and independent unresolved effect tails are refused. All12 report rows
pass the focused Native regression. This feeds the checked writer below;
unsupported contract forms retain explicit refusals. See
[checked type export evidence](../bootstrap/checked-type-export-results.json).

## Checked interface writer and complete seed closure

`Bootstrap.Export` produces canonical schema8 documents for ordinary checked
data and values, preserving source reexports, private defining nominal identities,
public constructor tags and fields, consumed dependency hashes and definition-owned
fixities. Native independently parses the documents, roundtrips their canonical
bytes and verifies their dependency chain. A real Cinch consumer accepts the public
reexports and refuses hidden constructor imports and importer fixity overrides.
The expanded writer and its staged CLI are described below. Contextual and
guarded instances and defining refinement/record contracts remain incomplete;
the full compiler contract gate is still open.

The compiler entry now imports every bundled module through a qualified alias.
The pinned seed actually builds this complete43-module source closure and emits
all43 compiler interfaces, including unused modules. The source identity in
[writer evidence](../bootstrap/checked-interface-writer-results.json) matches this
checked snapshot exactly. The later actual self-built compiler also emits all43
contracts; its complete readback and entry-consumer results are recorded below.
Seed-produced contracts do not substitute for self-produced compiler contracts.

The baseline full run exposed stale parser and pipeline test expectations. The
updated parser table exactly matches all109 measured corpus entries, including
zero unmodeled nodes in the new exporter modules. Four actual pipeline probes
confirm that parse, check, emit and manifest refusals truncate stale IR. The corrected complete
pipeline and parser targets pass on their named repair snapshots; historical
failures remain recorded. Final-source coverage belongs to the fresh full sweep.

## Source contracts reach the compiler CLI

The writer now exports checked signature parameters, method slots and public
aliases, unconditional instance heads and private method identities, and exact
checked primitive assumptions. Private effects and primitive names gain no public
source names. The actual self-built Stage3 compiles and executes the expanded
writer; a real Cinch consumer resolves an exported implicit instance.

`--emit-interfaces SOURCE PREFIX` writes each checked `PREFIX.Module.cinchif`
without IR. `--emit-ir-with-interfaces`, with the checked base/object configuration,
emits both contracts and runtime IR. The stage driver requests that combined mode
and records the output protocol in its cache inputs. The complete pipeline target
and real CLI probes pass on their named snapshots. Unsupported contract forms
remain explicit refusals. See
[advanced contract evidence](../bootstrap/advanced-interface-contract-results.json).

Module spans previously covered only the six-byte `module` keyword, making every
produced source hash identical. Spans now cover the complete module body; independent
FNV comparisons against the actual module slices pass. The full compiler contract
probe exposed a missing intrinsic-effect classification (`Diverge`), now repaired
and covered by the complete writer regression. Contextual/guarded instances,
source refinements and records are not claimed complete.

The full candidate23 sweep finished with201 target groups passing and4 failing:
parser coverage, pipeline refusal output, lattice transfers and refinements. The
first two corrected whole targets pass on fresh snapshots. The latter failures
exposed compile-time evaluation of runtime dictionary-self markers; that Rust
repair and the declaring-class identity correction now pass their scoped checks.
These repairs do not substitute for a complete final-source test run.

## Rust declaring-class and refinement evaluation repair

Semantic class selection now uses the declaring signature identity separately
from lexical names. Distinct `A.Foo` and `B.Foo` families select their own
instances, and evidence for one cannot satisfy the other. Legacy keys remain
limited to actual embedded prelude declaration sites.

Compile-time evaluation now interprets a callable dictionary-self projection
through the actual checked instance method and captured context. It no longer
looks for a fictitious instance ending in `#self`. Strict-value dictionary
environments retain their explicit unsupported boundary. All17 lattice and18
refinement tests pass, including the three baseline false accepts. The wider
repair run passes238 tests and records one obsolete parameter-name expectation;
all seven parameter tests pass with its defining identity corrected to
`Api.Stepper`. This is scoped evidence, not a complete final-source sweep.
See [repair results](../bootstrap/native-class-refinement-repair-results.json).

## Complete checked compiler-interface production

An actual Cinch-built compiler now emits all43 compiler interfaces from its
complete bundled source. Native independently parses and canonically roundtrips
all43 and validates their dependency chain and coherence together with55 base
interfaces. Every source hash independently matches its complete module slice.
This snapshot still records the reserved entry symbol as `main`; the corrected
`Main.main` defining identity passes the full writer regression separately.

Both writers now retain public effect operations instead of omitting/refusing
them. Schema8 keeps their actual checked display schemes, defining symbols and
arities with an explicit null resolved contract; this does not claim portable
handler metadata or permit an interface consumer to invent an ordinary function.
The interface/type/parser and16 effect tests pass on the corrected writer.

The final gate now checks canonical schema-shaped contracts, declaration and ABI
boundaries and the exact dependency hash chain. All41 tooling checks and55 real
base contracts pass. It also requires the self-built compiler to check its entry
using its emitted APIs with compiler-module bodies absent. That actual consumer
probe exposed missing private nominal kind facts when completing public
constructor layouts (`Text.Lex.Span`). That repair now passes the actual rebuilt
consumer and the complete focused regression below; the fresh compiler/interface
fixed point remains required. See
[production evidence](../bootstrap/compiler-interface-production-results.json).

## Real entry consumption and private field kinds

The rebuilt Cinch compiler now checks its actual entry against its emitted
compiler APIs with implementation bodies absent. The earlier check refused
`Text.Lex.Span`. A tiny independent public-Parcel/private-Secret reproduction
failed the same way with the previous self-built compiler.

Kind collection now retains defining private nominal facts from checked public
constructor schemes before completing public layouts. It adds no source bindings
and does not expose private constructors. The repaired actual Cinch compiler
accepts the public return and refuses private constructor import, private type
annotation and a wrong field carrier. The complete source-checker and expanded
writer/privacy, nominal identity, parser and region-contract targets also pass.
See [before/after and real consumer evidence](../bootstrap/private-constructor-kind-results.json).
The fresh complete stage chain and final-source test sweep remain required.

## Self-built test runner executes and reports exactly

The actual rebuilt Cinch compiler emits and links both bootstrap test-runner
programs. The mixed suite returns2 and reports exactly its two intentional
failures; the passing suite returns0 and reports zero failures. Both execute
with a32M heap and report zero leaks. The separate checked test-library bundle
adds the actually requested `Data.List.length[T]` variant using the existing
library-only producer pin; compiler stage roots and both pins are unchanged.
See [test-runner evidence](../bootstrap/self-built-test-runner-results.json)
and [the reproducible test root set](../bootstrap/test-runner-object-roots.json).

## Earlier checkpoints

The entries below retain the evidence and limitations of their named snapshots;
they do not supersede the current result above.

## Imported class authority and demanded library variants
Cinch retains checked imported class kinds and method descriptions even when a
module declares no instance. Kinds come from the defining parameter contracts;
closed portable method schemes are probed in private stores, without granting
source names or changing the module's inference state. Given-only, renamed and
higher-kind imported-class cases pass alongside the existing source instances.
All82 backend executables still report zero leaks, and the instance-resolution,
nominal-identity and global-authority regression targets pass. Cinch accepts all25
updated backend modules with zero diagnostics.

The Rust object producer accepts explicit function, dictionary and method-vector
roots. It collects requested variants and their dependencies, rejects incomplete
representation demands, and records its actual class method slot families.
Runtime method sites and factories identify their declaring class canonically.
A separately linked probe checks two instances sharing Raw/TObj method slots,
escaping partial methods and demanded generic functions, with zero leaks.
All147 unit,6 class-default,16 effect,12 module,3 dictionary and3 representation
checks pass, plus7 module-parameter,17 surface and2 interface-contract checks
(213 total); every Rust test target compiles and the actual rooted
object CLI passes. The five object tests are included in the unit count.

The fresh genuine stage chain passes27 tooling and all six Stage0/1 checks.
Stage2 accepts the full compiler frontend and clears every missing-declaring-class
refusal, then stops at missing executable library functions/factories and other
runtime lowering. Its7127 diagnostics include cascades; IR is empty and no Stage2
compiler exists. Cinch manifest consumption, shared layout seeding, checked
external calls, private String/scalar lowering, full suites/correctness and
Stage2/3 remain required. The seed is unchanged. See
[the combined evidence](../bootstrap/runtime-imported-class-results.json) and
[the object linkage protocol](runtime-object-linkage.md).


## Executable library object producer and indexed ownership repair
The Rust compiler now emits closed library dependency objects with canonical
physical manifests, checked interface closures and explicit closure accessors.
Objects have no process entry or independent layout registration. A linked
consumer installs the exact recorded table prefix and appends its own layouts;
actual Raw/Obj/TObj fields, escaped method closures, dictionaries, private String
wrappers and heap integers in indexed data cross this boundary with zero leaks.
Extended pointer-map words and invalid object boundaries are checked separately.
The object probe caught missing built-in constructor maps and a Rust indexed-data
bug: erased numeric indices shifted the element's representation slot. Both
built-in and source constructors now use their actual scheme type-parameter order.
All146 unit,14 dependent,10 incremental,12 module,21 ownership,18 reclamation
and3 representation checks pass (224 total), and the real emit-object CLI passes.
This is the producer boundary. Cinch manifest consumption, shared layout seeding,
checked external calls/variants and class method families remain to be integrated.
No Stage2 compiler or Stage3 fixed point is established. The seed is unchanged.
See [the object linkage contract](runtime-object-linkage.md) and
[the executable object evidence](../bootstrap/runtime-object-results.json).


## Source signatures and polymorphic dictionary methods
Cinch now checks source-declared signature kinds, method schemes, self evidence,
module exports and declaration-order slots. The resolved method node retains its
actual global binding separately from the field label. Method-local type
parameters use canonical Raw/Obj/TObj slot families shared by every dictionary
of the declaring class; reached vectors are planned before final emission.
All82 executable programs report zero leaks, including escaping polymorphic
methods and generic factory context. Five invalid signature cases remain
rejected. All nine new source programs also run through Rust with zero leaks.
Cinch accepts all25 updated backend modules with zero diagnostics. The real CLI,
source-instance contracts and instance-resolution regression targets pass. The
latter caught an earlier head-selection regression: malformed factory contexts
now stay out of ambiguity selection without committing failed trial substitutions.
Copy and Share remain compiler-owned; their redeclarations are rejected.
Source defaults and laws, base-family scope, executable library bodies, private String/scalar/layout
lowering, full current suites/correctness and Stage2/3 remain required. The last
completed genuine full-source attempt passes the frontend and traverses beyond
read-argv, then emits only runtime refusals with empty IR and no Stage2 compiler.
Its diagnostics include cascades and do not count remaining features. See
[the source-signature evidence](../bootstrap/runtime-source-signature-results.json).


## Rust same-file module dependency repair
The Rust source loader now preserves dependency order when one file declares
several modules. It previously sorted the graph, then moved the entry file's
modules back into declaration order, breaking an import of a signature declared
later in that file. The new regression compiles and runs that program, returns213,
and checks the imported module's presence in the interface. All12 module,
10 incremental and142 unit tests pass; all seven source-signature reference
programs compile and execute with zero leaks. The seed remains unchanged. See
[the module ordering evidence](../bootstrap/native-module-order-results.json).


## Imported public constructor layouts
The Cinch frontend now retains runtime data declarations from complete checked
imported constructor tables. The backend uses the defining constructor order,
actual tags, resolved schemes and field representations. Abstract carriers remain
owned objects; a partial public table never invents hidden constructor layouts.
All73 executable programs report zero leaks, including six new imported cases
covering Raw/Obj/TObj fields, nullary tags, declaration order and constructor
closures. The hidden-constructor probe refuses emission with no IR. The real CLI,
nominal identity and constructor/evidence regression targets pass. Cinch accepts
all25 backend source modules with zero diagnostics. The fresh full stage chain
is recorded separately; no Stage2 executable or fixed point is established.
The actual25-module String source probe identified missing source-signature
namespace elaboration as another prerequisite for compiling base implementations.
That source-signature work is still private and under validation. See
[the imported layout evidence](../bootstrap/runtime-imported-layout-results.json).


## Cinch dictionary and IO checkpoint
The typed Cinch backend now executes dictionary values, factory contexts,
self-method dispatch, escaped evidence and strict values shared between fields
and escaping closures. The physical ABI probe checks actual dictionary slot
order, Raw/Obj/TObj maps, shared Box pointers and lifetimes after dictionary
destruction. Erased region parameters and Unit IO entries execute; sleep,
cancellation and observable byte writes use the shared runtime. All67 executable
programs report zero leaks. Cinch accepts all25 backend source modules with zero
diagnostics; the real CLI,27 tooling checks and six fresh stage0/1 smokes pass.
Instance search now chooses matching heads before solving their contexts,
avoiding exponential work while retaining ambiguity and numeric defaulting rules.
The genuine full-source Stage2 attempt accepts the compiler frontend, then
refuses executable base/library implementations and imported layouts. Its IR is
empty and no Stage2 compiler exists; diagnostics do not count remaining features.
Private String representation, complete current suites/correctness gates and the
Stage2/3 fixed point remain required. The pinned seed stays unchanged. See
[the dictionary/IO evidence](../bootstrap/runtime-dictionary-results.json).


## Rust strict dictionary value checkpoint
The Rust compiler now initializes strict dictionary fields once in dependency
order. Self projections use the initialized value, and methods and escaped
closures capture that actual value. A source program that previously exhausted
a32M heap now runs successfully. A physical ABI probe verifies shared Box
pointers and an escaping method retaining the Box after its containing dictionary
is dropped. All21 executable dictionary cases report zero leaks. The142 unit,
21 ownership,18 reclamation,16 effect,3 representation,6 class-default,17 surface,
10 incremental,11 module and one class-contract tests pass. A supplemental
incremental executable check runs cold197, unchanged197 and edited199 with zero
leaks; unchanged IR remains byte-identical. Recursive strict initialization,
full strict-self evidence and method-local polymorphic value environments still
receive named refusals. The pinned seed is unchanged; full current-suite and
Stage2/3 success remain unverified. See
[the Rust strict-value evidence](../bootstrap/native-dictionary-values-results.json).


## Rust dictionary ownership and scope checkpoint
The Rust compiler now records the checked result word class and preserves it
through representation specialization. Dictionary maps count actual eager
value fields: large integers, objects and function values are no longer
unconditionally marked Raw. Named source instances can supply ordinary
signature constraints through `with`, including exported/import-renamed names;
IO supplies retain their runtime-region escape checks. Explicit factory
refinement obligations still receive a named refusal.
All18 executable dictionary/IO cases report zero leaks, and five invalid or
out-of-scope uses stay rejected. The focused unit, ownership, reclamation,
effect, representation, class-default and surface targets all pass. The seed
is unchanged. Cinch dictionary/IO lowering is still under validation; no
Stage2 or complete current-suite pass is established here. See
[the Rust dictionary evidence](../bootstrap/native-dictionary-check-results.json).


The typed backend now lowers real ABI5 closures, lexical captures, partial
functions/constructors, indirect calls and overapplication. All51 executable
programs pass with zero leaks, including generic captures and higher-order
recursive mapping. The real CLI and a C probe verify actual closure headers,
field maps, shared partial copies and bulk application. Cinch accepts the entire
25-module backend source closure with zero diagnostics. All27 tooling and six
fresh stage0/1 checks pass. The genuine Stage2 attempt accepts the full updated
compiler source, then refuses the erased-region/IO/dictionary boundary. IR is
cleared and no Stage2 executable exists; traversal stops at main, so its three
diagnostics are not a count of remaining features.
Erased/dependent parameters, IO/effects, dictionaries, executable base code,
extended layouts and complete correctness/current-suite/fixed-point gates remain
required. See [the closure checkpoint](../bootstrap/runtime-closure-results.json).

## Native checker checkpoint
The development Rust checker now accepts equations returning functions and
structural lambda patterns through ordinary result checking and the existing
exhaustive pattern compiler. All18 new executable examples report zero leaks;
four invalid cases stay rejected. The142 unit,21 ownership,18 reclamation and17
surface tests pass. This leaves the pinned stage0 unchanged. See
[the Rust closure checker repair](../bootstrap/native-closure-check-results.json).

## Current Cinch backend checkpoint
The typed backend now shares polymorphic workers by Raw/Obj/TObj class vectors
and specializes regular source data layouts by those vectors. All33 executable
programs pass with zero leaks. Actual worker-count checks prove sharing across
Int/Nat and distinct heap types, and finite type-growing recursion; a C probe
checks the real generic field maps. The complete inference and emission CLI gates
pass, including delayed numeric equality and literal-True guard coverage repairs.
Fresh27 tooling and six stage0/1 checks pass. The genuine Stage2 attempt accepts
the complete updated compiler source, then refuses the IO entry, erased parameters
and dictionary/effect boundary. Its IR is empty and no Stage2 compiler exists.
Demand traversal stops at that entry; the three diagnostics are not a count of
remaining features. Closures, partial calls, dictionaries, effects,
executable base implementations, higher/dependent representations and complete
correctness/current-suite gates remain required. See
[the class-specialization checkpoint](../bootstrap/runtime-class-results.json).

## Previous source-data checkpoint
Typed runtime lowering now executes monomorphic source data, recursive constructors,
refutable and nested patterns, case guards and multiple function equations. All23
programs pass with zero leaks; a C probe checks actual constructor tags and aliases.
The complete inference, emission CLI and pipeline targets pass, as do27 tooling
checks, six fresh stage0/1 smokes and the updated compiler checking its full source
with zero diagnostics. Constructor literals now wait for their enclosing type
before carrier/range validation. The genuine Stage2 attempt exits5 with zero
frontend diagnostics and5050 named runtime refusals; its IR is empty and no compiler
is built. The refusal count includes cascades and is not a count of missing features.
Polymorphic representation specialization, closures, dictionaries, effects, executable base ABI,
complete correctness/current-suite gates and the Stage2/3 fixed point remain
required. See [the ADT checkpoint](../bootstrap/runtime-adt-results.json).

## Previous typed stage checkpoint
The stage runner now selects the actual typed backend: each later stage invokes
its preceding Cinch compiler with `--emit-ir`, then the platform linker links
the runtime built from the unchanged pinned seed. Protocol3 records that runtime
archive hash in cache identity and checks the resulting compiler's startup.
All27 tooling checks, six fresh stage0/1 smokes and a real numeric linkage probe
pass. The genuine full compiler attempt reaches only typed runtime refusals:
exit5, zero frontend diagnostics and2830 emitter diagnostics. There is no Stage2
executable, usable IR or Stage3 fixed point; the IR file is cleared to zero bytes.
This staged snapshot precedes the tuple checkpoint below. Source ADTs,
polymorphism, closures, dictionaries, effects, foreign/executable base ABI and
complete correctness/current-suite gates remain required. See
[the typed stage result](../bootstrap/typed-stage-driver-results.json).

## Previous tuple checkpoint

Typed runtime lowering now includes tuples, nested tuple fields and irrefutable
parameter/case patterns. One ABI5 registry records actual Raw/Obj/TObj field maps;
borrowed fields remain protected by owning roots, and scope exits reclaim them.
All13 executable cases pass with zero leaks, including a full16-field inline
map. A17-field layout is refused explicitly. Cinch accepts the backend's entire
25-module source closure with zero diagnostics. Source ADTs, refutable matches,
polymorphism, closures, dictionaries, effects, foreign/base ABI lowering and the
complete correctness/test/fixed-point gates remain unfinished. The new typed
stage runner is separately under validation. See
[the tuple checkpoint](../bootstrap/runtime-tuple-results.json).

## Previous typed compiler CLI checkpoint

The real compiler CLI now exposes `--emit-ir [--interfaces MANIFEST] SOURCE IR`
through the retained-Core runtime backend. It writes LLVM without linking or
running. A real emitted bignum binary returns67 and reports zero leaks; both
interface flag orders, unsupported lowering and stale-IR clearing pass. The
complete existing pipeline regression also passes. Cinch accepts the entire
updated compiler source bundle with zero diagnostics. The stage runner still
needs to select typed emission and link the matching runtime for later stages;
there is no Stage2 executable or fixed point yet. See
[the emit-mode checkpoint](../bootstrap/runtime-emit-results.json).

## Previous shared-runtime checkpoint

Typed Core now has an executable shared-runtime scalar backend in
`Bootstrap.RuntimeIr`. Eight runtime-fed programs emit real LLVM, link to ABI5,
return exact results and report zero leaks. They cover numbers above2^128,
aliases, direct calls, branches, recursion, negatives, Naturals and discarded
arguments. The corrected whole Rust target passes; Cinch independently accepts
the backend's entire source dependency closure with zero diagnostics. Unsupported
polymorphism is refused before IR is returned. The real `--emit-ir` CLI route is
under separate validation; it is not yet part of this checkpoint.
See [the runtime backend evidence](../bootstrap/runtime-ir-results.json).

The retained-Core pipeline also passes fresh six stage0/1 smokes and a genuine
Stage2 attempt accepts the complete compiler source, stopping only at the old
emitter's named refusals. That stage snapshot precedes the new backend module.
There is still no Stage2 executable or Stage3 fixed point. Aggregate, closure,
dictionary/effect/foreign lowering, the whole-program base/layout ABI, complete
correctness checks and complete Native/self-built suites remain required.
See [the retained pipeline stage result](../bootstrap/checked-program-stage2-results.json).

## Previous retained-Core checkpoint

The pipeline now retains typed Core instead of discarding it after checking.
A private `CheckedProgram` constructor associates the resolved source and its
checked elaboration; the emitter receives that value. Runtime-input tests retain
polymorphic definitions and data, refuse an actual type error, and reject an
outside attempt to forge the constructor. Both complete scalar IR cases and the
whole pipeline regression pass. The existing scalar renderer still uses the
associated source; typed runtime lowering remains unfinished. The exact earlier
Stage2 acceptance below belongs to the arity snapshot, before this handoff change.
See [the retained-Core report](../bootstrap/checked-program-results.json).

## Previous full-source acceptance checkpoint

The genuine Stage2 attempt now accepts the complete compiler source through
the Cinch frontend. All6925 diagnostics are named bootstrap.ir refusals;
there are zero frontend diagnostics. Stage2 exits5 and emits neither LLVM
nor an executable. The compiled source bundle matches the published compiler
sources exactly. All six fresh stage0/1 smokes pass. Shared-runtime typed
lowering, complete correctness checks, self-built tests and the Stage2/3 fixed
point remain required. See [the exact stage result](../bootstrap/arity-stage2-results.json).

## Previous arity checkpoint

Both imported arity checks now bind the actual computed Nat before conversion:
the inference checker binds scheme arity plus dictionary requirements, and the
legacy checker binds its type arity. The complete23-module inference and24-module
checker closures pass through Cinch, reaching only the emitter's named runtime
layout refusals. Both imported-precondition and global-checking regressions pass.
Fresh stage0/1 pass all six smokes. The complete new Stage2 attempt is running;
no Stage2 compiler or IR is claimed. The preceding inference-only repair reduced
the actual attempt to20 diagnostics, first at the now-repaired legacy arity check.
See [the scoped arity checkpoint](../bootstrap/arity-binding-results.json).

## Previous external frontend checkpoint

The shared test helpers can now explicitly select a compiled Cinch frontend
for check, build and run. Selection errors and compiler failures never fall
back to Rust. A real Stage1 compiler passes all three modes on owned two-module
source, a separately executed binary returns37, and invalid input is refused.
Four invocation receipts retain the actual inputs and commands. Both adapter
control tests and the complete default-Native source-path and pipeline targets
pass. EmitIr and tests calling Native APIs directly still require work; this is
not a complete self-built suite. See [the adapter report](../bootstrap/external-frontend-results.json).

The older frozen handler/parser snapshot has now completed all191 integration
targets and six general gates successfully, with unchanged inputs. It predates
the later argument-witness, JSON, interface, resolver, Boolean and CLI changes;
it does not prove a complete latest-source pass or self-hosting. See
[the scoped full report](../bootstrap/handler-parser-full-suite-results.json).

## Previous Boolean checkpoint

Boolean branch facts now preserve valid one-way implications when an opaque
right operand prevents an exact condition reading. A true conjunction retains
its left guard; a false disjunction retains the negative left guard. The wrong
converses remain rejected. Source elaboration, independent Core typing and
proof/certificate checking share the polarity-aware rule. All three focused
targets pass, including six source cases and existing Core checks; the larger
guarded-call and source-effect regressions are still running.

The combined driver/checker candidate passes all six stage0/1 smokes. Actual
stage2 clears Bootstrap.Guarded, then exits4 with446 diagnostics, first at
Bootstrap.Infer.check-imported-value's inline arity sum passed to nat-to-int.
Most later errors may be cascades. No stage2 IR or compiler exists yet. See
[the Boolean branch checkpoint](../bootstrap/boolean-branch-results.json).

## Previous driver and resolver checkpoints

The Cinch driver now has explicit check-only and compile-only modes. Checking
stops before emission and clears stale diagnostics; compilation links without
running its output. The original compile/link/run interface is preserved. The
whole pipeline target passes, including real binaries, imported checks, invalid
options and an execution spy, and fresh stage0/1 pass all six smokes. These modes
prepare external frontend testing; they do not establish a complete self-built
suite or complete correctness checking. See [the mode checkpoint](../bootstrap/frontend-modes-results.json).

The last genuine stage2 result below is unchanged: the Boolean guard repair is
separately under validation. Typed runtime lowering and the fixed point remain
unfinished.

## Previous resolver checkpoint

Resolver source metadata now binds its actual span and offsets before numeric
conversion. Its complete15-module source closure passes checking, and both whole
resolver and pipeline regressions pass. Fresh stage0/1 pass all six smokes.
Actualstage2 clears the resolver, then exits4 with872 diagnostics, first at
Bootstrap.Guarded.read-expression's compound slot guard. Most later diagnostics
may be cascades; no stage2 IR or compiler is produced. The next checker repair
must preserve only the logically valid implication for each branch. See
[the resolver checkpoint](../bootstrap/resolver-span-results.json).

## Previous interface checkpoint

The interface reader now binds the actual checked suffix length inside its
original splice guard, then uses that value for allocation and copying. The
whole module passes source checking; the complete interface hash/corruption
and loader dependency regressions pass. Fresh stage0/1 pass all six smokes.
The next actual stage2 attempt is running; no stage2 IR or compiler is claimed.
All four intended argument-witness regressions also pass. See
[the interface splice checkpoint](../bootstrap/interface-splice-results.json).

The preceding JSON candidate cleared Text.Json and stopped at this splice
calculation, with1196 stage2 diagnostics; later errors may be cascades. Source
acceptance remains distinct from typed runtime lowering. Full correctness
checks, shared-runtime code generation, current-source and self-built tests,
and the stage2/3 fixed point remain required.

## Earlier implementation checkpoints

The development Rust compiler now retains checked byte-length equations for
strict aliases whose actual carrier is refined Nat, including Size. It preserves
lexical value identities and actual primitive/body authority. All four whole
guarded-let, primitive-length, safe-byte and refinement targets pass, including
three new positive and five negative source cases. The c774 producer is unchanged.
See [the Native alias result](../bootstrap/native-length-alias-results.json).

Checked dependent arguments now keep their actual strict witness bindings in
scope through application, with the original function evaluated first. Hoisting
a witnessed value out of let generalization adjusts only that value's inference
depth; unrelated value scopes and type skolems retain their escape checks.
All seven focused source cases, independent Core checks, and the whole indices
and guarded-call regressions pass. The whole refinement-constructor regression
also passes; the correctly named dependent-function regression remains queued.

Text.Json now binds its actual cursor offset and buffer before the existing
byte-access guards. The complete module passes source checking; all nine JSON
runtime tests and the whole parser corpus pass. Fresh stage0/1 pass all six
smokes. The new actual stage2 attempt is running; no stage2 compiler or IR is
claimed. Typed runtime lowering, full usage/totality checks and the stage2/3
fixed point remain required. See [the scoped checkpoint](../bootstrap/argument-witness-results.json).

## Previous parser, handler and producer checkpoint

The parser span helper now binds its endpoint before the existing bounds
check and its start inside that guarded branch. The whole parser passes
source checking; its corpus and complete pipeline regression pass. Valid
handlers reach the emitter's named refusal, while erased clause effects
still fail checking. The deliberately updated c774 Rust seed reproduces
its tested sources exactly and passes180 checks. Fresh stage0/1 pass all
six smokes. Actualstage2 clears the parser and exits4 with1267 diagnostics,
first at Text.Json.digest-hex-into's nested remainder argument. No stage2
compiler or IR is produced. Argument witness scopes are the next work.

Verification now captures the merged syntax experiments and runs them in
an external copy with the frozen Native compiler. All23 tooling and39
syntax tests pass, with no skips or frozen-input changes. A complete191
integration plus six-general-gate Native-assisted matrix is running on
that exact snapshot. This does not establish a self-built frontend pass.
See [the scoped evidence](../bootstrap/parser-handler-seed-results.json).


Cinch now elaborates source handlers to retained, independently checked Core.
All four focused targets pass:34 Core checks at five large identities,26
source handler cases,20 source effect cases and the existing typing suite.
The immediately preceding staged snapshot passes six smokes and clears the
Text.Parse recovery handler; stage2 then exits4 at Bootstrap.Parse.slice-src,
where an inline span projection needs a stable dependent argument. The final
Core correction also retains effects while constructing handler fields.
No current stage2 compiler or full self-built suite pass is claimed. Typed
runtime lowering, full usage/totality checks and the stage2/3 fixed point remain
required. See [the exact scope](../bootstrap/handler-core-results.json) and
[the handler implementation boundary](bootstrap-handlers.md).


The development Rust checker now rejects duplicate handler operation clauses
and checks operation type parameters universally, with clause-local scope
escape checks. The complete16-test effects target passes, including runtime
execution of a valid polymorphic abort. The pinned b16 producer remains
unchanged pending separate producer validation. Cinch handler elaboration
and independent Core checking are under validation; no new stage2 compiler
is claimed. See [the Native handler results](../bootstrap/native-handler-results.json).


Checked refined runtime results now retain their full contracts when used as
dependent arguments. The original function and value are each bound once;
only the bound value is projected, preserving effects and call obligations.
All three focused targets pass, including49 source cases and40 Core checks.
Fresh stage0/1 pass all six smokes. Actual stage2 clears Text.Parse.line-text-at
and exits4 with1422 diagnostics, first at the handler in run-parser-with.
Handler elaboration, complete correctness checks, typed runtime lowering and
the stage2/3 fixed point remain required. The188+5 complete baseline below
belongs to an earlier source snapshot. See [the scoped result](../bootstrap/refined-argument-results.json).

## Current verification boundary

The complete188 integration targets and five general gates now pass on the
8ea856a snapshot. Five focused whole targets were reused only from the same
exact frozen inputs;188 gates ran fresh. This is the latest complete baseline,
preceding the later lexer bindings, Native ownership repair/b16 producer and
the refined-argument materialization candidate. It does not establish a complete
current-source or self-built frontend pass. See [the full baseline report](../bootstrap/hypothesis-full-suite-results.json).

Rust now retains the checked collection type when reifying constant lists and
their constructors. Counted object and tagged bignum heads keep their actual
class vectors; unresolved generalized result types retain typed runtime
construction. Both leak-debug regressions pass, including direct small/bignum
integer heads. The modern Native compiler passes182 focused checks. Separately,
the previous f88 producer with only this repair passes180 checks; archive plus
patch reproduces those exact sources as the intentionally pinned b16efd1 seed.
ABI5/schema8 remain unchanged, and ABI6 arrays remain isolated. A new stage
chain is required for this producer. See [the ownership report](../bootstrap/native-fold-ownership-results.json).

The complete Text.Lex source now passes the independently compiled Cinch
checker. It reaches only the emitter's explicit runtime-layout refusals.
Attribute-name validation binds its string length once; attribute trimming
binds its immutable Scan source and offset once before checking the existing
buffer bound. These are actual checked values, requiring no invented symbolic
contract for string-length or record projections. Runtime probes retain the
missing-nonempty-guard refusal. A new stage chain will use these implementation
inputs; the188+5 sweep below remains scoped to the preceding8ea856a snapshot.
See [the whole-lexer report](../bootstrap/lexer-bound-values-results.json).

Cinch now derives a checked total call's numeric bounds when that call appears
only in an existing hypothesis. For example, `pos + width cp <= bytes-len out`
can imply `pos <= bytes-len out` using the actual checked Nat-returning body of
`width`. Bounds still require the matching defining body; no source spelling
or stale body supplies authority. Text.Lex binds `char-code c` once, then uses
that checked Nat value for both buffer sizing and encoding.

The regression fails before the proof fix and passes afterward, including
stronger-result, missing-body and changed-body refusals at five identities
above2^128. All37 source cases,40 guarded-call Core checks and five whole
regression targets pass. Fresh stage0/1 pass all six smokes. Actualstage2 clears
the character-buffer bound and reaches `Text.Lex.attr-name-valid`, whose repeated
`string-length name` call has no checked symbolic index contract. It exits4 with
1649 diagnostics and produces no compiler. See [the actual stage report](../bootstrap/hypothesis-body-bounds-stages-results.json).
The full188+5 current-source matrix is running. The pinned f88 producer is
unchanged: Text.Lex is compiled toolkit source, outside its embedded prelude.
See [the hypothesis-bound report](../bootstrap/hypothesis-body-bounds-results.json).

The guarded742 snapshot completed all187 integration targets and five general
gates:176 fresh gates and16 whole gates reused only after checking identical
relevant inputs. This is a complete development-compiler baseline, preceding
the later f88 byte-let fix, curried-let fix and syntax/tactic integration.
See [the guarded full-suite report](../bootstrap/guarded-full-suite-results.json).

The f88 stage chain with the curried-let fix passes all six stage0/1 smokes.
Its actual stage2 clears Text.Builder's byte-copy bound, then exits4 with1709
diagnostics. The first remaining error is Text.Lex's repeated character-code
conversion inside a dependent Nat index. No stage2 compiler, stage3 fixed point
or self-built frontend suite exists yet. See [the stage report](../bootstrap/guarded-stages-results.json).

The reconciled syntax/tactic integration separately passes273 Native tests,
39 experiment tests and19 bootstrap-tooling tests. Its bootstrap sources and
f88 seed pin are unchanged. Neither that focused result nor the earlier full
baseline establishes a complete suite pass for the combined latest sources.
Historical checkpoints below retain their original validation boundaries.

## Malleable syntax experiment

The branch also carries the isolated [syntax and tactic prototypes](../experiments/malleable-syntax/INTEGRATION.md):
declarative grammar islands, HTML with interleaved Cinch, generated editor
grammars/source maps, choreography projection, tensor pullbacks and finite
goal-aware tactic plans. Strict aliases preserve both multiplicity and checked
value/equation provenance. Tactic goals use the native checker's current value
scope. These are development Rust compiler/front-end experiments; general
reflected tactics and equivalent self-hosted support remain planned. The pinned
stage0 seed and bootstrap source are unchanged by this integration.

## Bootstrap progress

Cinch now checks later curried arguments inside the strict scopes created
for earlier dependent arguments. Their checked equations remain available to
the final call guard. Hidden environment names preserve the caller scope of
source arguments when a function expression itself contains a shadowing let.

The compiled Cinch checker passes35 runtime-fed source cases, including the
previously failing inline byte-copy bounds, explicit offsets, refined buffers,
wrong offsets/missing guards and argument-capture regressions. The40 call,
50 body/declaration/stale-alias and50 admission checks also pass. All142 Native
units and12 whole Native targets pass on the same Native implementation.
A fresh f88 staged attempt completed as recorded above; the187+5 sweep passed
on the preceding742 snapshot. No genuine bootstrap or full current-source suite pass
is claimed. See [the curried-let scope report](../bootstrap/curried-let-scope-results.json).


Rust now retains byte-offset equations through strict Nat lets by inspecting
the actual typed byte-length operation and selected checked arithmetic bodies.
Wrong offsets, missing guards, changed bodies and unknown providers remain
refused. All142 Native units and12 whole regressions pass, including the five
expanded guarded-let tests. The f88d6a4 seed intentionally reproduces those
exact sources; archive plus patch is verified. ABI5/schema8 remain unchanged.
See [the Rust byte-let report](../bootstrap/native-byte-let-results.json).

The preceding742 stage run passes all six smokes but actualstage2 exits4
with1713 body diagnostics, first at Text.Builder's inline byte-copy guard;
it produces noIR/compiler. Runtime probes accept the same byte equation and
an explicit let in Cinch, locating the remaining gap in later applications
escaping the internal dependent-argument let scopes. A private scope repair
and35-source-case target are under validation. The ongoing187+5 suite is
scoped to742, and the completed182+5 baseline to c126. Genuine self-hosting
remains incomplete.


Cinch now uses checked conditional Nat bodies to retain actual result
equations. Body admission rechecks totality under the complete callable
preconditions. Core independently associates each equation with the actual
external declaration and rechecks the call evidence. A symbolic Nat result is
bound once with a mathematical equation; unknown subtraction is still not an
unconditional type-level index. No class/name/slot equation rule was added.

The compiled Cinch checkers pass25 runtime-fed source cases,40 call-evidence
checks,50 body/declaration/stale-alias checks and50 body-admission checks. The
new whole parser-corpus and global-declaration targets pass. All142 Native
units and12 whole Native regressions pass on the same implementation bytes.
The187-integration plus five-general-gate suite is running on the742 seed.
All six fresh stage0/1 smokes pass; the actual stage2 attempt is running.
Genuine self-hosting remains incomplete. See
[the guarded-body consumer report](../bootstrap/guarded-body-consumer-results.json).


Rust now retains strict Nat let equations from actual checked bodies and
keeps their original values across aliases and numeric result-contract
shadowing. Unknown dictionaries grant no equation; a result fact cannot
supply its own earlier call guard. Ordinary evidence preserves opaque measure
expressions and mutable invalidation. The linear solver now retains negated
multi-term equality goals before pruning facts, and still checks its generated
certificates independently. All142 Native units and12 whole regression targets
pass, including four new guarded-let tests. The74239b7 seed intentionally
reproduces those exact sources through archive plus patch. ABI5/schema8 remain
unchanged. See [the Rust guarded-let report](../bootstrap/native-guarded-let-results.json).

The latest Cinch body-admission and body-association targets each pass50
independent checks. The source-level guarded-call target is still running;
these private changes are not part of this Rust checkpoint. The complete
182+5 result below remains scoped to the older c126 conditional baseline.
Genuine self-hosting is still incomplete.


The complete conditional compiler baseline now passes all182 integration
targets and five general gates. Each continuation audits the frozen inputs and
reuses only unaffected whole gates. The module-instantiation fixture now keeps
its exported body constant. The total-body cache fixture checks both unchanged
reuse and retyping when an imported proof-relevant body changes; it still
checks current transitive dependencies and loss of cyclic authority. Both
corrected whole fixtures also pass against the newer published641613b seed.
See [the complete conditional baseline report](../bootstrap/conditional-full-suite-results.json).

This full result belongs to the conditional baseline with the c126 seed,
not the newer private guarded-body/Rust scope and arithmetic fixes. Their
fresh validation is running. No self-built frontend or genuine bootstrap pass
is claimed; handlers, complete independent proof/usage/totality, typed runtime
lowering and the stage2/stage3 fixed point remain required.


Rust now exports a bounded, optional Nat body tree for closed conditional
methods from the actual checked implementation and primitive identity. Changing
the body with the same class, slot and scheme changes its interface hash.
Out-of-scope arguments and over-depth records are refused. All 141 Native unit
tests and the whole class, conditional-method and guarded-body interface targets
pass. The pinned seed is intentionally updated to those exact tested Native
sources (641613b); archive plus patch reproduces them. ABI5/schema8 remain
unchanged. See [the body-export report](../bootstrap/guarded-body-export-results.json).

The Cinch consumer and a newly exposed Rust strict-let equation bug remain
private and under validation. The conditional182+5 full suite stopped after
142 fresh passes at a module-instantiation fixture that also changed its
exported body; a fresh continuation now holds that body constant. No complete
current or self-built frontend suite pass is claimed. Genuine self-hosting
remains incomplete.


The independent Cinch checker now selects imported closed conditional
implementations using the actual checked class head and dictionary slot.
Lexical dictionaries retain priority. Calls target the real private function,
retain complete dependent preconditions and discharge them at the actual
arguments; no unrestricted dictionary or body equation is manufactured.
Defining nominal/refinement/private measure closure remains available without
granting source aliases.

All19 runtime-fed source cases and40 independent Core checks pass, together
with seven whole regressions: dictionary resolution, source instances,
inference, effects, function preconditions, refinement constructors and the
pipeline. The current182+5 matrix is running. The seed remains c12617b at
ABI5/schema8. Definition-owned fixities remain enforced.

The preceding14case consumer passes all six fresh stage0/1 smokes. Actual
stage2 still exits4: Text.Builder's byte-copy precondition needs the actual
selected subtraction result equation. Later coverage/type/row diagnostics
follow a prior Store error; no stage2 compiler, fixed point or self-built suite
pass is claimed. Actual numeric body authority, handlers, full proof/usage/
totality and typed runtime lowering remain required. See
[the conditional consumer report](../bootstrap/conditional-method-consumer-results.json).

## Previous schema regression checkpoint

The remaining stale schema7 assertion in the private-refinement facade
test now uses the compiler's schema constant. The whole target passes with
all structural and source-visibility assertions retained. The previous180+5
matrix stopped after 92 fresh passes at that assertion; it was not a full pass.
The private conditional consumer passes19source/40Core checks, with seven
whole regressions and a fresh182+5 matrix running. Its actual stage chain
passes six smokes but still produces no stage2 compiler: a byte-copy bound
needs the actual selected subtraction body equation. The consumer is not yet
published. See [the regression report](../bootstrap/refinement-schema-regression-results.json).

## Previous Native conditional export checkpoint

Native interfaces can now retain an optional checked conditional method
record: its actual class head, complete dependent preconditioned callable
scheme and the implementation identity from the real checked dictionary slot.
A same-carrier implementation cannot replace that slot. Conditional factories
remain unresolved; these records grant no unrestricted dictionary or body
equation. Older display-only records preserve their encoding and hash.
All141 Native units and both whole class/conditional interface targets pass.
The pinned c12617b seed reproduces those tested sources; ABI5/schema8 are unchanged.

The Cinch selection consumer is private: Native preflight passes, and its
14source/40Core target is running. The preceding1056/constructor8 stage chain
passes all six smokes but exits4 with214 stage2 body diagnostics and no compiler.
The new seed's stages and a complete current suite remain unproven. The older
180+5 matrix is still running. Self-hosting is incomplete. See
[the conditional export report](../bootstrap/conditional-method-export-results.json).

## Previous dependent constructor checkpoint

Explicit refinement/proof-application results can now be retained once
as checked runtime values for dependent calls. The actual `nat-rem n
(NonZero d w)` call and its strict-let form pass; evidence for a different
payload is refused. All 23 runtime-fed source cases and 50 independent Core
checks pass. The Rust anonymous/precondition/byte interface targets also pass,
including the compiled byte consumer. The seed remains the tested1056e48
source at ABI5/schema8.

The wider180+5 matrix still runs on the preceding366/constructor snapshot7;
it is not a complete result for this later code. The preceding actual stage
chain passes six smokes but exits4 at stage2 with214 body diagnostics and no
compiler. Conditional-method selection remains a private prototype. Full
self-hosting, independent semantic/usage/totality checks and typed runtime
lowering remain incomplete. See
[the dependent constructor report](../bootstrap/dependent-refinement-constructor-results.json).

## Previous Rust anonymous-refinement checkpoint

Rust now distinguishes an anonymous refinement's unbound value from
outer telescope arguments. The formerly rejected `n <= bytes-len bytes`
contract checks, while stronger result claims and wrong observation carriers
remain refused. Refinement rendering also avoids capturing an outer `r`.
All 141 Native unit tests and the whole witness-identity, refinement,
evidence-invalidation, case-evidence and incremental targets pass. The seed is
intentionally updated to these exact sources (1056e48); archive plus patch
reproduces them. ABI5/schema8 remain unchanged.

The whole interface regressions are running. The 180+5 full matrix still runs
on the preceding366 snapshot and establishes no complete current pass. That
snapshot passes all six fresh stage0/1 smokes; actual stage2 exits4 with214 body
diagnostics and produces no compiler. Conditional `Sub Nat` remains the first
blocker. A further dependent constructor-result materialization fix is private
and under validation. Genuine self-hosting remains incomplete. See
[the Rust anonymous-refinement report](../bootstrap/native-anonymous-refinement-results.json).

## Previous Cinch constructor checkpoint

The independent Cinch checker now accepts explicit named refinement
constructors using their actual defining predicate, checked payload and erased
witness. All 20 runtime-fed source cases and 50 independent Core checks pass,
including wrong payloads/evidence, shadowing, aliases, changed custom predicates,
Nat literals and retained effects. Explicit evidence survives independent scope,
closure and typing checks; certificate refresh cannot repair a wrong witness.

The complete current 180-integration plus five-general-gate Native-assisted
suite is running. Fresh stage0 smoke checks pass; stage1 and actual self-hosting
are still running. Neither a current complete-suite pass nor a stage2 compiler
is claimed. The Rust seed remains the tested 3667043 source at ABI5/schema8.
Conditional method authority, handlers, complete independent proof/usage/totality,
typed runtime lowering, the full self-built frontend suite and stage2/3 fixed
point remain required. Definition-owned fixities are preserved. See
[the constructor checkpoint report](../bootstrap/refinement-constructor-checkpoint-results.json).

## Previous Rust witness checkpoint

The Rust producer now preserves a constructor witness's original value
bindings across shadowing and accepts direct strict-let aliases of the actual
payload. Hidden logical handles grant no ordinary source names. All 141 Native
unit tests, eight new witness-identity cases and four whole regression targets
pass: refinements, mutable-evidence invalidation, case evidence and incremental
compilation. The pinned seed is intentionally updated to those exact tested
Native sources; archive plus patch reproduces them. ABI5/schema8 are unchanged.

The complete collection matrix stopped after 85 passes at another obsolete
primitive-observation refusal assertion. It now checks the ordinary function's
own private defining body and absence of primitive/name authority. The corrected
whole byte-observation target and all eight compiled Cinch consumer cases pass
on the preceding Native fix. Remaining full-suite gates are unproven. The
private Cinch explicit-refinement-constructor work is still under validation
and is not part of this published Native checkpoint.
See [the Rust proof-identity report](../bootstrap/native-proof-identity-checkpoint-results.json).

Collection literals now elaborate through the actual defining `FromList`
method and retained `Nil`/`Cons` constructors. The independently compiled Cinch
checker passes all 32 runtime-fed source cases, including custom containers,
lexical dictionary priority, numeric ambiguity, byte limits, refinements,
polymorphic empties, nested tuples, list-pattern coverage and element effects.
Tuple fields receive expected structure before numeric defaulting. List and
Bytes factories now carry complete defining schemes after removing the
`Data.FromList`/`Data.List` dependency cycle. The pinned seed was intentionally
updated to those tested Native sources; archive plus patch reproduces them.

The same Native seed inputs pass 140 unit tests, the prelude split and nominal
interface targets. Fresh whole source-refinement and interface-precondition
targets pass. The latter checks an ordinary same-spelled measure's actual
identity, complete scheme and body, and refuses primitive identity substitution.
The subsequent complete 178-integration plus five-general-gate suite stopped
at the obsolete assertion described above; no complete current or self-built
frontend pass is claimed.

The preceding staged attempt with the collection seed passes all six
stage0/1 smokes and reaches 215 stage2 body diagnostics, down from 597. Unsupported
expression diagnostics fall from 333 to one. It produces no stage2 IR or compiler.
Isolated dependent byte-buffer contracts pass source/Core checks and reach the
named typed-lowering boundary. Imported conditional `Sub Nat` selection and the
explicit `NonZero value proof` constructor fail independently. Subsequent
stage2 dependent/effect-row errors may be cascades. A display method contract
cannot authorize an unconditional dictionary or invented primitive body.

Selected method implementation authority, kernel `NonZero` association, handler
elaboration, complete independent proof/usage/totality, typed runtime lowering,
the full self-built frontend suite and stage2/3 fixed point remain unfinished.
Definition-owned fixities are preserved. Later stages receive checked base
interfaces, never Native compiler implementation bodies or a Native frontend
fallback. Work stays within five heavy jobs: four shared slots and one reserved
fifth lane, with serial Cargo/test/codegen workers and heaps at most 2G.
See [the collection checkpoint report](../bootstrap/collection-checkpoint-results.json)
for exact snapshots, individual results and limits. The preceding complete
172+5 body-only run remains the last verified full-suite pass.

## Previous body-binding checkpoint (35d36d6)
Checked total bodies can now be associated with actual external global declarations
and retained through final Core checking and certificate refresh. The association
checks the Native identity, full monomorphic ordinary scheme, actual body and
result refinement. Calls must retain the actual global type and argument identities.
Changed bodies invalidate incompatible refinements and stale certificates.

The compiled Cinch target passes 16 checks at five identities above 2^128,
including missing-authority, conflicting-global and changed-callee cases.
All five whole focused regressions pass: typing, globals, effects, parser and
pipeline. Results and the exact snapshot are recorded in
[the body-binding report](../bootstrap/body-binding-checkpoint-results.json).
The full 175+5 suite for this newer snapshot has not run. The preceding body-only
foundation passed all 172 integration targets and five general gates, including
140 compiler units, 166 runtime tests and 19 tooling tests. That older complete
result does not establish a complete pass for newer Type/Core changes.

Source elaboration still needs interface registration of these associations.
Selected dictionary method association and replacement of the old Core class/slot
recognition remain unfinished. The schema8 interface prototype is not part of this
checkpoint; the pinned seed remains unchanged at ABI5/schema7. The previous real
stage2 still refuses one imported measure header before bodies. Genuine self-hosting,
typed runtime lowering and the full self-built frontend suite remain unfinished.

## Previous typed-call checkpoint (4df5731)
Typed total calls now retain the checked executable body and their actual typed
argument spine. Kind/carrier checks, scope traversal, substitution, unification,
escape checks and type encoding preserve that evidence. Bare function-name
applications remain inadmissible. The independent body recognizer unfolds
checked straight-line calls and strict lets to exact fixed arithmetic and
comparison equations. Changing the actual defining `Ord Nat.<` body to `False`
retains its class/slot but removes numeric comparison recognition.

The typed-call target passes 24 checks at five identities above2^128. The
actual-body target passes nine classifications for each of two real defining
method bodies; the existing three whole byte-observation regressions pass too.
All six whole regression targets pass: typing, effects, dependent functions,
indices, parser corpus and the complete pipeline. The parser corpus table now includes
both new modules with zero unsupported nodes; every previous count is unchanged.
Fresh stage0/1 pass all six smokes. Actual stage2 still exits4 at the one imported
measure header before bodies and produces no IR or compiler. The complete
174+5 suite for this Type/Core change has not run; the corrected 172+5 matrix
belongs to the preceding body foundation. Snapshot `47ade0f579de4a420a7d2cd9f9382882299082d6f6736c4e47ec90bf289aade3`
and exact validation scope are recorded in
[the total-call report](../bootstrap/total-call-checkpoint-results.json).

These components still need source/Core association with actual runtime globals
and selected dictionary fields. The old Core class/slot reflection path has not
been replaced, and `.cinchif` schema7 does not admit the body format. Full
exported-declaration/refinement and package checks, the interface migration and
intentional seed update remain necessary. No new measure-contract header has
been cleared. Self-hosting and typed shared-runtime lowering remain incomplete;
the seed, ABI5/schema7 and definition-owned fixities are unchanged.

## Previous total-body foundation (2b60c7e)
The next imported-contract foundation now carries actual closed total bodies.
Native records checked first-order bodies and follows the selected dictionary
field to its real method and fixed primitive operations. An independently
compiled Cinch checker validates scope, sorts, arities, closure and acyclicity.
It derives finite result bounds from checked literal leaves, lets and subcalls:
the actual `Data.String.utf8-encode-width` body has bounds `1..4`, while changing
an ordinary helper's body changes its bounds. Unknown values remain unknown.
Cached callers retain current semantic dependencies and lose authority when a
callee becomes cyclic or unavailable. Closed-root projection excludes unrelated
bodies while retaining the complete dependency closure.

The compiled Cinch target, seven Native body regressions, 140 compiler units,
whole incremental target and two interface targets pass. Fresh stage0/1 pass
all six smokes; actual stage2 still exits4 at the one imported measure-contract
header before bodies, producing no IR or compiler. The complete new
172+5 Native-assisted matrix passed. The corrected frozen
validation snapshot is `e44ad613bdb4b2f65740ddf30aa7c50805f632378d21f68ab32baaa4a60d0ede`. Exact evidence and limits are in
[the total-body report](../bootstrap/total-body-checkpoint-results.json).

This standalone version-1 body format is not yet admitted by `.cinchif` schema7.
The type/Core total-call bridge, declaration/refinement and package association,
and actual-body authority for numeric methods remain to be connected. No new
imported measure header has been cleared by this work. The pinned stage0 is
unchanged and does not contain the new development Native producer. Later stages
must receive only checked base-library contracts, never Native-produced compiler
implementation bodies. Full self-hosting, typed runtime lowering and the
self-built frontend suite remain unfinished.

## Previous source-decision checkpoint (614bed9)
Source `decide` now elaborates checked numeric and NonZero predicates into
retained Core decisions. Logical operators follow the parser's short-circuit
conditionals. Expected propositions are used only when the independent kernel
validates the executable test against the actual scoped values. Ordinary local
or module definitions named `decide` retain their normal bindings; the kernel
syntax is not a first-class callable value.

Strict-let aliases preserve checked primitive/value identity. A selected class
method's name and slot cannot make its result a mathematical alias: without
checked body authority that result remains a fresh runtime value. The source
and Core decision targets pass, including wrong identities, negative evidence,
logical ambiguity, large integers, metadata/type-cache corruption and unknown
method-body aliases. The dependent-function target also passes. Fresh stage0/1
pass all six smokes. Actual stage2 accepts the decision header and exits4 at
one imported measure-contract header before bodies; it produces no IR or
compiler executable. The exact staged compiler bundle matches this checkpoint.
All 11 whole focused regression targets and the complete 170+5 Native-assisted
matrix pass, including 140 compiler units, 166 runtime tests and 19 tooling tests
with no compiler/runtime ignores. The resolver runner now expects successful
source decision elaboration; its obsolete refusal assertion was the only failure
in the preceding run. The final frozen snapshot is
`5e1dd3b718118c664d4243cde312a19774adaa83c0cba405936ed850b3943a75`:
118 gates ran fresh, and 57 whole-target results have byte-identical implementation
and fixture inputs. These results establish the published source-decision code;
the main working checkout still contains the preceding 169+5 constructor baseline.
The stage evidence remains applicable because the resolver runner is outside the
unchanged compiler bundle. This is not a full self-built frontend test pass.

Self-hosting remains incomplete. Ordinary total-measure contracts, actual
primitive/instance-body authority, complete proof/usage/totality checking,
handlers and typed shared-runtime lowering remain required. No new stage2/3
fixed point or full self-built frontend suite has passed. The Native seed,
ABI5/schema7 and definition-owned fixities are unchanged.
[The source decision report](../bootstrap/source-decision-checkpoint-results.json)
records exact inputs, current results and limitations.

## Previous decision-constructor checkpoint (fbd7318)

Source `Yes` and `No` now use kernel-owned proof-arrow schemes. Their metadata
must retain the defining family, Prop parameter, ABI, tag, source arity, zero
runtime arity and erased field. Applications pass scoped evidence; patterns bind
the actual proposition or its negation. Coverage requires both alternatives.
Ordinary source constructors may share the visible spelling without replacing
the kernel identities.

Core also has an explicit decision node. It rechecks the executable fixed
comparison against the proposition in the actual value scope. Swapped tests,
changed value identities, arbitrary comparison methods and forged constructor
schemes are rejected. Source `decide` syntax is still refused; it needs the
checked predicate-construction bridge. Typed runtime erasure/lowering remains
unfinished.

All12 source constructor cases, nine metadata-corruption cases and18 source
proof cases pass. The latter now check that an erased call retains declared
divergence. Compiled decision/proof checks pass at five identities above2^128,
together with Core typing, effects and parser coverage. All15 whole focused
targets pass, including the complete pipeline, source dictionaries, actual
Text.Json, dependent functions, intrinsics and inference. Fresh stage0/1 pass
six smokes. Stage2 now accepts the Yes/No headers and exits4 at two remaining
headers: decide and the imported measure contract. It stops before bodies and
produces no IR or compiler executable; header counts are not a completion
percentage. The exact staged compiler bundle matches this checkpoint.
The complete new169+5 Native-assisted suite is running:159 fresh gates and15
exact-source whole focused targets. The installed164+5
snapshot remains the last complete baseline; it does not establish this later
implementation or the self-built frontend suite. [The decision report](../bootstrap/decision-checkpoint-results.json)
records exact sources and scope. Self-hosting is not complete. The Native seed,
ABI5/schema7 and definition-owned fixities remain unchanged.

## Previous source-proof checkpoint (91dc9e7)

Source proof arrows now elaborate to explicit erased binders and applications.
`Prop` quantifiers, named and unnamed proof domains, scoped references and actual
proof identities survive elaboration. Named evidence lives separately from
runtime value indices and can justify checked refinements. Erased applications
retain their call and latent effect row; ordinary runtime values cannot fill a
proof slot, and proofs cannot become conditions, integer results or runtime
indices. Proof computation/return and totality still require further work.

The compiled Core proof target passes at five runtime-derived identities above
2^128. The source target passes all16 cases, including generic proof forwarding,
proof lambdas, shadowing and rejection of wrong, computed or runtime evidence.
Seven existing focused regression targets are still running. This is a
work-in-progress checkpoint: the complete new167+5 suite and fresh stages have
not run. The existing fragment emitter refuses these non-runtime signatures;
typed proof erasure and runtime lowering remain required. `decide`, `Yes` and
`No` source forms are not enabled here. [The source proof report](../bootstrap/source-proof-checkpoint-results.json)
records exact validation scope. No self-built stage2 compiler or full frontend
suite has passed. The seed, ABI5/schema7 and definition-owned fixities are unchanged.

## Previous proof-arrow type checkpoint (b331049)

The type engine now distinguishes an erased proof arrow from a runtime
function arrow. Its domain must have kind Prop; its result is a proper runtime
type and its latent effect row is retained. Source arity includes the proof
slot; runtime arity omits it. Scope checks, substitution, instantiation,
generalization, normalization, inference escape checks and carrier encoding
preserve all three parts. Ordinary applications cannot supply runtime values
as proof arguments.

The compiled proof-arrow target passes at five input-derived identities above
2^128, together with the existing Core typing, value-kind, effect-type and
source dependent-function targets. All nine whole focused targets now pass,
including source instances, inference, parser and the complete pipeline. Its
matching stage0/1 pass six smokes; stage2 refuses the same four headers before
bodies and produces no compiler. This is a work-in-progress type
foundation: named proof terms and decisions are not enabled by this checkpoint.
The complete new165+5 suite has not run. The completed164+5
matrix belongs to the preceding kind/term checker. No stage2 compiler or
self-built frontend suite has passed. [The proof-arrow report](../bootstrap/proof-arrow-checkpoint-results.json)
records exact inputs and validation scope. The seed, ABI5/schema7 and
definition-owned fixities are unchanged.

## Previous kind/term checkpoint (c757621)

Core annotations now receive a separate kind, type and effect check after
elaboration. Nominal names must match their declaring kinds; quantified and
value-index references must belong to their actual scopes. Applications,
lambda and clause results, constructor fields, branches, tuples, primitive
operands, strict lets and latent effect rows are rechecked. The global-reference
checks apply to the same source definitions and dictionary method bodies.

All seven focused whole Rust targets pass: forged Core annotations, source
dictionaries, dependent functions, effects, inference, parser coverage and the
complete pipeline. Runtime-input checks use five identities above 2^128. All
21 source dictionary cases, actual Text.Json, strict-let dependent aliases,
function-valued indices and retained rejection cases pass. The implementation
and each reused target's own fixtures are byte-identical; only the separately
rerun pipeline Rust fixture differs from the preceding focused snapshot.

This kind/term snapshot now passes the complete Native-assisted suite of164
integration targets and five general gates: 140 compiler unit tests, 166 runtime
tests, binary/doc tests and tooling. Compiler/runtime gates contain no ignored
tests. Coverage is162 fresh gates and seven exact-source whole focused targets.
Its implementation is installed in the main working checkout. This result does
not establish the later proof-arrow/source-proof implementations or the suite
through a self-built frontend. Fresh stage0/1 builds pass all six smoke checks. Actual stage2 exits4
with four header diagnostics before bodies: decide/Yes/No evidence and the
imported measure contract. No stage2 IR or compiler executable is produced.
The exact compiler bundle matches this checkpoint. The prior installed
source-instance snapshot passed all162+5 gates, including 140 compiler units and
166 runtime tests; that result does not establish this implementation's complete
suite. [The Core typing report](../bootstrap/core-typing-checkpoint-results.json)
records exact source and validation scope.

This is not a complete independent proof/usage kernel. Complete declaration and
coverage checking, actual primitive/instance-body numeric authority, totality,
ownership and generative regions, and decision/proof erasure remain required.
Ordinary measure contracts, handlers and typed shared-runtime lowering are also
unfinished. Actual stage2/3 fixed point, changed-source behavior and the full
self-built frontend suite remain unproven. The seed, ABI5/schema7 and
definition-owned fixities are unchanged; work stays within five heavy jobs.

## Previous global-reference checkpoint (426d796)

Core references now undergo an independent check against retained global
declarations. Global and local uses must match their actual schemes and type
argument kinds. Method owners and slots, constructor schemes, dictionary factory
arguments, lexical providers, universal openings and declared dictionary binders
are rechecked. Dictionary method bodies pass the same reference checks as ordinary
source definitions. The global context has a private validated constructor.

All six focused whole Rust targets pass: global-reference forgeries, real source
dictionaries, inference, intrinsics, the parser corpus and the complete pipeline.
The compiled Cinch tests run at five input-derived identities above 2^128; all
21 source dictionary cases and the actual Text.Json source dictionary pass.
The pipeline accepts dictionary declarations and explicit selection, then reaches
the emitter's dictionary refusal. Using a dictionary as Int remains a type error;
unimported class methods remain hidden. Parser coverage for Bootstrap.Global is
zero unmodeled nodes.

This remains a work-in-progress checkpoint. The complete current suite has 163
integration targets and five general gates and has not run. The completed
162-target source-instance suite establishes the preceding implementation's
Native-assisted result. Its exact sources are installed in the main working
checkout. That pass does not establish this later implementation's complete
result. Fresh stage0/1 smokes pass for the global-reference implementation;
stage2 still exits4 with four headers before bodies and produces no IR/executable. [The global-reference report](../bootstrap/global-reference-checkpoint-results.json)
records exact sources and results. The prior fully completed Native-assisted
checkpoint remains 623d33e; no self-built frontend suite has passed.

This phase does not validate all interior term/kind rules or numeric primitive
semantics. A separate kind and Core term checker is under validation. Actual
primitive or checked instance-body authority, totality, decision/proof erasure,
measure contracts, handlers, usage checking and typed shared-runtime lowering
remain required. Stage2/3 fixed-point and changed-source behavior remain unproven.
The Native seed stays source81/patch655, runtime ABI5/interface schema7, with
definition-owned fixities. Work stays within five heavy jobs.

## Previous source-instance checkpoint (146b3b2)

Source instance dictionaries are now published as a work-in-progress checkpoint.
The typed class and instance records retain defining owners, exact method slots,
specialized method schemes, factory parameters and captured dictionaries.
Source members are checked against the defining class interface, including
method-local quantifiers and requirements. Explicit `with` accepts checked
class dictionaries and retains the existing Io region-escape check.
Ordinary values remain invalid dictionary arguments; only implicit instances
participate in implicit search. Fixities remain owned by their declaring module.

All eight focused whole targets pass: the new dictionary checks, six existing
regressions, and the checker fixture corrected to retain its non-signature-target
refusal. The compiled Cinch checker passes 21 source cases and the actual
Text.Json source dictionary. Fresh stage0 and stage1 pass all six smoke checks.
The actual stage2 attempt now reports four header diagnostics before bodies:
decide/Yes/No evidence and the imported measure contract. The source instance
header is resolved; no stage2 IR or compiler executable is produced.

The complete Native-assisted suite of 162 integration targets and five general
gates is running. Exact-source audits reuse only unchanged whole targets; the
independent checker and parser corpus fixtures are rerun. The parser reads the
new Bootstrap.Instance module with zero unmodeled nodes; its pinned corpus table
now includes that measured entry, with every previous count unchanged. This is not a complete current-suite pass
or the suite through a self-built frontend. The previously verified baseline
remains commit 623d33e. Header counts do not measure overall completion.
[The instance checkpoint report](../bootstrap/source-instances-checkpoint-results.json)
records the frozen inputs and the exact validation state at publication.
Real dictionary runtime lowering, the independent global kernel, and the full
stage2/stage3 bootstrap remain unfinished.

## Last complete Native-assisted checkpoint (`623d33e`)

Machine-integer value contracts and source effect declarations are now installed
in the main working checkout and published with the source-matching seed.
All 161 Native-assisted integration targets pass, together with all five general
gates: 140 compiler unit tests, 166 runtime tests, binary/doc tests and tooling.
The compiler/runtime test gates contain no ignored tests. This is not the suite
through a self-built frontend, and full self-hosting is not achieved.

The complete suite uses 152 fresh gates, seven unchanged completed whole targets,
and seven whole focused targets. Reuse audits retain byte-identical compiler,
Cinch implementation, shared harness and each reused target's own fixtures;
the independently changed runners and pipeline fixture are rerun. The frozen
snapshot has 575 inputs, SHA256
`d6f7b21cfefdd56ede0467af55ff7abd691928dc64149c05b57089958bf4f840`. Coverage and results are in
[the checkpoint report](../bootstrap/source-effects-checkpoint-results.json).

Cinch retains exact machine carriers and literal bounds, checked byte observations
and strict-let proof identities. Machine arithmetic remains opaque to mathematical
proofs. Source CoreEffect/CoreOperation records retain declaration owners, kinds,
parameters, signatures and ordered slots; calls preserve their effect rows.
Declaration closure checks reject forged scopes, carriers, slots and improper
value types. The independent global term/proof kernel remains required.

Fresh stage0 and stage1 builds pass all six smoke checks. The actual stage2
attempt exits 4 with five header diagnostics before body checking: decide/Yes/No
proof forms, the utf8-encode-into imported measure contract, and Text.Json's
source JsonCodec_Json instance. The Parse effect header is resolved. Stage2
produces no IR or compiler executable. Header counts include cascades and are
not a completion percentage. The current tested compiler bundle is byte-identical
to the staged one: named-source hash
`e10dac1b622287180da4d453b1deff8e92da63ba7c10016e7dda2312c46d0a01`,
actual bundle byte SHA256
`27ddda055b2a0bdbb7be53ea70963ea5bc29ca907fa9f288a2752bdfa5de8541`.

The installed Native source SHA256 is
`81bb6e364f45201360e11d5b32e2b3f22cd4719a9dcb018551da034416e725e9`,
patch SHA256
`655dfb04a042df9cd0aff5abebbbcd23510fb35efc92567e2d9ae8cfb9dedd5d`.
An independent archive-plus-patch check reproduces the exact tested sources,
including another verification at installation. Runtime ABI 5, interface schema 7
and definition-owned fixities remain unchanged. Unrelated site and reader-doc
edits are preserved.

Genuine source instance dictionary construction is under separate validation.
Source decision/proof erasure, checked imported total-measure contracts, handler
typing/runtime dispatch, an independent global typing/proof kernel and typed
shared-runtime lowering remain required. Actual stage2/3 fixed-point agreement,
changed-source behavior and the full suite through a self-built frontend remain
unproven. Work uses at most five heavy jobs: four shared slots and one explicit
staged-build/probe lane, with one Cargo worker/test thread and Cinch heaps at 2G.

## Previous checkpoint details

Source effect declarations now retain checked CoreEffect/CoreOperation records:
the declaring owner and kind, ordered parameters, operation schemes, and handler
dispatch slots (return slot 0; operations start at 1). The declaration checker
rechecks binder identities and kinds, exact index carriers, proper value types,
parameter prefixes, unique names/slots and the exact declaring effect on the
last argument arrow. Earlier arrows stay pure. Source operations cannot also
have an ordinary definition. Calls propagate their declared effect rows.
The elaboration result retains this typed declaration information.

All four focused gates pass. Twenty changed-source cases cover polymorphic and
curried operations, type/region parameters, dependent results, first-class calls,
effect unions, wrong owners/regions and effect erasure refusals. Each run also
checks 22 Core declaration cases at a runtime-derived identity above 2^128,
including forged scopes, wrong carrier kinds and invalid argument/result nodes.
The existing effect-type, effect/capability and Core regressions pass. These are
Native-assisted executable checks, not the full suite through a self-built
frontend. The complete current suite has 161 integration targets plus five
general gates and is not yet completed for this implementation.

The current combined complete suite is now running from snapshot
`8b0fe40febabe2529f791a97023103e622917050136a90d315a2295b6eb05ad9`.
Its updated instance-resolution runner passes at all five runtime inputs; it
accepts checked I64 comparisons while refusing opaque lexical ordering, wrong
slots/carriers and machine arithmetic facts. The guarded-arithmetic runner also
passes all 26 source cases. Six focused whole targets plus 160 fresh remaining
gates cover the planned 161 integration targets and five general gates.
The four earlier focused targets are reused only after auditing that their
implementation, harness and own fixtures are unchanged. This snapshot changes
only the instance runner from the staged snapshot; the actual compiler bundle
is byte-identical, so the verified stage boundary above still applies.
The full suite is pending; neither earlier stopped machine matrix was a full pass.

The end-to-end pipeline now also checks source effect declarations and
operation calls through the actual checker into the emitter. Both reach the
emitter's named refusal; source handlers remain a checking refusal. The full
pipeline target passes, including its existing runnable examples, module and
interface checks. The prior run stopped on its obsolete declaration-refusal
message; the corrected snapshot changes only that Rust fixture and reruns it.
The remaining complete suite continues with exact-source reuse audits from
`d6f7b21cfefdd56ede0467af55ff7abd691928dc64149c05b57089958bf4f840`. The compiler implementation and
verified staged bundle are unchanged; full-suite completion remains unproven.

Fresh stage0 and stage1 builds pass all six smoke checks. The actual stage2
attempt exits 4 before body checking with 5 header diagnostics.
It produces no IR or compiler executable. The exact remaining categories are
recorded in [the source-effect checkpoint report](../bootstrap/source-effects-checkpoint-results.json).
Header counts include cascades and are not a completion percentage.
The frozen staged snapshot has 575 inputs, SHA256
`4a83dcac07a3df0aeed88f8f49089db7d5ae7bf326ffd4a27137116a087333ae`. Its implementation matches the focused snapshot
`d32f6cb89ddb4268e4162f0309a771711f82fed9ca5c7c415e7f99c109eafa4e`; only the independently checked guarded-arithmetic
runner was updated. The compiler bundle is `e10dac1b622287180da4d453b1deff8e92da63ba7c10016e7dda2312c46d0a01`.

The Native seed remains source SHA256 `81bb6e364f45201360e11d5b32e2b3f22cd4719a9dcb018551da034416e725e9`,
patch SHA256 `655dfb04a042df9cd0aff5abebbbcd23510fb35efc92567e2d9ae8cfb9dedd5d`. A fresh archive-plus-patch check independently
reproduces its exact source bytes. Interface schema 7 and definition-owned
fixities remain unchanged. The original working checkout retains the prior
fully validated byte checkpoint; the published source-effect checkpoint is work
in progress. The separately running earlier machine suite does not establish
that the current complete suite passes.

Source proof/decision forms, imported total-measure contracts, source instance
construction, handler typing/runtime dispatch, an independent global typing
kernel and typed shared-runtime lowering remain required. Stage2/3 fixed-point
agreement, changed-source behavior and all tests through a self-built frontend
remain unproven. Work stays within five concurrent heavy jobs, four shared slots
and one staged-build lane, with one Cargo worker/test thread and Cinch heaps
limited to 2G.

## Previous machine-value checkpoint (5d7af34)

The published compiler now includes checked machine-integer value contracts.
All eight fixed-width carriers retain their exact ranges. Cinch checks integer
literals and patterns against the expected carrier, projects refined machine
values to that carrier, and propagates expected types through case branches.
Core independently rechecks literal ranges and derives bounds from actual typed
local identities. Machine arithmetic remains opaque to mathematical proofs.
The Native exporter preserves checked comparison and conversion contracts
through facades. Interface schema 7 and definition-owned fixities are unchanged.

This is a work-in-progress checkpoint. The new focused gates pass: 46 source
cases, 121 Type/Core checks at each of five runtime-generated identities,
Native export checks, existing precondition/anonymous-interface tests, and
13 real compiled Cinch conversion cases. The complete Native-assisted suite
of 160 integration targets and five general gates is still running. The prior
fully validated byte-observation checkpoint is commit cf1f491, with 158
integration targets, 140 compiler units and 166 runtime tests passing.
Those earlier results do not establish that the new complete suite passes.

The guarded-arithmetic runner now accepts checked machine-value comparisons
and explicitly rejects machine subtraction certificates and wrapping-arithmetic
proof facts. All 26 source cases and its retained certificate invariants pass.
The first complete machine run stopped on its old blanket comparison refusal;
the corrected snapshot changes only this runner. The remaining 133 gates resume
with per-target audits for 19 unchanged completed/in-flight targets and the
unchanged focused gates. The complete suite remains pending; the older stopped
run is not a full pass. The corrected snapshot is
`1cd10f728dc82864948ac6d8eb852e186928d16efbc88d9dd15f569e6b023e05`.

Fresh stage0 and stage1 builds pass all six smoke checks with the new seed.
The actual stage2 attempt exits 4 before body checking with six header
diagnostics: decide/Yes/No evidence, the utf8-encode-into imported measure
contract, Text.Parse's effect declaration and Text.Json's instance declaration.
The nat-to-i64 imported-contract error is resolved. Stage2 produces no IR or
compiler executable. Diagnostic counts include cascades and are not a measure
of completion.

The new seed independently reproduces the exact tested Native sources:
source SHA256
`81bb6e364f45201360e11d5b32e2b3f22cd4719a9dcb018551da034416e725e9`,
patch SHA256
`655dfb04a042df9cd0aff5abebbbcd23510fb35efc92567e2d9ae8cfb9dedd5d`.
Archive plus patch was independently verified again before publication.
The frozen validation snapshot has 573 inputs, SHA256
`eaa5d0e0e8e52b2ef0e6e9dae8a4cbc7843ee97ceb6fa35226cc2c0541d124f2`.
The staged snapshot differs only in the intentionally updated seed pair:
`9d8c0744b1cef256ac9dd36d72afa354ced0fa4398633ee7f160d7ec3ba0d10a`.
The compiler bundle is
`8500b23e4fd519777f47c2f61c82c39ef290e0206df8f87aa622eacb20e1a46d`.
Current results and pending gates are recorded in
[the machine checkpoint report](../bootstrap/machine-checkpoint-results.json).
The older checkpoint-results.json and integration reports describe cf1f491.
The original working checkout retains that fully validated byte checkpoint
until the new complete suite passes; this publication contains the new work.

Proper source declarations and decision/proof forms, checked imported measure
contracts, an independent global typing kernel and shared-runtime typed lowering
remain required. Stage2/3 fixed-point agreement, changed-source behavior and all
tests through a self-built frontend remain unproven. Work uses at most five
concurrent heavy jobs, one Cargo worker and one test thread per job, and
limits generated Cinch heaps to 2G.

## Previous fully validated byte-observation checkpoint (cf1f491)

Byte observations and strict-let proof identities are installed and validated.
Cinch retains the actual typed buffer through inference, substitution, private
refinement contracts and Core checking. The Native exporter emits a closed
kernel operation only for the checked primitive. Ordinary same-named functions
remain unresolved. ABI 5 supplies nonnegative, machine-sized byte-length facts
only for actual scoped byte buffers. Interface schema 7 and definition-owned
fixities are retained.

All 158 Native-assisted integration targets pass, together with compiler unit,
binary, documentation, runtime and tooling gates. Compiler units pass 140 tests
and runtime suites pass 166, with no ignored tests in those gates. This is not
the suite through a self-built frontend. The final validation snapshot contains
570 unchanged inputs, SHA256
`db363ca63a9fc22322ad420233ff206d00ffa879d7761a66c56a962e029ebe52`.
Twenty completed targets and ten focused targets retain byte-identical compiler,
bootstrap, prelude/runtime, shared harness and own-fixture inputs. The two
updated Rust fixtures are rerun; all remaining 127 integration targets and the
five general gates run fresh. Evidence and per-target reuse audits are in
`/tmp/cinch-byte-observer-validation-12c/`.

Strict monomorphic lets carry checked value witnesses separately from local
types. Core reconstructs them from each actual retained RHS. Nine source alias
cases pass, including refined buffers and precondition calls. The Type/Core
runner passes 30 checks at each of five runtime-generated identities above
2^128, including rejection of a copied alias certificate with a different RHS.
The imported NonZero runner passes 41 caller cases; wrong buffers, shadows and
missing conditions remain refused. Numeric interface tests retain ordinary
same-name refusals and check the actual closed primitive allocation promise.

The installed Native seed independently reproduces every checked source byte:
source SHA256
`74b4457a63c4898091ad5a31324a8ca098cf1aecfa789670926fff3be2f9b1ab`,
patch SHA256
`61685f05a4a781d58fa0417df2282402e9e2b76ad0fb1bb07303bcdfc6e25825`.
Archive plus patch is verified independently during preparation, staged builds
and installation. The Root installation preserves unrelated site/docs changes.

Fresh stage0 and stage1 builds pass all six smoke checks. The actual stage2
attempt exits 4 before body checking with seven header diagnostics, down from
66 at checkpoint 6d67ed6: decide/Yes/No evidence, two imported using contracts,
Text.Parse's effect declaration and Text.Json's instance declaration. All
byte-length/index-conversion header diagnostics are gone. Stage2 produces no IR
or compiler executable. Counts include cascades and do not measure completion.
The compiler bundle is
`ecebef95541d62e92f8020553668c31e4ab31fa152c136a2994ad906fa9b0a66`.
The unchanged staged snapshot has 570 inputs, SHA256
`85a0ddb9f861a44c76fb66635f8fde71c495073307eb15d05aa8db4d6fc7c232`;
its implementation is identical to this matrix, with the independently verified
seed files and preceding versions of the two Rust fixtures. Stage evidence is
in `/tmp/cinch-byte-observer-stages-12/`.

New machine-value/conversion support is under separate validation. Its Type/Core
runner passes 121 checks at five runtime-generated identities, and thirteen real
Cinch conversion cases pass. A private staged attempt passes all six smokes and
reaches six header diagnostics. These machine changes are not installed here;
plain machine declarations exposed a legacy-dispatch gap being checked in the
next candidate. This evidence does not establish a complete machine regression
suite or a self-built compiler.

Source declarations and decision/proof forms, checked imported measure contracts,
an independent global typing kernel and shared-runtime typed lowering remain
required. Actual-call probes identify nat-to-i64 and utf8-encode-into as the two
unresolved imports at this checkpoint. Stage2/3 fixed-point agreement,
changed-source behavior and all tests through a self-built frontend remain
unproven. Work uses at most five concurrent jobs, one Cargo worker and one test
thread per job, and generated Cinch heaps limited to 2G.

## Prior checkpoint history and acceptance details

Interface schema 7 carries the defining template of a private named
refinement when that type occurs in an exported value contract. The closure
retains its defining module, ordered parameters and kinds, binder, carrier,
and predicate through renamed, value-only facades. Carrying this metadata
does not make the private type name available to source importers.

The producer supports closed numeric templates over primitive or declared
parameter carriers. The consumer registers the defining templates before
checking imported signatures. Refinements require their template and forbid
constructor metadata; other nominal declarations require a null refinement
payload. Contradictory defining contracts are rejected by the interface
loader, and public/private representations must agree in source checking.
Template changes participate in canonical interface hashes.

The real Native producer fixture checks private Window and Positive types
through a facade, parameter order, hidden names, canonical round trips,
predicate hash changes, and four malformed payloads. The compiled Cinch
consumer passes seven source cases covering projection, call bounds, hidden
names, and insufficient result bounds, plus a rehashed contradictory facade
that must fail in the loader with the exact defining-contract diagnostic.
All seven validation gates pass, including Native contract tests, import
regressions, source/type/Core contracts, dictionaries/effects, and the
pipeline/type-syntax/stage-ladder targets. The first two Native gates reuse
audited evidence from the byte-identical producer; the consumer and later
gates are rerun. The final audit covers 560 inputs:
`812d3b7672c10b46b39f368d9fcef70a6a1eb991342b21c842cfcc344fc1dfba`.
Evidence is in `/tmp/cinch-refinement-closure-validation-2/`.

The Native seed now matches the exact tested schema-7 sources:
source `7a2dd486210e6beaaa6db3c7b622983f9f4a6f7afe0481dd2e56b84532c8cd8c`,
patch `1d3479e414067f9d86a86d927f3ea323f24432632c55743de811a50f18e5d3c3`.
Archive plus patch reproduces every Native source byte, checked both during
preparation and independently at installation. Runtime ABI 5 is unchanged.
Fresh schema-7 stage0 and stage1 builds pass all six smoke checks. The
actual stage2 attempt exits 4 before body checking with 76 header
diagnostics, down from 84. It produces no IR or compiler executable. Of
these diagnostics, 22 have imported/synthetic positions and 19 concern
unchecked calls; counts include cascades. The final audit confirms all
560 inputs of staged snapshot
`9e68828fdc7bd9c28a618c3ff072ac891196c91f8a212d15b9a1c2c76222560a`
are unchanged. The compiler-source bundle is
`e3967c48eb773c9b453abeb846794799b979251b6569336fa78c305305b9a57d`.
Evidence and source-mapped diagnostics are in
`/tmp/cinch-refinement-closure-stages/`. Generic Div/Rem method contracts
now expose two abstract-carrier NonZero refusals; those still require
proper predicate formation and checked numeric reflection.

General nominal carrier closure, anonymous refinement serialization,
checked measure/call contracts, remaining source forms, independent global
typing checks, and typed runtime lowering still require implementation.
These validation gates do not establish a self-built stage2, stage3 fixed
point, changed-source behavior, or the complete compiler/runtime suite
through the self-built frontend.

Structural NonZero propositions now retain a distinct typed node through
substitution, normalization, scope checking and codecs. Logical discharge
reads the checked numeric operand as inequality to zero; type conversion
retains the structural proposition identity. Core reifies operands against
the actual local environment and rechecks refinement evidence. Same-spelled
refinement constructors in carrier position retain their defining templates;
the closed structural predicate in predicate position keeps its own meaning.

Source aliases, anonymous refinements and function preconditions use this
node. Local fixed-width integer predicates retain explicit mathematical
promotion and the actual input carrier; floating-point operands and machine
arithmetic remain opaque. Native-produced NonZero preconditions over plain
Nat/Int arguments now preserve the checked structural predicate, actual
argument positions and defining function identity through renamed facades.
Calls needing measure contracts and exported machine-integer preconditions
remain explicitly unsupported by this producer tier.

The compiled type/Core runner passes 31 checks at each of five runtime
identities above 2^128. The source checker passes 52 local cases and ten
real native-interface cases; the imported-precondition runner passes 36
real-interface caller cases. Native contract tests, all twelve affected
regression targets, type syntax and the complete pipeline pass. The final
audit confirms all 557 inputs of snapshot
`191dbdaa00cfab050fdcc656cb6729c4aae63550467a1424a763b8672879e491`
are unchanged; evidence is in `/tmp/cinch-nonzero-import-validation-2/`.

The Native seed is intentionally refreshed to the exact tested sources:
source `70bc65c32dad2b6e376c7c7c31ed5ae1c23c4dc27639cb9e100708f78820c54d`,
patch `7795dad12688a232db7ec315e5848de375199d7f00bfcf363a054d6adedad0f1`.
Archive plus patch reproduces every Native source byte. Interface schema 6
and runtime ABI 5 are unchanged. Fresh staged verification passes the
combined ladder and all six rebuilt stage0/stage1 smoke checks. Stage2 exits
4 before body checking with 84 header diagnostics, down from 85; no IR or
compiler executable is produced. Counts include cascades. There are 30
imported/synthetic signature diagnostics and 19 unchecked-call diagnostics.
The 557 inputs of staged snapshot
`b61cc0462187eb99c74fcb8c53a17abc64067d014f62f2829a65e74edad10305`
have an unchanged final audit. Evidence is in `/tmp/cinch-nonzero-stages/`;
the compiler-source bundle is
`0a26f4889588a531cc57bcc31bace0554cf6d956ac71a0bbb158b323ced36ece`.
Closed imported refinement/measure contracts, remaining language forms,
the independent global typing kernel and typed runtime lowering still
require work. These checks do not establish self-hosting.

Named refinement declarations now register their defining templates and expand
saturated applications with ordered, kind-checked actual arguments. Expansion
retains a typed refinement and its proof obligation. Recursive templates,
wrong scopes and arities, unsupported predicate calls, and unsaturated
higher-kinded aliases are refused. Imported templates use the actual defining
identity and checked native-produced refinement metadata, including aliases
re-exported by a facade; display spellings do not establish ownership.
Imported private/nominal carriers still need defining contract closure, and
structural predicates such as NonZero remain a separate required layer.

The compiled source checker passes 24 named-refinement cases and six cases
using real native-produced interfaces. Anonymous refinements, all affected
regressions, type/Core contracts, syntax and the complete pipeline pass.
The final audit confirms all 552 inputs of snapshot
`83bc43a806260a3e31b8b38ff169a128cc4420e435b3623ef0ecb0bfc7fcc1c0`
are unchanged; evidence is in `/tmp/cinch-refinement-alias-validation-3/`.
The native seed is unchanged. Fresh staged verification passes the combined
ladder and all six rebuilt stage0/stage1 smoke checks. Stage2 exits 4 before
body checking with 85 header diagnostics, down from 89; no IR or compiler
executable is produced. Counts include cascades. There are 19 unchecked-call
diagnostics and 31 imported/synthetic signature diagnostics. Structural
NonZero predicates, closed imported refinement/measure contracts, remaining
declarations and typed runtime lowering still require implementation.
The 552 inputs of staged snapshot
`4dc4b6669b5c55fefe38f3e5801b0c9e8a8ca5eb58f28ec0b2e06517d0023c14`
have an unchanged final audit. Evidence is in `/tmp/cinch-refinement-alias-stages/`;
the compiler-source bundle is
`0f89a6971df70a2ce8ee871af81f85ceb908e9bb04a204911ece2dc14b677299`.
These checks do not establish self-hosting.

Source and Core now retain typed value-index projections through dependent
arguments, numeric arithmetic and local aliases. Predicate comparisons promote
Nat and fixed-width machine integers explicitly to mathematical Int when
needed. Projection and promotion check their operand against the actual Core
local environment; wrong sorts, identities and scope cannot supply proof facts.
Core comparison canonicalizes both retained value-rigid representations
consistently; inference still checks their original scope levels and identities.
The Source Pi regression exercises numeric constructor-pattern applications.
Machine arithmetic, float comparisons and unverified calls remain opaque.

The runtime-fed source runner passes 51 cases. The compiled type/Core runner
passes 58 checks at five runtime-selected identities above 2^128, including
forged projection/promotion and wrong-identity checks. Both earlier refinement
contract runners, all twelve affected regression targets, type syntax and the
complete pipeline pass. The final audit confirms all 550
inputs of snapshot
`28da388e4a59200a1850e6e90101c304049c210e0fbc6e95556ac78b0ef66503`
are unchanged; evidence is in `/tmp/cinch-typed-index-source-validation-2/`.
The native seed is unchanged. Named imported refinement aliases, checked
primitive/measure contracts, the independent global typing kernel and the
typed runtime backend remain required. Fresh staged verification passes the
combined ladder and all six rebuilt stage0/stage1 smoke checks. Stage2 exits
4 before body checking with 89 header diagnostics, down from 92; no IR or
compiler executable is produced. Counts include cascades. Mixed Int/Nat/I64
comparison sort errors are gone. There are 19 unchecked-call diagnostics and
32 imported/synthetic signature diagnostics. An additional arithmetic refusal
in Text.Json.digest-hex-into contains unverified bytes-len calls; it is not
evidence that those calls or machine arithmetic have been admitted. The 550
inputs of staged snapshot
`00f38e98c1d90c76fdd8db44ef7b0f6a3ce2f7cfed8601646dff7e11bf665b19`
have an unchanged final audit. Evidence is in `/tmp/cinch-typed-index-stages-2/`;
the compiler-source bundle is
`3aae51ec1cf16bbbde0fc1228bf4e1b6f58605da2f031ca6e54974db08aab5f6`.
These checks do not establish self-hosting.

Typed value-index projection, integer promotion and mathematical arithmetic
are integrated in the type engine. The operand retains its actual carrier
through dependent substitution, meta solutions, scope checks, alpha conversion
and codecs. Promotion is explicit; unification cannot forget a refinement or
silently equate Nat, Int and fixed machine integer sorts. Machine arithmetic,
floats and open partial Nat subtraction remain refused, and D28 conversion
continues to fold numerals only.

The compiled runner passes 47 checks at each of five runtime-selected
identities above 2^128. All nine affected regression targets pass. The final
audit confirms all 550 inputs of snapshot
`7264ab3b047a7cb309406bf5b580959b4605bf2d976b6400dea3bc2d30477f85`
are unchanged; evidence is in `/tmp/cinch-typed-index-validation-2/`.
This is the type-engine layer. Source projection/promotion and the corresponding
Core proof reifier are prepared separately and are not enabled by this change.
The new index forms remain refused by Core until that integration is checked.
The native seed is unchanged; the latest actual staged result remains 92
header diagnostics, before body checking, with no stage2 IR or executable.

Source anonymous refinement checking is integrated after 42 runtime-fed
source cases pass, followed by the Core/type contract and syntax checks,
all eight affected regression targets and the complete pipeline. The source
runner parses and resolves each input before checking it, then independently
runs the existing Core scope/certificate checks on the retained elaboration.
The pipeline confirms a valid refined result reaches the named typed-lowering
boundary, while an invalid ignored refinement is rejected before producing
IR or an executable. The final audit confirms all 548 inputs of snapshot
`99a531ddb9e8f6cb4666915ab2c985f73b6a667c6c99e94cfd830c0a90d6625d`
are unchanged; evidence is in `/tmp/cinch-refinement-pipeline-validation/`.
Earlier source/contract/regression results are reused from
`/tmp/cinch-refinement-source-validation-2/` only after verifying every input
except the two added pipeline cases is identical.

Anonymous mathematical refinements now retain a scoped result binder,
carrier and predicate. Expected refinements propagate through branches,
guards and strict lets; introduction checks the actual value and scope.
Numeric comparison refinements project explicitly through `CForget` when
used as ordinary values, while stored refined values retain their facts.
Generic `refined (True) a` retains its carrier. Source projections preserve
their real local binding levels. Bad bounds, wrong/shadowed identities,
ignored invalid arguments/ascriptions, erased effects, replacement comparison
operators and captured global/argument names are refused. Unknown predicate
calls and machine/float arithmetic gain no mathematical proof facts.

Named refinement aliases, checked imported refinement and measure contracts,
typed index projection for dependent refined arguments and the typed runtime
backend remain required. Global Core references still rely on checked source
or interface schemes; the independent global typing kernel is incomplete.
The native seed is unchanged. Fresh staged verification passes the combined
ladder and all six rebuilt stage0/stage1 smoke checks. Stage2 exits 4 before
body checking, with 92 header diagnostics and no IR or compiler executable.
The new refinement path exposes additional call/sorting failures behind the
earlier blanket refusal; counts include cascades, not independent features.
The 548 inputs of staged snapshot
`92a184b7407c73aa2509f9bc1764430bf7f17eef9857203e570143d326e1836e`
have an unchanged final audit. Evidence is in
`/tmp/cinch-source-refinement-stages/`; the compiler-source bundle is
`aab578ea6507272941c756459adf54b42c8b4378762a0153d4b746ba7ea400eb`.
There are 21 unchecked-call diagnostics in the new result, including calls
inside the newly elaborated result refinements. Typed index calls/projections,
mathematical promotion and checked imported refinement/primitive contracts
remain required. These checks do not establish self-hosting.

Earlier Core-only checkpoint:

Core refinement introduction and projection are now integrated. `CRefine`
retains its actual value and a certificate; the closed checker recomputes the
predicate from that value and its actual lexical scope before checking the
certificate. `CForget` requires the actual refined operand and exact carrier.
Refined-local facts come from the actual binding types, and both forms retain
their operand's effects. Bare literals, tuples and primitive nodes cannot
establish a refinement merely by carrying its type annotation. Refined branch
and let results are checked on every path, and definition parameter patterns
must agree with the declared function domains. The earlier scope-test fixture
now declares the function arrows its argument patterns require.

The compiled Core runner passes 26 checks at five runtime-chosen identities;
the 29 refinement-type checks also pass with these rules enabled. All nine
regression targets pass, including the 20 real-interface precondition cases.
The 546 inputs of snapshot
`febea3f4922bbbaddb7e6e5b58b29784a1fc5b13e357fc0fd6443297fe6dff9d`
have an unchanged final audit. Evidence is in
`/tmp/cinch-refinement-core-validation-2/`; the focused checks are reused from
`/tmp/cinch-refinement-core-validation/` only after verifying identical
compiler and focused-test inputs. The sole input change is the separate scope
fixture's function type.

Source refinement conversion, imported refinement contracts and typed runtime
lowering remain required. This is the scope/certificate-checking tier; complete
independent validation of the global typing environment remains required.
The native seed is unchanged. No new stage-2 executable is established by
these checks; the latest actual staged boundary remains the 86-header result.

Earlier engine-only checkpoint:

The refinement type engine is integrated after its compiled runner passes
29 checks over five runtime-chosen identities above 2^128, and all six
Core/inference/Pi/proposition/value-index/value-kind regression targets pass.
The final audit confirms the 544 inputs of snapshot
`6e9007d2745aeb723193cde8526c50682d784908fb747d1c0bec3810133131ac`
are unchanged; evidence is in `/tmp/cinch-refinement-type-validation-3/`.
`URefine` retains its carrier, value binder and predicate through codecs,
alpha comparison, capture-avoiding substitution, normalization, occurs checks
and scope checks. Opening returns the precise predicate to prove and supplies
no evidence. Core runtime use remains refused until certified introduction
and elimination are implemented; source refinements and imported refinement
contracts are not enabled by this engine change. The native seed is unchanged.
The most recent actual stage-2 boundary remains the 86-header result below.

The imported-instance and numeric-condition continuation is integrated after
all four frozen validation groups pass: four native interface targets, the
compiled instance-resolution checks at five runtime-chosen identities, all
20 real-interface source cases plus the complete pipeline, and all eight
affected regression targets. The final audit confirms all 542 inputs of
snapshot `27e50cd933afc19f63e7cd8fb3de0b8b37a3146ca3791a3cd0a0e47db85f55c9`
are unchanged. Evidence is in `/tmp/cinch-instance-integration-validation-2/`.

Checked imported implicit factories now resolve recursively after lexical
givens, with typed contexts, unique determined selection, coherence and
transactional failed candidates. Canonical mathematical class operations
retain their runtime dictionaries while proof reification checks their
actual identities, signatures and operands. Lexical or unresolved dictionaries,
replacement operators, machine arithmetic and unknown calls gain no proof
facts. Native resolved interfaces preserve checked numeric conditions and
their actual dependent argument identities; unsupported refined or conditional
factory contracts remain unresolved.

The intentional seed update was prepared from those exact validated native
sources and verified by applying its patch to the pinned revision. Native
source hash: `07ca077760713a26f6c2ae59b4b82efda8c4d0aeb84a043fb356816f67fd67a3`;
patch hash: `cb2568a1cb3247fbe12d702a977d5877efa7f2f6c4b18a756b0c54448d8c36f6`.
Schema 6 and runtime ABI 5 are retained. Fresh staged verification passes
type syntax, the combined ladder and all six rebuilt stage0/stage1 smoke
checks. Stage2 still exits 4 with 86 header diagnostics and emits no IR or
compiler executable. The final audit confirms all 542 inputs of snapshot
`3e40c92a7d049b87b1e4a78a6f7d4da0edaf1fa4a85486840c1cc54662f6c0fa`
are unchanged. Evidence is in `/tmp/cinch-instance-stages/`; its compiler
source bundle is
`84f6366e1cb42bb497de6aea9574da5286eede765308e4cb3cd74dcfbc917dbd`.
The pipeline result is reused from the complete candidate validation only
after verifying the compiler/test inputs are identical; only the intentionally
verified seed pin/patch and these design records differ. These focused and
staged results do not replace the full compiler/runtime and self-built
frontend acceptance gates.

The internal proposition-type continuation now passes 33 checks over five
runtime-chosen identities, plus all 13 affected regression targets. The
535-input frozen snapshot is
`d14fa6cc5ae76e969ae12fa045aae1c6ea8c9ce3755cdf93d885a9f301009f84`;
its final source audit is unchanged. Evidence is in
`/tmp/cinch-proposition-types/`.

Source arithmetic preconditions now have checked introduction and call-site
discharge, including partial applications, conditional lambdas, recursion,
effect arrows and ignored strict results. The isolated candidate and the
integrated focused test pass 25 runtime-source cases plus nine Core
invariants. The integrated test passed in 106.25 seconds on frozen snapshot
`404dcf349f07da6a62d4dc24ed3100d22fa985344e8784c6826c2eb0a7db6a34`;
all 14 affected regression targets pass, with a final unchanged-source
audit, in `/tmp/cinch-function-preconditions-2/`. Named erased proof bindings,
imported predicates, refinements and typed runtime lowering remain required.
No new stage2 executable is established by these checks.

The fresh function-precondition staged run passes type syntax, the combined
ladder, the complete pipeline, and all six stage0/stage1 smoke checks.
Stage2 exits 4 with 86 header diagnostics, down from 91; it emits no IR or
compiler executable. Many new diagnostics are cascading failures around
unchecked reflected index calls, rather than independent missing features.
The final audit confirms the 537 inputs of snapshot
`8184038c9013b01ed9f88928600de2a618d22bfeb1ee15af6cda59542cdacf58`
are unchanged. Evidence is in `/tmp/cinch-function-preconditions-stages/`;
the compiler source bundle hash is
`6d2f482c8bda77288a07853b741c56d5cc0ebf91407e4eb79797a71e4aa8d9c4`.
Imported predicates/refinements and checked reflected calls remain required,
along with declarations and the typed Core runtime backend.

Earlier intrinsic bootstrap evidence (2026-11-01): the source dependent-binder,
strict-let alias and runtime intrinsic continuations pass their focused
checks and affected regressions. The fresh 531-input staged snapshot
`535b746836e11a6d3b785c309f1ff35cfa1b939e58325c52f43d18f6eb1d3e5d`
passes type syntax, the combined ladder, the end-to-end pipeline and all six
stage0/stage1 smoke checks. Stage2 exits 4 with 91 header diagnostics,
down from 94 before intrinsic checking and 136 before source Pi checking.
It produces no IR or compiler executable. The source audit is unchanged;
evidence is in `/tmp/cinch-intrinsic-pipeline-stages/`. Proof contexts and
refinements, declarations and the typed Core runtime backend remain
required. Earlier counts below record historical snapshots. These focused
and staged results do not replace the full-suite acceptance gate.

The bootstrap continuation is now authorized (2026-10-30). Its self-hosting
acceptance and remaining frontend/backend work are in `self-hosting.md`.
The original merge status below records the starting point. The first
completed continuation baseline and its remaining repairs are recorded
separately; self-built frontend parity is still outstanding.

Where the work stands, what comes next in order, what needs a decision
from the user, and the working rules that kept parallel agents safe. Every
claim here is recorded in more detail in `compiler/CONFORMANCE.md`
(D388-D419) and the linked design notes. No em dashes.

## 1 Where it stands

- **Memory: GC-free reference counting is on by default** (`design/reclamation.md`,
  D389-D411). Header word, small `Nat`/`Int` immediates, the tag in the
  header, pointer maps, iterative drop, Perceus insertion with every
  parameter owned, mark-on-spawn atomic counts, a leak check and a
  use-after-free debug mode. Peak memory fell 30-90x on the measured
  programs, and they end with zero live blocks. `CINCH_RECLAIM=0` turns it
  off for baselines.
- **Faults: a well-typed program does not fault** (`design/no-faults.md`,
  D401, D407-D417). Pattern coverage counts no guarded row; division takes
  a `NonZero` divisor; `Nat` subtraction needs `b <= a`; exhaustion is one
  defined `Exhausted` exit (73), with stack overflow caught by guard bands;
  byte indexing is proven in bounds; conversions carry their ranges.
  Evidence comes from every `case` branch (`if`, guards, `&&`, `||` are
  case forms), and obligations discharge by difference logic, then
  Fourier-Motzkin, each proof with a re-checked certificate.
- **The original merged failure is fixed in the working tree.** The
  2026-10-30 merge baseline was 1081 passed, 1 failed. The completed serial
  first serial continuation run had 1102 passed and two failures. Both
  subsequent repairs are now verified in the complete frozen typed-data
  run: 1105 native compiler tests and 166 runtime tests pass, with zero
  failures or ignored tests. All six stage-0/stage-1 executable smoke
  checks passed too. Those are historical schema-4 execution results.
  A later run exposed shared-cache test binaries with live source paths;
  source content audits alone do not prove frozen runtime inputs. Newer
  schema-6 changes have current regressions and require verification with
  private caches. See section 2.0 and `self-hosting.md`.
- **Design directions recorded and decided:** representation option C with
  demand-driven specialization, compiler-chosen default layouts,
  expanding-edge boxing, representations as instances of `Represent t c`
  selected with `via` (`design/layouts-and-specialization.md`, D392-D394);
  allocators (`design/allocators.md`, D397-D398); floats as one unboxed type
  with proofs in refinements and unsafe, non-implicit IEEE instances
  (`design/float-refinements.md`, D414); verified primitives and proof
  automation (`design/verified-primitives.md`, `design/proof-automation.md`,
  D395).
- **The bootstrap ladder** now separates the pinned Rust seed (stage 0),
  the seed-built Cinch compiler (stage 1), and self-builds (stages 2/3).
  A schema-4 seed and stage 1 rebuilt and passed Fibonacci, precedence and
  named-refusal executable smoke checks before the typed-core changes.
  The heap-default repair refreshed the verified seed lock. Stages 0 and 1
  have now rebuilt with the typed-data continuation; all six fresh smoke
  checks pass in the completed full gate. The latest stage-2 attempt passes
  parsing, checked-interface loading and source namespace resolution,
  then exits 4 with 136 header diagnostics on schema 6, down from 232 on
  schema 5. Counts include cascading errors rather than independent missing
  features. Regular imported constructor
  refusals are gone. The remaining nominal refusals name `NonZero` and
  `InRange`, whose proof checking remains required. Header errors still
  prevent body checking and IR emission; stage 2 has not self-built.
  `Bootstrap.Type` and `Bootstrap.Infer` implement rank-one instantiation,
  rigid variables, occurs/scope checks, let generalization, annotated
  higher kinds, regular constructors, positivity and pattern coverage.
  Module aliases, private definitions and source provenance survive
  resolution. Lexical class givens and closed kernel IO effects now have
  focused coverage; global instances, dependent checking and the shared
  runtime lowering remain required. Pure typed
  terms now retain instantiations, constructor fields, generalized lets
  and scoped private types. Source data retain complete checked constructor
  schemes and parameter kinds. They are not yet the emitter's input.
  The reference-collection stack failure is fixed, with a regression over
  more than 6,000 declarations. All six affected checker/loader/parser/
  pipeline gates and the 11 orchestration tests pass. The completed native
  baseline's two failing targets pass after focused repairs; the self-built
  frontend's full behavior gate is still required. See `self-hosting.md`.
- **Interface contracts** and the pinned seed use schema 6. The seed patch
  and source hash were intentionally verified before installation. Stages
  0/1 rebuilt and pass all six executable smoke checks. Schema 6's native region/class and Cinch loader focused
  gates pass; resolved values retain Region binders, kernel Io contexts and
  closed kernel effect rows. Source IO checking passes 15 runtime-input cases,
  and all six affected regression targets pass. The full current-tree gate
  remains required.
  Schema 5 records proof preconditions and named
  refinements are joined by declaration kinds/implicitness, full constructor
  schemes and symbols, defining identities and explicit method visibility.
  Higher-kinded function quantifiers retain their resolved kinds. Pure
  schemes now retain definition-site nominal occurrences, private kinds
  and complete constructor identity sets; nominal dependencies record
  the checked interface hashes. Schema 5 adds pure class contexts,
  declaration-order dictionary slots, private class closure and portable
  pure instance heads while retaining conditional method proof contracts.
  The producer, loader, inference, Core and imported-class focused gates
  pass; calls use scoped givens and preserve dictionary arguments in ABI
  arity. The seed patch is intentionally refreshed and verified; stages
  0/1 rebuilt and pass all six smoke checks. The fresh schema-6 stage 2 exits
  4 with 136 header diagnostics, down from schema 5's 232. IO/region/effect-arrow
  refusals are gone; dependent/proof contexts and declarations still block
  body checking. It emits no IR or executable. The complete green
  regression baseline still belongs to schema 4. Complete interface-only
  checking needs richer evidence, coherence and ABI consumption.
- **Cinch interface loading** now checks real emitted contracts and their
  dependency closure, preserving the full JSON metadata and parsing
  selected value schemes with quantifiers and proof preconditions intact.
  Loader and parser-corpus checks passed with serial code generation. The
  loader is connected to parsing, resolution and pure polymorphic nominal
  checking; complete kind/coherence/ABI checking remains required.

## 2 Next, in order

### 2.0 Fix the one failing test

The latest continuation exposed and repaired an optimizer growth-bound
violation: a whistle retained the oversized expansion, leading inference
tests to emit more than 2 GB of LLVM. The driver now retains the preceding
bounded residual and discards the rejected round's helpers and report
entries. Two private-cache snapshots pass the focused bootstrap and optimizer
gates, including the formerly failing inference runner. The intentionally
updated seed source hash is
`475b8887d71c8becfd9b572897717ac58fac3d3df5db8294c2ed3ef2bb81fcec`.
The replacement private-cache full gate has now finished on the 525-input
D28 snapshot `c314883e3b3c419e8668e14ca1c7f59231ada5d0768d10a569d3b644d33e743f`:
1,121 compiler tests, 166 runtime tests, 19 tooling tests and all six
stage0/stage1 smoke checks pass, with no failures or ignored tests and a
final unchanged-source audit. Stage2 still exits 4 with 136 header
diagnostics and produces no compiler executable. The later working-tree Pi
and contextual carrier-kind changes are not covered by that snapshot. They
now pass focused Pi, Core, effect, inference, imported-contract and
index/carrier regressions. Source dependent binders now retain actual
term/witness association in Core and pass 21 runtime source cases plus all
nine affected regression targets. A fresh staged snapshot also passes type
syntax, ladder and pipeline tests and all six stage0/stage1 smokes. Its
actual stage2 attempt exits 4 with 94 header diagnostics, down from 136;
no stage2 compiler or IR is produced. Proof predicates/refinements,
declarations and shared runtime lowering remain required. See
`self-hosting.md` for evidence.

Continuation: fixed in the working tree. Compile-time evaluation lacked
total integer widening and tried to decode `natFromI64`/`intFromI64` inputs
as mathematical integers before dispatching. The original failure, an
unsigned byte-boundary test and a staged signed/unsigned conversion test
pass in the completed full run too. That serial run finished with 1102
native compiler tests passing and two failures: `bootstrap_ladder` imports
`interface-global` from both Contracts and Nominal, and `prelude_split`
chooses a sequencing probe's scheme from unordered signature contracts.
The repairs keep one helper export home and choose the direct provider by
its canonical identity. The runtime's 165 tests and all stage-0 smoke
checks pass. A separate repair keeps an explicit heap cap on the quiet
allocation probe and checks default/fallback selection in a runtime unit
test; the runtime behavior is unchanged. The 24 affected compiler checks
and three heap unit tests pass. The runtime source edit required an
intentional seed refresh; that lock and its patched source hash are now
verified. Stages 0 and 1 have rebuilt; stage 2 still refuses the same 110
unsupported header contracts. The frozen schema-4 full gate completed on
2026-10-31 at 13:57 UTC: 1105 compiler tests, 166 runtime tests and all six
stage-0/stage-1 smoke checks pass, with zero failures or ignored tests.
The source audit confirms all 497 captured inputs are unchanged. The
snapshot SHA256 is
`fc21e0d1be1c5a53a1cd36d4b743e83c1ee38e3e6ba190f5acd4f1a319ad8dd3`.
Results and the audit are in `/tmp/cinch-typed-core-full/`. Schema-5 and
lexical dictionary changes postdate that snapshot and are being checked
separately; their first build caught a class lookup argument type mismatch,
which was repaired for its focused gate. These are historical execution
results. The later shared-cache provenance
failure means they must not be used as proof of the replacement frozen
verification setup. The schema-6 run had failed targets and was stopped
because its cached executables read live source paths. Focused repairs are
being tested before a new private-cache full run. The earlier overlapping gates were
interrupted by a machine crash. Verification now runs one command at a
time with one worker and test thread; interrupted gates are not passing
gates.

NF3b and NF5 (D418-D419) were merged with one failure outstanding:
`surface::bytes_literal_folds_and_map_literal_round_trips`. A bytes list
literal (`buf :: Bytes; buf = [1, 2, 3]`) no longer folds to a constant
buffer at compile time; the program still runs correctly. `FromList Bytes`
now takes `U8` elements (`Data/Bytes.cinch` `bytes-from-u8s`, through
`byte-of-u8`: `byte-word` then `nat-from-i64`), and `fold_constant`
(`check.rs`) returns `None` because the compile-time evaluator
(`eval_core`) fails somewhere on that path [INFERENCE: most likely a
widening intrinsic or the refined-result plumbing it does not evaluate].
Fix the evaluator (or the instance) so the literal folds again, and run
the full suite with no time limit. The same merge also changed one
diagnostic: a refinement flow that no fact discharges now
reports "the obligation `v > 5` ..." instead of "does not flow to"
(`tests/lattice.rs`), which names the failing obligation but no longer
names both types; restore the type names in the message if that is
preferred.

### 2.1 Close the remaining reachable faults

The current list is in `design/no-faults-inventory.md` ("Runtime exits
classified", reachable rows):

- `sym-intern-bytes` on non-UTF-8 bytes: give it an `Option` front door or
  a UTF-8 evidence obligation (the documented `intern :: String -> Sym`
  is already total).
- `OutputFailed` (the OS refusing a stdout write) and the IO buffer paths:
  surface them as `Result` in the IO API rather than an exit.
- The cooperative scheduler's deadlock check (`coop.rs`): decide whether a
  deadlock is a typed outcome or ruled out statically for the structured
  concurrency the language admits.
- `f64-from-decimal` on untrusted text: an `Option` front door (belongs
  with the float slice).
- Foreign code: stays behind contracts and the manifest. Bring it to the
  user if it ever needs more than that (D407: anything unprovable comes
  back for discussion).

### 2.2 Replace quiet fallbacks with proofs

These sites are total today, but rest on an invariant the checker cannot
see, so on a broken invariant they now do something quiet instead of
faulting. Each should become a proof, an own `using` obligation, or a
visible `Option`/`Result`:

- `Data.Rational.normalise` (zero gcd; needs the positive-denominator
  invariant in the `Rat` constructor or a product-of-non-zeros rule).
- `Text.Builder` overflow, `Text.Zlib` writing past the end and `out-back`
  at distance 0, `Text.Regex` program counters and repeat bounds,
  `Text.Yaml` offsets clamped to the length, `Data.Date` `civil-field`
  negatives, `Text.Lex` lengths above 2^63, `Bootstrap.Interface`
  `bytes-splice` and `nat-to-u64` residues, `Bootstrap.*` `string-stop`,
  `key-anchored`, `slice-src`.
- Behavior choices the user may veto (D417, D419): durations clamp at zero,
  reversed byte ranges are empty, `BigDecimal.div` and `Enum`'s
  `pred`/`to-enum` return `Option`, the bootstrap evaluator reports a zero
  divisor as a diagnostic.
- The `NonZero Int` `Semigroup`/`Monoid` preservation law is sample-checked,
  not proved.

### 2.3 Floats (`design/float-refinements.md`)

Tier 1 (the predicates, `decide`, IEEE-correct facts) and tier 2 (refined
operation signatures, lawful instances on `NotNaN F64`, total
float-to-integer conversion from `InRange`), then the `unsafe instance`
mechanism for the IEEE instances. Needs the user's answer in section 4.

### 2.4 Reclamation follow-ons (`design/reclamation.md` section 10)

- S6b: borrow inference (skip dup/drop pairs around calls) and drop
  specialization.
- S7: reuse of uniquely held cells (count exactly +1; a marked block at -1
  must not be reused, D409) and static dictionaries for instances without
  sub-dictionaries.
- S8: cycle teardown per D396 (region-scoped `letrec` knots, enforced memo
  regions); the knot test leaks exactly 3 blocks until then.
- Conservative `Raw` slots that leak and are not marked for atomic counts:
  memo and knot values, dictionary value slots, joins wider than four,
  packed and boxed-out boundary words, builtin index-family constructors,
  compile-time constants built without type arguments. Give each its real
  class.
- S10: wasm headers and frees. Linux verification of the stack guard and
  thread paths (written from each ABI, run only on macOS arm64).
- Allocators A1-A4 (`design/allocators.md` section 6): the in-house
  small-object allocator, the user-supplied allocator interface and the
  static pool, the `Heap r` capability with arenas, then `#[fip]` and
  `#[alloc "bounded(n)"]` for the `jpl` profile.

### 2.5 Proof automation and the kernel

- The difference-logic and Farkas certificates (`DiffCert`, the
  Fourier-Motzkin certificates) are re-checked in Rust today. Move the check
  into the dependent kernel by reflection (`design/proof-automation.md`
  section 5).
- The Cinch bootstrap parser now retains `using` predicates, refined
  results, index arguments, quantifier kinds and structured effect rows.
  Port their kind, dependent and effect checking into the Cinch checker;
  the current fragment refuses them explicitly. Parser coverage alone
  does not establish those obligations.
- `row_result` substitution in nested local definitions may pick the wrong
  names (rare; noted by the NF3b slice).

### 2.6 Representation and layouts (`design/layouts-and-specialization.md` section 8)

Value types and field unpacking, demand-driven multi-word layouts with
expanding-edge boxing, `specialize` bodies, representation instances
(`Represent t c`, `via`), the justification rule.

### 2.7 Older follow-ups still open

A list literal of 300 or more elements overflowed the CLI stack (recheck:
it should now be the `Exhausted` exit); top-level values are re-evaluated
per use (CAF); `Mode::Run` reports a signal-killed process as exit 1;
`Text.Builder` copies twice; `Eq_Bytes` could use `bytes-compare`;
`Text.Parse.render-errors` is quadratic; `Text.Json`'s duplicate-key check
is O(k^2); `Bootstrap.Ir`'s signature search is O(decls^2);
`Bootstrap.Driver` copies per byte.

## 3 Working rules that kept the agent waves safe

- **Memory.** Never run a compiled program without `CINCH_HEAP_LIMIT` (2G or
  less; `compiler/.cargo/config.toml` sets it for `cargo test`). Never raise
  a cap.
- **Cargo.** Never pipe cargo output through `head` or anything that exits
  early: SIGPIPE kills the build and leaves the old binary in place, which
  looks like a stale cache.
- **The full suite takes about 70 minutes** on this machine (over 110 test
  binaries, longer when agents run tests at the same time). Run it with no
  time limit, and after every wave, because slices verify only their own
  target lists: four waves in a row, the full suite caught a regression no
  slice had run (a test program using a now-refused form, or a typing
  change in a target outside the slice's list).
- **Gate a snapshot, not a moving tree.** When agents keep editing during a
  gate, stage the tree, `git write-tree`, commit that tree to a branch with
  `git commit-tree`, and run the suite in a worktree of that branch (`mwt
  new <branch>`); fast-forward `main` only if it is green.
  For the current read-only Git setup, use
  `python3 tooling/verify_bootstrap.py --evidence /tmp/cinch-verification-NAME`.
  Its source copy, Cargo caches and stage outputs are private to that run.
  Never symlink Cargo targets between checkouts: cached tests can retain a
  live CARGO_MANIFEST_DIR even when Cargo is invoked from the snapshot. The
  schema-6 run exposed this when its parser corpus read the new live proof
  module. The common harness now checks the embedded path against the active
  package root. The new verifier and its checks still need their first run.
- **Shared tree.** Agents never commit, branch, stash or reset; each owns a
  named region and messages the owner before touching another region; a
  slice blocked by another's in-flight edit closes out with deferred checks
  instead of waiting or fixing it.
- **Embedded and non-embedded prelude.** 55 prelude modules are embedded in
  the compiler and 38 are not; the 38 compile only through a `Main` that
  imports them (or their `lib_*` test). Programs embedded in
  `compiler/tests/*.rs` are separate again. A migration must check all
  three; `rg` over `compiler/tests` for the changed functions.
- **Agent budgets.** A slice agent stops at about 200-300 requests. Large
  migrations should delegate disjoint module sets to sub-agents from the
  start; a stopped agent can be resumed with a precise list of what
  remains.

## 4 Decisions waiting for the user

- **Float equality:** should the lawful instances on `NotNaN F64`
  (`totalOrder` equality, so `-0 /= +0`) be implicit, or must every float
  equality be selected explicitly? Also the unsafe module's name and the
  `Real64` alias names (`design/float-refinements.md` section 8).
- **The behavior choices** in section 2.2.
- **Representation details** still open in
  `design/layouts-and-specialization.md` section 9 (value type spelling,
  `Copy`-only represented element types, specialized body spelling).
