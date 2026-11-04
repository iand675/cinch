# Self-hosting

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

## Verified repaired-source bootstrap (2026-11-03)

The fresh final repaired-source chain passes every Stage0/1/2/3 smoke check,
byte-exact Stage2/Stage3 compiler IR and executable comparison, and all43
canonical compiler-interface comparisons. Stage3 checks its actual entry using
those emitted APIs without compiler-module bodies. Stage2 also rebuilds changed
compiler source; that executable exhibits the requested changed diagnostic.
See [the complete fresh-chain evidence](../bootstrap/final-repaired-bootstrap-results.json).

This run verifies frozen source
`c6260e01e2fdab4397cd2d8087f835510fd80b4ad52d8729a6222a349b733703`,
including the reexport-fixity and Rust class/data-name repairs. The operator's
declaring module remains the sole authority for fixity declarations.
All127 ordinary compiler-facing groups and the171 runtime,41 tooling and39
syntax/editor checks pass on the same snapshot. Exact-final-Stage3 external
acceptance also passes its original test-runner, fixity/facade and portable class
cases; see [that evidence](../bootstrap/final-stage3-external-results.json).
The81-group bootstrap-specific sweep still runs. Acceptance below remains
unchanged; the goal stays open until the complete sweep and final audit pass.
The [next steps](next-steps.md) and
[live dashboard](https://cinch-bootstrap-status.ian92821.chatgpt.site) track them.

The [earlier combined chain](../bootstrap/fresh-combined-bootstrap-results.json)
remains evidence for its preceding6c6644d snapshot; it does not replace the
repaired-source result above.

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

## Earlier Boolean branch checkpoint

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

## Previous producer and collection checkpoints

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
Use at most five heavy jobs across every snapshot and stage chain: four
shared worker slots and one explicitly reserved fifth lane. This follows the
user's later request for bounded concurrency. Each job still uses one Cargo
build worker, one test thread and one Rust code generation unit, including
root-launched builds. Keep jobs at low priority and each Cinch heap at most 2G;
the per-process heap cap does not replace the shared worker limit.
The earlier gates interrupted by the machine crash are not completed verification.

## Historical refinement boundary

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

The following schema-5/schema-6 entries record earlier checkpoints.

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
compiler/runtime and self-hosting gates were subsequently executed on the
525-input frozen snapshot
`c314883e3b3c419e8668e14ca1c7f59231ada5d0768d10a569d3b644d33e743f`.
All 1,121 compiler tests, 166 runtime tests, 19 tooling tests and six fresh
stage0/stage1 smoke checks pass, with zero failed or ignored tests. The final
source audit is unchanged. The self-hosting gate fails: stage2 exits 4 with
136 header diagnostics and produces no compiler executable. Stage3,
fixed-point and changed-source checks remain unreachable. Evidence is in
`/tmp/cinch-d28-full-verification/`. This run predates the Pi and contextual
source-kind changes described below; it does not verify them.
These are internal conversion checks. Source dependent kinds, Pi binders,
refinements, actual bound/global typing and proof-context construction are
still required; no new source proof acceptance is enabled by this slice.

The next working-tree slice represents dependent function types as
`UPi binderId displayName domain effectRow codomain`. It includes structural
carrier encoding, capture-avoiding simultaneous substitution, alpha
conversion, scoped unification, dependent application and Core visitors.
Binders scope the row and codomain, never their own domain. Unification
opens both bodies with a fresh rigid above every existing inference level;
pre-existing metas cannot capture it. Domain inference is checked again
before opening, since solving a domain can introduce a binder collision.
Generalization retains unresolved holes whose carriers depend on Pi-local
binders rather than hoisting them outside their scope.

The focused `bootstrap_pi` runner passes 34 checks at five nonce inputs and
26 complete JSON packets supplied after compilation, in 32.72 seconds.
Its private source snapshot is
`0dbce34e3abb0d3645e26923146914caa3ae5a5ec42820d9a309e3b940f4034c`;
evidence is in `/tmp/cinch-pi-regressions-2/pi.log`, with an unchanged final
source audit. After correcting source-kind pattern syntax, the private
snapshot `af00d781da6b3e6c5e1bf0d910a5dd215c0a3dd8baac405169446afeba686cc6`
passes Core, effect-type, source-effect and inference regressions. The
subsequent 527-input snapshot
`dba00d2242373bc985adb6839bbee0cf279ad88f648fcd05c14b9d39a591d0be`
passes the imported-contract, typed-index and carrier-kind regressions,
with a final unchanged-source audit. Evidence is in
`/tmp/cinch-pi-regressions-3/` and `/tmp/cinch-pi-regressions-4/`. The compiler
modules are unchanged between those two snapshots; the latter corrects the
new import tests to re-export their values, forcing scheme materialization.
Source Pi checking remains disabled pending actual source integration.
Source binder/witness association, contextual higher-order inference,
dependent constructor kinds, multiplicity and proof checking remain required.
The active full verifier uses the preceding frozen D28 source snapshot; its
results will not verify these later Pi changes.

The following source-kind change passes those focused regressions. Kind
annotations now elaborate with preceding quantified parameters in scope:
`forall a (x :: a)` retains the identity of `a` in the carrier of `x`.
Function carriers such as `a -> a` remain value kinds; universe arrows such
as `Type -> Type` remain constructor kinds. Imported declaration parameters
are assigned identities in declaration order, with duplicate names rejected.
The inference runner adds five accepting and four rejecting source cases,
plus an assertion over the actual checked carrier annotations. These changes
do not enable source Pi binders, dependent pattern matching or proofs.
Resolved imported schemes now qualify carrier annotations in the preceding
binder context too, including nominal carriers through their defining
identities. Six synthetic interface-consumer cases exercise generic,
function and nominal carriers and reject forward, self and unknown names.
All 28 imported-contract cases pass, including these six carrier cases.
The expanded index runner passes 37 checks at five inputs and 28 runtime
JSON packets, and the carrier runner passes its 33 checks at five inputs.
A fresh staged build on snapshot
`7b687e73f56f0b95f0a448e207d3d4cfb84d6a8f0b3697a41589d3a965ace379`
passes all six stage0/stage1 smoke checks, with a final unchanged-source
audit over 527 inputs. Stage2 checks compiler bundle
`dc4c87c3f547a545451b96de8253f28edee91c01eaedbbca57e9bc3caf0f34f0`
and still exits 4 with 136 header diagnostics, with the same categories as
the preceding full baseline. No stage2 compiler executable is produced.
Evidence is in `/tmp/cinch-pi-source-stages/`. The internal Pi machinery
does not yet admit source dependent binders: 49 diagnostics name that
missing elaboration. Connecting those binders to actual term arguments in
typed Core is the next step. These focused and staged results do not
establish self-hosting or a full current-tree compiler/runtime pass.

The next Core change gives `CPVar`, `CPAs` and `CLet` stable numeric local
identities and uses a distinct `CLocal` term for their references. The
elaborator retains each local's identity and inference level, including
ordinary parameters, lambda/case patterns and generalized lets. Mapping,
freezing, meta collection and dictionary traversal preserve those identities.
Core closure now also checks lexical local scope: a let does not bind its
own RHS, branch/lambda binders stay in their bodies, duplicate active
identities reject, and a bare name with role `local` cannot bypass the check.
This is not an independent type, kind or proof rechecker.

The expanded Core runtime reader passes 19 source cases, including shadowing,
as-pattern binding, ten malformed/local-scope invariants and the existing
1,501-definition stress case, in 64.31 seconds. Effect-type regressions also
pass. Evidence is in `/tmp/cinch-term-identities-3/`, frozen snapshot
`b9d7fae71f11662ed685bac2f9005a450a53ad70d2c14ea4a1e2a0bd7e11ee91`.
The broader effect, inference, imported-contract, parser, Pi and index/carrier
regressions also pass, with a final unchanged-source audit. The new as-pattern case exposed a
parser mismatch: Cinch now parses native `name@atomic-pattern`, and refuses
the former nonstandard postfix `pattern as name`. Source dependent binders
and index reification are still not enabled by this identity change.

The following source-Pi slice now converts named function binders with
identities fresh for the complete quantifier context, binds actual pattern
values, and instantiates result types and effects from checked Core
arguments. Local indices use `CLocal` identities and validated scheme
instantiations. Mathematical literals decode without machine narrowing;
unknown calls and unchecked operations cannot be reflected merely because
they have a pure-looking type. Core closure checks bound/rigid index scope,
including local-index escape through a let or inferred lambda. Erased forall
indices do not authorize runtime local references.

All 20 initial runtime source cases pass in 75.91 seconds in snapshot
`4d8aebb65c056ff68290428f45b211c23ecbb0a5df41600a011da219885b35ad`,
with evidence in `/tmp/cinch-source-pi-2/source-pi.log`. The broader run caught
an error-recovery hole: malformed class applications could be classified as
checked constraints. They now require `valid-type` too. A further guard
refuses dependent declaration parameter kinds until binder-aware kind
application is implemented; ordinary `KFun` cannot preserve their scope.
All 21 source cases, including those guards, pass in 75.75 seconds on the
529-input snapshot
`89641068821fc2f05cc2ea872e7661abfa79ed4363877ef2f3a443253fe0ba29`;
evidence is in `/tmp/cinch-source-pi-3/source-pi.log`. All nine affected
regression targets also pass: Core, effect types, effects, inference,
nominals, parsing, Pi, value indices and value kinds. The final source audit
confirms the same 529 inputs are unchanged. This is focused verification,
not a full current-tree verification result.
Source proof contexts, dependent constructor patterns, total-call reification,
dependent-let beta checking, inferred dependent lambdas and the shared ABI
backend remain required.

A fresh staged attempt on the 529-input snapshot
`de7349e0d853ba3504c7580bc6bf1014b3dc57c64d9903f07635bb96db127669`
passes the type-syntax, combined ladder and end-to-end pipeline regressions,
then rebuilds stages 0 and 1 and passes all six executable smoke checks.
Stage 2 exits 4 with 94 header diagnostics, down from 136 in the preceding
attempt. No stage-2 executable or IR is produced. Source named-binder
refusals are gone; proof predicates and refinements, declaration support
and shared runtime lowering still block the self-build. The diagnostic
count includes cascades and is not a count of independent missing features.
Evidence and the final unchanged-source audit are in
`/tmp/cinch-source-pi-pipeline-stages/`. The compiler bundle hash is
`a67b08ceaedde83e4ba9d660789d666e245d30497cd8392944bd39033efda108`.

The next slice retains checked index values for strict, monomorphic let
aliases. A literal, actual outer local or reifiable mathematical operation
can supply the alias's type-level value; the runtime `CLet` and `CLocal`
slot remain in Core. Failed reification retains an opaque local and rolls
back its speculative store, so an unknown call is never unfolded as an
index. Source mathematical operators now retain an explicitly inferred
`Nat` carrier for total operations; `Nat` subtraction still requires
ordering evidence. All 30 runtime-source cases pass in 76.36 seconds on
snapshot
`8643e7d003a048c2eb812a69cde60bd79f72dfdb85c3e8ec5c3b5f6756ab438f`,
including alias chains, annotation scope, preserved runtime slots and
negative identity/carrier/partial-operation checks. All nine affected
regression targets also pass, with a final unchanged-source audit of the
529 inputs in `/tmp/cinch-let-index-2/`. This slice has not yet had a
fresh staged attempt; arbitrary dependent let expressions and complete
dependent conversion remain required.

Source foreign declarations now have checked contracts for ABI 5's seven
pure text intrinsics: SHA-256, SHA-512, SHA-1, MD5, HMAC-SHA-256 and both
F64/text conversions. Registration checks the source scheme against the
kernel contract and rejects polymorphic/constrained declarations, wrong
types, unsupported ABIs or symbols, duplicates and source definitions that
would replace an intrinsic. Core uses retain the canonical intrinsic
identity; declaration contexts retain the runtime extern and ABI version.
The compiled Cinch checker passes 20 runtime-source cases in 76.81 seconds
on the 531-input snapshot
`60dfbc003b9f17b229d27aa3bf0918203373779ef492e9682564b1e4bff524d4`.
The cases also compare retained externs with the native runtime contract.
Evidence is in `/tmp/cinch-intrinsics/intrinsics.log`. The affected Core,
effects, inference and 30-case source-Pi regressions also pass, with a final
unchanged-source audit of all 531 inputs. The fragment emitter continues to refuse
foreign declarations before producing IR. A fresh pipeline and staged
attempt now passes type syntax, the combined ladder, the end-to-end
pipeline (including intrinsic imports through module aliases), and all six
stage0/stage1 executable smoke checks. Stage2 exits 4 with 91 header
diagnostics, down from 94; the three source foreign-declaration refusals
are gone. No IR or stage2 compiler is produced. The 531-input staged
snapshot is
`535b746836e11a6d3b785c309f1ff35cfa1b939e58325c52f43d18f6eb1d3e5d`,
with a final unchanged-source audit in
`/tmp/cinch-intrinsic-pipeline-stages/`. The compiler bundle hash is
`b0576b53867d0c234fac44273e27912948f199c678c3c6b53b09cee8bfaa133c`.
This verifies the checker extension, not the remaining shared-ABI backend.

The next source-proof slice handles `Nat` subtraction obligations under
checked comparisons. Branches, guards and short-circuit expressions carry
lexical facts identified by actual local IDs; strict numeric lets also
retain checked equality facts. The elaborator searches for a certificate,
checks it, and retains it in `CProved` alongside the original operation.
Core reconstructs the operation's goal, typed numeric locals and branch
facts from its retained tree and checks the certificate again. Certificates
cannot supply their own hypotheses or claimed goal. Machine arithmetic and
unknown calls are not reflected into this arithmetic context. This remains
the compiled certificate-check tier, not a reflected soundness theorem or
an independent full typing kernel.

All 25 runtime-source cases pass in 83.51 seconds on the 533-input snapshot
`4485f9e0b0bd45a6f1dc495b9f313d3b26e747681cf2747ea4643e8fca3b3cd9`.
The runner also checks seven Core invariants at runtime-chosen local IDs,
including changed goals, moved certificates, fabricated hypotheses,
uncertified subtraction and machine-scalar rejection. Evidence is in
`/tmp/cinch-proof-scope/guarded-arithmetic.log`; affected regressions are
still running. A later test-harness change copies the proof dependency only
for projects that need Core or inference. No newer staged result is claimed.
Source `using` predicates, refined signatures and general dependent
conversion still require their full contract and erasure implementation.

That 25-case snapshot subsequently passes all seven affected regression
targets, with a final unchanged-source audit. An additional inferred-lambda
case exposed a sealing bug: later inference can identify another local as
`Nat`, adding a domain fact and shifting the earlier certificate's hypothesis
positions. Snapshot
`7fd8fa89dfbffa638d392412278e3fbc7c839adc778082066348af0309e99f91`
rejects that valid source with an unresolved-Core diagnostic. Sealing now
finishes type substitution first and checks certificates in the resulting
actual scope. A stale certificate is regenerated only if the retained
operation is provable in that scope. The independent Core validator still
rejects invalid certificates without repair. All 26 source cases and the
seven runtime Core invariants pass in 83.22 seconds on snapshot
`5bf59aea532baea543aa269140ade154d4b0c8082d8c2f39f92b42fc95b753ef`;
evidence is in `/tmp/cinch-proof-sealed-scope/guarded-arithmetic.log`.
All seven affected regression targets also pass, with a final unchanged-source
audit of the 533 inputs. A fresh pipeline/staged run also passes type syntax,
the combined ladder, the complete pipeline (including guarded `Nat` lowering
refusal), and all six stage0/stage1 smoke checks. Stage2 still exits 4 with
the same 91 header diagnostics and emits no IR or compiler executable.
The final source audit is unchanged. Evidence is in
`/tmp/cinch-proof-pipeline-stages/`, snapshot
`bca5f0d9d2912875b733a95247c703179434d43c56ed529861de7a89c39f4d32`;
the compiler bundle hash is
`4df4534b433eee01e2c4d96f1e318e9f11586b7e0c18de02cd5546c37dd9f1e7`.

The next internal type representation adds `KProp`, typed propositions and
`URequire` function contracts. Predicate operands and required contracts
survive normalization, substitutions, alpha conversion, scope/occurs/level
checks and complete dependent-kind carrier encoding. Unification preserves
requirements; it neither strips them nor treats logical entailment as
definitional equality. Core rejects propositions and undischarged required
types as ordinary runtime values. All 33 runtime-nonce checks pass at five
identifier offsets above 2^128, first in an isolated frozen candidate and
then in the integrated 535-input snapshot
`d14fa6cc5ae76e969ae12fa045aae1c6ea8c9ce3755cdf93d885a9f301009f84`
in 45.89 seconds. Evidence is in `/tmp/cinch-proposition-types/`; all
13 affected regression targets pass and the final source audit is unchanged.

The next source continuation introduces anonymous arithmetic preconditions
at declared function boundaries and discharges them at fully applied calls,
including strict calls whose result is ignored and conditional nullary
values. `CRequire` retains the declared contract and `CDischarge` retains
proof certificates. The Core pass reconstructs each goal from the actual
callee, arguments and lexical facts; arbitrary proof introductions, moved
certificates and substituted arguments are rejected. Certificates carry no
cached caller hypotheses. Partial applications and generic identity retain
contracts. Certified `Nat` subtraction used as a dependent argument is
bound once with its checked equality fact, preserving evaluation order;
symbolic partial subtraction is not added to type conversion. Effect rows
are retained. These are compiled certificate checks, not a reflected
soundness theorem or an independent full typing kernel.

The isolated frozen candidate
`fbe02c4f3ac2e5a96554a14314ec01a7d60443b173fde85efe227794e3ce891e`
builds and passes 25 source cases plus nine Core invariants whose identities
are chosen from source supplied after executable compilation. Evidence is
in `/tmp/cinch-function-preconditions-validation-3/`. Its implementation is
integrated; the focused test passes in 106.25 seconds on snapshot
`404dcf349f07da6a62d4dc24ed3100d22fa985344e8784c6826c2eb0a7db6a34`
in `/tmp/cinch-function-preconditions-2/`. All 25 source cases and nine
Core invariants pass there too. All 14 affected regression targets pass;
the final audit confirms all 537 snapshot inputs are unchanged. The first integrated test attempt
failed because its temporary project omitted compiler dependencies, before
checking source cases; the fixture now includes those dependencies.

Named erased proof bindings, interface predicate preservation, refinements,
total reflected calls and typed Core runtime lowering remain required.
Unsupported forms retain named refusals. The fresh staged continuation
passes type syntax, the combined ladder, the complete pipeline and all six
stage0/stage1 smoke checks. Stage2 exits 4 with 86 header diagnostics,
down from 91, and emits no IR or compiler executable. Its remaining errors
include 17 unchecked index-expression refusals and cascading ill-sorted
predicates/contracts, six imported using contexts, named refinements,
dependent declaration kinds, decisions, methods, effects and instances.
The 537-input snapshot
`8184038c9013b01ed9f88928600de2a618d22bfeb1ee15af6cda59542cdacf58`
has a final unchanged-source audit in
`/tmp/cinch-function-preconditions-stages/`. Its compiler bundle hash is
`6d2f482c8bda77288a07853b741c56d5cc0ebf91407e4eb79797a71e4aa8d9c4`.
These diagnostic counts include cascades, not independent feature counts.

A subsequent real-interface consumer test exposed a Core scope bug:
application validation called inference `open-pi` with a fresh level-zero
store, rejecting valid surrounding type rigids in a polymorphic caller.
Closed Core application checking now performs capture-avoiding substitution
in its already checked lexical context. Source inference retains its strict
escape checks. The isolated before/after consumer confirms the repair; the
integrated focused gate passes all 25 source cases plus ten Core invariants
in 107.51 seconds, including a new generic caller at level one. Snapshot
`94a659811cfee8eae0275437eb0d904805f7845a5ba0f9be46d743921a571e26`
passes all six affected regression targets in `/tmp/cinch-core-closed-scope/`;
the final audit confirms all 537 snapshot inputs are unchanged.
The 86-header staged evidence above predates this narrow Core repair.

The next interface producer is still isolated. Its four native targets pass,
but its compiled Cinch consumer found that implicit Prelude operators remain
method applications, so arithmetic guard reflection cannot recognise them.
Forwarded assumptions, literals and partial applications work; arbitrary
method bodies are not mathematical primitive evidence. Checked global
instance selection and primitive semantics must be implemented before this
consumer path can be admitted. The native producer, qualifier and seed pin
have not been integrated from this draft.

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
