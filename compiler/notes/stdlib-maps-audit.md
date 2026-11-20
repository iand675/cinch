# Maps operation audit

This is a static audit at published source
`bfb4cb57113016a6cb054fd5c460b6cea6fcf60d`, against the actual ten `maps`
records in the pinned `go1.27.4` SDK. It proposes implementation boundaries;
it adds no library declarations, changes no shared surface inventory, and
does not mark the package complete. No compiler, runtime, benchmark or
bootstrap job was run for this audit. All Ring production, rejected
extraction and storage-pinning candidate snapshots remain unchanged.

`../evidence/stdlib/stdlib-maps-audit.json` preserves the exact reference declarations, source
hashes, all five homes' exports and signatures, per-operation gaps and
future fixture requirements. Reproduce it without compiling anything:

```sh
python3 tooling/generate_maps_audit.py --sdk /tmp/cinch-go-current/go
python3 tooling/generate_maps_audit.py --sdk /tmp/cinch-go-current/go --check
python3 tooling/audit_stdlib_surface.py --go /tmp/cinch-go-current/go/bin/go --package maps
```

## Actual homes and capabilities

There is no `Data.Mutable.Map` or `Data.Mutable.HashMap` module at this
basis. The mapped mutable homes are `HashTable`, `BTree` and `IndexMap`.

| Home | Key identity and representation | Traversal | Duplicate construction | Counts / update cost |
| --- | --- | --- | --- | --- |
| `Data.Map` | `Ord k`, equal when neither key is less; unbalanced persistent tree, public `Tip`/`Bin` | Pure right fold and eager lists in ascending key order | `map-from-pairs` inserts the tail first: **first value wins** | Size O(n); lookup/update O(height), worst O(n) |
| `Data.HashMap` | `Hash k` and `Eq k`; persistent AVL index of digests with linear collision buckets | Pure right fold and eager lists; bucket order is not a contract | `hash-map-from-pairs`: **first value wins** | Size O(n); lookup/update O(log distinct-digests + bucket length) |
| `Data.Mutable.HashTable` | `Hash k` and `Eq k`; mutable chunked SWAR tag table with overflow counts | Pure full fold; eager list; effectful `for-each` over a captured entry list | `hash-table-from-list`: **last value wins** | O(1) count; expected O(1) lookup/update; full-collision degradation |
| `Data.Mutable.BTree` | `Ord k`; mutable pooled degree-eight B-tree | Ascending pure full/range folds, eager list; effectful snapshot `for-each` | `btree-from-list`: **last value wins** | O(1) count; O(log n) lookup/update |
| `Data.Mutable.IndexMap` | `Hash k` and `Eq k`; dense insertion-order storage plus SWAR index | Pure full fold/eager list in current dense order; **no effectful `for-each`** | `index-map-from-list`: **last value wins** | O(1) count; expected O(1) lookup/insert/swap-remove; shift-remove O(n) |

All mutable operations expose `Mut r`. Their pure fold callbacks cannot
run `Mut r` actions. HashTable's fold walks its selected generation with a
pure callback; it does not copy the entries into an immutable array first.
The snapshot claim relevant to callback mutation is implemented by
`hash-table-for-each` and `btree-for-each`: both call `to-list` before any
action. IndexMap shift-remove preserves relative order; swap-remove moves
the last entry into the hole; an overwrite keeps its position.

Capabilities are requirements on individual operations, not policies
stored inside the container. Ordered homes have no separate equality
dictionary; hash homes require equality and hashing separately. There is
no direct `new-by comparator` or `new-with-hasher` callback constructor.
Custom key types and their instances can choose policies, but all later
operations must use a consistent policy. A handle does not certify that
its next caller supplies the same dictionary used during construction.
`Ord` does not imply `Eq`, and `Data.Compare.Compare` implies neither.

Insertion also has observable representative-key differences: persistent
Map and HashMap replace the stored equivalent key along with its value;
HashTable, IndexMap and BTree keep the original key and replace only the
value. This matters for equivalent custom keys with distinct payloads and
signed-zero wrappers; fixtures must observe keys as well as values.

The source text's O(n log n) union claim for the unbalanced Map is not a
worst-case guarantee. A sorted insertion sequence can make its height n
and construction quadratic. A balanced persistent map would be a separate
collection improvement, not a prerequisite silently satisfied by this
audit or an excuse to change existing public constructors here.

## Ten operation records

The following are capability mappings, not ten implemented equivalents.
Pure return-value adaptations are idiomatic for persistent maps; mutation
and effectful iteration remain necessary on mutable homes.

| Reference record | Existing capability | Concrete remaining work |
| --- | --- | --- |
| `All` | Every home lists pairs and folds; HashTable/BTree have effectful snapshot actions | A reusable pair sequence/visit interface with immediate stop, explicit invocation timing and mutation policy. Unit-returning `for-each` cannot stop. |
| `Clone` | Persistent maps can safely share their immutable structure; mutable lists and fresh constructors compose | Named shallow cloning with fresh mutable cells, preserved managed payload owners and defined order/policy behavior. Aliasing a mutable handle is not cloning. |
| `Collect` | Mutable `from-list` has last-wins semantics; persistent left-folded inserts would too | Pair-sequence consumer into a new map. Existing persistent `from-pairs` has the **opposite** duplicate rule and must retain its current API behavior. |
| `Copy` | `map-union source destination` / `hash-map-union source destination` return source-preferred persistent results; mutable single inserts exist | Bulk copy into an existing mutable destination, cross-home source adapters, safe self-copy and documented owner replacement. Reversing union arguments reverses overwrite priority. |
| `DeleteFunc` | Persistent filter with the negated pure predicate returns a new map | Mutable predicate/delete traversal including effectful callbacks and callback mutation; live semantics cannot be claimed from a stale snapshot. |
| `Equal` | Size, lookup and traversal exist in all homes | Size-first binding equality with value equality, independent of traversal order; empty adaptation; key-policy compatibility; no pointer-identity shortcut for non-reflexive values. |
| `EqualFunc` | Same building blocks; values may have unrelated types | Explicit value predicate across different value types, early decision, optional effectful form. The callback changes **value** comparison only. |
| `Insert` | Single insert exists everywhere; pure Producer fold can return a persistent result | Consume a sequence into an existing mutable map, with last-wins order and explicit producer effects. A constructor is not an update to the supplied destination. |
| `Keys` | Eager key lists in every home | Lazy/reusable current-map sequence and immediate stop; eager list availability is only partial coverage. |
| `Values` | Eager value lists in every home | Same sequence work; preserve duplicate values and managed owners. Separately obtained key/value views need not zip into the original entries. |

`Data.Producer` supplies a pure pull state machine, `Done`/`Skip`/`Yield`,
mapping, taking and folding. It can support persistent pair streams and
pure input sequences consumed from a mutable worker. Its step function
cannot itself execute mutation or IO, and its existing fold callback is
pure. It is not by itself a replacement for an arbitrary effectful
push-style pair producer or a mutable live-map iterator. Source/consumer
effects and early-stop discipline require a deliberate typed API.

Equality must compare corresponding bindings using a single chosen key
identity, rather than compare eager list order. Two different hash
histories can produce different orders. Two ordered homes can order the
same logical keys differently. Different key equivalences are a
precondition issue, not something a value comparison callback repairs.
Existing strict folds can avoid later value-predicate calls after a
mismatch, but still traverse later storage; true early traversal stop
needs a different worker.

## Empty values, aliases and callback mutation

The reference distinguishes a nil map from an allocated empty map in
Clone and attempted assignment; equality and traversal identify them as
empty. Nil Copy/Insert destinations accept an empty source but reject an
actual assignment. Collect always returns an allocated map, even for an
empty sequence. DeleteFunc makes no calls for either empty form.

Cinch's base map values are always valid values: `Tip`, empty HashMap, or
a fresh mutable handle. There is no null handle to dereference. The base
API should use these empty values. If absent-versus-present-empty is
observable at an integration boundary, use an explicit `Option map`
adapter: Clone preserves `None`, equality normalizes `None` as empty,
collect returns `Some empty`, and writes to an absent destination require
an explicit module-owned error/result or an explicitly requested
allocation operation. Do not invent a hidden null sentinel or claim that
normalizing nil to empty preserves all reference observations.

A persistent clone may return the same immutable value with shared
subtrees. A mutable clone needs different container storage; shallow
payload sharing is still required. A payload that is itself a mutable
handle remains shared, and its region/lifetime rules still apply. Tests
must mutate both original and clone containers, then mutate shared
payloads, and drop each owner in either order. No public map cursor or
generational entry handle exists here; IndexMap numeric positions are
current positions and can change after removal.

The reference range contract says an unvisited entry deleted during
iteration is **not produced**; a new entry may or may not be produced.
The iterator captures a live map and starts range when invoked, not when
created. A reusable sequence therefore sees later changes on a later
invocation. The current mutable snapshot callbacks will still call an
action with the original pair after an earlier callback removes that
entry, and won't show a later overwrite of its captured value. This is a
real behavioral difference, not merely a more stable iteration order.

Do not repair that claim by re-looking-up every snapshot key without
further design: it changes representative/value behavior, and cannot
recover non-reflexive keys. Also do not keep raw slot arrays/indices across
a callback: a callback may clear, delete, insert, grow, shift or swap the
container. Any eventual live traversal needs a proved lifetime and
entry-identity plan. A separate explicitly named snapshot traversal is
useful, but does not complete the live-mutation capability.

## Floating keys and values remain explicit

The reference package explicitly makes no special provision for
non-reflexive keys. With IEEE equality, a NaN key can be stored and ranged
over but cannot be found or deleted by looking up that key. Repeated NaN
insertions can add multiple entries. Equal on a nonempty map containing
a NaN key is false even against itself; EqualFunc cannot fix the missing
key by changing its value predicate. A NaN *value* also makes ordinary
Equal false, but EqualFunc may choose its own value-NaN policy. DeleteFunc
with a predicate accepting a NaN-key entry still cannot remove it by
ordinary key deletion. Clone must preserve such entries, not canonicalize
or lose them.

At this basis raw `F32` and `F64` have no global `Eq`, `Ord` or `Hash`
instances. Explicit IEEE `feq` and quiet comparisons exist. `Data.Compare`
has a separate NaN-first comparison that equates all NaNs and both signed
zeros. `Data.Machine.total-order-f32/f64` is another explicit policy.
Neither provides map key equality/hash capabilities automatically.
Supplying only `flt` as an ordered map's comparator is invalid: a NaN
would compare neither less nor greater than every number, collapsing
distinct keys. Ordered trees require a sound total-order equivalence.

Two possible future key policies must remain distinct:

1. A reflexive wrapper with a stated NaN and zero equivalence and a hash
   consistent with that equivalence. This is useful but does not reproduce
   non-reflexive-key behavior. Bit-pattern identity is another useful
   wrapper; it distinguishes signed zeros and NaN payloads.
2. An explicitly IEEE-equal hashed wrapper, with zero-normalized hashing
   and documented non-reflexive keys. Current generic hash workers may be
   usable, but the required insertion/deletion/clone/iteration behaviors
   must be executed and observed before claiming support. An equality
   dictionary name is not evidence that these cases work.

Future fixtures need multiple quiet/signaling NaN bit patterns and signs,
repeat insertion of the *same* NaN bits, positive/negative zero, finite
values and infinities. Observe key multiplicity/representation, lookup,
overwrite, deletion, clone, Equal and EqualFunc separately. An IEEE-equal
hash must give both zeros the same digest; bitwise hashing alone violates
that requirement. Value-only float tests are insufficient.

## Implementation and validation sequence proposed

First preserve current APIs and implement narrowly named last-wins
sequence insertion/collection, binding equality with a value predicate,
shallow mutable cloning and source-preferred copy in the appropriate
homes. Persistent helpers return a new map; mutable helpers mutate the
supplied one. Existing `from-pairs` duplicate semantics stay unchanged.
Cross-home adapters should traverse a pair capability rather than clone
each algorithm five times. This is a proposal, not a checked Cinch
interface: no new generic class, effect row or wrapper is admitted here.

Resolve live iteration and non-reflexive key capabilities before counting
All/Keys/Values/DeleteFunc complete. Retain explicit snapshot operations
where they are useful. Do not silently narrow callbacks to pure functions
or discard the float cases to finish the package sooner. Nil adaptation
should be separately documented at optional-map boundaries.

The JSON has per-record fixture requirements. The eventual independent
SDK generator should compare order-independent pair/key/value multisets,
preserve duplicate values/NaN-key multiplicity, check last-wins sequences,
and record callback traces only where order is actually specified.
Mutation tests should delete *all* other entries from the first callback
to make the reference's unvisited-deletion observation deterministic.
Insertion-during-iteration assertions must allow both permitted outcomes.
Tests also need source/destination aliasing, shallow shared payloads,
cross-home key policy agreement, empty no-callback behavior and precise
first-failure diagnostics. Both frozen frontends need full source and
source-absent interface consumers, 32 MiB/8 MiB normal and separate
256 MiB/8 MiB poison runs with zero leaks. Their exact epochs must be
recorded; no such new gate is claimed by this static audit.

The existing `lib_map_set_ops`, `lib_hashmap_real`, `lib_hash_keys` and
`lib_mutable` sources contain relevant persistent semantics, hash laws,
collision/growth/churn, BTree model and IndexMap model checks. They were
inspected, not executed or treated as tests of the ten new helpers.

HashTable and IndexMap already match eight control tags per U64 using
SWAR. Future scan/clone/filter kernels should retain that approach and
inspect actual LLVM/assembly for target-width operations and redundant
boxing/ARC traffic before considering a wider inline LLVM kernel.
Generic key equality, hashing and mutation callbacks cannot be reordered
or vectorized without an additional sound specialization contract.
Ordered pointer/tree dependencies and owner-aware copies need scalar
control; bulk raw control-word processing is a plausible SIMD target.
No new acceleration or speed claim is made here. Benchmarks must measure
public collect/copy/equality/traversal pipelines with independent output
checks and owner accounting, including sorted ordered-map input,
collision-heavy hashing, managed values and early termination.
