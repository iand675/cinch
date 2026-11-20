# Checked MVar effect transport candidate

This isolated patch is based on qualified source2a558db0. The original
`check_interface.rs` is byte-identical to the corresponding file in the
Native MVar source lane. No Root file, runtime, ABI version, executable body,
active pin or object metadata is changed. No Cargo build has run here.

## Semantic model and narrow repair

`MVarOp::sig` supplies the sealed `MVarCell :: Type -> Type` carrier and the
real argument/result types. `check_decls::prim_scheme` constructs the actual
`EffRow::closed` with canonical `EffLabel::plain("MVar")`, adding `Diverge`
exactly when `MVarOp::blocking()` is true. These semantic heads are interned
identities: declaration-owned effects resolve to their qualified owner, even
when the source spelling is MVar. No source-owned effect gains kernel status.

The existing exporter already recognizes the sealed carrier. Its closed-row
source renderer deliberately cannot serialize MVar; therefore all fourteen
public wrapper contracts currently have `resolved: null`. This candidate
leaves that unpaired renderer unchanged. It instead traverses the paired
source/checked `Ty::Fun` or `Ty::Pi` structure when a checked MVar effect is
present. A new path requires both rows closed, definition-site effect heads
unchanged, zero MVar arguments, full normalized semantic label equality, and
the existing Region-kind checks for every other label. Unsupported shapes,
open rows, custom effects and unpaired aliases retain no portable authority.
No type conversion, inference mutation or source-name substitution is used
while printing. Total-body and serialized LLVM authority are unchanged.

The private fourteen cancellation primitives carry MVar and Diverge, without
an invented Io dictionary or Cancel region. The public four cancellation
wrappers really require `using (io :: Io r)` and retain `Cancel r` plus
Diverge. Those public obligations are serialized, not removed or replaced.
Ordinary local operations accept nonshareable payloads such as Bytes; they
must not acquire an invented Share/Copy requirement in their schemes.

## Prepared meaningful gates

Three private unit tests validate complete semantic row matching, divergence
and region identity, open-row/arity/kind refusal, latent callback detection,
and the unchanged refusal of unpaired source-only MVar rendering.

Six integration tests produce genuine contracts from the exact public module
renamed to a nonembedded defining owner, plus its actual fourteen foreign
primitive declarations. They check all fourteen private/public schemes,
canonical/reexport identities, higher-order rows, Io/Cancel contexts, no body
authority, and unchanged custom/open-row null contracts. Source-absent tests
delete the producer source and use an actual compiled Native importer for
all fourteen public calls, effect/role/owner/Io refusals and contradictory
foreign-kind/ABI/operation/role/blocking-effect controls with coherent hashes.

The Native importer tests explicitly depend on the separately reviewed
Native MVar source increment (`1d4415dd`, parser correction `95792ff3`, with
its source-body curation prerequisites). Base2a alone lacks that feature.
The tests do not silently replace the importer or treat JSON roundtripping
as source-absent elaboration. Initial Rust-only unit/producer tests can run
before that integration, but the full target must subsequently run; no tests
are ignored. Actual commands, source hashes and failures must be retained.

## Ownership boundary remains explicit

Source-known public MVar contains `MkMVar (MVarCell a)`. Copy of the handle is
unconditional; sharing or transferring it across a task requires payload
Share, since another handle may remain outside. Source-known Nat must pass
and Bytes must fail. Sealed source-absent `MVarCell Nat`/Bytes controls test
the actual kernel rule independently.

Current opaque public headers expose no private field types. The Native
checker explicitly refuses task proofs for a datatype without constructors,
so both imported public MVar Nat and Bytes currently fail closed. The test
records that scoped refusal without inventing a private field. The separate
`mvar-source-absent-ownership-obligations.json` keeps imported public Nat
success as a mandatory OPEN full-feature gate. Passing effect-row tests
cannot claim complete public task sharing or full Native-family admission.
A real payload-shape/proof transport is required before that positive gate
can change; no metadata or defining hash may be restamped.

Rust currently has no production source-absent interface-import API. Its
ordinary structural-use behavior on a hypothetical empty constructor table
is not evidence of a reachable imported-MVar vulnerability. This patch
neither claims such a bug nor changes that unrelated code.

## Pending validation

Rustfmt syntax parsing and `git diff --check` are the only local checks.
The intended first warm-cache gate is the three `mvar_effect_contract_tests`
plus Rust-only genuine producer/owned-effect/source-known ownership tests.
After the reviewed Native source increment is available, run the entire
`interface_mvar_contracts` target and preserve all source-absent outcomes.
Any new Rust producer then needs a fresh source envelope, release snapshot,
artifact invariance and actual provider units before admission. The existing
qualified089/2a/d91 snapshot and c31d seed proposal remain separate history.

## Failure evidence and predicate interaction

Each source-absent call preserves a unique source, manifest reference, exact report bytes,
stdout, stderr and exit status before the next call can overwrite shared driver paths.
Each genuine or contradictory interface version has its own immutable directory; repeated
positive calls share that exact version rather than duplicating the full interface closure.
The producer source is retained separately before source-absent removal. A panic guard keeps
the entire producer/importer project outside the common stale-project prefix, including
failed builds and all attempts, with an old/new root mapping for unchanged absolute manifests.

`Ty::Prp` and the refined carrier paths print only that checked result and recursively print
its carrier; the enclosing function or dependent-function branch validates the paired complete
row before recursing into its result. A new direct checked-refinement control requires matching
MVar+Diverge and refuses removed/added effects, a declaration-owned MVar, or an absent source
row. This is prepared source coverage only; no new test execution or compiler qualification
is claimed by this follow-up.

## Single-task transfer witness

The ownership target additionally checks one `%1` handle captured by exactly one task,
using `is-empty` so the returned payload does not itself obscure capture admission.
Sealed `MVarCell Nat` and genuine public `MVar Nat` must admit; their Bytes counterparts
must refuse because a consumed handle can retain aliases elsewhere. An ordinary owned
Bytes buffer must still transfer once. All five actual importer outcomes are saved before
assertions, so either a Nat availability refusal or a Bytes acceptance remains inspectable.
These new controls are unexecuted. The earlier two-task public-Nat fail-closed observation
remains a distinct sharing-availability gap, with required final behavior still positive.


Current scoped qualification is recorded in `../evidence/native/native-private-usage-qualified-results-v1.json`:344 release units,11 Rust source/profile controls and8 fresh Native importer controls pass. Earlier pending/failed stages in this plan remain historical. The actual importer uses a32-module subset of the globally checked52-source family; full Native provider, exporter, fixed-point, wide representation and emitted runtime gates remain separate.
