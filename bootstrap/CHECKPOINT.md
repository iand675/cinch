# Compiler checkpoint — 1 November 2026

This is an in-progress compiler checkpoint. Cinch now retains typed byte-length
observations through inference, substitution, private refinement contracts and
Core checking. The defining Native primitive emits a closed kernel operation;
ordinary same-named functions cannot supply those facts. ABI 5 supplies the
nonnegative, machine-sized length bounds only for actual scoped byte buffers.
Interface schema 7 and definition-owned fixities are retained.

Strict monomorphic lets carry checked value witnesses in a separate proof
context. Core rebuilds those witnesses from the actual retained right-hand
side. Nine source alias cases pass, including refined buffers and precondition
calls; wrong buffers and shadowed aliases remain refused. An adversarial Core
check passes at five runtime-generated identities above 2^128: the real alias
is accepted and a copied certificate with a different buffer RHS is rejected.
The updated anonymous-interface test checks the exact bytes-new allocation
promise rather than its preceding unsupported status.

The exact candidate Native seed is independently reproduced from its pinned
archive and patch: source SHA256
`74b4457a63c4898091ad5a31324a8ca098cf1aecfa789670926fff3be2f9b1ab`,
patch SHA256
`61685f05a4a781d58fa0417df2282402e9e2b76ad0fb1bb07303bcdfc6e25825`.

Fresh stage0 and stage1 builds pass all six smoke checks. The real stage2
attempt exits 4 before body checking with seven header diagnostics, reduced
from 66 at checkpoint 6d67ed6: decide/Yes/No evidence, two imported using
contracts, Text.Parse's effect declaration and Text.Json's instance declaration.
All byte-length/index-conversion header diagnostics are gone. Stage2 still
produces no IR or compiler executable; diagnostic counts include cascades and
do not measure completion. The compiler bundle is
`ecebef95541d62e92f8020553668c31e4ab31fa152c136a2994ad906fa9b0a66`.
The independently audited staged snapshot contains 570 unchanged inputs,
SHA256 `85a0ddb9f861a44c76fb66635f8fde71c495073307eb15d05aa8db4d6fc7c232`.
Evidence is in `/tmp/cinch-byte-observer-stages-12/`.

The byte-length NonZero contract now passes 41 real caller cases, including
wrong-buffer, shadowed-buffer and missing-condition refusals. Numeric interface
regressions check the closed primitive contract and keep ordinary same-name
functions unresolved. These replace expectations from the preceding unsupported
tier; no compiler, bootstrap or runtime implementation changed for these updates.

The full 158-target Native-assisted integration matrix and five general gates
are continuing against frozen candidate snapshot
`db363ca63a9fc22322ad420233ff206d00ffa879d7761a66c56a962e029ebe52`.
Twenty completed matrix targets and the continuing focused gates retain exact
compiler, bootstrap, prelude/runtime, shared harness and own-fixture bytes; only
the two revised Rust test fixtures differ. The revised NonZero target is rerun,
and all remaining targets run fresh. Staged implementation sources are identical;
the staged snapshot additionally differs in its verified seed lock/patch.
The matrix is not yet a full pass. The preceding snapshot's ten passing gates
and outdated bytes-new refusal assertion are retained as diagnostic history,
not treated as a completed suite. The preceding installed checkpoint passed
all 155 integration targets plus compiler/runtime/tooling gates; that evidence
belongs to its earlier sources. Current result records distinguish these scopes.

Remaining source declarations, decision/proof forms, checked imported measure
and conversion contracts, an independent global typing kernel and shared-runtime
typed lowering are required. Stage2/3 fixed-point agreement, changed-source
behavior and all tests through a self-built frontend remain unproven. Work uses
at most five concurrent jobs, one Cargo worker and one test thread per job, and
generated Cinch heaps limited to 2G.

The compiler/bootstrap/runtime/test/tooling files match the frozen staged
sources. Publication adds these status records and retains unrelated top-level
README content; its documentation files are not the frozen snapshot itself.

[checkpoint-results.json](checkpoint-results.json) records current seed, stages
and focused results. [integration-matrix-results.json](integration-matrix-results.json)
records the pending matrix and preceding checkpoint evidence. Full logs and
frozen snapshots remain local. This checkpoint is intentionally published while
regression verification continues.
