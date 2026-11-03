# 17. Structured tasks and cancellation

Structured concurrency gives child tasks a lexical lifetime. A parent cannot quietly discard a child handle and leave the work detached. Joining is also a policy about failures and cancellation, not merely a way to retrieve two values.

# Spawn and join a pair

<!-- cinch:run exit=0 -->
```cinch
module Main where
  work :: forall r. using (io :: Io r) => Unit ->{Async r, Cancel r, Diverge} Result LoadError (Nat, Nat)
  work u = withScope $ \scope -> do
    left <- spawn scope (\() -> Ok 20)
    right <- spawn scope (\() -> Ok 2)
    joinAllOrCancel scope left right

  main :: Unit ->{Async r, Cancel r, Diverge} Int
  main u = with (io = newIO ()) in case work () of
    Ok pair -> case pair of
      (a, b) -> if a * 10 + b == 202 then 0 else 1
    Err _ -> 2
```

`withScope` establishes the task scope. Each `spawn` returns a handle. `joinAllOrCancel` consumes the handles and returns their payloads in handle order. That order is independent of which worker finishes first. The example uses the shipped `LoadError` vocabulary to keep attention on task structure; applications can model their own errors where the API admits them.

# Failure is a group operation

When a child fails, the join policy requests cancellation of the others and waits for acknowledgment. The scope cannot report clean completion while a child is still running cleanup. Cancellation is cooperative and observed at defined points; it is not an arbitrary asynchronous exception injected into every instruction.

A child that ignores cancellation or never reaches a cancellation point may prevent prompt completion. The `Diverge` effect in the signature reflects the possibility of nontermination. Structured scope ownership does not promise a deadline for an external blocking operation.

# Captures and handles

A task closure may capture a value only when the ownership rules permit sharing or transferring it. Do not capture a mutable borrow in several tasks and assume the runtime will serialize it for you. A linear task handle must be consumed exactly once; ignoring it, joining it twice, or returning it beyond its scope is rejected.

Blocking foreign calls use the dedicated blocking policy so they do not silently occupy the wrong runtime execution resource. Their effect accounting still applies. The native backend uses the Rust runtime's task support; the cooperative Wasm profile has its own admission limits, described in [targets and concurrency](../reference/13-concurrency-and-runtime.md).

<span class="support-tag support-tag--planned">Coming soon</span> **Bounded channels and race/select APIs.** The intended channel model specifies capacity, ownership transfer, and cancellation-safe sends. The current library supports the scopes/tasks/joins used above; it does not yet ship a general channel API. [The concurrency reference](../reference/13-concurrency-and-runtime.md) explains the intended commit-point rule and separates the optional STM proposal.

# Exercise and answer

Why would replacing `joinAllOrCancel` with “return as soon as the first task fails” change the guarantee?

<details><summary>Answer</summary>

The remaining tasks could still be using resources after the parent believes the scope has completed. The join waits for cancellation acknowledgment and cleanup before returning. A first failure can initiate cancellation without itself proving that the other tasks have stopped.

</details>
