# Structured concurrency and runtime operation

The native runtime supplies threads, structured scopes, cooperative cancellation, I/O, and resource accounting. A task is a value with a declared lifetime, so the rules that hold in checked source also hold across the generated code and the runtime calls.

# Scopes, tasks, and joins

Three builtin values carry the surface, and the checker installs their schemes rather than declaring them in Prelude:

```text
withScope       :: forall r s a e. using (io :: Io r) => Scope r s ->{|e} a ->{Diverge | e} a
spawn           :: forall r s x a eb.
                   Scope r s -> (Unit ->{eb} Result x a) ->{Async r | eb} Task s x a
joinAllOrCancel :: Scope r s
                   -> Task s x v %1 -> Task s x w %1
                   ->{Async r, Cancel r, Diverge} Result x (v, w)
```

`Scope r s` names the runtime identity `r` and the scope region `s`, and `Task s x a` names the scope region, the body's error type, and the body's payload type. A task body is `Unit ->{eb} Result x a`: it returns a `Result`, so a failing task reports through the same channel as a successful one.

The scope argument arrives through a callback. `withScope (\s -> ...)` hands you the scope token, `spawn s (\() -> body)` starts a task and yields its handle, and `joinAllOrCancel s h1 h2` consumes handles and returns their payloads in handle order:

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

The join is n-ary for n of one or more. `joinAllOrCancel s a b c` returns a `Result x (Nat, Nat, Nat)` and consumes three handles, and `joinAllOrCancel s h` returns the payload itself rather than a one-tuple. A join group is the join: the handles it takes are the handles it consumes, and no separate wait operation exists.

The declared rows in that program are the whole cost of using the mechanism. `Async r` is added by `spawn`, `Cancel r` and `Diverge` by the join, and `Diverge` by `withScope`, because cleanup may not complete against an unresponsive child. Neither label implies parallelism, fairness, or a latency bound: `Async r` says work may overlap, not that it runs on another core, and `Diverge` says the group is not promised to finish.

## Scope rules

| Rule | What it means | Refusal |
| --- | --- | --- |
| A handle is linear | It is consumed exactly once, by a join | ``a task handle must be consumed exactly once by its function: this argument position is %many, so dropping it would silently detach execution (declare the parameter `%1`)`` |
| Every handle is joined | Binding a handle and never joining it is a mistake, not a detach | `task handle h must be joined exactly once; dropping it would silently detach execution (concurrency specification, section 2), but it is never used` |
| One join per handle | Joining the same handle twice duplicates it | `task handle h must be joined exactly once; ... but it is used 2 time(s)` |
| A handle cannot outlive its scope | The result of `withScope` must not mention the scope region | ``region escape: a task handle or scope of region r#14 leaves its `withScope` in Task r#14 ?m2015 ?m2016`` |
| A scope region is rigid per call | Two scopes never unify, so an outer handle cannot be joined inside an inner scope | `regions do not match: r#14 vs r#18` |
| Unjoined children are cancelled and joined at scope exit | Defense in depth behind the usage rule above | n/a |
| Task bodies use one-shot frames | `%1` and `%aff` values transfer exactly once, including on a cancellation early return | `used 2 time(s)`, `must be used exactly once` |

Linearity follows from the type: any binder whose type mentions `Task` is `%1`, and an application position that takes a task-mentioning parameter without `%1` is refused. That is why a helper that forwards a handle must declare its parameter `%1`, and why a helper that hides a scope token in a closure is refused rather than silently capturing it.

# Awaiting, cancellation, and acknowledgment

Cancellation is a cooperative request observed at defined points. `pollCancel ()` is the explicit one:

```text
ioSleep    :: forall r. using (io :: Io r) => Nat ->{IO r, Cancel r} Unit
pollCancel :: forall r. using (io :: Io r) => Unit ->{Cancel r} Bool
```

Both need the same `Io r` capability as any other runtime operation, and both are labelled with `Cancel r`, so a function that polls has to declare the label. `ioSleep` takes milliseconds and returns early when a cancellation is observed. On the program's own thread `pollCancel ()` answers `False`, so a body that only ever runs as `main` behaves as if cancellation never arrived.

A cancellation request and its acknowledgment are different events. The request comes from the join when a sibling has failed. The acknowledgment is the cancelled task finishing its own cleanup and returning. `joinAllOrCancel` waits for every sibling's cleanup before it returns the first failure, so the program below writes `C` from the cancelled task before the process exits with the failure code path:

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

The first task returns `Err (LoadError 2)`. The join requests cancellation of `spin`, `spin` observes it at `pollCancel ()`, writes one byte, and returns `Ok 0`. Only then does the join return `Err (LoadError 2)`, and `main` maps that to exit status 0. The byte count is 67 in decimal, `C` in ASCII. A task that never reaches a cancellation point delays the acknowledgment, and nothing in the type system rules that out: that possibility is exactly what `Diverge` on the join records.

## Threads, blocking calls, and the cooperative profile

Every task is a green thread owned by a structured scope: it has a stack of its own, and the runtime multiplexes tasks over a bounded set of OS threads (`CINCH_THREADS`, by default the number of processors), the program's own thread among them. A task that waits, for a join or in `ioSleep`, is suspended and its thread runs another task, so a program limited to one OS thread still runs all of its tasks. `spawn` and join are the memory model's fork and join points: everything before a `spawn` happens before the task body, and everything the body does happens before the join that observes its completion. The cancellation token is a sequentially consistent atomic, so a request becomes visible at the next poll point without a data race, and it is the only atomic a program can observe: the scheduler is built from atomics internally, but none of them is reachable from source, and they add happens-before edges rather than observable behaviours. Tasks are not preempted: `pollCancel` and `ioSleep` are also where a task lets the tasks queued behind it run.

A `foreign import` whose row carries `Blocking` is the one exception to suspension being free, and the reason is that the runtime cannot suspend inside foreign code: it has no point at which the call's own frames could be saved and restored, and it does not know whether the callee holds a lock, a stack reference, or thread-affine state. So the call really does hold the OS thread under it for as long as it runs. The offload contract is kept around that fact rather than by defeating it: the compiler brackets the call as a blocking region, the runtime counts the context as blocked for the duration and allows one more thread, and a fiber that is runnable with no thread to take it starts that thread. The call therefore blocks only its own task, and it is cancellation-blind until it returns. The label folds into the spawning scope's row, so a program that makes such a call inside a task has to declare `Blocking` at `main` as well, and dropping it is an effect honesty failure.

A task is not pinned to a thread. It runs on whichever scheduler thread takes its fiber from the run queue, and after a suspension it may continue on a different one, so no program may assume that two operations in one task body happen on the same OS thread. What the language does promise is the memory model above: the fork and join edges order the work, and the capture rules keep a value reachable from one task at a time, or shared only where sharing is safe.

The cooperative profile used for Wasm artifacts preserves the same source-visible observations: the same poll points, the same join-and-acknowledge protocol, the same ownership rules. Code written against native threading primitives does not become portable by passing through that profile, and the presence of `Async r` in a signature does not promise an OS thread on every target.

# Capabilities and identity

`Async r` and `Cancel r` carry the same runtime identity `r` as the `Io r` capability of the scope that created the task, so a signature quantifies one `r` and declares all three:

```cinch
work :: forall r. using (io :: Io r) => Unit ->{Async r, IO r, Cancel r, Diverge} Result LoadError Nat
```

Two consequences follow, and both are checked rather than documented only:

| Property | How it shows up |
| --- | --- |
| One identity per runtime | `withScope` requires the `Io r` capability in its `using` context, and a program with no provider is refused with ``no Io r#844 capability in scope; supply one lexically with `with (io = ...) in` or `f @{io = ...}`, or take it in a `using` clause`` |
| Scope identity is rigid per call | Joining an outer handle inside an inner scope is refused with `regions do not match: r#14 vs r#18` |

The effectful entry writes the same labels without an explicit `forall`: `main :: Unit ->{Async r, IO r, Cancel r, Diverge} Int`. An unbound lowercase name in a signature is implicitly quantified, and the checked interface records the binder explicitly as `r :: Region`. The identity is discharged by the `with (io = newIO ()) in` body, which constructs the runtime identity the program runs under.

# Ownership across a task boundary

A task body runs concurrently with the code that spawned it, on whichever scheduler thread takes its fiber, so a capture has to be safe to share or has to be transferred. The checker classifies each capture:

| Capture | Accepted when | Refusal |
| --- | --- | --- |
| `%1` value consumed once by the body | The transfer is visible at the capture | `used 2 time(s)` for two tasks capturing the same value |
| `Copy` value | Always | n/a |
| Shared `Ref` borrow | The referent is readable for the scope | n/a |
| Affine thunk | Moved into the task by transfer | n/a |
| `Bytes` buffer | Only with shareable evidence, since it is mutable | ``a task capture must be `Copy`, a shared `Ref`, or transferred exactly once: b is shared across tasks without shareable evidence`` |
| Exclusive `Mut` borrow | Never | ``an exclusive `Mut` borrow cannot cross a task boundary: m holds one`` |
| `Lazy` memo cell | Never, since the cell is single-threaded by construction | ``a thread-shared memo cell cannot cross a task boundary: y holds a `Lazy` cell, which is single-threaded by construction (a shared memo needs an explicit synchronized structure)`` |

The classifications survive aliasing: a callback stored under a second name, or wrapped in a source-level forwarder, is still checked against the values it actually captures, and a large immutable value that happens to sit beside a buffer does not launder the buffer.

# Refusals

Three refusals cover the mistakes that matter most. A handle that is dropped reaches a sink whose parameter is not `%1`:

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

A handle returned past the scope's lifetime is a region escape:

<!-- cinch:reject contains="leaves its" -->
```cinch
module Main where
  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad u = withScope $ \s -> spawn s (\() -> Ok 1)

  main :: Int
  main = 0
```

A handle bound and then ignored is refused by its usage obligation, with the message quoted in the scope rules table. All three are check-time refusals: the program never reaches the runtime, so no thread is started and no work is detached.

# Entry point and exit status

`main` is the only entry point, and it has exactly two admitted shapes: `main :: Int` for a pure program, and `main :: Unit ->{e} Int` for an effectful one. It takes no other parameters and no `using` context.

| Written | Diagnostic |
| --- | --- |
| `main :: Unit -> Unit -> Int` | `main must have type Int or Unit ->{e} Int in the bootstrap slice, got (Unit -> (Unit -> Int))` |
| `main :: forall r. using (io :: Io r) => Int` | `main must have type Int or Unit ->{e} Int in the bootstrap slice` |

The exit status is the low eight bits of the returned `Int`, because the generated C entry truncates the word. A `main` returning 300 leaves 44, one returning `-1` leaves 255, and one returning 256 leaves 0.

Four statuses belong to the runtime rather than to the program:

| Status | Meaning | stderr line |
| --- | --- | --- |
| 70 | `BlackHole` fault: a memo cell demanded while it is being evaluated, on a path that crossed a trust boundary | none; the generated `__cinch_blackhole` calls `exit(70)` directly |
| 71 | generic fault path: `cinch_panic`, or a compiler-bug trap | the fault's message, and a trap's message starts `compiler bug:` |
| 73 | `Exhausted`: one of heap, stack, or threads | `cinch: exhausted: heap (<n> bytes)`, `cinch: exhausted: stack`, or `cinch: exhausted: threads` |
| 74 | failed leak check | `cinch-leak: live=<n> immortal=<n> leaked=<n>` with `leaked` not zero |

A program's own result can equal any of those numbers, so the stderr line is what distinguishes an outcome from a result. Script `cinch run` against the line, not only the status, and use `cinch check` when you need to separate a compile failure (status 1 from the compiler) from an application result.

`cinch run Main.cinch word word` forwards every word after the entry file to the program's own command line, which `process-args` from [Effect.Process](../library/Effect.Process.md) reads as `forall r. using (io :: Io r) => Unit ->{IO r} Result ProcessError (List String)`. The list starts after the program name:

```text
$ cinch run Main.cinch alpha beta
alpha
```

# Environment variables

The runtime reads these at program start, and the compiler reads the build-time ones. `cinch run` passes its own environment through, and a built executable reads them directly.

| Variable | Value | Read | Observed effect |
| --- | --- | --- | --- |
| `CINCH_HEAP_LIMIT` | a byte count with an optional binary suffix, `<n>`, `<n>K`, `<n>M`, or `<n>G`; `0` disables the cap | program start | Caps the live heap, default 8 GiB. `CINCH_HEAP_LIMIT=1M` on a program allocating one 100 MB buffer prints `cinch: exhausted: heap (100000194 bytes)` and exits 73; `CINCH_HEAP_LIMIT=0` lets the same allocation succeed |
| `CINCH_STACK_SIZE` | the same count syntax; unparsable or zero keeps the default, and a value under 256 KiB is raised to it | program start | Sizes the reserved stack of the thread that runs program code, default 1 GiB. `CINCH_STACK_SIZE=256K` on a million-deep recursion prints `cinch: exhausted: stack` and exits 73; `CINCH_STACK_SIZE=abc` keeps the default and the same program exits 64 |
| `CINCH_THREADS` | a positive decimal count, at most 256 | program start | The thread budget: the OS threads that run tasks, the program's own thread included, default the available parallelism. `CINCH_THREADS=1` runs every task on the program's thread, suspending and resuming them there. It is also the parallel pool's participant budget when the program never called `parSetWorkers`, where `1` runs every parallel entry sequentially on the calling thread |
| `CINCH_LEAK_CHECK` | `1` | program start | Prints `cinch-leak: live=0 immortal=0 leaked=0` at exit for a clean program and keeps the program's status; a nonzero `leaked` makes it 74 |
| `CINCH_ALLOC_STATS` | `1` | program start | Prints one `cinch-alloc:` line at exit, for example `cinch-alloc: blocks=6 bytes=282 peak=218 header-blocks=0 header-bytes=0 immortal=0 freed=0 live=0` |
| `CINCH_RC_DEBUG` | `1` | program start | Poisons and quarantines freed blocks and validates every header the count entry points touch; a correct program still produces its own output and status |
| `CINCH_RECLAIM` | `0` | compile time | Builds the leak-everything baseline: the allocating program above reports `freed=11 live=0` with reclamation on and `freed=0 live=11` with `CINCH_RECLAIM=0` |
| `CINCH_CLANG` | a path or command name | compile time | The C compiler the driver invokes, default `clang` on `PATH` |
| `CINCH_STAGE_MODE` | `refresh`, `replay`, or `live`; default `refresh` | compile time | The compile-time I/O mode for staging effects; an unrecognised value falls back to `refresh` |
| `CINCH_STORE` | a directory | compile time | Root of the package store, default the platform's per-user store |
| `CINCH_REGISTRY`, `CINCH_REGISTRY_TOKEN` | a URL, and an upload token | `cinch pkg` commands | The registry a package operation talks to, and the credential `publish` needs |

The two lines a program's exit can print are its own observable trace of resource use, which is why they are worth running under a test. This program builds a three-cell list and discards it:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let xs = [1, 2, 3]
    in case xs of
         _ -> 0
```

```text
$ CINCH_LEAK_CHECK=1 CINCH_ALLOC_STATS=1 ./example
cinch-leak: live=0 immortal=0 leaked=0
cinch-alloc: blocks=9 bytes=284 peak=196 header-blocks=3 header-bytes=72 immortal=0 freed=3 live=0
```

`header-blocks=3` is one counted header per list cell at 24 bytes each, and `freed=3` matches them, so the cells were released before `main` returned. `live=0` in both lines is the same fact read by two switches: one counts what is still held, the other scans for leaks.

# Runtime I/O

Four builtin values provide the runtime's own I/O, and each is labelled:

| Value | Scheme |
| --- | --- |
| `ioWrite` | `forall r. using (io :: Io r) => Bytes ->{IO r} Unit` |
| `ioRead` | `forall r. using (io :: Io r) => Nat ->{IO r, Diverge} Option Bytes` |
| `ioSleep` | `forall r. using (io :: Io r) => Nat ->{IO r, Cancel r} Unit`, taking milliseconds |
| `pollCancel` | `forall r. using (io :: Io r) => Unit ->{Cancel r} Bool` |

`ioWrite` writes `Bytes` to stdout and flushes, so `string-to-bytes` converts a `String` first. `ioRead` asks for a count and answers `None` at end of input, so a caller handles the absence. [Compiler-provided vocabulary](20-compiler-vocabulary.md) lists the names; each one is installed by the checker rather than declared as a Prelude function, and none of them can be captured as a plain value to escape its row.

Everything else lives in checked library modules: `Effect.Fs` for files, `Effect.Clock` for time, `Effect.Process` for subprocesses and the command line, `Effect.Log` and `Effect.Trace` for observability, and the networking, HTTP, resolution, signals, pseudo-terminal, and file-utility modules beside them. Their APIs return named `Result` errors where failure is expected, and the shipped HTTP client is plain HTTP and refuses HTTPS rather than falling back to it. See [the module reference](../library/00-index.md).

# Software transactional memory

`Effect.Stm` is shared mutable state that composes. A `TVar a` is a shared variable, an `STM a` describes reads and writes of such variables, and `atomically` runs one description so that all of it takes effect at one instant or none of it does. No other transaction sees some of its writes without the rest.

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

| Operation | Type | Meaning |
| --- | --- | --- |
| `atomically` | `STM a ->{STM, Diverge} a` | Run the transaction; run it again if another commit got in the way |
| `retry` | `STM a` | Abandon the attempt and wait until a variable it read is written |
| `or-else` | `STM a -> STM a -> STM a` | Run the second when the first would wait; the first's writes are discarded |
| `check` | `Bool -> STM Unit` | `retry` unless the condition holds |
| `tvar-new`, `tvar-read`, `tvar-write` | `a -> STM (TVar a)`, `TVar a -> STM a`, `TVar a -> a -> STM Unit` | The variable |
| `tvar-modify`, `tvar-swap`, `tvar-state` | `TVar a -> (a -> a) -> STM Unit`, `TVar a -> a -> STM a`, `TVar a -> (a -> (b, a)) -> STM b` | Read and write in one step |
| `tvar-new-io`, `tvar-read-io` | `a ->{STM} TVar a`, `TVar a ->{STM} a` | One-operation transactions outside `atomically` |
| `atomically-or-cancel` | `using (io :: Io r) => STM a ->{STM, Cancel r, Diverge} Option a` | As `atomically`, answering `None` when the task is cancelled while it waits |

`do` in a definition with a `using (seq :: StmSequencing)` binder sequences transactions, and `STM` is also a `Functor`, `Applicative`, `Monad` and `Alternative` (`empty` is `retry`, `<|>` is `or-else`). `stm-pure`, `stm-bind`, `stm-map` and `stm-then` are the same operations as plain functions. `Data.Option` exports an `or-else` of its own, so a module that imports both whole leaves one out: `import Data.Option hiding (or-else)`.

`retry` is how a transaction blocks. A producer and a consumer hand values through a `TMVar` (`Effect.Stm.TMVar`), each waiting while the slot is in the wrong state:

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

The structures built on the variable are ordinary library modules:

| Module | Structure |
| --- | --- |
| `Effect.Stm.TMVar` | A variable that is empty or full; `tmvar-take` waits while empty, `tmvar-put` while full |
| `Effect.Stm.TQueue` | An unbounded first-in first-out queue |
| `Effect.Stm.TBQueue` | A bounded queue; a write waits at capacity |
| `Effect.Stm.TSem` | A counting semaphore |

## What a transaction guarantees

| Guarantee | Statement |
| --- | --- |
| Atomicity and isolation | Committed transactions are serializable, in the order of their commits |
| Opacity | An attempt that will not commit still reads only a snapshot some serial order could have produced, so a body cannot loop or fault on values that never existed together |
| No lost wakeup | A transaction waiting in `retry` runs again after any commit that writes a variable it read |

There is no fairness and no bound on attempts. A long transaction can be overtaken indefinitely by short ones that write what it reads, and a `retry` whose variables nobody writes waits forever, which is why `atomically` carries `Diverge`. A transaction may run many times: what an abandoned attempt allocated is released, and nothing an attempt did is visible unless it commits. An effect on the outside world belongs after the commit, under a protocol the application chooses.

## What a transaction may not do

A transaction's steps are pure functions, so I/O inside one has nowhere to put its effect row:

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

A step may run once per attempt, so a value that must be used exactly once cannot be carried across one:

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

A `TVar` handle is `Copy`, and a variable is reached from every task that holds it. Capturing one in a task therefore shares its value, and the value's type has to allow that:

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

The algorithm is versioned write locks with a global version clock (TL2), written in Cinch over inline target IR; the runtime supplies only the cell, the clock and the wait. [The STM design note](../design/stm.md) has the algorithm, the two rules that keep reference counts right when a value is replaced under readers, and the per-architecture instructions.

# Exhaustion and defenses

Heap, stack, and thread exhaustion are one outcome: exit 73 and exactly one stderr line naming the resource. They are not ordinary return values, and no program can observe them as data. The heap line reports the bytes the program asked to hold, which is the live heap past the cap rather than the cap itself.

The internal black-hole path and the impossible-boundary traps remain defenses for compiler and runtime invariant failures. Their presence is not permission for a safe API to return an arbitrary fallback word. A green test run does not show that a program is free of them; `CINCH_LEAK_CHECK=1` and `CINCH_ALLOC_STATS=1` are the diagnostics that do show what the program allocated, freed, and left live.

# Coming soon

<span class="support-tag support-tag--planned">Coming soon</span> **Bounded channels and the fuller concurrency library.** The concurrency specification describes explicit queue capacity and backpressure, ownership transfer at a send commit point, cancellation-safe blocked sends and receives, and `select` or `race` combinators that dispose of losing registrations. There is no shipped `Channel` type, send, receive, or `select` in the current library, so the import fails with ``module Effect.Channel is imported, but Effect.Channel.cinch does not exist and no other source file beside the entry declares `module Effect.Channel` ``. Task spawning and joining do not imply them.

Long-lived supervised services, deterministic scheduler control, cancellation injection, and replayable schedule exploration remain design targets beyond the current structured-task API: no module in the library mentions a supervisor, a restart policy, or a scheduler, and no task API accepts a schedule. A scheduling test, when one exists, states its exploration bound.

<span class="support-tag support-tag--planned">Coming soon</span> **End-user runtime supervision.** Runtime upgrade and JIT mechanisms and the target APIs do not yet supply a complete service launcher, update supervisor, or cross-platform runtime packaging tool: `cinch --help` lists `check`, `emit-ir`, `emit-object`, `build`, `run`, `lsp`, `doc`, `repl`, `jit`, and `pkg`, and nothing that starts or supervises a service. Use the documented native CLI and the checked library contracts today.

# Implementation

The task and scope typing is in `check_decls.rs` (`install_concurrency_api`) and `check_terms.rs` (capture, scope, and join elaboration); the runtime half is `compiler/runtime/src/task.rs`, `sched.rs` with `sched/fiber.rs`, `sched/switch.rs` and `sched/thread.rs`, `park.rs`, `platform.rs`, `heap.rs`, and `stack.rs`. The code generator brackets a `Blocking` foreign import in `codegen.rs` (`foreign_call`). Transactions are `compiler/prelude/Effect/Stm.cinch` and the modules under `Effect/Stm/`, over `compiler/src/stm.rs` and `compiler/runtime/src/stm.rs` and `park.rs`. Suites: `concurrency.rs`, `reclamation_tasks.rs`, `stm.rs`, `exhaustion.rs`, `platform.rs`, `argv.rs`, and the scheduler's own `compiler/runtime/tests/sched.rs`. The normative text is [the concurrency specification](../specifications/concurrency-specification.md), amended by decisions D74 to D81 and D423 to D428. The scheduler's design and measurements are in [green threads](../design/green-threads.md). Tutorial: [structured concurrency](../learn/17-structured-concurrency.md).
