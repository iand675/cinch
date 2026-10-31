# Ownership, regions, and memory reclamation

The usage checker constrains duplication and dropping independently of the runtime's memory reclamation. A reference-counted representation does not make an affine or linear resource logically duplicable.

# Multiplicities

`%0` arguments are erased, `%1` arguments are used exactly once on every admitted path, `%aff` arguments at most once, and `%many` arguments admit repeated use subject to structural/resource rules. Branch alternatives are checked path-sensitively. Two uses in mutually exclusive branches are different from two uses on one path.

Projection places matter: disjoint fields may be consumed separately, but repeatedly consuming the same place stacks uses. Captured values retain their usage restrictions when placed in lambdas, thunks, tasks, and resumptions. A generic function that can discard its argument cannot be used as a sink for a linear task handle.

# Copy and borrows

`Copy` (`Share` alias) is a compiler-known structural property. It is not a user-extensible class. `copyOut` creates an owned value from an admitted copyable place/borrow and cannot launder a linear argument into multiple uses.

`Ref r a` is a shared borrow, `Mut r a` a mutable borrow. `borrow` and `mutBorrow` introduce the corresponding scoped access. Escape checking prevents a result from carrying a reference beyond the owner/region lifetime. Lexical uniqueness constrains mutable borrows. Writes invalidate overlapping evidence facts, including bounds/refinement facts about previous contents.

Record layouts add access constraints: packed unaligned fields may be copied by marshaling but cannot yield ordinary aligned borrows. A foreign pointer's ownership role and deallocator contract are separate from a region's proof that an internal reference remains valid.

# Reclamation

Native code uses GC-free reference counting by default. Compiler ownership analysis inserts the required retains/releases; pointer maps guide reclamation of aggregate fields, and iterative dropping avoids recursive drop chains. Small mathematical integers may use immediate representations. Shared task captures use the admitted cross-thread accounting discipline.

`CINCH_RECLAIM=0` in the **compiler environment** disables insertion for comparisons/baselines. It is not a recommended application memory mode and is not a runtime flag that retroactively changes an already built executable. Resource protocols remain part of checking regardless of this code-generation selection.

# Runtime diagnostics and limits

| Variable | Applied when | Meaning |
| --- | --- | --- |
| `CINCH_ALLOC_STATS=1` | Running a generated program | Print allocation counters. |
| `CINCH_LEAK_CHECK=1` | Running a generated program | Enable leak diagnostics. |
| `CINCH_RC_DEBUG=1` | Running a generated program | Validate reference-count headers and poison/retain freed blocks for debugging. |
| `CINCH_HEAP_LIMIT=n[K\|M\|G]` | Running a generated program | Bound live accounted heap; default 8 GiB, `0` disables the cap. |
| `CINCH_STACK_SIZE=n[K\|M\|G]` | Running a generated program | Select reserved program-stack size; zero/invalid uses the runtime default. |

Suffixes use binary multiples. Invalid heap limits fall back to the default. Exhaustion is a defined process exit (73), not a successful value or a caught `Result` error. Debug poisoning intentionally retains memory and changes measurements; compare performance with the relevant debug modes disabled.

See [allocators and heap customization](25-allocators.md) for the intended design of replaceable program allocators, scoped heaps, arenas, pools, and proven allocation budgets.

<span class="support-tag support-tag--planned">Coming soon</span> **allocator and representation extensions.** The [allocator design](../design/allocators.md) and [layout/specialization design](../design/layouts-and-specialization.md) describe selected future directions. Their APIs must be marked planned until accepted by the Rust implementation. Current explicit layout attributes are documented separately.

Implementation: `check_terms.rs`, `rc.rs`, `core.rs`, runtime `block.rs`, `rc.rs`, `heap.rs`, `stack.rs`; suites: `ownership.rs`, `reclamation.rs`, `reclamation_tasks.rs`, `alloc_accounting.rs`, `exhaustion.rs`.
