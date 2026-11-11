# Arenas for short-lived compiler data

Status: built and measured, 2026-11-11. The arena serves constructor cells
from the runtime and the compiler wraps one phase in it; section 8 is what
that measured. The blockers of section 4 are resolved (F1 by the `uses`
declare slices, F3/F4 by the ABI 9 pin bump), and the representation is the
one section 3.5 expected: `RC_IMMORTAL` headers, not a new block kind.
Companions: `design/compilation-speed.md` (the measured profile this
follows), `design/allocators.md` (the `Heap` capability and slice A3, which
is the prerequisite), `design/reclamation.md` (the header word and the
immortal count), `design/stack-allocation.md` (the other answer to the same
measurement).

Conventions: `path:line` cites this tree on 2026-11-11. Measured numbers are
commands the lane ran, listed in section 7 so they can be repeated.
`[INFERENCE]` marks a claim derived rather than observed.

---

## 1 Verdict

The design is written out here (section 3) and is sound in the language's own
terms. It cannot be built in this tree today, and the obstacle is not the
design: it is that allocation is owned by the runtime, and the runtime this
lane compiles against is pinned by two lock files that a source-only change
cannot move.

Five findings, each demonstrated in section 4:

| # | Finding |
| --- | --- |
| F1 | A `prim` body, the one in-source escape to target IR, cannot obtain memory: allocation symbols are refused, `cinch_*` is refused, and a `uses` entry emits a contract but no `declare`, so a call to libc does not assemble. |
| F2 | No block-shaped value may enter or leave a `prim`: its arguments and result must be machine scalars (`check_decls.rs:23282`). So an arena cannot be handed a value to store, nor return one. |
| F3 | The runtime every stage and every gate links is the pinned library producer's, not this tree's (`tooling/bootstrap_objects.py:113-152`, `tooling/bootstrap.py:744-757`). A change under `compiler/runtime/src` cannot reach the measured compiler. |
| F4 | The runtime ABI is pinned twice: `Bootstrap.RuntimeAbi.cinch` must equal the library provider lock's `runtime_abi` (`tooling/bootstrap.py:407-421`), and `bootstrap/stage0.lock.json` pins the seed. A new block kind is a seed bump plus a producer re-pin, which is the workflow `bootstrap/CHECKPOINT.md` records for ABI 5 through 8. |
| F5 | The population an arena would free is not a bulk. 85.9% of the self-check's blocks are constructor cells spread across elaboration and string decoding, not a phase output tree (section 2). |

What follows from F5 changes the order things should be built in, and that is
the most useful result in this note. The prize is the count traffic, not the
teardown, so the first thing worth having is a measurement profile that
allocates the frontend immortal and never frees (section 6.1); but the same
measurement shows that profile costs 170 times the resident set, so the
teardown is load-bearing and the per-module scope is part of the conversion
rather than a later refinement. And the language already ships one immortal
arena, `Sym`, which is the right home for the population section 2's caller
table puts at the top (section 6.2).

## 2 Measured state

Stage1 (the Cinch compiler source built by the seeded Rust compiler, PGO,
`-O2`) checking the 40,681-line 46-module compiler bundle against the 59
pinned interfaces, `CINCH_ALLOC_KINDS=1`:

| kind | blocks | bytes |
| --- | --- | --- |
| ctor | 1,067,160,595 | 23.36 GB |
| closure | 99,957,285 | 4.05 GB |
| nat | 61,020,981 | 1.95 GB |
| bytes | 14,540,307 | 0.35 GB |
| int | 34,920 | 0.001 GB |
| total | 1,242,714,088 | 29.71 GB |

CPU 82.8 s user, 91.5 s wall. Size classes: 24 bytes 652.3M blocks, 16 bytes
388.6M, 40 bytes 104.6M, 32 bytes 74.7M, 48 bytes 9.9M. Almost nine in ten
blocks are two- or four-word constructor cells.

A 40 s `sample` of the same run (65,416 samples on working threads) puts
13,927 of them inside the runtime and outside the compiler's own frames. The
nearest Cinch caller of each, by the tool's own accounting
(`tooling/alloc_sample.py`, which counts any `cinchruntime` frame as runtime
time, so this is runtime cost and not allocation counts):

| caller | samples | share of runtime samples |
| --- | --- | --- |
| `bytesCompare` | 3,094 | 22% |
| `natCmp` | 2,367 | 17% |
| `natSub` | 1,125 | 8% |
| `hash-bytes-from` | 724 | 5% |
| task spawn | 710 | 5% |
| `app` | 660 | 5% |
| `Cons` | 420 | 3% |

The phase frames of `tooling/alloc_sample.py` matched nothing on this binary:
PGO inlined `check-source-phase`, `elaborate-program` and the rest, so the
sample carries their callees (`Bootstrap.Infer.check-definition`,
`Bootstrap.Type.normalize`, `Bootstrap.Type.unify`) and not the phases.
[INFERENCE] The phases are therefore priced from
`design/compilation-speed.md` section 1 (infer 67 s of 195 s, interface
export 34 s, Core checks 24 s, resolve 23 s, RuntimeIr 22 s) rather than
re-derived here.

The two facts that decide the design are in this section. First, 85.9% of the
traffic is one kind, so an arena that cannot hold a constructor block cannot
touch the traffic. Second, the traffic is per-operation: the census of
allocation sites in `design/compilation-speed.md` section 2.2 names
`Data.String.utf8-at`, `Data.Char.char-from-code`, `Data.List.reverse-into`,
`Bootstrap.Type.read-carrier-tree`, `Data.String.unpack-from`,
`Bootstrap.Load.value-exports` and `Data.Core.Cons`, which is string decoding
and list construction at every call site, not one tree built once and dropped
once.

### 2.1 Baseline gates (before any change)

The same build, so a conversion has something to preserve:

| gate | result |
| --- | --- |
| `build --stage 3` | exit 0 |
| stage2 against stage3 `compiler.ll` | identical, 49,160,311 bytes, md5 `8623c62667f3dacea235cb63f83986ff` |
| stage2 against stage3 `.cinchif` | 46 files, 0 differing |
| `smoke --stage 3` | fib, precedence and refusal pass |
| self-check, peak resident set | 183,091,200 bytes (174.6 MiB) against 29.71 GB allocated |

## 3 The design

Written so that it is a transcription once section 5's prerequisite lands.

### 3.1 Storage

Storage comes from the runtime, because that is where allocation is. A
`KIND_ARENA` block (`KIND_LAST` is `KIND_COLUMNAR` = 16 at
`block_kinds.rs:106-108`, so 17 is the next free kind)
holds `{ next; end; chunks }`: a bump pointer, the end of the current chunk,
and the list of chunk pointers for teardown. `cinch_arena_new(bytes)` allocates
the first chunk from the ambient allocator; `cinch_arena_leave(word)` frees
every chunk and the arena block.

The in-source alternative, a base-library block of `Bytes`-backed slots, is
expressible today and is rejected here for a measured reason. `Bytes` is a
mutable unscanned buffer, so slots are bytes, and every 24-byte constructor
cell would need eight `bytes-set` calls with eight index obligations, each of
which boxes a `Nat` index (a `Nat` outside the immediate range is a
`KIND_NAT` block). It costs more than the allocation it removes. Section 4.5
records what the in-source route can and cannot do.

### 3.2 The handle

```
data Arena a where MkArena :: word -> Arena a
```

`Arena a` is an owned value, not a borrow, and it is not `Copy`: the checker's
existing ownership rules already give exclusivity, so a region parameter is not
needed to make the buffer unique. The type parameter `a` is the element type
the arena hands out slots for; it carries no runtime content.

### 3.3 What the handle guarantees

1. `arena-new n` returns an `Arena a` whose every slot is uninitialised.
2. `arena-slot arena` returns `(Arena a, Option (Slot a))`. `None` is
   exhaustion, a value, never a fault (`design/no-faults.md`).
3. A `Slot a` is only dereferenceable against the arena that issued it:
   `arena-get :: Arena a -> Slot a -> a` and
   `arena-set :: Arena a -> Slot a -> a -> Arena a`. The slot does not name its
   arena, so holding it without the arena is inert, and the checker refuses the
   use because the arena is a linear argument of every accessor.
4. `arena-leave arena` frees the storage. After it, no slot is meaningful and
   no `Arena a` exists to read one through, because the value was consumed.

### 3.4 Exhaustion

Chunked growth: `arena-slot` allocates another chunk (doubling, starting at
64 KiB) when the bump pointer reaches the end, and returns `None` only when the
heap cap refuses the chunk. There is one failure mode and it is typed. A
process-wide cap is already enforced by `CINCH_HEAP_LIMIT`, so a runaway phase
exits with the existing heap message rather than thrashing.

### 3.5 Representation: why dropping an arena value is a no-op

An arena block is allocated with a header whose `rc` field is `RC_IMMORTAL`
(`block_kinds.rs:136`, count 0), and its layout's pointer map is empty. That
gives the property this whole design exists for, and it is not a new mechanism:

- `cinch_dup` on an immortal block is a no-op (`rc.rs:201`, `rc.rs:231`);
- `cinch_drop` on an immortal block is a no-op, and the recursive release walk
  stops at it (the same test that keeps a static block safe, `rc.rs:647`,
  `rc.rs:1047`);
- mark-on-spawn stops at it too, so a value in an arena can be read by a task
  without a count.

So an arena value is an ordinary word pointing at an ordinary block with one
header field different. Every pattern match, projection and case reads it
exactly as it reads a heap block. The only difference is that no count moves
and no free happens, and the whole population dies with the chunk. In debug
builds the teardown counts the blocks still reachable from outside the arena
and reports a non-zero count as a compiler bug, never as a program fault.

### 3.6 Who frees it and when

The scope form that binds the arena is the unit of freeing: the arena's region
ends when the form's body does, and `arena-leave` is the last thing the form
does. For the compiler that means one arena per module in the frontend, entered
where the module is parsed and left where its Core is emitted, so the
population that a module's parse, resolve and elaboration build is freed once.
The emitted `Bytes` and the module's interface are copied out before the leave.

One structural condition applies and the tree does not meet it yet: the
pipeline is whole-program. `check-and-retain` takes the whole `Program` and
resolve, totality and elaboration each walk every module
(`Bootstrap.Check.cinch`, `Bootstrap.Pipeline.cinch`'s `check-only` path), so a
module's trees stay live until the compile ends. A per-module arena therefore
needs the phase schedule driven module by module first, which is the same
restructuring the parallel phase schedule needs
(`design/compilation-speed.md` section 12 item 10). Until then the scope is the
compile, which is what section 6.1 prices.

### 3.7 Why no reference can outlive the arena

Three rules. The first and third are checks the tree already performs; the
second is a property of the representation, which is why it needs no rule.

1. **Ownership.** The arena is an owned value with no `Copy` instance and no
   alias, so the buffer has exactly one handle. Dropping the handle frees the
   chunks; there is no second handle through which a freed chunk could be
   reached.
2. **Slots are inert without the arena.** A `Slot a` carries no pointer, so an
   escaping slot cannot be dereferenced: every accessor takes the arena as an
   argument, and the arena was consumed by `arena-leave`.
3. **Copied values are owned again.** `arena-get` returns a value with a count
   of one in the outer heap, so a value that crosses the boundary is a value
   that was explicitly copied out, the same shape the core specification
   requires for expensive copies.

Rule 2 is what makes this sound without a new checker rule, and it is why the
design does not hand out `Ref region a` views of arena memory. That route needs
the escape rule to hold between a function body and its declared signature, and
it does not (section 4.4).

### 3.8 What an arena must not hold

No memo cell, no knot, no `Delay` value whose region is outside the arena, and
no closure that captures an outer value by borrow. The first two are counted
kinds whose states are resumed by the runtime; a third is a region crossing.

## 4 Why it cannot be built in this tree

### 4.1 A `prim` body cannot obtain memory (F1)

`prim ... where llvm { ... }` is the one in-source route to target IR
(`design/inline-target-ir.md`). Containment for an `llvm` body is
`check_decls.rs:10684`, and it imposes:

- no `br`, `switch`, `invoke`, or label (`:10732`, `:10757`);
- no symbol reference other than `llvm.*` or a `uses` entry (`:10810`);
- a symbol starting `cinch_` or `cinch.` is refused as "a runtime call"
  (`:10825`) unless it is a `uses` entry;
- a symbol in `ALLOCATION_SYMS` (`malloc`, `calloc`, `realloc`, `free`,
  `aligned_alloc`, `posix_memalign`) is refused (`:10829`, `:23264`) unless it
  is a `uses` entry, because `uses` is matched first (`:10816`).

So `uses (malloc :: I64 -> I64)` passes containment. It does not link. The
contract is recorded as a `prim-use` foreign export in the module's interface
(`check_decls.rs:10592`) and nothing reads that kind: `grep -rn "prim-use"
compiler/src` finds only the producer. A body calling `@malloc` therefore
assembles against no declaration. Measured, `/tmp/arena/probe/Probe.cinch`:

```
cinch: clang failed to link .../program.ll:258821:17:
  error: use of undefined value '@free'
```

and the emitted module declares `@cinch_alloc_ctor`, `@cinch_alloc_raw`,
`@memcpy` and the counting entries unconditionally (`Probe.ll:5-17`) but
neither `@malloc` nor `@free`. So a prim can reach the runtime's own block
allocator and cannot reach an allocator of its own. An arena needs the second.

### 4.2 No value crosses a `prim` boundary (F2)

A prim's arguments and result must be machine scalars: `is_prim_abi_ty` is
`Ty::Prim(_) | Ty::Bool` (`check_decls.rs:23282`) and the envelope check
refuses anything else (`:10483`, `:10493`). There is no polymorphic prim,
because the signature must be monomorphic and closed (`:10474`). So a prim can
neither store a block nor return one, which closes the "write the block header
yourself" route: the arena cannot be handed a value.

### 4.3 The runtime is pinned, twice (F3, F4)

`tooling/bootstrap.py:744-757` (`runtime_archive`) links every stage against
the runtime built from the **seed snapshot**, which is the pinned git revision
in `bootstrap/stage0.lock.json`; `tooling/bootstrap_objects.py:113-152` builds
the library producer, its runtime archive and the library object from
`bootstrap/library-provider.lock.json`'s revision, and both files feed every
gate (`tooling/run_stage_targets.py:26-46`). Neither mentions
`compiler/runtime/src` in this tree.

The ABI itself is pinned three ways: `RUNTIME_ABIS = (5, 6, 7, 8)`
(`bootstrap.py:38`), `library_provider_lock` requires
`bootstrap/Bootstrap.RuntimeAbi.cinch` to be byte-equal to the rendered
version of the lock's `runtime_abi` (`bootstrap.py:407-421`), and
`collect_compiler_sources` renders that module from the lock rather than from
source (`bootstrap.py:497-499`). A new block kind is therefore a Rust-side
change, a new seed revision, a new producer revision, and a regenerated
interface set: exactly the ABI 5 to 8 workflow in `bootstrap/CHECKPOINT.md`.

### 4.4 No scope form binds a region for a user library (F4, second half)

Region borrows are real and documented (`reference/10-ownership-and-memory.md`
"Shared and exclusive borrows", with a runnable `Ref region`/`copyOut` example),
and the checker refuses a value that leaves its `withScope`
(`bootstrap/Bootstrap.Infer.cinch:4349`). But `withScope`, `spawn` and
`joinAllOrCancel` are *builtin* forms with task-shaped types
(`Bootstrap.Infer.cinch:4334-4370`, `Bootstrap.Builtin.cinch:182`), and there
is no user-definable scope form, so a library cannot open a region.

The escape rule also does not cover the shape an arena would rely on. Measured
with the installed checker:

<!-- reference example: a borrow of a local block, returned -->
```
bad :: forall region. Ref region (List2 Nat)
bad = borrow (Cons2 1 Nil2)
```

`cinch check` accepts this (`/tmp/arena/probe/Regions.cinch`, exit 0), as it
accepts the same shape over `Nat`. The rule that fires is about inner *scopes*
(memo cells, `Eval`, `withScope`), not about a body against its declared
signature. [INFERENCE] That is why an arena must not hand out `Ref` views:
their lifetime would rest on a check that does not hold in this position.

### 4.5 What the in-source route can do, and why it does not pay

The parts that do work, verified by `/tmp/arena/probe/Probe.cinch`:

- `prim raw-get :: I64 -> I64 where unsafe llvm { %p = inttoptr i64 %0 to ptr
  %v = load i64, ptr %p, align 8 ret i64 %v }` checks;
- a store needs an effect row and is then accepted
  (`prim raw-put :: forall r. I64 -> I64 ->{IO r} I64`);
- `Unit` is not a legal prim result (`:10493`), so a store returns the stored
  word;
- a body may call the runtime's own allocator entries because the module
  declares them unconditionally, but they allocate on the counted heap, which
  is the population the arena exists to leave.

So an arena of machine scalars is expressible and an arena of Cinch values is
not, and the measured population is 85.9% constructor cells. Building the
scalar arena would remove nothing that the profile names.

## 5 The prerequisite, precisely

Slice A3 of `design/allocators.md`, reduced to what the measurement asks for.
Four changes, in this order:

1. **Runtime** (`compiler/runtime/src`): `KIND_ARENA` at 17 (`block_kinds.rs`,
   `KIND_LAST` moves), `cinch_arena_new(i64) -> i64`, `cinch_arena_leave(i64)`,
   chunk growth and teardown through `heap::alloc_block`/`free_block` so the
   accounting and `CINCH_HEAP_LIMIT` keep working, and `cinch_alloc_ctor` and
   friends allocating from the ambient arena when one is entered. Blocks get
   `RC_IMMORTAL` headers and empty pointer maps.
2. **Intrinsics**: `cinch.runtime.arenaNew` and `cinch.runtime.arenaLeave` in
   the seed's builtin table (`bootstrap/Bootstrap.Builtin.cinch` mirrors it for
   the self-hosted compiler), plus the `--rc-debug`-style profile switch that
   selects arena allocation for a build.
3. **ABI and pins**: `RUNTIME_ABI_VERSION` to 9, `RUNTIME_ABIS` to include 9,
   a seed revision containing 1 and 2 (`bootstrap/stage0.lock.json`), a library
   producer revision containing them (`bootstrap/library-provider.lock.json`),
   and the 55 interfaces regenerated by the new producer.
4. **Base library** (`compiler/prelude/Data/Arena.cinch`): the handle of
   section 3, and the package's interface set regenerated so the compiler can
   import it. `collect_compiler_sources` bundles only `bootstrap/*.cinch` and
   five `Text` modules (`bootstrap.py:496-518`), so either the compiler's
   arena lives in `bootstrap/` or the bundle list grows.

Then one conversion, gated as the assignment specifies, with the phase chosen
by ownership boundary rather than by guess.

The minimum that unblocks a first measurement is items 1 and 2 plus a profile
switch: one arena entered at the start of the frontend and left at exit, no
scope form, no per-module schedule, no `Data.Arena` module (the compiler's
handle can live in `bootstrap/`). That measures the count traffic directly and
answers whether the rest of A3 is worth building. The cost is the resident set
discussed in section 6.1, so the measurement has to be taken on both a small
input and the self-check.

## 6 What to build first, given the numbers

### 6.1 The count traffic is the prize, and the teardown is what keeps it affordable

The measurement says the prize is the count traffic, not the free. The
compiler is a batch process: it checks one program and exits. Freeing at exit
is what the operating system already does, so an arena's *teardown* is worth
approximately nothing to it, and its *uncounted allocation* is worth the
12.4% of leaf samples in `rc::release` plus the dup and count traffic around
it.

That suggested a profile in the shape of the existing `--rc-debug`: allocate
the frontend's blocks immortal, never free, let `exit` reclaim them. It is
sound (an immortal block is never freed, so nothing can dangle) and it needs
item 1's allocator hook and nothing else. The memory says no. Measured on the
same self-check, peak resident set is 183,091,200 bytes, 174.6 MiB, against
29.71 GB allocated, so never freeing costs 170 times the resident set the
compiler needs today. `CINCH_HEAP_LIMIT` would refuse the run long before it
finished. The profile is a useful upper bound on the win (take it once, since
it isolates the count traffic with no scope machinery) and it is not a
shippable mode.

So the teardown is load-bearing, not incidental: the arena has to return its
chunks per phase for the compiler to stay inside a sane bound, which makes
section 3.6's per-module schedule part of the conversion and not a later
refinement. The order that follows is: a measurement profile for the upper
bound, then one arena per module once the phase schedule is driven per module,
then the interface set.

Two smaller levers come out of the same census and need no runtime change, and
they are what section 2's caller table names: string decoding that builds a
`List Char` per call (`Data.String.unpack-from`, `utf8-at`, `reverse-into`),
and name identity that compares spellings instead of interned ids
(`design/compilation-speed.md` section 12 item 7). Both remove constructor
blocks of exactly the kind the arena cannot.

### 6.2 The arena the language already has

`Sym` is an interned, never-freed, one-word handle to a name cell:
"Cells are immutable and never freed (the runtime's block discipline), so a
handle stays valid for the life of the process and sharing one is free"
(`compiler/runtime/src/sym.rs:8-13`). `Eq` is identity (one pointer test), `Ord`
is lexicographic by name and deterministic across runs, and literals intern at
compile time (`compiler/prelude/Data/Sym.cinch:1-12`). That is an arena with
the whole process as its scope, keyed by content, and it has exactly the
property section 3.5 wants: no count ever moves.

The self-hosted compiler does not use it. `Bootstrap.Inputs.cinch:32` already
lists `Data.Sym` among the interfaces the compiler checks against, and
`Bootstrap.RuntimeIr.cinch:302-304` classifies `Sym` as an immortal `Raw`
handle, so the type and its representation are in place; `grep -rn "\bintern\b"
bootstrap/*.cinch` finds no use, and names are `String` throughout (97
occurrences in `Bootstrap.Ast.cinch` alone). Section 2's caller table is what
that costs: byte comparison is the largest runtime caller (22%) and byte
hashing the fourth (5%), and both are the two things an interned identity
makes constant time.

[INFERENCE] Interning the name population is therefore the same lever as this
document, applied to the one population a batch compiler holds for its whole
life, and it needs no runtime, ABI or seed change. It is a large edit (every
name in the AST, the environments, the interface rows and the export tables)
and it should be measured as a sequence of conversions, not one change. It is
named here because it is the honest answer to "move the short-lived data off
the counted heap": for names, the arena already exists.

## 7 Evidence

```
CINCH_HEAP_LIMIT=2G python3 tooling/bootstrap.py build --stage 3   # stage1..3, fixed point
CINCH_ALLOC_KINDS=1 build/bootstrap/stage1/cinch --check \
  --interfaces build/bootstrap/stage2/base-interfaces.json \
  build/bootstrap/stage1/compiler.cinch /tmp/arena/self1          # section 2 table
sample <pid> 40 -file /tmp/arena/base/sample.txt
python3 tooling/alloc_sample.py /tmp/arena/base/sample.txt        # section 2 callers
```

Probes, all under `/tmp/arena/probe`, run with the installed checker:

| probe | result |
| --- | --- |
| `prim` with `load`/`store` through `inttoptr` | checks; store needs an effect row |
| `prim` with `uses (malloc :: I64 -> I64)` | checks; `emit-ir` leaves `@malloc` and `@free` undeclared; `run` fails at clang with `use of undefined value '@free'` |
| `prim` returning `Unit` | refused: "a prim body returns the declared result shape ... not Unit" |
| `bad :: forall region. Ref region (List2 Nat); bad = borrow (Cons2 1 Nil2)` | accepted, so the escape rule does not cover body against signature |

---

## 8 Built, and what it measured (2026-11-11)

The lane `dasl-arena-real` implemented sections 3.5 and 5, wrapped one phase,
and measured it. This section supersedes the "blocked" verdict of section 1:
what remains true from it is F5 and section 6.1's memory argument, which the
measurement in 8.4 confirms.

### 8.1 The representation actually built

`compiler/runtime/src/arena.rs`. `cinch_arena_new(bytes: i64) -> i64` enters
an arena on the calling thread and returns its record pointer as an opaque
handle word; `cinch_arena_leave(handle) -> i64` releases it and restores the
arena entered before it, so arenas nest. The record and every chunk are
[`heap::alloc_block`](compiler/runtime/src/heap.rs) allocations, so the heap
cap, the stats line and the leak check account for them; a chunk is a 16-byte
header (`next`, data size) followed by an 8-aligned data area, starting at
64 KiB and doubling to a 64 MiB cap.

No `KIND_ARENA` was added and no kind number moved. A served block is an
ordinary header block with its own kind and layout, so `case` reads its tag
and a projection reads its fields unchanged. Its `rc` is `RC_IMMORTAL`:
`cinch_dup` and `cinch_drop` return without writing, the inlined dup/drop
helpers of emitted code take their count-zero arms and never reach
`cinch_drop_slow`, the release walk and `cinch_frame_release` stop at it, and
mark-on-spawn stops at it. `cinch_free` already returns early on
`RC_IMMORTAL`. Zero lines of `rc.rs` changed for counting; the frame mark was
rejected for the reason section 3.5 gives and this confirms: `RC_FRAME`'s drop
is a payload *release*, so an arena block carrying it would recursively
release its tree on every drop instead of doing nothing.

`block::allocate_block` is the one hook: `KIND_CTOR`, `KIND_CLOSURE` and
`KIND_DICT` go to the bump path when an arena is entered and the caller did
not ask for zeroed memory. `Nat`/`Int` boxes, `Bytes` and `Sym` always come
from the heap: their payloads own Rust-side resources only the counted free
path drops.

### 8.2 The finding the unit test could not have made

The first build passed every byte gate and failed the leak check: 533,113
`live` blocks against 0 with `CINCH_ARENA=0`. Cause: a served cell can own an
outer heap block (a rendered `Bytes`, a boxed `Nat`, a `String`'s backing),
and since a drop of the cell is a no-op, nothing ever returned that child.
The arena's first draft said children "are freed normally", which is only true
when something else owns them.

Teardown now does what a frame block's release does: it walks the blocks it
served and drops each one's counted children exactly once
(`rc::release_children`, extracted from `cinch_frame_release`), then frees the
chunks. The arena records the payload pointer of every block it serves in a
`Vec` for that walk; `payload_bytes` would also recover sizes from the header,
but the side list cannot desynchronise. After the fix the gate reads
`cinch-leak: live=0 immortal=0 leaked=0` in both arms.

That teardown is not optional bookkeeping: without it the arena is a memory
leak of roughly a quarter of the `Bytes` traffic of the phase it wraps.

### 8.3 ABI 9 and the pins

* `interface::RUNTIME_ABI_VERSION` 8 to 9; `RUNTIME_ABIS` gains 9;
  `bootstrap/Bootstrap.RuntimeAbi.cinch` is the rendered 9.
* Both locks (`bootstrap/stage0.lock.json`, `bootstrap/library-provider.lock.json`)
  name revision `e3705d4f78bb5de5168f540e519e8bacc5a214fe` with source hash
  `0ebe3caf1980c0a7884730eba5265aa84a20c950598022f7bac324e0fae2c38a` and
  `runtime_abi` 9. The revision is a git commit object of the lane's
  `compiler/` tree, written with `git commit-tree` so no ref moved.
* The intrinsics: `cinch.runtime.arenaNew`/`arenaLeave` in the Rust
  compiler's `PrimOp::Platform` table (arity 1, signature `I64 -> I64`,
  declared `i64 @cinch_arena_new(i64)` / `@cinch_arena_leave(i64)`), in
  `Bootstrap.Builtin.platform-intrinsic` for the self-hosted compiler, and in
  `Bootstrap.RuntimeIr.declarations-counting`. The namespace is reserved
  (D73), so a program reaches them only by spelling the intrinsic.
* The mechanism is off by default: `CINCH_ARENA=1` enters a real arena and
  unset makes `cinch_arena_new` return 0, so every scope is a no-op and every
  block is a heap block. One build therefore has both arms, and the shipped
  default carries no allocation cost at all.

### 8.4 The phase, and the numbers

The arms below were measured before the default flipped: at that time unset
meant entered. 8.6 separates the two costs the numbers mix, and states the
shipped default.

The phase is interface serialization, entered after `export-interfaces`
(the `Json` documents are built before it and the manifest embeds them) and
left after the last `.cinchif` and the manifest are written
(`Bootstrap.Pipeline.serialize-artifacts`). It is the one finished phase in
this tree whose output is files, so nothing it serves outlives it by
construction; `write-source-artifacts` returns `Unit` so no value leaves.

Stage3 compiler, `--check`, `CINCH_HEAP_LIMIT=2G`, interleaved pairs on one
build.

Counting arms (`CINCH_ALLOC_STATS=1 CINCH_ALLOC_KINDS=1`, one pair each):

| workload | arm | blocks | bytes | peak live | header blocks | ctor blocks |
| --- | --- | --- | --- | --- | --- | --- |
| toolkit (3,940 lines) | off | 263,949,613 | 5,842,786,204 | 24.2 MB | 253,519,221 | 235,991,699 |
| toolkit | on | 262,609,433 | 5,908,618,422 | 106.9 MB | 252,179,021 | 234,780,462 + 1,211,234 arena |
| self-check (1.12 G blocks) | off | 1,117,248,587 | 26,437,113,944 | 154.2 MB | 1,082,593,416 | 957,480,192 |
| self-check | on | 1,112,939,710 | 26,572,619,922 | 364.4 MB | 1,078,284,517 | 954,986,310 + 2,493,879 arena |

So the phase is 1,340,204 of 263.9M blocks on the toolkit (0.51%; 1,211,234
of them constructor cells and the rest closures) and 4,308,904 of 1.117G on
the self-check (0.39%). The arena does what it was
built to do, exactly: the ctor count on the heap falls by the arena's count
and the totals are conserved. And the arithmetic in bytes goes the wrong way:
chunk granularity, the served-block list and the record cost more bytes than
the header-block accounting saved (+1.1% on the toolkit).

Timing arms (census off, `/usr/bin/time -l`, same build):

| workload | arm | CPU | wall | peak RSS |
| --- | --- | --- | --- | --- |
| toolkit, median of 3 interleaved pairs | off | 7.03 s | 7.12 s | 40.3 MB |
| toolkit | on | 7.30 s | 7.46 s | 85.4 MB |
| self-check, one pair | off | 83.19 s | 84.83 s | 183.7 MB |
| self-check | on | 127.48 s | 171.27 s | 356.9 MB |

The toolkit is the reliable row: the arena is **3.8% slower on CPU, 4.8% on
wall, and doubles resident memory** on a phase worth 0.5% of the traffic. The
self-check pair is one pair and confounded (its census run in the same session
showed the opposite CPU ordering, 90.6 on against 114.2 off), so it is
reported and not used. Where the cost comes from is structural: a bump
allocation is not cheaper than the per-thread size-class allocator for a
40-byte cell, the served-block `Vec` adds a store per block, the chunks are
cold pages (the RSS), and teardown walks every served block to release the
children that make the mechanism sound.

### 8.5 Verdict

A negative result with a clean mechanism. The arena is sound, uncounted,
tears down to `live=0 immortal=0 leaked=0`, and preserves every byte gate
(stage2 and stage3 `compiler.ll` and `cinch` are identical, 46 interfaces
identical, smoke passes), but the population a *sound* scope can reach in this
tree is 0.4 to 0.5% of the compiler's blocks, because the pipeline is
whole-program: every phase's output is retained, so the only escape-free scope
is a rendering step whose bytes are written and never read again. Within that
scope the counted-allocator win does not pay for the chunked memory, the
per-block bookkeeping and the teardown. Per section 6.1 the lever is the
schedule, not the allocator: a per-module phase schedule is what would let an
arena hold elaboration's traffic, and until then the mechanism has nothing
large enough to hold.

Two facts worth keeping from the exercise. First, `RC_IMMORTAL` is the whole
mechanism, so an arena needs no new kind, no new mark, and no change to any
count path. Second, an arena block that owns a heap block is a leak unless
teardown releases children; a runtime arena without that walk is not
shippable, and the leak check finds it in one run.

### 8.6 What the mechanism costs when it is not used

The hook is in `block::allocate_block`, on the path of every constructor,
closure and dictionary allocation, so the question that decides whether this
lands is what the hook costs when no arena is ever entered. The shipped
default is now off (`CINCH_ARENA=1` is the only value that enters one), which
makes that the ordinary configuration.

Two stage-3 binaries, same machine, no census variables, interleaved pairs,
`/usr/bin/time -l`, stage3 `--check`:

| workload | pairs | pre-change CPU (spread) | with the hook CPU (spread) | delta |
| --- | --- | --- | --- | --- |
| toolkit, arena never entered | 5 | 6.64 s (1.25) | 6.57 s (0.65) | -1.05% |
| toolkit, arena entered | 5 | 6.06 s (0.11) | 6.14 s (0.17) | +1.32% |
| self-check, arena entered | 3 | 62.27 s (5.93) | 62.48 s (10.00) | +0.34% |

The first row is the hook alone: 264 million blocks, 236 million of them
constructor cells, and the two spreads overlap. The second and third rows add
the arena's own work and are dominated by it (the toolkit phase is +3.8% when
on, section 8.4). Wall follows CPU in every row to within 0.2 percentage
points. Peak resident set is unchanged when the arena is not entered (40.1 MB
in both toolkit arms) and doubles when it is (84.8 MB).

So: the unused mechanism is inside noise, and the whole mechanism including
its one phase is inside the self-check's spread. The pre-change binary was
built from `f158d5c4` with its own pin into a separate `CINCH_BOOTSTRAP_BUILD`,
same flags, and both builds are PGO'd; the profile keys differ
(`14534dc6e637`/`2938100e49d0` against `ea93e7dd4a49`/`9fc4075e675d`) because
the sources differ, and each was trained on the same workload by the same
builder, so neither arm is an unoptimised build. A `--no-pgo` cross-check
would need two more chains and was not run.

Two caveats on the numbers. The self-check spread is 10% to 16%, so only the
toolkit rows carry signal, and its 0.34% is reported as "inside the spread",
not as a measurement of a difference. And the pairs were taken with other
lanes possibly running: `tooling/build_slots.py` did not exist yet, so nothing
excluded a neighbour's build from a pair; the interleaving is what makes the
comparison usable, and a quiet-machine rerun is the honest next step for the
self-check row.

**A separate finding from the rebase, not about this change.** Main's slot
semaphore changed the default stage link from one translation unit to
`min(cores/2, cores/slots)`, which on this machine is `-j 3`. With `-j 3` the
stage-1 profile *training* run of the instrumented link exits `-10` (SIGBUS)
after about 2.4 s, reproducibly, and no stage builds; with `-j 1` every stage
trains and links. The same instrumented binary linked as one translation unit
runs the training workload in 6.8 s and exits 0, so the fault is in the
sharded instrumented link or in its profile dump, not in the compiler's work
and not in this arena. The chain gates in 8.7 were taken with `-j 1`.

### 8.7 The chain gate on the rebased tree

`tooling/bootstrap.py build --stage 3 -j 1`, exit 0. Stage2 against stage3:
`compiler.ll` identical, `cinch` identical, 46 `.cinchif` identical.
`smoke --stage 3`: fib, precedence and refusal pass. Leak check on a check of
the toolkit bundle: `cinch-leak: live=0 immortal=0 leaked=0` with
`CINCH_ARENA` unset and with `CINCH_ARENA=1`.
