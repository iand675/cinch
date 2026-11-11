# Green threads: structured tasks as fibers on a scheduler

Status: as built, 2026-11-11. The native runtime runs every structured-concurrency task as a stackful fiber, multiplexed over a small set of OS threads by a scheduler (`compiler/runtime/src/sched.rs`). A task that waits is suspended and its thread runs another task; a program whose whole thread budget is one OS thread still suspends and resumes its tasks, on that thread. This note records the model, the protocol that makes a park safe, which per-thread state follows a fiber, the rule that keeps a migrating fiber from reading another thread's state, blocking regions, timers, the measured cost, and what is not done.

The short version:

1. A task is a fiber: a body, an 8 MiB reserved stack with a guard band, and a saved context. The context switch is a few lines of assembly per architecture (aarch64 and x86_64).
2. One run queue behind one lock. Worker threads are started lazily up to the budget (`CINCH_THREADS`, else the available parallelism). The program's thread counts toward the budget and runs fibers whenever its own code waits.
3. Everything that waits goes through the park seam (`park.rs`): a join, a scope's close, `ioSleep`, a transaction's `retry`. Parking a fiber is a state transition split across the switch so that a wake racing the park is never lost and never resumes a fiber that is still on its stack.
4. The running task, the stack fault zone, the entered arena and the quote evaluator's state follow the fiber from thread to thread. The small-object heap and the parallel pool registration stay with the thread.
5. A `Blocking` foreign call and a stdin read are blocking regions: while one is in progress the scheduler may run one more thread, so a blocked task does not hold up the others.
6. Wasm keeps its run-to-completion cooperative backend, and a native target without a context switch keeps one OS thread per task. Both sit behind the same runtime entries.

# 1 What it replaces

Before this, every task was one OS thread (`std::thread`) with an 8 MiB stack, created at `spawn` and joined through a condition variable. That is simple and it keeps a blocking call from holding up anything else, but a task costs a thread creation, and the only single-threaded backend (`coop.rs`, the Wasm profile's) runs each body to completion on the caller's stack, so a body that has to wait for a task suspended below it on that stack can never proceed.

The park seam (`park.rs`) was already the one place the runtime suspends a context. Its contract did not change: a `Parker` names a context, `park()` suspends the caller until its token arrives, `unpark()` leaves at most one token, a park may return spuriously and every caller parks in a loop over its own condition. What changed is what a context is.

# 2 The model

## 2.1 Contexts

`park::current()` answers one of three kinds of context:

| context | what it is | what `park()` does |
|---------|------------|--------------------|
| fiber | a task body on its own stack | switches to the scheduler; the thread takes the next runnable fiber |
| helping thread | the program's thread (`cinch_run_main` registers it) | runs queued fibers on this thread until its own token arrives, and sleeps when none is runnable |
| any other thread | a parallel pool worker, a thread foreign code created | `std::thread::park()` |

A fiber is not bound to a thread. It runs on whichever scheduler thread takes it from the run queue, and after a park it may continue on a different one.

## 2.2 Threads and the budget

The budget `N` is `par::budget()`: `cinch_par_set_workers` when the program set one, else `CINCH_THREADS`, else the available parallelism. It is the number of OS threads that run tasks:

- The program's thread is one of them. It runs fibers only while its own code is parked.
- Up to `N - 1` worker threads (`N` when no helping thread is registered) are started lazily, one for each queued fiber that finds no thread asleep to take it. A worker runs the scheduler loop on a 512 KiB stack inside the stack guard and never runs task frames on that stack.
- With `N = 1` there are no workers. Every fiber runs on the program's thread, one at a time, and a fiber that parks is resumed later on that same thread.
- A thread the scheduler does not own cannot run fibers, so the first fiber such a thread spawns makes one worker allowed whatever `N` is. (A C host that calls an exported Cinch function is the case: there is no program thread.)

A worker with nothing to run waits on one condition variable. A worker that finds itself idle while more workers exist than are allowed (after a blocking region ends, section 7) exits.

## 2.3 The run queue and who is woken

There is one first-in first-out queue of runnable fibers, behind the scheduler's one mutex. Queueing a fiber wakes exactly one thread for it: an idle worker nothing has claimed yet, else a helping thread asleep in its park, else a new worker if the budget allows. If none of those exists the fiber waits for a running thread to reach the queue. The scheduler counts the threads that are on their way to the queue (notified, or just started) so that two queued fibers do not count on the same sleeper.

Nothing preempts a running fiber. It runs until it parks, yields or returns. The yield points are `pollCancel` and `ioSleep 0`: a fiber yields there when another fiber is queued, or when it is running on the program's thread and that thread's own wake has arrived. That is what lets a task that polls for cancellation in a loop share one thread with the task that will cancel it.

## 2.4 Stacks

A fiber's stack is a reservation of 8 MiB of address space (`TASK_STACK_BYTES`), committed by the operating system as it is touched, whose low 64 KiB are never made accessible. A fiber gets its stack when it first runs, not when it is spawned, and a finished fiber's stack is reused: each scheduler thread keeps one for the next fiber it starts, and up to 64 more are kept in a shared cache. So the stacks in use follow the fibers that are running or suspended, not the fibers that exist: ten thousand tasks that each run to completion on four threads use four stacks.

## 2.5 The context switch

`sched/switch.rs` has one naked function per architecture. It pushes the callee-saved registers of the platform's C calling convention on the current stack, stores the stack pointer through its first argument, adopts the stack pointer in its second, pops the same registers and returns:

- aarch64 (AAPCS64, Darwin and Linux): `x19` to `x28`, `x29`, `x30`, `d8` to `d15`, `sp`. `x18` is never touched.
- x86_64 (System V): `rbx`, `rbp`, `r12` to `r15`, `rsp`, and the control bits of `mxcsr` and the x87 control word.

A context that has never run is laid out so that its first resume lands in a two-instruction trampoline that calls the fiber's entry function with a cleared frame pointer. The floating-point control register of aarch64 is not switched: generated code never changes it.

# 3 The state machine and the post-switch protocol

Every fiber has a state and a token, both sequentially consistent atomics.

| state | meaning | who moves it on |
|-------|---------|-----------------|
| `RUNNABLE` | in the run queue, or just taken from it | the scheduler thread that takes it: `RUNNING` |
| `RUNNING` | its code is executing on some thread | the fiber itself: `PARKING` or `DONE` |
| `PARKING` | it has decided to park and is switching away | the scheduler thread it ran on: `PARKED` |
| `PARKED` | switched out, waiting for a wake | an unpark, a timer, or the scheduler thread that parked it: `RUNNABLE` |
| `DONE` | its body returned and its stack is recycled | nobody |

A fiber cannot store `PARKED` itself. Between that store and the instruction that saves its registers, another thread could see `PARKED`, queue the fiber, and a third could resume it on a stack the first is still using. So the transition is split across the switch:

1. The fiber finds its token clear, stores `PARKING`, and switches to the scheduler context of the thread it is on.
2. That thread, now off the fiber's stack with the fiber's registers saved, stores `PARKED` and then reads the token. If the token is set it exchanges `PARKED` for `RUNNABLE` and queues the fiber.
3. `unpark` stores the token and then exchanges `PARKED` for `RUNNABLE`, queueing the fiber if the exchange succeeds.

Steps 2 and 3 each store first and read second. Whenever an unpark races a park, at least one of the two sees the other's store, so the wake is never lost; both may see it, and then exactly one exchange succeeds, so the fiber is never queued twice. An unpark that finds the fiber `PARKING` only leaves the token, and step 2 queues the fiber immediately. A fiber clears its token when it runs again. A wake that arrives after that is kept for the next park; one that arrived before it is covered by the condition the caller rechecks, because every waker sets the condition before it unparks.

A yield is the same path with the token set by the fiber itself, so the fiber goes to the back of the queue. A fiber ends the same way too: it stores `DONE`, switches away for the last time, and the thread it ran on recycles the stack it has just left.

The non-atomic fields of a fiber (saved stack pointer, stack, body, saved per-task state) have one owner at a time: whoever created it until it is queued, then the scheduler thread that took it from the queue, then the fiber's own code. Storing `PARKED` is what gives the ownership up.

# 4 State that follows the fiber

Some state the runtime keeps per OS thread is logically per task. The scheduler exchanges it at every switch in and switch out, so a fiber finds its own wherever it resumes and the thread underneath keeps its own. The audit covers every `thread_local!` and thread-specific key in `compiler/runtime/src`:

| state | where | follows | why |
|-------|-------|---------|-----|
| the running task | `task.rs` `CURRENT_TASK` | the fiber | `pollCancel` and every cancellable wait ask whether the running task is cancelled |
| the stack fault zone | `stack.rs`, the thread-specific key the signal handler reads | the fiber | a fault in the fiber's own guard band is its stack overflow, on whichever thread it runs |
| the entered arena | `arena.rs` `CURRENT` | the fiber | an arena belongs to the scope that entered it; another task on the same thread must not allocate from it |
| the quote evaluator's nesting, re-entry count and pending fault | `eval.rs` `TLS` | the fiber | an evaluation's frames are the fiber's frames |
| the running fiber, the thread's scheduler context, its spare stack, its helper record | `sched/fiber.rs` `RUN` | the thread | this is the scheduler's own view of the thread |
| the small-object heap | `small.rs` `CURRENT`, `TEARDOWN`, and the program thread's `SOLE_THREAD` fast path | the thread | a heap has one owner at a time and no allocation spans a switch; a block freed on another thread takes the existing remote-free path |
| the parallel pool registration | `par.rs` `LOCAL` | the thread | a parallel entry joins everything it published before it returns and its jobs are pure, so no fiber switch happens inside one |
| a wait being registered | `stm.rs` `WAITING` | the thread | set by `cinch_stm_wait_begin` and taken by the call that ends the wait before anything suspends |
| the cooperative ready queue | `coop.rs` `READY` | the thread | that backend has no fibers |
| test instrumentation | `heap.rs` `FORCE_RC_DEBUG`, `THREAD_FREED`; `rc.rs` `FREE_LOG` | the thread | test builds only, and no test moves a fiber while it records |

One consequence for the parallel pool: a joiner helps only while more than half of its stack remains, and it used to measure that against the thread's stack. A task's frames are on its fiber's stack, so a fiber is now measured against its own (`sched::help_floor`).

# 5 The migration rule

After a switch a fiber may be on a different OS thread, and the compiler is entitled to compute the address of a thread-local once per function and reuse it. A frame that is live across a park must therefore never do that. Two rules hold in the runtime:

1. The functions that switch (`sched::park`, `park_timeout`, `yield_now`) and every function that reads a thread-local on behalf of a frame that parks are `#[inline(never)]`, so each call computes the address on the thread it is running on: `sched::current`, `blocking_enter`, `blocking_exit`, `task::current_task_canceled`, `stm::take_wait`, `eval::swap_state` (every access of the evaluator state goes through it), and the join's and the close's hand-over of results (`task::hand_over`, `task::discard_results`), which release blocks into the thread's heap.
2. No frame holds a reference derived from a thread-local across a call that may park. A join and a close drop the scope's lock before they park for the same reason: the frame may resume on another thread.

The scheduler's own frames are exempt because they do not move: a fiber always switches back to the scheduler context of the thread it is running on, so the frame that switched into a fiber is resumed on the thread that left it. The allocator is reached through the allocator symbol, which is a call boundary in every build the runtime is made with (no link-time optimization), and it looks its heap up anew on every call. Generated code contains no thread-locals.

# 6 Waits

- **Join and close.** `Scope::join_all_or_cancel` and `Scope::close` register the caller's `Parker` in the scope under the scope's lock and park. A finishing body stores its result and its done flag under the same lock, takes the registered parkers and unparks them after releasing it, so the result is published before the wake, as the fork-join rule requires.

    A single-threaded waiter is woken once per finished child, so each pass does work in proportion to what changed rather than to the member count, by two cursors rather than one scan of the list:

    * `finished` only moves forward, and only over members that are done. A member's done flag is set once and never cleared, so the finished prefix of the member list only grows, and each pass advances the cursor past whatever finished since the last one. The first member not done ends that pass's scan. Members are not joined in completion order, so the prefix can stall on one slow member while later ones finish; those passes then do one load and park again, which is constant work per wake.
    * `failures_seen` is the scope's failure count at the last look. The scope counts a failure in `Scope::finish` under the same lock, so a pass that finds the count unchanged reads no member's result at all. The counter belongs to the scope, not to this join, so it can move because of a child this join does not hold; the scan that follows then finds no failure among `states` and records the new count. The trigger is conservative and never misses one: every failure among the members bumps the counter, under the lock the waiter takes.

    The cancellation request that follows a found failure goes once to the members after `finished`, since the ones before it have already returned. Without the two cursors, joining ten thousand tasks on one thread rescanned all ten thousand on every wake. Measured on this machine, debug build, budget one: 1.66 to 1.84 s for the naive rescan against 0.02 to 0.19 s for the cursors, for the same ten thousand tasks joined in order.
- **Cancellation.** A request sets the task's flag and unparks its context, once. A body parked in a cancellable wait resumes and sees the flag; a body that reaches one later reads the flag before it suspends.
- **`ioSleep`.** A timed park (section 8). It ends early on a cancellation request because the request unparks the context. `ioSleep 0` is a yield.
- **`retry`.** `stm.rs` is unchanged in meaning: it parks through the seam, so a blocked transaction in a task now suspends the fiber.
- **`pollCancel`.** Yields first, then reports the flag, so a request made by a task that ran during the yield is observed at the same poll.

# 7 Blocking regions

A `Blocking` foreign call is the one place a task holds an OS thread, and the reason is structural rather than an omission: the scheduler can only suspend a fiber at a point it controls, and there is no such point inside foreign code. It cannot save and restore the callee's frames, and it does not know whether the callee holds a lock, a pointer into the fiber's stack, or thread-affine state that a later resume on another thread would invalidate. Suspending there would mean resuming a foreign stack on a thread the callee never expected. So a fiber that blocks in the operating system blocks the thread under it.

The offload contract (D79: a `Blocking` call blocks only its own task) is therefore kept around that fact instead of by defeating it, by bracketing such calls:

- `sched::blocking(|| ...)` inside the runtime, used for the stdin read of `cinch_io_read`.
- `cinch_sched_blocking_enter()` and `cinch_sched_blocking_exit()` for generated code. The code generator puts the pair around the call in the wrapper of every foreign import whose declared row carries `Blocking` (`compiler/src/codegen.rs`, `foreign_call`; the plan records the label as `ForeignPlan::blocking`). A program with no such import emits neither the calls nor their declarations, so its IR is unchanged. The cooperative (Wasm) profile emits no bracket.

While a context is inside a region the number of allowed workers is one higher. A worker is started at the moment a fiber is runnable with no thread on its way to it, which may be at entry to the region or later. When the region ends the allowance drops, an idle worker is told to look again, and one that finds itself surplus exits. Only contexts the budget counts take part: a fiber, or the program's thread in its own code. On any other thread both calls do nothing.

The counter is per context and nests. A fiber that parks inside a region (a foreign call that calls back into Cinch code that waits) keeps its region open while it is parked, so the allowance is one too high for that time; nothing depends on it being exact.

# 8 Timers

`park::park_timeout(d)` on a fiber inserts the fiber into a queue ordered by deadline (an ordered map keyed by deadline and a sequence number) and parks it. Every scheduler thread fires the due timers each time it looks for work, and one with no work sleeps no longer than the earliest deadline. A timer is removed when its park returns early, so a cancelled sleep leaves nothing behind. A new earliest deadline wakes one sleeper so that it re-reads the queue.

On the program's thread a timed park is the helping loop with a deadline of its own, and on a thread the scheduler does not own it is `std::thread::park_timeout`. Neither uses the timer queue.

Timers are only as punctual as the threads are free: if every scheduler thread is inside a fiber that does not yield, a sleep overshoots until one of them reaches the queue.

# 9 Exhaustion

- A fiber that overflows its stack faults in its own guard band. The handler of `stack.rs` finds the band through the thread's key, which the scheduler set to the fiber's zone at the switch, and takes the `Exhausted` exit: `cinch: exhausted: stack`, exit 73. It runs on the alternate signal stack of the thread underneath, which is why every thread that runs fibers is inside `stack::guarded`.
- A fiber stack that cannot be reserved is `cinch: exhausted: stack` as well, at the fiber's first run.
- `cinch: exhausted: threads` remains for an OS thread the system refuses: the program's thread, a scheduler worker, a parallel worker.

# 10 Reclamation and the memory model

Nothing changes in what crosses between tasks or when. `Scope::spawn` still marks everything the closure reaches (`cinch_mark_mt`) before the task can run anywhere, and results still cross at the join unmarked. The happens-before edges the plain counts rely on are still there: a spawn queues the fiber under the scheduler's lock and a thread takes it under that lock; a body's result and done flag are stored under the scope's lock, which the joiner takes to read them; and a fiber's own execution is ordered across a migration, because the thread it left stores `PARKED` after the fiber's last instruction there and the thread that resumes it took it from the queue after that store. A block no other task can reach is therefore still touched by one thread at a time.

# 11 Measurements

Apple arm64 (M-series), 18 cores, macOS 27.0.1, release build of the runtime, 2026-11-11. The harness is `compiler/runtime/tests/sched.rs` (`what_tasks_cost`, run with `--release -- --ignored --nocapture`); "before" is the same tree built with `--cfg cinch_thread_tasks`, which is the old one-OS-thread-per-task code path. Task bodies return at once. Times are the best of five passes of 2,000 tasks; the ranges are across four runs of the harness, each in its own build directory so the two backends cannot be confused.

| shape | one OS thread per task | fibers |
|-------|------------------------|--------|
| spawn one task and join it, from the program's thread, budget 1 | 18.1 to 24.9 µs | 0.167 to 0.184 µs |
| the same, budget 4 | 18.0 to 21.9 µs | 0.29 to 0.56 µs |
| the same, from a thread the scheduler does not own, budget 4 | 17.0 to 22.6 µs | 4.1 to 5.0 µs |
| spawn 2,000 tasks then join them all, per task, program's thread, budget 1 | 10.2 to 11.5 µs | 0.157 to 0.184 µs |
| the same, budget 4 | 10.7 to 11.1 µs | 0.37 to 0.45 µs |
| the same, from a thread the scheduler does not own, budget 4 | 10.7 to 11.4 µs | 0.41 to 0.73 µs |

The third row is slower than the first two because both the worker and the waiting thread are woken through the operating system; on the program's thread the join runs the task itself.

Ten thousand tasks suspended at once (each waits until its scope's close cancels it), budget 4:

| | one OS thread per task | fibers |
|-|------------------------|--------|
| time until all ten thousand are suspended | 3.2 to 3.4 s | 27 to 30 ms |
| resident memory for them | 344 MiB (352,048 to 352,224 KiB, 35 KiB each) | 160 MiB (163,312 to 163,488 KiB, 16 KiB each: one 16 KiB page of stack) |
| cancel and join them all | 96 to 103 ms | 42 to 46 ms |

Ten thousand tasks that run to completion (the `many` scenario) use four stacks with a budget of four, three with a budget of four from the program's thread, and one stack with a budget of one. Every one of the ten thousand stacks above is a live mapping, because those tasks are all suspended at once; the reuse shows up only when tasks finish.

# 12 Tests

`compiler/runtime/tests/sched.rs` runs each scenario in a child process with its own `CINCH_THREADS` and a watchdog, on the test's own thread and as a program's `main`:

1. one thread: task A parks mid-body, task B, started later on the same OS thread, unparks it, A resumes, both are joined; no worker thread is started;
2. 10,000 tasks in one scope with a budget of 4 (and of 1), joined, nothing leaked, at most four stacks mapped;
3. 64 pairs of tasks passing a turn 2,000 times each through park and unpark, at budgets 1, 2, 4 and 8, with tasks observed to resume on other threads;
4. a task that recurses without bound is exit 73 with `cinch: exhausted: stack`, on a worker and on the program's thread;
5. a cancellation request resumes a task parked in `cinch_stm_wait_park(1)`, which answers 1;
6. with a budget of 2, two tasks inside blocking regions wait for a third, which runs on a third thread that then exits; and the program's thread blocked in a region with a budget of 1;
7. `cinch_io_sleep` in a task returns after its time, another task runs meanwhile on the same thread, a cancellation ends a ten-minute sleep at once, and the timer queue is empty afterwards.

`compiler/tests/concurrency.rs` does the end-to-end versions with compiled programs: the one-thread suspend and resume, ten thousand tasks with the leak check, the failure policy with a sibling that polls without sleeping (which finishes on one thread only because the poll yields), a sleeping task that lets a later task write first, and two `Blocking` foreign calls that wait for a third task under a budget of two (which deadlocks without the generated bracket). `compiler/tests/exhaustion.rs` runs the task stack overflow on a worker and with a budget of one.

# 13 What is not done

- **No I/O reactor.** A read that would block blocks its thread. Only `cinch_io_read` and `Blocking` foreign imports are blocking regions. The runtime's other blocking operations are not bracketed: `cinch_net_tcp_connect`, `cinch_net_read`, `cinch_net_write`, `cinch_net_resolve`, `cinch_proc_run`, `cinch_process_read`, `cinch_process_write_stdin`, `cinch_process_wait`, `cinch_pty_read`, `cinch_pty_write`, `cinch_pty_wait`, and the file operations. A task inside one holds its thread, and with a budget of one thread it holds the program.
- **No preemption.** A fiber that computes without reaching a park or a yield keeps its thread. On the program's thread that includes a fiber the thread picked up while it was parked: the thread's own code continues only when that fiber next parks, yields or ends.
- **No work stealing.** One queue, one lock. The lock is taken once to queue a fiber and once to take it.
- **A task waiting in a parallel join holds its worker.** `par.rs` sleeps on its own condition variable, outside a blocking region.
- **`cinch_stm_yield` is still an OS thread yield**, not a fiber yield.
- **The x86_64 context switch is assembled but has never run.** `cargo rustc --target x86_64-unknown-linux-gnu --crate-type staticlib` produces it and the disassembly is the intended sequence, but this machine cannot execute x86_64 code. Linux on either architecture has not been run either: everything was exercised on macOS arm64.
- **Foreign frames larger than the guard band.** Generated code and the runtime's Rust probe every page of a large frame. A C function with an unprobed frame larger than 64 KiB can step over a fiber's band, as it could over a task thread's.
- **Fiber stacks are 8 MiB.** `CINCH_STACK_SIZE` sizes the program's thread and the parallel workers only, as before.
- **A suspended fiber costs at least one page of stack**, 16 KiB on Apple arm64, and a finished fiber's small record stays allocated as long as its task handle does (handles are leaked by design, `task.rs`). Every stack is two mappings (the band and the rest), so a system that caps the mappings of a process caps the fibers suspended at once; past the cap a stack cannot be reserved, which is the stack `Exhausted` exit. Not measured: the ten thousand above are far below any default cap.
- **Wasm is unchanged.** The cooperative backend runs each body to completion on the caller's stack, because that target has no second stack to switch to without a transform of the generated code.
