# Compiler checkpoint — 1 November 2026

The compiler now carries checked anonymous refinement contracts through interfaces
and retains actual argument/result identities in scoped Core witnesses. Private
nominal contract closure and definition-owned fixities remain enforced. The pinned
Rust seed is refreshed to the exact validated Native source.

Six focused validation gates pass, including twenty freshly executed compiled
consumer/import/source/Core/dictionary/effect/pipeline/ladder targets, with audited
reuse of ten byte-identical Native contract targets. All 94 independent Native
integration targets pass. The 19 additional/repeated integration targets are still
running; a full-matrix pass is not claimed.

Fresh stage0 and stage1 builds pass all six smoke checks. The actual stage2 attempt
fails before body checking with 70 header diagnostics, down from 76; it produces
no stage2 IR or compiler executable. Stage3 fixed-point agreement, changed-source
behavior and the full suite through a self-built frontend remain unproven.

[checkpoint-results.json](checkpoint-results.json) records exact gate outcomes,
seed identities and failure categories; [native-integration-results.json](native-integration-results.json)
records the independent Native test scope. Full logs and frozen snapshots remain
local. [The design record](../design/self-hosting.md) details remaining work.

Deferred generic NonZero formation and expected-result application checking are
being developed separately. Their focused checks pass, but the broader matrix is
still running. Those changes remain on the separate development branch and are
not included in this installed checkpoint.

Unrelated website, branding and reader-documentation work is managed separately.
Compiler changes are pushed incrementally with their validation status recorded.
