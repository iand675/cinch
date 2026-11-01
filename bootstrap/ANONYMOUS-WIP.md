# Anonymous refinement contracts: work in progress

This branch preserves the candidate changes under active validation. Its parent
`codex/cinch-bootstrap` is the last installed checkpoint. This is a development
snapshot, not a passing bootstrap or a completed validation result.

The candidate serializes checked anonymous result contracts through interfaces,
retains their defining and argument identities through facades, and materializes
dependent argument/result witnesses in Core let scopes. Result weakening retains
effects and cannot use a producer implementation as a stronger public contract.

The latest fix derives a fresh temporary's equality to its actual right-hand side
through the existing checked Core let rule. It addresses this rejected valid case:

```cinch
keep :: (n :: refined (r > 0) Nat) -> refined (out == n) Nat
keep n = n
```

A focused compiled-Cinch checker passes all 63 unchanged source-refinement cases,
including rejection of proofs about a different argument, false strengthening,
zero refined arguments, escaped branch assumptions and erased effects. The
checker reads test source at runtime and checks retained Core evidence.

[anonymous-wip-results.json](anonymous-wip-results.json) records the actual cases
and candidate file hashes. Fresh broader validation covers 20 target jobs across
six gates, with audited reuse of byte-identical Native producer tests. Those
checks are still pending. Work shares a global limit of four concurrent jobs.

The pinned seed stays at the last verified checkpoint until the broader candidate
checks pass. Its source hash therefore does not match the Native candidate yet;
this WIP must not be described as a verified stage0/stage1/stage2 chain. Integration,
seed refresh, fresh staged acceptance, the remaining language implementation and
full-suite verification remain required.
