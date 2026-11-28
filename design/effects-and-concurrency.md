# Effects, concurrency, and parallelism

This document covers Cinch's effect system, lexical capabilities, sequencing modules, green-thread concurrency, synchronization primitives (`MVar` and `STM`), pure parallelism, and data-parallel arrays / Metal GPU execution.

Normative specifications live in [`specifications/runtime-capabilities-effects-and-sequencing.md`](../specifications/runtime-capabilities-effects-and-sequencing.md) and [`specifications/concurrency-specification.md`](../specifications/concurrency-specification.md). User guides live in [`reference/09-effects-and-capabilities.md`](../reference/09-effects-and-capabilities.md) and [`reference/13-concurrency-and-runtime.md`](../reference/13-concurrency-and-runtime.md).

---

## 1. Effects, handlers, capabilities, and sequencing

Cinch keeps three mechanisms orthogonal:

1. **Row-typed effects (`->{IO r, Throw e, Diverge | e}`):** declare what a computation may perform. Effect labels carry optional region/runtime identities (`IO r`, `Async r`, `Cancel r`, `Eval r`, `Blocking`, `Diverge`, `STM`, `MVar`, `Foreign Name`) or type parameters (`effect Throw (e :: Type)`), plus user-declared `effect` blocks. Effect honesty is enforced by row unification.
2. **Handlers (`handle ... with`):** interpret effect operations and remove handled labels from the performed row. Resumptions (`resume k`) are one-shot (enforced by `%aff`/`%1` linearity so linear captures cannot be duplicated). Tail-resumptive and abortive handlers dispatch across arbitrary call depths and higher-order combinators via Koka-style evidence passing and cooperative bubbling (`Effect.Throw`), with polymorphic effect monomorphization, exact Perceus RC cleanup on every unwound frame, zero yield checks in pure `->{}` functions, and static refusal of `%1` linear values live across abort. Direct non-tail-resumptive handlers work inside the `handle` expression; non-tail-resumptive cross-call handlers remain deferred.
3. **Lexical capabilities (`using (io :: Io r) => ...`, `#[capability]`):** represent runtime authority and scoped environments as ordinary dictionary parameters supplied lexically via `with (name = ...) in body` or explicitly via `f @{name = ...}`, including higher-rank `using (...) =>` callback arguments. `Io r` carries a region-scoped runtime identity `r` so capabilities cannot escape their scope; user-defined `#[capability]` types (such as `Effect.Reader`'s `Read a` and `TaggedRead tag a`, combinable with `#[repr(transparent)]`) preserve pure arrows (`->{}`) and `Ref r` region-escape safety.
4. **Sequencing modules (`do`):** unqualified `do` desugars through an in-scope `Sequencing` provider (direct binding by default, or `Result`, `Option`, or `StmSequencing` short-circuiting/transactional binding). Refutable `<-` patterns require a provider with typed failure.

### Value-level computations (`Comp {e} a`)
Rather than adding a new core form, `Comp {e} a` is a `#[repr(transparent)]` library newtype over `Unit ->{e} a` with `suspend` and `run`. It allows data structures and pull-based streams (`Effect.Stream`) to store effectful steps as first-class values without implicit sequencing.

### Draft proposals: effect boundaries and policies
Future extensions under consideration (not yet implemented):
- **Effect boundaries and interposition:** declarative interceptors at module/package boundaries to audit, mock, or restrict capabilities (e.g., network/filesystem policy enforcement).
- **Scoped and transaction effect policies:** typing database/transactional sessions so effects inside a transaction scope cannot escape or commit inconsistently.

---

## 2. Structured concurrency and green threads

Structured tasks (`withScope`, `spawn`, `joinAllOrCancel`) run as **stackful green threads (fibers)** multiplexed onto a bounded pool of OS threads (`seed/runtime/src/sched.rs`):

- **Fibers & stacks:** each fiber receives an 8 MiB reserved virtual stack with a 64 KiB guard band on first run; completed stacks are cached and recycled (1 per scheduler thread + up to 64 shared). Context switching uses small naked assembly stubs for `aarch64` (AAPCS64) and `x86_64` (System V).
- **Thread budget:** `CINCH_THREADS` (or `cinch_par_set_workers`, defaulting to available cores) bounds total OS threads. The program's main thread participates as a helping thread whenever it parks, so `N = 1` still runs and suspends arbitrary numbers of fibers on a single thread.
- **Post-switch park protocol:** `park.rs` provides token-based `park`/`unpark`. Fibers transition `RUNNING -> PARKING -> PARKED` across the context switch so a racing `unpark` can never lose a wake or resume a fiber still executing on its stack.
- **State migration:** per-task runtime state (`CURRENT_TASK`, stack guard zone, entered arena, quote-evaluator TLS) swaps with the fiber; per-thread allocator heaps stay on the OS thread. Functions that park or read thread-locals across a park are `#[inline(never)]` so TLS addresses are never cached across migrations.
- **Blocking regions:** `Blocking` foreign imports and `cinch_io_read` bracket their calls with `cinch_sched_blocking_enter`/`exit`, temporarily raising the allowed worker count by 1 so an OS-blocked call does not stall other fibers.
- **Linear handles & cancellation:** `Task` handles are `%1` linear (dropping a handle without joining is a compile error). Cancellation is cooperative (`pollCancel`, cancellable sleeps/waits).
- **Wasm fallback:** `wasm32` uses a run-to-completion cooperative scheduler (`coop.rs`) behind the same runtime entry points.

**Current scheduler limits:** there is no async I/O reactor yet (blocking socket/process/file syscalls hold their OS thread), no preemptive scheduling, and a single mutex-guarded FIFO run queue (no work stealing yet).

---

## 3. Synchronization: `MVar` and `STM`

### Native `MVar` (`Effect.Concurrent.MVar`)
A synchronized empty-or-full single-value box backed by a native mutex and FIFO wait queues (independent of STM):
- Supports `mvar-new`, `mvar-new-empty`, `mvar-take`, `mvar-put`, `mvar-read`, `mvar-swap`, nonblocking `mvar-try-*`, and cancellable `mvar-*-or-cancel`.
- Blocking operations carry `{MVar, Diverge}`; cancellable forms additionally require `Io r` and carry `Cancel r`.
- Waiters park through `park.rs` (works even with 1 worker thread). Handoffs are strictly FIFO by registration order, a `put` wakes all waiting readers together without starving takers, and a completed handoff wins over a racing cancellation.
- Element types crossing tasks must be `Share`; counted values are marked shared (`cinch_mark_mt`) before publication.

### Software transactional memory (`Effect.Stm`)
An optimistic TL2 (Transactional Locking II) implementation with snapshot extension, written mostly in Cinch source (`seed/prelude/Effect/Stm.cinch`) over inline LLVM atomic `prim`s so uncontended transactions avoid C-ABI runtime calls:
- **Runtime support (`seed/runtime/src/stm.rs`):** owns only `KIND_TVAR` mutable-slot blocks (`[lock/version, value, scan_code]`), the cache-line-padded global clock and sleeper count, `cinch_stm_publish`, and `park.rs`-backed sleep/wake.
- **RC soundness on mutable slots:** (1) before committing to a shared `TVar`, staged values are marked shared (`cinch_stm_publish` to a fixed point); (2) readers set a reader-pin bit around loading and incrementing counted values while writers CAS only from unlocked, unpinned words, swapping old values out to release after locks drop.
- **Purity & rollback safety:** `STM a` steps are pure functions (preventing I/O inside transactions by construction). `StmSequencing` uses `ContKind::Many` so `%1` linear values cannot cross `<-` inside a retryable transaction.
- **Waiting & composition:** `retry` registers a 64-bit read-set bitfilter and parks via `park.rs` with SeqCst Dekker ordering (no lost wakeups); `or-else` merges read sets on retry. Derived structures (`TMVar`, `TQueue`, `TBQueue`, `TSem`) are pure library code.

---

## 4. Pure parallelism

Cinch provides deterministic pure parallelism (`parBoth`, `parTabulate`, `parMap`, and the `Parallelism` kernel signature) backed by a work-stealing/helping pool (`seed/runtime/src/par.rs`):

- **Safety rules enforced by the checker:**
  - Parallel callbacks may perform only the empty row `{}` or `{Diverge}` (no observable side-effect ordering or thread-affine state; rigid row variables are rejected).
  - Captures shared across parallel branches or indexed callbacks must be `Share` (no `Mut` borrows, no single-threaded `Lazy` memo cells).
  - Callbacks must be applied directly (lambda, global, or partial application) unless forwarded through the `Kernel (a -> b)` evidence class.
- **Status of `Parallelism` class:** the `Parallelism` interface (`both`, `tabulate`, `mapArray`, `workers`) and host lowering table exist, but dictionary-level forwarding collides with per-call-site representation specialization (`PrimOp::Par` records element classes per site) and `Row`-kinded data parameters (blocking a library `Parallel` applicative for `parTraverse`). Direct `par*` builtins and `Kernel`-evidence data-parallel combinators are the working paths today.

---

## 5. Data-parallel arrays, Typed Array IR, and Metal GPU

`Data.Array.Compute` and `Data.Device` provide deterministic parallel array operations across multicore CPU and Apple Metal GPUs (`seed/src/array_ir.rs`, `seed/runtime/src/par_arrays.rs`, `seed/runtime/src/metal.rs`):

- **Zero-copy shape-indexed views:** `View1 n a`, `View2 m n a`, and `View3 f h w a` wrap immutable `Array a` buffers with static dimension witnesses (`Fin n` indexing, `Sigma`-typed filter lengths, zero-copy `slice` and `transpose`, explicit `pack` for non-contiguous reshapes).
- **Explicit numerical contracts:**
  - `array-foldl-*` is strictly sequential left-to-right (rejected on required GPU targets).
  - Integer reductions specify an explicit `IntReducer` (wrapping/bitwise/min/max).
  - Float reductions require an explicit `FloatMode`: `FloatExactTree` (left-complete binary tree over 1024-element blocks, bit-for-bit identical across scalar CPU, multicore CPU, and Metal GPU) or `FloatRelaxed` (SIMD / `simd_sum`).
- **Typed Array IR (`seed/src/array_ir.rs`):** runs after specialization and before RC/codegen to perform producer-consumer fusion (`zip-mul -> reduce`, `map -> reduce`, `gather + zip-mul -> segmented-reduce`, multi-step `stencil2d` ping-pong fusion), adaptive nested-parallelism scheduling (`many-small-frames` vs `few-large-frames`), and storage planning. Inspectable via `--array-report` / `--array-report-json`.
- **Kernel portability & `.cinchif` metadata:** `Kernel (a -> b)` allows reusable higher-order parallel combinators to forward callbacks across modules. Interfaces record `{"kernel": {"cpu_parallel": bool, "gpu_metal": bool, "blocker": ...}}`, and `--array-target=metal-required` rejects GPU-ineligible kernels at compile time naming the blocker.
- **Metal GPU runtime:** dynamically loads `Metal.framework` / `Foundation.framework`, keeps intermediate buffers device-resident in single-command-buffer pipelines (1 upload, 0 intermediate round-trips, 1 download), and exports a C ABI usable from C and Python (`examples/gpu-pipeline/`).
