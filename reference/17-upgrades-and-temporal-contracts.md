# Runtime upgrades and temporal contracts

Two subsystems live on this page. `upgrade` relates two state schemas and records what changed, and `contract` states a temporal property of a definition and asks the checker to discharge it over the definition's steps. Both record their evidence in the compiled interface and the artifact manifest, and both refuse a claim they cannot establish instead of trusting it.

# The upgrade declaration

```text
upgrade OldState to NewState where
  migrate = migration
  related = OldState
```

The block takes the two reserved component names `migrate` and `related`, each at most once. Any other name is refused: `` `x` is not an upgrade component; expected `migrate` or `related` ``.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record StateV1 where
    value :: Nat

  record StateV2 where
    base :: Nat
    delta :: Nat

  migrate :: StateV1 %1 -> StateV2
  migrate s = StateV2 { base = s.value, delta = 0 }

  upgrade StateV1 to StateV2 where
    migrate = migrate

  viewV2 :: StateV2 -> Nat
  viewV2 s = s.base + s.delta

  main :: Int
  main = if viewV2 (migrate (StateV1 { value = 5 })) == 5 then 0 else 1
```

`migrate` is an ordinary definition here, and running it proves only that this conversion computes what it says. It does not perform a rollout, activate an artifact, or migrate live state; those are runtime operations below.

# Migration obligations

The migration must have exactly the declared type, taking the old state linearly and returning the new one. A wrong signature is refused with both types printed:

<!-- cinch:reject contains="the migration must have type" -->
```cinch
module Main where
  record StateV1 where
    value :: Nat

  record StateV2 where
    base :: Nat

  bad :: StateV1 %1 -> StateV1
  bad s = s

  upgrade StateV1 to StateV2 where
    migrate = bad

  main :: Int
  main = 0
```

```
cinch: 11:3: the migration must have type `(Main.StateV1 %1 -> Main.StateV2)`, but it is declared `(Main.StateV1 %1 -> Main.StateV1)` (runtime evolution specification section 3)
```

A change the checker classifies as incompatible, such as retyping a field, needs a migration. A migration-free record view across such a change is refused rather than reinterpreted:

<!-- cinch:reject contains="incompatible field change" -->
```cinch
module Main where
  record StateV1 where
    value :: Nat

  record StateV2 where
    value :: Int

  upgrade StateV1 to StateV2 where
    related = StateV1

  main :: Int
  main = 0
```

```
cinch: 8:3: incompatible field change `value` in the evolution of StateV1 to StateV2 needs a migration (runtime evolution specification section 2)
```

A purely additive change is admitted without a migration, and the added field is recorded as added rather than collapsed into a default value. `related` names the relation the two schemas are claimed to satisfy; with a migration it points at the old type, and the migration is what establishes the relation.

# Evidence recorded at replacement

The compiled interface (`build/Main.cinchif`) carries the checked mapping that a replacement reads: which fields the migration produces, which fields changed incompatibly, and whether a migration was checked at all.

```json
{
  "added": [ { "name": "base", "type": "Nat" }, { "name": "delta", "type": "Nat" } ],
  "migrated": true,
  "new": "StateV2",
  "old": "StateV1",
  "retyped": []
}
```

A `#[repr]` change is also a representation change: it alters the recorded layout and the interface identity, so a dependent checked against the old identity is stale and must be replaced. A replacement must carry this evidence for the exact pair of artifacts, and a change that retypes a field without a recorded migration is not admissible.

# What the runtime does now

The runtime library exposes version, update-set, frame, variant, and deoptimization entry points as `cinch_evo_*` symbols. A program binds them as ordinary foreign imports and drives them; the effects are `{Global, IO}` and the entry point declares that row.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  foreign import ccall "cinch_evo_module_new" evoNew :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_version_new" evoVer :: I64 -> I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_current_version" evoCur :: I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_update_begin" evoBeg :: I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_add_replacement" evoAdd :: I64 -> I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_mark_complete" evoCmp :: I64 -> I64 -> I64 ->{Global, IO} I64
  foreign import ccall "cinch_evo_activate" evoAct :: I64 ->{Global, IO} I64

  main :: Unit ->{Global, IO} Int
  main u =
    let m = evoNew 0 in
    let v1 = evoVer m 0 0 in
    let b = evoBeg m v1 in
    let z1 = evoAdd b 0 v1 in
    let z2 = evoCmp b 1 0 in
    let z3 = evoAct b in
    let v2 = evoVer m 1 0 in
    let u2 = evoBeg m v2 in
    let z4 = evoAdd u2 1 v2 in
    let act = evoAct u2 in
    let cur = evoCur m in
    if act == -2 then (if cur == v1 then 0 else 1) else 2
```

This program builds the first version, activates it, starts a second update set, adds one replacement, and tries to activate an incomplete set. Activation returns `-2` and the current version is unchanged: an incomplete update set never becomes current. The same entry points express code retention while a frame is live (`cinch_evo_frame_enter`, `cinch_evo_version_retired`), refusal to resume old code over migrated state (`cinch_evo_frame_resume` returning `-4`), target compatibility checked before reachability (`cinch_evo_version_new` with an instruction requirement, `cinch_evo_activate` returning `-3`), variant install and retire, and a consumption-safe deoptimization that refuses an unbalanced linear transfer with `-7`. The `evolution` suite drives each of those as a compiled program.

The compatibility dimensions a replacement must satisfy are stated in the [runtime evolution specification](../specifications/runtime-evolution-specification.md) section 2: inputs, outputs, effects, state, resources, in-flight work, temporal properties, representation, and dependencies.

<span class="support-tag support-tag--planned">Coming soon</span> **an operator-facing update workflow.** There is no `cinch upgrade` subcommand, no service supervisor, no package activation command, and no rollback tooling. The subcommand does not exist, and the CLI answers with its usage text and exit status 2:

```
$ cinch upgrade Main.cinch
cinch: bootstrap compiler for the safe language design

usage:
  cinch check <file.cinch> [--rebuild]         typecheck only
  ...
$ echo $?
2
```

Driving the runtime entry points from a program you build yourself is the current workflow. Packaging an update, choosing a deployment policy, and coordinating a distributed rollout are specified but not implemented.

# Temporal contract declarations

```text
contract <kind> <name> for <definition> where
  <operator> = (<formula>)
  assume = (<formula>)
```

The kind and the operator are paired: `safety` with `always`, `stability` with `stable`, `liveness` with `eventually`, `recurrence` with `recur`. An unknown kind is refused with the four names. `assume` states what the claim rests on; a formula rather than a name is the admitted form, and a named external assumption stays an assumption.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  countdown :: forall region. Mut region Nat -> Int -> Nat
  countdown m n =
    if n == 0
    then copyOut m
    else
      let u = refSet m 3 in
      countdown m (n - 1)

  contract safety loopInv for countdown where
    always = (copyOut m < 5)
    assume = (copyOut m < 5)

  main :: Int
  main = 0
```

The checker walks the definition's transitions over the borrowed state and checks the invariant at each. `build/Main.manifest.json` records the discharged obligation under `temporal`, which is the manifest's only contract section:

```json
{
  "temporal": {
    "contracts": [
      {
        "assumptions": [
          "assume (copyOut m < 5)"
        ],
        "contract": "loopInv",
        "definition": "Main.countdown",
        "formula": "always (copyOut m < 5)",
        "kind": "safety",
        "result": "verified (the invariant holds at every step: 2 transition(s) of the state, 1 discharged by the inductive step)"
      }
    ]
  }
}
```

A write that falsifies the invariant names the step, the state, and the formula:

<!-- cinch:reject contains="is falsified by the step" -->
```cinch
module Main where
  countdown :: forall region. Mut region Nat -> Int -> Nat
  countdown m n =
    if n == 0
    then copyOut m
    else
      let u = refSet m 9 in
      countdown m (n - 1)

  contract safety loopInv for countdown where
    always = (copyOut m < 5)
    assume = (copyOut m < 5)

  main :: Int
  main = 0
```

```
cinch: 7:15: temporal safety: the contract `loopInv` on `Main.countdown` is falsified by the step at 7:15: the write leaves the state `9 < 5` false, so `always (copyOut m < 5)` fails at this transition (core specification section 10) (in Main.cinch)
```

# The checked fragment

| Kind | Operator | What is checked today |
| --- | --- | --- |
| `safety` | `always` | The invariant at the initial state and at every write step of the definition. A violating step is refused with the state; a step the checker cannot decide is refused by name, never assumed. The formula must read borrowed state. |
| `stability` | `stable` | A fact about one location survives every transition that does not touch its footprint. A write, including a call's potential write, to that footprint kills the fact and the error names evidence invalidation over mutable state. |
| `liveness` | `eventually` | Finite structured obligations within their scope: every spawned handle joined, cleanup run. An open-ended claim over arbitrary progress needs a fairness assumption and a well-founded progress argument that this fragment does not provide, so it is refused. |
| `recurrence` | `recur` | A state recurs at each cycle of a structural recursion, bounded by the decreasing argument. A ladder of literal equations has no structural bound and is refused. |

Nonstructural or unbounded temporal claims receive a named refusal instead of being accepted unchecked. Liveness does not prove that arbitrary network access eventually succeeds; an external outcome remains an external outcome.

<span class="support-tag support-tag--planned">Coming soon</span> **general unbounded temporal reasoning.** Simulation-based refinement, divergence-sensitive liveness, and proofs over arbitrary process trees are specified and unimplemented. Until their proof procedure exists, use the finite fragment and state external assumptions honestly as `assume`.

Implementation: `check_decls.rs` (upgrade components, contract clauses), `interface.rs` (upgrade and contract evidence), `analysis.rs` (transition walk); suites `evolution.rs`, `temporal.rs`, `evidence_invalidation.rs`; runtime `evolution.rs`. Intended semantics: [runtime evolution specification](../specifications/runtime-evolution-specification.md) sections 1 to 13 and [core language specification](../specifications/core-language-specification.md) section 10. Tutorial: [upgrades and contracts](../learn/21-upgrades-and-contracts.md).
