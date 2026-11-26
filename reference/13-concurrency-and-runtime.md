# Structured concurrency and runtime operation

Cinch's native runtime provides M:N green threads, lexically structured task scopes, cooperative cancellation, software transactional memory, runtime I/O, and resource exhaustion accounting. Tasks carry explicit scope lifetimes and linear handles, so detached execution, region escapes, and unsynchronized mutable sharing are rejected during typechecking before any code is generated.

# Scopes, tasks, and joins

Three compiler-installed operations define the structured concurrency surface:

```text
withScope       :: forall r s a e. using (io :: Io r) => (Scope r s ->{|e} a) ->{Diverge | e} a
spawn           :: forall r s x a eb.
                   Scope r s -> (Unit ->{eb} Result x a) ->{Async r | eb} Task s x a
joinAllOrCancel :: Scope r s
                   -> Task s x v %1 -> Task s x w %1
                   ->{Async r, Cancel r, Diverge} Result x (v, w)
```

`Scope r s` binds the runtime identity `r` and a fresh scope region `s`. `Task s x a` names the owning scope `s`, the failure type `x`, and the payload type `a`. Every task body returns `Result x a`, unifying normal completion and typed failure on one channel.

`withScope` supplies the scope token to a callback, `spawn` starts a task in that scope and returns its linear handle, and `joinAllOrCancel` consumes one or more handles and returns their results in handle order:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    a <- spawn s (\() -> Ok 20)
    b <- spawn s (\() -> Ok 2)
    joinAllOrCancel s a b

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> case p of
      (a, b) -> if a * 10 + b == 202 then 0 else 1
    Err e -> 2
```

`joinAllOrCancel` is n-ary for one or more handles. Joining a single handle returns `Result x a` directly rather than a 1-tuple; joining two or more handles returns `Result x (a_1, ..., a_n)`. There is no separate detached `wait` operation: a join group is defined by the handles passed to `joinAllOrCancel`.

Effect rows track concurrency obligations precisely:

- `Async r` — introduced by `spawn` and `joinAllOrCancel`; permits concurrent interleaving without promising multi-core parallelism, fairness, or latency bounds.
- `Cancel r` — introduced by `joinAllOrCancel`, `pollCancel`, and `ioSleep`; marks cooperative cancellation observation.
- `Diverge` — introduced by `withScope` and `joinAllOrCancel`; reflects that a scope cannot finish if a child never reaches a cancellation point.

## Scope and linearity rules

| Invariant | Meaning | Check-time refusal |
| --- | --- | --- |
| Task handles are `%1` | Any binder whose type mentions `Task` is linear; forwarding helpers must mark the parameter `%1`. | `a task handle must be consumed exactly once by its function: this argument position is %many` |
| Every handle is joined | Ignoring a bound handle is an error, never an implicit detach. | `task handle h must be joined exactly once ... but it is never used` |
| Joined at most once | Passing a handle to a join twice duplicates a linear value. | `task handle h must be joined exactly once ... but it is used 2 time(s)` |
| No scope escape | The return type of `withScope` cannot mention the scope region `s`. | ``region escape: a task handle or scope of region ... leaves its `withScope` `` |
| Rigid scope regions | Distinct `withScope` calls produce distinct regions; outer handles cannot be joined in an inner scope. | `regions do not match` |
| One-shot task frames | `%1` and `%aff` captures transfer into the task once, including on early cancellation return. | `used 2 time(s)` / `must be used exactly once` |
| Scope-exit cleanup | As runtime defense in depth, any unjoined children are cancelled and awaited before scope exit. | — |

# Cancellation and acknowledgment

Cancellation is cooperative and observed at explicit points:

```text
ioSleep    :: forall r. using (io :: Io r) => Nat ->{IO r, Cancel r} Unit
pollCancel :: forall r. using (io :: Io r) => Unit ->{Cancel r} Bool
```

Both require an `Io r` capability in scope and add `Cancel r` to the effect row. `ioSleep` sleeps for the given number of milliseconds and returns early if cancellation arrives. On the main task outside a cancelled join group, `pollCancel ()` always returns `False`.

Requesting cancellation and acknowledging it are distinct events. When any sibling in `joinAllOrCancel` returns `Err`, the join requests cancellation of every remaining sibling and **waits for every sibling to clean up and return** before returning the first failure. In the program below, `spin` observes cancellation at `pollCancel ()`, writes byte value `67` (ASCII `'C'`), and returns `Ok 0`; only then does `joinAllOrCancel` return `Err (LoadError 2)`:

<!-- cinch:run stdout="C" exit=0 -->
```cinch
module Main where
  fill :: InRange 0 255 Nat -> Bytes
  fill v = let b = bytes-new 1 in case bytes-set b 0 v of
    _ -> b

  putByte :: forall r. using (io :: Io r) => InRange 0 255 Nat ->{IO r} Unit
  putByte v = ioWrite (fill v)

  spin :: forall r. using (io :: Io r) => Unit ->{IO r, Cancel r} Result LoadError Nat
  spin u = case pollCancel () of
    True -> case putByte 67 of
      _ -> Ok 0
    False -> case ioSleep 5 of
      _ -> spin ()

  work :: forall r. using (io :: Io r) => Unit ->{Async r, IO r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Err (LoadError 2))
    h2 <- spawn s (\() -> spin ())
    joinAllOrCancel s h h2

  main :: Unit ->{Async r, IO r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> 1
    Err e -> case e of
      LoadError c -> if c == 2 then 0 else 3
```

A task that loops forever without calling a cancellation point delays scope completion indefinitely, which is why `joinAllOrCancel` and `withScope` carry `Diverge`.

## Scheduler, memory model, and blocking FFI

Every task runs as a green thread (fiber) with its own stack, multiplexed across a bounded pool of OS worker threads (including the program's main thread). The pool size defaults to available hardware parallelism and is configured at program startup with `CINCH_THREADS` (a positive integer up to `256`). With `CINCH_THREADS=1`, all tasks run cooperatively on the main thread, yielding at joins, `ioSleep`, `pollCancel`, and STM `retry`. `CINCH_THREADS` also sets the default worker budget for the data-parallel pool when `parSetWorkers` is not called.

- **Non-preemptive scheduling.** Tasks are not preempted asynchronously; suspension happens at joins, `ioSleep`, `pollCancel`, and STM waits.
- **No thread pinning.** A fiber may resume on a different OS thread after any suspension point. Never assume thread affinity inside a task body.
- **Happens-before guarantees.** `spawn` establishes a happens-before edge from the spawning task to the child body; joining establishes a happens-before edge from the child body to the caller. The cancellation flag is sequentially consistent, making requests visible at the next poll point without data races.
- **Blocking foreign calls.** A `foreign import` carrying `Blocking` cannot suspend mid-call because the runtime cannot capture foreign stack frames or thread-local state. Instead, the compiler brackets the call as a blocking region so the scheduler can temporarily admit an extra worker thread while the call occupies its OS thread. Blocking calls are cancellation-blind until they return, and `Blocking` propagates upward through the enclosing scope's effect row.
- **Cooperative WebAssembly profile.** On `wasm32`, the cooperative scheduler preserves the exact same poll points, join-and-acknowledge protocol, and ownership rules on a single thread.

# Capabilities and runtime identity

`Async r` and `Cancel r` share the runtime identity `r` of the `Io r` capability governing the enclosing `withScope`:

```cinch
work :: forall r. using (io :: Io r) => Unit ->{Async r, IO r, Cancel r, Diverge} Result LoadError Nat
```

Calling `withScope` without an `Io r` provider in scope is rejected (`no Io ... capability in scope; supply one lexically with with (io = ...) in or f @{io = ...}, or take it in a using clause`). At the entry point `main :: Unit ->{Async r, IO r, Cancel r, Diverge} Int`, the unbound `r` is implicitly quantified as `r :: Region` and discharged by `with (io = newIO ()) in`.

# Ownership across a task boundary

Because a spawned task runs concurrently with its parent and siblings, every captured value must be either safe to share or uniquely transferred:

| Captured value | Allowed? | Rule / diagnostic |
| --- | --- | --- |
| `Copy` value | Yes | Duplicated freely into the task. |
| Shared `Ref` borrow | Yes | Readable for the duration of the enclosing scope. |
| `%1` value | Once | Transferred into one task; capturing in two tasks fails with `used 2 time(s)`. |
| Affine (`%aff`) thunk | Once | Moved into the task by ownership transfer. |
| `Bytes` buffer | Only with shareable evidence | Mutable by default; rejected with `is shared across tasks without shareable evidence`. |
| Exclusive `Mut` borrow | Never | `an exclusive Mut borrow cannot cross a task boundary`. |
| `Lazy` memo cell | Never | Single-threaded by construction; `a thread-shared memo cell cannot cross a task boundary`. |

Capture analysis sees through aliasing and wrapper closures: storing a closure under another name or pairing a mutable buffer inside a larger immutable structure does not bypass the check.

## Common check-time refusals

Passing a task handle to a non-linear (`%many`) sink is rejected:

<!-- cinch:reject contains="consumed exactly once" -->
```cinch
module Main where
  ignore :: forall a. a -> Unit
  ignore x = ()

  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Unit
  bad u = withScope $ \s -> ignore (spawn s (\() -> Ok 1))

  main :: Int
  main = 0
```

Returning a handle out of its `withScope` is rejected as a region escape:

<!-- cinch:reject contains="leaves its" -->
```cinch
module Main where
  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad u = withScope $ \s -> spawn s (\() -> Ok 1)

  main :: Int
  main = 0
```

# Entry points, exit statuses, and runtime diagnostics

A standalone executable defines `main` with one of two signatures—no extra arguments and no `using` context:

- `main :: Int` for a pure program.
- `main :: Unit ->{e} Int` for an effectful program.

The process exit code is the low 8 bits of the returned `Int` (`300` becomes `44`, `-1` becomes `255`, `256` becomes `0`). Command-line arguments after `<file.cinch>` under `cinch run` are forwarded to `process-args` in [Effect.Process](../library/Effect.Process.md) (use `--` before arguments that look like compiler flags; see [Chapter 01](01-using-the-compiler.md#entry-points-and-exit-statuses)).

Four exit statuses are reserved for runtime outcomes rather than application return values:

| Status | Meaning | stderr diagnostic |
| --- | --- | --- |
| `70` | `BlackHole` fault (re-entrant demand on a memo cell across a trust boundary) or failed pattern guard | None for `__cinch_blackhole` (calls `exit(70)` directly). |
| `71` | Generic runtime fault (`cinch_panic`) or internal compiler-bug trap | Fault message (internal traps start with `compiler bug:`). |
| `73` | Resource exhaustion (`Exhausted`: heap, stack, or threads) | `cinch: exhausted: heap (<n> bytes)`, `cinch: exhausted: stack`, or `cinch: exhausted: threads`. |
| `74` | Failed leak check under `CINCH_LEAK_CHECK=1` | `cinch-leak: live=<n> immortal=<n> leaked=<n>` with `leaked > 0`. |

Because an application can return any 8-bit integer, scripts should check stderr alongside the exit code and run `cinch check` separately to distinguish compile failures (exit `1`) from program return values.

## Runtime tuning and allocation tracing

Beyond `CINCH_THREADS` (described above), runtime memory caps and leak/reclamation diagnostics (`CINCH_HEAP_LIMIT`, `CINCH_STACK_SIZE`, `CINCH_LEAK_CHECK`, `CINCH_ALLOC_STATS`, `CINCH_RC_DEBUG`, `CINCH_RECLAIM`) are documented in [Chapter 25](25-allocators.md#environment-variables), and build-time variables (`CINCH_CLANG`, `CINCH_STAGE_MODE`, `CINCH_STORE`, `CINCH_REGISTRY`) are listed in [Chapter 01](01-using-the-compiler.md#environment-variables).

For example, this program allocates a three-element list and drops it before returning:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let xs = [1, 2, 3]
    in case xs of
         _ -> 0
```

Running the compiled binary with leak and allocation reporting enabled shows all three list cells reclaimed (`header-blocks=3`, `freed=3`, `live=0`):

```text
$ CINCH_LEAK_CHECK=1 CINCH_ALLOC_STATS=1 ./example
cinch-leak: live=0 immortal=0 leaked=0
cinch-alloc: blocks=9 bytes=284 peak=196 header-blocks=3 header-bytes=72 immortal=0 freed=3 live=0
```

Building the same program with `CINCH_RECLAIM=0` (or `--no-reclaim`) omits reference-counting ops and reports `freed=0 live=3` (and exits `74` under `CINCH_LEAK_CHECK=1`).

Heap, stack, and thread exhaustion all exit `73` with a single stderr line naming the exhausted resource. The heap message reports the live bytes the refused allocation would have required, not the configured cap.

# Runtime I/O

Four compiler-installed builtins expose minimal runtime I/O:

| Builtin | Type | Behavior |
| --- | --- | --- |
| `ioWrite` | `forall r. using (io :: Io r) => Bytes ->{IO r} Unit` | Writes bytes to stdout and flushes (convert strings via `string-to-bytes`). |
| `ioRead` | `forall r. using (io :: Io r) => Nat ->{IO r, Diverge} Option Bytes` | Reads up to `n` bytes from stdin; returns `None` at EOF. |
| `ioSleep` | `forall r. using (io :: Io r) => Nat ->{IO r, Cancel r} Unit` | Sleeps for `n` milliseconds; wakes early on cancellation. |
| `pollCancel` | `forall r. using (io :: Io r) => Unit ->{Cancel r} Bool` | Returns `True` if cancellation has been requested for the current task. |

Higher-level OS interfaces live in checked standard-library modules (`Effect.Fs`, `Effect.Clock`, `Effect.Process`, `Effect.Log`, `Effect.Trace`, networking, HTTP, DNS, signals, and PTY modules); see the [library reference](../library/00-index.md).

# Software transactional memory

`Effect.Stm` provides composable shared mutable state via transactional variables (`TVar a`). An `STM a` value describes an atomic read/write transaction, and `atomically` executes it so all of its updates take effect together or none do.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Effect.Stm

  move :: using (seq :: StmSequencing) => TVar Nat -> TVar Nat -> Nat -> STM Unit
  move src dst n = do
    a <- tvar-read src
    if n <= a then do
        _ <- tvar-write src (a - n)
        tvar-modify dst (\b -> b + n)
      else retry

  main :: Unit ->{STM, Diverge} Int
  main u =
    let a = tvar-new-io (100 :: Nat) in
    let b = tvar-new-io (0 :: Nat) in
    case atomically (move a b 30) of
      _ -> if tvar-read-io a == 70 then (if tvar-read-io b == 30 then 0 else 2) else 1
```

| Operation | Type | Behavior |
| --- | --- | --- |
| `atomically` | `STM a ->{STM, Diverge} a` | Executes a transaction, retrying automatically on conflict. |
| `atomically-or-cancel` | `using (io :: Io r) => STM a ->{STM, Cancel r, Diverge} Option a` | Like `atomically`, returning `None` if cancelled while waiting in `retry`. |
| `retry` | `STM a` | Aborts the current attempt and blocks until any read `TVar` is modified. |
| `or-else` | `STM a -> STM a -> STM a` | Runs the first transaction; if it calls `retry`, discards its writes and runs the second. |
| `check` | `Bool -> STM Unit` | Calls `retry` when the condition is `False`. |
| `tvar-new`, `tvar-read`, `tvar-write` | `a -> STM (TVar a)`, `TVar a -> STM a`, `TVar a -> a -> STM Unit` | Create, read, and write a `TVar` inside a transaction. |
| `tvar-modify`, `tvar-swap`, `tvar-state` | `TVar a -> (a -> a) -> STM Unit`, `TVar a -> a -> STM a`, `TVar a -> (a -> (b, a)) -> STM b` | Atomic read-modify-write helpers. |
| `tvar-new-io`, `tvar-read-io` | `a ->{STM} TVar a`, `TVar a ->{STM} a` | Single-operation variable creation and read outside `atomically`. |

`do` notation inside `STM` uses `using (seq :: StmSequencing)`. `STM` also implements `Functor`, `Applicative`, `Monad`, and `Alternative` (`empty` is `retry`, `<|>` is `or-else`), with plain combinators `stm-pure`, `stm-bind`, `stm-map`, and `stm-then`. Because `Data.Option` also exports `or-else`, import it with `hiding (or-else)` when combining both modules.

Blocking synchronization builds naturally on `retry`. Here a producer and consumer exchange values through a `TMVar` (`Effect.Stm.TMVar`):

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Effect.Stm
  import Effect.Stm.TMVar
  import Data.Machine

  produce :: TMVar Nat -> I64 -> Nat ->{STM, Diverge} Nat
  produce slot n x = if n == 0 then 0 else case atomically (tmvar-put slot x) of
    _ -> produce slot (wrapping-sub-i64 n 1) (x + 1)

  consume :: TMVar Nat -> I64 -> Nat ->{STM, Diverge} Nat
  consume slot n acc = if n == 0 then acc else
    let x = atomically (tmvar-take slot) in consume slot (wrapping-sub-i64 n 1) (acc + x)

  work :: forall r. using (io :: Io r) => TMVar Nat ->{Async r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat)
  work slot = withScope $ \s -> do
    a <- spawn s (\() -> Ok (consume slot 1000 0))
    b <- spawn s (\() -> Ok (produce slot 1000 1))
    joinAllOrCancel s a b

  main :: Unit ->{Async r, Cancel r, Diverge, STM} Int
  main u = with (io = newIO ()) in
    let slot = tmvar-new-empty-io () in
    case work slot of
      Ok p -> (case p of
        (total, _) -> if total == 500500 then 0 else 1)
      Err _ -> 2
```

Standard transactional data structures ship as library modules over `TVar`:

| Module | Data structure |
| --- | --- |
| `Effect.Stm.TMVar` | Empty-or-full synchronized cell (`tmvar-take` blocks when empty, `tmvar-put` when full). |
| `Effect.Stm.TQueue` | Unbounded transactional FIFO queue. |
| `Effect.Stm.TBQueue` | Bounded transactional FIFO queue (writes block at capacity). |
| `Effect.Stm.TSem` | Counting transactional semaphore. |

## Transaction guarantees

| Property | Guarantee |
| --- | --- |
| **Serializability** | Committed transactions behave as if executed in a single global commit order. |
| **Opacity** | Even an attempt that eventually aborts sees a mutually consistent snapshot, preventing inconsistent-state loops or faults. |
| **No lost wakeups** | A transaction parked in `retry` wakes after any commit modifying a `TVar` in its read set. |

STM does not promise fairness or bounded retries: a long transaction can be repeatedly invalidated by short conflicting ones, and a `retry` whose read set is never written waits forever (hence `Diverge` on `atomically`). Memory allocated by an aborted attempt is reclaimed cleanly, and external I/O must happen after `atomically` commits.

The implementation uses versioned write locks with a global version clock (TL2) integrated with Cinch's reference-counting runtime.

## What a transaction may not do

**No arbitrary effects.** Transaction steps are pure, so performing `IO` inside `STM` is rejected:

<!-- cinch:reject contains="effect rows do not match" -->
```cinch
module Main where
  import Effect.Stm

  loud :: forall r. using (io :: Io r) => TVar Nat -> STM Nat
  loud t = stm-bind (tvar-read t) (\n -> case ioWrite (bytes-new 1) of
    _ -> stm-pure n)

  main :: Int
  main = 0
```

**No linear captures across retries.** Because a step may execute multiple times across retries, a `%1` value cannot be captured inside an `STM` step:

<!-- cinch:reject contains="multiplicity %1" -->
```cinch
module Main where
  import Effect.Stm

  bad :: using (seq :: StmSequencing) => TVar Nat -> Bytes %1 -> STM Nat
  bad t b = do
    a <- tvar-read t
    pure (a + bytes-len b)

  main :: Int
  main = 0
```

**Only shareable payloads across tasks.** Because `TVar` handles are `Copy`, capturing a `TVar` in a spawned task shares its element type across tasks, which requires shareable evidence (rejecting mutable `Bytes` without synchronization):

<!-- cinch:reject contains="shareable evidence" -->
```cinch
module Main where
  import Effect.Stm

  size :: TVar Bytes ->{STM, Diverge} Nat
  size t = bytes-len (atomically (tvar-read t))

  work :: forall r. using (io :: Io r) => TVar Bytes ->{Async r, Cancel r, Diverge, STM} Result LoadError (Nat, Nat)
  work t = withScope $ \s -> do
    a <- spawn s (\() -> Ok (size t))
    b <- spawn s (\() -> Ok (size t))
    joinAllOrCancel s a b

  main :: Int
  main = 0
```

# Fair `MVar` and temporal handoff guarantees

When concurrent tasks need starvation-free synchronization, prefer `Effect.Concurrent.MVar` over optimistic `Effect.Stm`. An `MVar a` is a synchronized single-slot cell backed by a **strict-FIFO ticketed runtime mutex** and **FIFO waiter queues** for blocked takers, putters, and swaps. Nonblocking calls (`mvar-try-take`, `mvar-try-put`, `mvar-try-read`) cannot steal a value or slot already reserved for a queued waiter, and cancellable variants (`-or-cancel`) preserve committed handoffs.

| Operation | Effect row | Behavior |
| --- | --- | --- |
| `mvar-new`, `mvar-new-empty` | `{MVar}` | Allocate a full or empty `MVar a`. |
| `mvar-take`, `mvar-put`, `mvar-read`, `mvar-swap` | `{MVar, Diverge}` | Blocking FIFO-served take, put, non-destructive read, and atomic swap. |
| `mvar-try-take`, `mvar-try-put`, `mvar-try-read`, `mvar-is-empty` | `{MVar}` | Nonblocking immediate operations; never jump ahead of queued waiters. |
| `mvar-take-or-cancel`, `mvar-put-or-cancel`, `mvar-read-or-cancel`, `mvar-swap-or-cancel` | `{MVar, Cancel r, Diverge}` | Cancellable blocking forms; unregister cleanly if cancelled before commit. |

Because `MVar` guarantees FIFO handoff fairness at runtime, a temporal contract claiming `eventually = (fair_handoff)` over an `MVar` function discharges automatically without an unverified assumption:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Effect.Concurrent.MVar

  exchange :: MVar Nat -> Nat ->{MVar, Diverge} Nat
  exchange slot x = do
    _ <- mvar-put slot x
    mvar-take slot

  contract liveness mvarFair for exchange where
    eventually = (fair_handoff)

  main :: Int
  main = 0
```

By contrast, `Effect.Stm` is optimistic and does not promise fairness under contention, so claiming `eventually = (fair_handoff)` on an `STM` function is rejected unless an explicit `assume` clause is recorded:

<!-- cinch:reject contains="does not guarantee fairness" -->
```cinch
module Main where
  import Effect.Stm

  bump :: TVar Nat ->{STM, Diverge} Nat
  bump v = atomically (tvar-read v)

  contract liveness stmFair for bump where
    eventually = (fair_handoff)

  main :: Int
  main = 0
```

See [Chapter 17](17-upgrades-and-temporal-contracts.md#concurrency-fairness-contracts-fair_handoff) and [state machines, fairness, and model checking](../learn/23-state-machines-and-model-checking.md#connecting-fairness-to-real-concurrent-code-mvar-vs-stm) for how runtime fairness connects to state-machine verification.

# Planned extensions

- <span class="support-tag support-tag--planned">Coming soon</span> **Bounded channels and `select`/`race`.** Typed bounded channels (`Effect.Channel`) with ownership transfer at send commit, cancellation-safe blocked operations, and multi-channel `select`/`race` are specified but not yet in the standard library.
- <span class="support-tag support-tag--planned">Coming soon</span> **Supervision and deterministic scheduling.** Long-lived supervision trees, restart policies, and deterministic schedule exploration / cancellation injection are planned beyond the current structured-task API.
- <span class="support-tag support-tag--planned">Coming soon</span> **Service launcher and runtime supervision CLI.** The CLI ([Chapter 01](01-using-the-compiler.md#commands)) covers building, running, and inspecting artifacts, but does not yet provide a service supervisor or rolling-update launcher.

For a guided walkthrough, see the [structured concurrency tutorial](../learn/17-structured-concurrency.md).
