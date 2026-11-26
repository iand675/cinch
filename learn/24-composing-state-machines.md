# 24. Composing state machines and debugging models

In [chapter 23](23-state-machines-and-model-checking.md), you checked single-component state machines. Real systems have multiple actors, interacting subsystems, layered abstractions, and bugs that take more than one glance to diagnose.

In this chapter you will learn how to compose larger models cleanly without blowing up the state space, and how to inspect and debug them with Cinch's CLI (`cinch model`), REPL (`:machines`, `:explore`, `:trace`, `:dot`), and LSP hover/go-to-definition:

1. **Multi-actor actions** using multi-parameter functions, `Option`, and tuples.
2. **Product-record composition** with automatic static Partial Order Reduction (POR).
3. **Assume-guarantee composition** using `assume = (\s -> ...)` to verify components under explicit environment invariants and then discharge those assumptions on the composed system.
4. **Refinement towers** (`Concrete -> Mid -> Abstract`) and **horizontal multi-view refinement** (one composed system simultaneously refining multiple independent abstract specs, including across modules).
5. **Debugging models** with per-step field-diff counterexamples, `cinch model`, the REPL, and the LSP.

# Multi-actor actions with multiple parameters, `Option`, and tuples

A real protocol action often depends on more than one input—a sender and a receiver, an optional target, or a pair of flags. Any number of finite parameters may appear before the final `State` argument of an action, including `Bool`, `Unit`, nullary `data` enums, `Fin n`, `InRange lo hi Nat`, `Option T`, and tuples `(A, B)`.

Here is a two-account transfer ledger where `transfer` takes `(Node, Node)` (source and destination) plus an `InRange 1 1 Nat` amount:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Account where
    Alice :: Account
    Bob :: Account

  record Ledger where
    alice :: Nat
    bob :: Nat

  transfer :: (Account, Account) -> InRange 1 1 Nat -> Ledger -> Option Ledger
  transfer pair amt s = case pair of
    (Alice, Bob) ->
      let a = s.alice in
      if a >= amt
      then Some (Ledger { alice = a - amt, bob = s.bob + amt })
      else None
    (Bob, Alice) ->
      let b = s.bob in
      if b >= amt
      then Some (Ledger { alice = s.alice + amt, bob = b - amt })
      else None
    _ -> None

  machine Bank where
    state = Ledger
    init = Ledger { alice = 2, bob = 1 }
    actions = [transfer]

  contract safety conservation for Bank where
    always = (\s -> s.alice + s.bob == 3)

  contract safety noDeadlock for Bank where
    deadlock_free = (True)

  main :: Int
  main = 0
```

Cinch automatically enumerates `(Alice, Alice)`, `(Alice, Bob)`, `(Bob, Alice)`, `(Bob, Bob)` paired with `amt = 1`, skips the self-transfer cases that return `None`, and verifies that total money `alice + bob == 3` is conserved in every reachable state.

For payload-carrying ADTs or records (such as `data Msg = Req Client Nat | Ack Client`), implement `Data.Finite.Finite` (`enumerate :: Unit -> List a`) and the model checker automatically evaluates `enumerate ()` at check time. See [Reference Chapter 30](../reference/30-tla-plus-guide-and-value-domains.md#extensible-domains-with-datafinite) for a full example and the table of enumerable and rejected parameter types.

## Native-speed randomized simulation with `Effect.Test.Hegel`

Compile-time `machine` checking excels at **exhaustive small-scope verification** (typically hundreds to a few thousand states, with type-checked `symmetry`, static POR, `refines`, and manifest provenance). When a model has unbounded inputs or millions of reachable states, reuse the **exact same pure transition functions** at native LLVM speed with the standard-library choice-stream simulator `Effect.Test.Hegel`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Finite
  import Data.List
  import Data.Option
  import Data.Result
  import Effect.Test.Hegel

  record Vault where
    balance :: Nat

  deposit :: Nat -> Vault -> Option Vault
  deposit amt s =
    if s.balance + amt <= 10
    then Some (Vault { balance = s.balance + amt })
    else None

  withdraw :: Vault -> Option Vault
  withdraw s =
    let b = s.balance in
    if b > 0 then Some (Vault { balance = b - 1 }) else None

  main :: Int
  main =
    let actions = [
          step-action-gen "deposit" (gen-range 1 5) deposit,
          step-action "withdraw" withdraw
        ] in
    -- Passes all 200 simulated traces when invariant holds:
    case hegel-check-machine (Vault { balance = 0 }) actions (\s -> s.balance <= 10) of
      Err _ -> 1
      Ok _ ->
        -- When an invariant fails, Hegel automatically shrinks both trace length and arguments:
        case hegel-check-machine (Vault { balance = 0 }) actions (\s -> s.balance < 5) of
          Ok _ -> 2
          Err fail ->
            let fin = machine-failure-final-state fail in
            if fin.balance >= 5 then 0 else 3
```

Because `Effect.Test.Hegel` records every `Gen` draw as a finite sequence of discrete choices (`List Nat`), shrinking deletes steps and binary-searches draws toward `0` automatically—giving you minimal counterexample traces and minimal action arguments without writing separate `shrink` instances.

# Product-record composition and automatic POR

The simplest way to compose two subsystems is a **product record** holding each subsystem's state as its own field, paired with lifted actions that update only the fields they own.

When actions operate on disjoint fields of the state record, Cinch's static field-footprint analyzer detects automatically that those actions commute (**Partial Order Reduction**). You do not have to annotate footprints manually:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record AppState where
    loggedIn :: Bool
    queuedJobs :: Nat
    doneJobs :: Nat

  signIn :: AppState -> Option AppState
  signIn s =
    if not s.loggedIn
    then Some (AppState { loggedIn = True, queuedJobs = s.queuedJobs, doneJobs = s.doneJobs })
    else None

  signOut :: AppState -> Option AppState
  signOut s =
    if s.loggedIn
    then Some (AppState { loggedIn = False, queuedJobs = s.queuedJobs, doneJobs = s.doneJobs })
    else None

  enqueue :: AppState -> Option AppState
  enqueue s =
    if s.queuedJobs < 2 && s.doneJobs < 2
    then Some (AppState { loggedIn = s.loggedIn, queuedJobs = s.queuedJobs + 1, doneJobs = s.doneJobs })
    else None

  processJob :: AppState -> Option AppState
  processJob s =
    let q = s.queuedJobs in
    if q > 0 && s.doneJobs < 2
    then Some (AppState { loggedIn = s.loggedIn, queuedJobs = q - 1, doneJobs = s.doneJobs + 1 })
    else None

  machine App where
    state = AppState
    init = AppState { loggedIn = False, queuedJobs = 0, doneJobs = 0 }
    actions = [signIn, signOut, enqueue, processJob]

  contract safety appBounds for App where
    always = (\s -> s.queuedJobs <= 2 && s.doneJobs <= 2)

  main :: Int
  main = 0
```

Because `signIn` and `signOut` read and write only `loggedIn`, while `enqueue` and `processJob` read and write only `queuedJobs` and `doneJobs`, the analyzer discovers all four independent action pairs (`signIn ∥ enqueue`, `signIn ∥ processJob`, `signOut ∥ enqueue`, `signOut ∥ processJob`) automatically.

# Assume-guarantee composition (`assume = (\s -> ...)`)

Product composition works well while the combined state space stays small. For larger systems, you want to verify **component A** under a stated assumption about its environment, and then separately verify that **component B** (or the composed system) actually guarantees that assumption.

On any `machine` contract, you can write an `assume = (\s -> EnvPred s)` clause:

- The checker evaluates `EnvPred` across reachable states and restricts checking to states reachable from `init` **while `EnvPred` holds continuously** (and restricts fair-cycle checks to cycles within that environment-valid region).
- Every assumption is recorded verbatim in `build/<entry>.manifest.json` and printed by `cinch model inspect`, so assumptions are never hidden.

Consider a reservation worker whose `commit` action assumes its environment never drops `budget` below `reserved`. Without `assume`, an unconstrained environment action `slashBudget` violates the component's safety contract:

<!-- cinch:reject contains="temporal safety violation" -->
```cinch
module Main where
  record BudgetState where
    budget :: Nat
    reserved :: Nat
    committed :: Nat

  reserve :: BudgetState -> Option BudgetState
  reserve s =
    if s.reserved < s.budget && s.reserved < 2
    then Some (BudgetState { budget = s.budget, reserved = s.reserved + 1, committed = s.committed })
    else None

  commit :: BudgetState -> Option BudgetState
  commit s =
    let r = s.reserved in
    if r > 0 && s.committed < 2
    then Some (BudgetState { budget = s.budget, reserved = r - 1, committed = s.committed + 1 })
    else None

  -- Unconstrained environment action that can shrink budget arbitrarily.
  slashBudget :: BudgetState -> Option BudgetState
  slashBudget s =
    let b = s.budget in
    if b > 0
    then Some (BudgetState { budget = b - 1, reserved = s.reserved, committed = s.committed })
    else None

  machine OpenWorker where
    state = BudgetState
    init = BudgetState { budget = 2, reserved = 0, committed = 0 }
    actions = [reserve, commit, slashBudget]

  contract safety neverOverdrawn for OpenWorker where
    always = (\s -> s.reserved <= s.budget && s.committed <= 2)

  main :: Int
  main = 0
```

Now state the environment contract explicitly with `assume = (\s -> s.reserved <= s.budget)` on `OpenWorker`, and then discharge that exact guarantee **without any assumption** on `GuardedSystem`, where `safeSlashBudget` refuses to shrink the budget below `reserved`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record BudgetState where
    budget :: Nat
    reserved :: Nat
    committed :: Nat

  reserve :: BudgetState -> Option BudgetState
  reserve s =
    if s.reserved < s.budget && s.reserved < 2
    then Some (BudgetState { budget = s.budget, reserved = s.reserved + 1, committed = s.committed })
    else None

  commit :: BudgetState -> Option BudgetState
  commit s =
    let r = s.reserved in
    if r > 0 && s.committed < 2
    then Some (BudgetState { budget = s.budget, reserved = r - 1, committed = s.committed + 1 })
    else None

  slashBudget :: BudgetState -> Option BudgetState
  slashBudget s =
    let b = s.budget in
    if b > 0
    then Some (BudgetState { budget = b - 1, reserved = s.reserved, committed = s.committed })
    else None

  safeSlashBudget :: BudgetState -> Option BudgetState
  safeSlashBudget s =
    let b = s.budget in
    let r = s.reserved in
    if b > r
    then Some (BudgetState { budget = b - 1, reserved = r, committed = s.committed })
    else None

  -- Component checked under an explicit environment assumption:
  machine OpenWorker where
    state = BudgetState
    init = BudgetState { budget = 2, reserved = 0, committed = 0 }
    actions = [reserve, commit, slashBudget]

  contract safety workerUnderEnv for OpenWorker where
    always = (\s -> s.committed <= 2 && s.reserved <= s.budget)
    assume = (\s -> s.reserved <= s.budget)

  -- Composed system discharging the assumption with no `assume` clause:
  machine GuardedSystem where
    state = BudgetState
    init = BudgetState { budget = 2, reserved = 0, committed = 0 }
    actions = [reserve, commit, safeSlashBudget]

  contract safety systemGuaranteesEnv for GuardedSystem where
    always = (\s -> s.reserved <= s.budget && s.committed <= 2)

  main :: Int
  main = 0
```

Use assume-guarantee checking whenever a subsystem relies on an external precondition (such as "no duplicate message IDs" or "leader lease has not expired"): write the assumption right on the component contract, then prove the matching `always` contract on the caller or composed system.

# Horizontal multi-view refinement and refinement towers

Two architectural patterns let you connect big implementations to small, readable specifications:

1. **Horizontal multi-view refinement:** One composed machine `FullSystem` refines **multiple independent abstract specs** simultaneously—for instance, `refines FullSystem AuthSpec` via `projectAuth` AND `refines FullSystem QueueSpec` via `projectQueue`. Each spec stays tiny and focused on one concern, while the composed system is proved faithful to every view at once.
2. **Vertical refinement towers:** Stack refinements layer by layer (`LowImpl` refines `MidProtocol`, and `MidProtocol` refines `HighSpec`). Because stuttering refinement is transitive, every stuttering-invariant contract verified on `HighSpec` holds on `LowImpl`.

Here is one composed `FullSystem` simultaneously refining two independent abstract specifications (`AuthSpec` and `QueueSpec`):

<!-- cinch:run exit=0 -->
```cinch
module Main where
  -- Abstract view 1: authentication lifecycle
  record AuthView where
    authed :: Bool

  authLogin :: AuthView -> Option AuthView
  authLogin s = if not s.authed then Some (AuthView { authed = True }) else None

  authLogout :: AuthView -> Option AuthView
  authLogout s = if s.authed then Some (AuthView { authed = False }) else None

  machine AuthSpec where
    state = AuthView
    init = AuthView { authed = False }
    actions = [authLogin, authLogout]

  -- Abstract view 2: single-slot job queue
  record QueueView where
    busy :: Bool

  qSubmit :: QueueView -> Option QueueView
  qSubmit s = if not s.busy then Some (QueueView { busy = True }) else None

  qFinish :: QueueView -> Option QueueView
  qFinish s = if s.busy then Some (QueueView { busy = False }) else None

  machine QueueSpec where
    state = QueueView
    init = QueueView { busy = False }
    actions = [qSubmit, qFinish]

  -- Composed concrete system combining both subsystems
  record FullState where
    authed :: Bool
    busy :: Bool
    auditCount :: Nat

  sysLogin :: FullState -> Option FullState
  sysLogin s =
    if not s.authed && s.auditCount < 2
    then Some (FullState { authed = True, busy = s.busy, auditCount = s.auditCount + 1 })
    else None

  sysLogout :: FullState -> Option FullState
  sysLogout s =
    if s.authed && not s.busy
    then Some (FullState { authed = False, busy = False, auditCount = s.auditCount })
    else None

  sysSubmit :: FullState -> Option FullState
  sysSubmit s =
    if s.authed && not s.busy
    then Some (FullState { authed = True, busy = True, auditCount = s.auditCount })
    else None

  sysFinish :: FullState -> Option FullState
  sysFinish s =
    if s.busy
    then Some (FullState { authed = s.authed, busy = False, auditCount = s.auditCount })
    else None

  machine FullSystem where
    state = FullState
    init = FullState { authed = False, busy = False, auditCount = 0 }
    actions = [sysLogin, sysLogout, sysSubmit, sysFinish]

  viewAuth :: FullState -> AuthView
  viewAuth s = AuthView { authed = s.authed }

  viewQueue :: FullState -> QueueView
  viewQueue s = QueueView { busy = s.busy }

  refines FullSystem AuthSpec where
    view = viewAuth

  refines FullSystem QueueSpec where
    view = viewQueue

  main :: Int
  main = 0
```

Notice how `sysSubmit` and `sysFinish` stutter with respect to `AuthSpec`, while `sysLogin` and `sysLogout` stutter with respect to `QueueSpec`. Neither spec needs to know the other exists.

## Cross-module specifications

Abstract specs can also live in a dedicated specification module (`Spec.cinch`) and be imported by an implementation module (`Main.cinch`):

<!-- cinch:project group=24-cross-module-refines file=Spec.cinch -->
```cinch
module Spec where
  record DoorView where
    open :: Bool

  specOpen :: DoorView -> Option DoorView
  specOpen s = if not s.open then Some (DoorView { open = True }) else None

  specClose :: DoorView -> Option DoorView
  specClose s = if s.open then Some (DoorView { open = False }) else None

  machine DoorSpec where
    state = DoorView
    init = DoorView { open = False }
    actions = [specOpen, specClose]
```

<!-- cinch:project group=24-cross-module-refines file=Main.cinch exit=0 -->
```cinch
module Main where
  import Spec

  record LatchState where
    unlatched :: Bool
    open :: Bool

  unlatch :: LatchState -> Option LatchState
  unlatch s =
    if not s.unlatched && not s.open
    then Some (LatchState { unlatched = True, open = False })
    else None

  openDoor :: LatchState -> Option LatchState
  openDoor s =
    if s.unlatched && not s.open
    then Some (LatchState { unlatched = True, open = True })
    else None

  closeDoor :: LatchState -> Option LatchState
  closeDoor s =
    if s.open
    then Some (LatchState { unlatched = False, open = False })
    else None

  machine LatchImpl where
    state = LatchState
    init = LatchState { unlatched = False, open = False }
    actions = [unlatch, openDoor, closeDoor]

  projectDoor :: LatchState -> DoorView
  projectDoor s = DoorView { open = s.open }

  refines LatchImpl DoorSpec where
    view = projectDoor

  main :: Int
  main = 0
```

# Debugging models with field diffs, CLI, REPL, and LSP

When a contract fails—or when you want to see why your state count grew—Cinch gives you four debugging surfaces over the exact same model checker.

## 1. Per-step field-diff counterexamples

Every counterexample trace annotates each transition with `(changed: ...)` listing only the record fields that actually changed on that step, so wide state records stay scannable:

```text
  State 0 (init): BudgetState { budget = 2, reserved = 0, committed = 0 }
    --[reserve]--> (changed: reserved: 0 -> 1)
  State 1: BudgetState { budget = 2, reserved = 1, committed = 0 }
    --[slashBudget]--> (changed: budget: 2 -> 1)
  State 2: BudgetState { budget = 1, reserved = 1, committed = 0 }
    --[slashBudget]--> (changed: budget: 1 -> 0)
  State 3: BudgetState { budget = 0, reserved = 1, committed = 0 }
```

## 2. CLI inspection: `cinch model`

Use `cinch model` to inspect state counts, action parameter domains, symmetry/POR reductions, deadlocks, shortest traces to arbitrary predicates, and Graphviz state graphs—**even when a contract is currently failing**:

```sh
# Summarize every machine (or one named machine) and all contract/refinement verdicts
cinch model inspect App.cinch
cinch model inspect App.cinch App
cinch model inspect App.cinch --json

# Find a shortest path to a specific state, a deadlock, or a predicate
cinch model trace App.cinch App --state 4
cinch model trace App.cinch App --deadlock
cinch model trace App.cinch App --where '\s -> s.doneJobs == 2'

# Export the explored transition graph to Graphviz DOT
cinch model dot App.cinch App -o app.dot
dot -Tsvg app.dot -o app.svg
```

`cinch model inspect` prints a compact report:

```text
machine Main.App (state :: Main.AppState)
  explored: 16 state(s), 32 transition(s), max depth 6, 0 deadlock(s)
  POR independent pairs (4): signIn || enqueue, signIn || processJob, signOut || enqueue, signOut || processJob
  init: AppState { loggedIn = False, queuedJobs = 0, doneJobs = 0 }
  actions:
    - signIn (Option State)
    - signOut (Option State)
    - enqueue (Option State)
    - processJob (Option State)
  contracts:
    - [PASS] safety appBounds (always (\s -> s.queuedJobs <= 2 && s.doneJobs <= 2))
```

`cinch model inspect` exits `0` when all contracts and refinements on the selected machines pass, and `1` when any fail (after printing the full summary and failing diagnostic details).

## 3. Interactive exploration in `cinch repl`

Load a project into `cinch repl` and step through the explored graph interactively:

```text
> :load App.cinch
loaded App.cinch
rechecked Main
> :machines
Main.App (16 states, 32 edges, 0 deadlocks, 1/1 contracts ok)
> :machine App
machine Main.App (state :: Main.AppState)
  ...
> :explore App 0
State #0 (init): AppState { loggedIn = False, queuedJobs = 0, doneJobs = 0 }
  outgoing transitions (2):
    --[signIn]--> #1: AppState { loggedIn = True, queuedJobs = 0, doneJobs = 0 }  (changed: loggedIn: False -> True)
    --[enqueue]--> #2: AppState { loggedIn = False, queuedJobs = 1, doneJobs = 0 }  (changed: queuedJobs: 0 -> 1)
> :explore App 2
State #2: AppState { loggedIn = False, queuedJobs = 1, doneJobs = 0 }
  ...
> :trace App 5
Shortest trace in `App` to state #5:
  State 0 (init): ...
> :dot App
digraph "App" { ... }
```

Use `:trace <Machine> deadlock` to jump straight to the first reachable deadlocked state if one exists.

## 4. Editor integration in `cinch lsp`

When `cinch lsp` is attached to your editor:

- **Hover over a `machine` name** to see its live state/transition count, deadlock count, symmetry/POR reductions, action domain sizes, and every contract/refinement verdict (`PASS` or `FAIL`) in rich Markdown—even while a contract is failing.
- **Hover over a `contract` name** to see its formula, active `assume` clauses, and current verdict (`PASS` or full counterexample trace on `FAIL`).
- **Go-to-definition** works on action names in `actions`/`fair`/`strong_fair`, symmetry types, `refines` machine targets, `view`/`measure` functions, and contract targets.

# What the compiler enforces

- Multi-parameter actions enumerate Cartesian products of finite parameter domains (`Bool`, `Unit`, enums, `Fin n`, `InRange`, `Option T`, tuples) up to 1,024 tuples per state, and reject unbounded parameter types.
- `assume = (\s -> Env s)` restricts checking of `always`, `deadlock_free`, `eventually`, `leads_to`, `recur`, and `stable` to states reachable while `Env` holds, and records every assumption in manifests and reports.
- Multiple `refines` blocks on the same concrete machine verify independent abstract views, including across module boundaries.
- Static field-footprint POR reports commuting action pairs without requiring manual annotations.
- Counterexample traces annotate each step with per-field `(changed: field: old -> new)` diffs.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| An action parameter with an unbounded type (e.g. `Nat`, `String`, `List a`) | `has no finite enumerable domain` |
| A state record holding a closure, thunk, `codata`, or `MVar` | `is not inspectable/hashable data` |
| An effectful action inside `actions = [...]` | `must be pure for model checking` |
| A component safety contract violated without an environment guard | `temporal safety violation` with `(changed: ...)` field-diff trace |
| `refines FullSystem SubSpec` where an action makes a disallowed move in `SubSpec` | `refinement violation` naming the offending action and projected jump |

# Exercises

**Exercise 1.** In the `OpenWorker` / `GuardedSystem` example above, why is checking `workerUnderEnv` with `assume = (\s -> s.reserved <= s.budget)` not enough by itself to claim the whole system is safe?

<details><summary>Answer</summary>

`assume` restricts the checker to states reachable while the environment keeps `s.reserved <= s.budget` true. If the real system still allows `slashBudget` to violate that invariant, the assumption is false at runtime. You close the loop by proving `always = (\s -> s.reserved <= s.budget)` **without** `assume` on the composed system (`GuardedSystem`) where `safeSlashBudget` enforces the guard.

</details>

**Exercise 2.** Suppose `FullSystem` has three subsystems (`Auth`, `Queue`, `Billing`). Why is writing three separate `refines FullSystem ...` declarations often better than writing one giant abstract spec?

<details><summary>Answer</summary>

Each abstract spec stays small, readable, and reusable across modules, and you can verify safety/liveness properties on each spec independently without multiplying unrelated state dimensions together. Every concrete step in `FullSystem` simply stutters on the views it does not affect.

</details>

---

**Next:** For a direct translation from TLA+ and the complete reference on checkable value domains, see [Guide for TLA+ users and checkable value domains](../reference/30-tla-plus-guide-and-value-domains.md).
