# Tutorial 13 Structured concurrency

Source: <https://chatgpt.com/space/page_1a35ce96cd3c819197921ef44d2d156e>  
Exported: 2026-10-22T18:59:36Z

---

Run two independent operations concurrently while keeping ownership and failure behavior visible. Concurrency is part of the effectful program layer; it does not weaken the total logical fragment used to check proofs.

# Give tasks a scope

```haskell
loadPair :: forall r. using (io :: Io r) =>
            Request -> Request
            ->{Async r, IO r, Cancel r, Diverge}
              Result LoadError (Bytes, Bytes)
loadPair left right = withScope $ \s -> do
  a <- spawn s (\() -> fetch left)
  b <- spawn s (\() -> fetch right)
  joinAllOrCancel s a b
```

These are proposed APIs. Each child owns its request. `joinAllOrCancel` returns both successful results or follows its documented error policy: request cancellation of the remaining work, wait for cleanup, then return an error. The `LoadError` contract includes supported task failures and cancellation. A child that cannot stop may prevent the scope from returning; the type does not claim bounded shutdown.

Scoped borrows are also useful: tasks can borrow input when the scope proves that no task outlives it. Capturing an exclusive borrow in two simultaneously live tasks is rejected unless a safe partition or synchronization abstraction justifies it. For work that may migrate between threads, captured values and results need transfer evidence.

# Treat cancellation as a protocol

Consider a task calling a native decoder. Receiving cancellation does not make the C stack disappear. The task retains its output buffer and module reference until the operation finishes or acknowledges that it will no longer access them.

The same issue occurs with asynchronous hosts. Cancellation requested, cancellation observed and cleanup completed are distinct events. A timeout can report that the caller stopped waiting, but an API must explain who now owns the unfinished work and its resources.

# Use bounded channels for pipelines

A channel carries owned messages and has an explicit capacity. A successful send transfers ownership to the channel or receiver. A send cancelled before its commit point returns the message; cancellation after commit cannot give the same ownership back to the sender.

This makes bounded producer/consumer pipelines natural. Backpressure limits queued messages, although it does not bound every allocation in the application. Closing an endpoint must specify what happens to queued messages and blocked peers. An actor can package state, a mailbox and its handler behind a module interface.

# Compose shared-state operations

For a Haskell-like alternative to manual locking, the proposed STM library permits atomic transactions over transactional cells. Its effect restriction forbids ordinary I/O, FFI and async suspension in a retryable transaction. A transaction can execute repeatedly before committing, so performing a payment or closing an external handle inside it would be unsound.

Use a post-commit protocol for external effects, with durable records when crash recovery matters. Atomic memory updates alone do not make a remote action exactly once.

# State the proof you actually need

Ownership can establish the absence of data races. A channel law can establish message ownership transfer. A lock-order argument can exclude certain deadlocks. A temporal theorem can establish eventual response under explicit scheduler and environment assumptions. These are different results.

Test cancellation at suspension points and explore schedules to find counterexamples. Record the explored bounds. Successful tests do not become a universal liveness proof.

During hot replacement, drain or migrate the scope's children, mailboxes, callbacks and pending host operations as well as the module's visible state. A quiescent exported function stack alone is insufficient.

See `examples/11-structured-tasks.lang` and the concurrency specification for scheduler capabilities, atomics, supervision and upgrade rules.

[Documentation overview](../overview.md)

# Continue reading

[Tutorial 12 WebAssembly components](12-webassembly-components.md) · [Language design overview](../overview.md)

# Current syntax and sequencing

The draft 0.2 surface uses layout, equations and unqualified `do`. See [Surface syntax and grammar](../specifications/surface-syntax-and-grammar.md) and [Runtime capabilities, effects and sequencing](../specifications/runtime-capabilities-effects-and-sequencing.md) for formal syntax, runtime arguments, effect-polymorphic calls and sequencing-module selection.