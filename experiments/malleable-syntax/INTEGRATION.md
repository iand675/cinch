# Bootstrap branch integration

The user requested integration with the ongoing compiler-bootstrap work on
2026-11-02. The integration starts at `codex/cinch-bootstrap` commit `695ab0e`
and replays these experiment commits:

- `6322025`: compiler/editor fixes and their regressions.
- `e5522fd`: declarative syntax islands and initial runnable probes.
- `738d843`: choreography projection and tensor reverse differentiation.
- `78f1dca`: goal-aware tactic plans and their regressions.

The large historical snapshot `fb198a8` is deliberately excluded. Its experiment
provenance file is retained as a historical record. The newer bootstrap source,
checked body contracts, proof-scope repairs and evidence reports remain owned
by the bootstrap branch.

## Reconciliation

Strict local bindings retain both mechanisms: bootstrap's checked Nat equations
and value-alias provenance, and the syntax probe's transfer of `%1`/`%aff`
multiplicity through direct aliases. Neither side's behavior replaces the other.

`Proof.search` now places its goal in the same current value scope as native
evidence lookup. This uses bootstrap's `proof_values_in_scope` rather than relying
on the old snapshot's source-name comparisons. A new regression checks both
assumption and arithmetic tactics on a saved value after its original name has
been rebound. Existing rejection cases still cover using the rebound value.
Arithmetic goals retain opaque measures during scope rebasing; exact assumption
matching performs reduction separately. Another regression covers a string-length
bound under both strategies.

The experimental syntax frontend and tactic hook remain development Rust
compiler features. `bootstrap/stage0.lock.json` and `bootstrap/seed/stage0.patch`
retain the exact previously tested seed. This integration does not promote a
new seed or claim equivalent syntax/tactic support in the self-hosted frontend.
The bootstrap source itself does not depend on these experiment features.

## Validation and handoff

The integration checks include the native compiler units, tactic, value-identity,
guarded-let, ownership, surface, LSP, staging, dependent, linear-fact, evidence,
guarded-interface and incremental targets, followed by the full experiment
suite. Native compilation and execution run serially in one shared worker slot
alongside the bootstrap chat's bounded workload. Bootstrap tooling tests and
archive-plus-patch seed reproduction are checked separately.

The reconciled source at `ea512cd` passed 273 native tests: 142 unit tests and
all 12 selected integration targets (131 tests). The full experiment suite
passed 39 tests, including 13 runnable programs, 21 rejection fixtures, 14
syntax descriptors and the actual native LSP/Tree-sitter checks. All 19
bootstrap-tooling tests passed. The pinned archive plus patch reproduces the
same f88 seed, and the bootstrap tree is byte-identical to `695ab0e`.

[integration-verification.json](integration-verification.json) records the
source identity, commands, counts, log hashes and resource settings.
[verification.json](verification.json) records the fresh experiment run and
its checked source hashes. The local handoff merges this reconciled history
into `codex/cinch-bootstrap` after confirming its clean, held `695ab0e` head.

This is not a full current-source bootstrap-suite or self-hosting claim. No
remote push is part of this request.
