# Allocators and heap customization

Every compiled Cinch program allocates through a single accounting layer (`Counting`) wrapping the operating system's allocator. This layer tracks cumulative and live bytes, enforces a hard live-heap cap, reports allocation statistics and leaks on exit, and supports reference-count debugging.

[Ownership and reclamation](10-ownership-and-memory.md) defines value usage and when blocks are freed; this chapter covers where storage comes from, how to bound and observe memory usage at runtime, and what custom-allocator capabilities are planned.

# Current runtime architecture

| Aspect | Behaviour |
| --- | --- |
| Backing allocator | A `Counting` global allocator wrapping the system allocator. Every process allocation passes through it—Cinch value blocks, bignum limbs, byte buffers, and runtime tables alike. |
| Accounting | Always active with negligible overhead: tracks cumulative blocks/bytes, live bytes, peak live bytes, and Cinch value blocks separately from internal runtime allocations. |
| Reclamation | Automatic reference counting with in-place reuse, inserted during code generation. |
| Default limits | **8 GiB** live heap cap (`CINCH_HEAP_LIMIT`) and **1 GiB** program thread stack reservation (`CINCH_STACK_SIZE`). |
| Custom allocators | Not yet implemented; there is no `#[global-allocator]` attribute or `Heap` capability. |

# Environment variables

The five runtime variables below are read by any **compiled Cinch executable** at startup (not by the `cinch` compiler CLI itself):

| Variable | Accepted format | Default | Effect |
| --- | --- | --- | --- |
| `CINCH_HEAP_LIMIT` | `<digits>[K\|M\|G]` (binary multiples) or `0` | `8G` (8 GiB) | Caps live bytes. `0` disables the cap; unparsable values fall back to `8G`. Exceeding the cap prints `cinch: exhausted: heap (...)` and exits **73**. |
| `CINCH_STACK_SIZE` | `<digits>[K\|M\|G]` (binary multiples) | `1G` (1 GiB) | Reserved stack for the program thread. Values below `256K` are clamped up to `256K`; `0` or unparsable values keep `1G`. Overflowing into the guard band exits **73**. |
| `CINCH_ALLOC_STATS` | Exact byte `1` | off | Registers an `atexit` handler that writes a single `cinch-alloc: ...` summary line to stderr. |
| `CINCH_LEAK_CHECK` | Exact byte `1` | off | After `main` returns, writes `cinch-leak: ...` to stderr and exits **74** if any mortal value blocks remain live. |
| `CINCH_RC_DEBUG` | Exact byte `1` | off | Poisons and quarantines freed blocks (never returning them to the system allocator) and validates block headers on every reference-count operation. |

Boolean switches require the exact value `1`; strings such as `true` or `on` have no effect.

Two additional environment variables are read by the **compiler** rather than the generated binary:

- `CINCH_RECLAIM=0` (or the CLI flag `--no-reclaim`): disables emitting `cinch_dup`/`cinch_drop` calls during code generation so every allocation stays live until exit. Useful for reclamation baselines; does not relax compile-time ownership checks.
- `CINCH_RC_TRACE` (any value, debug compiler builds only): dumps the ownership verifier's input and output to stderr before aborting on an internal ownership-verifier bug.

For compile-time allocation assertions (`#[no-alloc]` and `#[alloc("once")]`), see [safety assertions](22-safety-assertions-and-llvm.md).

# Allocation statistics (`CINCH_ALLOC_STATS`)

Consider a program that allocates a 1,000-element list and measures its length:

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

Running the compiled binary with `CINCH_ALLOC_STATS=1` prints one summary line to stderr at exit:

```text
$ CINCH_ALLOC_STATS=1 ./example
cinch-alloc: blocks=1922 bytes=48862 peak=24126 header-blocks=1916 header-bytes=48648 immortal=0 freed=1916 live=0
```

| Field | Meaning |
| --- | --- |
| `blocks`, `bytes` | Cumulative allocations and bytes across the entire run (including runtime internals). |
| `peak` | High-water mark of live bytes. |
| `header-blocks`, `header-bytes` | Cinch value blocks and their bytes including the 8-byte header (a subset of `blocks`). |
| `immortal` | Dynamically allocated uncounted blocks (such as interned `Sym` cells). Static data-section blocks are never allocated and do not appear here. |
| `freed` | Header blocks released by reclamation. |
| `live` | `header-blocks - freed`. |

The difference `blocks - header-blocks` represents internal runtime allocations (bignum limbs, backing byte buffers, handle tables). Because the handler is installed via `atexit` on the first allocation, it runs on normal exit, heap exhaustion, and panic faults, but **not** on stack exhaustion (which terminates directly via `_exit`).

# Leak checking (`CINCH_LEAK_CHECK`)

`CINCH_LEAK_CHECK=1` reports surviving blocks after `main` returns:

```text
$ CINCH_LEAK_CHECK=1 ./example
cinch-leak: live=0 immortal=0 leaked=0
```

`leaked` is `live - immortal`. When `leaked > 0`, the runtime overrides the process exit code to **74**:

```text
$ CINCH_RECLAIM=0 cinch build Main.cinch -o example-no-reclaim
$ CINCH_LEAK_CHECK=1 ./example-no-reclaim
cinch-leak: live=1916 immortal=0 leaked=1916
$ echo $?
74
```

Comparing `CINCH_ALLOC_STATS=1` with and without reclamation shows the effect on peak memory:

| Build | `peak` | `freed` | `live` |
| --- | --- | --- | --- |
| Default | 24126 | 1916 | 0 |
| `CINCH_RECLAIM=0` | 48795 | 0 | 1916 |

Exit code **74** is strictly a leak-check diagnostic status, distinct from runtime program faults: **70** (`BlackHole`), **71** (generic runtime fault), and **73** (`Exhausted`).

# Reference-count debugging (`CINCH_RC_DEBUG`)

`CINCH_RC_DEBUG=1` turns every `cinch_free` into a poison-and-quarantine operation and checks block header invariants on every retain and release. A valid program produces the same output and exit code, while `peak` grows because quarantined blocks are never reused:

```text
$ CINCH_RC_DEBUG=1 CINCH_ALLOC_STATS=1 ./example
cinch-alloc: blocks=1922 bytes=48862 peak=48790 header-blocks=1916 header-bytes=48648 immortal=0 freed=1916 live=0
```

Traps triggered in this mode indicate a compiler or runtime bug (such as a use-after-free or corrupt header) and immediately take the fault exit path. Always benchmark with `CINCH_RC_DEBUG` and `CINCH_LEAK_CHECK` unset.

# Live heap cap (`CINCH_HEAP_LIMIT`)

`CINCH_HEAP_LIMIT` bounds **live** bytes—memory currently allocated and not yet freed. A long-running program that reclaims temporary values can allocate far more than the cap over its lifetime; a program that retains every allocation stops deterministically at the limit.

The following program allocates a 1 MiB buffer per step and retains all of them in a list, exhausting any finite heap cap and exiting **73**:

<!-- cinch:run exit=73 -->
```cinch
module Main where
  import Data.List

  grow :: Int -> List Bytes -> Int
  grow n acc = grow (n + 1) (Cons (bytes-new 1048576) acc)

  main :: Unit -> Int
  main u = grow 0 Nil
```

Running the compiled binary with an 8 MiB cap shows the exact refusal and optional statistics:

```text
$ CINCH_HEAP_LIMIT=8M CINCH_ALLOC_STATS=1 ./example
cinch: exhausted: heap (8389071 bytes)
cinch-alloc: blocks=30 bytes=8389207 peak=8389135 header-blocks=16 header-bytes=384 immortal=0 freed=2 live=14
$ echo $?
73
```

`8389071` is the live byte total the refused allocation would have produced (one buffer past 8 MiB). System allocation failures and size-overflowing requests take the same `Exhausted` exit path (code **73**) rather than aborting inside the OS allocator.

Setting `CINCH_HEAP_LIMIT=0` disables the cap completely. An unparsable value falls back safely to the 8 GiB default.

# Stack reservation (`CINCH_STACK_SIZE`)

Generated executables run user code on a dedicated program thread with a **1 GiB** reserved stack by default so deep non-tail recursion has headroom. The low end of the stack is protected by a 64 KiB inaccessible guard band; touching it reports `Exhausted` cleanly instead of crashing:

```text
$ CINCH_STACK_SIZE=256K ./deep
cinch: exhausted: stack
$ echo $?
73
```

Values below `256K` are clamped up to `256K` to leave room for the 64 KiB guard band and runtime frames. Unparsable values or `0` preserve the 1 GiB default. Because stack exhaustion terminates via `_exit`, `CINCH_ALLOC_STATS` does not print after it. Memory faults outside the guard band restore the default signal disposition rather than masquerading as stack exhaustion.

# Header reference-count width and the heap cap

Every allocated value block carries an 8-byte runtime header at offset `-8` with a 40-bit signed reference count, an 8-bit block kind, and a 16-bit layout ID (see [runtime headers, constructor tags, and pointer maps](24-type-layout.md#runtime-headers-constructor-tags-and-pointer-maps) for the exact bit layout and negative-count atomic sharing rules).

Because the 40-bit count can represent more references than the heap cap can hold live pointer words, reference counts cannot saturate or overflow in valid programs and require no overflow branch on retain. Changing the count width would change the runtime ABI and heap-cap invariant together; swapping the backing allocator policy does not.

# Planned: replaceable program allocators

<span class="support-tag support-tag--planned">Coming soon</span> The [allocator design](../design/allocators.md) defines a replaceable global allocator architecture. None of the following is shipped yet:

| Planned facility | Intended behaviour | Current status |
| --- | --- | --- |
| In-house small-object allocator | Size classes, per-thread heaps, sharded page free lists, address-derived allocator identity | Runtime wraps the system allocator |
| `#[global-allocator]` selection | Install a Cinch-written or C-ABI allocator as the ambient program allocator | Rejected as an unknown attribute |
| Allocator callbacks | Allocate, free, and optional resize with explicit size and alignment | No runtime allocator ABI |
| System allocator backend option | Retain `malloc` as an explicit choice for debugging and comparison | Currently the only backend |
| Static startup pool | Pre-claim all program memory before `main` from a proven bound | Requires planned allocation-bound proofs |

Accounting and `CINCH_HEAP_LIMIT` will wrap whichever allocator is installed so telemetry and exhaustion semantics remain uniform. Under the design, a Cinch-written allocator must itself satisfy `#[no-alloc]`, every deallocation must return to its originating allocator, and cross-task releases must not assume the freeing task is the allocating task.

# Planned: scoped heaps, arenas, pools, and budgets

<span class="support-tag support-tag--planned">Coming soon</span> Four higher-level memory abstractions are designed on top of replaceable allocators, none yet accepted by the compiler:

| Planned facility | Intended behaviour | Current status |
| --- | --- | --- |
| `Heap r` capability | Explicit allocation context (`using (heap :: Heap r) =>`, `with (heap = arena) in e`) | `Heap` is not a known signature |
| Scoped bump arenas | Fast bump allocation released wholesale at scope exit | Not implemented |
| Arena escape checking | Region check ensuring no arena pointer outlives its scope without explicit copy-out | Region checking currently covers borrows, cells, and `Eval` labels |
| Counted pools | Fixed-size block pools returning blocks individually on last release | Not implemented |
| Typed budget boundaries | Turn sub-computation exhaustion into a typed outcome instead of process exit 73 | Only process-wide exit 73 exists |
| `evacuate`-style copy-out | Deep-copy a value from an inner arena to an outer heap | Provisional design |

# Planned: proven allocation bounds

<span class="support-tag support-tag--planned">Coming soon</span> The `jpl` profile enforces recursion bounds, and two function-level allocation assertions are active today. Stronger in-place and numeric bound assertions remain planned:

| Assertion | Status |
| --- | --- |
| `#[no-alloc]` | **Implemented.** Rejects any allocation site inside the definition. |
| `#[alloc("once")]` | **Implemented.** Bounds the definition to at most one allocation site per call. |
| `#[fip]` (fully in-place) | **Planned.** Will couple unique-ownership reuse analysis to a zero-fresh-allocation guarantee. |
| `#[alloc("bounded(n)")]` | **Planned.** Currently rejected (`#[alloc]` only accepts `"once"`). |
| Proven-bound startup pool | **Planned.** Depends on bounded-allocation verification. |
