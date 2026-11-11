# Parallel evaluation: deterministic pure parallelism for the compiler

Status: proposal for review, 2026-11-10. User direction: compiles should be fast and scale over N cores, and Cinch's concurrency primitives may need to grow to allow that. This note surveys what the structured-task mechanism provides today, proposes a small family of pure, deterministic parallel primitives that need no `Io r` capability, specifies the runtime and the work in both compilers, and shows where the self-hosted compiler would use them and what that should buy, from a phase profile of a real self-build.

The short version of the decisions:

1. Three checker-installed builtins, `parBoth`, `parTabulate` and `parMap`, whose meaning is defined to be their sequential reading (`(f (), g ())`, `array-tabulate n f`, `map f xs`). Parallelism is permission, never semantics, so results are identical for every worker count and every schedule.
2. Callbacks may perform only the parallel-safe row: the closed empty row or `{Diverge}`. Captures must be `Share`, or move `%1` into exactly one `parBoth` branch. Callbacks are read literally at the call site, the way `spawn` is.
3. A work-stealing pool of N participants (`-j N`, `CINCH_THREADS`, default the available parallelism), with per-thread heaps, help-while-waiting joins, and reference-count promotion (marking) only at the moment a job becomes stealable.
4. The compiler parallelizes per definition (body inference, the independent Core re-check, total-body collection, RuntimeIr emission) and per module (parsing, interface export by dependency wave). A measured 585-second single-threaded self-build is 90% parallelizable work by this decomposition; the model predicts about 137 s at 8 workers and 107 s at 16.
5. Byte-identical output regardless of N is a requirement the compiler meets by construction (no shared counters across parallel units, merges in input order) and the ladder checks at several N.

# 1 What exists today

## 1.1 Surface and typing

The concurrency surface is three builtin values with checker-installed schemes (`install_concurrency_api`, `compiler/src/check_decls.rs:666-922`; `reference/13-concurrency-and-runtime.md`):

```text
withScope       :: forall r s a e. using (io :: Io r) => (Scope r s ->{|e} a) ->{Diverge | e} a
spawn           :: forall r s x a eb. Scope r s -> (Unit ->{eb} Result x a) ->{Async r | eb} Task s x a
joinAllOrCancel :: Scope r s -> Task s x v %1 -> Task s x w %1 ->{Async r, Cancel r, Diverge} Result x (v, w)
```

Decisions D74 to D81 (`specifications/concurrency-specification.md`, Amendments) fix the rest: the scope region is rigid per call, so handles of distinct scopes never unify (D74); a body returns `Result x a` and the join returns the first observed failure after every sibling's cleanup (D75); any binder mentioning `Task` is `%1` and `withScope`'s result may not mention the scope region (D76); task and scope bodies are one-shot frames (D77); the join is n-ary (D78); `Blocking` foreign calls run on the task's thread and fold their label into the scope's row (D79). The effect cost is the point of the design: a definition that spawns needs an `Io r` capability (`withScope` takes `using (io :: Io r)`) and carries `Async r`, `Cancel r` and `Diverge` all the way to `main`, and nothing discharges `Async r` or `Cancel r` (`learn/17-structured-concurrency.md`, "The capability identities and exit statuses").

## 1.2 Captures

`check_task_captures` (`compiler/src/check.rs:4989-5072`) classifies each value a task body captures, after the definition is inferred. A captured memo cell (`mentions_memo`) is refused; a `Mut` borrow is refused; a region released inside the task is an escape; otherwise a `%1`/`%aff` binder may transfer if the type passes `structural_usage(.., Transfer)`, and anything else must pass `share_safe`, which is `structural_usage(.., Share)` plus class dictionaries over shareable method records (`check.rs:5077-5087`). The judgment (`check.rs:5180-5283`) has consequences that matter for any parallel use, and they are stated here as the code reads:

| Type | `Share` | Note |
| --- | --- | --- |
| `Int`, `Nat`, `Bool`, `Unit`, machine scalars | yes | |
| immutable ADT, tuple, `Array` | iff every field or element is | structural recursion |
| `Ref r a` | iff `a` is | |
| `Bytes` | no, transfer only | mutable buffer (`check.rs:5201`) |
| `Sym` | no, transfer only | "never `Copy`" (`check.rs:5202-5205`), although the cell is immortal |
| function, handler, codata | no, transfer only | a function type does not record its captures (`check.rs:5216`) |
| type parameter `a` | no | no `Share a` constraint exists (`check.rs:5228-5234`) |
| `Mut`, `Lazy` | never | |

`Data.String.String` is `StrBuf :: Bytes -> String` behind a module-private constructor (`compiler/prelude/Data/String.cinch:70-71`), so by the judgment as written a `String` capture is not `Share` either: the immutability contract lives in a comment, not in anything the checker reads. The compiler's own data is full of `String` (every name), so this gap has to close before the compiler can share its contexts across threads.

Task callbacks are read literally: `spawn` refuses to be a first-class value or partially applied, because the capture check reads the callback (`bootstrap/Bootstrap.Infer.cinch:2503-2504`, "spawn must be applied directly to check the task callback"). Source-level task forwarders exist, but their obligations "cannot travel through this interface schema", so they are source-only and dropped from exported interfaces (`bootstrap/Bootstrap.Export.cinch:910-918`).

## 1.3 Runtime thread model

Every task is a fiber owned by its scope (`compiler/runtime/src/task.rs`, `compiler/runtime/src/sched.rs`; `design/green-threads.md`): a body on an 8 MiB reserved stack (`TASK_STACK_BYTES`) with a guard band, run by a scheduler over at most `CINCH_THREADS` OS threads, the program's thread among them. `Scope::spawn` marks the closure shared and queues the fiber; the body runs on whichever scheduler thread takes it, and its result and done flag are stored under the scope's mutex. `join_all_or_cancel` parks the caller through the park seam (a waiting task gives its thread to another task) and is woken as members finish; the first member it sees finished with `Err` becomes the failure and the others are asked to cancel. Which failure that is depends on completion timing whenever two members fail. Cancellation is a sequentially consistent flag observed only at `pollCancel` and `ioSleep` (D80). The program's own code runs on a thread whose stack is `CINCH_STACK_SIZE` (default 1 GiB), and an OS refusal to create a thread is the `Exhausted` exit (`stack::spawn_refused`).

The Wasm profile uses a single-threaded cooperative scheduler with the same source contract (`compiler/runtime/src/coop.rs:22-65`): FIFO ready queue, bodies run to completion between suspension points, deterministic interleaving.

## 1.4 How values cross threads

Reference counts follow the values at exactly two points (`task.rs:37-62`, `design/reclamation.md` section 8, "As built (S9)"):

- At spawn, `cinch_mark_mt` negates the count of every mortal block reachable from the body closure before the thread exists. A negative count is atomic; a positive one is single-threaded. The walk is pointer-reversal, uses no memory beyond the blocks, stops at immortal and already-marked blocks, and the marked set is closed under reachability, so an unmarked block is reachable from one thread only (`compiler/runtime/src/rc.rs:12-27`, `64-80`). A root is therefore promoted once; a later spawn that reaches it costs one header read at that root.
- At join, results cross back without marking: the body's stores happen before the joiner's loads, so the joiner's plain counts follow the task's.

Freed blocks go back to the owning thread's size-class heap; a free from another thread is one compare-and-swap onto the owner's remote list, and heaps outlive their threads and are adopted by new ones (`compiler/runtime/src/small.rs:33-61`). Heap accounting is per-thread atomic tallies summed against `CINCH_HEAP_LIMIT` (`compiler/runtime/src/heap.rs:83-178`). Exhaustion and faults end the whole process from whichever thread hits them (`compiler/runtime/src/abi.rs:90-101`).

## 1.5 Costs and limits

- A task costs a queue insertion, a context switch in and out, a scope mutex round trip, and a mark walk over everything the closure reaches that is not yet marked: measured at 0.2 to 0.5 µs for a spawn and a join of an empty body, against 10 to 16 µs when every task was an OS thread (`design/green-threads.md` section 11). That is cheap enough for I/O concurrency in bulk, and still the wrong tool for fine-grained parallel work: a task carries effects and a capability, its failure order depends on timing, and the mark walk is paid per spawn.
- After marking, every dup and drop on a marked block is an atomic read-modify-write. Measured at S9, the sign test and atomic header loads on unmarked blocks cost under 1% (`design/reclamation.md:1076-1086`); the cost of contended RMWs on marked blocks has not been measured.
- Marking cannot see counted blocks behind slots S5 left `Raw`: memo values, knot values, a dictionary's value-method slots, join tuples wider than four, boxed-out boundary words, builtin index-family constructors, unrecorded class vectors (`design/reclamation.md:1046-1054`). Two threads counting such a block race.
- Using tasks for compiler phases would force pure functions such as `checked-source :: Program -> List LoadedInterface -> Result (List Diagnostic) CheckedProgram` (`bootstrap/Bootstrap.Pipeline.cinch`) into `{IO r, Async r, Cancel r, Diverge}` with an `Io r` capability, would make the reported failure depend on timing, and would refuse every `String` capture.

## 1.6 The compilers themselves are single-threaded

The self-hosted compiler never uses tasks. `tooling/bootstrap.py` concatenates every `bootstrap/*.cinch` module and five `Text.*` toolkit modules into one bundle (`collect_compiler_sources`, `bundle`) and runs one compiler invocation per stage (`build_self`). Inside it, `resolve-context` replaces the module list with a single flattened `Main` module before checking (`bootstrap/Bootstrap.Resolve.cinch:1012-1030`), so all 4,548 top-level definitions are checked as one module by `elaborate-module-with-source-bodies` (`bootstrap/Bootstrap.Infer.cinch:5454-5507`). One `clang` process assembles the result (`bootstrap/Bootstrap.Driver.cinch`, `spawn-clang`). The stage scripts pin `CARGO_BUILD_JOBS=1` and one test thread (`tooling/bootstrap.py:97`).

The Rust seed is sequential too: one `Checker` is threaded through every module in order (`compiler/src/check_decls.rs:17779-17805`), and its only threads exist to get a larger stack (`check_decls.rs:17806`, and three test-only threads in `codegen.rs`).

# 2 The primitive

## 2.1 The family

Three builtins, installed by the checker like `withScope` and named in its camelCase style:

```haskell
parBoth     :: forall a b e. (Unit ->{|e} a) -> (Unit ->{|e} b) ->{|e} (a, b)
parTabulate :: forall a e. Nat -> (Nat ->{|e} a) ->{|e} Array a
parMap      :: forall a b e. (a ->{|e} b) -> List a ->{|e} List b
```

Each has a defining equation, and the equation is the semantics:

```haskell
parBoth f g     = (f (), g ())
parTabulate n f = array-tabulate n f        -- Data.Array, the sequential reading
parMap f xs     = map f xs
```

`array-tabulate :: forall a. Nat -> (Nat -> a) -> Array a` already exists in `compiler/prelude/Data/Array.cinch:21-25`. `parBoth` is the fork-join building block for divide and conquer; `parTabulate` is the data-parallel workhorse, where one index is one unit of work; `parMap` is the form the compiler wants, since its phases are list folds. Wider fork-joins are nested `parBoth` or a `parTabulate` over an index that selects the job. No first-class future or handle exists: a parallel value is never observable before its join, so there is nothing to leak, drop or join twice, and none of the D76 handle rules are needed.

Futures in a region were considered and rejected for this slice. A future is a first-class handle that must be linear (or the program can observe that evaluation has not happened yet), needs a region to bound it, and makes "when is this forced" a scheduling question the type checker would have to reason about. Fork-join gives the compiler everything it needs. Spark-style `par`/`pseq` annotations depend on lazy evaluation, which Cinch does not have. Implicit parallelism of every call (heartbeat scheduling, Acar, Charguéraud and Rainey, PLDI 2018) is the attractive later step (section 7, phase 6).

## 2.2 Effects, divergence and partiality

A callback's row `e` must be parallel-safe: after the definition is inferred, `e` must resolve to a closed row that is the empty row or `{Diverge}`. An `e` still unsolved at the end of the definition defaults to the empty row. A rigid row variable (a row-polymorphic caller) is refused in this slice with a message naming the rule; phase 6 (section 7) adds a bound that lets it through.

Every other label is excluded, each for a reason:

| Label | Why excluded |
| --- | --- |
| `IO r` | the order of observable effects would depend on the schedule; that is what tasks are for |
| `Async r`, `Cancel r` | task operations would block or orphan a pool worker, and need the capability anyway |
| `Blocking` | a blocking foreign call would pin a pool worker; D79 gives such calls their own thread |
| `Eval r` | demanding a captured memo cell runs its single-threaded state machine from two threads |
| `Foreign`, `Global`, `Fail`, `Reentrant`, `ThreadAffine` | foreign state and thread affinity are exactly what the pool cannot promise |
| user-declared effects | an operation dispatches to a handler record built on the caller's stack, whose closures may hold that frame's state |

An effect handled entirely inside a branch never appears in the branch's row, so it is admitted without any rule: a branch can install its own handler.

`Diverge` is admitted because the sequential reading already permits it: if a branch may not return, `parBoth` may not return, and `e` carries the label to the caller. A pure caller with pure callbacks stays pure.

The outcome rule covers everything that is not a normal return:

> If every branch returns, the result is the defining equation's value, independent of the worker count and the schedule. If some branch does not return (it diverges, faults, or exhausts a resource), the outcome is the outcome of one such branch, and which one is unspecified.

This differs from strict left-to-right evaluation in one case: sequentially, a diverging first branch hides a fault in the second; in parallel the fault ends the process. That is acceptable for three reasons. Under `design/no-faults.md` a well-typed program does not fault apart from `Exhausted`, a `BlackHole` across a trust boundary, and compiler-bug traps; the core specification already lets logical reasoning assume unbounded resources only when stated (section 6); and the core specification's rule that "scheduler progress is not used as an unchecked premise in the total logical fragment" holds trivially, because the defining equations do not mention a scheduler. Proofs, the law sampler and staging evaluate the defining equations directly.

## 2.3 Ownership and sharing

The callbacks are read literally, as `spawn`'s are. Admitted callback forms: a lambda literal, a global function name, a partial application of a global to arguments, and a local alias of any of these, traced through the way the existing task check traces "a callback stored under a second name" (`reference/13-concurrency-and-runtime.md`, "Ownership across a task boundary"). A function-typed parameter of the enclosing definition is refused, because its captures are unknown; phase 6 (section 7) and the open decisions (section 8) lift this.

Each capture is classified with the existing machinery and these rules:

| Capture | `parBoth` | `parTabulate`, `parMap` |
| --- | --- | --- |
| `Share` value (section 2.4 for the extensions) | shared by all branches | shared by all invocations |
| `%1` or `%aff` value | moves into exactly one branch; a second branch using it is `used 2 time(s)` | refused: the callback runs n times |
| `Ref r a` with `a` `Share` | shared | shared |
| `Bytes` not transferred | refused | refused |
| `Mut` borrow | refused | refused |
| `Lazy` cell | refused (lazy specification section 9) | refused |
| function value | traced to its captures, refused if untraceable | same |

`parMap`'s element type must be `Share`, because the input becomes one array read by every participant. Results have no constraint: each result is built by one participant and moves to the caller at the join, exactly like a task result.

No capture is mutable, so the only cross-thread traffic on captured data is reference counting. Two decisions keep that small:

- Promotion happens once per transferred root, and only when needed. The runtime marks a callback's closure (and `parMap`'s input array) when a job that reaches it first becomes stealable (section 3.3), not at the call. A call that never publishes, because no worker is idle or N is 1, marks nothing. Marking stops at already-marked blocks, so a large shared context is walked once for the whole program, and later publications pay one header read at its root.
- Captures are borrowed inside the branch. The caller keeps the callback closures alive until the join and drops them afterwards, so a branch reads captured fields through `__apply`'s borrowed closure (`design/reclamation.md` section 4, "Borrowed parameter inference") and dups a captured value only when it stores it into its result.

## 2.4 `Share` extensions this needs

The judgment of section 1.2 has to grow in four places before the compiler can use the primitives, and the same changes benefit tasks:

1. **`Sym` is `Share`.** A `Sym` is a handle to an immortal, immutable cell in a mutex-protected table (`compiler/runtime/src/sym.rs:1-14`). `Share` is not `Copy`: the specification's refusal of `Copy` for `Sym` stands, but sharing a reference to a cell that is never freed and never written is safe.
2. **Declared-immutable abstract types.** A data declaration whose constructors are module-private may carry a `share` attribute (the attribute mechanism of `design/attribute-annotations.md`), asserting that no operation of the module writes a buffer reachable from a constructed value. `Data.String.String` takes it, as does any other module-private wrapper over `Bytes` with the same contract. The assertion is trusted, so it is listed in the acceptance manifest (core specification section 11), and it travels in the compiled interface, because an importer cannot see the private constructor's fields.
3. **`Share a`.** A type parameter is `Share` under a `Share a` constraint, the compiler-known class with structural instances that `design/ownership-and-usage.md` section 3 already describes. Generic code that calls `parMap` over `List a` says so.
4. **Marking blind spots.** Every slot kind in the "what marking cannot see" list must either get a scan class or make its type non-`Share`. The dictionary value-method slot matters most, since shareable dictionaries are admitted.

## 2.5 Cancellation and failure

Pure parallel code has no cancellation and no failure channel of its own. A failure is a value: a callback that returns `Result e b` produces an array of results, and the caller picks the first `Err` by index, which is deterministic. The task join's "first observed failure" rule is not reused, because observation order is schedule order.

The cost of that choice is wasted work after an error: sibling invocations run to completion even when an earlier index already failed. For the compiler that happens only on a failing compile. A pruning variant that stops publishing indices beyond a known failing index can be added without changing results (section 7, phase 6).

A fault or exhaustion in any participant ends the process, as it does in a task today (`learn/17-structured-concurrency.md`: "a task has no isolated failure channel of its own"). A diverging branch means the join never returns; the other participants finish their jobs and go idle. A task cancelled while its body is inside a parallel call observes the request at its next poll point after the call returns, as with any long pure computation.

## 2.6 Granularity

The unit of work is one callback invocation, and the program chooses its size: one module, one definition, one chunk of definitions. The runtime adds no unit smaller than that. Above it:

- `parTabulate` and `parMap` start as one index range owned by the caller. A participant working a range of more than one index splits off the upper half onto its deque only when some worker is idle and its own deque is empty (lazy binary splitting, Tzannes, Caragea, Barua and Vishkin, PPoPP 2010); otherwise it runs the next index inline. The first split marks the callback and input; later splits find them marked.
- `parBoth` publishes `g` only when a worker is idle, which costs one relaxed load of an idle count; otherwise it runs `f ()` then `g ()` inline with no marking and no synchronization.
- Sequential fallback is automatic and gives identical results when N is 1 (`-j 1`, `CINCH_THREADS=1`), on the cooperative (Wasm) backend, when no worker is idle, and when n is below 2.
- For recursive divide and conquer, the program cuts off below a size, because only it knows the cost of a subproblem:

```haskell
sum-tree :: Tree -> Nat
sum-tree t = case t of
  Leaf n -> n
  Node size l r ->
    if size < 4096 then sum-tree l + sum-tree r
    else case parBoth (\() -> sum-tree l) (\() -> sum-tree r) of
      (a, b) -> a + b
```

# 3 Runtime

## 3.1 The pool

A process has one pool of N participants. N comes from, in order: `parSetWorkers n` (below), `CINCH_THREADS` read at program start, and otherwise `std::thread::available_parallelism()`. `N = 1` means no pool threads at all. The pool holds N - 1 worker threads, because the thread that calls a parallel primitive participates too; it starts lazily at the first publication, so a program that never publishes never creates a thread.

```text
parSetWorkers :: forall r. using (io :: Io r) => Nat ->{IO r} Unit
```

`parSetWorkers` exists so a program's command line can set the budget; the Cinch compiler maps `-j N` to it. It is an `IO r` operation because it changes process state, even though no result depends on it.

Workers run guarded stacks the size of `CINCH_STACK_SIZE` (default 1 GiB, reserved and committed on touch), not the 8 MiB task stack: a branch must be able to recurse as deeply as the same code on the program thread, or the sequential reading would not hold. Sixty-four workers reserve 64 GiB of address space and commit only what they touch. Each worker allocates from its own `small.rs` heap and adopts a parked heap when one exists; nothing in the allocator changes.

## 3.2 Scheduling and joins

Each participant owns a Chase-Lev work-stealing deque, registered the first time that thread calls a primitive (the program thread, a task thread, or a worker). Owners push and pop at the bottom; thieves steal from the top. Idle workers spin briefly, then park on a futex or condition variable; publication wakes one when the idle count is nonzero.

A join waits on the job's completion latch, and the waiting thread does not block while it can be useful:

- If the published job is still on the owner's deque, the owner pops it and runs it inline.
- If it was stolen, the owner helps: it steals first from the thief that took its job (leapfrogging, Wagner and Calder, 1993), then from anyone, and rechecks the latch between jobs.
- It parks only when nothing is stealable, or when less than half of its stack reservation remains, because helping runs a stolen job on top of the waiting frame and could exhaust a stack that sequential evaluation would not.

There are no locks in pure parallel code and every job's dependencies are its own children, so the only way a join fails to return is a diverging branch.

## 3.3 Marking, results and the ABI

Four new runtime entries, each taking closures borrowed (the caller drops them after the call):

```text
cinch_par_both(f, g, out)            -> 0      out[0], out[1]: owned result words
cinch_par_tabulate(n, f, class)      -> array  element words in index order
cinch_par_map_array(f, arr, class)   -> array  f applied to each element
cinch_par_set_workers(n)             -> 0
```

`class` is the element scan class the code generator already passes to array constructors. Publication follows the S9 rule that makes marking sound: before a job that reaches a closure becomes stealable, the publishing thread runs `cinch_mark_mt` on the closure (and on the input array for `parMap`), while every unmarked block it reaches is still reachable from this thread only. The deque push is a release and the steal an acquire, so a thief sees the marks.

Results cross back without marking, for the reason they do at a task join. Each invocation's result is written by one participant into its own slot of an output buffer the caller allocated; the participant's completion is a release on the job's latch and the join's observation an acquire, so the caller's plain counts on result blocks follow every count the participant made. A participant keeps no reference to a result after its job returns, because pure code has nowhere to keep one: no mutable globals, captured memo cells refused, `Sym` cells immortal. Blocks a worker allocated and the caller later frees take the remote-free path into the worker's heap.

Programs built without reclamation (`CINCH_RECLAIM=0`) never mark, as with tasks (`rc::reclaiming`).

## 3.4 Interaction with structured tasks

The two mechanisms stay separate. Tasks remain the mechanism for I/O, blocking calls and cancellation, each a fiber on the task scheduler's threads; the pool runs only pure jobs, so it never runs a task body, an `IO r` operation or a `Blocking` call, and the rows of section 2.2 guarantee that statically. A task body may call a parallel primitive; the thread running it registers a deque and participates like any other, and the task stays on that thread until the call returns. The pool size does not depend on the number of tasks.

The cooperative backend implements the four entries as sequential loops, which satisfies the defining equations and keeps Wasm output identical.

## 3.5 Memory model

The source-visible model gains one statement: everything before a parallel primitive happens before each branch, and each branch happens before the primitive returns, the same fork-join edges D80 gives tasks. The deque and latch atomics are runtime-internal. D80's "no other atomics" becomes "no other source-visible atomics".

## 3.6 As built (phase 1)

The runtime half is `compiler/runtime/src/par.rs`, the cooperative loops are `cinch_coop_par_*` in `coop.rs`, and the tests are `compiler/runtime/tests/parallel.rs`. It decides these points the sections above leave open:

- **Argument words.** `n` arrives through `cinch_nat_to_len`, as `cinch_array_replicate`'s does; `parTabulate`'s callback receives the `Nat` word of its index; result words are stored as the callback returns them, as `cinch_array_from_list` stores a list's words, in an array of the given element format. `cinch_par_set_workers` takes a count through `cinch_nat_to_len`, where 0 restores the default.
- **Closure references.** Today's apply stubs consume their closure (`compiler/src/codegen.rs`, `stub_def`), so the runtime gives a reclaiming program's borrowed closure one reference per invocation, and a counted `parMap` element one reference as its owned argument. On a marked closure that is two atomic operations per invocation on one header word, plus the stub's dups of counted saved words. Measured with a release build on this host (`parallel_tabulate_benchmark`), a `parTabulate` of 10^6 trivial items takes 2.9 ms at N = 1 and 0.6 ms at N = 8 through a static closure, but 4.2 ms at N = 1 and 33 to 50 ms at N = 2 to 8 through a heap closure, all of it contention on that one header. Phase 2 should therefore add a borrowing apply for parallel callbacks (section 2.3's "captures are borrowed inside the branch"), which removes both the closure's count traffic and the per-call dups of captures. Callbacks of a millisecond or more, the grain section 5 plans, do not notice it.
- **Who counts as idle.** Publication compares the participants looking for work with the jobs queued (`idle > queued`, two relaxed loads), rather than reading an idle count alone, so `k` idle workers get at most `k` jobs. A joiner whose job was stolen counts as idle while it may help, and sleeps in the same sleeper set as idle workers, so a busy thief publishes to it and a publication wakes it. Without that, a two-participant `parBoth` tree runs on one core whenever the caller finishes its half first.
- **Deques.** Fixed capacity (1,024 jobs); a full deque runs the work inline. Publication needs an idle participant per queued job, so depth stays near the budget. A participant is registered on a thread's first parallel call and handed, empty, to a later thread when this one exits; participants are never freed, because thieves read any deque without a lock.
- **The budget.** `CINCH_THREADS` is a positive decimal count; anything else selects the available parallelism; both clamp to 256. The pool starts at the first call that could publish (a budget above 1 and at least two units of work). A lower budget set later retires the surplus workers (they wait, they do not exit); a higher one starts or reactivates workers at the next call.
- **Marking `parMap`'s input.** The general walk of `cinch_mark_mt` rescans a block's slots from the top each time it returns to it, which is quadratic in an array's length (a 100,000-element input would cost 5 * 10^9 reads). The input array is marked by `rc::mark_mt_array` instead: claim the array, then mark each counted element as its own root. An array reached through a closure's captures still takes the general walk, and phase 5's counters should show whether that matters.
- **The parking counter.** Phase 1's acceptance asks for a counter of workers that park beside a nonempty deque. A sleeper checks the queued count under the sleeper lock after announcing itself, and a publisher reads the sleeper count after counting its job, so one of the two always sees the other; a separate deque scan at park time races with a concurrent push and would count wakeups that are already on their way. The test checks the observable property instead: in 200 rounds, every worker asleep, the caller publishes `g` and then blocks until another participant starts it; any lost wakeup or withheld publication stalls a round (`sleeping_workers_wake_for_every_publication`, zero misses).
- **Failure.** Nothing is caught. A fault, a compiler-bug trap or an `Exhausted` exit ends the process from the participant that hit it, and workers run inside the stack guard, so a stolen job's overflow is exit 73 with the `stack` line.
- **The ABI version.** The four entries are runtime ABI 8. `RUNTIME_ABI_VERSION` (`compiler/src/interface.rs`), `RUNTIME_ABIS` in `tooling/bootstrap.py`, `bootstrap/Bootstrap.RuntimeAbi.cinch` (rendered from the lock), `bootstrap/library-provider.lock.json` and `bootstrap/stage0.lock.json` move to 8 in phase 2, together with the code generator that calls the entries.

# 4 Work in both compilers

Both compilers must implement the primitives before the compiler's own source may use them: Stage1 is built by the Rust seed (Stage0) and Stage2 by Stage1, so the seed, the runtime and the Cinch implementation land first, the pinned seed is updated, and only then does `bootstrap/*.cinch` call `parMap`.

## 4.1 Checker rules

Rust seed: an `install_parallel_api` beside `install_concurrency_api` registers the three schemes as `PrimOp::Par(ParOp)` values. `check_terms.rs` elaborates their applications like `spawn`'s: refuse first-class and partial use, record the callback's captures with a one-shot frame flag for `parBoth` branches and a many-shot flag for the other two, and defer classification to the end of the definition. A new pass beside `check_task_captures` applies section 2.3's table and the row rule of section 2.2, reusing `share_safe` with section 2.4's extensions.

Cinch compiler: `Bootstrap.Builtin` gains three `task-rule`-style `builtin-syntax` entries (`ParBoth`, `ParTabulate`, `ParMap`) with `first_class` false; `Bootstrap.Infer` gets the application rules (the `b.spawn` path at `Bootstrap.Infer.cinch:4135-4171` is the template); `Bootstrap.Typing` gets a `checked-par` rule and a trace that reuses `task-trace`'s capture walk with the many-shot distinction.

Diagnostics should match between the two, as the task diagnostics do: `a parallel callback may perform only the parallel-safe row ({} or {Diverge}): ...`, `a parallel capture must be Share, or move into one parBoth branch: ...`, `parMap must be applied directly to check its callback`.

## 4.2 Core form

Seed: `PrimOp::Par(ParOp::Both | ParOp::Tabulate | ParOp::MapArray)` with units `cinch.runtime.parBoth`, `cinch.runtime.parTabulate`, `cinch.runtime.parMapArray`. `parMap f xs` elaborates to `array-to-list (MapArray f (array-from-list xs))`, using the existing `cinch.runtime.arrayFromList` and the prelude's `array-to-list`, so the runtime never builds list cells.

Cinch compiler: Core constructors `CParBoth`, `CParTabulate` and `CParMap`, beside `CSpawn` (`bootstrap/Bootstrap.Core.cinch`), with effects computed like `task-spawn-effects`.

Every evaluator implements the defining equations sequentially: the seed's `eval.rs` (staging, law sampling), the REPL and JIT paths, `Bootstrap.Eval` and `Bootstrap.StageEval`.

## 4.3 Reference counting at the fork and the join

- Fork: the callback closures are built as any closure is, dup-ing their captures. `prim_consumes` (`compiler/src/codegen.rs`, read generically by `rc.rs`) reports the closure arguments as borrowed, unlike `spawn`, which consumes its body; the caller drops them after the call returns. The runtime marks at publication, never generated code.
- Branch: captures are borrowed from the closure through `__apply`, as today.
- Join: result words are owned by the caller. For `parBoth`, generated code builds the tuple at the call site with scan classes from the instantiated result types (a type parameter takes `TObj`, which covers both counted classes), so no mask and no `JOIN_SHAPED_MAX` limit apply. The array results carry the element class passed to the runtime.

In the Cinch compiler, RC is not a Core pass but part of RuntimeIr emission (`bootstrap/Bootstrap.RuntimeIr.cinch:1-4`, `count-call`, `drop-locals`); the same three rules apply there.

## 4.4 Code generation

The units call the four externs; the cooperative profile selects `cinch_coop_par_*` names through the same `task_rt` switch the task units use. RuntimeIr declares the externs in its preamble beside `@cinch_task_spawn`.

## 4.5 Interface schema and ABI

- The builtins are installed, not exported, so phase 2 needs no interface schema change for them. Exported definitions that call them have ordinary schemes.
- Interfaces that carry checked bodies (total and proof bodies, the checked import-body index) encode a parallel call as its defining equation. The proof layer only needs the meaning, and this keeps the body encoding unchanged.
- `RUNTIME_ABI_VERSION` (`compiler/src/interface.rs:30`, currently 7) bumps, since four externs are new, as S9 bumped it for marking.
- The `share` attribute of section 2.4 does change the schema (`SCHEMA_VERSION`, `interface.rs:21`, currently 10): an importer must learn that an opaque type is `Share`. Whether `Share a` constraints need a schema change depends on whether the reader must treat `Share` as compiler-known rather than as an ordinary class name; phase 0 decides it.
- Exportable parallel forwarders (a user `parFold` whose callback obligation reaches its callers through the interface) need a further schema change (section 7, phase 6, and section 8).

# 5 Using it in the self-hosted compiler

## 5.1 Where the time goes

Profile of one self-build, taken 2026-11-10: an `-O2` Stage1 compiler compiling the current compiler bundle with `--emit-ir-with-interfaces`, sampled with macOS `sample` for one second roughly every eleven seconds over the whole 585-second run (581 s real, 559 s user, peak resident set 628 MB). Each snapshot's samples were attributed to the outermost phase function on the program thread, and phase durations interpolated between snapshots, so each phase boundary is uncertain by about ten seconds. The phases are strictly sequential in time, which makes the interpolation straightforward.

| Phase | Functions | Seconds | Share | Parallel grain | Span limit |
| --- | --- | ---: | ---: | --- | --- |
| load inputs | `admit-runtime-object`, `validate-interface-set` | 12.7 | 2.2% | sequential in this plan | |
| parse | `parse-program-with`, `module-scope-fixes` | 67.1 | 11.5% | per module | largest module, `Bootstrap.Infer`, 19% of lines |
| resolve | `resolve-program-with` | 11.0 | 1.9% | sequential | |
| total bodies | `source-total-bodies` | 34.0 | 5.8% | per definition | |
| body inference | `check-definition`, `check-body-declaration` | 195.4 | 33.4% | per definition | largest definition, 86 lines |
| Core re-check | `Typing.clause-typed`, `Core.proof-equations` | 120.9 | 20.7% | per definition | |
| interface export | `export-interfaces` | 33.0 | 5.6% | per module, by wave | largest interface, `Bootstrap.Core`, 27% of bytes |
| RuntimeIr emission | `worker`, `global-tail-worker` | 78.4 | 13.4% | per unit, by frontier | |
| write output | a final pass over the IR text (`utf8-valid-from`) | 7.5 | 1.3% | sequential | |
| unattributed | | 24.9 | 4.3% | treated as sequential | |

The earlier recorded whole-build figure, 664 s with the release runtime against 4,508 s with the debug runtime (`bootstrap/current-stage1-runtime-performance-results.json`), is the same workload on an older source snapshot. Sequential improvements to the checker change these proportions; the decomposition below does not depend on them.

## 5.2 Module waves alone are not enough

The compiler's import graph has 14 dependency waves (widths 7, 3, 5, 2, 7, 2, 3, 2, 1, 1, 1, 4, 1, 1, computed from the `import Bootstrap.*` lines of the 39 `bootstrap/*.cinch` files). The longest path by source lines is `Ast`, `Report`, `Parse`, `Load`, `BodyContract`, `Core`, `Instance`, `Global`, `Typing`, `Infer`, `Check`, `RuntimeIr`, `Pipeline`: 23,030 of 34,997 lines. If checking cost tracks lines, compiling modules separately in dependency waves can never be more than 1.52 times faster, whatever N is. The parallelism is inside modules: 4,548 definitions with signatures, a median of 5 lines and the largest 86.

## 5.3 Phase by phase

**Body inference** (`check-body-loop`, `Bootstrap.Infer.cinch:4591-4600`). Today the loop threads one `Store` (`Bootstrap.Type.cinch:469-515`) through every definition: the metavariable counter `next`, the cell list, the errors, and the `seen` names. With every top-level definition carrying a signature, a body's inference needs only the module-level environment and store, so the loop becomes `parMap (check-one-body ctx) ds`. Each body starts from the module-level store with its own metavariable numbering, so ids are local to the definition and never depend on another body. The `seen` list is the prefix of declaration names, computed up front. Diagnostics keep today's behavior exactly: the loop stops at the first failing definition, so the parallel version reports the diagnostics of the first failing definition in source order and discards the rest. A definition without a signature, if one appears, is checked with its binding group sequentially before the parallel map, in dependency order. A side benefit is sequential: `find-cell` is a linear scan of every cell created so far in the module (`Bootstrap.Type.cinch:517-522`), and per-definition stores keep that list short.

**Core re-check** (`check-core-closed`, `Bootstrap.Infer.cinch:4648-4654`). Each definition's check only adds an error, so it is `parMap (definition-errors globals) definitions` followed by an in-order fold of the errors. Instance-field checks are the same shape.

**Total bodies** (`source-total-bodies`). Per definition, same pattern.

**Parsing.** The bundle is a concatenation of modules, each prefixed `-- source: NAME`. Module headers and the fixity scope are computed first; each module's body is then parsed against its own fixity table in a `parMap`, and the modules are concatenated in bundle order before `resolve-context` flattens them. `module-scope-fixes` is per module once the headers are known.

**Interface export** (`export-interfaces`, `Bootstrap.Export.cinch:935-938`). `export-loop` threads the hashes of interfaces already produced (`Bootstrap.Export.cinch:903`), because an interface names its dependencies' hashes. Export therefore runs by the waves of section 5.2: each wave's modules in a `parMap`, then the next wave. The `.cinchif` files are written afterwards, in module order.

**RuntimeIr emission.** This needs the most restructuring. Emission is demand-driven from `main`: `workers` pops demands from a worklist, emits each unit once, and enqueues the demands its body discovers (`Bootstrap.RuntimeIr.cinch:411-415`, `3035-3056`). All state lives in one `Writer` (`Bootstrap.RuntimeIr.cinch:80-94`): one monotonic counter for every fresh name, two append-only builders, the precomputed layout plan, and the `pending` and `done` queues. Three changes make it parallel without losing dead-code elimination:

1. Fresh names become unit-local. LLVM local names (`%v`, `%ptr`, `%slot`, labels) only need to be unique within a function, so each unit restarts its counter. Global names the counter currently produces (`@text`, `@number`, `@empty-ctor`, `.lambda`, `.rec`) are qualified by the unit's demand key instead, and metadata ids (`!`) are renumbered by a deterministic offset at merge.
2. Discovery runs in frontier waves. The current frontier's units are emitted with a `parMap`; each returns its code text, its globals text and the demands it discovered, in discovery order. The next frontier is the concatenation of those lists in frontier order, minus demands already seen. That is a deterministic breadth-first order, independent of N.
3. The module text is assembled in that order: the preamble, every unit's globals, the layout table, the entry glue, every unit's code. The order differs once from today's worklist order; it is then fixed.

**Load and write** stay sequential in this plan. Interface validation hashes every input (`sha256-bytes`) and could be a `parMap` later.

## 5.4 Determinism

The fixed point compares the Stage2 and Stage3 executables, the whole LLVM IR text, and every `.cinchif`, byte for byte (`check_fixed_point`, `tooling/bootstrap.py:473`). The design keeps that true for every N:

1. A primitive's result is its defining equation's value; the schedule cannot reach it.
2. No address is observable. `Sym` order is lexicographic and `Sym` hashing never reads a cell address (`compiler/prelude/Data/Sym.cinch`); nothing exposes pointer identity.
3. No counter is shared between parallel units. Each unit's fresh names come from its own identity (definition position, demand key), never from completion order.
4. Every merge is in input order: Core definitions in source order, diagnostics by source position of their unit, IR units in frontier order, interfaces in module order.
5. The first-error rule selects by source order, not by which participant failed first.

The ladder checks it directly: Stage2 is built at `-j 1` and at `-j 2, 4, 8, 16`, all five outputs must be identical, and the Stage3 that a `-j 16` Stage2 builds at `-j 1` must equal the Stage3 that a `-j 1` Stage2 builds at `-j 16`.

One thing does depend on N: peak memory. Each participant holds its own working set, so peak live heap rises with N. Suites that run under a tight `CINCH_HEAP_LIMIT` (some use 32M or 256M) must either run at `-j 1` or have their limit raised; `Exhausted` is already outside the logical model, but a suite going red at `-j 16` would read as a regression.

## 5.5 Expected speedups

Model: sequential phases cost what they cost now; each parallel phase costs the larger of its work times 1.1 divided by N and its span limit from the table in section 5.1. The 10% overhead for marking, atomic counts on shared contexts, and merges is an assumption, not a measurement.

| Workers | Seconds | Speedup |
| ---: | ---: | ---: |
| 1 | 585 | 1.0 |
| 2 | 347 | 1.7 |
| 4 | 202 | 2.9 |
| 8 | 137 | 4.3 |
| 16 | 107 | 5.5 |

With no overhead the 16-worker figure is 104 s; with 25% it is 111 s. The floor is the sequential remainder of about 56 s plus the parse and export span limits of about 22 s, so about 78 s, or 7.5 times. Parallelizing input validation and resolution would take the 16-worker figure to about 89 s. Two cautions: the model assumes identical cores, and this host has 6 performance and 12 efficiency cores (`sysctl hw.perflevel0.physicalcpu`, `hw.perflevel1.physicalcpu`), so figures above 6 workers are optimistic; and the cost of contended atomic counts on the shared checker context is unmeasured, which is why phase 5's acceptance asks for counters.

## 5.6 The Rust seed's own phases

Nothing above requires the seed to run in parallel itself. It could check bodies in parallel with `std::thread::scope` once its single mutable `Checker` (`compiler/src/check.rs:2886-2932`) is split into read-only tables and per-definition state, but that is an independent Rust refactor and not part of this plan.

# 6 Specification and reference text to amend

Not edited by this note; each needs a dated amendment when the corresponding phase lands.

- `specifications/concurrency-specification.md`: section 1 (define scoped parallel work as deterministic pure parallelism, distinct from tasks), section 3 (the capture table for parallel callbacks and publication-time promotion), section 6 (the memory model statement of section 3.5; amend D80's "no other atomics"), section 9 (sequential evaluation is the stated fallback, so no capability request is needed), section 11 (initial scope), and the Amendments list.
- `specifications/core-language-specification.md`: section 6 (the parallel-safe row), section 7 (`Share` for declared-immutable abstract types as part of "safe synchronized sharing"), "Foreign and concurrent computation" (parallel primitives belong to the pure layer through their defining equations), section 11 (`share` attributes in the acceptance manifest).
- `specifications/runtime-capabilities-effects-and-sequencing.md`: "Backend and compatibility consequences" (the pool, `CINCH_THREADS`, `parSetWorkers`).
- `specifications/lazy-computation-and-reclamation-specification.md`: section 5 and section 9 (memo cells refused as parallel captures; required rejection cases).
- `specifications/primitive-types-specification.md`: section 9 (`Sym` is `Share`, still not `Copy`).
- `specifications/modules-and-compilation-specification.md`: section 4 (interfaces record `share` attributes) and section 12 (artifacts independent of the worker count).
- `specifications/webassembly-and-wasi-specification.md`: section 7 (the cooperative backend evaluates parallel primitives sequentially).
- `reference/13-concurrency-and-runtime.md`: a new "Deterministic parallel evaluation" section, `CINCH_THREADS` in "Environment variables", and the "Implementation" pointers.
- `reference/20-compiler-vocabulary.md`: "Runtime capabilities and concurrency" lists `parBoth`, `parTabulate`, `parMap` and `parSetWorkers`.
- `reference/09-effects-and-capabilities.md`: the `Diverge` section (divergence through a parallel callback).
- `reference/10-ownership-and-memory.md`: "Captured values keep their multiplicity" and "Reclamation is code generation" (publication-time marking, borrowed closures).
- `reference/01-using-the-compiler.md` and `reference/18-artifacts-and-incremental.md`: `-j N` for the self-hosted compiler and the determinism guarantee.
- `design/reclamation.md` section 8 and `design/ownership-and-usage.md` section 3, to point here.
- `compiler/CONFORMANCE.md`: one decision number per phase.

# 7 Implementation plan

Each phase ends green on its own acceptance tests and leaves the four-stage fixed point intact.

**Phase 0: `Share` and marking prerequisites.**
Make `Sym` `Share`; add the `share` attribute, its interface field (schema 11) and its manifest entry, and put it on `Data.String.String`; add `Share a`; give every blind-spot slot kind a scan class or make its type non-`Share`.
Acceptance: a task capturing a `String` and a `Sym` is accepted, and a task capturing a type with a private `Bytes` field and no attribute is refused; for each formerly `Raw` slot kind, an `rc::tests` race in the S9 style (eight threads, 200,000 dup and drop rounds on a block reachable only through that slot) ends at the exact count under `CINCH_RC_DEBUG`, with `leaked=0`; Rust and Cinch interfaces agree on the new field.

**Phase 1: the runtime pool.**
`compiler/runtime/src/par.rs`: deques, workers, latches, publication-time marking, help-while-waiting with the stack-headroom rule, `CINCH_THREADS`, the four entries, and the cooperative versions in `coop.rs`.
Acceptance (Rust tests calling the entries with test closures): a shared 1,000-cell list read by every invocation of a 10,000-index tabulate at N of 2, 4, 8 and 16, 20 runs plain and 20 under `CINCH_RC_DEBUG`, `leaked=0`; a nested `parBoth` tree of depth 20 completes, and while it runs no worker parks with a nonempty deque anywhere (a parking counter reads zero); at N = 1 no thread is created and no block is marked; a stack overflow inside a stolen job takes exit 73 with the `stack` line; a parallel call from a task thread participates and returns; `RUNTIME_ABI_VERSION` is 8.

**Phase 2: the Rust seed.**
Schemes, application elaboration, the row and capture rules, `PrimOp::Par`, borrowed closure arguments, units, codegen, the coop lowering, sequential evaluators.
Acceptance (`tests/parallel.rs`, `cinch:run` style): `parMap` over 100,000 elements equals `map`; a nested `parBoth` Fibonacci; one program printing a checksum produces the same stdout at `CINCH_THREADS` 1, 2, 8 and 16 and on the Wasm profile; `CINCH_LEAK_CHECK=1` reports `leaked=0` for each; rejection cases: an `IO r` operation in a callback, a `Mut` capture, an untransferred `Bytes` capture, a `Lazy` capture, a `%1` capture in a `parMap` callback, the same `%1` value in both `parBoth` branches, a callback that is a function parameter, `alias = parMap`, and a rigid row variable.

### As built (phase 2)

The seed half is `install_parallel_api` (`compiler/src/check_decls.rs`), `infer_par_app` and `parallel_safe_row` (`compiler/src/check_terms.rs`), `check_parallel_captures` (`compiler/src/check.rs`), `PrimOp::Par` (`compiler/src/core.rs`), the unit bodies and the conditional extern declarations (`compiler/src/codegen.rs`), and the suite `compiler/tests/parallel.rs`. What the sections above left open, and how it was decided:

- **Rows per branch.** `parBoth` gives each branch its own row variable, not the single `e` of section 2.1. Rows unify by equality, so one shared variable would refuse a total branch beside a diverging one (`{}` never equals `{Diverge}`), which the defining equation admits. The call performs each branch's row, which is what evaluating `(f (), g ())` does. `parTabulate` and `parMap` keep the single `e` their schemes state.
- **Unit identity carries the classes.** `PrimOp::Par { op, classes }`: the pair's two component classes, or the result array's element class. Each is an immediate in the unit body, so the unit name carries the vector (`cinch.runtime.parBoth@RO`, `cinch.runtime.parTabulate@T`), exactly as a specialized unit's name does (`repr::spec_name`). `subst_classes` substitutes them in a generic definition's specialized copies. The tuple is built in the unit from `reg.shape_desc(&classes, 0)`, so no join mask and no `JOIN_SHAPED_MAX` limit apply (section 4.3).
- **`parMap` needs `Data.Array` in the program.** The design's lowering uses `array-from-list` and `array-to-list`, and `Data.Array` is a source module: it is not in the embedded base package, so a program calling `parMap` ships `prelude/Data/Array.cinch` beside its entry. Absent that, the refusal names the requirement. `parTabulate` needs nothing: it returns the `Array` directly. Making `parMap` module-free is the later step that would put the two conversions in a `cinch/prim` module (section 8).
- **Element-type check is deferred.** `parMap`'s element type is often an unsolved metavariable at the call (a list literal of polymorphic literals), so the `Share` check runs at the definition's end beside the capture classification. An element type still undetermined there is refused: nothing is known to be shareable.
- **Publication-time marking is the only marking.** Generated code marks nothing: `prim_consumes` reports all three units' closure arguments as borrowed, the caller drops them after the call, and the runtime duplicates one reference per invocation and marks at the first stealable publication (section 3.3). The `rc` pass needs no change beyond that table entry.
- **The parallel callback is not a borrowed environment.** A callback runs inside its call and is dropped before the call returns, so the conservative borrowed-capture rule (core specification section 7) does not apply to it: `check_parallel_captures` classifies what it reads instead. A literal `parBoth` branch is a one-shot frame, so a `%1` capture used once moves into it and `note_use` counts one use; a second branch using the same value is the existing `used 2 time(s)`.
- **Compile-time evaluation.** The staging evaluator runs `parBoth` as `(f (), g ())`. `parTabulate` and `parMap` refuse there, with the array primitives they lower through: the compile-time value model has no array.
- **The ABI bump.** `RUNTIME_ABI_VERSION` is 8 and its doc comment states the four entries. Still to move, outside this phase's files: `RUNTIME_ABIS` in `tooling/bootstrap.py` (5, 6, 7 to 5, 6, 7, 8), `bootstrap/Bootstrap.RuntimeAbi.cinch`, `bootstrap/library-provider.lock.json` and `bootstrap/stage0.lock.json`, all of which the pinned seed and the ladder own.
- **Byte identity.** The extern declarations for the four entries are emitted only when the program contains a `PrimOp::Par`, so a program using none of the builtins emits identical IR (checked byte for byte against the previous compiler on a 6.6 MB module).
- **Scaling measured here.** `parTabulate` over 64 items of U64 arithmetic (2,000,000 loop steps each), best of three runs per count, `CINCH_RC_DEBUG=1` and `CINCH_LEAK_CHECK=1`: N = 1 0.245 s, N = 2 0.125 s (1.96x), N = 4 0.068 s (3.59x), N = 8 0.038 s (6.41x). Single readings on this host vary by up to a factor of two because other work shares it, which is why the test reports the minimum of three.
- **Determinism and reclamation.** One program checks `parMap` against `map` over 100,000 elements, `parTabulate` against `array-tabulate` over 50,000, and nested `parBoth` Fibonacci against the sequential reading, and prints one byte per check. Its stdout is `AAA` and its leak report is `live=0 immortal=0 leaked=0` at N = 1, 2, 4, 8 and 16.

Phase 3 mirrors these decisions in the Cinch compiler: `ParBoth`, `ParTabulate` and `ParMap` builtins with `first_class` false; the same three refusals with the same text; per-branch rows for `parBoth`; the class vector in the unit name; the `Data.Array` lowering for `parMap`; and the ABI 8 extern names through the same `task_rt`-style switch.

**Phase 3: the Cinch compiler implements the primitives.**
Builtin entries, Infer and Typing rules, the Core constructors, RuntimeIr lowering, export of bodies through the defining equations, sequential `Eval` and `StageEval`. The compiler's own source does not use the primitives yet.
Acceptance: every phase 2 program compiles through Stage1 with identical output and identical refusal text; Native and Cinch agree on the emitted interfaces of a module that uses the primitives; the four-stage fixed point holds; the pinned seed moves to the phase 2 commit.

**Phase 4: determinism preparation, still sequential.**
Per-definition stores with local metavariable numbering; unit-local RuntimeIr counters, qualified globals and metadata renumbering; frontier-order emission; per-module parsing; wave-ordered export. Everything uses the sequential `map` in the places that will become `parMap`.
Acceptance: the fixed point holds; every suite in the current Stage3 matrix passes; the IR differs from the previous compiler's only in names and unit order (checked by a normalizing diff), and diagnostics are byte-identical on the refusal fixtures; the sequential build time is recorded against the 585-second baseline.

**Phase 5: the compiler goes parallel.**
Replace those `map`s with `parMap`; add `-j N` to the driver and pass it from `tooling/bootstrap.py`.
Acceptance: Stage2 at `-j 1, 2, 4, 8, 16` gives byte-identical IR, executables and interfaces, and the cross-N Stage3 check of section 5.4 passes; the self-build at `-j 8` takes at most 200 s on this host, with wall time, user time, peak RSS and the number of `cinch_dup_mt` and `cinch_drop_mt` calls (a debug counter) reported per N; the full Stage3 suite passes at the default N, or the suites whose heap limit cannot cover the parallel peak are pinned to `-j 1` and listed.

**Phase 6: later.**
Callback obligations that travel through interfaces, so user code can define parallel combinators: a `Share`-capturing function type, or exportable forwarders, with a schema change. A row bound that admits rigid row variables known to be parallel-safe. Error pruning for `Result`-valued maps. Heartbeat-style promotion of latent parallelism. Parallel `clang` over several IR modules, which is I/O and belongs to tasks; it needs its own measurement because `clang -O2` on the 47 MB compiler IR is not part of the profile above.

# 8 Open decisions

- Whether the `share` attribute stays a trusted assertion or the checker verifies it, for example by checking that no function of the declaring module passes a buffer reachable from a constructed value to a writing primitive.
- How user-defined parallel combinators carry their callback obligation: a `Share` qualifier on arrows checked at closure creation (closest to the specification's `Send`-like and `Sync`-like markers), or forwarder obligations in the interface (closest to the existing source-only task forwarders).
- Whether the three builtins move to an ordinary `cinch/prim` module once obligations can travel, as `design/base-as-a-package.md` wants for installed values.
- The default N on asymmetric hosts: all cores, or performance cores only.
- Whether `parSetWorkers` should exist at all, or the budget should come only from the environment and the command line handled by the runtime.
- Whether contention on marked shared contexts needs a remedy beyond borrowed captures, such as an explicit freeze that makes a context immortal for the rest of the process.

[Documentation overview](../overview.md)
