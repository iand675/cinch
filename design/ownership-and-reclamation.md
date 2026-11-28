# Ownership, reclamation, and allocators

Cinch is **garbage-collector-free by design**. Memory safety and deterministic reclamation come from combining compile-time usage/region checking with Perceus-style precise reference counting, self-describing heap blocks, stack allocation for non-escaping values, and pluggable/scoped allocators.

Normative requirements live in [`specifications/core-language-specification.md`](../specifications/core-language-specification.md) and [`specifications/lazy-computation-and-reclamation-specification.md`](../specifications/lazy-computation-and-reclamation-specification.md). User documentation lives in [`reference/10-ownership-and-memory.md`](../reference/10-ownership-and-memory.md) and [`reference/25-allocators.md`](../reference/25-allocators.md).

---

## 1. Usage multiplicities, regions, and `Share`

Every binder and function arrow carries a multiplicity checked path-sensitively across branches, error paths, and cancellation paths:

| Multiplicity | Meaning | Runtime effect |
| --- | --- | --- |
| `%0` | Erased; never inspected at runtime | Zero representation; erased before codegen |
| `%1` | Linear; consumed exactly once on every path | Ownership transfer; dropping without use is a compile error |
| `%aff` | Affine; used at most once | Never duplicated; dropping runs deterministic release |
| `%many` | Unrestricted; freely duplicated | Backed by `Share` / reference counting |

### Regions and borrows
- `Ref r a` (shared borrow) and `Mut r a` (exclusive borrow) carry a lexical region `r`.
- Borrows cannot escape their region via return types, escaping structs, or escaping closures. A region quantified by a function signature belongs to the caller; a body-local `borrow` cannot unify with a caller-quantified region.
- `Mut r a` is lexically unique within its region (no overlapping `Ref r` or second `Mut r`).
- `Share` / `Copy` are compiler-known structural classes (machine scalars, `Ref r a`, immutable ADTs whose fields are `Share`). Linear handles (such as `Effect.Pty.Pty`, `Task` handles, and owned foreign pointers) cannot be implicitly copied or dropped.

---

## 2. Precise reference counting (Perceus)

Reclamation is inserted on typed Core (`seed/src/rc.rs`) after representation specialization so every binder and field has a known **representation class**:

| Class | Words | Counted | Meaning |
| --- | --- | --- | --- |
| `Raw` | 1 | No | Machine scalars (`<= 64` bits), `Bool`, `Unit`, raw pointers, handles |
| `Obj` | 1 | Always | Pointer to a heap or static block (ADTs, closures, dictionaries, `Bytes`, `Array`) |
| `TObj` | 1 | When low bit is `0` | Tagged immediate (low bit `1`) or counted pointer (`Nat`, `Int`) |
| `Wide` | 2 | No | Unboxed 128-bit scalar (`I128`, `U128`) |

### Small `Nat` / `Int` immediates
Small mathematical integers avoid heap allocation entirely by setting bit 0:
- `Nat` immediates hold `0 ..= 2^(csize-1) - 1` (`0 ..= 2^63 - 1` on 64-bit targets).
- `Int` immediates hold `-2^(csize-2) ..= 2^(csize-2) - 1` (`-2^62 ..= 2^62 - 1` on 64-bit targets).
- Values outside the immediate range box onto the heap (`KIND_NAT` / `KIND_INT`).

### Key RC optimizations
- **Borrow inference:** parameters that are only read and never stored or returned are marked borrowed, eliminating redundant `dup`/`drop` pairs across calls.
- **Drop specialization & in-place reuse:** when matching an owned constructor block whose runtime reference count is uniquely `+1` (never a shared/atomic block), `Release` recycles the cell directly into a freshly constructed block of matching size (`Reuse`) instead of freeing and reallocating.
- **Static immortal blocks:** nullary constructors, closed dictionaries, and constant strings live in read-only data with `RC_IMMORTAL`, making `dup`/`drop` a single fast check.
- **Thread-sharing mark-on-publish:** blocks reachable from one thread use plain non-atomic RC operations. When a value crosses a task boundary (`spawn`, `MVar`, `TVar`, parallel callback), `cinch_mark_mt` walks its unmarked subgraph and flips counts to atomic mode (stopping at already-atomic blocks).
- **No count overflow:** `CINCH_HEAP_LIMIT` is capped well below `8 * 2^35` bytes, so live pointer counts can never overflow the 35-bit reference count.

---

## 3. Self-describing block headers (ABI 10)

Every heap and static block begins with a single 64-bit header word (`seed/runtime/src/block.rs`). There is **no global layout table**, no startup layout registration, and no cross-object layout ID coordination:

```text
bits  0..34   rc      35-bit signed count (RC_IMMORTAL, RC_FRESH, RC_FRAME, or live count)
bits 35..39   kind    5-bit block kind (constructor, closure, dict, bytes, nat, tvar, ...)
bits 40..63   layout  24-bit layout description
```

### Inline vs escaped layouts
- For **shaped blocks** (constructors, closures, dictionaries):
  - `bits 40..47`: constructor tag (`0` for closures/dictionaries; `0xFF` = escaped).
  - `bits 48..63`: 16-bit counted-field mask (bit `i` set iff field `i` is `Obj`/`TObj` and must be scanned on drop/mark). Fields are never reordered, preserving FFI and ABI layout stability.
- **Escaped descriptors (`tag == 0xFF`):** used when a type has `>= 255` tags, a counted field at index `>= 16`, or payload `> 31` words. The word immediately before the header points to a private, read-only, 8-aligned static descriptor (`[tag, payload_words, counted_mask_words...]`). Descriptors are compared by content, never address identity, so separately compiled objects link freely.
- For **unshaped blocks** (`Bytes`, `Array`, `Nat`, etc.): `bits 40..63` store payload word count directly (with `0xFFFFFF` spilling to a pre-header word).
- **Block sizing on free:** small-object blocks (`<= 256` bytes) derive their size from their aligned segment/page address; escaped blocks carry size in their descriptor; closures derive size from prefix + filled count; oversized inline blocks record size in a fallback table.

---

## 4. Stack allocation of non-escaping blocks

After RC insertion, an escape pass (`seed/src/stack.rs`) identifies constructors and closures that never outlive the allocating function frame and allocates them with `alloca` using the `RC_FRAME` header marker instead of calling the heap allocator:

- A frame block is never passed to `cinch_free`; its counted payload fields are released explicitly (`cinch_frame_release`) before the frame exits or before a tail call replaces the frame.
- Non-tail continuations consumed once by a callee (including closures passed as arguments) qualify for frame allocation; the apply stub recognizes `RC_FRAME`, duplicates captured words for the call, and releases the frame payload once.
- A tail call inside the allocating unit cannot reuse a frame block whose storage is about to be overwritten, so tail-escaping allocations stay on the heap.

---

## 5. Allocators, arenas, and proven budgets

Cinch structures memory allocation in three layers:

1. **Program allocator (default: in-house small-object allocator):** per-thread heaps with size-classed free lists in aligned segments (`seed/runtime/src/small.rs`), finding page metadata by address mask with zero per-block allocator header bits. System `malloc` remains selectable for debugging/ASan; a pluggable global allocator and a startup-reserved static pool are designed for embedded and `jpl` targets.
2. **Scoped allocators and region-delimited arenas (`Effect.Arena`):** `with-arena` and `with-arena-budget` run pure computations inside a dedicated bump arena scoped to a Rank-2 region `r` (`ArenaToken r e`), supporting `O(1)` prompt escape (`arena-abort`) and typed byte-budget exhaustion (`Err Exhausted`) with zero leaks. Four static invariants keep `O(1)` escape sound: Rank-2 region token isolation, scalar/Copy escape-safe return and error types, refusal of `%1` linear captures, and a closed pure (`->{}`) callback arrow. General `Heap r` capabilities and heap-value `evacuate` remain planned extensions.
3. **Proven allocation budgets (JPL Rule 3):** functional code can satisfy strict no-dynamic-allocation rules without abandoning algebraic data types through three compile-time checked tiers:
   - `#[no-alloc]` — zero allocation sites.
   - `#[fip]` (fully in-place, after FP²/Koka) — every allocation reuses a uniquely owned freed cell of matching size (net zero allocation).
   - `#[alloc "bounded(n)"]` — statically bounded allocation sized into a startup pool.

---

## 6. Open decisions

- **Cycle reclamation for `letrec` knots and memo cells:** enforcing region-scoped teardown for cyclic knots (`seed/runtime/CYCLE-COLLECTOR-DESIGN.md`).
- **Default multiplicity & user `Share`:** whether unannotated owned parameters default to `%aff` or `%many`, and whether user-extensible `Share` instances should exist.
- **Two-phase borrows and reborrowing:** relaxing lexical `Mut` uniqueness where safe.
- **Scoped `Heap r` capability surface:** spelling of `using (heap :: Heap r)`, explicit arena `evacuate`, and `#[fip]` / `#[alloc "bounded(n)"]` checking.
- **Remaining conservative `Raw` slots:** giving exact representation classes to memo/knot slots, wide joins, and compile-time constants so they never leak.
