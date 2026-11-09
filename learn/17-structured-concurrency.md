# 17. Structured tasks and cancellation

A task in Cinch is a child of a lexical scope. The scope owns the task, the scope's exit joins it, and a task handle is a linear value that you consume exactly once. There is no way to start work and walk away from it, because dropping a handle is a type error rather than a detached thread. The same machinery decides what happens when one child fails, and the join's policy answers that question.

| Name | Type | Effect row it performs |
| --- | --- | --- |
| `withScope` | `(Scope r s ->{e} a) ->{Diverge \| e} a` | `Diverge`, plus the body's row |
| `spawn` | `Scope r s -> (Unit ->{eb} Result x a) ->{Async r \| eb} Task s x a` | `Async r`, plus the body's row |
| `joinAllOrCancel` | `Scope r s -> Task s x v ->%1 ... ->%1 Result x (v, w, ...)` | `Async r`, `Cancel r`, `Diverge` |
| `ioWrite` | `Bytes ->{IO r} Unit` | `IO r` |
| `ioRead` | `Nat ->{IO r, Diverge} Option Bytes` | `IO r`, `Diverge` |
| `ioSleep` | `Nat ->{IO r, Cancel r} Unit` | `IO r`, `Cancel r` |
| `pollCancel` | `Unit ->{Cancel r} Bool` | `Cancel r` |

`Scope`, `Task`, `withScope`, `spawn`, `joinAllOrCancel` and the four `Io r` operations are compiler-known builtins, not foreign imports. `LoadError` and `Request` come from `Effect.Concurrent`, which the prelude re-exports.

Prerequisites: [handlers and sequencing](11-handlers-and-sequencing.md) for `do` and `with (io = ...) in`, [ownership and regions](12-ownership-and-regions.md) for `%1`, `Ref`, `Mut` and transfers. The runtime guarantees are in [concurrency and runtime](../reference/13-concurrency-and-runtime.md).

# A scope owns its children

<!-- cinch:run exit=0 -->
```cinch
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok 20)
    h2 <- spawn s (\() -> Ok 2)
    joinAllOrCancel s h h2

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> case p of
      (a, b) -> if a * 10 + b == 202 then 0 else 1
    Err e -> 2
```

- `withScope $ \s -> ...` gives the body a scope of type `Scope r s`. `r` is the IO capability's region and `s` is the scope's own region, and `s` is rigid per call, so handles from two different scopes never unify.
- A task body has type `Unit ->{eb} Result x a`. It takes `Unit` and returns a `Result`, so a failing child reports failure as a value rather than by unwinding.
- `spawn s body` performs `Async r` and the body's own row `eb`, and returns `Task s x a`. The row is the point: spawning makes the body's effects the spawning scope's responsibility, so `IO r` inside a task appears in `work`'s row and in `main`'s.
- `joinAllOrCancel s h h2` consumes both handles and returns `Result LoadError (Nat, Nat)`, with each payload in handle order. `20` and `2` land in the tuple that way regardless of which worker finishes first.
- `withScope` itself performs `Diverge`, because joining a child may not complete. That label is in the signatures above and cannot be dropped.

# The handle is consumed by the join

A binding whose type mentions `Task` is `%1`, whatever you annotate. The join is what consumes it, so a handle has exactly one legitimate use.

<!-- cinch:reject contains="task handle h must be joined exactly once" -->
```cinch
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError (Result LoadError Nat, Result LoadError Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Ok 1)
    joinAllOrCancel s h h

  main :: Int
  main = 0
```

The rule is deliberate. A handle that could be dropped would let work outlive the scope that owns its captures, and a handle that could be joined twice would let two parents claim one result.

For the same reason a scope value and a task handle stay inside their scope, and `spawn` has to be applied where the callback is visible:

<!-- cinch:reject contains="leaves its `withScope`" -->
```cinch
module Main where
  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Diverge} Nat
  bad u = withScope $ \s -> spawn s (\() -> Ok 1)

  main :: Int
  main = 0
```

The scope is `s`'s region; returning the handle would return a reference to a scope that has already exited. `spawn` also refuses to become a first-class value: `spawn` unapplied reports `spawn must be applied directly to check the task callback`, because the capture check needs the literal body.

# Cancellation is a protocol, not an exception

When a child fails, the join requests cancellation of its siblings and waits for them to acknowledge before it returns. Cancellation is cooperative: a running task never has an exception injected into it. It observes the request at a defined point, finishes its cleanup, and returns.

This program has one child fail immediately and one child loop until it sees the cancellation request, then write a byte and return. Its output is the single byte `C`, written before the join returned.

<!-- cinch:run exit=0 stdout="C" -->
```cinch
module Main where
  one-byte :: InRange 0 255 Nat -> Bytes
  one-byte v = let b = bytes-new 1 in case bytes-set b 0 v of
    _ -> b

  put-byte :: forall r. using (io :: Io r) => InRange 0 255 Nat ->{IO r} Unit
  put-byte v = ioWrite (one-byte v)

  worker :: forall r. using (io :: Io r) => Unit ->{IO r, Cancel r} Result LoadError Nat
  worker u = case pollCancel () of
    True -> case put-byte 67 of
      _ -> Ok 0
    False -> case ioSleep 1 of
      _ -> worker ()

  work :: forall r. using (io :: Io r) => Unit ->{Async r, IO r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \s -> do
    h <- spawn s (\() -> Err (LoadError 2))
    h2 <- spawn s (\() -> worker ())
    joinAllOrCancel s h h2

  main :: Unit ->{Async r, IO r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok p -> 1
    Err e -> case e of
      LoadError c -> if c == 2 then 0 else 3
```

- `worker` loops on `pollCancel ()`. `False` means no cancellation has been requested, and the loop yields with `ioSleep 1`, which is a millisecond slice that returns early when cancellation arrives.
- `True` means the request arrived. `worker` writes `C` and then returns `Ok 0`, and that return is the cleanup acknowledgment.
- `joinAllOrCancel` returns `Err (LoadError 2)`, the first observed failure. It returns only after `worker` has acknowledged, so `C` is in the output before the parent sees the failure.
- The ordering comes from the join's protocol, not from the sleep's duration. Nothing in the program says "wait long enough"; the parent blocks until the child stops.

The failure policy is what distinguishes this join from a first-completion race. A join that returned at the first failure would leave `worker` running against a scope the parent already considers closed. Because the join waits, `Err` means every child has stopped, and cleanup cannot be observed halfway through.

`pollCancel ()` is `False` on the main thread, and cancellation can only be observed from inside a task. `worker` therefore terminates only because the failure policy requested cancellation, which is a property you can read off the program.

# What cannot cross a task boundary

A task body is a one-shot closure. `%1` and `%aff` values transfer in exactly once, including on the cancellation early return. `Ref` borrows and `Copy` values share, because reading is safe from two threads. Everything else is refused at the spawn.

An exclusive borrow is the clearest case:

<!-- cinch:reject contains="exclusive `Mut` borrow cannot cross a task boundary" -->
```cinch
module Main where
  read-mut :: forall g. Mut g Nat -> Nat
  read-mut m = copyOut m

  bad :: forall r g. using (io :: Io r) => Mut g Nat ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad m = withScope $ \s -> do
    h <- spawn s (\() -> Ok (read-mut m))
    joinAllOrCancel s h

  main :: Int
  main = 0
```

A mutable buffer without a transfer is refused for the same reason, because two tasks writing one `Bytes` is a data race the type system will not accept on a promise:

<!-- cinch:reject contains="shared across tasks without shareable evidence" -->
```cinch
module Main where
  bad :: forall r. using (io :: Io r) => Bytes ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad b = withScope $ \s -> do
    h <- spawn s (\() -> Ok (bytes-len b))
    joinAllOrCancel s h

  main :: Int
  main = 0
```

A memo cell is singled out by name, because the previous chapter's cells are single-threaded by construction and their `Eval` state is not atomic:

<!-- cinch:reject contains="thread-shared memo cell cannot cross a task boundary" -->
```cinch
module Main where
  bad :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError Nat
  bad u = let ~y = bytes-len (bytes-new 2) in withScope $ \s -> do
    h <- spawn s (\() -> Ok (copyOut (demand y)))
    joinAllOrCancel s h

  main :: Int
  main = 0
```

A thread-shared memo needs an explicit synchronized structure. A `Lazy` cell is not one, and the refusal says so instead of leaving you to discover it as a race.

The alternative to refusing is to copy, and the type tells you when that is what you want: change `Bytes` to `Nat` in the second program and the capture is accepted, because a `Nat` is `Copy` and each task gets its own copy by structure.

<span class="support-tag support-tag--planned">Coming soon</span> **Bounded channels and race or select APIs.** The intended channel model specifies capacity, ownership transfer, and cancellation-safe sends, and a send race says whether ownership moved. The library ships the scopes, tasks and joins used above and does not ship a general channel API, so a program that needs one today has to model it over a scope and a synchronized structure, and the prelude does not ship one of those either.

A blocking foreign call is the one capture that is admitted and still needs care. A `foreign import` labelled `Blocking` runs on the task's dedicated OS thread and is cancellation-blind until it returns, so its label has to appear in the spawning scope's performed row. The checker folds it in, which forces your program to carry `Blocking` and makes the reason visible at the call site.

# The capability identities and exit statuses

`Async r` and `Cancel r` are effect labels with a region argument, and every task operation in one scope uses the same `r`, so the labels name a capability rather than a task. They are identities in the sense that matters for checking: they name the capability a task operation needs, and no other operation may claim them.

| Label | Performed by | Means |
| --- | --- | --- |
| `Async r` | `spawn`, `joinAllOrCancel` | this definition starts or observes a task under capability `r` |
| `Cancel r` | `joinAllOrCancel`, `pollCancel`, `ioSleep` | this definition may observe a cancellation request for capability `r` |
| `Diverge` | `withScope`, `joinAllOrCancel`, `ioRead` | completion is not promised |
| `Blocking` | a `foreign import` labelled `Blocking` | the operation blocks its thread and is cancellation-blind |

Nothing in the language discharges `Async r` or `Cancel r`. `withScope` discharges only its own scope region and adds `Diverge`, so a definition that spawns a task carries `Async r` and `Cancel r` in its public row all the way to `main`. Delete them from a signature and the checker refuses it with `effect rows do not match: {Diverge} vs {Async r#19, Cancel r#19, Diverge \| ?r905}`.

Exit statuses come from two places. A program's own status is whatever `main` returns. A runtime fault is a status the runtime defines, and it terminates the process even when it happens inside a task, because a task has no isolated failure channel of its own:

| Status | Fault |
| --- | --- |
| 70 | `BlackHole`: a memo cell demanded while being evaluated, or an early read of an unevaluated `letrec` knot |
| 71 | the generic fault path, including the compiler-bug traps and every operation whose precondition is carried by a type |
| 73 | `Exhausted`: the heap or a stack ran out, with a stderr line naming the resource |
| 74 | `LEAK`: a reclamation debug build observed an unbalanced reference count |

`Diverge` does not cover a fault. It records that a scope may not finish its join, and a fault ends the process instead.

# What the compiler enforces

- `spawn` and `joinAllOrCancel` produce and consume `Task s x a`, and `s` is rigid per scope, so handles of different scopes never unify.
- Any binder whose type mentions `Task` is `%1`. `joinAllOrCancel` takes its handles at `%1`, so a handle is joined exactly once and a second use is a multiplicity or usage error.
- A `withScope` result may not mention the scope region, so no handle and no scope value leaves its scope.
- The scope body and the task body are one-shot frames: a `%1` or `%aff` capture transfers exactly once including on the cancellation early return, `Ref` and `Copy` share, `Mut` is always refused, and an untransferred `Bytes` or `Lazy` cell is refused.
- A task body's effects are the spawning scope's effects. `spawn` performs `Async r` plus the body's row, so the body's `IO r` and `Blocking` show up in every enclosing signature.
- `withScope` and `joinAllOrCancel` perform `Diverge`, so a definition containing a scope cannot claim totality.
- Scope exit cancels and joins any child that was not joined, as defense in depth. The primary rule is still that the handle is consumed.
- `spawn` refuses to be a first-class value and refuses a partially applied form, because the capture check reads the literal callback.

# Diagnostics you will meet

| You wrote | The compiler says |
| --- | --- |
| `joinAllOrCancel s h h` | `task handle h must be joined exactly once; dropping it would silently detach execution (concurrency specification, section 2), but it is used 2 time(s)` |
| a bound handle that is never joined | `task handle h must be joined exactly once; ... but it is never used` |
| `withScope $ \s -> spawn s (\() -> Ok 1)` | ``region escape: a task handle or scope of region r#14 leaves its `withScope` in Task r#14 ?m2015 ?m2016 (concurrency specification, section 2)`` |
| a `Mut` borrow captured by a task | ``an exclusive `Mut` borrow cannot cross a task boundary: m holds one (concurrency specification, section 3)`` |
| a `Bytes` captured by a task | ``a task capture must be `Copy`, a shared `Ref`, or transferred exactly once: b is shared across tasks without shareable evidence`` |
| a `Lazy` cell captured by a task | ``a thread-shared memo cell cannot cross a task boundary: y holds a `Lazy` cell, which is single-threaded by construction`` |
| `alias = spawn` | `spawn must be applied directly to check the task callback` |
| a spawning signature without `Async r` | `effect rows do not match: {Diverge} vs {Async r#19, Cancel r#19, Diverge \| ?r905}` |

Region identifiers such as `r#14` and metavariables such as `?m2015` are allocated per compilation, so the numbers differ between programs; the text around them names the rule.

# Exercises

**Exercise 1.** `worker` in the cancellation program returns `Ok 0` after writing its byte. Why does `main` see `Err` rather than `Ok`, and what would change if `worker` returned `Err (LoadError 3)` instead?

<details><summary>Answer</summary>

`joinAllOrCancel` returns the first observed failure, and `h`'s body is `\() -> Err (LoadError 2)`, so the join's `Err` carries `LoadError 2` and the payload tuple is never produced. Whether `worker` succeeds is irrelevant to which branch the parent takes, because a failed sibling turns the whole join into `Err`. If `worker` returned `Err (LoadError 3)` the join would be choosing between two failures, and the parent still reads `Err`; the specific payload is whichever failure the join observed first. That is why a program that needs to distinguish them should model the outcome in the payload type and keep the `Err` channel for the failure policy.

</details>

**Exercise 2.** `bad b = withScope $ \s -> do { h <- spawn s (\() -> Ok (bytes-len b)); joinAllOrCancel s h }` is refused. Give two accepted rewrites and say what each one gives up.

<details><summary>Answer</summary>

Pass the length instead of the buffer: change the parameter to `Nat` and capture `n`, then `Ok n`. The task gets a `Copy` value and each side owns its own copy, so nothing is shared; you give up the ability to read the buffer inside the task, and the caller pays for whatever produced the `Nat`.

Take the buffer by transfer: annotate the parameter as `Bytes %1`, so the enclosing definition owns the buffer and the capture is a one-shot transfer, and do not use it in the parent afterwards. The task ends up as the only owner, so nothing is shared and nothing races; what you give up is the parent's access to the buffer after the spawn.

Neither rewrite shares the buffer. A shared buffer with synchronization needs a synchronized structure, and the prelude does not ship one, which is why the refusal names the alternative rather than offering it.

</details>

Next: [compile-time code and hygienic macros](18-staging-and-macros.md).
