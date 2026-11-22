# Goal-aware tactic probe

The syntax library can now request proof search where the native checker knows
the expected proposition. It chooses a finite plan; the compiler executes that
plan against its actual goal and evidence snapshot. The Python front end does
not interpret nearby signatures, branch conditions or types.

```text
safe-index :: (i :: Nat) -> (n :: Nat) -> Option (Fin n)
safe-index i n =
  if i + 1 <= n then Some (Fin i (by!{ linear })) else None
```

Here `Fin i` requires evidence of `i < n`. The checker supplies that goal and
the branch fact `i + 1 <= n`; arithmetic search produces certificates whose
constraints and contradiction are independently checked before acceptance.

## Syntax and phases

- `by!{ assumption }` expands to `Proof.search "assumption"`.
- `by!{ linear }` expands to `Proof.search "linear"`.
- `by!{ assumption or linear }` expands to the ordered plan
  `Proof.search "assumption linear"`.
- Existing `reflexivity` and `exact { ... }` still emit ordinary proof terms.

`Proof.search` is an experimental native elaboration hook in this worktree.
It only works with an expected proposition. It cannot be inferred as a
first-class value or escape into runtime code. The argument must be a literal
plan, at most 128 bytes and 1–8 known steps. The whole plan is validated before
any step runs. Alternatives do not unify metavariables or change the evidence
state on failure.

`assumption` compares the reduced goal with propositions in the checker's
evidence snapshot, newest first. `linear` reuses native arithmetic search and
its certificate checker. This includes integer linear reasoning supported by
that existing solver; it does not make nonlinear goals decidable. These steps
return erased proof core only after checking. Search failure, false goals and
unsupported reasoning never become evidence.

Both strategies record precisely the evidence dependencies they use. The
existing post-unification invalidation check still rejects a proof about a
mutable location after a write. The snapshot excludes facts whose index names
have been rebound. Integer sorts and refinement facts come from the same
native context as ordinary obligation discharge.

## Evidence

`examples/Tactics.cinch` executes direct assumption lookup, a two-fact chain,
strictness conversion and ordered fallback, checking resulting `Fin` values.
Its rejection fixtures cover absent facts, a shadowed index and a stale proof
after mutation. `compiler/tests/goal_tactics.rs` also checks malformed plans,
the phase boundary, false goals, assumption failure and rejection of arithmetic
certificates with changed goals, missing facts or empty certificates.

The regular Tree-sitter tests parse all original tactic examples and the
updated library grammar. The native LSP bridge test checks that a failed tactic
reports `i < n` at the original `by!{ ... }` notation.

The example uses multiline `case` layout: the base Tree-sitter grammar
currently rejects inline semicolon-separated alternatives that the native
parser accepts. Broader parser/Tree-sitter grammar parity remains open.

The final aggregate run passed 39 experiment tests (13 native programs,
21 rejection fixtures, 14 syntax descriptors). Separately, all 43 native
regressions passed: dependent 14, evidence invalidation 12, goal tactics 9
and linear facts 8. `verification.json` records the aggregate output and source
hashes; the native regression command below reproduces the separate run.

Run the experiment with:

```sh
python3 experiments/malleable-syntax/poc.py run experiments/malleable-syntax/examples/Tactics.cinch
python3 experiments/malleable-syntax/verify.py
```

From `compiler/`, run the native regression suites with:

```sh
cargo test --quiet --test goal_tactics --test dependent --test linear_facts --test evidence_invalidation
```

## Remaining design work

This is a real goal-directed probe, but its two strategies are implemented by
the compiler. Libraries currently choose their plan and notation. They cannot
yet inspect a reflected goal, traverse local types, construct arbitrary proof
terms or return certificates through a public tactic API. That API, tactic
traces, interactive goal display, general proof search budgets and provenance
across native macro expansion remain design work. This pass does not claim a
general tactic system or complete the broader malleable-syntax experiment.
