# Next steps (as of 2026-11-07)

Schema 10 is the compiled-interface format on `main`. Rust, Cinch and the
bootstrap verifier require it. The pinned library producer rebuilt all 55
library interfaces. Rebuild old compiled interfaces and their dependent
objects from source. The stricter Stage1 passed the original library-family
checks and the format cutover checks. The audited named-proof snapshot passed
the independent Stage2 and Stage3 agreement audit and the changed-source
control. The complete matching test sweep remains open.
See [the cutover checks](../bootstrap/current-schema10-family-and-cutover-results.json),
[the current bootstrap audit](../bootstrap/current-schema10-strict-named-proof-bootstrap-results.json)
and [the first schema 10 fixed point](../bootstrap/current-schema10-bootstrap-results.json).

The current published compiler source (`45bbfe94`) passed the independent
bootstrap audit. Its rebuilt Stage2 and Stage3 executables, LLVM IR and
all 46 compiler interfaces agree. The separate interface client and the
changed-source rebuild passed with the same pinned library provider. The
complete matching test suite remains open.
See [the published fixed-point audit](../bootstrap/current-schema10-published-bootstrap-results.json).

The preceding published snapshot (`0b518fbd`) passed the independent
fixed-point audit with the corrected library provider. Stage2 and Stage3
match on the executable, LLVM IR and all 46 compiler interfaces. The audit
also checked all 55 library interfaces, a separate compiled-interface
client and a changed-source rebuild.
See [the preceding published audit](../bootstrap/current-schema10-preceding-published-bootstrap-results.json).

The staged compiler links the pinned standard-library object and runtime
archive. It reads the library's checked interfaces separately. The executable
source-rebuild checks verify interface dependencies and refuse changed bodies
when only the old object is available. The Foldable and Category fixtures now write only a client module. All 15
Foldable and 12 Category cases passed through Native. All 26 executed
programs reported zero leaks. Both fixtures use the shared frontend harness
for the pending Stage3 replay.
See [the Native fixture audit](../bootstrap/current-schema10-source-free-library-fixtures-results.json).

The development Cinch Stage1 passed all 15 original Foldable cases
through the shared frontend harness. Every fixture supplied only its
client source. The checked object added the requested `non-empty [O]`
specialization, preserved all 55 library interface identities and ran
each program with zero leaks. Matching Stage3 coverage remains open.
See [the compiled Foldable audit](../bootstrap/current-schema10-original-foldable-object-results.json).

The development Cinch dictionary ABI repair passed all 12 original
Category cases on that same object. Eleven programs ran with zero leaks.
The refusal test exercised both original visitor diagnostics. The
matching Stage3 library replay remains open.
See [the compiled Category audit](../bootstrap/current-schema10-original-category-object-results.json).

The published Stage3 and Rust both passed the two original fixity tests.
The project case preserves the declaring operator through a renamed
facade and checks the fixities and value identities in the emitted
interfaces. The tests use the selected frontend when configured.
See [the fixity dispatch audit](../bootstrap/current-schema10-stage3-fixity-dispatch-results.json).

Rust and the development Cinch Stage1 both passed all three original
command-line argument tests. The Cinch executable linked each direct CLI
client with the checked standard-library object and runtime archive. All
four original argument vectors reached it, including the word containing
a space and the flag-like words following `--`. Matching Stage3 coverage
remains open.
See [the argv audit](../bootstrap/current-schema10-compiled-argv-results.json).

The refinement erasure candidate passed eight checks on development Stage1.
Both original source cases and both additional runtime controls ran without
leaks. Four invalid constructions were refused. The LLVM for refined and
plain list construction agrees after renaming local values and labels.
Returning a refined parameter through `unrefine` transfers the existing
owned value. The combined compiler rebuild remains open.
See [the refinement erasure audit](../bootstrap/current-schema10-refinement-erasure-results.json).


The pinned
library object preserves all 55 checked interfaces and includes
`both [TTT]` and `swap [TT]`. All nine
Function.Combinators cases passed against that object with zero leaks.
See [the compiled consumer audit](../bootstrap/current-schema10-compiled-library-consumer-results.json).
The package design calls for materializing missing variants from checked
polymorphic IR and caching them by representation vector.
See [the specialization design](layouts-and-specialization.md#2-words-specialization-over-three-representation-classes-option-c).

The runtime assigns Unit's word representation to private, unconstrained
type variables after Core has checked their scope. The original `Result`
client now requests the concrete `from-result [RT]` variant. The expanded
object preserves the two required Alternative method vectors and all 55
checked interface identities. All 23 original Option, Result, function, Map
and Set cases ran through the repaired Stage1 with zero leaks.
See [the selected runtime audit](../bootstrap/current-schema10-private-runtime-library-results.json).

Native now isolates kind inference within each class method and law, while
combining the enclosing class parameter uses. The original Traversable
`sequence` contract emits `f :: Type -> Type`. All 205 Native library
tests and 24 higher-kinded tests passed. The rebuilt pinned library
passed readback of all 55 interfaces and retained every requested object
root. The original List-to-Option traversal progressed from a missing
declaring class to a representation error on the earlier Stage3. The
complete traversal replay with higher-kinded runtime support is pending.
See [the Native method kind audit](../bootstrap/current-native-scoped-method-kind-results.json)
and [the rebuilt library audit](../bootstrap/current-schema10-scoped-member-kind-provider-results.json).

The finite law sampler checks the original Const identity law on two
constructor samples. It also checks an independent identity implementation
and detects a deliberate counterexample. Stage3 agreed with Stage1 on both
checked interfaces and the refusal. The audited ladder used the preceding
pinned provider and matched Stage2 and Stage3 on the executable, LLVM IR and
all 46 compiler interfaces. The separate consumer and changed-source
rebuild passed. Rechecking with the new producer and completing the
full suite remain open. The exact published combination has started a fresh
four-stage rebuild with the corrected producer.
See [the quantified law audit](../bootstrap/current-schema10-quantified-law-bootstrap-results.json).

The pure function constructor now unifies with an inferred effect row
only after the row resolves to empty. All four effect-type runs passed,
including real-effect refusals. The runtime preserves the type former's
word class in specialization vectors. The original function-family
program ran against the expanded object with zero leaks. String equality
now uses the authenticated private wrapper layout. Five focused client
runs passed with zero leaks, including the original Foldable list
combinator case. The expanded object retains all 55 checked library
interface identities. These runs used the recorded development compiler
snapshots. The full matching suite remains open.
See [the function and String audit](../bootstrap/current-schema10-function-and-string-object-results.json).

Set `CINCH_TEST_INVOCATIONS` to an evidence directory when running the
selected frontend tests. Each record identifies the test case, selected
compiler, input files and observed compiler exit code. It survives project
cleanup. All eight harness tests passed, including ten actual invocations
from the six tests that select a frontend. See [the harness audit](../bootstrap/current-selected-frontend-invocation-results.json).

Native `run` now creates a fresh scratch directory and removes it after
the executable exits. The CLI and both library run helpers passed the
cleanup checks. The explicit build and linker failure checks preserved
their artifacts. See [the scratch audit](../bootstrap/current-native-run-scratch-results.json).

Schema 10 conditional method rows now require the checked implementation
declaration. The producer omits optional bodies when they are unavailable.
The Rust reader, Cinch loader and bootstrap verifier refuse display-only rows,
null bodies and empty body tables. Thirteen compiler tests and 43 tooling
tests passed. Two previously built interface closures kept all their
recorded identities under the stricter verifier. A fresh ladder and full
suite remain separate checks.
See [the format consolidation checks](../bootstrap/current-schema10-single-instance-format-results.json).

The Stage1 probe with a faster diagnostic renderer refused the original
Stage2 source and library inputs. The pinned producer wrote an unqualified
`InRange` in the `bytes-len` intrinsic record and the defining qualified name
in the exported value. Stage1 requires the two declarations to agree. The
Rust repair uses the checked exported value's spelling for both records.
The rebuilt provider passed readback of all 55 interfaces. The 14 changed
identities follow from the corrected `Data.Bytes` contract and its dependants.
The library object, LLVM IR and runtime archive are byte-identical. Stage1
accepted a separate client using the renamed observation in a proof. Stage2
then built all 46 compiler modules and passed its smoke tests. Stage3 rebuilt
the same sources and matches Stage2 on the executable, LLVM IR and all 46
compiler interfaces. Stage3 passed its smoke tests and a separate client.
The complete byte-observation target passed all three tests, including the
renamed facade and forged implementation controls. The changed-source control
also passed. An independent audit checked both artifact closures and confirmed
that the rebuilt compiler uses the changed source. The complete suite is
running separately.
See [the Stage2 refusal and exact inputs](../bootstrap/current-schema10-stage2-byte-observation-results.json)
[the Stage3 agreement audit](../bootstrap/current-schema10-repaired-stage3-agreement-results.json)
[the full byte-observation target](../bootstrap/current-schema10-byte-observation-full-target-results.json)
and [the independent bootstrap audit](../bootstrap/current-schema10-repaired-bootstrap-results.json).

The development checker passed the complete Core proof test with an erased
`refl` witness for `Eq`. The added checks reject false equalities,
comparison-only propositions, mismatched goals and runtime use. The source
pipeline emitted IR for a valid equality and refused an unequal one. Its
comparison-only case reached the simpler checker and missed the intended
diagnostic. The corrected development compiler routes predicate syntax
through typed checking and sends named proof declarations to the dedicated
proof pass. The unchanged valid Proof example ran without leaks and the
invalid example reported `obligation disproved` at its original source
location. Both original pipeline tests passed on that development snapshot.
A separate four-stage ladder using the consolidated library and named proofs
passed the independent fixed-point, consumer and changed-source audit.
The named-proof implementation and consolidated producer are now integrated
with the stricter interface reader. The combined Stage2 and Stage3 match on
the executable, LLVM IR and all 46 compiler interfaces. Both independent
interface readers agreed on the closures, and the separate client and
changed-source checks passed. The earlier audited named-proof Stage3 also
ran both unchanged original Proof examples. The valid program ran with zero leaks at
32 MiB and the invalid proof reported the expected obligation at line 4,
column 22.
See [the Core proof replay](../bootstrap/current-schema10-reflexivity-core-results.json)
[the original Proof example audit](../bootstrap/current-schema10-original-proof-replay-results.json)
[the consolidated ladder audit](../bootstrap/current-schema10-consolidated-proof-bootstrap-results.json)
and [the original Proof replay on Stage3](../bootstrap/current-schema10-consolidated-stage3-proof-results.json).

The repaired Stage3 ran all 34 original language cases. Nine of thirteen
valid programs ran at 32 MiB with poisoned allocations and zero leaks.
All 21 invalid programs were refused, and eight refusals met the original
reason and source-location assertions. Proof, Protocol, Tactics and
Choreography remain open on that exact compiler. Named proof support and
the finite-index tactic work are tracked on separate source snapshots.
See [the repaired language-suite audit](../bootstrap/current-schema10-repaired-external-language-results.json).

The dependent constructor candidate opens the index binder using the same
checked substitution as function arguments. The original valid Tactics
program now runs with zero leaks at 32 MiB. Both proof-search refusals and
the original Html and private Tensor checks passed. The stale-evidence
fixture still stops at the unresolved `refSet` operation.
See [the original tactic replay](../bootstrap/current-schema10-original-tactics-results.json)
and [the constructor and diagnostic replay](../bootstrap/current-schema10-dependent-pattern-and-diagnostics-results.json).

The separate library targets exposed missing runtime specializations and
conflicts between copied Category and Foldable sources and loaded interfaces.
The test library rebuild takes its specialization requests from the compiler's
retained diagnostics. The first expanded object preserved all 55 interface
identities. The completed sweep exposed four additional HashMap variants, so
the replay waits for the object containing those requests too.

The executable replacement candidate authenticates the copied module bytes
and rechecks their interfaces before linking the library object. The Core
control and original Algebra program ran. The edited and appended source
controls were refused with empty IR. Category differs only on the
`Data.Const` map-identity law observation. Foldable needs the checked
`Data.Bool.not` declaration, whose complete dependency closure leads back
to Foldable. A separate candidate projects the checked value into the
copied implementation scope while retaining the interface checks.
See [the copied-module replay](../bootstrap/current-schema10-executable-source-copy-results.json).

The additional library sweep found 29 more missing List function variants.
The expanded candidate object built with the current pinned producer and
preserved all 55 interface identities. All eight original List cases then
ran on the earlier repaired Stage3 with zero leaks.
See [the original List replay](../bootstrap/current-schema10-expanded-list-results.json).

The original small Nat/Int tests exposed a local numeric inference failure.
The map value defaulted to Int before the body of its let could constrain it
to Nat. The candidate keeps unresolved literal carriers shared with their
uses and validates them when the enclosing definition closes. The
original map-key fixture and both literal controls passed checking.
The complete runtime group remains open.
See [the local numeric checks](../bootstrap/current-schema10-local-numeric-check-results.json).

The UTF-8 diagnostic renderer passed both original pipeline tests when
replayed with the matching debug and release runtime archives. The stricter
compiler and producer passed 45 interface, array and library-object tests
and source admission for all 46 compiler modules. A separate law-identity
candidate passed 14 tests across law checks, source instances, the pipeline
and the original array case with a custom build directory.
See [the renderer audit](../bootstrap/current-schema10-diagnostic-renderer-results.json)
and [the scoped interface and law audits](../bootstrap/current-schema10-interface-and-law-suite-results.json).

The Rust checker now follows task captures through local closure aliases
and direct calls to checked forwarding helpers. All 19 concurrency tests,
187 compiler unit tests and 15 array tests passed. In the five paired CLI
cases, the fix refused both hidden mutable captures and preserved the other
three outcomes. The matching Cinch Stage3 replay remains a separate check.
See [the Rust task-capture audit](../bootstrap/current-schema10-native-task-capture-results.json).

String `unpack` now advances by the decoded byte width. All 19 string and
string-I/O tests passed, including a 16,384-character mixed-width regression.
In the 4,096-character paired runtime probe, elapsed time fell from 17.8
seconds to 0.26 seconds. The running ladder retains its pinned library inputs.
See [the string tests and timing measurements](../bootstrap/current-schema10-linear-string-results.json).

The consolidated producer combines the byte-observation, task-capture and
string fixes with the library variants requested by the original examples.
It reproduced all 55 repaired interface identities and passed the separate
Stage1 client. On the same Native inputs, all 171 runtime tests, 42 tooling
tests and the eight allocation tests passed. The complete 235-target Rust
sweep and the consolidated ladder with named proofs are running separately.
See [the consolidated provider and scoped suites](../bootstrap/current-schema10-consolidated-provider-results.json).

The schema 9 chain remains a historical checkpoint. Its four stages and
smoke tests passed, including the Stage2 and Stage3 fixed point, separate
interface consumer and changed-source test.
See [the schema 9 audit](../bootstrap/current-schema9-bootstrap-results.json).

The 105-target Stage3 run reproduced the unchanged Ordering replacement
failure. `Relation.Ord` rejects the stale dependency on `Data.Ordering` after
Ordering inherits `mconcat`. Its interface still differs from the pinned producer
in instance names, the retained `Data.Core` dependency, and law observations.
The defining-nominal repair passed the focused imported-default regression
and the whole interface writer group. Class-law declarations still need to
travel from their owners to separate instance checks.

The schema 9 candidate passed the complete original pipeline target plus
the new finite-constructor laws. It checked 27 associativity samples and
three identity samples, then rejected the wrong identity without emitting an
interface. It also passed the original inherited-default and library
replacement assertions in that target. The separate source-instance group
also passed, and the matching Stage3 replay remains open.
See [the pipeline receipt](../bootstrap/current-schema9-finite-law-pipeline-results.json).

The schema 10 candidate carries the checked law equations in the same
interface as the class methods. Rust passed all 18 law, interface-kind and
method-body tests. The separate Cinch consumer reached rechecking and refused
a named law argument because the decoder reconstructed the wrong terminator.
The corrected decoder parses. Its source-instance replay will also check
Nat-backed and generic constructor fields with lawful and unlawful examples.

Rust now supplies typed `Nat` and `Int` samples and evaluates arithmetic
on them directly in law equations. The tests check the lawful identity and
associativity samples and reject both the wrong Nat-backed identity and an
incorrect arithmetic law. The schema 10 method rows now use the checked
type spelling that Cinch already writes, which addresses the Functor
interface difference. All 21 tests in the four affected Rust targets
passed across the original run and an exact replay of the law target.
See [the Rust contract receipt](../bootstrap/current-native-numeric-and-class-method-contract-results.json).

The schema 10 Stage1 compiler matched the original Core, Ordering, and
Functor interfaces. The unchanged replacement test now reaches
Applicative. A separate diagnostic run isolated five value spellings that
differed while their checked schemes agreed. The other Algebra checks found
missing support for division and an early dependency on the Boolean facade.

The combined law, interface and staging snapshot passed the pinned Rust
seed's source check on all 46 modules. Its normal Git-pinned library
reproduced all 55 interfaces and the development object byte for byte.
Stage1 compiled all 46 modules into Stage2. Stage2 ran Fibonacci and the
precedence program and rejected the unknown-name control. The audit checked
the dependency identities across the 46 compiler and 55 library interfaces.
Stage3 passed its smoke tests. Stage2 and Stage3 agree on the executable,
LLVM IR and all 46 compiler interfaces. Stage3 checked a separate client
using its own emitted interfaces. Stage2 rebuilt the changed source and
produced the changed diagnostic. The independent audit confirmed acceptance
checks 1 through 4. The combined Stage3 also passed all 14 original external
acceptance checks, including both imported-fixity redeclarations.
See [the audited inputs and Stage1 results](../bootstrap/current-schema10-stage1-and-integrated-input-results.json).
See [the Stage2 and source-law audit](../bootstrap/current-schema10-stage2-and-source-laws-results.json).
See [the Stage3 and library audit](../bootstrap/current-schema10-stage3-and-library-replays-results.json).
See [the complete bootstrap audit](../bootstrap/current-schema10-bootstrap-results.json)
and [the external acceptance audit](../bootstrap/current-schema10-external-acceptance-results.json).

The separate schema 9 finite-law candidate also passed all three
source-instance tests. The newer schema 10 work preserves positional
proof names and flattened type applications in public value signatures.
The checked contract still retains its scoped binders and canonical
applications. The updated readers and writers are under regression tests.
The matching pinned library passed its 55-interface audit. All five Rust
Applicative display signatures match the earlier Cinch output. The stricter
Stage1 passed the original Core, Ordering, Category and Algebra replacement
test. Rust, Cinch and Python accepted the schema 10 control and rejected an
actual schema 9 file, fingerprint-only defaults and two malformed contracts.
See [the original family and format checks](../bootstrap/current-schema10-family-and-cutover-results.json).
The full 105-target Stage3 matrix is split into three groups of 35. Each
group waits for the same audited Stage3, then takes a shared worker slot
for one target at a time.

The staged declaration pipeline checked and emitted a generated `Nat` value
through typed IR. The linked program returned 0 with zero leaks. The unchanged
array input refuses `arrayReplicate` at its original source location.
All 15 original array cases passed on the Stage1 candidate with staging and
CopyOut. Fourteen passed in the complete run. The remaining case passed
on an exact retry after supplying the matching runtime archive. The
matching Stage3 replay remains open. See [the arrays receipt](../bootstrap/current-stage1-combined-arrays-results.json).
All 15 original Rust array tests now also pass with a custom Cargo output
directory. The driver builds the runtime archive in the directory its linker
reads. The earlier ten archive-lookup failures are preserved in the audit.
The newer Cinch source-instance run passed the comparison of qualified law
names and its recursive sampling control. See [the replay results](../bootstrap/current-schema10-arrays-and-law-identity-results.json).

The published Stage3 passed all eight allocation-accounting tests when
linked with the library object that provides `Data.List.length@T`.
The original run passed five and refused three builds at that missing
variant. The replay used the same compiler, test binary, manifest and
runtime, with the same 55 interface identities.
See [the allocation replay](../bootstrap/current-stage3-allocation-object-replay-results.json).
With the same replacement object, the published Stage3 also passes both
original body-binding tests and the structural NonZero test. The three
tests previously stopped at the missing `Data.List.FromList_List@R` factory.

The normal Git-pinned schema 9 provider rebuilt and verified all 55 library
interfaces. It includes the raw `list-length` variant requested when the
development Stage2 stopped. The failed attempt still emitted 44 checked
compiler interfaces, which the independent Rust reader validated with their
dependencies. The matching original library replacement suite remains open.
See `bootstrap/current-schema9-pinned-provider-results.json`.

The checked compile-time evaluator returns 42 through a stage function,
refuses the inner array primitive by its checked name, and enforces a shared
work budget. The complete pipeline and original arrays group still need their
matching results. See `bootstrap/current-staging-evaluation-results.json`.

The staging parser now retains stage signatures and equations, generator
blocks, declaration quotations and expression splices. The complete
syntax target passed with zero leaks. Checked compile-time execution and
the original array primitive refusal remain open. See
`bootstrap/current-staging-surface-results.json`.

The Cinch interface writer emits the same public signature type and layout
rows as Rust for the defining module and its re-export. The complete writer
test also confirms that an instance uses the instance table. Category source
replacement still needs matching instance heads and display schemes. See
`bootstrap/current-class-interface-exports-results.json`.

The checked `copyOut` development patch passes both complete pipeline
tests and the full runtime IR group. It checks Copy evidence for elements
of an immutable array and retains the original refusal for affine
`Thunk` elements. The tuple runtime case returns 42 with zero leaks.
The patch reapplies to the integrated base without fuzz. The combined
Stage3 still needs its own replay. See
`bootstrap/current-copy-out-development-results.json`.

## Development checkpoint for function constructors and class-law observations

The structural function constructor candidate `b655a0a0` adds kind-checked
`(->)`, supports partial application in instance heads, and rejects
overapplication. The complete pipeline, type-syntax, and source-instance groups
passed six tests. Apply `bootstrap/structural-arrow-development.patch` to the
integrated task/law source `5bf127d9`.

The class-law observation candidate `390867a9` builds on that change. It records
two checked Boolean samples for a lawful instance, refuses an actual Boolean
counterexample, and records an unchecked observation for an unsampled function
carrier. The complete pipeline and interface-writer groups passed three tests.
The two patches reapplied without offsets or fuzz and left the seed and library
pins intact. The original source-library families, broader sample generation,
imported law transport, and matching Stage3 verification remain open. The
receipts are `bootstrap/current-structural-arrow-results.json` and
`bootstrap/current-class-law-observations-results.json`.

## Development checkpoint: checked defaults, coverage and structural refinements (2026-11-07)

The published compiler remains Cinch3268 with Native8ba8. Immutable Native22
seed/library pins and defining-module-only fixities remain unchanged. The
reviewable development patches below are not applied to the published compiler.

Private28d2 fixes inferred constructor-family coverage without using branch-local
index equations to eliminate other constructors. Both exact44 source gates and
all8 complete component groups (10 Rust tests) and all9 full compiler/object tests
pass independent audit, including17 C clients and
50 zero-leak reports. All11 positive default runtime clients execute their expected
results at32M with poisoning and zero leaks; the original owned Box counterexample
now runs7. Five invalid-source controls refuse. Strict cycles still refuse during
backend lowering with empty IR. Their fresh interfaces match successful
interface-only output byte for byte and all57 contracts pass the independent
reader; the new runner's no-interface-after-backend-refusal assertion was mistaken.
The raw failure remains retained. See
the three `bootstrap/current-inferred-case-coverage-*-results.json` receipts.

Privatea029 preserves structural NonZero while retaining the designed widening
of comparison-only refinements. Eleven self-contained source controls agree
between actual compiled Cinch and Native. Its complete9-test compiler/object gate
also passes17 C clients and50 zero-leak reports. All8 complete component groups
(10 Rust tests) pass independent audit. The original API/default replay is still
16/21: three diagnostic/path assertions, source-Prelude replacement and the
original mconcat runtime remain failures. The runtime now gets past inferred
containers but exposes generic-call result inference forcing NonZero to Int.
Candidate2866 preserves a structural expected result when hinting a call, while
comparison-only refinements still hint their container. All17 Native controls
and the complete original Cinch NamedRefinements target pass. The full44-module
compiler/object gate also passes independent audit:9 tests,17 C clients and50
zero-leak reports. All8 complete component groups (10 Rust tests) also pass independent audit; the
original runtime replay remains required. See
`bootstrap/current-refinement-expected-result-component-results.json`. See
`bootstrap/current-refinement-expected-result-compiler-results.json`. Its patch and
evidence are in `bootstrap/current-refinement-expected-result-development.json`. The
superseded broad range-preservation assumption and subsequent fixture/cache-guard
setup errors remain recorded; no original assertion or cache guard was weakened.
See `bootstrap/current-refinement-structural-development.json` and
`bootstrap/current-refinement-structural-compiler-results.json`.

Private7ccd retains source instance/class names and declaration locations; its
complete source-instance and resolver targets pass independent audit (4 Rust
tests, all16 default controls). Source-origin candidate14a8 appends actual file
and local-line labels already supplied by project bundles, preserving the phase
and bundled position. Seven renderer controls pass. The first candidate's
reserved-word parse error and the second run's inherited stale FFI diagnostic
assertions remain recorded. The third complete pipeline/resolver/source-instance
run retains the already-published fa405 FFI diagnostic correction, its positive
ABI control and all original refusal sources. All3 complete groups (6 Rust tests)
now pass independent audit; see
`bootstrap/current-source-origin-component-results.json`. Native accepts all44 modules;
earlier self-built4f refuses one guarded Nat subtraction and two pattern-coverage
checks in the new helper. Cut-down differential probes are queued. The latest
actual14a8 compiler now passes all9 complete compiler/object tests,17 C clients
and50 zero-leak reports; see `bootstrap/current-source-origin-compiler-results.json`.
Its original API/default replay now passes19/21: all three original diagnostic
name/file assertions pass, defaults6/7, with source-library replacement and the
unchanged mconcat refinement runtime still failing. Its own source-admission
check and matching Stage1/2/3 build remain required. See
`bootstrap/current-source-origin-api-results.json`. See
`bootstrap/current-source-origin-input-results.json`. No filename is guessed from a module name. See
`bootstrap/current-source-default-diagnostic-component-results.json` and
`bootstrap/current-source-default-origins-development-2.json`.

The2866 original API/default replay stays19/21, with an important failure-stage
change: mconcat now passes checking and reaches runtime-object lowering. Its
checked Semigroup_Function and Monoid_Function OO factories are absent from the
old test library object. A fresh object from the unchanged pinnedNative22
library-only producer supplies both, with ten physical symbols checked and all55
interface files plus runtime bytes unchanged. The unchanged replay with that
object passes 20 of 21 tests, including all seven class-default tests. The
original function-carrier, refined product and empty-unit program exits zero
with a 32 MiB heap, allocation poisoning and zero leaks. The two checked
factory roots are now declared in the test provider. Source-library replacement
is the remaining replay failure. See
`bootstrap/current-refinement-expected-result-api-results.json` and
`bootstrap/current-function-default-library-provider-results.json`. See also
`bootstrap/current-function-default-api-results.json`.

The diagnostic byte scanner passes the full pipeline group, including the
original diagnostic-origin controls, a two-megabyte Unicode bundle, and invalid
UTF-8. The public constructor formatter also passes both complete exporter and
nominal groups, with the Native `Parcel` comparison and original resolved and
hidden constructor assertions. Base-family compatibility and source replacement
still have their own tests. See `bootstrap/current-source-origin-byte-scan-results.json`
and `bootstrap/current-constructor-public-display-results.json`.

The Rust development patch rejects the original unsafe local callback and
unsafe strict and nested aliases. Safe local callbacks still check. All
18 concurrency tests and 186 Rust units pass, and the original helper-wrapped
capture still needs a checked contract at the call boundary. See
`bootstrap/current-native-local-task-capture-results.json`.

Candidate32ab repairs a Core branch-implication gap: a true conjunction and a
false disjunction retain valid implications from the selected operand even if
the test operand is opaque. Exact condition readings remain unchanged, and
partial implications are never negated for an opposite branch. Nine new controls
cover operand order, nested guards, wrong identities, wrong branches and scope
escape. The first fixtures used the reserved opaque parameter and failed
parsing; these raw failures are retained and negative parser refusals grant no
semantic credit. All9 corrected flag controls pass independent Native audit:
5 valid programs and4 actual ordering/precondition refusals. The whole original Cinch precondition target now passes with all9 added controls
and every original assertion retained. All 44 exact compiler modules also pass
Native admission. The pinned Native builder produced the candidate Cinch CLI,
which passed all nine compiler and object tests. The independent artifact audit
confirmed 17 foreign ABI clients and 50 zero-leak reports. The candidate also
checked every one of its own 44 exact source modules with an empty executable
search path, the 55 verified library interfaces, and zero leaks. All nine
complete component groups pass, with 11 Rust tests exercising the Cinch
components. The fresh rebuild passed Stage0 and Stage1 smoke checks. The Stage1
audit matches all 44 source and interface identities, the pinned builder,
and the exact executable. Stage1 emitted all 44 checked Stage2 interfaces,
then stopped in code generation because the library object lacked the
`Data.List.list-length` variant for raw elements. A fresh object built with
that exact root retains all 55 interface files and the runtime archive bytes.
Stage1 used that object to rebuild all 44 Cinch modules into runnable Stage2.
Stage2 passed the Fibonacci, precedence and named-refusal smoke tests. The
independent receipt verifies the 44 interface files, exact inputs, generated
IR and executable hashes. Stage3's first run at 256 MiB emitted all 44
interfaces byte for byte with Stage2 and then exhausted its heap before
writing LLVM. With a 512 MiB cap in one worker, the same inputs produced a
runnable Stage3. Its executable, LLVM and 44 canonical interfaces match
Stage2 byte for byte. All Stage3 smoke tests and the interface-only compiler
entry check pass. The changed-source rebuild also passed. The new compiler
produced the intended changed diagnostic and the independent audit verified
all four bootstrap requirements for the 32ab candidate. The full test and
frontend parity requirement stays open. See
`bootstrap/current-partial-boolean-chain-results.json`,
`bootstrap/current-partial-boolean-stage3-output-results.json`,
`bootstrap/current-partial-boolean-stage3-heap-results.json` and
`bootstrap/current-partial-boolean-stage2-results.json`. See also
`bootstrap/current-partial-boolean-precondition-results.json` and
`bootstrap/current-partial-boolean-input-results.json`. See also
`bootstrap/current-partial-boolean-native-controls.json`,
`bootstrap/current-partial-boolean-compiler-results.json`,
`bootstrap/current-partial-boolean-own-source-results.json`,
`bootstrap/current-partial-boolean-component-results.json` and
`bootstrap/current-partial-boolean-branch-development-2.json`.
See also `bootstrap/current-partial-boolean-stage1-results.json`,
`bootstrap/current-partial-boolean-stage2-demand.json` and
`bootstrap/current-partial-boolean-stage2-provider-results.json`.

The current 32ab Stage3 also passed all 14 original external checks, including
the two poisoned 32 MiB programs and both defining-module fixity controls.
Across the separate 34 language cases, eight of 13 valid programs ran,
all 21 invalid programs refused, and three matched the required reason and
source location. The case-level results keep every failure visible. See
`bootstrap/current-partial-boolean-external-acceptance-results.json` and
`bootstrap/current-partial-boolean-stage3-language-results.json`.

The complete earlier 4f Stage3 sweep finished with 102 of 105 targets passing.
The four failing assertions are in arrays, the bootstrap pipeline and the
source-family override test. The 105 original target results are independently
audited in `bootstrap/earlier-stage3-frontend-suite-results.json`. The current
32ab compiler requires its own Stage3 sweep.

The complete source-effects target passes all 43 cases, including a foreign
call whose declared row omits `Blocking`. Its diagnostic now retains both
checked rows. The original concurrency replay remains a separate gate. See
`bootstrap/current-task-effect-diagnostic-results.json`.

Task ownership refusals now carry the specific checked reason. All nine
original task controls refuse with empty IR, including seven assertions
for the exact task-use, scope escape and sharing reasons. The same complete
runtime-IR test executable passes after adding the missing shared runtime
fixture. See `bootstrap/current-task-trace-diagnostic-results.json`.

The combined development build also distinguishes a dropped handle from a
duplicated or otherwise invalid use. The original runtime-IR target passes
with the two drop cases requiring the join diagnostic. The original
`Tasks` program and helper capture checks remain a separate replay. See
`bootstrap/current-task-drop-diagnostic-results.json`.

Both development compilers now check the actual callback and scope for
a saturated source helper whose entire body forwards to the task kernel.
The Cinch build passes six controls, including the unchanged Tasks runtime
and refusal examples, the unsafe Bytes capture, and two helper escape
checks. The Rust build passes the original unsafe capture regression,
all 19 concurrency tests and 186 unit tests. Separately compiled helpers
still require a portable transfer contract. The complete matching Stage3
and Rust sweeps remain separate. See
`bootstrap/current-cinch-task-source-forwarder-results.json` and
`bootstrap/current-native-task-source-forwarder-results.json`.

All eleven follow-up patches reapply without fuzz over the verified 32ab
compiler, retaining the typed FFI test and both producer pins. The combined
44-module source has a new identity and requires a fresh bootstrap. On the
unchanged 32ab executable, both original promotion pipeline tests pass with
the corrected test-library object and a 512 MiB heap. The otherwise matching
256 MiB run passed diagnostic origins and exhausted its heap during self-build.
See `bootstrap/current-task-law-bootstrap-integration-results.json` and
`bootstrap/current-stage3-promotion-pipeline-512-results.json`.

The exact verified 32ab Stage3 admitted all 44 combined modules in one
check with empty diagnostics. The fresh pinned Stage0 through Stage3 run
on the frozen integrated source `5bf127d9` passed independent audit.
Stage2 and Stage3 produced identical executables, LLVM IR and all 44
interfaces. The changed compiler source reached the consumer, and Stage3
checked a separate interface consumer. The complete Stage3 suite is
running. Its original library replacement test gets through Core and
stops when Ordering needs the inherited `mconcat` implementation. See
`bootstrap/current-task-law-bootstrap-chain-results.json`.

The corrected anonymous refinement fixture passed all 18 cases on the
audited 32ab Stage3. It checks that an explicit `unrefine` returns the
carrier, a bare structural result refuses, and the projected value still
cannot prove a stronger predicate. The fixture patch applies to the
integrated test source and needs its matching replay. See
`bootstrap/current-structural-projection-stage3-results.json`.

The base constructor formatter matches the complete `Data.Core` interface
hash against the Native writer and matches the three isolated `Data.Ordering`
constructor contracts. A source-defined `Shadow.List` keeps its own identity.
Both exporter and nominal groups pass. See
`bootstrap/current-constructor-base-display-results.json`.

The source replacement pipeline now checks a base source home against its
dependency closure, emits a fresh interface, and validates every retained
cached dependency hash before checking its consumers. The complete pipeline
passes, including stale-cache and ordinary-interface collision refusals. The
unchanged original family test passes `Data.Core` and reaches `Data.Ordering`'s
omitted `mconcat` default. Category, Algebra, and source replacement in runtime
object builds remain open. See
`bootstrap/current-source-interface-overlay-results.json`.

The early-library scope repair exposes facade names backed by a checked
defining interface in the dependency closure. The exact defining hash is
recorded without a Prelude back-edge. Explicit and implicit Data.Core
references produce equal Data.Set contracts, and a qualified reference
without an import refuses. Both complete pipeline tests pass. The unchanged
Algebra source now reaches its class law without missing constructor names.
See `bootstrap/current-source-overlay-base-scope-results.json` and the
reapplied patch receipt `bootstrap/current-source-overlay-base-scope-pipeline-results.json`.

The class-law parser retains quantifiers, arrow binders and the two term
operands. The law carrier patch opens the class dictionary and binders in the
ordinary checker, then checks both operands against one carrier with an
empty effect row. The complete syntax and pipeline tests pass. A mismatched
carrier produces a type refusal, and a well-typed law still awaits checked
instance evidence. The original Category syntax probe reaches the structural
function instance. See `bootstrap/current-source-class-law-carrier-results.json`.
The Boolean sampler also passed both actual instance checks. It enumerates
both Boolean values through the selected checked method slots and reports a
named counterexample for a constant-false implementation. Lawful samples
remain internal until checked law transport is available. See
`bootstrap/current-source-class-law-instance-samples-results.json`.

A separate development compiler preserves the checked index mismatch and
unknown record field reasons. The original Query, Audio, Circuit and
TensorADShape negatives produce the expected reasons at their original
source locations. The pipeline and typed-index tests pass. The original
Stage3 acceptance receipt retains its four failures until the repairs run
on a matching Stage3. See
`bootstrap/current-external-refusal-diagnostics-results.json`.

All9 development patches apply in order to a fresh published-source snapshot,
produce the exact32ab compiler-module bytes and preserve the published FFI test
correction. This is reproducibility evidence, not promotion or bootstrap proof:
`bootstrap/current-development-patch-chain-results.json`.

The current Native8ba8/Cinch3268 Rust sweep is complete. All 235 targets and
1,647 tests pass without skips. The interrupted run's completed results were
preserved, and the eight unfinished targets passed after resumption. The
original arrays executable also passed all 15 tests once its runtime fixture
was built. See `bootstrap/current-native-full-suite-results.json`. The
matching Cinch-built Stage3 suite is tracked separately.

Current Native8ba8/Cinch3268 also passes all43 tooling and39 syntax/editor tests
under independent audit, with no skipped tests. These preserve their Native
editor integration role and do not replace the separate final Cinch-built
language/frontend checks. See `bootstrap/current-native-tooling-editor-results.json`.

The fresh full235-target Native8ba8/Cinch3268 sweep retains every original target
and assertion, with shared worker leases, one codegen unit/test thread per job,
and checked optimized runtime support. Its missing-runtime-fixture array failure
is preserved; the identical binary subsequently passes all15 original array tests.
The older NativeE5/4f complete sweep independently records234/235 groups and
1635/1636 tests, retaining its stale pipeline-diagnostic assertion failure. Earlier
results do not close current full-suite acceptance.

A partial review covers 11 of 130 targets outside the historical 105-target
frontend matrix. Two targets exercise the Native harness and nine contain
source behavior, spanning 141 Rust test functions. Another 119 targets still
need review. The complete original concurrency target through development
Cinch passes six tests and records eleven failures. Eight executed programs
report zero leaks. The unresolved cases cover representation declarations,
borrow and thunk operations, and precise ownership, lifetime and effect
diagnostics. See `bootstrap/current-original-concurrency-frontend-results.json`.
A dispatch-only adapter preserves both whole original fixity-interface tests;
they now pass through actual2866 Cinch, including exact defining/facade metadata,
the renamed operator runtime result9 and one32M poisoned zero-leak report. This
completes that target's assertion-level adaptation review; final matching Stage3
execution remains required. See
`bootstrap/current-additional-fixity-frontend-results.json`. See
`bootstrap/current-outside-frontend-target-review.json`.

Still required: every original default/runtime/diagnostic/path assertion,
source-Prelude replacements and their checked dependency graph, imported default
implementation transport, remaining source laws/base names and language/runtime
gaps, opaque mutable task captures, the matching latest Stage1/2/3 fixed point,
changed-source rebuilding and every original compiler/runtime/tool/editor/external
acceptance item. No completion percentage or final bootstrap sign-off is claimed.

## Strict defaults execute; constructor-case inference remains a real gap (2026-11-07)

Candidate75fb passes the independently audited full44-module compiler/object gate:
all9 tests,17 real C clients and50 zero-leak reports. Its runtime extension passes
15 controls, including all13 prior intended cases, reversed strict-field order
and transitive method dependencies. Both actual8fce regressions now execute
correctly. Ten positive clients run32M poisoned with zero-leak reports; five
semantic rejection controls pass. See
`bootstrap/current-source-class-default-self-values-compiler-results.json` and
`bootstrap/current-source-class-default-self-values-runtime-results.json`.

The owned Box client is accepted by current Native8ba8 but rejected by Cinch as
non-exhaustive before constructor-pattern inference finishes. Its unchanged
source is `bootstrap/programs/ClassDefaults.OwnedCase.Counterexample.cinch`.
This remains a real frontend gap, not a reason to replace the inference case
with an easier annotated program. Strict cycles do reject with empty IR, but
the new runner stopped because checked interfaces remained after backend
refusal. The combined-mode intermediate-output contract needs review before
classifying that assertion as a compiler defect or claiming17/17 success.
The terminal runner state and all original artifacts remain recorded.

Original/default/API replay and the two corrected complete remaining regression
groups continue. Published Cinch stays3268; neither candidate is promoted.
All diagnostic/source attribution, refinement, imported default/library-family,
matching full bootstrap, complete suites and Acceptance1-5 requirements remain.

## Namespace controls pass; strict-default dependencies are under validation (2026-11-06)

Candidate00b5 passes all16 original/new source-default controls and five complete
component groups, including typed value-index behavior. Its complete44 sources
also pass Native8ba8 and genuine earlier4f Stage3 input checks. The coordinator
then attempted two nonexistent target paths; that runner error is retained and
is not a compiler failure. The two real remaining targets are
`bootstrap_buffer_aliases` and `bootstrap_function_preconditions`; both complete
original groups are queued separately, retaining every case and assertion.

Candidate75fb additionally specializes and rechecks author-resolved default
clauses at checked class/method binder aliases. This keeps direct self-method
dependencies visible to the existing strict dictionary initializer; unknown
bindings and strict cycles must continue to refuse. Its44 Native source check
passes. Seven complete components, full9 compiler/object tests and17 actual
runtime controls (including reversed field order, transitive dependencies,
owned constructor values and a cycle refusal) are validating independently.
The unapplied review patch is `bootstrap/source-class-default-self-values-development.patch`,
with exact applicability and limited source-gate evidence in
`bootstrap/current-source-class-default-self-values-development.json`.
Neither private compiler is applied to published Cinch3268.

An optimized current Native8ba8 test host now passes all186 units and all7
class-default tests, with assertions and overflow checks retained. It speeds
remaining large checker builds; it does not replace immutable Native22 bootstrap
seed/library roles or the21-group semantic audit. See
`bootstrap/current-native-optimized-test-host-results.json`.
All remaining actual compiler/default failures and complete Acceptance1-5 remain.

## The complete pipeline target passes with current typed-FFI diagnostics (2026-11-06)

The full original bootstrap_pipeline test now freshly builds and passes all its
original fib, modes, source checks, interface readback and invalidation cases.
Two pure unaccounted foreign declarations remain unchanged and continue to refuse.
Only their obsolete pre-typed-FFI diagnostic strings change to the current precise
closed/function/effect requirement. A new positive effectful I64 binding verifies
canonical interface JSON, local/external symbols, effects, parameter/result ABI
types and the explicit foreign assumption; interface-only mode emits no IR.
The full target has zero failed, ignored or filtered tests, and its exact inputs
and original-case retention are independently audited. See
`bootstrap/current-native-pipeline-results.json`. This is Native8ba8/Cinch3268
pipeline evidence; matching final Stage3 and all complete suites remain required.

## Actual source-default runtime and original tests retain new failures (2026-11-06)

Private Cinch8fce passes the complete actual44-module compiler/object gate:
all9 tests,17 real C clients and50 observed zero-leak reports, with unchanged
Native22 builder/library pins. See `bootstrap/current-source-class-default-compiler-results.json`.
Thirteen dedicated actual-Cinch runtime/refusal controls then pass11/13 after
correcting and separately preserving one malformed new missing-method fixture.
Six positive clients run32M with poisoning and zero leaks, including method/class
annotations, author-private helper scope, overrides and a function carrier.

Two valid programs still fail: `identity a = (a :: a)` confuses the term and type
namespaces; and a nullary default `duplicate = value` reaches an explicit runtime
refusal while materializing self evidence with strict dictionary fields. Their
unchanged sources are `bootstrap/programs/ClassDefaults.SameName.Counterexample.cinch`
and `bootstrap/programs/ClassDefaults.Nullary.Counterexample.cinch`. See
`bootstrap/current-source-class-default-runtime-results.json`.

All9 original API groups plus all7 class_defaults tests also run:14/21 tests pass,
7 fail, with no skipped or filtered cases. The original default test bodies and
assertions are preserved; only outcome dispatch changes to require actual Cinch.
Its11 direct Rust annotation checks stay explicitly classified as Rust evidence.
The six original default failures retain strict field initialization, exact
class/instance diagnostics and defining source-file attribution, and the original
NonZero refinement/container runtime program. Correct semantic rejection alone
does not pass the diagnostic/path assertions. Source-library replacement also
continues to fail. See `bootstrap/current-source-class-default-api-results.json`.

Candidate00b5 separates proper type-head lookup from dependent value-index
lookup. It preserves the original15 default controls and adds the same-spelled
annotation case as16; all original four components plus three complete index,
value-alias and precondition groups are validating. This candidate is unproven.
The strict nullary-field dependency, diagnostics, refinement case, imported
default transport and library-family work remain. Neither private compiler is
promoted; published Cinch stays3268 and Native stays8ba8. Latest matching full
bootstrap, every original suite and Acceptance1-5 remain open.

## Native annotation binders fixed; source defaults pass component audit (2026-11-06)

Native8ba8 preserves definition-owned annotation binders and the actual class and
method substitutions used by inherited defaults. Unknown names still refuse;
independent rigid parameters do not unify, nested quantifiers shadow safely and
the annotation scope ends with its definition. All21 complete affected groups
pass332 Rust tests, including all186 units, all6 original class-default tests and
11 new controls in the seventh test. A real inherited annotated default runs42.
Both exact44-module compiler projects, all13 machine-contract cases, original
False-proof/forgery controls and three actual old/new CLI examples pass independent
audit. See `bootstrap/current-native-default-annotation-results.json`.
The callback/helper task-capture gaps still reproduce and remain required fixes.
Immutable Native22 seed/library pins remain unchanged.
All nine original interface/API groups also replay with the published8ba8 reader:
8/9 groups and13/14 tests pass, with the unchanged source-Prelude collision still
failing. The actual Cinch3268 executable is unchanged. The earlier995f reader
receipt remains in `bootstrap/cinch3268-native995-interface-api-results.json`.

The private Cinch8fce source-default patch separately passes all four complete
component groups: six Rust tests, all15 new default controls, all30 original
source-dictionary cases and actual Text.Json behavior. Its44 sources also pass
Native995f and genuine earlier4f Stage3 checks; these are input checks, not a
matching8fce bootstrap. See `bootstrap/current-source-class-default-component-results.json`.
The actual44-module compiler/object gate now runs with pinned Native22 as builder.
The private patch remains unapplied pending full compiler/runtime/API validation.
Imported defaults, source-Prelude families, laws, implicit base names, every
original failure and complete matching Acceptance1-5 remain required.

## Source class-default implementation is under validation (2026-11-06)

The reviewable implementation is retained as
`bootstrap/source-class-default-development.patch`, with exact inputs and scope
in `bootstrap/current-source-class-default-development.json`. It is a development
patch, not applied to the published Cinch3268 compiler.

Defaults resolve in the class author's scope and check at their own defining
scheme even when unused or overridden. Checked generic Core remains explicit;
an inheriting method calls that checked default at the instance's actual types,
including dependent and erased arrows. Exported default identities cover the
checked scheme and span-free resolved source clauses. Fifteen Native-compared
cases check inheritance, overrides, unused errors, private helpers, class/method
annotations, nullary fields and identity change/stability.

The current generic-call revision now passes all44 Native source checks under
independent artifact audit. See `bootstrap/current-source-class-default-input-results.json`.
Its actual compiled-Cinch component groups and an input check by genuine earlier4f
Stage3 are running;
a later parser failure and interrupted test attempt remain recorded separately.
The current source-interface format records imported default hashes but does not
carry the checked implementation needed to inherit an arbitrary imported default.
That transport, source-family dependencies, class laws and implicit base names,
actual runtime/object/API replay, matching latest full bootstrap, all original
suites and Acceptance1-5 remain required. No current passing default result is
claimed from this patch-application audit.

## Source-Prelude replacement needs a checked cache transition (2026-11-06)

The current actual Cinch3268 executable accepts the retained Data.Core source
alone and with its original Main consumer when no library interfaces are loaded.
The same source pair refuses with the authenticated55-interface library set:
`source module conflicts with a loaded interface`. The original complete API6
replacement test fails at this same first family and remains unchanged.

The fresh Core contract and cached Native22 contract agree on resolved metadata
and all other checked facts, but seven public constructor/field type spellings
differ. Their actual contract identities therefore differ too:4364992559017108310
versus3420315651835929958. This comparison covers Core only, not the Ordering,
Category or Algebra families. See `bootstrap/current-source-prelude-counterexample.json`
and `bootstrap/programs/SourcePrelude.Core.Counterexample.cinch`.

Source precedence must preserve checked family dependencies, current provider
hashes and the absence of facade back-edges. Ordinary unrelated source/interface
collisions and importer fixity redeclarations must continue to refuse. Merely
removing the collision check or replacing dependency hashes without compatibility
checks/rechecking would not establish the required behavior. All four original
replacement families and the complete chain verifier remain required.

## Quoted preconditions retain their argument telescope (2026-11-06)

Native995f extends the numeric-index repair through quoted declarations.
Using contexts see their actual following formal arguments after hygiene,
while binders nested inside an argument domain stay local to that domain.
Kernel True/False retain their logical type identities during template capture
and header readiness. No unknown-name fallback or guessed numeric kind is used.

All20 complete affected groups pass325 Rust tests without skips/filters.
Every original staging24 and refinement27 test remains; nine new source cases
and an actual program returning42 check argument scope, ordinary globals,
numeric-kind refusals, proof domains and effect contexts. Four retained actual
CLI probes and quoted Prop/Nat/Type controls also pass. All13 machine cases,
both exact44-module source projects, original False-proof/forgery controls
and five retained task-capture probes are independently audited. See
`bootstrap/current-native-quoted-precondition-results.json`.

Cinch remains3268. All nine original interface/API groups have now replayed
with this published995f reader:8/9 groups and13/14 tests pass, with source-Prelude
replacement still failing. All original assertions remain. The earlier Native d13
receipt stays in `bootstrap/cinch3268-natived13-interface-api-results.json`. The two opaque
task-capture bypasses still reproduce on995f and remain open. Source-Prelude
replacement, every other original failure, matching latest self-built bootstrap,
all full suites and Acceptance1-5 remain required. Native22 seed/library pins stay.

## Cinch preserves erased-proof interfaces and source value arities (2026-11-06)

Cinch3268 now transports proof-arrow propositions, effects, results and nominal
references through checked interfaces. Qualified proof arrows use proof domains
while retaining their effect rows and preconditions. Public metadata counts
source arrows, dictionaries and hidden value arguments, matching Native schema8;
runtime proof erasure remains unchanged. Only the defining module declares an
operator fixity, and all existing privacy, nominal, hash and fixity assertions stay.

All three complete compiled component groups pass, including27 formatter cases,
16 new provider/facade arity comparisons and all25 original plus12 new qualified
proof source cases. The source harness resolves names before checking them.
The actual44-module Cinch compiler passes all9 compiler/object tests,17 C clients
and canonical contract comparisons. All nine original interface/API groups run:
8/9 groups and13/14 tests pass; source-Prelude replacement remains the one failure.
Three passing direct Native API tests are classified separately from the11 actual
Cinch source-path tests (10 pass,1 fails). See the three
`bootstrap/current-value-interface-*-results.json` receipts.

The full compiler was built by immutable Native22 and runs Cinch's implementation;
it is not a fresh self-built Stage3. Native d13 provides the public reader and
comparison oracle. The exact44 sources also pass the earlier genuine4f Stage3
input check. Matching latest full bootstrap, all original suites, remaining
source-Prelude/record/staging/copyOut/language/Native gaps and Acceptance1-5 remain
required. No original failed case has been skipped or weakened.

## Native signature arguments constrain numeric indices (2026-11-06)

Actual argument domains now constrain numeric signature indices independently
of forall parameter inference. Proof, function, Boolean and unmeasured Bytes
arguments cannot become numeric literals, arithmetic or ordering operands.
Captured names retain their lexical identities; numeric mutable places and
explicit structural measures remain valid. All prior26 refinement tests remain,
with12 new positive and negative controls in the new27th test.

All20 complete affected Rust groups pass323 tests with no skips or filters.
Both exact44-module compiler projects, all13 actual machine-contract cases,
the original False-proof and forgery CLI controls and five task-capture probes
are independently audited. The staging symbol-interning case retains one
intentional immortal root and reports zero leaked cells; all other observed
reports are zero throughout. See `bootstrap/current-native-index-kind-results.json`.

This publishes Native d13; Cinch remains1c76 for this checkpoint. Quoted using
contexts and logical proof types need the separately validated hygiene fix.
Opaque task captures, source-Prelude replacement, every other original failure,
matching latest self-built bootstrap/full suites and Acceptance1-5 remain open.
Immutable Native22 seed/library pins and defining-module-only fixities remain.

## Numeric preconditions need actual argument-kind checks (2026-11-06)

The complete Cinch precondition comparison passes all25 original cases and
the first9 added qualified-proof cases, then its Native oracle accepts a
signature using an erased `True` witness as a number. The unchanged source
`bad :: using (h > 0) => (h :: True) -> Nat` must refuse. Both earlierF6e and
publishedNative301 accept it; two broader probes also admit function and
Boolean parameters as numeric indices. Numeric and actual-proof controls
pass in both compilers. This is ill-kinded declaration admission, not a
demonstrated bad runtime call. Exact sources and CLI/log hashes are retained
in `bootstrap/current-native-index-kind-counterexample.json` and
`bootstrap/programs/index-kind-counterexamples/`.

A new12-control regression passes six valid numeric/proof/binder-position/
byte-measure/mutable-place controls before exposing the erased-proof
acceptance. Every original refinement test remains. PrivateNative d13df
checks actual header argument carriers independently of forall classification,
keeps captured lexical identities, and checks numeric order, arithmetic and
literals. It is validating all13 original primary groups plus seven complete
evidence/index/syntax/staging groups and both44-module projects. This private
correction is unproven until those gates finish. Cinch3268 production is
unchanged; its complete three component groups rerun against the corrected
oracle before actual compiler/object and all nine original API groups.
No expected refusal, original assertion, seed or library pin is weakened.

## Native literal preconditions corrected; all15 affected groups pass (2026-11-06)

Native301 treats `using (True)` and `using (False)` as propositions. A constant
with preconditions checks its instantiated predicates at its reference, using
actual scoped proof evidence or checked static truth. False and unresolved
obligations cannot be assumed. Functions retain their direct-application rule;
runtime Booleans and the anonymous refinement-former grammar stay distinct.
The identical retained source refuses on published NativeF6e and checks on
Native301. A real provider/facade program executes7 and preserves checked
True/False metadata through aliases, serialization and contract hashing.

All15 complete affected groups pass269 Rust tests, including all186 unit
tests, without ignored or filtered cases. The full13-case machine consumer
passes with the established256M normal profile; smaller runtime groups keep
32M poisoning. The strengthened26-test refinement group replaces an added
early signature refusal with a valid quantified declaration plus its actual
unresolved-index-use refusal. Every original assertion remains. Both exact44-module input projects pass, including private3268; unchanged
actual CLI proof/forgery controls and all five capture probes are also
verified. All15 groups and the complete input/evidence replay now have an
independent audit. See `bootstrap/current-native-literal-context-results.json`
and `bootstrap/current-native-literal-precondition-comparison.json`. These
scoped results do not complete the matching final bootstrap or full suites.

All five task-capture probes rerun on exact Native301: both opaque callback/
helper bypasses remain required fixes. Native22/85f00ca seed and library pins
stay unchanged. Cinch3268 remains private: both exporter groups pass; its
complete precondition group runs against Native301, followed by the actual
compiler/object and all nine original API groups. The genuine earlier4f
Stage3 already accepts all44 current Cinch modules. Source-Prelude replacement,
record interfaces, remaining language/staging/copyOut and hidden-type gaps,
latest matching full suites/bootstrap and every Acceptance1-5 requirement
remain open.

## Native portable proof contracts pass267 tests; Cinch public arity needs correction (2026-11-06)

NativeF6e8319 publishes checked literal/generic/compound proposition schemes,
explicit Prop binders and owner-checked kernel Dec. Ordinary application
formatting remains canonical. The strengthened provider/facade test requires
real resolved metadata; additional controls check source readback, actual Core
proof erasure, effect rows and a source-defined Dec. All15 complete affected
groups pass267 Rust tests, with none ignored or filtered. Both exact44-module
projects and unchanged original positive/forgery CLI controls pass. The actual
machine consumer retains all13 original cases at the established full-suite
256M RC_DEBUG0 settings; small runtime groups retain32M poisoning. The two
opaque task-capture bypasses are reproduced on this exact Native build and
remain required. Seed/library pins stay unchanged. See
`bootstrap/current-native-portable-proof-contract-results.json`.

Private CinchC1aeb passes its complete44-module compiler/object gate:9 tests,
17 actual runtime clients,50 zero-leak reports and3 checked canonical contract
comparisons. Its original nine interface/API groups now pass8/9 groups and
12/13 tests; source-Prelude replacement is the remaining failed group. Three
of those passing tests use direct Native APIs and are explicitly classified
separately from the10 actual Cinch source-path tests (9 pass,1 fails). The added
literal proof provider/facade source test now passes with resolved metadata.
See `bootstrap/private-erased-proof-arrow-full-compiler-results.json` and
`bootstrap/private-erased-proof-arrow-interface-api-results.json`.

An independent actual compiler comparison found another mismatch: Cinch uses
runtime argument counts in public value metadata. Native schema8 counts source
arrows plus dictionaries and hidden value parameters. Proof-only, mixed proof/
runtime and hidden-Nat signatures publish0vs1,1vs2 and1vs2 respectively. All
three sources check and emit interfaces in both implementations. The original
programs and exact interface/binary identities remain in
`bootstrap/current-cinch-value-arity-counterexample.json`. Private7dde corrects
metadata counts only, with eight new Native comparisons through provider and
facade; runtime proof erasure remains unchanged. Its complete gates are queued
or running, including an actual earlier4f Stage3 source check with empty PATH.
Neither Cinch private correction is published yet.

The new component/object executables above were built by pinned Native22 and
run the Cinch implementation; they are not a fresh self-built Stage3. This
clarifies earlier checkpoint wording. The independently audited complete
four-stage chain remains earlier4f. Matching latest bootstrap, all original
235/105+14+34/171+43+39 suites and every Acceptance1-5 requirement remain open.

## Private erased-proof interface transport passes both source and exporter gates (2026-11-06)

PrivateC1aeb changes only the Cinch type printer and nominal-dependency
visitor. Both complete compiled exporter groups pass. All21 original formatter
controls remain, with six added proof-arrow controls; all27 pass, including
literal/generic/effectful proof domains, malformed-arrow refusals and the
original canonical Scope/Task comparisons. The proof constructor-field erasure
guard remains. Both exact44-module input projects pass the actual published
Native91cf CLI and actual Native22-built1c76 Cinch implementation, whose check uses55
authenticated library-only interfaces and an empty executable PATH. See
`bootstrap/private-erased-proof-arrow-input-results.json` and
`bootstrap/private-erased-proof-arrow-export-results.json`.

The added canonical comparison exposed a missing Rust portable proof-contract
tier. A private writer correction uses checked proposition structure, keeps
Prop binders and owner-checked kernel Dec, and preserves existing formatting
for predicate-free types. The literal provider/facade test now requires actual
resolved metadata rather than allowing None==None. New controls check parser
readback, compound predicates, effect rows, actual Core proof erasure and a
source-defined Dec. All12 complete groups outside the last machine-consumer
group pass246 Rust tests; that complete last group remains running.

The retained consumer's missing report was reproduced as heap exhaustion with
32M and256M poisoning. Its identical executable/input passes with zero leaks
under the full Native suite's established256M RC_DEBUG0 settings. This is one
case only; the complete original13-case group remains required. See
`bootstrap/private-proof-contract-memory-replay.json`. Smaller runtime groups
keep32M poisoning. Actual full44-module compiler/object and all nine original
API groups are queued/running; source-Prelude replacement and all other
original failures remain required. Neither private compiler change is yet
published. Published Cinch1c76, Native91cf, Native22/85f00ca seed/library roles,
all original235/105+14+34/171+43+39 coverage and Acceptance1-5 remain intact.

## Cinch erased-proof arrows need portable interface transport (2026-11-06)

The current1c76 Cinch producer and Native91cf reader run all nine original
API groups, preserving every original assertion and adding the new logical
proof-contract case. Ten of the original11 tests still pass; source-Prelude
replacement still fails. The added test exposes another gap: its unchanged
source passes actual Cinch checking, but interface export refuses its erased
proof arrow. The complete latest API result is therefore7/9 groups and10/12
tests passing, with no skips. See
`bootstrap/current-native-reader-interface-api-results.json` and the retained
`bootstrap/programs/ErasedProofArrow.Interface.Counterexample.cinch`.

The missing cases are in the type printer and nominal-dependency visitor,
not logical literal spelling. A private correction retains the proposition
domain, latent effects, result and nominal references; malformed proof arrows
and moving early preconditions across an erased argument remain refused.
Its complete original exporter groups plus six new formatting/Native-readback
controls are validating. Actual full compiler/API replay, complete matching
suites and every Acceptance1-5 requirement remain required before completion.

## Native logical literals retain checked proof hypotheses (2026-11-06)

Native91cf supports logical True/False in Prop, separately from Boolean
runtime values. Their traversal, equality, unification, printed contracts and
static/runtime decisions retain that distinction. An already checked lexical
proof hypothesis remains usable even when its proposition is closed and
false: `(h :: False) -> Dec False` is a valid function over an uninhabited
domain. The selected proof term still checks normally, including evidence
identity and invalidation. An ordinary Bool, integer or unrelated proof cannot
forge False. The unchanged original positive source now passes the actual CLI;
the unchanged original `Yes 1` forgery still refuses.

All nine complete affected unit/proof/dependent/refinement/interface/pattern/
guarded/stale-evidence groups pass257 Rust tests, with no ignored or filtered
cases and73 observed zero-leak reports. Fourteen new source controls, three
decision programs, a real provider/facade interface round trip and a kernel
identity test retain every original assertion. Three physically retained CLI
programs also independently execute with32M poisoning and zero leaks. Both
exact44-module compiler projects pass. See
`bootstrap/current-native-proposition-literal-results.json`.

The two opaque task-capture bypasses are reproduced on this exact Native91cf
build and remain required fixes. Cinch stays1c76, and the immutable Native22/
85f00ca seed/library roles stay unchanged. Source-Prelude replacement, record
interface transport, remaining language and staging declarations, copyOut,
Native existential introduction and the complete matching latest-source
bootstrap/suites/105+14+34/Acceptance1-5 remain open. Earlier4f Stage3's14
external checks now have their own independent artifact/replay audit in
`bootstrap/current-record-indexed-external-acceptance-results.json`.

## Task wrappers need capture contracts in both compilers (2026-11-06)

Five exact source checks confirm a NativeE5 gap: a direct mutable Bytes
capture at `spawn` refuses, while the same capture through a let-bound
callback or a normally typed `fork` helper is accepted. Both safe no-capture
controls pass. No race was executed. The programs, actual compiler identity
and log/source hashes are retained in
`bootstrap/current-native-task-capture-counterexample.json`.
The correction must preserve safe wrappers while carrying real capture,
transfer and lifetime requirements through aliases, helpers and interfaces.
Cinch currently conservatively refuses the opaque wrapper; making every
function shareable would reproduce the Native hole. This remains required
work even when existing Native tests pass.

The completed earlier4f Stage3 language replay is independently audited:
7/13 positives execute, all21 negatives refuse,2 intended diagnostic checks
pass and25 failed subcases remain. Every original source/assertion remains,
and all seven successful programs independently rerun with32M poisoning and
zero leaks. See `bootstrap/current-record-indexed-stage3-language-results.json`.
The newer1c76 development result remains separate:8/13 positives and3/21
intended diagnostic checks pass after the Format correction.

## Proof elimination order repairs Format; four-stage4f bootstrap verified (2026-11-06)

Compiler1c76 chooses the arithmetic elimination column producing the fewest
rows. This repairs valid byte writes following an observed payload length,
without changing the trusted certificate checker or the64-step/256-row/split8
budgets. All original tests and assertions remain; ten captured-context packets
and six Native-compared source cases cover the repair and invalid controls.
Five complete component groups and both exact44-source checks pass. The actual
44-module compiler/object gate passes9 tests,17 optimized C clients,50 zero-leak
reports and3 real resolved-contract comparisons. All13 record/indexed runtime
clients and5 source refusals pass, with13 independently poisoned reruns.
See `bootstrap/current-proof-elimination-results.json`.

All three exact retained byte-size positives now pass. The original Format
positive executes and its negative satisfies its original predicate/path
assertions. The complete original34-case language replay executes8/13 positives;
all21 negatives refuse,3 meet their intended diagnostic assertions, and23 failed
subcases remain. Choreography, Proof, Protocol, Tactics and Tasks remain positive
failures. Original API coverage remains8/9, with source-Prelude replacement
failing. Record interface transport and Native existential/False-domain parity
gaps remain open. No example, assertion or required suite is removed.

The earlier4f compiler now independently proves Acceptance1-4: all four actual
stages,12 smokes, matching Stage2/3 LLVM/executables/all44 interfaces, actual API
consumption and changed-source behavior. Its14 original external acceptance
checks pass. See `bootstrap/current-record-indexed-bootstrap-results.json`.
Its235-group Native sweep and105-target Stage3 suite continue with their exact
earlier source; they do not establish matching1c76 completion. Full matching
latest-source suites, bootstrap,105 frontend targets,14 acceptance checks,
all34 language cases and every Acceptance1-5 requirement remain required.

NativeE5 production and the pinned Native22/85f00ca seed/library roles remain
unchanged. A separately audited optimized development runtime keeps debug
assertions and overflow checks enabled and passes all171 runtime tests;
`bootstrap/current-native-checked-runtime-profile-results.json` records its
exact profile and unchanged pinned runtime. It is not a new bootstrap seed.

## Matching auxiliary suites pass; observed byte-size proof gap isolated (2026-11-06)

Published4f/NativeE5 passes all171 runtime tests,43 tooling tests and39 Native
editor integration tests, with zero skipped or filtered cases. Frozen source,
all500 expected Rust inputs, CLI provenance, logs and counts are independently
verified in `bootstrap/current-record-indexed-auxiliary-results.json`. Native
editor integration is separate from Cinch-built frontend parity.

The matching four-stage4f chain has passed Stage0 and Stage1 with all their
smokes; Stage2 is building. Seed and library-only provider audits pass. A fresh
235-group NativeE5/4f sweep is running. All105 frontend targets, the original14
external acceptance checks and all34 original language cases are queued behind
the exact independently audited matching Stage3. These runs do not close any
incomplete acceptance requirement in advance.

The unchanged Format source now has a smaller retained counterexample: a byte
write following `n = bytes-len payload` and allocation of `n + 4` bytes. Native
accepts three reduced positives; actual4f refuses each. All seven controls with
an ordinary Nat size agree between implementations, and end-index refusals
remain correct. Private diagnostic instrumentation shows the actual scoped
bounds and unsolved write obligation. See
`bootstrap/current-observed-byte-size-counterexample.json` and its unchanged
positive program. A private correction chooses a cheaper arithmetic elimination
order while retaining the same trusted certificate checker,64-step/256-row
limits and original tests. Its complete proof test and ten new captured-context
packets pass. Actual source controls, full compiler checks and replays remain
pending. Source-Prelude replacement, record interface transport, the remaining
language examples, complete matching suites and every Acceptance1-5 stay open.


## Typed record fields and indexed constructor layouts (2026-11-06)

Compiler4f now propagates declared and instantiated field types into record
initializers, including Nat/U8 literals, nested fields and callback bodies. It
preserves written strict evaluation order. Projection of an unknown receiver
uses exactly one declaring record visible in its module; ambiguous, private and
unselected owners remain refused. Checked unequal closed numeric indices can
prove constructor alternatives impossible; symbolic uncertainty cannot.

The runtime backend now erases checked value indices with compatible carriers
while retaining ordinary parameter identities and field representations. The
complete runtime-IR group passes137 clients plus2 C ABI executions,139 zero-leak
reports. The actual44-module compiler/object gate passes9 tests,17 optimized C
clients,50 zero-leak reports and3 real resolved-contract comparisons. All13
record/indexed runtime clients and5 source-refusal controls pass; independent
poisoned reruns of all13 clients report zero leaks. All44 sources pass both
Native and the audited Cinch-built reader. Earlier D13 frontend/export gates
retain their explicit source scope:5 groups,70 constructor cases and9 metadata
mutations. See `bootstrap/current-record-indexed-runtime-results.json`.

The unchanged original34 language cases now execute7/13 positives, including
Audio and Query. All21 negatives refuse, but only2 satisfy their original
diagnostic assertions;25 assertions remain failed. All seven successful
programs also pass independent32M poisoned reruns. See
`bootstrap/current-development-external-behavior-results.json`.

All9 original API groups remain8/9, with source-Prelude replacement failing.
Record interface transport and the remaining six language positives stay open.
The old Nat-field refusal is retained as historical counterexample evidence;
the exact positive source passes D13 and the4f field/runtime controls pass.
NativeE5 stays current and Native22/85f00ca remains the immutable seed/library
producer. A matching4f chain and all complete suites,105 frontend targets,14
external acceptance checks,34 language cases and Acceptance1-5 remain required.



## Native constructor patterns infer their input shape (2026-11-06)

Native e5a9 fixes the original nullary-constructor lambda failure. An unknown
match input can obtain its shape from a concrete pattern or the scheme of its
normally resolved constructor. The existing matrix compiler still checks each
row, field, family and uncovered case. Later application arguments do not choose
the constructor owner, and rigid generic inputs do not become concrete families.

Four complete groups pass29 Rust tests, none skipped or filtered, with17
zero-leak reports. Twelve new controls retain the original nullary lambda and
cover payload, tuple, unit, Bool/list/literal matrices, incomplete matches,
wrong families and incompatible tuples. A real program executes constructor
and tuple lambda paths. All original pattern/empty-data controls and the earlier
proposition correction remain. Both exact44-module compiler projects pass the
real Native loader, and the unchanged original lambda now passes the actual CLI.
See `bootstrap/current-native-pattern-inference-results.json`.

Native existential introduction and False-proposition domains remain retained
parity gaps. Full matching Native/Cinch suites remain required; seed/library
roles stay Native22/85f00ca. The public Cinch compiler remains dd8d.

The private4f indexed-constructor backend repair now passes the complete runtime
IR group:137 client executions plus two C ABI executions,139 zero-leak reports.
All original cases/assertions remain and three new fixed-index programs execute,
including generic payload representations and a first-class constructor. Exact44
sources pass optimized NativeA2 checking. The full actual compiler/object gate,
Cinch-built source check and fresh runtime/API/external replays are running or
queued. These are separate gates; the latest full bootstrap,105 frontend targets,
14 external acceptance checks, all34 language cases and Acceptance1-5 stay open.

## Native proposition binders retain proof carriers (2026-11-06)

Native a2b9 fixes the original positive-proof constructor failure. An explicitly
quantified proposition used as a proof domain keeps its declared Prop carrier;
it no longer acquires a contradictory ordinary-type use. Actual proposition
argument uses also constrain binder kinds, so Type parameters cannot silently
become propositions. Scoped carrier hints are removed after each signature.

Four complete groups pass28 Rust tests with no skipped or filtered cases and16
zero-leak reports. The original eight proof-identity controls remain, alongside
11 new kind/scope/erasure controls and two programs that compile and execute
with erased evidence. Both exact44-module compiler projects, published dd8d
and private d13f, pass the real Native project loader. The unchanged original
positive proof source now passes the actual Native CLI; its6b/58e refusals stay
retained. See `bootstrap/current-native-proposition-carrier-results.json`.

An independent optimized build of the identical Native source also passes all
four groups, preserving debug assertions. Full matching suites remain required;
these results do not replace them. Seed/library roles remain Native22/85f00ca.

Private d13f now passes the complete constructor/inference/resolver groups,
all70 constructor source cases and nine metadata mutations, both44-source
checks, and the independently audited actual compiler/object gate. The latter
passes nine Rust tests,17 optimized C clients and50 zero-leak reports. Its nine
original API groups remain8/9, with source-Prelude replacement still failing.
Twelve of13 runtime clients pass. The indexed Signal client checks successfully
but emission refuses its fixed-index constructors; that real backend gap is
retained. Earlier source-string escaping mistakes in two added fixtures are
separately retained and corrected, without changing any compiler source or
original assertions. The complete external-language replay awaits successful
runtime checks. d13f remains private while the backend repair proceeds.

The earlier frozen5f2f/Native6b full sweep finished:234/235 groups and1630/1631
tests pass, none skipped or filtered. Its original bootstrap_infer failure stays
failed; current targeted results do not rewrite historical evidence. A matching
latest-source full sweep, bootstrap,105 frontend targets,14 external acceptance
checks, all34 language cases and every Acceptance1-5 requirement remain open.

## Bootstrap and external-language evidence refreshed (2026-11-06)

The preceding aa4e compiler now independently proves Acceptance1-4: all four
stages, twelve smokes, identical Stage2/3 LLVM and executables, all44 canonical
interfaces, actual compiler-interface consumption and changed-source behavior.
See `bootstrap/current-canonical-contract-bootstrap-results.json`. This is
source-specific evidence; dd8d and later repairs still need a matching chain.

The exact published dd8d compiler now has an independently audited replay of
all34 original external language cases. Five of13 positives execute: Circuit,
Differentiate, Html, Reactive and TensorAD. Each also passes an independent32M
poisoned rerun with zero leaks. All21 negatives refuse; only2 satisfy their
original reason/path assertions. Both whole tests still fail, with27 failed
subcases. No examples or assertions were removed. See
`bootstrap/current-development-external-behavior-results.json`.

The private typed-field, closed-index coverage and visible-record projection
repair passes all44 compiler-source inputs through both Native and an audited
Cinch-built checker. Its complete constructor, inference and resolver groups
are running against Native58e. A newly added annotation fixture was corrected
to the supported operand syntax; the failed earlier fixture is retained.
All70 component source cases and their original expectations remain required.
A separate Native preflight found four older component positives rejected by
both Native6b and58e. These are retained parity gaps, not passing Native tests.

The original105 frontend targets, separate14 external acceptance checks,
all34 language cases, complete Native/runtime/tooling suites and every
Acceptance1-5 requirement remain required. The goal is not complete.

## Native empty matches use checked data-family provenance (2026-11-06)

Native58e fixes a coverage fast-path bug: empty rows previously failed before
checking whether the scrutinee could have any constructor. A declared empty
data type, or a data family whose every constructor is refuted by its indices,
now justifies an empty match. Ordinary primitive, opaque, record and dictionary
carriers do not gain emptiness from lacking exposed constructors. Unknown or
possible equations remain non-exhaustive. Every constructor trial restores the
original unifier, including failed and unknown trials.

The complete original pattern-field group plus8 new source controls passes4
Rust tests with0 skipped/filtered cases. Both valid new programs compile and
execute; all4 native executions report zero leaks at32M with poisoning. See
`bootstrap/current-native-empty-data-coverage-results.json`.

The Cinch implementation remains dd8d; its valid numeric record-initializer
counterexample remains open. The separate Cinch typed-field correction passes
its complete constructor/inference groups. A combined closed-index/projection
repair is validating against this new Native oracle. The abandoned lookahead
prototype's original ambiguous lambda is retained: Native correctly requires
an annotation when an unknown receiver has multiple visible declaring records.
Its replacement uses actual module visibility, not a global field-name guess.

The seed and library producer stay explicitly pinned to Native22/85f00ca ABI6.
This development Native fix does not replace those roles or prove a matching
full235-target Native sweep, current-source bootstrap,105 frontend targets,
14 external acceptance checks,34 external language cases or Acceptance1-5.

## Source records and constructor parameter identity pass scoped gates (2026-11-06)

Compiler dd8d supports named record construction, complete named patterns,
projection and generic/nested fields through ordinary independently checked
single-constructor Core. It preserves written initializer order and strict
evaluation of discarded fields. Missing, duplicate and unknown labels refuse.
Record interface writing/import transport remains incomplete.

Constructor results retain their declared ordinary type, constructor and
proposition parameters; only value indices may be fixed or computed. The
original invalid `Box :: a -> Box Bool` source now refuses. The hidden-payload
escape source remains unchanged and rejects via the independent Core guard;
its old unsupported-feature diagnostic expectation now names that actual guard.
Five new Native-compared parameter controls preserve the numeric-index cases.

Eight complete affected groups pass16 Rust tests. The actual44-module compiler
passes17 optimized C clients and50 zero-leak reports, including the3 actual
resolved-writer comparisons. Six fresh record runtime clients pass at32M with
poisoning and12 further zero-leak reports. All44 exact inputs pass Native and
the independently audited Cinch-built checker. All9 original API groups run
unchanged:8 pass; source-Prelude replacement still fails. See
`bootstrap/current-record-and-constructor-results.json`.

A real current-CLI counterexample remains: the valid `User { age = 36 }` for a
Nat field is rejected because an untyped temporary defaults the literal to Int.
Native accepts the identical source. The retained positive program and actual
rejection are in `bootstrap/programs/Record.FieldCarrier.Counterexample.cinch`
and `bootstrap/current-record-field-carrier-counterexample.json`. The private
typed-initializer correction passes its complete constructor and inference
groups. Combined application-inference and indexed-coverage repairs continue;
these later sources are not claimed as part of this checkpoint.

The preceding aa4e four-stage chain passes its exact-source independent
audit; the receipt above preserves that source scope. The29e6 Acceptance1-4 proof stays scoped to29e6. A matching latest chain,
full105 targets, complete Native suite, separate14 external acceptance checks,
all34 original external language cases and every Acceptance1-5 remain required.

## Actual resolved value contracts match Native (2026-11-06)

Compiler aa4e fixes the missed real-writer call site. Public type applications
remain flat; resolved value contracts retain canonical binary applications.
Three permanent real CLI comparisons check complete Pair, Scope and Task
resolved contracts against Native. All21 earlier exporter controls remain.

Five complete affected groups pass12 Rust tests,17 optimized C clients and50
zero-leak reports. All44 exact compiler inputs pass Native checking and the
independently audited Cinch-built5f2f checker with an empty executable PATH.
All9 unchanged API groups run:8 pass; source-Prelude replacement still fails.
See `bootstrap/current-resolved-application-results.json`.

The preceding29e6 compiler independently proves Acceptance1-4: all4 stages,
12 smokes, identical Stage2/3 code and44 interfaces, actual compiler interface
consumption and changed-source behavior. See
`bootstrap/current-owner-display-bootstrap-results.json`. This proof is scoped
to29e6; a matching aa4e chain and all matching suites remain required.
The full105-target inventory, complete Native suite, separate14 acceptance
checks, all34 external language cases and Acceptance1-5 remain the goal.
Record syntax/type/runtime work stays private while external parity is tested.

## Defining owners and public type applications pass scoped gates (2026-11-06)

Compiler29e6 requires an opaque foreign header's exact defining module.
An admitted Host interface cannot claim Host.Nested.Handle without that
module's defining header. The original fresh-hash forgery is retained as a
negative control; legitimate direct and facade pointer clients still pass.

Public value schemes flatten type applications, including nested applications.
Every original19 exporter control remains, plus2 Native readback cases; the
complete unchanged indexed-type API group now passes. A later actual-writer
comparison exposes a missed call site: resolved Pair and Scope value contracts
still use public spelling, unlike Native canonical contracts. The original
formatter controls did not cover that path. The incorrect acceptance is retained
in `bootstrap/current-resolved-application-counterexample.json`; a correction
with3 permanent actual-writer/Native comparisons is validating.

All10 complete affected groups pass18 Rust tests. The actual44-module compiler
passes17 permanent optimized C clients and49 zero-leak reports, ABI and object
identity mutations, source refusals and the schema-valid pure-body forgery.
All44 exact staged inputs pass Native checking and the independently audited
Cinch-built5f2f checker with an empty executable PATH. All9 original API groups
and11 Rust tests run unchanged:8 groups/10 tests pass; source-Prelude replacement
still fails. See `bootstrap/current-owner-and-display-results.json` and
`bootstrap/current-original-interface-api-results.json`.

A fresh matching44-module four-stage chain is running. Its Stage0 smokes and
library-only provider pass; later stages and independent fixed-point/interface/
changed-source checks remain open. The earlier5f2f full Native and105-target
Stage3 sweeps keep their own frozen scope. Matching full suites, the separate14
external acceptance checks, all34 external language cases and every Acceptance1-5
requirement remain required. Record syntax/type/runtime work remains private.

## Earlier93fc typed C imports passed scoped gates; owner gap recorded (2026-11-06)

Compiler93fc supports typed C scalar/float/unit and recorded opaque/pointer
imports, with explicit effects, checked ABI plans, non-null guards and no
foreign total-body authority. Canonical interfaces match Native serialized
foreign/value rows. C-only interfaces may accompany a library object while
all represented object module identities remain exact. Direct and facade
opaque-pointer imports execute successfully.

All10 affected groups pass18 Rust tests, with17 permanent real C clients
against optimized C code,48 zero-leak reports, two rehashed ABI disagreements,
a changed represented-object identity, a schema-valid matching identity/type
pure-body forgery and three invalid source signatures. Six narrow signed and
unsigned C cases independently check direct literals and C-returned values.
All44 exact compiler inputs pass both Native checking and the audited
Cinch-built5f2f Stage3 source checker. See
`bootstrap/current-typed-foreign-results.json`.

An added owner control exposes a remaining bug: Host can claim the opaque
identity Host.Nested.Handle without admitting its defining Host.Nested
module. This is a confirmed incorrectly accepted contract, retained in
`bootstrap/current-foreign-owner-counterexample.json`; the scoped passing
gates do not close it. Defining-owner validation is the next repair.

All nine original API groups run unchanged: seven pass. Kinds now passes the
C declaration and reaches public application formatting (`((Vec 0) a)`);
source-prelude replacement remains refused. Aggregates, owned/deallocator
protocols and higher-order foreign values are explicit boundaries. The earlier
5f2f bootstrap proof is retained on its exact source. A matching new bootstrap,
complete105-target and14-check acceptance, external language parity and every
Acceptance1-5 requirement remain open.

## Actual Stage3 external language parity exposes further gaps (2026-11-06)

The complete original external behavioral module now runs through the audited
5f2f Stage3, with its original assertions retained: all13 positive examples
and21 negative examples. Circuit executes at32M with poisoning and zero leaks.
The other12 positives refuse before producing IR. All21 invalid examples
refuse, but none reaches its original expected diagnostic reason; these are
failed assertions, not passing negative controls. Both whole tests fail with
33 failed subcases. No cases are skipped or filtered.

An independent audit reconstructs every exact expanded source section from
the unchanged engine, syntax libraries and examples, verifies the actual
Stage3 executable and invocation records, and reruns Circuit. The Python
adapter only bundles source, selects the actual Cinch CLI, links its emitted
IR and maps source positions; it never substitutes a Native source frontend
or rewrites diagnostic reasons. See
`bootstrap/current-stage3-external-behavior-results.json`.

Source records and their construction/pattern/projection syntax, borrow/lazy
operations, indexed-pattern coverage, source assertions/proof search and task
sharing remain frontend parity work. The matching Native editor suite's39
passing tests exercise its own explicitly recorded Native CLI role and do not
close this gap. The original105-target inventory and separate14-check external
acceptance suite remain intact and required. Acceptance5 is not complete.

## Matching runtime, tooling and Native editor checks pass (2026-11-06)

Published5f2f/Native6b passes all171 runtime tests,43 tooling tests and39
compiler/editor integration tests, with no skipped or filtered cases. An
independent audit verifies the complete frozen source snapshot, every log,
the freshly compiled matching Native6b CLI and its Cargo compilation record.
The43 compiler input bytes match the independently verified5f2f bootstrap
chain exactly. See `bootstrap/current-native-auxiliary-results.json`.

The editor integration role here is the matching Native CLI. This evidence
alone does not establish Cinch-built editor/frontend parity or Acceptance5.
The separate complete235-group Native sweep and all105 original/current
Stage3 target sweep continue; their incomplete or failed rows remain visible.
Array ownership/staging, foreign contracts and source-prelude replacement
still require repairs and matching final bootstrap/suite verification.

## Published5f2f reaches a verified bootstrap fixed point (2026-11-06)

The current published compiler independently passes Acceptance1-4 on its exact
5f2f source: all four stages and twelve smokes, matching Stage2/3 LLVM and
executable bytes, all43 canonical interfaces, actual compiler-interface
consumption and changed-source rebuilding with observable changed behavior.
Stage1 links the optimized exact pinned85 runtime. The Stage0 and library-only
producer archives, all physical artifacts and Native interface decoding are
independently verified. See
`bootstrap/current-checked-decision-bootstrap-results.json`.

All105 original/current targets now run through an immutable copy of this
actual Stage3, with matching freshly built Native6b test hosts and authenticated
library-only85 dependencies. The preceding development-compiler diagnostic is
retained: type syntax and total semantics pass; the original array and two API
gaps remain, and abstract nonzero methods exposed a missing output report.
The final Stage3 run replays every complete original group. The diagnostic was
retired with its completed failures preserved and its live partial group marked
explicitly, once the audited Stage3 sweep started. A separate matching full
Native suite remains running. These runs, runtime/tooling/editor/external
checks, all105-target accounting and Acceptance5 must pass before completion.

A private44-module foreign-call candidate passes the complete Native source
check. It retains real effect/ABI/pointer contracts and no total-body authority;
its pinned compiler build and actual C-call/interface tests remain separate
validation, not published language support.

## Checked source helpers participate in runtime decisions (2026-11-06)

Compiler `5f2f8556` accepts numeric decisions over fully applied pure total
source helpers. The original identity input was valid according to the current
Rust compiler; its previous negative expectation hid an independent Core gap.
The source test now retains that input as a positive and checks real source-body
closure. Empty, changed or mismatched body authority still refuses. Partial and
effectful helpers remain rejected. Saturated two-argument Int/Nat helpers work.

Both complete decision groups and both adapter/object groups pass eleven Rust
tests on this exact merged source, without ignored or filtered cases. Ten new
Core controls run at five fresh scopes. All43 exact compiler inputs build and
check through the actual Cinch-built917f reader with empty PATH. Three real
clients execute both branches, including128-bit-range Int values, at32M with
poisoning and zero leaks; three invalid source cases refuse. The actual object
round trip, canonical metadata/layouts and all physical entries are audited.
See `bootstrap/current-checked-source-decision-results.json`.

A fresh full Native suite and four-stage bootstrap are starting on this exact
source, rather than carrying forward earlier-source pass counts. Interface
API7/9 remains an earlier711 checkpoint until replayed. Typed foreign
contracts, source-prelude replacement, original array ownership/staging gaps,
higher-order opaque decision reification, all105 targets and final matching
Acceptance1-5 remain open. The Native-accepted opaque function-argument case
is retained as an explicit gap, not recast as an invalid negative test.

## Paired runtime measurement and earlier917f bootstrap complete (2026-11-06)

One paired run checks the same43 compiler inputs with the exact same
Native-produced LLVM, linked to release versus debug builds of the exact
pinned85 runtime. Both checks pass with empty PATH and zero leaks. Release
takes664.45s and debug4508.15s: an observed6.78x improvement for this input
under bounded concurrent load. This is one observation, not a general timing
promise. The Stage1 build-profile fix is already published in067446e. See
`bootstrap/current-stage1-runtime-performance-results.json`.

The frozen917f chain independently passes all four stages and twelve smokes.
Stage2/3 LLVM, executable and all43 canonical interfaces match; actual compiler
interface consumption and changed-source behavior pass. This proves
Acceptance1-4 for that earlier source, with explicit85f00ca Native22 seed and
library-only provider roles. See
`bootstrap/current-byte-observation-bootstrap-results.json`.

Published711103 remains at7/9 original API groups. The merged private5f2f
checked-helper decision repair has passed the actual self-built43-source
check; full compiler/object and reached32M runtime controls are validating.
Matching latest-source bootstrap, all105 targets and Acceptance5 remain open.

## Original class contracts pass; API scope is7/9 (2026-11-06)

Compiler `7111030f` exports contextual dictionary factories with one hygienic
binder allocation for their target, context and complete scheme. Contextual
factories grant no closed method-body authority. Private classes needed by
retained methods are included transitively without publishing their names.
Admitted class headers/methods retain exact defining contracts. Signature rows
sort by name then identity, matching the Native canonical bytes and hashes.

Source data parameters and constructor fields now retain their declared names,
consistent with Native interfaces. The two former `q0` goldens require the
actual source `a`; direct Native comparisons cover Vec/Parcel/Phantom names,
kinds and implicitness. Canonical scheme alpha-stability remains unchanged.
The three-level private dependency and cross-module ordering controls pass.

All four complete affected exporter/adapter/object groups pass eleven tests.
All43 exact compiler inputs build with the explicit Native22 seed role and
check through the actual Cinch-built917f reader with empty PATH. The real
object round trip passes, including five zero-leak client reports. See
`bootstrap/current-contextual-factory-results.json`.

All nine original API groups execute unchanged: seven groups and nine of eleven
Rust tests pass. The complete original class-contract test now passes its
factory, private closure, defining-contract/hash and mutation assertions. The
two failures still require typed foreign declarations and source-family
replacement with genuine dependency rechecking. See
`bootstrap/current-original-interface-api-results.json`.

The frozen full Native sweep also exposes a valid checked-helper decision
rejection, reproduced in the actual current Cinch CLI and accepted by current
Rust. Its independent Core repair controls pass privately; complete source
fixture validation is still open. No runtime dictionary-factory, final
Stage2/3, all105-target or Acceptance5 completion is claimed here.

## Stage1 links an optimized pinned runtime (2026-11-06)

The stage builder now builds the exact pinned seed runtime in release mode
before building Stage1. The runtime archive hash participates in the Stage1
cache key. A failed or stale runtime build refuses instead of silently using
the Native linker's debug fallback; changing archive bytes invalidates the
cached compiler. The seed revision and compiler sources remain explicit.

All43 tooling tests pass, including two cache/failure controls. A fresh
85f00ca/Native22 seed builds all43 inputs of compiler1dd5b24b into Stage1;
Stage0 and Stage1 each pass the original fib, precedence and named-refusal
smokes. See `bootstrap/current-stage1-runtime-profile-results.json`.
This is a verified build-profile correction; a same-IR runtime performance
comparison is still running. It grants no Stage2/3 fixedpoint or final suite
acceptance. Existing older chains retain their inputs and binaries.

The private dictionary-context exporter builds and passes its complete
exporter, adapter and object gates, but its original class API replay exposes
missing transitive private-class method dependencies. A closure repair with
three levels of private class context is in validation. All105 required
targets, matching final bootstrap/suites and Acceptance5 remain open.

## Constructor declarations retain their own type and index binders (2026-11-06)

Compiler `1dd5b24b` checks constructor-local Type/Nat binders and fixed or
computed result indices. It keeps inherited and local scopes distinct and
omits unused family index placeholders from the constructor scheme. Result
kinds and declaring identities, strict positivity, independent Core closure
and the hidden-pattern scope guard remain checked. Actual hidden construction
checks; a hidden payload still cannot escape as Int or Bool.

The real interface writer preserves these schemes and indices. A hidden
payload constructor has `resolved: null`, rather than an ordinary universal
pattern contract. All five complete affected groups pass thirteen Rust tests,
including ten new source controls and independent Native schema/tree checks.
All43 exact staged inputs also build with the explicit Native22 seed role and
are checked by a checksum-verified copy of the actual Cinch-built917f compiler
with an empty executable PATH. The real object round trip remains green.
See `bootstrap/current-constructor-source-contract-results.json`.

Actual CLI controls refuse reached hidden/indexed physical layouts with code5,
a cleared empty IR sentinel and no executable. An ordinary generic Box client
runs42 at32M with poisoning and zero leaks. Scoped elimination and physical
representation support remain separate; declaration support does not grant
those features. The Rust comparison still has its documented constructor-local
index-only boundary, so the valid hidden construction is not reported as a
matching Rust oracle pass.

All nine original API groups run with their assertions retained: six pass and
three fail. The class-contract fixture now reaches contextual dictionary
factory export; kinds reaches typed foreign declarations. Source-family
replacement remains refused. Those failures, matching full bootstraps/suites,
all105 required targets and Acceptance5 remain open. A separate private factory
exporter is being validated. Storage migration changed canonical dependency
paths and triggered an extra cache rebuild in the older chain; existing builds
continue on their fixed source, and their storage paths will stay fixed.

## Constructor patterns preserve hidden type scope (2026-11-05)

Compiler `4507111e` refuses retained constructor patterns that choose a concrete
payload type for a hidden universal parameter. The previous `038faba0` checker
accepted the forged `Exists -> Int` extraction in five fresh scopes. Ordinary
parameterized patterns and polymorphic constructor introduction remain valid.
Only direct result parameter slots determine a pattern instantiation; a computed
index is not treated as an injective witness for its hidden parameters.

All five complete affected groups pass: global checks, nominal/import checks,
the full runtime group, the frontend adapter and the library-object round trip.
That is thirteen Rust tests, with twenty-five additional scope controls. All
136 exposed runtime leak reports and five object-client reports are zero.
The actual compiler builds from all 43 exact staged inputs with the explicit
Native22 seed role. See `bootstrap/current-constructor-pattern-scope-results.json`.

This establishes the independent checker prerequisite for existential support.
Source existential declarations still refuse; admitting them requires real
scoped type openings, indexed refinement evidence and corresponding runtime
representations. The earlier917f bootstrap and Native6b sweep retain their
frozen inputs. Matching final bootstrap, all105 required targets and Acceptance5
remain open.

## Updated parser compiler builds and performs the library round trip (2026-11-05)

The exact 43 staged-driver inputs of compiler038f build into an actual Cinch
compiler using the explicit Native22 seed role. Both complete adapter/object
groups pass all nine tests. The actual Cinch CLI emits and reloads the library
object; an independently linked owned-bignum/ADT/array client runs at32M with
poisoning and zero leaks. All1,842 physical units, fourteen new public/accessor
symbols,4,224 descriptor words,60 interface contracts and26 method families
are independently audited. See `bootstrap/current-row-parser-compiler-results.json`.

The constructor-scope prerequisite is now separately verified and published
above; the original038f parser/compiler receipts retain their exact scope.

The earlier917f Stage2 build and Native6b full sweep continue on their frozen
inputs. A matching final chain, full current suites, all105 required targets
and Acceptance5 remain open.

## Row arguments preserve their complete syntax tree (2026-11-05)

Compiler `038faba0` parses effect rows in type-argument position, including
`Thunk {} Int`, open tails and qualified effect labels. It preserves labels,
tails and the following value argument in the existing structured AST; it
does not consume a row as a declaration block or drop it. Rows remain type
arguments rather than standalone type atoms, matching the Rust parser's
boundary.

Both complete existing groups pass: the syntax-tree checks and the full parser
corpus/agreement group, with none ignored or filtered. Five added whole-scheme
tree checks include the exact original Ref/Array/Thunk copy signature; six
malformed row inputs refuse. Actual compiler917f independently checks all43
exact staged-driver inputs of this new compiler. See
`bootstrap/current-row-argument-parser-results.json`.

The separate generic user-row-family experiment is private: its Rust comparison
finds that general declared Row parameters are still outside the current Native
baseline. This parser change does not add such family kinds, Copy/borrow/Thunk
semantics, staging or constructor/foreign support. The running917f chain retains
its exact earlier inputs and is rebuilding Stage2. Matching038f bootstrap,
complete suites, all original100 targets plus five additions and Acceptance5
remain required. Existing failing array/API receipts retain their917f scope.

## Original array and interface/API groups expose the remaining gaps (2026-11-05)

The published `917f6e1f` compiler runs the complete original fifteen-case
mixed array target with an actual Cinch frontend and the explicit 85f00ca
library-only provider. Thirteen cases pass; two fail without changing any
original source, assertion or filter. The real object case, generic and packed
arrays, ownership/lifetime clients, tensor shapes and columnar projections pass.
All 31 runtime-emitter clients pass at 32M with poisoning and zero leaks; all
three corrupted nominal-layout controls refuse without an artifact.

The two open array cases require structural Copy/borrow/Thunk ownership and
stage/generate evaluation with the specified named unsupported-array refusal.
The source refuses before those respective semantic checks today. The direct
Rust packed-width API case stays explicitly classified inside the mixed target.
See `bootstrap/current-original-array-suite-results.json`.

All nine original interface/API groups also execute through the actual Cinch
writer and original Native public-reader/hash/mutation assertions. Six whole
groups pass: anonymous predicates, nominal identities, numeric preconditions,
private refinement closure, refined exports and region/effect metadata. Of the
eleven Rust tests, eight pass and three fail, with none ignored or filtered.
The three failed groups require existential/indexed constructors, typed foreign
declarations and source-family replacements of imported prelude homes. The
mixed kinds group separately passes its opaque-parameter-kind case.

The adapter copies exact provided library-only contracts for assertions that
inspect imported modules, and exact Cinch-emitted bytes for source modules;
these distinct roles never grant a Native frontend fallback for client source.
An earlier missing-imported-output transport failure is retained, and the
corrected complete replay makes the anonymous group green. See
`bootstrap/current-original-interface-api-results.json`.

These are complete failing-group receipts, not final Stage3 acceptance. All
original100 targets and five additions remain required. The new matching917f
chain has passed Stage0/1; later stages, fixedpoint/API/changed-source audit,
matching full suites and Acceptance5 remain open. The private parser row-
argument repair passes complete syntax-tree and corpus groups, including the
original nested Ref/Array/Thunk signature, and is awaiting its full43-source
check. It does not yet establish Copy or staging semantics.

## Checked byte observations retain actual body authority (2026-11-05)

Compiler `917f6e1f` accepts a renamed imported byte-length primitive even when
its defining interface also carries the actual checked primitive body. The
loader checks the exact foreign identity, pure scheme and canonical BytesLen
body before adding an intrinsic view, retaining the body for portable callers.
A well-typed constant-zero replacement body still refuses.

The source refinement fixture independently rechecks retained Core using the
successful elaboration's actual checked body and guard context. The standalone
empty-context Core checker is unchanged. The original constant-helper and
local-shadow inputs remain in the suite as valid certified-body cases; stronger
false-result, false-primitive-observation and recursive-opaque controls preserve
the name-authority boundary.

Seven complete affected groups pass: eleven Rust tests and all 48 byte-
observation runtime inputs. A separate exact 43-module compiler build and both
complete adapter/object groups pass all nine tests, including the real owned
client at 32M with poisoning and zero leaks. See
`bootstrap/current-byte-observation-repair-results.json` for the independent
source, executable, log and object audits and the distinct Native producer roles.

The original Native58 failure stays recorded on its frozen older Cinch inputs.
A fresh matching stage chain has passed Stage0; the previous independently
verified Acceptance1-4 chain belongs to compiler74f0. Matching final suites,
all original API behavior and Acceptance5 remain open. The required inventory
in `bootstrap/current-frontend-acceptance-scope.json` retains all original100
and adds five later groups: 96 portable and nine interface/API targets. Checker
cache helpers cannot remove groups from selection. Full arrays, copyOut/borrow
ownership, dictionary factories and generic object roots remain unfinished.

The related complete measure-contract group now passes all21 runtime-fed
checker inputs. Its original constant-one helper and true `fake n <= 4`
precondition remain as a positive; rejecting that source was an obsolete
unsupported-feature expectation after checked local numeric bodies were added.
Three new refusals under both imported-library variants cover false result
bounds, different actual arguments and recursive helpers without numeric body
authority. All four retained/new local controls independently agree with the
current Rust compiler. Original imported-contract mutation checks remain.
See `bootstrap/current-measure-body-authority-results.json`; compiler production
sources are unchanged, and complete matching suites remain required.


## Method-proof compiler completes the bootstrap audit (2026-11-05)

The frozen `74f0d661` compiler has an independently verified Stage0/1/2/3
bootstrap. Stage1 rebuilds every compiler module into Stage2, which rebuilds
Stage3. All twelve stage smoke checks pass. Stage2 and Stage3 have identical
LLVM, executables and all 43 canonical compiler interfaces. Both executables
hash to `ec948a720945da704bd4f9701d82a52bf00e2c7bb571dba35abfc7df7348a99c`.

The compiler's interface-only entry consumer passes with no compiler source
bodies. A real changed-source rebuild changes its diagnostic behavior. The
independent auditor also validates the emitted interface sets through the
Native public reader. This proves Acceptance 1-4 for that exact snapshot;
see `bootstrap/current-method-proof-bootstrap-results.json`.

The later published `5fc23130` library-object compiler and the private byte-
observation repair need their own matching bootstrap and complete suites.
The Native sweep's byte-observation failures remain visible. Original
API/100-target coverage and Acceptance 5 remain open; this record is not a
full-goal completion claim. Explicit 85f00ca seed/library pins are unchanged.

## Cinch emits and reloads a real closed library object (2026-11-05)

Compiler `5fc23130` now exposes `--emit-object` through its actual checked
pipeline. First-order function and concrete constructor entries share the
backend's ownership plan and real closure accessors. The object records exact
nominal descriptors, method-family slots, layout words, checked interfaces and
IR/object identities. It combines authenticated library-only provider bytes
with Cinch-produced code; source visibility still comes from checked exports.
Only the final executable registers the complete compatible layout table.

All 43 exact staged-driver compiler source inputs build in one fixture.
The two complete affected groups pass: eight adapter tests and a real object,
reload and linked-client test. An independent audit confirms all 1,842 physical
units and their accessors in the object, including seven new units and fourteen real
public/closure symbols, all content hashes and descriptor prefixes. The owned
bignum/ADT/array client passes at 32M with poisoning and zero leaks. The original
array object case also passes unchanged through this actual Cinch frontend with
a fresh current-Native host; its fourteen other cases were explicitly filtered.
See `bootstrap/current-cinch-library-object-results.json` and
`bootstrap/current-cinch-array-object-results.json`.

Reached source dictionary factories remain an explicit refusal, and generic
representation-root CLI requests, the full fifteen-case array group, matching
legacy runtime/self-built/full Native/API100 and Acceptance 5 gates remain open.
The existing `74f0d661` chain has built equal Stage2/3 executables and passed all
stage smokes; its final independent interface/changed-source audit continues
on that separate snapshot. The current Native6b full sweep retains its older
5cea Cinch fixtures and a byte-observation failure; it is not a green-suite
claim for this new compiler. Seed and library-provider pins remain unchanged.

## Portable test output modes use the selected compiler (2026-11-05)

The external test adapter now routes LLVM and object output requests to its
selected frontend, requiring an explicit library dependency configuration for
objects and checking that output companions exist. It retains source import
closure selection, argument boundaries and refusal of a Native fallback.
All eight complete adapter tests pass against the current Rust producer,
including the six original controls. See
`bootstrap/current-output-mode-adapter-results.json`.

This is transport coverage. The actual Cinch library-object CLI is still private
and under end-to-end validation; the published compiler source and stage pins
are unchanged. Matching full self-built suites, original API/100-target coverage
and Acceptance 5 remain required.

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
