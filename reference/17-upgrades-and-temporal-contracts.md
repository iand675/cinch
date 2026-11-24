# Runtime upgrades and temporal contracts

This chapter covers two verification-backed mechanisms whose evidence is recorded in compiled interfaces (`.cinchif`) and build manifests (`.manifest.json`):

1. **Schema upgrades (`upgrade`)** relate an old and new state representation, check migration signatures and field compatibility, and pair with runtime version-activation primitives (`cinch_evo_*`).
2. **Temporal contracts (`contract`)** assert invariants across a definition's state transitions and discharge bounded safety, stability, liveness, and recurrence properties during checking.

Both mechanisms refuse claims they cannot establish rather than accepting them unchecked.

# Upgrade declarations

```text
upgrade OldState to NewState where
  migrate = migration
  related = OldState
```

An `upgrade` block accepts two reserved component bindings—`migrate` and `related`—each at most once. Any other binding name is rejected (`'x' is not an upgrade component; expected 'migrate' or 'related'`).

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

## Migration obligations and compatibility

Every `upgrade` declaration is checked against the structural diff between `OldState` and `NewState`:

- **Linear migration type.** When `migrate` is supplied, it must have exact type `OldState %1 -> NewState` so the old state value is consumed exactly once. Any other signature is rejected:

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

- **Incompatible changes require a migration.** Retyping or structurally altering a field cannot be bridged with `related` alone:

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

- **Additive changes.** Purely additive record evolutions are admitted without a migration, with added fields recorded explicitly rather than defaulted silently.
- **Representation changes.** Changing a type's `#[repr(...)]` changes its recorded ABI layout and interface identity, marking dependents built against the old identity stale until replaced.

## Recorded interface evidence

Checking writes the evolution summary into `build/<Module>.cinchif`, recording added fields, incompatibly retyped fields, and whether a checked migration was supplied:

```json
{
  "added": [ { "name": "base", "type": "Nat" }, { "name": "delta", "type": "Nat" } ],
  "migrated": true,
  "new": "StateV2",
  "old": "StateV1",
  "retyped": []
}
```

# Runtime version activation (`cinch_evo_*`)

The runtime library exports low-level module-evolution entry points (`cinch_evo_*`) for registering code versions, assembling atomic update sets, tracking active call frames, validating target capabilities, and checking linear-consumption safety during deoptimization. Programs bind these via `foreign import ccall` under `{Global, IO}`:

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

In this example, `v1` is marked complete and activated, whereas `u2` is incomplete: `cinch_evo_activate` returns `-2` and leaves `v1` active. Key runtime error codes across the `cinch_evo_*` surface include:

| Code | Meaning |
| --- | --- |
| `-2` | Incomplete update set rejected before activation. |
| `-3` | Target instruction/capability requirement failed compatibility check. |
| `-4` | Attempt to resume an old code frame (`cinch_evo_frame_resume`) over already-migrated state. |
| `-7` | Consumption-unsafe deoptimization (unbalanced linear transfer). |

Active frames retain their code version (`cinch_evo_frame_enter`, `cinch_evo_version_retired`) until all in-flight calls drain.

<span class="support-tag support-tag--planned">Coming soon</span> **Operator-facing upgrade CLI and rollout tooling.** There is no `cinch upgrade` subcommand, service supervisor, package activation command, or distributed rollout controller yet; driving `cinch_evo_*` directly from compiled code is the current workflow.

# Temporal contract declarations

A `contract` attaches a temporal property to a named definition:

```text
contract <kind> <name> for <definition> where
  <operator> = (<formula>)
  assume = (<formula>)
```

Each contract kind pairs with one temporal operator: `safety` with `always`, `stability` with `stable`, `liveness` with `eventually`, and `recurrence` with `recur`. An optional `assume` clause states preconditions the proof relies on.

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

The checker traverses the definition's state transitions over borrowed mutable state (`Mut`) and verifies the invariant at every step. Discharged contracts are recorded in `build/<entry>.manifest.json` under `temporal.contracts`:

```json
{
  "temporal": {
    "contracts": [
      {
        "assumptions": ["assume (copyOut m < 5)"],
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

Any write transition that violates the invariant is rejected with the failing source location, written state, and violated formula:

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

```text
cinch: 7:15: temporal safety: the contract `loopInv` on `Main.countdown` is falsified by the step at 7:15: the write leaves the state `9 < 5` false, so `always (copyOut m < 5)` fails at this transition (core specification section 10) (in Main.cinch)
```

# The checked temporal fragment

| Kind | Operator | Checked today |
| --- | --- | --- |
| `safety` | `always` | Verifies the invariant at entry and across every write transition on borrowed `Mut` state. Undecidable steps fail with an explicit refusal rather than being assumed. |
| `stability` | `stable` | Verifies that a fact about a location holds across all transitions outside its footprint; any direct or callee write to that footprint triggers evidence invalidation. |
| `liveness` | `eventually` | Verifies finite structured obligations within a scope (joining spawned task handles, completing scope cleanup). Open-ended progress claims requiring fairness are rejected. |
| `recurrence` | `recur` | Verifies recurrence along a structurally decreasing recursion argument. Non-structural or unbounded loops are rejected. |

Non-structural and unbounded temporal claims always receive a named check-time refusal. In particular, `liveness` cannot prove that external I/O or network calls eventually succeed; any external guarantee must be stated explicitly via `assume`.

<span class="support-tag support-tag--planned">Coming soon</span> **Unbounded temporal reasoning.** Simulation-based refinement, divergence-sensitive liveness, and proofs over arbitrary concurrent process trees are specified but not yet implemented.

See the [runtime evolution specification](../specifications/runtime-evolution-specification.md), [core language specification](../specifications/core-language-specification.md) §10, and the [upgrades and contracts tutorial](../learn/21-upgrades-and-contracts.md).
