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

Cinch checks temporal contracts across three surfaces: borrowed mutable-state functions, explicit-state `machine` declarations (with checked stuttering refinement via `refines`), and concurrency primitives (`withScope`, fair `MVar`, and `Effect.Stm`).

| Kind | Operator | Checked today |
| --- | --- | --- |
| `safety` | `always` | On `Mut` functions: inductive invariant at entry and across every write transition. On `machine` declarations: exhaustive bounded BFS over every reachable state, reporting a shortest counterexample trace on failure. |
| `safety` | `deadlock_free` | On `machine` declarations: verifies every reachable state has at least one enabled outgoing transition, or prints a shortest path to a deadlocked state. |
| `stability` | `stable` | On `Mut` functions: verifies footprint invariance across untouched transitions. On `machine` declarations: verifies that once `P` holds, no reachable transition makes `P` false. |
| `liveness` | `eventually` | On `withScope` functions: discharges finite structured-scope obligations (`joined`, `cleanup`). On `Effect.Concurrent.MVar` functions: discharges `fair_handoff` automatically from FIFO mutex and queue fairness (`Effect.Stm` requires an explicit `assume`). On `machine` declarations: checks absence of fair cycles avoiding `Q` via Tarjan SCC and Emerson-Lei fairness checking. |
| `liveness` | `leads_to` | On `machine` declarations (`(P) ~> (Q)`): verifies that from every reachable `P`-state, every fair infinite run reaches a `Q`-state, or prints a fair-cycle lasso counterexample. |
| `recurrence` | `recur` | On recursive functions: structural recursion at every cycle. On `machine` declarations: verifies `Q` holds infinitely often on every fair run. |

Non-structural and unbounded temporal claims outside these fragments receive a named check-time refusal. In particular, `liveness` cannot prove that external I/O or network calls eventually succeed; any external guarantee must be stated explicitly via `assume`.

# Explicit-state machines (`machine`)

A `machine` declaration defines a finite-state transition system directly in ordinary Cinch code, reusing the same records, enums, refinements, and functions your program executes:

```text
machine Name where
  state = StateType
  init = initialStateExpr
  actions = [act1, act2, ...]
  fair = [act1]                 -- optional: weak fairness (WF)
  strong_fair = [act2]          -- optional: strong fairness (SF)
  symmetry = [NodeType]         -- optional: type-checked symmetry reduction
  max_states = 4096             -- optional: BFS exploration cap (default 4096)
  observe = projectionFn        -- optional: observation function
```

## Action signatures and finite parameter domains

Each action listed in `actions` is a pure Cinch function whose **last** parameter is `StateType`, returning one of three shapes:

- `... -> StateType -> StateType` — unconditionally enabled deterministic step.
- `... -> StateType -> Option StateType` — guarded step (`Some next` = enabled, `None` = disabled).
- `... -> StateType -> List StateType` — non-deterministic step (`Nil` = disabled).

Any parameters before `StateType` are automatically enumerated by the model checker from their Cinch types:

| Parameter type | Enumerated domain |
| --- | --- |
| `Bool` | `[False, True]` |
| Nullary `data` enum | All declared constructors in declaration order |
| `Fin n` (static numeral `n`) | `0 .. n - 1` |
| `InRange lo hi Nat` (and refined integer ranges) | `lo .. hi` |

Unbounded parameter types (such as bare `Nat` or `String`) are rejected rather than silently truncated.

## Machine contracts, fairness, and stuttering invariance

A `contract` targeting a `machine` uses predicate lambdas `\s -> ... :: StateType -> Bool` (or `(True)` for `deadlock_free`):

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Pool where
    queued :: Nat
    running :: Nat
    done :: Nat

  accept :: Pool -> Option Pool
  accept s =
    if s.queued + s.running < 2
    then Some (Pool { queued = s.queued + 1, running = s.running, done = s.done })
    else None

  start :: Pool -> Option Pool
  start s =
    let q = s.queued in
    if q > 0 && s.running < 1
    then Some (Pool { queued = q - 1, running = s.running + 1, done = s.done })
    else None

  finish :: Pool -> Option Pool
  finish s =
    let r = s.running in
    if r > 0 && s.done < 3
    then Some (Pool { queued = s.queued, running = r - 1, done = s.done + 1 })
    else None

  machine WorkerPool where
    state = Pool
    init = Pool { queued = 0, running = 0, done = 0 }
    actions = [accept, start, finish]

  contract safety boundInv for WorkerPool where
    always = (\s -> s.queued + s.running <= 2 && s.running <= 1)

  main :: Int
  main = 0
```

- **Weak fairness (`fair`)** discards any cycle in which a listed action remains continuously enabled in every state of the cycle without firing.
- **Strong fairness (`strong_fair`)** discards any cycle in which a listed action is enabled in at least one state of the cycle without firing.
- Fairness clauses can be placed on the `machine` (applying to all contracts on it) or overridden/extended on an individual `contract`.
- **Stuttering invariance (`next` forbidden):** Public contracts reject a raw `next` operator (`\`next\` is forbidden in public temporal contracts; contracts must be stuttering-invariant`) so that properties proved on an abstract machine are preserved by every stuttering refinement:

<!-- cinch:reject contains="stuttering-invariant" -->
```cinch
module Main where
  record S where
    x :: Nat

  step :: S -> S
  step s = S { x = s.x }

  machine M where
    state = S
    init = S { x = 0 }
    actions = [step]

  contract safety badNext for M where
    next = (\s -> s.x == 0)

  main :: Int
  main = 0
```

## Symmetry reduction and Partial Order Reduction (POR)

- **Symmetry reduction (`symmetry = [T1, ...]`)** canonicalizes reachable states under permutations of each listed enum type, collapsing symmetric states during BFS. Soundness requires that no listed type has an `Ord` instance in scope; if `Ord T` is present, checking fails with `unsound symmetry reduction`.
- **Static field-footprint POR** analyses which record fields each action reads and writes. Pairs of actions with disjoint read/write footprints are identified automatically and reported in the manifest summary (`symmetry_merged`, `por_independent_pairs`).

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Node where
    N0 :: Node
    N1 :: Node

  record Sys where
    owner :: Option Node
    left :: Nat
    right :: Nat

  acquire :: Node -> Sys -> Option Sys
  acquire n s = case s.owner of
    None -> Some (Sys { owner = Some n, left = s.left, right = s.right })
    Some _ -> None

  release :: Sys -> Option Sys
  release s = case s.owner of
    Some _ -> Some (Sys { owner = None, left = s.left, right = s.right })
    None -> None

  bumpLeft :: InRange 0 1 Nat -> Sys -> Option Sys
  bumpLeft d s =
    if s.left == 0 && d == 1
    then Some (Sys { owner = s.owner, left = 1, right = s.right })
    else None

  bumpRight :: Sys -> Option Sys
  bumpRight s =
    if s.right == 0
    then Some (Sys { owner = s.owner, left = s.left, right = 1 })
    else None

  machine LockSys where
    state = Sys
    init = Sys { owner = None, left = 0, right = 0 }
    actions = [acquire, release, bumpLeft, bumpRight]
    symmetry = [Node]

  contract safety boundedCounts for LockSys where
    always = (\s -> s.left <= 1 && s.right <= 1)

  main :: Int
  main = 0
```

# Checked stuttering refinement (`refines`)

A `refines` declaration prevents specification-implementation drift by verifying a stuttering simulation from a concrete machine to an abstract machine:

```text
refines ConcreteMachine AbstractMachine where
  view = projectFn              -- ConcreteState -> AbstractState
  measure = rankFn              -- optional: ConcreteState -> Nat
```

The checker explores every reachable state and edge of `ConcreteMachine` and verifies:

1. `view concreteInit == abstractInit`.
2. For every concrete transition `c1 --[act]--> c2`, either `view c1 == view c2` (a stuttering step) or `view c1 -> view c2` is an existing transition in `AbstractMachine`'s reachable transition graph.
3. When `measure` is provided, every stuttering step (`view c1 == view c2`) must satisfy `measure c2 < measure c1`, ruling out infinite silent stuttering (livelock).

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record AbsState where
    done :: Bool

  absFinish :: AbsState -> Option AbsState
  absFinish s = if not s.done then Some (AbsState { done = True }) else None

  machine AbsSpec where
    state = AbsState
    init = AbsState { done = False }
    actions = [absFinish]

  record ConcState where
    phase :: Nat

  prepare :: ConcState -> Option ConcState
  prepare s = if s.phase == 0 then Some (ConcState { phase = 1 }) else None

  commit :: ConcState -> Option ConcState
  commit s = if s.phase == 1 then Some (ConcState { phase = 2 }) else None

  machine ConcImpl where
    state = ConcState
    init = ConcState { phase = 0 }
    actions = [prepare, commit]

  project :: ConcState -> AbsState
  project s = AbsState { done = s.phase == 2 }

  stutterRank :: ConcState -> Nat
  stutterRank s = if s.phase == 0 then 2 else (if s.phase == 1 then 1 else 0)

  refines ConcImpl AbsSpec where
    view = project
    measure = stutterRank

  main :: Int
  main = 0
```

Discharged refinements are recorded in `build/<entry>.manifest.json` under `temporal.contracts` with `"kind": "refinement"`, and any illegal transition or non-decreasing stuttering step fails check time with the exact offending action and state trace.

# Concurrency fairness contracts (`fair_handoff`)

`eventually = (fair_handoff)` connects liveness verification to the runtime's synchronization guarantees (see [Chapter 13](13-concurrency-and-runtime.md#fair-mvar-and-temporal-handoff-guarantees)):

- Functions performing the `MVar` effect (`Effect.Concurrent.MVar`) discharge `eventually = (fair_handoff)` automatically because `MVar` uses a strict-FIFO ticketed mutex and FIFO waiter queues.
- Functions performing `STM` (`Effect.Stm`) are refused unless an explicit `assume = "..."` documents the external fairness assumption, because optimistic transactions can retry indefinitely under contention.

<span class="support-tag support-tag--planned">Coming soon</span> **Unbounded inductive proofs over arbitrary state spaces and process trees.** Bounded explicit-state model checking, weak/strong fairness, symmetry reduction, POR, stuttering refinement, clocked `Later k`, and fair `MVar` discharge are implemented today; unbounded symbolic/inductive proofs beyond `max_states` and proofs over dynamically spawned concurrent process trees remain planned.

For guided walkthroughs, see [state evolution and temporal contracts](../learn/21-upgrades-and-contracts.md) and the beginner guide to [state machines, fairness, and model checking](../learn/23-state-machines-and-model-checking.md).
