# A growable sequence, and the copy-on-write array it needs

Status: in progress, 2026-11-13. Part 1 is **built** (section 2.5): a second
runtime entry, `cinch_array_set_inplace`, plus a code-generation selection that
uses it where borrow inference proved the array operand is consumed. Section
2.4 records the earlier attempt, which tried to decide this at run time inside
`cinch_array_set` and was reverted because the answer is per call site. Part 2
(the scoped mutable array and `ArrayList`) is designed but not built; section 9
states what blocks it.

Companions: `design/arrays.md` (the `Array` family this extends),
`design/reclamation.md` (the header word, the counts, the ownership table),
`design/effects-capabilities-sequencing.md` (the row and handler machinery
part 2 uses), `design/compilation-speed.md` (the measurement part 1 answers).

Conventions: `path:line` cites this tree on 2026-11-13. Measured numbers are
commands section 10 lists so they can be repeated.

---

## 1 Why, in one measurement

A toolkit check (the five `Text.*` modules the compiler imports, 4,060 lines)
allocates:

```
$ CINCH_HEAP_LIMIT=2G CINCH_ALLOC_STATS=1 build/bootstrap/stage3/cinch \
    --check /tmp/toolkit-baseline.cinch /tmp/toolkit-diag.txt
cinch-alloc: blocks=3780103 bytes=88766606 peak=5906542
             header-blocks=3734847 header-bytes=88158064 immortal=0 freed=3734847 live=0
```

98.8% of those blocks are header blocks, so 99.4% of the bytes are headers
plus payload in one-word-or-two cells. A `List a` of n elements is n cons
cells: n headers and n payload words, and its length is a walk, its index is
a walk that rebuilds. A growable array of n values is one or two blocks, its
length is a field read, its index is a load.

The two things this note delivers attack that from both ends:

* **Part 1** makes the array the language already has stop copying on every
  write, which is what a fill loop needs. It is a runtime change with no
  source-visible signature change.
* **Part 2** adds the growable sequence itself, and the ownership mechanism
  that makes in-place growth expressible without I/O.

## 2 What `array-set` did, what the change made it do, and why it is back

`cinch_array_set` (`compiler/runtime/src/arrays.rs`) allocated a fresh block,
re-`keep`ed every element into it, wrote one slot and returned the new block.
So it was O(n) with n count increments, and a loop filling an n-element array
through it allocated n blocks and did n^2/2 count operations. `design/arrays.md`
states the behaviour the signature has: "setting an element returns a new
value".

The change made it branch on the block's reference count:

| count | path | blocks | count operations |
| --- | --- | --- | --- |
| `RC_FRESH` (1) | in place: `keep` the new value into the slot, `cinch_drop` the replaced element, `cinch_dup` the block for the result | 0 | 3 |
| above 1 | copy, as before | 1 | n `keep` |
| `RC_IMMORTAL` (0) | copy | 1 | n `keep` |
| negative (atomic) | copy | 1 | n `keep` |

The in-place path is three count operations regardless of n. The copying path
is one allocation plus one `keep` and one pointer-map-sized store per element,
so it exceeds a `memcpy` by exactly n in reference-count traffic: n `keep`
calls each read a header, add one and write it back, and n stores each write a
word the pointer map will later scan. The in-place path's three operations are
the three the convention requires (section 2.2), and 2.4 is why the third of
them cannot be decided at the runtime.

### 2.1 Why a count of one is the right test

`Array a` is presented as an immutable value. Writing in place is
observationally correct exactly when no other word can name the block. A count
of one says precisely that: the only reference being consumed is the caller's
argument, and the returned handle takes that reference over. Above one, some
other value may still read the block, so the write must copy, which is the
case the signature always described.

`RC_IMMORTAL` and negative counts are excluded deliberately. An immortal
block is a static, an interned `Sym` cell or an arena block, all shared by
construction, so writing one would corrupt data another part of the program
already names. A negative count is an atomic count, which means more than one
thread reached the block (`design/reclamation.md`), so the caller is not
exclusive.

### 2.2 The count convention the in-place path must satisfy

(Superseded by 2.4: this convention is what the wrapper needs, but it is not a
property the extern can rely on, because the wrapper's own drops are decided
per program.)

The primitive unit that wraps the extern owns its parameters and drops them at
`ret`: `prim_consumes` (`compiler/src/codegen.rs:1857`) keeps every array
operation's arguments, so `ret_drops` (`codegen.rs:3428`) releases them. The
extern therefore owes the same convention as the copying path: its result is
one owned reference, and every slot it writes holds its own reference.

The copying path gets that for free from a fresh block. The in-place path does
it explicitly, and the order is load-bearing:

1. `keep` the new value into the slot first, so a write of a value the slot
   already holds cannot free it before it is stored;
2. then `cinch_drop` the replaced element, which frees it when that was its
   last reference;
3. then `cinch_dup` the block, because the unit is about to drop the argument
   and the caller must still own the result.

Getting this wrong is a use-after-free or a leak, not a wrong answer, which is
why section 6's tests assert counts directly.

### 2.3 What part 1 does not do

No signature change, no type-system work, no source-visible difference. A
program that shares an array still observes persistence. A program that does
not share one now writes in place, which is the case where nobody could tell.

### 2.4 Why it was reverted: the wrapper's ownership is caller-dependent

The copy-on-write extern needs one thing from its caller: to know whether the
array reference it is handed is borrowed (so the result must be `dup`ed) or
consumed (so the result must not be). That answer is **not fixed by the
extern's signature**, and it differs between two programs compiled from this
tree.

`Data.Array.array-set@T` is a primitive unit. Its parameter ownership comes
from `lent`, the borrow-inference result for that unit
(`compiler/src/codegen.rs:1037`, applied at `codegen.rs:3428`), which is
computed **per program** from how callers use the argument. Two programs, one
with the array in a tail-recursive fill loop and one with the array in a
top-level definition read again afterwards, produce wrappers that differ:

```
$ grep -c 'drop.obj(i64 %p0)' prog.ll   # the fill loop
1
$ grep -c 'drop.obj(i64 %p0)' p2.ll     # the top-level array
3
```

So in one program the wrapper releases `%p0` after the extern returns and in
the other it does not. A single extern cannot satisfy both: `dup`-ing for the
result leaks in the first program, and not `dup`-ing frees the block under the
caller in the second, which is exactly what the failure looked like:

```
cinchruntime: compiler bug: corrupt array value
```

Two further failures the same change produced, both real and both fixed before
the revert, are worth recording because they are the shape any future attempt
will hit:

* `cinch_tensor_set` passes its tensor's **borrowed** data field to
  `cinch_array_set`. Under always-copy that was harmless; under copy-on-write
  the field's count is one, so the update wrote into the caller's tensor and
  `tensor-set` stopped being persistent. The fix is to hold a reference across
  the call (`cinch_dup` before, `cinch_drop` after), which makes the count two
  and forces the copy.
* A program that builds a list into a **top-level** array (a global `Array Int`
  defined by a list literal) crashed outright, because the wrapper for that
  program drops `%p0`.

The conclusion is not that copy-on-write is wrong. It is that the decision
belongs to the compiler, not the runtime: the code generator already knows, per
call site, whether it is handing over its reference (`prim_consumes` plus
`lent`). The sound shape is therefore a **separate primitive** with a
uniqueness contract the checker enforces (a `%1` array parameter, or the
region-scoped mutable array of part 2), rather than a runtime `rc == 1` test on
an operation whose signature says "immutable value".

The change was reverted so the tree stays green. Everything below that is
marked as measured was measured with the change in place; everything marked as
designed is unaffected by the revert.

## 2.5 What was built instead: two entries, chosen by code generation

The reverted attempt asked the runtime "is this block shared" and got an answer
it could not act on, because the wrapper's drops are decided per program. The
built version moves the decision to where the fact already is.

**Two runtime entries, one contract.**

* `cinch_array_set` is unchanged: it borrows its argument and returns a fresh
  block.
* `cinch_array_set_inplace` (new) borrows its argument and returns an owned
  reference too, but when the block's count is one it writes the slot in place
  and `cinch_dup`s the block for its result; above one it copies, which is what
  the copying entry does.

Both entries therefore have the same observable contract: borrow the array,
return an owned reference, and every slot holds its own reference. That is what
the wrapping unit imposes, and the emitted IR is the proof: `prim_consumes`
does not list the array operations, so `ret_drops` (`codegen.rs:3428`) releases
every counted parameter at `ret`. A version that consumed the argument instead
freed the block under the caller and trapped with `corrupt array value`, which
is how this contract was found.

**The selection.** `compiler/src/arrays_codegen.rs` chooses between them from
`Program::lent` (`crate::rc::borrow_modes`). `lent` holds an entry only for a
unit with at least one borrowed parameter, and its bool is true for borrowed,
so a unit with no entry, or an entry whose first word is false, is one whose
callers hand the array reference over. That unit takes the in-place entry; a
borrowing unit keeps the copying one. The choice is invisible to source: both
entries have the same signature and the same result semantics, and the
difference is only whether the write allocates.

**Why the count is still checked at run time.** Consumption is a property of the
reference, not of the block. A program may hold two words for one array and
consume one of them (`Shared3` in section 10), which is correct code with a
count of two at that call, so the entry must copy there rather than trap. The
count is therefore a fact the entry acts on, not an assertion it enforces. The
trap in the reverted version would have fired on a good program.

**`tensor-set`'s borrowed field.** `cinch_tensor_set` passes its tensor's data
field, which it borrows from the tensor, to `cinch_array_set`. Holding a
reference across the call (`cinch_dup` before, `cinch_drop` after) makes the
count two, so the update copies and the original tensor keeps its element. That
is a regression test (`tensor_set_leaves_the_original_tensor_alone`), because it
is exactly what an ownership-selected version must not reintroduce.

**What the selection actually reaches, measured.** The in-place entry fires
where the array operand is genuinely handed over, and `lent` is a per-unit
answer, so it does not fire in a recursive fill loop. Two programs show both
sides:

| program | `Data.Array.array-set@T` emits | why |
| --- | --- | --- |
| `M2` (one `array-set` on a top-level array) | `cinch_array_set_inplace` | no `lent` entry, so the unit consumes |
| `Main.step` (recursive fill loop) | `cinch_array_set` | the array parameter is live across the recursive call, so `lent` marks it borrowed |

So the entry is correct and reachable, but the loop shape this note most wants
to speed up is exactly the shape `lent` refuses: a tail-recursive accumulator
is live across its own call, which is a borrow by construction. Removing that
limit is not a runtime change at all. It needs the decision at the *call site*
(where `crate::rc` already knows whether it dups the argument) rather than per
unit, which is a code-generation change of its own and is why part 2's scoped
region exists: `with-array`'s callback receives the buffer at `%1`, so the loop
is not a borrow and the entry applies. Section 9 carries this as item 6.

## 3 The growable sequence

`Data.ArrayList`, backed by a new runtime block kind. The layout:

```
KIND_ARRAYLIST: { len; cap; format; elements... }
                 ^0   ^1   ^2      ^3 ..
```

`len` is the logical length, `cap` the allocated slots, `format` the element
representation the existing `Array` code uses (0 word, 1 counted object, 2
counted-or-tagged, `0x100 | width` packed scalar). Elements live inside the
block's payload, so the pointer map is the same shape `KIND_ARRAY` already has
and the drop and mark walks need no new code path.

Capacity above length is what makes append amortised O(1). The growth policy
is doubling with a floor:

```
new_cap = if cap == 0 then 4 else cap * 2
```

An append that finds `len == cap` allocates a block of `new_cap` slots, counts
the `len` live elements into it, releases the old block and appends. Total
copying over n appends is `4 + 8 + ... + n < 2n`, so n appends do fewer than
3n element moves and at most `log2(n/4) + 2` allocations: amortised O(1) per
append, which is the argument the doubling buys and a fixed increment does
not.

### 3.1 `push` mutates in place and returns the sequence

`push` has to mutate, because the whole point is that a fill loop allocates
once. But a plain `ArrayList a -> a -> ArrayList a` that mutates its argument
would be a lie under this language's model: `Array` is an immutable value and
`array-set` copies when shared. So `push` uses the same discipline part 1
introduced, at the sequence level:

* count one: write the element at `len`, bump `len`, return the same handle.
* otherwise: allocate `cap` (or doubled) slots, copy the live elements, write
  the new one, return the fresh handle.

That makes `push` a copy-on-write operation with the same observational
semantics as `array-set`, and it makes a builder loop linear because the loop
holds the only reference. It costs a count check per push, which is one header
load, and it means a program that keeps an old reference keeps seeing the old
sequence, which is the behaviour every other value in the language has.

### 3.2 Why this is not enough on its own, and what is

Copy-on-write gives linear append only when the caller is provably exclusive.
In this language a `let`-bound sequence that is pushed in a loop is exclusive
in fact but not in the type, so the runtime count is the only witness, and
that witness is dynamic: one stray reference turns the loop quadratic without
a type error.

The static answer is a **rigid region carried by an effect row and discharged
by a handler**, which this tree already implements and tests
(`compiler/tests/bootstrap_source_handlers.rs`, `compiler/tests/bootstrap_source_effects.rs:62`
refuses a wrong region identity):

```cinch
signature Mut (r :: Region) where
  read-at  :: forall a. MutArray r a -> Nat ->{|Mut r} Option a
  write-at :: forall a. MutArray r a -> Nat -> a ->{|Mut r} Unit
  grow     :: forall a. MutArray r a -> Nat ->{|Mut r} MutArray r a

with-array :: forall a b. Nat -> a ->
  (forall (r :: Region). MutArray r a ->{|Mut r} b) -> b
```

`with-array` allocates once, runs the callback with a fresh rigid `r`, handles
`Mut r` and drops the buffer. The region cannot escape because the callback's
type is `forall r`, so the handler is the only place `r` is discharged, which
is exactly what `design/effects-capabilities-sequencing.md` describes. No I/O
and no monad in the surface syntax: the effect row is `ST`'s `s`.

The row is `{|Mut r}` and not open because an open row would let the callback
perform effects the caller cannot see, and because a closed row is what makes
the region's scope the handler's scope. A caller that needs I/O says so in its
own row; the callback's row is the one the handler discharges.

`%1` on the handle is a separate question from escape. Linearity gives no
duplication, not no escape: a `%1` handle is used exactly once, and returning
it is a use. The skolem alone is what prevents escape, and `%1` on
`MutArray r a` is worth adding for the reason it is worth adding anywhere
(`compiler/prelude/Effect/Pty.cinch:40-47` documents the same reasoning for
terminal handles): it forbids aliasing a handle you hold. Whether the checker
accepts it on this type is unverified here, so the design does not depend on
it.

## 4 The API

| Operation | Signature | Cost |
| --- | --- | --- |
| `array-list-new` | `Unit -> ArrayList a` | O(1) |
| `array-list-push` | `ArrayList a -> a -> ArrayList a` | amortised O(1) |
| `array-list-get` | `ArrayList a -> Nat -> Option a` | O(1) |
| `array-list-set` | `ArrayList a -> Nat -> a -> Option (ArrayList a)` | O(1) exclusive, O(n) shared |
| `array-list-length` | `ArrayList a -> Nat` | O(1), a field read |
| `array-list-from-list` | `List a -> ArrayList a` | O(n), one allocation |
| `array-list-to-list` | `ArrayList a -> List a` | O(n), n allocations |
| `array-list-fold` | iteration, left to right | O(n), no allocation |

`get` returns `Option` because an index can be out of range and this language
does not fault on a total function (`design/no-faults.md`). `length` is a
field read and never a walk, which is the whole point: the compiler's
`list-length` sites currently walk.

`to-list` is O(n) and allocates n cons cells, so it is the wrong call when the
caller is about to fold, count, index or re-iterate. It is the right call at a
boundary whose type is a list, and at a point where the sequence is finished.
`from-list` is the mirror: one allocation, so it is the right call when a list
arrives and is then indexed or counted repeatedly.

### 4.1 What it does not do

No removal from the middle (`remove`, `pop`, `delete`), no insertion at an
index, no sorting, no searching, no stable order guarantees beyond insertion
order, no slicing or views, no capacity API, no shrink-to-fit, and no
`ArrayList` as a key or in an `Eq`/`Hash` instance. Those are each a design
question of their own, and none of them is needed for the retrofits that pay.

## 5 Retrofits

The compiler sites that build a list and then only index, count or reverse it:

| Site | What it builds | What it needs |
| --- | --- | --- |
| `Bootstrap.Load`'s interface row walk | rows accumulated reversed, then `reverse`d | order-preserving accumulation |
| `Bootstrap.Export`'s table construction | rows appended, then counted | length without a walk |
| `Bootstrap.RuntimeIr`'s unit emission loop | units accumulated, then reversed | same as the loader |
| `Bootstrap.Core` table builds | tables assembled by `Cons` then `list-length` | length without a walk |

Each retrofit is measured by the block and byte counts on the toolkit check
and the self-check, with output byte-identical (the fixed point is the gate),
and by the per-target suite against `bootstrap/stage3-suite-baseline.json`.

## 6 Tests

The structure's own tests, at the runtime level where the counts are visible:

* growth across several capacities: 0, 4, 8, 16, 32 slots, asserting the
  allocated capacity and that `len` tracks pushes;
* an empty sequence: `get` is `None`, `length` is 0, `to-list` is `Nil`;
* a million-element push loop, asserted with the block count rather than
  time: the count is the allocation count, which must be
  `log2(1000000/4) + 2` blocks, not a million.

Part 1's tests, which were built and passed with the change in place (they
went with the revert, since they assert behaviour the tree no longer has):

* `set_in_place_when_exclusive_and_copies_when_shared`: a fill loop over a
  fresh array allocates nothing and returns the same handle; with a second
  reference the write copies and both arrays keep their contents;
* `set_in_place_balances_counts_for_counted_elements`: counted elements move
  their references exactly once, and writing an element its own value does
  not free it;
* the existing persistence test was changed to make a second reference first,
  so it asserted the copy path rather than the only path;
* `tensor_set_leaves_the_original_tensor_alone`, which pinned the borrowed
  field bug of section 2.4.

## 7 The lock bump

`compiler/runtime/src` and `compiler/prelude` are seed sources: they are
hashed into `bootstrap/stage0.lock.json`'s `source_sha256` and pinned again by
`bootstrap/library-provider.lock.json`, and the runtime ABI is pinned a third
time by `Bootstrap.RuntimeAbi.cinch` (`design/arena.md` F4).

As delivered, **no lock needs bumping**: the runtime file is back to `HEAD`, so
the seed source hash is unchanged (verified: the tree's `seed_sources` hash
equals both locks' `source_sha256`).

For the work described and not built:

* Part 1 as designed needs a seed and producer re-pin but **not** a runtime ABI
  bump: no block layout, header field, kind number or extern name changes, so
  the ABI stays 9. What it does need is the compiler-side change of section
  2.4, because the ownership answer is per call site.
* Part 2 adds a block kind (`KIND_ARRAYLIST`), which is a runtime ABI change
  and a re-pin.

## 8 What part 1 can and cannot be measured on

The stage chain links the **pinned** runtime, not this tree's
(`tooling/bootstrap_objects.py` builds `libcinchruntime.a` from the pinned
snapshot; `tooling/bootstrap.py:744-757` links the stage compilers against
it). So a stage-3 binary cannot exhibit part 1's change until the locks move.
What can be measured now is the tree's own runtime through the seed compiler,
which is what section 10 records.

### 8.1 Measured: a 200-write fill loop

The seed compiler (`compiler/target/release/cinch`) building a program that
fills a 200-element array through `array-set` in a tail loop, run under
`CINCH_ALLOC_STATS=1`, with the runtime archive rebuilt from this tree:

| runtime | blocks | bytes | peak |
| --- | --- | --- | --- |
| always copy (before) | 409 | 329,957 | 3,509 |
| copy-on-write (after) | 209 | 5,157 | 1,885 |

The 200 difference is exactly the 200 copies the old path made. The remaining
209 blocks are the array, the `Option` `Some` cell each call allocates
(`Data.Array.array-set` boxes its result, which is the primitive's shape and
not part of this change), and the program's own cells. Bytes drop 64x because
each copied array carried 200 element slots.

The copying path is exercised end to end by a program that keeps the array
live across the update (`Shared3` in section 10): it observes the original
element unchanged and the copy changed, so persistence is intact where a
second reference exists.

### 8.2 What this measurement does not show

It does not show a stage-3 or self-check number, because those link the
pinned runtime (section 8). It does not show an amortised O(1) append, because
`Array` is fixed size and the growable sequence is part 2. And it does not
show a win on the compiler's own sources, because the compiler's hot list
builds do not currently go through `array-set` at all: they are `Cons` chains,
which is what part 2 exists to replace.

### 8.3 The revert, and what it cost

The numbers in 8.1 were measured with the change in place, on a runtime
rebuilt from this tree and driven by the seed compiler. The change is now
reverted (section 2.4), so the tree's `compiler/runtime/src/arrays.rs` is
byte-identical to `HEAD` and `cargo test --release --lib` in
`compiler/runtime` is 144 passed, 0 failed, with `CINCH_LEAK_CHECK=1`
reporting `live=0 immortal=0 leaked=0`.

What the attempt produced that survives:

* the measurement above, which is the evidence that the copying path is the
  cost worth removing;
* the two failure modes a future attempt must handle (`tensor_set`'s borrowed
  field, and a top-level array), both stated in 2.4;
* the conclusion that the decision belongs to code generation, where
  `prim_consumes` and `lent` already know it per call site, and not to a
  runtime count test on an operation declared as an immutable value.

No lock was bumped, and no lock needs bumping now, because the tree's seed
sources are unchanged.

## 9 What part 2 needs

`KIND_ARRAYLIST` is a new block kind, so:

1. `block_kinds.rs`: the kind number and `KIND_LAST`;
2. `block.rs`: its fixed scan map (elements from word 3, by format), which is
   the same shape `KIND_ARRAY` already decodes;
3. `arrays.rs` or a new `arraylist.rs`: the externs;
4. the seed and producer re-pins and a runtime ABI bump;
5. the compiler side: an intrinsic family, its scheme table, codegen and the
   bootstrap compiler's own lowering, which is the work part 1 avoided by
   changing no signature.

That is why part 1 is separated: it pays with none of that.

## 10 Reproducing

```
python3 -c "import sys;sys.path.insert(0,'tooling');import pgo;from pathlib import Path;Path('/tmp/toolkit-baseline.cinch').write_bytes(pgo.workload_source(Path('.').resolve()))"
CINCH_HEAP_LIMIT=2G CINCH_ALLOC_STATS=1 build/bootstrap/stage3/cinch --check /tmp/toolkit-baseline.cinch /tmp/toolkit-diag.txt
cd compiler/runtime && CINCH_LEAK_CHECK=1 cargo test --release --lib arrays
```
