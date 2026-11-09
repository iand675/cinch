# Allocators and heap customization

A compiled Cinch program allocates every value block through one accounting layer that wraps the system allocator. That layer is what exists today: it counts live bytes, enforces the heap cap, reports allocation totals, and reports leaks. Program allocators, scoped heaps, arenas, pools and proven allocation budgets are a design decision with no shipped API, and this page keeps them behind that boundary.

[Ownership and reclamation](10-ownership-and-memory.md) governs how values may be used and when their storage is released. This page governs where the storage comes from, and how to observe it.

# What exists today

| Fact | Today |
| --- | --- |
| Where blocks come from | The runtime installs `Counting`, a `GlobalAlloc` over `System`, as `#[global_allocator]` (`compiler/runtime/src/heap.rs`). Every allocation in the process passes through it, including the runtime's own limbs, byte buffers and handle tables. |
| What is counted | Cumulative blocks and bytes, live bytes with a high-water mark, and Cinch value blocks separated from the runtime's own allocations. |
| Reclamation | Reference counting with reuse (`compiler/runtime/src/rc.rs`), inserted by `compiler/src/rc.rs` during code generation. |
| Cap | 8 GiB live bytes by default, adjustable with `CINCH_HEAP_LIMIT`. |
| Selection of a different allocator | Not available. There is no `#[global-allocator]` attribute and no `Heap` capability in the type checker. |

Allocation for a generated program is therefore uniform, and every measurement below is available without opting into anything: the counters exist whether or not you print them.

# Environment variables

Each variable is read by a **generated program at startup**, except `CINCH_RECLAIM`, which the **compiler** reads while generating code, and `CINCH_RC_TRACE`, which a debug build of the compiler reads when the ownership verifier rejects a function. The runtime variables belong to a linked executable, and none of the seven is a `cinch` subcommand flag.

| Variable | Format accepted | Read by | Effect |
| --- | --- | --- | --- |
| `CINCH_ALLOC_STATS` | exactly `1` | Generated program, at first allocation | Register an `atexit` handler that writes `cinch-alloc: ...` to stderr. |
| `CINCH_LEAK_CHECK` | exactly `1` | Generated program, at first allocation | After `main` returns, write `cinch-leak: ...` and return exit code 74 when mortal blocks are still live. |
| `CINCH_RC_DEBUG` | exactly `1` | Generated program, at first allocation | Poison and quarantine freed blocks, and validate headers on every count operation. |
| `CINCH_HEAP_LIMIT` | `<digits>` or `<digits>` with `K`, `M` or `G` | Generated program, at first allocation | Cap live bytes. Unset means 8 GiB, `0` disables the cap, an unparsable value falls back to 8 GiB. |
| `CINCH_STACK_SIZE` | `<digits>` or `<digits>` with `K`, `M` or `G` | Generated program, before the program thread starts | Reserved stack for the program thread. Unset means 1 GiB; zero or unparsable keeps the default; a value below 256 KiB is raised to 256 KiB. |
| `CINCH_RECLAIM` | `0` disables, anything else enables | The compiler, while generating code | Stop inserting `cinch_dup` and `cinch_drop`. Intended for comparisons and baselines. |
| `CINCH_RC_TRACE` | any value | The compiler, only in a debug build and only when the ownership verifier rejects a function | Write `cinch-rc: <function>` with the verifier's input and output to standard error before the `compiler bug: ownership verifier rejected ...` message. It changes nothing about the program; it is how to report that failure with a reproducer inside it. |

Suffixes use binary multiples. The truth of each variable is the presence of the exact byte `1` for the three diagnostic switches: `CINCH_LEAK_CHECK=on` does nothing.

Two related compiler-level attributes are implemented and enforced at the definition: `#[no-alloc]` refuses any admitted allocation site in the definition, and `#[alloc("once")]` bounds it to one allocation site per call. See [safety assertions](22-safety-assertions-and-llvm.md).

# Allocation totals

`CINCH_ALLOC_STATS=1` writes one line at process exit:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Data.List

  build :: Int -> List Int -> List Int
  build n acc = case n == 0 of
    True -> acc
    False -> build (n - 1) (Cons n acc)

  main :: Unit -> Int
  main u = case length (build 1000 Nil) == 1000 of
    True -> 0
    False -> 1
```

A program built and run without a variable prints nothing and exits 0. With statistics on, the same binary prints:

```text
$ CINCH_ALLOC_STATS=1 ./example
cinch-alloc: blocks=1922 bytes=48862 peak=24126 header-blocks=1916 header-bytes=48648 immortal=0 freed=1916 live=0
```

The fields mean:

| Field | Meaning |
| --- | --- |
| `blocks`, `bytes` | Cumulative allocations and bytes over the whole run. |
| `peak` | Highest live byte count reached. |
| `header-blocks`, `header-bytes` | Cinch value blocks, and their bytes including the 8-byte header. These are a subset of `blocks`. |
| `immortal` | Blocks born uncounted: interned `Sym` cells. Static blocks live in the data section and are never allocated. |
| `freed` | Header blocks released by reclamation. |
| `live` | `header-blocks - freed`. |

`blocks - header-blocks` is the runtime's own allocation (bignum limbs, buffer data, handle tables). The list above is 1000 `Cons` cells, which are a subset of those 1916 header blocks; the rest are the other value blocks the run allocated. `freed` equals `header-blocks` because everything the run allocated has died by the time `main` returns.

The line is written by an `atexit` handler registered on the first allocation, so it also appears after a normal exit, after a heap exhaustion, and after a panic fault. It does not appear after stack exhaustion, which leaves through `_exit`.

# Leak checking

`CINCH_LEAK_CHECK=1` writes one line after `main` returns:

```text
$ CINCH_LEAK_CHECK=1 ./example
cinch-leak: live=0 immortal=0 leaked=0
```

`live` counts blocks still allocated, `immortal` counts the uncounted ones, and `leaked` is `live - immortal`. A nonzero `leaked` turns the process exit code into 74:

```text
$ CINCH_RECLAIM=0 cinch build Main.cinch -o example-no-reclaim
$ CINCH_LEAK_CHECK=1 ./example-no-reclaim
cinch-leak: live=1916 immortal=0 leaked=1916
$ echo $?
74
```

That is the practical difference between the checker and the runtime. `CINCH_RECLAIM=0` does not change what the usage checker accepts: the program still checks, and it still must not duplicate a `%1` value. What changes is code generation, which stops inserting the retains and releases, so every block the program builds stays live. The same binary under `CINCH_ALLOC_STATS=1` shows the difference in the counters:

| Build | `peak` | `freed` | `live` |
| --- | --- | --- | --- |
| Default | 24126 | 1916 | 0 |
| `CINCH_RECLAIM=0` | 48795 | 0 | 1916 |

74 is a diagnostic code for a failed leak check, not a program fault. The program fault codes are 70 for `BlackHole`, 71 for the generic fault path, and 73 for `Exhausted`.

# Reference-count debugging

`CINCH_RC_DEBUG=1` makes `cinch_free` poison and quarantine each freed block and makes every count operation validate the header it reads. A correct program prints nothing and exits 0. Because quarantined blocks are never returned to the allocator, the counters show the retained memory:

```text
$ CINCH_RC_DEBUG=1 CINCH_ALLOC_STATS=1 ./example
cinch-alloc: blocks=1922 bytes=48862 peak=48790 header-blocks=1916 header-bytes=48648 immortal=0 freed=1916 live=0
```

`peak` is 48790 instead of 24126 for the same allocation totals: the same bytes stayed held. The traps this mode adds fire only on a compiler or runtime bug, such as a use after free or a corrupt header, and they take the fault exit paths rather than becoming program errors.

Compare performance and memory with `CINCH_RC_DEBUG` and `CINCH_LEAK_CHECK` unset. Neither changes the program's result; both change what the process holds.

# The heap cap

`CINCH_HEAP_LIMIT` bounds **live** bytes, the bytes allocated and not yet freed. A program that frees what it is done with runs to completion however much it allocates over its life; a program that keeps every allocation live stops at the cap. This recursion allocates a 1 MiB buffer per step and keeps every one:

<!-- cinch:run exit=73 -->
```cinch
module Main where
  import Data.List

  grow :: Int -> List Bytes -> Int
  grow n acc = grow (n + 1) (Cons (bytes-new 1048576) acc)

  main :: Unit -> Int
  main u = grow 0 Nil
```

The documentation checker runs each example with `CINCH_HEAP_LIMIT=512M` in the environment, which is why the marker above expects exit 73. Built and run by hand under an 8 MiB cap, the same program prints:

```text
$ CINCH_HEAP_LIMIT=8M ./example
cinch: exhausted: heap (8389071 bytes)
$ echo $?
73
```

8389071 is the live heap the refused allocation would have produced, so it sits one buffer past the cap. `Exhausted` is a declared outcome of every program at `main`, and 73 is one code for every resource: the stderr line names which resource ran out.

With statistics enabled, the `atexit` line still appears after the exhaustion line, which is what makes a cap failure inspectable:

```text
$ CINCH_HEAP_LIMIT=8M CINCH_ALLOC_STATS=1 ./example
cinch: exhausted: heap (8389071 bytes)
cinch-alloc: blocks=30 bytes=8389207 peak=8389135 header-blocks=16 header-bytes=384 immortal=0 freed=2 live=14
```

Sixteen `Cons` cells are live at the cap, `freed=2` records the blocks reclamation already released along the way, and `bytes` exceeds `peak` because the run allocated and freed repeatedly on its way to the limit.

`CINCH_HEAP_LIMIT=0` removes the cap, so the same program keeps running past the point where 8 MiB stopped it and never reaches a defined outcome. An unparsable value falls back to the 8 GiB default rather than disabling the cap.

A failed system allocation, and an allocation request too large to represent as a size, take the same exit with the same message. The process never aborts through the Rust allocator's failure path.

# Stack size

Every generated program runs its code on a thread with a large reserved stack, 1 GiB by default, so deep recursion has room. `CINCH_STACK_SIZE` changes the reservation.

The stack's low end is made an inaccessible guard band, and a fault inside it becomes the defined `Exhausted` outcome for the stack rather than a crash. A non-tail recursion that needs more stack than the reservation gets:

```text
$ CINCH_STACK_SIZE=256K ./deep
cinch: exhausted: stack
$ echo $?
73
```

The same binary with the default reservation exits 0. `CINCH_STACK_SIZE=64K` also prints `cinch: exhausted: stack`: values below 256 KiB are raised to 256 KiB, because the guard band alone is 64 KiB and the runtime's own frames need the rest. `CINCH_STACK_SIZE=banana` keeps the 1 GiB default, and the run completes.

Stack exhaustion leaves through `_exit`, so the `CINCH_ALLOC_STATS` line does not follow it. A fault outside the guard band is not treated as exhaustion at all: the handler restores the previous disposition and the process ends as it otherwise would.

# Where the header widths live

Block bookkeeping is one header word at byte offset -8: a 40-bit signed count in the low bits, an 8-bit kind, and a 16-bit layout id. A negative count marks a block shared across tasks, and its increments and decrements are atomic. A count of zero marks an immortal block.

Those widths are named constants in `compiler/runtime/src/block_kinds.rs`, which the runtime and the code generator both include, so every shift and mask derives from them. The same file states the cap relation: the heap cannot hold `RC_MAX` words, so a count never saturates and needs no overflow check. A build that traded count width for a wider layout id would change the runtime ABI and its cap together. Selecting a different allocation policy alone would not.

# Planned: replaceable program allocators

<span class="support-tag support-tag--planned">Coming soon</span> The [allocator design](../design/allocators.md) selects a replaceable program allocator with an in-house small-object allocator as the default. Nothing in the following list is callable today.

| Planned facility | Intended behaviour | Current boundary |
| --- | --- | --- |
| In-house small-object default allocator | Size classes for small blocks, per-thread heaps, free lists sharded per page, allocator identity recovered from the address. | The runtime wraps `System`. |
| `#[global-allocator]`-style selection | Install a Cinch-written or C-ABI allocator as the ambient program allocator. | Unknown attribute: `` unknown attribute `global-allocator`: outside the `meta` namespace, an attribute name must be compiler-known or declared by a macro ``. |
| Allocator callbacks | Allocate, free, and optional resize, with explicit size and alignment. | No runtime allocator interface and no ABI for one. |
| System backend as a choice | Keep `malloc` for debugging and comparison. | This is the only backend. |
| Static startup pool | Claim all memory before `main`, sized from a proven bound. | Needs the proof machinery below and is not implemented. |

Accounting and the cap are designed to wrap whichever allocator is installed, so a replacement keeps the measurements and the failure mode rather than each allocator implementing its own reporting.

Integrity rules the design states for a future implementation: a Cinch-written allocator must itself satisfy `#[no-alloc]`, because implementing allocation cannot allocate through the allocator being implemented; every free must return to the originating allocator; and a block shared across tasks can be released by whichever task drops the last reference, so release policy cannot assume release happens in the allocating task.

# Planned: scoped heaps, arenas, pools and budgets

<span class="support-tag support-tag--planned">Coming soon</span> The design describes four layers above the program allocator. Each is a proposal, with no accepted spelling in the language today.

| Planned facility | Intended behaviour | Current boundary |
| --- | --- | --- |
| `Heap r` capability | Allocation context supplied like I/O: `using (heap :: Heap r) =>` in a signature, `with (heap = arena) in e` at a call site. | `Heap` is not a known signature: `` cinch: unknown signature Heap ``. |
| Arena via capability | Bump allocation in one scope, released together at the end. | Not implemented; needs the capability. |
| Arena escape checking | No arena pointer may outlive its scope; a returned value must contain no arena pointer, and copy-out is explicit. | Region escape checking exists for borrows, cells and `Eval` labels; it does not yet see allocation lifetimes. |
| Counted pool | Reserved blocks for one size, returned individually to the pool's free list when their last owner releases them. | Not implemented. |
| Budget boundary | A typed allowance that turns exhaustion into a declared outcome inside a larger service. | The only boundary today is the process itself, which exits 73. |
| `evacuate`-style copy-out | Deep copy a value from an inner heap into a named destination heap. | Provisional name and signature; the design notes call it `evacuate`. |

The name `bump-arena`, the `mebibytes` helper, and the exact inference and display rules for `Heap` requirements are all open in the design notes. Use the design page as the source for these sketches, not as a description of the current compiler.

# Planned: proven allocation bounds

<span class="support-tag support-tag--planned">Coming soon</span> The `jpl` profile already enforces recursion restrictions, and two allocation assertions are implemented. The rest of the intended bound discipline is not.

| Assertion | Status |
| --- | --- |
| `#[no-alloc]` | Implemented and enforced: `` `#[no-alloc]` on Main.build asserts the no-allocation rule (JPL rule 3): the constructor `Cons` is constructed here ``. |
| `#[alloc("once")]` | Implemented and enforced: one allocation site per call. |
| `#[fip]` | Not implemented: `` unknown attribute `fip` ``. |
| `#[alloc("bounded(n)")]` | Not implemented: `` `#[alloc]` takes `"once"` ``. |
| Startup pool sized from proven bounds | Not implemented; needs the bound machinery. |

Fully-in-place execution is intended to connect the allocation guarantee to reuse analysis: a function that consumes uniquely owned cells reuses their storage for compatible replacements, so a functional transformation can run inside reserved storage without a fresh allocation per value. Functional syntax alone does not establish that guarantee, and the current checker does not accept `#[fip]` at all.

# Source authority

Implemented behaviour: `compiler/runtime/src/heap.rs` (accounting, cap, stats line), `rc.rs` (counts, leak check, debug poisoning), `block.rs` and `block_kinds.rs` (headers, allocation, free), `abi.rs` (exit codes 70, 71, 73, 74 and the `Exhausted` message), `platform.rs` and `stack.rs` (program thread and stack size), `compiler/src/rc.rs` and `core.rs` (`CINCH_RECLAIM`). Behaviour suites: `compiler/tests/alloc_accounting.rs`, `exhaustion.rs`, `reclamation.rs`, `reclamation_tasks.rs`. Planned direction: [allocator design notes](../design/allocators.md), [reclamation](../design/reclamation.md), [resource exhaustion](../design/no-faults.md), [proof automation](../design/proof-automation.md).
