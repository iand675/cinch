# Allocators: a replaceable default, an allocation capability, proven budgets

Status: direction decided by the user (2026-10-29); details below are
proposals for the implementing slices. Companions: `design/reclamation.md`
(reference counting, the block header, slices S0-S12),
`design/safety-assertion-attributes.md` (the JPL "Power of 10" mapping, the
`jpl` profile, `#[no-alloc]`, `#[alloc "once"]`),
`design/layouts-and-specialization.md` (block layouts),
`design/proof-automation.md` (the arithmetic that proves allocation bounds).
No em dashes.

## 1 Decisions (user, 2026-10-29)

- **The program allocator is user-supplied, with an in-house small-object
  allocator as the default.** System `malloc` stays available as one choice.
- **Scoped allocation is a capability**, so arenas and bump allocators
  (Rust's `bumpalo`) are ordinary library values: `using (heap :: Heap r)`
  in signatures, supplied by `with (heap = arena) in e`, the same machinery
  as `using (io :: Io r)` and `with (io = ...) in`.
- **JPL-style code keeps functional style.** The `jpl` profile gains
  fully-in-place functions and proven allocation bounds beside
  `#[no-alloc]` (section 5).
- **Layout-id width is a compiler build option.** The default header
  (`design/reclamation.md` section 3, revised) is a 40-bit count, an 8-bit
  kind and a 16-bit layout id. A build for very large programs (millions of
  lines, many specializations) may widen the layout id and narrow the count,
  for example 24 and 32 bits. Count overflow is impossible by construction:
  the allocator caps the heap below `8 * 2^count_bits` bytes, the most
  pointer words that could exist, so a narrower count lowers that cap; the widths are named constants in
  `runtime/src/block.rs` and are recorded in `runtime_abi`, so objects built
  with different widths never link together.

## 2 Why: what the user asked for

"I mentioned wanting to support JPL-style programming, so I think that sort
of puts us in the realm of zig or rust where we should be able to supply
allocators within the runtime". JPL rule 3 is "no dynamic memory
allocation after initialization" (Holzmann, "The Power of Ten", 2006).
Cinch already enforces the rule's letter with `#[no-alloc]` and
`#[alloc "once"]`; what was missing is control over where memory comes
from, and a way for idiomatic functional code (which constructs values
constantly) to satisfy the rule rather than be banned by it.

## 3 Layer 1: the program allocator

| Choice | Use | Notes |
| --- | --- | --- |
| In-house small-object allocator (default) | general programs | size classes for small blocks, per-thread heaps, free lists sharded per page (mimalloc's design: Leijen, Zorn, de Moura, "Mimalloc: Free List Sharding in Action", APLAS 2019; Koka uses mimalloc, Lean 4 has its own small-object allocator). Pages live in aligned segments, so a free finds its page, and the page's owning heap, by masking the address: no header bits name the allocator. Fixed-size blocks are exactly what reference counting with reuse frees, so the free lists recycle perfectly. |
| Static pool reserved at startup | `jpl` profile, flight and embedded targets | all memory is claimed before `main`, sized from the program's proven bound (section 5), so exhaustion after startup is impossible; reserving it can fail only before any user code runs. Code without a proven bound uses a typed budget boundary (`design/no-faults.md` section 4). |
| System `malloc` | debugging (ASan), comparison | today's behavior (`runtime/src/heap.rs` wraps `System`). |
| User-supplied | anything else | a runtime allocator interface (allocate, free, optional resize; size and alignment explicit, as Rust's `GlobalAlloc` and Zig's `std.mem.Allocator` vtable are), implemented in Cinch through a `#[global-allocator]`-style declaration or by a C-ABI object at link time. |

The accounting and the `CINCH_HEAP_LIMIT` cap (`heap.rs`) wrap whichever
allocator is chosen, so every choice keeps the measurements and the cap.

## 4 Layer 2: the `Heap` capability and scoped allocators

```
parse-all :: forall r. using (heap :: Heap r) => Bytes -> Result ParseError Summary
...
main u = with (heap = bump-arena (mebibytes 64)) in summarize (parse-all input)
```

- Allocation sites inside a function that needs `Heap r` draw from the
  supplied allocator. Without a supplied one, the program allocator is the
  ambient default, so ordinary code never mentions `Heap` (how the default
  is spelled, inferred or defaulted in signatures is an open question tied
  to effect-row defaults, section 7).
- `#[no-alloc]` becomes, in addition to today's syntactic check, "needs no
  `Heap` capability", so the type checker carries the property across calls.
- A block is freed back to the allocator it came from; the in-house
  allocator and pools find it from the address (section 3). Foreign memory
  keeps its own deallocator pairing (`specifications/foreign-function-interface-specification.md`
  section on allocator identity).

**Arenas and escape.** An arena's value is that its blocks need no
per-block bookkeeping: they are allocated uncounted (the header's immortal
count, so dup and drop on them are the existing no-op test) and released
all at once when the arena's scope ends. Nothing allocated in an arena may
therefore outlive that scope, and this must be static, not a runtime hope:

- the result of `with (heap = arena) in e` may not contain a pointer into
  the arena: its type's class is `Raw` (scalars), or the value is copied
  out explicitly into the outer heap (`evacuate`, a deep copy, explicit as
  the core specification requires for expensive copies);
- inside the scope, storing an arena-allocated `Obj` into state that
  outlives the scope (an outer `Mut`, a closure or task that escapes) is
  refused by the same escape checker that enforces `Ref r a`, `Mut r a` and
  region memo cells (D396 uses it for recursive values);
- as a safety net in debug and `jpl` builds, the arena counts the blocks
  still reachable from outside at teardown through the runtime's leak
  check; a nonzero count is a compiler-bug trap (the static rule makes it
  unreachable), never a program fault (`design/no-faults.md`).

Counted pools (a fixed-capacity free-list pool for one block size, Ada's
`Storage_Pool` model) are the other kind: their blocks are reference
counted and freed individually, and only the pool's teardown needs the
"no live blocks" check.

## 5 Layer 3: proven allocation budgets for the `jpl` profile

- `#[no-alloc]` (exists): no allocation sites in the body.
- **Fully in-place functions** (`#[fip]`, after Lorenzen, Leijen and
  Swierstra, "FP²: Fully in-Place Functional Programming", ICFP 2023, and
  Koka's `fip` keyword): a function that owns its arguments uniquely and
  reuses every freed cell for a new cell of the same size allocates nothing
  net. The checker proves it from the reclamation pass's reuse analysis
  (slice S7). Idiomatic code such as `map` over an owned list, a tree
  insertion with rebalancing, or a state-machine step then runs after
  initialization with no new memory, so functional style satisfies JPL rule
  3 instead of being excluded by it.
- `#[alloc "bounded(n)"]`: a static upper bound on allocation, proven with
  the bounded-recursion machinery (`#[recursion "bounded(n)"]`) and linear
  arithmetic (`omega`-style, `design/proof-automation.md`). A program's
  startup pool is sized from the sum of its components' proven bounds.
- The `jpl` profile requires every definition reachable after
  initialization to be `#[no-alloc]`, `#[fip]` or `#[alloc "bounded(n)"]`
  against a static pool.

Zig and Rust let a programmer choose an allocator and document a budget;
Cinch proves the budget, which is the difference that matters for JPL-style
code.

## 6 Slices

| Slice | Content | Depends |
| --- | --- | --- |
| A1 | In-house small-object allocator as the default (size classes, per-thread heaps, segment-aligned pages, free by address mask); `heap.rs` accounting wraps it | reclamation S1 (header-aware allocation) |
| A2 | Runtime allocator interface; user-supplied program allocator; the static startup pool with its exhaustion fault | A1 |
| A3 | `Heap r` capability, `with (heap = ...)`, the arena escape rule, bump arenas and counted pools in the base library, the teardown check | reclamation S6 (dup/drop exist), D396's escape checking |
| A4 | `#[fip]` checking and `#[alloc "bounded(n)"]`; the `jpl` profile's allocation rule | reclamation S7 (reuse), the bounded-recursion attributes, linear-arithmetic proofs |

## 7 Open questions

- How `Heap` appears in signatures: always inferred and shown in
  interfaces, defaulted to the ambient program allocator like an effect row,
  or written explicitly only where a scoped allocator is required.
- The spelling of explicit copy-out from an arena (`evacuate` is a
  placeholder).
- Whether arenas may hold counted blocks at all (a mixed arena reclaims
  some blocks early at the cost of counting them), or are uncounted only.
- How a Cinch-written program allocator avoids allocating while it
  allocates: it must be `#[no-alloc]` itself, which the attribute already
  checks.
- Resolved by `design/no-faults.md`: exhaustion is never a fault; it is
  impossible (proven bounds) or a typed `Exhausted` at a budget boundary.
