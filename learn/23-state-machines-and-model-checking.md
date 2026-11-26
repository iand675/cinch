# 23. State machines, fairness, and model checking

Unit tests check the handful of execution paths you thought to write down. Concurrent services, retry loops, payment pipelines, and lock protocols fail on the interleaving you did not think to write down: the third retry that runs while a lease expires, the worker that steals a slot before a check finishes, or the state where every actor waits on everyone else.

Standalone model checkers such as TLA+ catch those bugs by exploring every reachable state of a system automatically. Their weak spot is **drift**: the specification lives in a separate language and file, so three months after you check the model, the production code no longer matches it.

Cinch builds explicit-state model checking directly into the compiler. Your state is an ordinary Cinch `record`, your transitions are ordinary Cinch functions, your action inputs come from Cinch's own finite types and refinements (`Bool`, enums, `Fin n`, `InRange`), and `refines` checks that a detailed implementation actually stays faithful to a simpler specification. You do not need any background in TLA+ or formal methods to use any of it.

Prerequisites: [data and matching](04-data-and-matching.md) for records and algebraic data types, [refinements](13-refinements-and-safe-operations.md) for `InRange`, [productive streams](15-streams-and-producers.md) for `codata` and `Later`, and [state evolution and temporal contracts](21-upgrades-and-contracts.md) for basic `contract` syntax. The exact rules live in [runtime upgrades and temporal contracts](../reference/17-upgrades-and-temporal-contracts.md).

# What a state machine is

Think of a protocol or service as three plain pieces:

1. **The state**: a snapshot of everything that matters right now, stored in an ordinary `record` (for example, how many jobs are queued, running, and completed).
2. **The initial state (`init`)**: the snapshot the system starts in before anything happens.
3. **The actions (`actions`)**: pure Cinch functions that take the current state and produce the next state.

An action function has one of three return shapes:

| Return shape | Meaning |
| --- | --- |
| `State -> State` | Always allowed (enabled in every state) and moves to one next state. |
| `State -> Option State` | Guarded: `Some next` means the action can fire and produces `next`; `None` means the action is currently disabled. |
| `State -> List State` | Non-deterministic: each element of the list is a possible next state; `Nil` means the action is disabled. |

When you declare a `machine`, the compiler runs a breadth-first search starting from `init`, trying every enabled action from every discovered state until it has mapped out the entire reachable state graph. Then it checks every `contract` attached to that machine against the whole graph.

# Your first machine and safety invariant

Start with a bounded worker pool that holds at most `2` active jobs (queued plus running) and runs at most `1` job at a time. Each action is an ordinary function you could also call at runtime:

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

  contract safety poolCapacity for WorkerPool where
    always = (\s -> s.queued + s.running <= 2 && s.running <= 1)

  main :: Int
  main = 0
```

Read `contract safety poolCapacity for WorkerPool` as a plain sentence: *"No matter what sequence of `accept`, `start`, and `finish` steps happens starting from `init`, `s.queued + s.running <= 2` and `s.running <= 1` is `always` true."*

Notice two details:

- `let q = s.queued in if q > 0 then ... q - 1 ...` gives Cinch's refinement checker the branch fact `q > 0` it needs to prove `q - 1` cannot underflow `Nat`. The same typechecker protects your model code and your runtime code.
- `finish` bounds `s.done < 3` so the reachable state space is finite. Bounded model checking explores every reachable state up to `max_states` (default `4096`). If a counter can grow without bound, the checker stops at `max_states` and tells you explicitly that the bound was exceeded rather than pretending it proved the unbounded case.

# Catching a bug and reading a counterexample trace

A **safety property** says *"something bad never happens."* When a safety property fails, there is always a concrete, finite sequence of steps from `init` to a bad state. Because the checker explores states in breadth-first order, it always prints the **shortest possible reproduction trace**.

Suppose we write a two-account transfer system with a bug: `overdraw` forgets to check that the sender has enough balance, or `accept` forgets to count running jobs against the queue cap:

<!-- cinch:reject contains="temporal safety violation" -->
```cinch
module Main where
  record Pool where
    queued :: Nat
    running :: Nat

  -- Bug: checks only s.queued < 2, ignoring jobs that are already running!
  accept :: Pool -> Option Pool
  accept s =
    if s.queued < 2
    then Some (Pool { queued = s.queued + 1, running = s.running })
    else None

  start :: Pool -> Option Pool
  start s =
    let q = s.queued in
    if q > 0 && s.running < 1
    then Some (Pool { queued = q - 1, running = s.running + 1 })
    else None

  finish :: Pool -> Option Pool
  finish s =
    let r = s.running in
    if r > 0
    then Some (Pool { queued = s.queued, running = r - 1 })
    else None

  machine BuggyPool where
    state = Pool
    init = Pool { queued = 0, running = 0 }
    actions = [accept, start, finish]

  contract safety poolCapacity for BuggyPool where
    always = (\s -> s.queued + s.running <= 2)

  main :: Int
  main = 0
```

The compiler rejects the program during `cinch check` and hands you the exact sequence of actions that breaks the invariant:

```text
cinch: temporal safety violation: contract `poolCapacity` on machine `BuggyPool` falsified:
  State 0 (init): Pool { queued = 0, running = 0 }
    --[accept]-->
  State 1: Pool { queued = 1, running = 0 }
    --[start]-->
  State 2: Pool { queued = 0, running = 1 }
    --[accept]-->
  State 3: Pool { queued = 1, running = 1 }
    --[accept]-->
  State 4: Pool { queued = 2, running = 1 }   <-- violates always
```

You did not have to guess that specific 4-step sequence (`accept`, `start`, `accept`, `accept`). You stated the capacity rule once, and the compiler found the minimal failing interleaving for you.

# Catching deadlocks

A **deadlock** is a reachable state where every action returns `None` (or `Nil`), so the machine is permanently stuck. Even if every safety invariant holds, a stuck protocol is usually a bug.

Attach `deadlock_free = (True)` to a `safety` contract to verify that every reachable state has at least one enabled outgoing action. Here is a bounded request limiter that forgets to add a reset/release transition once it hits its cap of `2`:

<!-- cinch:reject contains="deadlock" -->
```cinch
module Main where
  record Limiter where
    inFlight :: Nat

  acquire :: Limiter -> Option Limiter
  acquire s =
    if s.inFlight < 2
    then Some (Limiter { inFlight = s.inFlight + 1 })
    else None

  machine StuckLimiter where
    state = Limiter
    init = Limiter { inFlight = 0 }
    actions = [acquire]

  contract safety noDeadlock for StuckLimiter where
    deadlock_free = (True)

  main :: Int
  main = 0
```

The compiler prints the shortest path to the deadlocked state:

```text
  State 0 (init): Limiter { inFlight = 0 }
    --[acquire]-->
  State 1: Limiter { inFlight = 1 }
    --[acquire]-->
  State 2: Limiter { inFlight = 2 }   <-- deadlock (no action enabled)
```

Adding a `release` action fixes the deadlock, and `deadlock_free` passes:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Limiter where
    inFlight :: Nat

  acquire :: Limiter -> Option Limiter
  acquire s =
    if s.inFlight < 2
    then Some (Limiter { inFlight = s.inFlight + 1 })
    else None

  release :: Limiter -> Option Limiter
  release s =
    let n = s.inFlight in
    if n > 0
    then Some (Limiter { inFlight = n - 1 })
    else None

  machine LiveLimiter where
    state = Limiter
    init = Limiter { inFlight = 0 }
    actions = [acquire, release]

  contract safety noDeadlock for LiveLimiter where
    deadlock_free = (True)

  main :: Int
  main = 0
```

If a machine intentionally has a terminal "finished" state (like a one-shot workflow that finishes in `Done`), model that terminal state explicitly with an `idle` self-loop (`if s.done then Some s else None`) when you want `deadlock_free` to distinguish intentional completion from accidental stuck states.

# Liveness: "something good eventually happens"

Safety tells you the system never enters a bad state—but a machine that sits still and does nothing satisfies every `always` invariant trivially. **Liveness** asks the other half of the question: *does the system actually make progress?*

Cinch checks four progress and persistence operators on machines:

| Contract kind | Operator | Plain-English meaning |
| --- | --- | --- |
| `liveness` | `eventually = (\s -> Q)` | Starting from `init`, every valid infinite run reaches a state where `Q` is true. |
| `liveness` | `leads_to = (\s -> P) ~> (\s -> Q)` | Whenever the system reaches any state where `P` is true, it must eventually reach a state where `Q` is true. |
| `recurrence` | `recur = (\s -> Q)` | `Q` becomes true over and over again, infinitely often (it never stops happening). |
| `stability` | `stable = (\s -> Q)` | Once `Q` becomes true, it stays true forever (`Q` never reverts to false). |

Notice the crucial difference from safety: you cannot disprove *"this request eventually gets a response"* with a finite trace, because maybe the response happens on the very next step! To disprove a liveness property, the model checker looks for an **infinite loop** (called a **lasso** or **fair cycle**): a prefix path from `init` followed by a cycle of states that repeats forever without ever reaching `Q`.

# Why fairness exists: Weak (`fair`) vs Strong (`strong_fair`)

If you have never used TLA+ before, **fairness** is the single concept that surprises people most—and once you see one example, it clicks immediately.

Suppose you model a job service with three actions:

- `tick`: a background clock or heartbeat that advances `ticks` (`0 -> 1 -> 2 -> 0`). It is always enabled.
- `complete`: finishes a pending job (`pending = True -> False`).
- `submit`: queues a new job (`pending = False -> True`).

You want to verify `leads_to = (\s -> s.pending) ~> (\s -> not s.pending)`: *"every pending job eventually completes."*

Without telling the checker anything about scheduling, the checker asks: *"What if the scheduler keeps running `tick` forever and never schedules `complete`?"* That is a completely valid loop in the state graph (`tick -> tick -> tick -> ...`), so the checker rightly rejects the liveness claim!

Fairness rules out unrealistic "ignore this ready action forever" schedules. There are **two** kinds of fairness, and the difference between them matters in real concurrent systems:

- **Weak fairness (`fair = [a]`)**: *"If action `a` stays **continuously** enabled from some point onward (nothing ever disables it), it must eventually fire."*
- **Strong fairness (`strong_fair = [a]`)**: *"If action `a` becomes enabled **repeatedly** (infinitely often, even if other actions keep turning it off and back on), it must eventually fire."*

## Weak fairness: continuously ready work

In the job service below, while `s.pending` is `True`, `tick` never disables `complete`—`complete` stays continuously ready across every `tick`. That is the exact shape **weak fairness** (`fair`) handles:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Job where
    pending :: Bool
    ticks :: Nat

  tick :: Job -> Option Job
  tick s =
    if s.ticks < 2
    then Some (Job { pending = s.pending, ticks = s.ticks + 1 })
    else Some (Job { pending = s.pending, ticks = 0 })

  complete :: Job -> Option Job
  complete s =
    if s.pending
    then Some (Job { pending = False, ticks = s.ticks })
    else None

  submit :: Job -> Option Job
  submit s =
    if not s.pending
    then Some (Job { pending = True, ticks = s.ticks })
    else None

  machine JobService where
    state = Job
    init = Job { pending = True, ticks = 0 }
    actions = [tick, complete, submit]
    fair = [complete, submit]

  contract liveness alwaysCompletes for JobService where
    leads_to = (\s -> s.pending) ~> (\s -> not s.pending)

  contract recurrence keepsServing for JobService where
    recur = (\s -> not s.pending)

  main :: Int
  main = 0
```

Because `complete` is weakly fair, any infinite loop that stays in `pending = True` forever is ruled out—`complete` stayed enabled on every step of the loop and never fired, violating weak fairness. And because `submit` is also weakly fair, the service cycles cleanly between pending and idle forever, discharging `recur = (\s -> not s.pending)`.

## Strong fairness: intermittently ready work (flaky links and contended locks)

Now consider a realistic case where weak fairness is **not enough**: sending a message over a link that keeps flapping up and down, or acquiring a lock that another worker keeps taking and releasing.

- `flap` toggles `linkUp` between `False` and `True` while `sent == False`.
- `deliver` can only fire when `linkUp == True && not sent`.
- `idle` self-loops once `sent == True`.

Watch what happens under all three fairness settings!

**1. No fairness** — rejected with a `flap` cycle:

<!-- cinch:reject contains="temporal liveness violation" -->
```cinch
module Main where
  record Link where
    linkUp :: Bool
    sent :: Bool

  flap :: Link -> Option Link
  flap s =
    if not s.sent
    then Some (Link { linkUp = not s.linkUp, sent = False })
    else None

  deliver :: Link -> Option Link
  deliver s =
    if s.linkUp && not s.sent
    then Some (Link { linkUp = False, sent = True })
    else None

  idle :: Link -> Option Link
  idle s = if s.sent then Some s else None

  machine FlakyLink where
    state = Link
    init = Link { linkUp = False, sent = False }
    actions = [flap, deliver, idle]

  contract liveness eventuallyDelivered for FlakyLink where
    leads_to = (\s -> not s.sent) ~> (\s -> s.sent)

  main :: Int
  main = 0
```

**2. Weak fairness (`fair = [deliver]`)** — **still rejected!**

<!-- cinch:reject contains="fair cycle" -->
```cinch
module Main where
  record Link where
    linkUp :: Bool
    sent :: Bool

  flap :: Link -> Option Link
  flap s =
    if not s.sent
    then Some (Link { linkUp = not s.linkUp, sent = False })
    else None

  deliver :: Link -> Option Link
  deliver s =
    if s.linkUp && not s.sent
    then Some (Link { linkUp = False, sent = True })
    else None

  idle :: Link -> Option Link
  idle s = if s.sent then Some s else None

  machine WeakLink where
    state = Link
    init = Link { linkUp = False, sent = False }
    actions = [flap, deliver, idle]
    fair = [deliver]

  contract liveness eventuallyDelivered for WeakLink where
    leads_to = (\s -> not s.sent) ~> (\s -> s.sent)

  main :: Int
  main = 0
```

Why did weak fairness fail? Trace the cycle:

```text
Link { linkUp = False, sent = False }
  --[flap]-->
Link { linkUp = True, sent = False }    -- deliver IS enabled here
  --[flap]-->
Link { linkUp = False, sent = False }   -- deliver is DISABLED again!
```

Because `deliver` keeps getting disabled every time `linkUp` flips back to `False`, it is **never continuously enabled** throughout the cycle. Weak fairness therefore allows the cycle—which matches real life: an unlucky polling loop or lock waiter that only checks while the link is down (or while another thread holds the lock) can starve forever!

**3. Strong fairness (`strong_fair = [deliver]`)** — **verifies!**

<!-- cinch:run exit=0 -->
```cinch
module Main where
  record Link where
    linkUp :: Bool
    sent :: Bool

  flap :: Link -> Option Link
  flap s =
    if not s.sent
    then Some (Link { linkUp = not s.linkUp, sent = False })
    else None

  deliver :: Link -> Option Link
  deliver s =
    if s.linkUp && not s.sent
    then Some (Link { linkUp = False, sent = True })
    else None

  idle :: Link -> Option Link
  idle s = if s.sent then Some s else None

  machine StrongLink where
    state = Link
    init = Link { linkUp = False, sent = False }
    actions = [flap, deliver, idle]
    strong_fair = [deliver]

  contract liveness eventuallyDelivered for StrongLink where
    leads_to = (\s -> not s.sent) ~> (\s -> s.sent)

  contract liveness reachesSent for StrongLink where
    eventually = (\s -> s.sent)

  contract stability staysSent for StrongLink where
    stable = (\s -> s.sent)

  main :: Int
  main = 0
```

Under strong fairness, because `deliver` becomes enabled on every pass through `linkUp = True`, a fair run cannot ignore it forever. Notice `staysSent` too: `stable = (\s -> s.sent)` proves that once `sent` flips to `True`, no action can ever revert it to `False`.

**Rule of thumb:** Always try `fair` first. Only reach for `strong_fair` when an action is genuinely intermittent and your runtime actually provides a strong-fairness mechanism (like a FIFO queue or Cinch's fair `MVar`, covered below). Over-assuming `strong_fair` hides real starvation bugs!

# Parameterized actions, symmetry, and partial order reduction

Real protocols rarely have zero-argument actions: a specific node acquires a lock, a specific client retries with a bounded backoff, or a worker handles a numbered shard.

In a standalone model checker, you have to maintain a separate configuration file listing finite constants for every parameter. In Cinch, **your types already tell the model checker the finite domain**:

- `Bool` enumerates `[False, True]`.
- Finite nullary `data` enums (such as `data Node where N0 :: Node; N1 :: Node`) enumerate every constructor.
- `Fin n` enumerates `0 .. n - 1` when `n` is a known numeral.
- `InRange lo hi Nat` (and refined integer ranges) enumerate `lo .. hi`.

Action parameters come **before** the `State` parameter (`acquire :: Node -> Cluster -> Option Cluster`).

As you add nodes or independent subsystems, raw state spaces grow quickly. Cinch ships two automatic state-space reductions that exploit information already in your types:

1. **Type-checked symmetry reduction (`symmetry = [Node]`).** If your protocol treats `N0` and `N1` interchangeably—never breaking ties by ordering `N0 < N1`—then "N0 holds the lock" and "N1 holds the lock" are symmetric images of the same state. Listing `symmetry = [Node]` merges them into one canonical state, shrinking the state space factorially as node count grows.
2. **Automatic Partial Order Reduction (POR).** The compiler statically inspects which record fields each action reads and writes. When two actions touch disjoint fields (like `bumpMetrics` and `bumpAudit` below), running them in either order produces the same state; Cinch detects those independent pairs automatically and records them in the build manifest.

Here is a cluster lock machine combining parameterized actions (`Node` and `InRange 0 1 Nat`), symmetry reduction, and independent counters:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  data Node where
    N0 :: Node
    N1 :: Node

  record Cluster where
    leaseHolder :: Option Node
    metricsTicks :: Nat
    auditTicks :: Nat

  acquire :: Node -> Cluster -> Option Cluster
  acquire n s = case s.leaseHolder of
    None -> Some (Cluster { leaseHolder = Some n, metricsTicks = s.metricsTicks, auditTicks = s.auditTicks })
    Some _ -> None

  release :: Cluster -> Option Cluster
  release s = case s.leaseHolder of
    Some _ -> Some (Cluster { leaseHolder = None, metricsTicks = s.metricsTicks, auditTicks = s.auditTicks })
    None -> None

  bumpMetrics :: InRange 0 1 Nat -> Cluster -> Option Cluster
  bumpMetrics d s =
    if s.metricsTicks == 0 && d == 1
    then Some (Cluster { leaseHolder = s.leaseHolder, metricsTicks = 1, auditTicks = s.auditTicks })
    else None

  bumpAudit :: Cluster -> Option Cluster
  bumpAudit s =
    if s.auditTicks == 0
    then Some (Cluster { leaseHolder = s.leaseHolder, metricsTicks = s.metricsTicks, auditTicks = 1 })
    else None

  machine LeaseCluster where
    state = Cluster
    init = Cluster { leaseHolder = None, metricsTicks = 0, auditTicks = 0 }
    actions = [acquire, release, bumpMetrics, bumpAudit]
    symmetry = [Node]

  contract safety boundedTicks for LeaseCluster where
    always = (\s -> s.metricsTicks <= 1 && s.auditTicks <= 1)

  main :: Int
  main = 0
```

## Symmetry soundness is enforced by the type system

Symmetry reduction is only sound if your code cannot break symmetry—for example, by electing the node with the smallest ID using `<`. In Cinch, declaring `symmetry = [Node]` asks the compiler to verify that `Node` has **no `Ord` instance** in scope. If you add `Ord Node`, the compiler refuses the unsound symmetry declaration immediately:

<!-- cinch:reject contains="unsound symmetry reduction" -->
```cinch
module Main where
  data Node where
    N0 :: Node
    N1 :: Node

  nodeTag :: Node -> Nat
  nodeTag n = case n of
    N0 -> 0
    N1 -> 1

  implicit instance EqNode :: Eq Node where
    (==) a b = nodeTag a == nodeTag b
    (/=) a b = nodeTag a /= nodeTag b

  implicit instance OrdNode :: Ord Node where
    (<) a b = nodeTag a < nodeTag b
    (<=) a b = nodeTag a <= nodeTag b
    (>) a b = nodeTag a > nodeTag b
    (>=) a b = nodeTag a >= nodeTag b

  record Cluster where
    leaseHolder :: Option Node

  acquire :: Node -> Cluster -> Option Cluster
  acquire n s = Some (Cluster { leaseHolder = Some n })

  machine BadSymCluster where
    state = Cluster
    init = Cluster { leaseHolder = None }
    actions = [acquire]
    symmetry = [Node]

  main :: Int
  main = 0
```

You never have to trust yourself to remember whether a helper somewhere secretly compared two node IDs.

# Stopping model-code drift with `refines`

Why do specifications and implementations drift in real projects? Because a clean specification is simple—say, a checkout goes from `paid = False` to `paid = True` in one step—while the real implementation has extra internal steps: `Idle -> Authorizing -> Capturing -> Paid`.

In Cinch, you connect the two machines with a **`refines` declaration** and a plain Cinch projection function `view :: ConcreteState -> AbstractState`:

```text
refines ConcreteMachine AbstractMachine where
  view = project
  measure = remainingInternalSteps   -- optional, prevents infinite internal spinning
```

For every reachable step `c1 -> c2` in the concrete machine, the compiler checks two things:

1. **Initial states match**: `view concreteInit == abstractInit`.
2. **Every concrete step is either a real abstract step or a stuttering step**:
   - Either `view c1 == view c2` (a **stuttering step**: an internal implementation step that has not changed the abstract view yet, like `Idle -> Authorizing`),
   - Or `view c1 -> view c2` is a legal transition of `AbstractMachine` (like `Capturing -> Paid` mapping to `paid = False -> True`).

And when you supply `measure :: ConcreteState -> Nat`, the compiler also checks that **every stuttering step strictly decreases `measure`**, proving the implementation cannot get stuck in an infinite loop of silent internal steps (livelock)!

## A valid multi-step checkout refining a one-step spec

<!-- cinch:run exit=0 -->
```cinch
module Main where
  -- High-level specification: an order simply moves from unpaid to paid.
  record OrderSpecState where
    paid :: Bool

  specPay :: OrderSpecState -> Option OrderSpecState
  specPay s = if not s.paid then Some (OrderSpecState { paid = True }) else None

  machine OrderSpec where
    state = OrderSpecState
    init = OrderSpecState { paid = False }
    actions = [specPay]

  -- Concrete implementation:
  -- phase 0 = Idle, phase 1 = Authorizing (stutter), phase 2 = Capturing (stutter), phase 3 = Paid
  record CheckoutState where
    phase :: Nat

  authorize :: CheckoutState -> Option CheckoutState
  authorize s = if s.phase == 0 then Some (CheckoutState { phase = 1 }) else None

  capture :: CheckoutState -> Option CheckoutState
  capture s = if s.phase == 1 then Some (CheckoutState { phase = 2 }) else None

  settle :: CheckoutState -> Option CheckoutState
  settle s = if s.phase == 2 then Some (CheckoutState { phase = 3 }) else None

  machine CheckoutImpl where
    state = CheckoutState
    init = CheckoutState { phase = 0 }
    actions = [authorize, capture, settle]

  project :: CheckoutState -> OrderSpecState
  project s = OrderSpecState { paid = s.phase == 3 }

  stepsUntilAbstractMove :: CheckoutState -> Nat
  stepsUntilAbstractMove s =
    if s.phase == 0 then 3
    else (if s.phase == 1 then 2
    else (if s.phase == 2 then 1 else 0))

  refines CheckoutImpl OrderSpec where
    view = project
    measure = stepsUntilAbstractMove

  main :: Int
  main = 0
```

Both `authorize` (`0 -> 1`) and `capture` (`1 -> 2`) leave `project s` equal to `OrderSpecState { paid = False }` while decreasing `stepsUntilAbstractMove` (`3 -> 2 -> 1`), and `settle` (`2 -> 3`) matches `specPay`.

## Catching implementation drift automatically

Now suppose someone adds a `chargeback` or `rollback` transition to `CheckoutImpl` that moves a paid order back to unpaid—something `OrderSpec` explicitly forbids:

<!-- cinch:reject contains="refinement violation" -->
```cinch
module Main where
  record OrderSpecState where
    paid :: Bool

  specPay :: OrderSpecState -> Option OrderSpecState
  specPay s = if not s.paid then Some (OrderSpecState { paid = True }) else None

  machine OrderSpec where
    state = OrderSpecState
    init = OrderSpecState { paid = False }
    actions = [specPay]

  record CheckoutState where
    phase :: Nat

  settle :: CheckoutState -> Option CheckoutState
  settle s = if s.phase == 0 then Some (CheckoutState { phase = 1 }) else None

  rollback :: CheckoutState -> Option CheckoutState
  rollback s = if s.phase == 1 then Some (CheckoutState { phase = 0 }) else None

  machine DriftedCheckout where
    state = CheckoutState
    init = CheckoutState { phase = 0 }
    actions = [settle, rollback]

  project :: CheckoutState -> OrderSpecState
  project s = OrderSpecState { paid = s.phase == 1 }

  refines DriftedCheckout OrderSpec where
    view = project

  main :: Int
  main = 0
```

The compiler catches the drift immediately and names the exact offending action (`--[rollback]-->`) and the illegal abstract jump (`paid = True` back to `paid = False`).

## Catching infinite internal spinning with `measure`

If an internal retry step loops silently (`phase 1 -> phase 1`) without reducing `measure`, the compiler catches the livelock too:

<!-- cinch:reject contains="did not decrease `measure`" -->
```cinch
module Main where
  record OrderSpecState where
    paid :: Bool

  specPay :: OrderSpecState -> Option OrderSpecState
  specPay s = if not s.paid then Some (OrderSpecState { paid = True }) else None

  machine OrderSpec where
    state = OrderSpecState
    init = OrderSpecState { paid = False }
    actions = [specPay]

  record CheckoutState where
    phase :: Nat

  spinAuth :: CheckoutState -> Option CheckoutState
  spinAuth s = if s.phase == 0 then Some (CheckoutState { phase = 0 }) else None

  machine SpinningCheckout where
    state = CheckoutState
    init = CheckoutState { phase = 0 }
    actions = [spinAuth]

  project :: CheckoutState -> OrderSpecState
  project s = OrderSpecState { paid = s.phase == 1 }

  rank :: CheckoutState -> Nat
  rank s = if s.phase == 0 then 1 else 0

  refines SpinningCheckout OrderSpec where
    view = project
    measure = rank

  main :: Int
  main = 0
```

## Why public contracts forbid `next`

You might wonder why Cinch has `always`, `eventually`, `leads_to`, `recur`, and `stable`, but **forbids** a raw `next` ("true in the very next single step") operator in public contracts.

Because of refinement! If a public contract on `OrderSpec` could say *"in the very next step, `paid` is `True`"*, then any multi-step implementation like `CheckoutImpl` (which needs `authorize` and `capture` before `settle`) would violate the contract just by taking internal steps. By restricting public contracts to **stuttering-invariant** operators, every safety and liveness property verified on `OrderSpec` stays true for every valid refinement of `OrderSpec`.

Writing `next` in a contract is rejected directly:

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

# Connecting fairness to real concurrent code: `MVar` vs `STM`

State-machine fairness assumptions should match what your runtime actually guarantees. Cinch connects its temporal contract checker directly to its concurrency primitives:

- **`Effect.Concurrent.MVar`** uses a strict-FIFO ticketed runtime mutex and FIFO waiter queues for blocked takers, putters, and swaps. Because waiters are served in arrival order, Cinch discharges `eventually = (fair_handoff)` **automatically** on `MVar` functions—no hand-waved assumption needed.
- **`Effect.Stm`** uses optimistic transactions (`TVar`, `atomically`, `retry`). Under heavy contention, a long transaction can be repeatedly invalidated by shorter conflicting ones. So when you claim `eventually = (fair_handoff)` on an `STM` function, Cinch **refuses** it honestly unless you either switch to `MVar` or write an explicit `assume = "..."` that gets recorded in the build manifest.

Here is `MVar` discharging `fair_handoff` automatically:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Effect.Concurrent.MVar

  handoff :: MVar Nat -> Nat ->{MVar, Diverge} Nat
  handoff slot x = do
    _ <- mvar-put slot x
    mvar-take slot

  contract liveness mvarFair for handoff where
    eventually = (fair_handoff)

  main :: Int
  main = 0
```

Trying the same unassumed claim on `Effect.Stm` is rejected with a clear explanation:

<!-- cinch:reject contains="does not guarantee fairness" -->
```cinch
module Main where
  import Effect.Stm

  readCounter :: TVar Nat ->{STM, Diverge} Nat
  readCounter v = atomically (tvar-read v)

  contract liveness stmFair for readCounter where
    eventually = (fair_handoff)

  main :: Int
  main = 0
```

If your application ensures low contention externally and you intentionally want to record that assumption in the manifest, add `assume`:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Effect.Stm

  readCounter :: TVar Nat ->{STM, Diverge} Nat
  readCounter v = atomically (tvar-read v)

  contract liveness stmFair for readCounter where
    eventually = (fair_handoff)
    assume = "low-contention-workload"

  main :: Int
  main = 0
```

# Compositional infinite streams with clocked `Later k`

In [chapter 15](15-streams-and-producers.md), you wrote productive infinite streams using `corec`, where every recursive call had to appear syntactically inside `delay (\() -> ...)` in the same function. That works for simple generators, but breaks down as soon as you try to build streams out of reusable combinators.

Cinch also supports **clocked `Later k a`**, where `k :: Region` is a logical clock ("one step of time on stream clock `k`"). Four built-in combinators give you compositional guarded recursion without syntactic `corec`:

| Combinator | Type | What it does |
| --- | --- | --- |
| `laterPure` | `forall k a. a -> Later k a` | Lifts a value you already have now into the next step of clock `k`. |
| `laterAp` | `forall k a b. Later k (a -> b) -> Later k a -> Later k b` | Combines two delayed values that share the **same** clock `k`. |
| `laterFix` | `forall k a. (Later k a -> a) -> a` | Builds a productive recursive value given a delayed handle to itself (Löb induction). |
| `forceClock` | `forall a. (forall k. Later k a) -> a` | Safely extracts a value when it is polymorphic over any clock `k` (meaning `k` cannot escape in `a`). |

Here is an infinite stream of `1`s built with `laterFix`, combined with `laterAp`, and evaluated cleanly at runtime:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  codata CStream (k :: Region) (a :: Type) where
    head :: a
    tail :: Later k (CStream k a)

  ones :: forall k. Unit -> CStream k Nat
  ones u = laterFix (\self -> CStream { head = 1, tail = self })

  addLater :: forall k. Later k Nat -> Later k Nat -> Later k Nat
  addLater x y = laterAp (laterAp (laterPure (\a -> \b -> a + b)) x) y

  runClosed :: (forall k. Later k Nat) -> Nat
  runClosed lx = forceClock lx

  main :: Int
  main =
    let s = ones () in
    let v = runClosed (laterPure (40 + s.head)) in
    if v == 41 then 0 else 1
```

Clocks prevent subtle synchronization bugs at compile time. Mixing values from two unrelated clocks `k1` and `k2` in `laterAp` is a type error:

<!-- cinch:reject contains="regions do not match" -->
```cinch
module Main where
  badMix :: forall k1 k2. Later k1 (Nat -> Nat) -> Later k2 Nat -> Later k1 Nat
  badMix lf lx = laterAp lf lx

  main :: Int
  main = 0
```

And calling `forceClock` on a `Later k a` whose clock `k` is fixed in an outer scope (not universally quantified as `forall k. Later k a`) is rejected too, because eliminating a clock prematurely would break guardedness:

<!-- cinch:reject contains="forceClock requires a clock-polymorphic" -->
```cinch
module Main where
  badForce :: forall k. Later k Nat -> Nat
  badForce lx = forceClock lx

  main :: Int
  main = 0
```

See [explicit laziness, memoization, and codata](../reference/11-laziness-and-codata.md#clocked-later-k-and-compositional-guarded-recursion) for the full clocked `Later` rules.

# What the compiler enforces

- Every `machine` explores its reachable state graph from `init` up to `max_states` (default `4096`) and refuses silently pretending an unbounded space was checked if the state cap is exceeded.
- `always` and `deadlock_free` violations always report a shortest breadth-first trace from `init`.
- `eventually`, `leads_to`, `recur`, and `stable` check fair cycles under the exact `fair` (weak fairness) and `strong_fair` (strong fairness) actions you declared.
- Parameterized actions auto-enumerate finite domains (`Bool`, nullary enums, `Fin n`, `InRange lo hi Nat`).
- `symmetry = [T]` requires that `T` has no `Ord` instance in scope; static field-footprint Partial Order Reduction runs automatically.
- `refines Concrete Abstract` checks initial-state equality, stuttering simulation at every reachable concrete step, and (when `measure` is present) strict descent on every stuttering step.
- Raw `next` is forbidden in public contracts to preserve stuttering invariance across refinements.
- `eventually = (fair_handoff)` discharges automatically on `Effect.Concurrent.MVar` and refuses unassumed `Effect.Stm`.
- Clocked `Later k` prevents cross-clock mixing and premature clock elimination at typecheck time.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| An action breaking an `always` invariant | `temporal safety violation: contract ... falsified` followed by the shortest `State 0 (init) --[action]--> ...` trace |
| A state where every action returns `None` under `deadlock_free` | `deadlock` with the shortest trace to the stuck state |
| A liveness claim with a starvation cycle | `temporal liveness violation` with the prefix and `fair cycle` lasso |
| `symmetry = [Node]` when `Ord Node` is in scope | `unsound symmetry reduction` mentioning `Ord` |
| A concrete action that makes a transition forbidden by the abstract spec | `refinement violation` naming the concrete machine, abstract machine, and offending action |
| A stuttering step that does not lower `measure` | `refinement liveness violation` (`did not decrease \`measure\``) |
| `next = (...)` in a public contract | `` `next` is forbidden `` (`stuttering-invariant`) |
| `eventually = (fair_handoff)` on `Effect.Stm` without `assume` | `Effect.Stm` `does not guarantee fairness` (suggesting `MVar` or an explicit `assume`) |
| `laterAp` combining `Later k1` and `Later k2` | `regions do not match` |
| `forceClock` on a non-polymorphic `Later k a` | `forceClock requires a clock-polymorphic` |

# Exercises

**Exercise 1.** In the `FlakyLink` example, why did `fair = [deliver]` fail while `strong_fair = [deliver]` succeeded? What real-world bug does that difference represent?

<details><summary>Answer</summary>

`fair` (weak fairness) only requires an action to fire if it stays **continuously** enabled without interruption. In `FlakyLink`, `flap` toggles `linkUp` back to `False` on every second step, disabling `deliver` over and over again. A weakly fair schedule is allowed to miss every brief `linkUp = True` window forever. `strong_fair` requires an action that becomes enabled **infinitely often** to eventually fire, eliminating the `flap` loop. In production, that difference is the exact gap between a system where work is always ready once queued (like an uncontended local worker) and a system where readiness flickers under contention or flaky connections (like a non-FIFO spinlock or an optimistic retry loop that can starve indefinitely).

</details>

**Exercise 2.** Suppose `CheckoutImpl` adds a `logAttempt` action that increments an `attempts` counter (capped at `2`) while staying in `phase = 1` (`Authorizing`). Does `CheckoutImpl` still refine `OrderSpec` with `project s = OrderSpecState { paid = s.phase == 3 }`? What happens to `measure`?

<details><summary>Answer</summary>

For safety refinement (`view = project` alone), `logAttempt` is a valid stuttering step because `project` still returns `OrderSpecState { paid = False }` before and after `logAttempt`. For livelock freedom (`measure`), `stepsUntilAbstractMove` must also strictly decrease on `logAttempt`—for example, by including `(2 - s.attempts)` in the rank function—so the compiler can verify that the machine cannot log attempts forever without eventually advancing to `capture` and `settle`.

</details>

**Exercise 3.** Why does Cinch refuse `symmetry = [Node]` as soon as `Ord Node` is in scope, even if your current `acquire` function does not call `<`?

<details><summary>Answer</summary>

Symmetry reduction collapses permutations of `Node` values (such as swapping `N0` and `N1`) on the assumption that no operation on `Node` can distinguish one label from another except by equality. Once `Ord Node` exists in scope, functions and library helpers (including ordered maps or tie-breaking checks) can legally branch on `N0 < N1`, making the collapsed states behave differently and causing the model checker to miss real counterexamples. Enforcing absence of `Ord` at the type level keeps symmetry reduction sound without asking the programmer to audit every callee by hand.

</details>

Next: read the exact rules in [runtime upgrades and temporal contracts](../reference/17-upgrades-and-temporal-contracts.md), or put the whole language together in the [order calculator](22-putting-it-together.md).
