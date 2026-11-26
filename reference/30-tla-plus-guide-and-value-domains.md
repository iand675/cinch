# Guide for TLA+ users and checkable value domains

This chapter has two parts:

1. **A guide for TLA+ users** comparing Cinch's `machine`, `contract`, and `refines` declarations against TLA+, TLC, TLAPS, and Apalache—showing what maps one-to-one, what Cinch's type and runtime systems handle differently, and what TLA+ still does that Cinch does not.
2. **A complete reference on checkable value domains** detailing exactly which state types, action parameter types, return shapes, action bodies, and reductions the compile-time model checker can explore and which it rejects.

For a beginner introduction to state machines in Cinch, see [Learn Chapter 23](../learn/23-state-machines-and-model-checking.md). For composing larger models and using the CLI/REPL/LSP debugging tools, see [Learn Chapter 24](../learn/24-composing-state-machines.md). For syntax and manifest schemas, see [Chapter 17](17-upgrades-and-temporal-contracts.md).

# Part I: Cinch for TLA+ users

## Rosetta Stone: TLA+ to Cinch

| TLA+ / TLC | Cinch | Notes |
| --- | --- | --- |
| `VARIABLES x, y` | `record State where { x :: ...; y :: ... }` | Ordinary typed Cinch record used by both the model and running code. |
| `TypeOK` invariant | The `State` record type + refinements (`Fin n`, `InRange lo hi Nat`) | Enforced statically by the type checker; no need to re-assert basic types as an invariant. |
| `Init == ...` | `init = State { ... }` | Single initial state expression evaluated at check time. |
| `Next == A \/ B` | `actions = [a, b]` | Each action is a pure Cinch function returning `State`, `Option State`, or `List State`. |
| Guard `x > 0 /\ x' = x - 1` | `if s.x > 0 then Some (State { x = s.x - 1, ... }) else None` | `None` (or `Nil` for `List State`) means disabled. |
| `\E n \in Nodes : Act(n)` | `act :: Node -> State -> Option State` | Prefix parameters before `State` are auto-enumerated from their finite Cinch types. |
| `\E a \in Nodes, b \in Nodes : Transfer(a, b)` | `transfer :: Node -> Node -> State -> Option State` (or `(Node, Node)`) | Multi-parameter actions enumerate the Cartesian product (up to 1,024 tuples per state). |
| Non-deterministic choice `\E v \in {1, 2} : ...` | Return `List State` (`[s1, s2]`) or take `InRange 1 2 Nat` | Returning `List State` models arbitrary finite branching from one action. |
| `INVARIANT P` (`[]P`) | `contract safety name for M where always = (\s -> P s)` | Checked via exhaustive BFS; fails with a shortest path and per-step field diffs. |
| Deadlock checking (TLC default) | `contract safety name for M where deadlock_free = (True)` | Explicit contract; fails with a shortest trace to a state with zero enabled actions. |
| `<>Q` | `contract liveness name for M where eventually = (\s -> Q s)` | Checked via Tarjan SCC + Emerson-Lei fair-cycle search. |
| `P ~> Q` | `contract liveness name for M where leads_to = (\s -> P s) ~> (\s -> Q s)` | Reports a shortest prefix to a `P`-state plus a fair cycle avoiding `Q`. |
| `[]<>Q` | `contract recurrence name for M where recur = (\s -> Q s)` | Checks no fair cycle permanently avoids `Q`. |
| Stability (`[](P => []P)`) | `contract stability name for M where stable = (\s -> P s)` | Checks no reachable transition turns `P` from `True` to `False`. |
| `WF_vars(Act)` | `fair = [act]` | On the `machine` (applies to all contracts) or overridden per `contract`. |
| `SF_vars(Act)` | `strong_fair = [act]` | Discards cycles where `act` is enabled in at least one state of the cycle without firing. |
| `ASSUME` / state constraint | `assume = (\s -> Env s)` on a machine `contract` | Restricts checked reachable states and fair cycles to states reachable while `Env` holds; recorded in manifests. |
| `SYMMETRY Permutations(Nodes)` | `symmetry = [Node]` | Type-checked: rejected if `Ord Node` is in scope. |
| `Refinement` (`INSTANCE Spec WITH ...`) | `refines Concrete Abstract where view = proj` | Checks `view cInit == aInit` and stuttering simulation on every concrete edge. |
| Stuttering-livelock ranking function | `measure = rankFn` inside `refines` | Checks `rank c2 < rank c1` on every stuttering step (`view c1 == view c2`). |
| TLC `-maxSetSize` / depth limit | `max_states = 4096` (default `4096`) | Marks exploration as `capped` when exceeded rather than pretending coverage is exhaustive. |
| TLC state-graph dump (`-dump dot`) | `cinch model dot File.cinch Machine -o out.dot` | Also available interactively via `:dot Machine` in `cinch repl`. |

### Side-by-side example: mutual exclusion with symmetry

In TLA+, a two-node lock model declares a constant set `Nodes`, an untyped `owner` variable, a `TypeOK` invariant, a `SYMMETRY` set in the `.cfg` file, and a `Spec` with `WF_vars(Release)`.

In Cinch, the exact same model is ordinary executable code—the `Node` enum, `LockState` record, `acquire`/`release` functions, `machine`, and `contract` live together in one module:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Node where
    N0 :: Node
    N1 :: Node

  record LockState where
    owner :: Option Node
    completed :: Nat

  acquire :: Node -> LockState -> Option LockState
  acquire n s = case s.owner of
    None ->
      if s.completed < 2
      then Some (LockState { owner = Some n, completed = s.completed })
      else None
    Some _ -> None

  release :: LockState -> Option LockState
  release s = case s.owner of
    Some _ -> Some (LockState { owner = None, completed = s.completed + 1 })
    None -> None

  machine Mutex where
    state = LockState
    init = LockState { owner = None, completed = 0 }
    actions = [acquire, release]
    fair = [release]
    symmetry = [Node]

  contract safety boundedDone for Mutex where
    always = (\s -> s.completed <= 2)

  contract liveness alwaysReleased for Mutex where
    leads_to = (\s -> case s.owner of { Some _ -> True; None -> False })
      ~> (\s -> case s.owner of { None -> True; Some _ -> False })

  main :: Int
  main = 0
```

## What is the same as TLA+ / TLC

- **Explicit-state BFS over finite graphs.** Like TLC, Cinch explores reachable states breadth-first from `init`, so every `always` and `deadlock_free` counterexample is guaranteed to be a **shortest** path from `init`.
- **Tarjan SCC + Emerson-Lei liveness checking.** Liveness (`eventually`, `leads_to`, `recur`) detects accepting cycles under weak fairness (`fair`) and strong fairness (`strong_fair`) and produces a concrete **lasso trace** (prefix from `init` into a repeating fair cycle).
- **Stuttering invariance by construction.** TLA+ forbids a bare prime-only temporal next operator so refinements can add internal steps. Cinch enforces the same rule: writing `next` in a public `contract` is a check-time error (`stuttering-invariant`).
- **Stuttering refinement (`refines`).** Just like proving `Concrete => Spec` under a refinement mapping in TLA+, `refines Concrete Abstract where view = project` checks that the initial state matches and every concrete step either leaves `view s` unchanged (a stuttering step) or matches an existing transition in `Abstract`.
- **Symmetry reduction.** Both TLC and Cinch canonicalize states modulo permutations of symmetric actor sets to shrink the explored graph.

## What is different (and what Cinch adds)

1. **Zero model-code drift.** A TLA+ spec lives in `.tla`/`.cfg` files separate from your implementation. In Cinch, the `record`, `data`, refinement types, and transition functions checked by `machine` are the exact same declarations compiled into your executable.
2. **Types replace `TypeOK` and set-comprehension boilerplate.** Instead of untyped variables plus `TypeOK`, `InRange 0 3 Nat`, `Fin 4`, `Option Node`, and tuples encode finite domains directly in signatures.
3. **Type-checked symmetry soundness.** TLC trusts the user not to break symmetry (for example, by comparing model values with `<` or using `CHOOSE` deterministically). Cinch enforces soundness through the type system: listing `Node` in `symmetry = [Node]` is **rejected at check time** if an `Ord Node` instance is in scope.
4. **Automatic static Partial Order Reduction (POR).** Cinch inspects each action's record field reads and writes statically, identifies commuting action pairs with disjoint footprints, and reports them in build manifests and `cinch model inspect`.
5. **Built-in stuttering livelock checking (`measure`).** Adding `measure = rankFn` to a `refines` block verifies that every silent stuttering step strictly reduces a `Nat` measure, ruling out infinite internal loops without manually crafting auxiliary liveness formulas.
6. **Assume-guarantee environment constraints (`assume = (\s -> ...)`).** Machine contracts accept predicate assumptions that restrict checking to environment-valid reachable states and record those assumptions explicitly in `.manifest.json` and `cinch model inspect`.
7. **Runtime-backed concurrency fairness (`MVar` vs. `STM`).** Cinch links temporal liveness directly to its runtime primitives: `Effect.Concurrent.MVar` uses strict FIFO ticketed locks and waiter queues so `eventually = (fair_handoff)` discharges automatically, while optimistic `Effect.Stm` refuses unassumed fairness claims.
8. **Built-in CLI, REPL, and LSP exploration.** `cinch model inspect|trace|dot`, REPL commands (`:machines`, `:machine`, `:explore`, `:trace`, `:dot`), and rich LSP hover/go-to-definition work out of the box—even when a contract is currently failing—and counterexamples annotate each step with `(changed: field: old -> new)`.

## What is missing compared to TLA+ / TLC / TLAPS / Apalache

Cinch's model checker is designed to live inside your compiler's normal check loop, not to replace heavyweight external verification suites. Be aware of these current gaps:

| Capability in TLA+ ecosystem | Status in Cinch today | Practical workaround |
| --- | --- | --- |
| **Unbounded symbolic checking (Apalache / SMT) or deductive proofs (TLAPS)** | Not implemented. Checking is bounded explicit-state BFS up to `max_states` (default `4096`). | Check small finite instantiations (`Fin 3`, `symmetry = [Node]`) and combine with Cinch's refinement/dependent types for unbounded invariants. |
| **Arbitrary nested LTL / TLA formulas** (`[](P => <>(Q /\ []R))`, `Until`, prophecy variables) | Not implemented. Contracts support the six top-level forms (`always`, `deadlock_free`, `stable`, `eventually`, `leads_to`, `recur`) plus `assume`. | Factor temporal phases into explicit state fields or decompose with a `refines` specification machine. |
| **Built-in `Set`, `SUBSET` (powerset), `[S -> T]` function spaces, and `CHOOSE`** | Not built into the model checker. | Represent finite sets and maps as small records, tuples, `Option` fields, or canonicalized `List`s. |
| **Per-instance fairness (`\A n \in Nodes : WF_v(Act(n))`)** | `fair = [act]` applies at the action-name level, not separately per enumerated parameter value. | When fairness must hold per actor independently, split per-actor actions (e.g. `stepA`, `stepB`) or track round-robin turn state explicitly. |
| **Multi-core disk-backed state stores (millions of states)** | Single-threaded in-memory BFS inside the compiler check pass. | Keep checked models compact with `symmetry`, product decomposition, assume-guarantee `assume`, and `refines` layers. |
| **Simulation / random-walk mode (`tlc -simulate`) & PlusCal imperative syntax** | Pure stdlib library (`Effect.Test.Hegel.hegel-check-machine`) — runs at native LLVM speed with integrated trace shrinking, reusing the exact same pure action functions. | Use `machine` for exhaustive small-scope checking and `Effect.Test.Hegel` for large-state native randomized simulation. |
| **Automatic extraction of `machine` from arbitrary `spawn`/`IO` code** | Not implemented. | Isolate pure protocol transitions as `machine` actions and invoke those exact same pure functions from your concurrent `MVar`/`IO` runtime loops. |

---

# Part II: What values and expressions machines can and cannot check

Because the model checker evaluates `init`, actions, `view`, `measure`, and contract predicates at compile time and hashes every visited state into a graph, every value participating in a `machine` falls into one of three positions: **state fields**, **action parameters**, and **action/predicate bodies**.

## 1. State types (`state = StateType`)

Every value reachable in `StateType` must be **inspectable and structurally hashable data**.

| Category | Checkable in `state`? | Examples |
| --- | --- | --- |
| Arbitrary-precision integers | **Yes** | `Nat`, `Int` |
| Fixed-width machine scalars | **Yes** | `I8`, `I16`, `I32`, `I64`, `U8`, `U16`, `U32`, `U64`, `Isize`, `Usize`, `Char` |
| Booleans, unit, text, symbols, bytes | **Yes** | `Bool`, `Unit`, `String`, `Sym`, `Bytes` |
| Records & tuples | **Yes** (recursively) | `record State where ...`, `(Nat, Bool)` |
| Algebraic `data` types | **Yes** (recursively) | Nullary enums, payload constructors, `Option a`, `Result e a`, `List a`, trees |
| Refined value types | **Yes** | `InRange 0 10 Nat`, `Fin 4`, `NonZero Int` |
| Functions / closures (`a -> b`) | **No** — rejected | Putting a callback `Int -> Int` in `State` fails with `not inspectable/hashable data` |
| Lazy thunks & `codata` streams | **No** — rejected | `Thunk a`, `Later k a`, `codata` instances cannot be hashed as finite state |
| Mutable references & borrows | **No** — rejected | `Mut r a`, `Ref r a` |
| Concurrency & I/O handles | **No** — rejected | `MVar a`, `TVar a`, `Task a`, `Scope`, `Io r` |
| Foreign opaque pointers | **No** — rejected | `RawPtr` |

Putting a function/closure inside a `machine` state is rejected immediately at check time:

<!-- cinch:reject contains="not inspectable/hashable data" -->
```cinch
module Main where
  record BadState where
    handler :: Nat -> Nat

  noop :: BadState -> BadState
  noop s = s

  machine BadMachine where
    state = BadState
    init = BadState { handler = (\x -> x) }
    actions = [noop]

  main :: Int
  main = 0
```

## 2. Action parameter domains (`P1 -> ... -> Pk -> State -> ...`)

Every action listed in `actions = [...]` must take `StateType` as its **last** parameter. You may place **zero or more** prefix parameters before `StateType`. At every state, the model checker enumerates the finite domain of each prefix parameter and forms their Cartesian product (up to **1,024 parameter combinations per action per state**; individual domains are capped at **256 values**).

| Parameter type | Enumerable? | Values enumerated |
| --- | --- | --- |
| `Bool` | **Yes** (2 values) | `False`, `True` |
| `Unit` | **Yes** (1 value) | `()` |
| Nullary `data` enum | **Yes** (all constructors) | Every constructor in declaration order (e.g. `N0`, `N1`, `N2`) |
| `Fin n` (static literal `n <= 256`) | **Yes** (`n` values) | `0, 1, ..., n - 1` |
| `InRange lo hi Nat` (or refined `Int`/`Nat`/scalar with static bounds, `hi - lo <= 256`) | **Yes** (`hi - lo + 1` values) | `lo, lo + 1, ..., hi` |
| `Option T` where `T` is enumerable (`1 + |T| <= 256`) | **Yes** | `None`, followed by `Some v` for every `v` in `T` |
| Tuple `(A, B)` / `(A, B, ...)` of enumerable types (product `<= 256`) | **Yes** | All combinations `(a, b, ...)` in lexicographic order |
| Any user record or payload ADT implementing `Data.Finite.Finite` (`<= 256`) | **Yes** | Evaluates `enumerate () :: List P` at check time |
| Multiple prefix params `P1 -> P2 -> ... -> State -> ...` | **Yes** (product `<= 1024`) | Cartesian product of each parameter's domain |
| Bare `Nat`, `Int`, `U64`, `I64`, `Char` | **No** — rejected | Unbounded / too large; refine with `InRange lo hi Nat` or `Fin n` |
| `String`, `Bytes`, `Sym` | **No** — rejected | Infinite domain; replace with a finite enum or a custom `Finite` wrapper |
| `List a`, `Map k v` | **No** — rejected | Unbounded length; replace with a fixed tuple, `Finite` wrapper, or simulate with `Effect.Test.Hegel` |
| Payload-carrying or recursive `data` without `Finite` | **No** — rejected | Implement `Data.Finite.Finite` (or pass components as separate parameters/tuples) |
| `Fin n` with non-constant `n`, or range size `> 256` | **No** — rejected | Keep per-parameter domains small and static |
| Functions (`a -> b`) | **No** — rejected | Higher-order action parameters are not enumerated |

Here is a valid action combining multiple prefix parameters—an enum `Node`, an `Option Node`, a finite tuple `(Bool, Fin 2)`, and an `InRange` bound—checked cleanly:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Node where
    A :: Node
    B :: Node

  record Net where
    delivered :: Nat
    lastFrom :: Option Node
    priority :: Bool

  route :: Node -> Option Node -> (Bool, InRange 1 2 Nat) -> Net -> Option Net
  route src maybeDst opts s = case opts of
    (hi, weight) -> case maybeDst of
      None -> None
      Some dst ->
        if s.delivered + weight <= 3
        then Some (Net { delivered = s.delivered + weight, lastFrom = Some src, priority = hi })
        else None

  machine Router where
    state = Net
    init = Net { delivered = 0, lastFrom = None, priority = False }
    actions = [route]

  contract safety capDelivered for Router where
    always = (\s -> s.delivered <= 3)

  main :: Int
  main = 0
```

### Extensible domains with `Data.Finite`

For payload-carrying algebraic data types or user records (such as protocol messages), implement `Data.Finite.Finite` (`enumerate :: Unit -> List a`) using `finite-map`, `finite-product`, and `finite-sum`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.Finite

  data Client where
    C0 :: Client
    C1 :: Client

  data Msg where
    Req :: Client -> Nat -> Msg
    Ack :: Client -> Msg

  implicit instance Finite_Client :: Finite Client where
    enumerate _ = [C0, C1]

  implicit instance Finite_Msg :: Finite Msg where
    enumerate _ =
      finite-sum [
        finite-product (\c -> \w -> Req c w) (enumerate ()) [1, 2],
        finite-map Ack (enumerate ())
      ]

  record Broker where
    load :: Nat

  dispatch :: Msg -> Broker -> Option Broker
  dispatch msg s = case msg of
    Req _ w ->
      if s.load + w <= 3
      then Some (Broker { load = s.load + w })
      else None
    Ack _ ->
      let l = s.load in
      if l > 0 then Some (Broker { load = l - 1 }) else None

  machine BrokerMachine where
    state = Broker
    init = Broker { load = 0 }
    actions = [dispatch]

  contract safety boundedLoad for BrokerMachine where
    always = (\s -> s.load <= 3)

  main :: Int
  main = 0
```

Passing an unbounded type such as bare `Nat` as an action parameter is rejected explicitly instead of silently guessing a cutoff:

<!-- cinch:reject contains="has no finite enumerable domain" -->
```cinch
module Main where
  record Counter where
    n :: Nat

  addAny :: Nat -> Counter -> Option Counter
  addAny k s = if s.n + k <= 5 then Some (Counter { n = s.n + k }) else None

  machine BadCounter where
    state = Counter
    init = Counter { n = 0 }
    actions = [addAny]

  main :: Int
  main = 0
```

## 3. Action return shapes and evaluation rules

Every action must be **pure** (empty effect row `{}`):

| Action return type | Meaning |
| --- | --- |
| `... -> StateType -> StateType` | Always enabled; produces 1 next state. |
| `... -> StateType -> Option StateType` | Guarded; `Some s'` produces 1 transition, `None` disables the action in this state. |
| `... -> StateType -> List StateType` | Non-deterministic; each element of the list becomes an outgoing transition (`Nil` disables the action). |

Effectful actions (`IO`, `STM`, `MVar`, `Async`, `Foreign`, etc.) are rejected:

<!-- cinch:reject contains="must be pure for model checking" -->
```cinch
module Main where
  import Effect.Io

  record S where
    n :: Nat

  logStep :: S ->{IO r} S
  logStep s = s

  machine Impure where
    state = S
    init = S { n = 0 }
    actions = [logStep]

  main :: Int
  main = 0
```

Additional evaluation boundaries:

- **Compile-time fuel:** Actions, `init`, `view`, `measure`, and contract predicates run in the compiler's pure evaluator with a per-call step budget. Non-terminating recursion inside a single action step fails checking with a fuel error rather than hanging the compiler.
- **No dynamic allocation of effects or FFI:** Only pure Cinch functions, pattern matching, arithmetic, records, lists, and pure standard-library helpers execute during exploration.

## 4. State caps, symmetry, and POR boundaries

- **`max_states` (default `4096`):** Exploration stops inserting new states once `max_states` distinct canonical states are reached. When this happens, the machine is flagged as **capped** (`[CAPPED]` in `cinch model inspect` and `:machines`, `"capped": true` in `--json` and manifests). Treat a capped exploration as bounded bug-finding, not exhaustive verification—either tighten state bounds, add `symmetry`, or raise `max_states`.
- **Symmetry reduction (`symmetry = [T1, ...]`):**
  - Works on nullary `data` enums (`data Node = N0 | N1 | N2`) and single-field nominal wrappers (`data JobId = JobId Nat`).
  - **Requires no `Ord T` or `Finite T` instance in scope.** Having either `Ord T` or `Finite T` exposes a deterministic order on `T`, breaks permutation invariance, and is rejected with `unsound symmetry reduction`.
  - Uses first-appearance canonicalization on every state visited, merging equivalent states during BFS.
- **Static field-footprint POR:**
  - Automatically analyzes each action body to determine which fields of the state record are read and which are written.
  - Two actions with disjoint `(writes(A) ∩ (reads(B) ∪ writes(B)) = ∅)` and `(writes(B) ∩ (reads(A) ∪ writes(A)) = ∅)` are proved independent.
  - Passing the whole state `s` opaquely to an un-inlined helper function conservatively marks that action's footprint as opaque (never unsound—it simply stops claiming independence for that action).

---

For hands-on patterns that keep state spaces small while verifying real multi-component systems—product records, assume-guarantee contracts, refinement towers, horizontal multi-view refinement, and CLI/REPL/LSP debugging—continue to [Learn Chapter 24: Composing state machines and debugging models](../learn/24-composing-state-machines.md).
