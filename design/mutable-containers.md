# Mutable containers: the slot core and the `Data.Mutable` family

Status: built, 2026-11-14. Part 1 (the trusted slot intrinsics) and part 2
(seven containers and a facade) are in the tree and tested; section 9 states
what remains, including the one thing the stage chain needs before a
stage-3-built program can import the family.

Companions: `design/arraylist.md` (the growable sequence this note delivers,
and the reasons the in-place `array-set` could not be decided at run time),
`design/linear-in-place.md` (why reuse, not linearity, is what pays),
`design/reclamation.md` (the header word, the counts, the free walk),
`design/ownership-and-usage.md` (regions).

Conventions: `path` cites this tree on 2026-11-14. Every number has its
command in section 10. No em dashes.

---

## 1 The short version

| Claim | Evidence |
| --- | --- |
| Mutable storage needs no new block kind and no ABI bump | a slot array is a `KIND_ARRAY` block with a word format; holes are word 0, which every count path already skips (section 2) |
| The trusted core is nine intrinsics, private to each module that uses them | `compiler/src/slots.rs`; `lib_mutable::the_trusted_core_never_leaves_its_modules` |
| Purity is still a checked property | operations carry `Mut r`; a pure caller is refused with "effect rows do not match" (`lib_mutable::a_pure_function_cannot_touch_a_container`) |
| The hash table is SwissTable's probe with F14's deletion | 8-slot chunks, 7-bit tags matched in SWAR, overflow counts instead of tombstones (section 5) |
| Delete churn does not grow the table | twenty rounds of 5,000 inserts and deletes leave the capacity unchanged (check 6 of the behaviour program) |
| Every container check passes leak-free, under count debugging and at `-O2` | `cinch-leak: live=0 immortal=0 leaked=0` in all three modes (section 8) |
| The mutable table beats the persistent map by 7 to 13 times | 1M scrambled keys (section 7) |
| It is within 2x of abseil on insert and 4x on lookup | the same benchmark against `absl::flat_hash_map` (section 7) |

## 2 The trusted core

`design/arraylist.md` section 2.6 established that an in-place write cannot
hide behind `Array a`'s immutable signature: the decision belongs to code that
owns the block outright. This note gives that owner a name. A container module
allocates a slot array, never hands out an `Array` alias of it, and writes it
in place; nothing else can observe the write, so it is not a mutation of a
value anyone else holds.

The storage is the existing `KIND_ARRAY` block with a word element format (0
raw, 1 counted object, 2 counted-or-tagged), so the drop walk, the free walk,
mark-on-spawn and the leak accounting all read it with the map they always
had. The one new state is the **hole**: a counted slot holding word 0.
`rc::is_block_word` already rejects 0, so a hole is free to drop, free and
scan, and `cinch_slots_new` allocates a block of holes (the array allocator
zero-fills). No block kind, header field or extern changed, so the runtime ABI
stays 9.

The intrinsics (`cinch.runtime.slots*`, `compiler/src/slots.rs`):

| Intrinsic | Signature | Emitted as |
| --- | --- | --- |
| `slotsNew` | `U64 -> Array a` | `cinch_slots_new(n, format)` |
| `slotsLen` | `Array a -> U64` | one load |
| `slotsRead` | `Array a -> U64 -> a` | bounds compare, load, hole check, dup for a counted class |
| `slotsWrite` | `Array a -> U64 -> a -> Unit` | bounds compare, store, release the old word; consumes the value |
| `slotsTake` | `Array a -> U64 -> a` | bounds compare, load, hole check, store 0; the reference moves with no count operation |
| `slotsMove` | `Array a -> U64 -> Array a -> U64 -> U64 -> Unit` | `cinch_slots_move`: memmove with each reference owned by exactly one slot before and after |
| `slotsClear` | `Array a -> U64 -> U64 -> Unit` | `cinch_slots_clear`: drop a range to holes |
| `slotsFoldNat` | `Nat -> U64` | the immediate inline, `cinch_nat_low_word` for a bignum |
| `slotsPrefetch` | `Array a -> U64 -> Unit` | `llvm.prefetch` (read, high locality) when in range |

Like the array and cell intrinsics, each is a unit specialized by class vector
(`specialize.rs`), so the element class is read off the vector and never off a
type's spelling. Indices are machine `U64` words because the containers compute
them with masks and shifts.

### 2.1 Why a read of a hole must never be exported

A counted hole is detectable (word 0 is no block), and the inline read and take
paths trap on one through `cinch_slots_fault` ("read of an empty mutable
slot"). A raw slot has no hole to detect: every word is some raw value. That is
harmless for a plain `U64` and unsound for a refined one: an
`InRange 1 10 U64` read out of an unwritten slot would be a zero carrying a
proof that it is at least one, and a refinement is exactly what licenses a
`bytes-get` without a bounds check. So the rule is the same one `Effect.Stm`
follows for its cell intrinsics: **each container module declares the
intrinsics it needs with a private `foreign import ccall` and exports none of
them**, and each module states the invariant that makes its reads sound (a
length, a ring range, control bytes). The test
`the_trusted_core_never_leaves_its_modules` enforces the export half.

The trap is defence in depth, and it earned its place during development: the
first build of `heap-from-list` wrote elements before it updated the length, so
a growth moved only the counted prefix, and the next sift read a hole. The trap
fired with a named message instead of a null block word reaching a count.

### 2.2 The ownership facts the emission relies on

* A primitive's counted parameters are borrowed unless the primitive consumes
  them (`rc::borrow_modes`), so `slotsWrite` consumes its value (`consumes(2)`)
  and stores the word with no dup/drop pair, and the array parameter is lent.
* The stack-allocation pass never hands a frame block to a primitive except as
  a lent argument (`stack.rs`, `callee_consumes`), so a stored value is always a
  heap word.
* The simplifier moves and drops only values (`simplify.rs`, `speculatable`),
  never calls, so direct-style mutation stays in program order; this is what
  `bytes-set` already relied on.
* The unit scan counts a store, an extern call and the fault call as writes, so
  every slot unit is `memory(readwrite)` and LLVM never deletes or reorders one.
  `llvm.prefetch` is classified as an extern for the same reason: a pure call
  with an unused result is dead, and the hint would vanish before inlining.

## 3 The effect discipline

Every container is indexed by a region `r` and every operation carries the
`Mut r` label. `Mut r` is a capability exactly like `IO r`: a definition whose
row lacks it cannot call a container operation, so a pure function stays pure
and a mutating one says so. Labels need no declaration in this checker (a row
names them freely), so the family exports none.

There is no `run-mut :: (forall r. Unit ->{Mut r} a) -> a`, the `ST` runner,
and the reason is a checked fact rather than a preference: the checker refuses
the rank-2 argument ("quantifiers are only supported at the outermost position
of a signature"), and a handler cannot stand in for it, because a handler only
dispatches operations its handled expression performs directly (D23), never
ones a called container operation performs. Without the quantifier a runner
would let a container escape the scope that ran it and be mutated under a pure
type. When the checker accepts the rank-2 argument, the runner is a two-line
addition and no container signature changes.

## 4 The family

| Module | Shape | After |
| --- | --- | --- |
| `Data.Mutable.Vec` | dense slots, length word, doubling growth (floor 4); introsort | `std::vector`, Rust `Vec` |
| `Data.Mutable.HashTable` | section 5 | `absl::flat_hash_map`, `folly::F14ValueMap` |
| `Data.Mutable.HashSet` | the table with a `Unit` value | `absl::flat_hash_set` |
| `Data.Mutable.Deque` | power-of-two ring; growth unwraps in two moves | Rust `VecDeque` |
| `Data.Mutable.Heap` | 4-ary heap, ordering fixed at construction, hole sifting, Floyd build | `boost::d_ary_heap` |
| `Data.Mutable.Array` | fixed length, every slot always written | `std::array` by reference, `MutableArray` |
| `Data.Mutable.Var` | one written slot | `IORef`, `STRef` |
| `Data.Mutable` | the facade (`export module` of all seven) | |

Common to all: a growth MOVES elements (`slotsMove`, one word copy each, no
count touched), which is why doubling rather than folly's 1.5x is the factor:
the factor trades copy work against slack, and the copy is already the cheap
kind. Every module re-reads its storage on each step of an effectful
iteration (`vec-for-each`, `deque-for-each`), or iterates a snapshot list
(`hash-table-for-each`), so an action that mutates the container it is
iterating cannot be handed a moved-out slot. Pure folds read a snapshot and
cannot mutate anything.

`vec-freeze` and `mut-array-freeze` copy into a fresh block with every slot
written, which is an ordinary `Array a`: the immutable family reads it with
no knowledge of where it came from, and no later write reaches it.

The sort is introsort: median-of-three Hoare partition with explicitly
bounded scans (the partition does not rely on sentinels surviving swaps),
insertion sort below 17 elements, heapsort past a `2 floor(log2 n)` depth, and
recursion into the smaller side so the stack is O(log n). The behaviour
program sorts random, sorted, reversed and constant inputs and a `String`
vector through a custom comparator.

## 5 The hash table

**Layout.** Slots come in chunks of eight. One `Array U64` holds the entry
count (word 0), the chunk mask (word 1) and, per chunk, its control word and
its overflow count side by side (words `2 + 2g` and `3 + 2g`), so a probe reads
both from one line. A control word has one byte per slot: 0 for empty,
`0x80 | t` for a full slot whose key's hash has 7-bit tag `t`. Keys and values
are two parallel slot arrays. At most seven entries per chunk (7/8 load), and a
power-of-two chunk count.

**Hashing.** `Hash` gives a `Nat`, and the shipped `Hash Nat` is the identity,
so the table folds the digest to 64 bits (`slotsFoldNat`) and runs splitmix64's
finalizer over it. The tag is the top seven bits; the home chunk is the low
bits under the mask.

**Probing.** SwissTable's group match in SWAR: `x = ctrl ^ (t * 0x0101..01)`
has a zero byte exactly where a tag matches, `(x - 0x0101..01) & ~x &
0x8080..80` lights those bytes (a rare false positive is settled by `Eq`, never
a false negative), and the trailing-zero count over eight is the slot. A miss
moves to the next chunk by the key's odd stride `2t + 1` (F14's double
hashing), which visits every chunk of a power-of-two table. The walk is also
bounded by the chunk count, so it ends even on a table whose counts were wrong.

**Deletion without tombstones.** F14's overflow counts: every chunk counts the
entries whose probe passed it because it was full. A lookup stops at the first
chunk with no match and a zero count, and a delete clears its byte and retraces
its key's probe from the home chunk, taking back the count of every chunk it
passed. A delete-heavy workload therefore never accumulates tombstones, never
needs a same-size rehash, and keeps absent-key lookups short. Check 6 of the
behaviour program asserts the consequence: after twenty rounds of 5,000
inserts and deletes on a 20,000-entry table, the capacity is the one it had
before.

**Prefetch.** Once the digest is known, the home chunk's key and value lines are
prefetched while the control word loads, so the three misses of a hit overlap
(F14 prefetches the same way).

**Soundness.** A slot is read only when its control byte says it is full; a
slot is written before its byte is set, and a byte is cleared before its slot
is taken. Growth clears each old control word before it takes that chunk's
slots.

## 6 What made it fast: three findings worth keeping

**Borrow inference withdraws a lent mode on one bad call site.** A callee's
parameter is lent only while EVERY caller passes it a variable, and, in tail
position, one the caller itself borrows (`rc.rs`, `call_sites`). One
`slots-read (store-of v) i`, or one tail call passing an owned local, withdraws
the borrow program-wide, and then every slot access pays a dup in the caller
and a drop in the unit. The containers therefore let-bind storage before every
slot call and keep slot calls out of tail position when the array is an owned
local. That is a general rule for any library code that wants its helpers'
parameters lent, and worth knowing beyond this family.

**The free walk re-derived every array slot's code out of line.**
`Scan::counted_below` handled a non-inline map by calling `decode_below`, which
re-read each slot's code; for a raw array it scanned every slot to find none.
Growth frees large all-hole arrays, so this was 13% of a 2M-key build's
instructions. An array's map is one code for word 2 onward, so `counted_below`
now answers it inline: the next counted slot is the one below the cursor, and a
raw or packed array has none. This benefits every `Array` in every program.

**Generation reads.** Each table operation reads the current generation once
and hands it to a helper that takes it lent (the call is not a tail call, so
the mode survives), and an insertion reports its outcome as one word (a slot,
`placed`, or `absent` for "grow first") so a present key's update allocates
nothing.

## 7 Measured

Provisional: measured 2026-11-14 on a 4-core cloud VM while the stage ladder
and regression suites were running (load average about 3), so absolute figures
are noisy; the ratios are the claim. One million keys, `--opt-level 2`, the
release runtime, clang 19, abseil 20220623. Commands in
`experiments/mutable-containers/README.md`.

| Phase (ns/op) | `Data.Mutable.HashTable` | `Data.HashMap` (persistent) | `absl::flat_hash_map` | `std::unordered_map` |
| --- | --- | --- | --- | --- |
| insert | 214-295 | 2,253 | 128 | 134 |
| lookup, hit | 132-155 (256 before the prefetch) | 1,716 | 36 | 13 |
| lookup, miss | 47-85 | | 13 | 15 |
| delete half | 336-390 | | 52 | 48 |

| Phase (ns/op) | Cinch | C++ |
| --- | --- | --- |
| `vec-push` / `push_back` | 28 | 1 |
| `vec-sort` / `std::sort` | 376 | 66 |
| `heap-push` / `priority_queue::push` | 51 | 6 |
| `heap-pop` / `priority_queue::pop` | 675 | 130 |

At 50,000 keys (cache resident) a hit costs 82 ns against abseil's 9, so at a
million keys roughly two thirds of the gap is memory (three arrays per hit,
section 9 item 2) and one third instructions, mostly the `Hash` and `Eq`
dictionary calls (section 9 item 3).

## 8 Gates

| Gate | Result |
| --- | --- |
| `cargo test --release --test lib_mutable` | 6 passed |
| behaviour program, `CINCH_LEAK_CHECK=1` | exit 0, `live=0 immortal=0 leaked=0` |
| behaviour program, `CINCH_RC_DEBUG=1` (freed blocks poisoned, every count checked) | exit 0, leak-free |
| behaviour program at `--opt-level 2` | exit 0, leak-free |
| `cargo test --release --lib` in `compiler/runtime` | 152 passed, 1 failed: `only_the_sys_layer_declares_foreign_functions`, which fails identically on the unchanged tree (`trace.rs` declares `_dyld_get_image_vmaddr_slide`) |
| neighbouring compiler suites | section 10 |

## 9 What remains

1. **The stage chain.** The family is a "ship source" family: a client copies
   the modules beside its entry. A stage-3-built program would compile them
   with the Cinch compiler, whose intrinsic table (`Bootstrap.Builtin`,
   `Bootstrap.RuntimeIr`) does not know `cinch.runtime.slots*`, and it links the
   pinned runtime, which predates `cinch_slots_*`. So stage 3 cannot build a
   `Data.Mutable` client yet. The work is the bootstrap's lowering of the nine
   intrinsics (the same IR `slots_codegen.rs` emits) plus a re-pin of both locks
   to a revision carrying the runtime entries.
2. **Key/value co-location.** A hit touches three arrays. Interleaving keys and
   values in one slot array would save a miss whenever the two classes agree,
   but one array has one element format, so it needs a class-dependent layout
   the library cannot branch on today.
3. **Dictionary calls.** `Hash` and `Eq` are called through dictionary apply
   stubs, about 47 instructions per `hash`. Specializing the table per instance
   (or inlining it at a static dictionary, where `direct_dict_calls` would fire)
   is a compiler change, not a library one.
4. **`run-mut`**, once the checker accepts a rank-2 argument (section 3).

## 10 Reproducing

The behaviour program and its harness:

```
cd compiler && cargo test --release --test lib_mutable
```

By hand (the non-embedded closure is copied beside the entry, as a client does):

```
mkdir /tmp/mut && cd /tmp/mut
for m in Vec HashTable HashSet Deque Heap Array Var; do
  cp ~/cinch/compiler/prelude/Data/Mutable/$m.cinch Data.Mutable.$m.cinch; done
cp ~/cinch/compiler/prelude/Data/Array.cinch Data.Array.cinch
cp ~/cinch/compiler/tests/fixtures/mutable_containers.cinch Main.cinch
CINCH_LEAK_CHECK=1 ~/cinch/compiler/target/release/cinch run Main.cinch
CINCH_RC_DEBUG=1 CINCH_LEAK_CHECK=1 ~/cinch/compiler/target/release/cinch run Main.cinch
```

The benchmark is `experiments/mutable-containers/` (its README has the
commands).
