# Compiler checkpoint — 1 November 2026

This branch contains the ongoing compiler/bootstrap work. Full self-hosting is
not achieved. Website and reader-documentation work is managed separately.

The pinned Rust seed, stage directories and verification commands are documented
in [README.md](README.md). The current implementation and remaining acceptance
criteria are described in [self-hosting.md](../design/self-hosting.md) and
[next-steps.md](../design/next-steps.md).

## Verified boundary

- Seven focused validation gates passed: Native interface units and contracts,
  compiled private-refinement consumers, imports, source/type/Core contracts,
  dictionary/effect checks, and pipeline/type-syntax/stage-ladder checks. Two
  Native gates reuse evidence after checking byte-identical producer inputs.
- Fresh stage0 and stage1 builds passed all six smoke checks.
- The actual stage2 attempt failed before body checking with 76 header
  diagnostics. It produced no stage2 IR or executable.
- A stage3 fixed point, changed-source behavior and the full compiler/runtime
  suite through a self-built frontend are still outstanding.

[checkpoint-results.json](checkpoint-results.json) records the gate outcomes,
separate validation/stage snapshot identities, seed hashes and stage2 failure
categories. Full logs and frozen source snapshots remain on the development
machine; this report is a checkpoint, not a claim that the full suite passes.

## Current work beyond this checkpoint

Anonymous refinement contracts and scoped argument/result witnesses are being
validated separately and are not installed in this checkpoint. Their current
candidate passes Native interface contracts, 17 compiled anonymous-contract
cases, eight private-refinement consumer cases and import regressions, but fails
this new source regression:

```cinch
keep :: (n :: refined (r > 0) Nat) -> refined (out == n) Nat
keep n = n
```

The failure says that the refinement predicate is not established for the value.
The candidate remains unmerged while that witness-scope failure is investigated.
A supplementary compiler/runtime/tooling run is partial and must not be read as
full-suite success. Test work uses a global limit of four concurrent jobs, with
one Cargo build worker and one test thread per job.

Push policy: publish incremental compiler fixes and meaningful checkpoints to
`codex/cinch-bootstrap`; label incomplete validation and known failures plainly.
