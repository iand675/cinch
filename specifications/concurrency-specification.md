# Concurrency specification

Source: <https://chatgpt.com/space/page_25f65d18f4508191ada994923cd97c35>  
Exported: 2026-10-22T18:59:36Z

---

Draft 0.2 extension. The language MUST support concurrent application code with ownership, effect and cancellation rules that remain meaningful across native and WebAssembly backends. The proposed default is structured concurrency; shared-memory and distributed protocols add explicit guarantees rather than inheriting them from task syntax.

# 1 Three distinct properties

Concurrency permits overlapping lifetimes and interleaved progress. Parallelism executes work simultaneously on multiple processing resources. Distribution introduces independent failure and communication boundaries. An `Async` effect does not imply parallelism, fairness, or distributed delivery guarantees.

The initial execution abstractions are local tasks, transferable tasks, bounded channels, scoped parallel work, and supervised long-lived services. The source semantics must specify permitted observations independently of a particular event-loop implementation. Determinism requires additional conditions; ordinary concurrency can observe different valid schedules.

# 2 Structured tasks

A task belongs to a scope whose lifetime bounds its captures. A scope cannot successfully release borrowed state while a child still uses it. Leaving the scope either observes completion or follows a declared cancellation-and-join protocol. Dropping a task handle MUST NOT silently detach execution.

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

This is a proposed library API. The combinator observes both tasks, selects the documented failure policy, requests sibling cancellation when needed, and waits for cleanup acknowledgment before returning. Its `LoadError` or task outcome contract includes cancellation and supported task failures. It cannot promise bounded completion if a child or foreign dependency can remain stuck.

Long-lived background services attach to an explicit supervisor or root scope. Supervision controls restart and reporting; it does not imply state rollback or exactly-once external effects.

# 3 Ownership across tasks

| Operation | Required ownership condition |
| --- | --- |
| Spawn local work | Captures remain valid for the scope; no incompatible aliasing during interleaving |
| Spawn migratable work | Captures and results satisfy a `Send`-like transfer interface |
| Share references across threads | The referent satisfies a `Sync`-like interface and the borrow remains live |
| Send an owned message | Ownership transfers at the channel's specified commit point |
| Share mutable state | Use an approved synchronization abstraction with an invariant |

The marker names are proposed and their implementations must be checked or explicitly trusted. An immutable wrapper around a thread-affine foreign pointer is not automatically transferable. A local task can still create a logical race by interleaving operations; staying on one thread does not make every state protocol atomic.

The type system must model borrows held across suspension. The initial default SHOULD reject ordinary blocking mutex guards across calls whose effects include `Async`, with separate async-aware abstractions where appropriate. This prevents a common problem but does not prove arbitrary deadlock freedom.

# 4 Cancellation and resource completion

Cancellation is a cooperative request delivered at defined suspension or polling points. It is a distinct control effect or outcome, not an arbitrary exception injected between any two machine instructions. Short, explicitly scoped masking regions may protect ownership transitions; unbounded masking must not hide a progress obligation.

The API distinguishes cancellation requested, cancellation observed, cleanup completed, and operation outcome. A foreign call or host operation may continue after a request. Owned buffers and callback registrations remain live until the operation's protocol permits release.

A channel send race must say whether ownership transferred. Failure before commit returns the message; after commit, the receiver or channel owns it. An API that loses a linear message when cancellation races with send is invalid. `select` and `race` combinators must dispose of losing registrations and tasks according to their protocols; dropping a future is not a universal undo operation.

Shutdown has a declared policy for unfinished cleanup and unresponsive children. Forced process or component termination is an outer containment action with explicit resource and external-outcome consequences. Structured concurrency guarantees lifetime discipline, not that every cleanup action eventually succeeds.

# 5 Channels and actors

Bounded channels are the default proposal so backpressure and queue capacity are explicit. Their contracts specify ownership transfer, closure, blocked send and receive behavior, cancellation, and the ordering actually guaranteed. Global total order and delivery exactly once are not inferred from a channel type.

Actors are a library abstraction owning private state and receiving typed messages. Message protocols can be represented using typestate or session-style interfaces. Such interfaces can establish legal local communication sequences; global progress requires additional constraints on participants, topology, scheduling and failure.

Actor restart and hot replacement preserve mailbox contents, reply obligations, resource handles and unknown external outcomes when required. Replacing the handler does not by itself validate those transitions.

# 6 Shared memory and atomics

The language memory model must define permitted concurrent observations and synchronize-with relationships. Safe code prevents data races; it does not automatically make a multi-step operation linearizable. Synchronization libraries package invariants, ownership transfer rules, and proofs at their logical linearization points.

Atomic operations expose the supported ordering choices explicitly. Release/acquire proofs must establish an actual synchronizes-with relation for the relevant communication; merely labeling two operations does not create one. Relaxed operations provide only their specified guarantees, and the compiler cannot infer ordinary inter-thread happens-before facts from them.

Low-level atomic libraries need memory-reclamation protocols as well as atomic instructions. ABA, premature reclamation, and callback lifetime can invalidate a lock-free structure even when all individual accesses use atomics. Epochs, hazard pointers or another chosen reclamation scheme require their own contracts.

Backends MUST preserve the source memory model through native or Wasm lowering. Unsupported orderings or atomics cannot silently become unsynchronized accesses. Initial high-level APIs should use proved locks and channels; lock-free algorithms are an advanced library boundary.

# 7 Software transactional memory

A Haskell-like STM library is a proposed addition. `atomically` evaluates a transaction over versioned shared cells and commits atomically or retries. Transactional code is restricted to approved STM effects and total logical computation; it cannot perform arbitrary I/O, FFI, suspension, or irreversible resource consumption inside a retryable body.

`retry` suspends until relevant state changes under the selected runtime contract. `orElse` composes alternatives with documented retry behavior. Transactions may be re-executed, so captured ownership and allocations must obey a rollback-safe discipline. Starvation freedom and fairness are additional claims, not consequences of atomic commit.

External effects occur after commit through an explicit protocol, such as a durable outbox when the application needs crash recovery. An in-memory transaction plus a network request is not a distributed atomic transaction.

# 8 Proofs and testing

The design supports library-specific proofs of state invariants, linearizability, protocol adherence, resource accounting, and temporal progress. Each proof identifies environmental and scheduling assumptions. A race-free program can still deadlock, starve, leak logical work, or produce wrong results.

Testing infrastructure SHOULD include deterministic scheduler control, cancellation injection at supported points, and replayable execution traces. Bounded schedule exploration and model checking are useful counterexample tools. Their bounds remain explicit; successful testing is not a universal proof.

Lock ordering can prove freedom from selected lock cycles. Session types can prove selected protocol properties. Neither mechanism alone proves that an arbitrary service always responds, particularly across foreign calls and network failures.

# 9 Backends and scheduler capabilities

The same structured-task API can have a cooperative single-threaded implementation or a multithreaded implementation where its contract permits. Code explicitly requiring parallel execution must request that capability or accept an explicit fallback policy. The library must not conceal unsupported parallelism behind a performance promise.

Blocking calls use an offload or host suspension contract supported by the target. A browser host without such support cannot satisfy an arbitrary blocking-native-call abstraction by running it on its event loop. Scheduler work budgets and cooperative yield points are part of responsiveness policies, not proof that all tasks terminate.

WASI async integration preserves completion, stream backpressure and cancellation rules. Native async and component async may use different runtime mechanisms while implementing the same source-level contract.

# 10 Concurrent hot replacement

A module update is itself a concurrent protocol. Its activation point defines which version accepts new calls, which handles and queues remain shared, and how outstanding operations are attributed. State snapshots need synchronization or a revalidation guard before migration commits.

Draining tasks includes children, foreign operations and callbacks, not just exported function frames. Version coexistence requires a cross-version state protocol. A shared state invariant cannot be changed while old code continues assuming the previous invariant without a relation that supports both.

Liveness proofs cover the updater and scheduler together. Repeated updates must not indefinitely restart work or lose progress. The runtime's cancellation, fairness and migration policies are therefore dependencies of temporal contracts.

# 11 Initial scope

Implement structured scopes, cooperative cancellation, bounded channels, local tasks and explicit blocking-call handling first. Add transfer-checked parallel tasks and proved synchronization primitives next. STM, session-style protocol libraries and lock-free structures can build on that foundation; their interfaces and evidence should remain replaceable modules where the runtime supports it.

Required rejection cases include borrowed state escaping its scope, unsafely sharing a thread-affine handle, losing a message during cancelled send, performing FFI inside a retryable transaction, and claiming a timeout stopped a foreign operation without acknowledgment.

[Documentation overview](../overview.md)

# Current syntax and sequencing

The draft 0.2 surface uses layout, equations and unqualified `do`. See [Surface syntax and grammar](surface-syntax-and-grammar.md) and [Runtime capabilities, effects and sequencing](runtime-capabilities-effects-and-sequencing.md) for formal syntax, runtime arguments, effect-polymorphic calls and sequencing-module selection.