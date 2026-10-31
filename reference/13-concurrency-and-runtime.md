# Structured concurrency and runtime operation

The native runtime supplies threads, structured scopes, cooperative cancellation, I/O, and resource accounting. Its source-level contracts remain in force across generated code and runtime calls.

# Task lifetimes

`withScope $ \scope -> ...` opens a scope. `spawn scope (\() -> computation)` returns a task handle. Captures must satisfy share-or-transfer ownership. A handle is linear: it cannot be discarded, duplicated, joined twice, or returned past the scope's lifetime.

`joinAllOrCancel scope h1 ... hn` admits n-ary joins. Successful payloads appear in handle order. The first failure requests cancellation of remaining work, and joining waits for acknowledgment and cleanup before the group completes. It does not merely race and abandon the losers.

The effect rows account for `Async r`, `Cancel r`, and `Diverge` where required. `withScope` does not prove that arbitrary external or looping work will terminate. A task that never reaches a cancellation point can delay cancellation completion.

# Cancellation and blocking

Cancellation is cooperative at defined runtime points, including polling and admitted waits/I/O. A cancellation request is not the same event as acknowledgment. Cleanup must not be skipped when the parent observes a failure.

Foreign calls labelled `Blocking` use the dedicated task-thread policy, with their effects retained. The Wasm profile uses cooperative task support under its target admissions rather than claiming that native threading APIs exist in every Wasm artifact.

# Channels, races, and supervised services

<span class="support-tag support-tag--planned">Coming soon</span> **Bounded channels and the fuller concurrency library.** The concurrency specification includes explicit queue capacity/backpressure, ownership transfer at a send commit point, and cancellation-safe blocked sends/receives. Failure before commit must return an owned message; after commit the receiver/channel owns it. `select`/`race` must clean up losing registrations/tasks. There are no shipped general `Channel`, send/receive, or `select` source APIs in the current Rust library; task spawning/joining does not imply those APIs exist.

Long-lived supervised services, broader synchronization/session-style protocols, deterministic scheduler control, cancellation injection, and replayable schedule exploration remain described by the design beyond the current structured-task API. A scheduling test, even when available, needs an explicit exploration bound.

**Proposed, not committed — STM.** The specification proposes an optional transactional library: `atomically`, versioned shared cells, `retry`, and `orElse`, with no irreversible I/O/FFI/linear consumption inside a retryable transaction. That is an intended design discussion, not a current library or a promise of fairness. External effects would occur after commit under an explicit protocol. No such STM operations are shipped today.

# I/O and operating-system libraries

The compiler-installed `ioWrite` writes `Bytes` and returns `Unit`; use `string-to-bytes` before writing a `String`. `ioRead` requests a byte count and returns `Option Bytes` under its I/O/divergence row; handle the absent/end-of-input case. `ioSleep` and `pollCancel` have their own effect contracts. See [compiler-provided vocabulary](20-compiler-vocabulary.md) for these names, which are not ordinary source declarations in Prelude.

`Effect.Fs`, `Clock`, and `Process` expose checked runtime facilities. Source-distributed modules add networking, HTTP, resolution, signals, pseudo-terminals, file utilities, and observability. Their APIs return named `Result` errors where expected failures are part of the contract. In particular, the current HTTP client is plain HTTP and explicitly refuses HTTPS; see its [module reference](../library/Effect.Http.md).

# Exhaustion and defenses

Heap, stack, and thread/resource exhaustion use the defined `Exhausted` path, process exit 73, with a resource diagnostic. They are not ordinary return values. Native program execution uses a separately reserved stack (default 1 GiB address space, committed as touched), configurable by `CINCH_STACK_SIZE`. Live accounted heap defaults to an 8 GiB cap, configurable by `CINCH_HEAP_LIMIT`. [Allocators and heap customization](25-allocators.md) describes the intended allocator design, including scoped heaps and recoverable budget boundaries.

Internal black-hole and impossible-boundary/fault paths remain defenses for compiler/runtime invariant failures or unsafe boundaries. Their presence is not permission for safe APIs to return arbitrary fallback words. Distinguish an application exit code from a runtime diagnostic when scripting `cinch run`.

<span class="support-tag support-tag--planned">Coming soon</span> **end-user runtime supervision.** Runtime upgrade/JIT mechanisms and target APIs do not yet supply a complete service launcher, update supervisor, or cross-platform runtime packaging tool. Use the documented native CLI and checked library contracts today.

Implementation: `check_decls.rs`, `check_terms.rs`, runtime `task.rs`, `platform.rs`, `heap.rs`, `stack.rs`; suites: `concurrency.rs`, `reclamation_tasks.rs`, `exhaustion.rs`, `platform.rs`.
