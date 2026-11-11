# Software transactional memory: `Effect.Stm`

Implementation design for section 7 of
`specifications/concurrency-specification.md` (normative). Status:
implemented, 2026-11-11. This note covers where each part lives, the
algorithm, the two rules that make a mutable counted slot sound under the
reference-counting runtime, and what was considered and left out. No em
dashes.

## 1 The split: algorithm in source, three things in the runtime

The transaction algorithm is Cinch source, `compiler/prelude/Effect/Stm.cinch`,
written over `prim` declarations whose bodies are inline LLVM IR
(`design/inline-target-ir.md`). The runtime (`compiler/runtime/src/stm.rs`)
owns only what source cannot express:

| Piece | Where | Why it cannot be source |
| --- | --- | --- |
| The cell: a block whose scanned slot is rewritten in place | `block_kinds.rs` `KIND_TVAR`, `block.rs` scan arm | Every other block's scanned slots are written once; the drop and mark walks must know this kind |
| The global version clock and the sleeper count | `stm.rs` `STATE`, reached through `cinch_stm_state` | One process-wide address, padded so the two words sit on separate cache lines |
| Publication | `cinch_stm_publish` | Marking a value shared is the runtime's mark walk |
| Waiting | `cinch_stm_wait_begin`, `_park`, `_cancel`, `cinch_stm_wake` | Suspension belongs to the scheduler, behind `park.rs` |

The compiler knows one type and five intrinsics (`compiler/src/stm.rs`):
the builtin opaque `TVar a`, and `stmCellNew`, `stmCellAddr`,
`stmCellHold`, `stmCellOpen`, `stmCellCast`. They are imported by
`Effect.Stm` and do not leave it. Everything a program can name
(`STM`, `atomically`, `retry`, `or-else`, the `tvar-*` operations) is
ordinary exported source.

The reason for the split is the usual one for this codebase: source is
checked, specialized and inlined by the same compiler as the code that
calls it, so a transaction over a `TVar Nat` compiles to loads, a
compare-and-swap and a store with no call into the runtime on the paths
that do not wait. A runtime implementation would cross the C ABI on every
read.

## 2 The cell and its lock word

A `TVar a` is a `KIND_TVAR` block of three words:

| Word | Name | Content |
| --- | --- | --- |
| 0 | lock | bit 0 the write lock, bit 1 the reader pin, the version above them (versions are multiples of 4) |
| 1 | value | the current value, one ABI word |
| 2 | format | the value word's scan code (`SCAN_RAW`, `SCAN_OBJ`, `SCAN_TOBJ`), fixed at creation |

The format word is what lets one block kind hold any value type: the
compiler passes the scan code of the specialized value class to
`cinch_stm_cell_new`, and the runtime's walks read it back. Stm units
are specialized by class vector like any other representation-polymorphic
primitive (`specialize.rs`).

A `TVar` handle is `Copy`. Sharing one across a task boundary requires
its value type to be `Share` (`check.rs::structural_usage`): a variable
is a channel, so what can be put in it must be able to cross.

## 3 The algorithm

TL2 (Dice, Shalev and Shavit, 2006) with snapshot extension.

- **Begin.** Read the global clock: the attempt's snapshot version.
- **Read.** Load the lock word, the value, and the lock word again. The
  value belongs to the snapshot when the word did not change, is not
  locked, and its version is no newer than the snapshot. A newer version
  does not abandon the attempt at once: the attempt reads the clock
  again and revalidates what it has read so far, which moves the
  snapshot forward. Reads are recorded with the version seen.
- **Write.** Stage the value in a private cell paired with the target.
  A later read of the same variable in the same attempt finds the staged
  value. Nothing shared is touched.
- **Commit.** Lock the written variables (a compare-and-swap from an
  unlocked, unpinned word), advance the clock, validate the read set
  against the snapshot (skipped when the new clock value is exactly one
  step past the snapshot, since then no other commit intervened),
  install the staged values, and release each lock by storing the new
  version. A read-only attempt commits with no shared write.
- **Abandon.** A lock that is not free after a bounded spin, or a failed
  validation, abandons the attempt: locks taken so far are released at
  their old versions, the log is dropped, and the attempt runs again
  after a randomized backoff (spin, then yield).

Nothing blocks while holding a lock, so a suspended or descheduled
committer delays others by at most its own commit sequence.

What this gives: serializability in commit-version order, and opacity
(an attempt that will not commit still only observes a snapshot some
serial order could have produced, so a body cannot loop or fault on a
combination of values that never coexisted). What it does not give:
fairness or a bound on attempts. A long transaction can be overtaken
indefinitely by short ones, which is why `atomically` carries `Diverge`.

## 4 Counted values in a mutable slot: the two rules

The runtime is reference counted with a mark-on-share rule: a block
reachable from one thread has a plain count, a block reachable from many
has an atomic one, and the walks stop at an atomic block because
everything below it is already atomic (`design/reclamation.md`). A slot
that is rewritten after its block has been shared breaks both
assumptions unless two rules hold.

1. **A marked cell holds only marked values.** Before commit installs a
   value into a cell that is shared, it calls `cinch_stm_publish` on the
   staged value, while the value is still private to the committer. An
   unmarked cell is reachable from one thread only and may hold
   anything. Commit repeats the publication pass until a pass marks
   nothing, because publishing a value that holds an unmarked `TVar`
   written in the same transaction shares that cell too.
2. **A value word is replaced only under the write lock, and a
   reference to it is taken only under the pin.** A reader that loaded
   the pointer and counted it afterwards could count a block a writer
   had already released. The reader sets the pin bit around the load and
   the increment; the writer's compare-and-swap succeeds only from a
   word with neither bit set. A value that needs no count (a machine
   scalar, an immediate `Nat`) is read under the version check alone,
   with no shared write.

Old values leave by swap: commit exchanges each staged value with the
value it replaces, so the replaced value sits in the staging cell and is
released with the attempt's log after the locks are gone. No value is
freed while a lock is held.

The log is one list for variables of every type, so its cells are held
as `TVar Erased`. `stmCellCast` and `stmCellOpen` are the two unchecked
steps in the module, and the invariant that makes them sound is local:
a staged cell is created paired with its target and is opened only
through that target's type.

`compiler/tests/stm.rs` runs every program under `CINCH_LEAK_CHECK=1` in
both a plain and an `rc_debug` build (where a count operation on a freed
block traps), which is the executable form of these two rules.

## 5 Waiting

`retry` abandons the attempt and must not run again until a variable it
read changes. The attempt summarizes its read set as a 64-bit filter
(one bit per variable, from the cell address) and then:

1. `cinch_stm_wait_begin` registers the calling context with the filter
   and increments the sleeper count.
2. The prelude validates its read set once more. If something already
   changed, `cinch_stm_wait_cancel` and run again.
3. Otherwise `cinch_stm_wait_park` suspends until woken.

A writing commit releases its locks, then loads the sleeper count, and
when it is not zero calls `cinch_stm_wake` with the filter of what it
wrote. Both sides store before they load with sequentially consistent
accesses (the waiter registers then validates; the committer publishes
then checks for sleepers), so one of them always sees the other: no lost
wakeup. A filter collision wakes a waiter that revalidates and sleeps
again, which costs time and not correctness.

`or-else a b` runs `a` in a nested log; when `a` retries, its writes are
discarded, its reads are kept (so the wait covers both branches), and
`b` runs.

Suspension goes through `park.rs`: `park()` blocks the current context
until its `Parker` is unparked, with token semantics so an unpark before
the park is not lost. Cancelling a task unparks it, and
`atomically-or-cancel` answers `None` when the wake was a cancellation.
The seam is what lets the scheduler (`design/green-threads.md`) replace
"block the thread" with "suspend the task" without `Effect.Stm`
changing.

## 6 The surface

```
atomically           :: STM a ->{STM, Diverge} a
atomically-or-cancel :: using (io :: Io r) => STM a ->{STM, Cancel r, Diverge} Option a
retry                :: STM a
or-else              :: STM a -> STM a -> STM a
check                :: Bool -> STM Unit
tvar-new             :: a -> STM (TVar a)
tvar-read            :: TVar a -> STM a
tvar-write           :: TVar a -> a -> STM Unit
tvar-modify          :: TVar a -> (a -> a) -> STM Unit
tvar-swap            :: TVar a -> a -> STM a
tvar-state           :: TVar a -> (a -> (b, a)) -> STM b
tvar-new-io          :: a ->{STM} TVar a
tvar-read-io         :: TVar a ->{STM} a
```

`STM a` is a data type whose steps are pure functions, with `Functor`,
`Applicative`, `Monad` and `Alternative` instances (`empty` is `retry`,
`<|>` is `or-else`). Purity is the enforcement of "no I/O in a
transaction": a step that performed I/O would have an effect row and
would not typecheck as a step. No separate rule exists.

`STM` is also an effect label: `atomically` and the two `-io` shortcuts
carry `{STM}`, so a signature shows that a function runs transactions.
It is a plain closed label. Discharging it with a handler was tried and
refused by the checker ("the handled effect is performed through a
called computation"), and there is nothing to interpret: the label is
documentation the checker propagates.

`do` works through a compiler-known sequencing provider,
`Effect.Stm.StmSequencing`, selected the same way as the `Result` and
`Option` providers (`using (seq :: StmSequencing)`). Its continuation
kind is `ContKind::Many`: the function to the right of `<-` may run once
per attempt, so its captures count as a lambda's and a `%1` value cannot
cross a `<-`. That is the rollback-safe ownership discipline the
specification asks for, obtained from the existing linearity check.

The derived structures are plain source over the above:
`Effect.Stm.TMVar`, `Effect.Stm.TQueue` (two lists, so writers and
readers contend on different variables), `Effect.Stm.TBQueue` and
`Effect.Stm.TSem`.

## 7 Target specifics

The atomic operations are LLVM `cmpxchg`, `atomicrmw` and atomic
`load`/`store`, so each target gets its own instructions: `lock cmpxchg`
and `lock xadd` on x86_64, `casal` and `ldaddal` on Apple silicon, the
`ldaxr`/`stlxr` pair on baseline aarch64 Linux. Two operations have no
portable spelling and carry a body per architecture with a portable
fallback:

| Prim | x86_64 | aarch64 | Fallback |
| --- | --- | --- | --- |
| `cpu-relax` | `pause` | `isb` | nothing |
| `cpu-ticks` | `rdtsc` | `mrs cntvct_el0` | its argument (the attempt number), so backoff grows but is not randomized |

`isb` rather than `yield` on aarch64 because `yield` is a no-op on most
cores and `isb` gives a delay comparable to `pause`.

Hardware transactional memory is absent on purpose. Intel has disabled
TSX by microcode on shipping processors and no Arm core implements TME,
so an HTM commit path could not be tested here and would rarely run
anywhere.

On a wasm target there is one thread: a `retry` drains cooperative work
and, when nothing is runnable, stops with "transaction blocked: `retry`
waits for a variable no runnable task can change".

## 8 Not done, and why

- **Fairness.** No queueing of starved transactions and no fallback to a
  global lock after N attempts. Either is addable inside `atomically`
  without changing the surface.
- **Invariants (`always`), `unsafeIOToSTM`.** Not provided. The second
  would defeat the purity argument of section 6.
- **Per-variable wait queues.** The global registry with a filter is
  simpler and cheap while few transactions sleep; a commit pays one load
  when none do.
- **A home module for `TVar`.** The type is installed by the checker,
  like `Array`, `Tensor` and `Columnar`, and has no declaring module.
  The source form that would declare it does not exist: `opaque type`
  means a foreign handle, an uncounted word the foreign side owns, and
  a cell is a counted runtime object. When runtime-owned types get a
  declaration form and home modules, `TVar` is declared in
  `Effect.Stm` and the checker refers to it by that identity.
