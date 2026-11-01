# Compiler checkpoint — 1 November 2026

The compiler now carries checked anonymous refinement contracts through interfaces
and retains actual argument/result identities in scoped Core witnesses. Private
nominal contract closure and definition-owned fixities remain enforced. The pinned
Rust seed is refreshed to the exact validated Native source.

Six focused validation gates pass, including twenty freshly executed compiled
consumer/import/source/Core/dictionary/effect/pipeline/ladder targets, with audited
reuse of ten byte-identical Native contract targets. All 94 independent Native
integration targets pass. All 19 additional/repeated integration targets also pass,
completing the [152-target Native-assisted integration matrix](integration-matrix-results.json).
The prior Native units/runtime/tooling/binary/doc gates passed on unchanged selected
inputs; self-built frontend acceptance remains unproven.

At the preceding anonymous-contract checkpoint, stage0 and stage1 builds pass
all six smoke checks. Its stage2 attempt
fails before body checking with 70 header diagnostics, down from 76; it produces
no stage2 IR or compiler executable. Stage3 fixed-point agreement, changed-source
behavior and the full suite through a self-built frontend remain unproven.

[checkpoint-results.json](checkpoint-results.json) records exact gate outcomes,
seed identities and failure categories; [native-integration-results.json](native-integration-results.json)
records the independent Native test scope. Full logs and frozen snapshots remain
local. [The design record](../design/self-hosting.md) details remaining work.

The installed compiler now also supports deferred generic NonZero formation
and result-directed application checking. All 154 Native-assisted integration
targets pass, including the new abstract NonZero type/Core and generic Div/Rem
consumer regressions. The exact validated snapshot is
`41173572185db6c95dbf9606e4bf3830d09b609b42c8b2b91844dc92037cdcfa`
(565 unchanged inputs). The Native seed is unchanged. Fresh stage0 and stage1
builds pass all six smoke checks. The new stage2 attempt exits 4 before body
checking with 66 header diagnostics, down from the preceding checkpoint's 70.
Twelve positions are imported/synthetic and fifty-four map to source; nineteen
still concern unchecked index calls. Stage2 produces no IR or compiler executable.
The unchanged staged snapshot is
`c1d04f9d2ffe44ec4f981cfdbd37b7744713017cae3175f50fb6268f24a36f72`;
the compiler bundle is
`e707fad57653acb7c6a95fe3f9504e8aae68c5f528c7b39a7d07f632f9cf6fcd`.
Evidence is in `/tmp/cinch-generic-nonzero-stages/`. Diagnostic counts include
cascades and do not measure completion. Stage2/3 acceptance remains unproven.

Unrelated website, branding and reader-documentation work is managed separately.
Compiler changes are pushed incrementally with their validation status recorded.
