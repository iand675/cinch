# Next steps (as of 2026-11-01)

Source instance dictionaries are now published as a work-in-progress checkpoint.
The typed class and instance records retain defining owners, exact method slots,
specialized method schemes, factory parameters and captured dictionaries.
Source members are checked against the defining class interface, including
method-local quantifiers and requirements. Explicit `with` accepts checked
class dictionaries and retains the existing Io region-escape check.
Ordinary values remain invalid dictionary arguments; only implicit instances
participate in implicit search. Fixities remain owned by their declaring module.

The focused instance target and six existing regression targets are still under
validation. The complete suite for this implementation has not run, and its
fresh staged build is queued after the focused checks. The previous checkpoint's
161 passing integration targets do not establish that this implementation passes.
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
