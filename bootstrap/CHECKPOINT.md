# Compiler checkpoint — 1 November 2026

The installed Native compiler identifies byte length by its checked primitive
operation. Fresh foreign aliases preserve its stable facts and execute the
actual primitive when an index witness is needed. Type-position observations
validate operand and result carriers against the converted signature telescope.
The eight focused regressions retain wrong-carrier, wrong-arity and ordinary
same-name refusals alongside executable witness and allocation-promise cases.

All 155 Native-assisted integration targets pass, together with fresh compiler
unit, binary, documentation, runtime and tooling gates. The validation snapshot
contains 566 unchanged inputs:
`321673612da1f7a3c2df37f9d82b4e47dbc7cf235bbc50f39f063f241a09a195`.
Compiler units pass 140 tests and runtime suites pass 166 tests, with no ignored
tests in those gates. This is not the suite through a self-built frontend.
Evidence is in `/tmp/cinch-primitive-length-validation-4/`.

The independently verified, source-matching Rust seed has source SHA256
`6851c25868101865ee3c137d61a372687ab560e9deb5d98dda6e808d5c4f5335`
and patch SHA256
`94b0d32ad0cd5620db4d04f9909c957090fac3d81fcfcb25c7213720dc0b3a0b`.
Archive plus patch reproduces every tested Native source byte. Interface
schema 7, runtime ABI 5, definition-owned fixities, generic NonZero checking,
expected-result applications and anonymous refinement contracts are retained.

Fresh stage0 and stage1 builds against this seed pass all six smoke checks.
The real stage2 attempt exits 4 before body checking with 66 header diagnostics:
twelve imported/synthetic positions and fifty-four source positions, including
nineteen unchecked index calls. Stage2 produces no IR or compiler executable.
The count matches the preceding generic NonZero checkpoint; it includes cascades
and does not measure completion. The fresh staged snapshot has 566 unchanged
inputs, SHA256
`1719a992b3a2f2a535571436a10800a1790065e2076cda35a9eae7d1c9ced601`.
The compiler bundle is
`e707fad57653acb7c6a95fe3f9504e8aae68c5f528c7b39a7d07f632f9cf6fcd`.
Evidence and source-mapped diagnostics are in `/tmp/cinch-native-length-stages/`.

Cinch-side byte observations, closed private observation contracts and a fix
for unconstrained expected-result hints are in private validation. Type/Core,
seventeen source cases and ten imported observation cases pass across recorded
snapshots. The combined wire-contract and existing source regression gates
are running. These Cinch changes are not installed in this checkpoint.

Remaining source declarations, proof forms, an independent global typing
kernel and shared-runtime typed lowering are still required. Stage2/3 fixed-point
agreement, changed-source behavior and all tests through a self-built frontend
remain unproven. Work uses at most five concurrent jobs, one Cargo worker and
one test thread per job, and generated Cinch heaps limited to 2G.

[checkpoint-results.json](checkpoint-results.json) records the exact seed and stage status;
[integration-matrix-results.json](integration-matrix-results.json) records the completed validation scope.
Full logs and frozen snapshots remain local. Compiler work is pushed incrementally;
unrelated website, branding and reader-documentation changes are managed separately.
