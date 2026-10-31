# Allocators and heap customization

Cinch's allocator design gives programs control over where memory comes from while preserving functional programming, ownership checking, and deterministic reclamation. It has three layers: a replaceable program allocator, scoped heaps supplied as capabilities, and proven allocation budgets.

This chapter describes the intended design. Example spellings illustrate the model; unresolved syntax is identified where it matters. The underlying decisions are recorded in [the allocator design notes](../design/allocators.md) and [the resource-exhaustion design](../design/no-faults.md).

# Choose the right level of customization

| Need | Mechanism |
| --- | --- |
| Ordinary application allocation | Use the default small-object allocator. |
| Change allocation policy for the whole program | Supply a program allocator. |
| Allocate temporary data for one computation | Supply a scoped arena through `Heap r`. |
| Recycle individually reclaimed blocks from reserved storage | Use a counted pool. |
| Recover when a computation exceeds its memory allowance | Run it inside a typed budget boundary. |
| Guarantee sufficient memory after startup | Prove allocation bounds and reserve a startup pool. |

These choices compose. A program can use a custom default, give a parser a temporary arena, and give an untrusted workload a recoverable budget. Allocator selection determines where storage comes from; [ownership and reclamation](10-ownership-and-memory.md) determine how values may be used and when their storage can be released.

# The program allocator

The program allocator supplies storage when no scoped heap has been selected. Ordinary functions can construct lists, trees, closures, and other values without passing an allocator explicitly.

The default is an in-house small-object allocator designed around the blocks produced by reference-counted functional programs:

- Size classes group small allocations into fixed-size blocks.
- Per-thread heaps and free lists sharded by page organize allocation and recycling.
- Pages live in aligned segments. A block's address identifies its page and owning heap, so allocator identity does not consume bits in each value's header.
- Reclamation returns dead blocks to the appropriate free lists; reuse can avoid requesting fresh storage altogether.

System allocation remains an option for debugging and comparison. A statically reserved pool serves programs with proven bounds. A user-supplied allocator can provide a policy suited to a particular target or workload.

Accounting and heap limits wrap the selected allocator. Choosing a different backend preserves measurement and budget enforcement rather than requiring each allocator to implement a separate reporting system. Live-byte limits concern storage still held, rather than the cumulative amount allocated and released over a program's lifetime.

# Supply a custom program allocator

A custom allocator implements a runtime interface with explicit sizes and alignments:

| Operation | Responsibility |
| --- | --- |
| Allocate | Provide storage meeting the requested size and alignment. |
| Free | Return a block to the allocator that owns it. |
| Resize, optional | Resize an allocation while preserving the retained contents. |

The design provides two integration routes: an allocator written in Cinch and selected by a `#[global-allocator]`-style declaration, or a C-ABI allocator object supplied at link time. The exact declaration spelling and ABI details remain to be specified; the attribute name describes the intended selection point.

A Cinch-written allocator must itself satisfy `#[no-alloc]`: implementing allocation cannot recursively request storage from the allocator being implemented. Allocator state and bookkeeping therefore need storage that does not depend on that recursive path.

Every free must return to the originating allocator. Changing the ambient heap changes subsequent allocation sites; it does not transfer ownership of blocks already allocated elsewhere. The default allocator and pools recover ownership from the address. Foreign allocations retain their matching deallocator under the [foreign resource protocol](../specifications/foreign-function-interface-specification.md#3-ownership-and-allocation).

Task sharing also preserves allocator ownership. A counted value can be released by whichever task drops its last reference, so an allocator's release policy must account for that lifetime rather than assume release always happens in the allocating task.

## Worked example: define and select a zeroing allocator

Suppose a program wants every fresh allocation zero-filled. Define a custom allocator that requests storage directly from the system backend, clears it, and returns it. Its free callback delegates to that same backend with the original size and alignment.

This is a **design sketch** of the intended callback interface and selection declaration. `Allocator`, its field names, and the backend primitive names are illustrative. `ByteSize` and `Alignment` represent checked machine-sized values; `Block` is an owned raw storage handle; `Allocation` is an unboxed `Granted`/`Refused` status. Constructing these callback results does not allocate Cinch value blocks. `Allocator` itself is a static table of function pointers.

```text
#[no-alloc]
zeroed-allocate :: ByteSize -> Alignment -> Allocation
zeroed-allocate size alignment =
  case system-allocate size alignment of
    Refused -> Refused
    Granted block ->
      let () = zero-memory (borrow block) size in
      Granted block

#[no-alloc]
zeroed-free :: Block -> ByteSize -> Alignment -> Unit
zeroed-free block size alignment =
  system-free block size alignment

#[global-allocator]
program-allocator :: Allocator
program-allocator = Allocator
  { allocate = zeroed-allocate
  , free = zeroed-free
  , resize = None
  }
```

The low-level primitive contracts are part of the sketch: `system-allocate` satisfies the requested layout or returns `Refused`; `zero-memory` borrows valid storage and clears exactly `size` bytes without allocating; `system-free` consumes the allocation handle. Their eventual signatures must carry those storage, ownership, and foreign-effect obligations.

The selection declaration installs `program-allocator` as the ambient program allocator. For an ordinary allocation, the sequence is:

1. The runtime's accounting and limit layer admits the request.
2. `zeroed-allocate` obtains storage from the system backend and clears it.
3. The runtime initializes the Cinch header and payload in that storage.
4. On final release, `zeroed-free` returns the original allocation base to the system backend with its matching layout.

`system-allocate` is the underlying backend operation, not a call back through the selected program allocator. That distinction prevents recursion. No logging, temporary collections, or heap-allocated error values are created inside the callbacks. A `Refused` result follows the enclosing budget's exhaustion protocol rather than becoming a usable pointer.

Resize is omitted because it is optional. An allocation layer needing to grow a block can obtain a fresh block, copy the retained bytes, and free the old one after successful allocation; if allocation is refused, the old block must remain intact. The fresh block is zeroed before the copy, so bytes beyond the copied prefix remain zero. Zero-filling has a cost proportional to the requested size; this example demonstrates a custom policy rather than a faster default.

# Scoped heaps are capabilities

`Heap r` makes an allocation context available to a computation, using the same capability-binding model as I/O:

```text
parse-all :: forall r. using (heap :: Heap r) => Bytes -> Result ParseError Summary
```

A caller supplies the context with `with`:

```text
with (heap = arena) in computation
```

Allocation sites requiring that capability draw from the supplied heap. The program allocator is the ambient default when a scoped allocator has not been supplied. The precise rules for inferring, displaying, and defaulting `Heap` requirements in signatures remain an open design choice.

To customize one computation:

1. Choose an arena, counted pool, or budget suited to its lifetime and capacity requirements.
2. Bind it as the computation's `heap` capability.
3. Keep scoped allocations inside their permitted lifetime, copying a result into an outer heap explicitly when necessary.

This keeps allocator policy at the caller's boundary while allocation-aware library functions work against the capability.

# Arenas and escape checking

An arena is useful when many temporary values share one lifetime, such as a parser's intermediate tree. A bump arena advances through reserved storage and releases it together when the scope ends.

The following sketch allocates parser temporaries within a 64 MiB arena and summarizes them before leaving it. It assumes the returned summary contains only scalar data with no pointers into the arena. `bump-arena` and `mebibytes` illustrate the proposed library interface:

```text
with (heap = bump-arena (mebibytes 64)) in
  summarize (parse-all input)
```

Arena blocks use the header's uncounted/immortal representation. Retaining or dropping a reference to one does not perform per-block reference counting; the arena owns the storage until teardown. Here, “immortal” means the count machinery does not reclaim the individual block, not that its storage survives the arena.

The compiler must prove that arena pointers cannot outlive the scope:

- A returned value must contain no pointer into the arena. Scalar results can leave directly; an arena-backed structure requires an explicit deep copy into the outer heap.
- An arena-backed value cannot be stored in outer mutable state or captured by a closure or task that escapes the scope.
- Existing region escape checking extends to these allocation lifetimes.

Copying out is explicit because a deep copy can be expensive. The design notes call that operation `evacuate`, but its final name and signature remain open.

Static escape checking is the lifetime guarantee. Debug and `jpl` teardown checks are defenses against compiler mistakes, not a recoverable way to permit escaping pointers. Whether arenas should also support a mixture of counted and uncounted blocks remains open.

## Worked example: copy an arena result into the outer heap

Reverse a list using temporary arena storage, then return an independently owned list to the caller. Capture the caller's heap **before** rebinding `heap`; pass that saved capability explicitly to the copy-out operation while the arena is still alive.

This **design sketch** uses the provisional spelling `evacuate destination value`. It means a deep copy into the explicitly named destination heap, including ownership/provenance information that lets the copy leave the arena scope. The final copy-out signature and capability-binding details remain open.

```text
reverse-into :: forall r. using (heap :: Heap r) =>
  List U64 -> List U64 -> List U64
reverse-into xs acc = case xs of
  Nil -> acc
  Cons x rest -> reverse-into rest (Cons x acc)

reverse-copy :: forall outer. using (heap :: Heap outer) =>
  List U64 -> List U64
reverse-copy input =
  let outer-heap = heap in
  with (heap = bump-arena (mebibytes 1)) in
    let temporary = reverse-into input Nil in
    evacuate outer-heap temporary
```

Called with a `List U64` containing `[10, 20, 30]`, `reverse-copy` returns `[30, 20, 10]` in the caller's heap:

| Step | Arena storage | Outer-heap storage |
| --- | --- | --- |
| Save `outer-heap` | No temporary list yet. | The input list belongs to the caller's heap. |
| Build `temporary` | Three new `Cons` cells contain `30`, `20`, `10`. | No result cells have been allocated here yet. |
| Evacuate | The temporary cells remain valid during copying. | Three new `Cons` cells form the independent result. |
| Leave the scope | Arena storage is released together. | The result survives and uses ordinary counted reclamation. |

The `U64` elements are scalars, so copying the list requires copying its cells and their scalar fields. For nested arena-backed data, copy-out must also copy reachable arena-backed children; copying only the outer record or list spine could leave dangling pointers. The destination argument makes allocation go to `outer-heap` even though `heap` is still bound to the arena.

Returning `temporary` directly instead of calling `evacuate` would be rejected: its cells belong to the arena. Merely rebinding the heap or retaining the temporary value cannot move those cells to the outer heap.

Copying takes linear time in the copied structure, and source and destination storage coexist until the copy finishes. The arena and outer budget must have room for that overlap. If destination allocation exhausts its budget, the exhaustion protocol cleans up the partial destination copy and tears down the arena while returning to the enclosing boundary; no partial or arena-backed result escapes.

# Counted pools

A counted pool reserves fixed-capacity storage for blocks of a particular size. Its blocks participate in reference counting and return individually to the pool's free list when their last owner releases them.

Choose a counted pool when values have different lifetimes and released capacity should be reused while the pool remains active. Choose an arena when all temporary storage can be discarded together. A pool must not be torn down while any of its blocks remain live.

Both mechanisms preserve allocation ownership. Releasing a pooled block returns capacity to that pool; it does not send the block to the ambient program allocator.

# Recoverable memory budgets

A computation without a static allocation bound can run inside a typed budget boundary. Exceeding the allowance returns `Exhausted` at that boundary. The proposed shape is:

```text
with (heap = budget (mebibytes 256)) in parse-all input
  :: Result Exhausted Summary
```

This sketch illustrates the exhaustion outcome; a parser's own error type must also be composed into the computation's result. The exact budget-boundary syntax remains open.

On exhaustion, the computation returns to the nearest enclosing budget boundary. Reference counting releases allocations inside the abandoned computation that are not reachable from outside. The caller can then handle the declared failure, for example by rejecting an oversized request.

Budgeting and arena lifetimes answer different questions: the budget controls how much storage a computation can hold; the arena controls how long its storage remains valid. A memory allowance does not authorize a scoped pointer to escape.

At the program boundary, exhaustion is an implicit declared outcome. Programs that need to recover within a larger service use explicit inner boundaries. The design treats resource exhaustion as a declared outcome or proves it impossible, rather than allowing an undeclared memory fault.

# Proven bounds and startup pools

For programs that must guarantee sufficient memory after initialization, the compiler proves allocation requirements and reserves storage before user code begins. The intended `jpl` discipline combines three forms of evidence:

| Assertion | Intended guarantee |
| --- | --- |
| `#[no-alloc]` | The computation requires no allocation capability. |
| `#[fip]` | Fully-in-place execution reuses uniquely owned cells without requesting fresh storage. |
| `#[alloc("bounded(n)")]` | Allocation stays within a statically proved bound. |

The bounded-allocation spelling is illustrative. Bounds are established through the recursion and arithmetic proof machinery, not accepted as unchecked programmer claims.

Fully-in-place checking connects allocation guarantees to the reclamation pass's reuse analysis. A function consumes uniquely owned cells and reuses their storage for compatible replacements. Functional transformations can therefore run within reserved storage without requiring a fresh allocation for every newly described value. An owned-list transformation is a candidate when its ownership and cell-reuse obligations can be proved; functional syntax alone does not establish the guarantee.

The startup pool is sized from the program's proven component bounds. Definitions reachable after initialization must satisfy the no-allocation, fully-in-place, or bounded-allocation discipline against that pool. Reserving storage can fail before the program starts; after successful reservation, the proven workload cannot exhaust it.

This is stronger than measuring a typical run or selecting a fixed pool and hoping it is large enough. The capacity is justified by the program's proof obligations.

# Headers, heap caps, and compatibility

Allocator customization preserves the shared value representation: a header carries reference-count, kind, and layout information, while allocator ownership is recovered separately.

The design's default header assigns 40 bits to the count, 8 to the kind, and 16 to the layout identifier. A compiler build can trade count width for a wider layout identifier. The heap cap is tied to that choice: it stays below `8 * 2^count_bits` bytes so the number of pointer words cannot overflow the count field.

Changing those widths changes the runtime ABI. Compiled objects record the choice, and objects with incompatible header formats must not link together. Selecting a different allocation policy alone does not change the header format.

See [the allocator design notes](../design/allocators.md) for the underlying decisions, [reclamation](../design/reclamation.md) for block ownership and reuse, and [proof automation](../design/proof-automation.md) for the machinery used to establish bounds.
