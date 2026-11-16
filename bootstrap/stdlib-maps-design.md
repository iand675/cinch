# Live mutable map façade design

This follows `stdlib-maps-audit.md` and published audit `9a58ecfc`. The
first implementation will add `Data.Mutable.Map`; it will not change
`Data.Map`, `Data.HashMap`, `Data.Mutable.HashTable`, `Data.Mutable.BTree`
or `Data.Mutable.IndexMap`. Their existing construction, representation,
order and snapshot contracts stay intact. This design is not a completed
runtime gate, an adopted replacement or a performance result.

## Representation and policy

`MutableMap r k v` owns a captured `MapKeyPolicy k`, a
`HashTable r Nat (List (ListElement r (k, v)))` digest index, and a
`LinkedList r (k, v)` entry store. A policy holds pure `k -> Nat` hashing
and `k -> k -> Bool` key comparison. The convenience constructor captures
the current `Hash` and `Eq` methods once; an explicit constructor accepts
ordinary functions, including IEEE float comparison without global
instances. All later operations use the captured policy. Cloning shares
that immutable policy value and creates fresh index/list storage.

The outer index uses the existing fixed `Hash Nat`/`Eq Nat` capabilities.
Each digest's immutable bucket list contains stable entry handles. A hash
collision searches that bucket using the captured key comparison. This
allows explicit hash/equality functions without dynamic dictionary
construction or edits to HashTable's representation. It also adds a
second level of indexing, owner traffic and collision-list storage; those
costs must be measured rather than hidden by the existing SWAR kernel.

An insertion searches the current bucket. A matching entry keeps its
stable identity and receives the new key representative and value. A new
entry is appended to the private linked list before its handle enters the
index. Deletion finds a matching key, removes that handle from its digest
bucket, and removes the checked list membership. No client can construct
or mutate the private list or an entry handle. Empty digest buckets are
removed from the index. Clear clears both stores; the list's generation
token invalidates all earlier memberships.

The graph has no ownership cycle: roots own the index and list; buckets
own handles; handles own a node and membership/owner tokens but never the
list root. Removed nodes retained by an active traversal are released
when that traversal's snapshot drops. A clone shares user payloads
shallowly, including any user-owned mutable handles, while all container
membership and value cells are independent.

Equal-key hashing must agree, and hashing/comparison must remain stable
while a key is stored. Non-reflexive equality is deliberately permitted.
Cross-map binding equality requires compatible key equivalence; hash
seeds/algorithms need not be identical because each map uses its own
captured hash. The library cannot decide equivalence of arbitrary
functions and will not pretend that closure-pointer identity proves it.
Distinct policies should use distinct key wrappers when that makes the
precondition clearer. Copy always inserts under the destination policy;
coarser destination equivalence may merge source entries in the source's
unspecified enumeration order.

## Live traversal without raw-slot borrowing

At *invocation*, All/Keys/Values capture the current index's immutable
bucket lists of stable handles. Creation by partial application captures
only the map, so invoking the same traversal again sees a fresh entry
set. The implementation checks each handle's **current list membership**
before reading its **current key/value cell** and calling the action.

Consequences:

- Deleting an unvisited entry skips it. Clearing skips all old entries.
- Overwriting an unvisited entry yields the replacement key/value.
- Deletion/reinsertion creates a new identity; an old handle cannot name
  the replacement even if its physical slot is reused.
- Entries inserted after invocation are excluded. The reference permits
  new entries to be skipped, so no obligation requires chasing new nodes.
- NaN entries remain enumerable: no key lookup is used to validate an
  entry identity.
- The first action returning False ends user callbacks immediately.
  Nested traversals each have their own identity snapshot.
- No raw arena, slot index, control-word borrow or value snapshot crosses
  a user callback. Membership/value are reacquired after each callback.

Setup scans the digest index and allocates a list of bucket-list owners:
O(index capacity + distinct digests), even when the first action stops.
It retains entry handles and hence deleted payloads until the traversal
returns. This is a material early-stop/allocation tradeoff. It is safer
than retaining raw mutable storage across callbacks, but is not a claim
of zero-cost iteration. Later optimization requires allocation and full
pipeline evidence, especially against existing snapshot iteration.

## Planned public signatures

These are exact proposed source signatures; body checking and imported
interface checking remain gates. All declarations and helpers will have
canonical `-- |` comments independent of the reference SDK.

```haskell
map-key-policy :: forall k. (k -> Nat) -> (k -> k -> Bool) -> MapKeyPolicy k

mutable-map-new :: forall r k v. using (h :: Hash k, eq :: Eq k)
  => Unit ->{Mut r} MutableMap r k v
mutable-map-new-with :: forall r k v. MapKeyPolicy k
  -> Unit ->{Mut r} MutableMap r k v
mutable-map-size :: forall r k v. MutableMap r k v ->{Mut r} Nat
mutable-map-is-empty :: forall r k v. MutableMap r k v ->{Mut r} Bool
mutable-map-lookup :: forall r k v. MutableMap r k v -> k ->{Mut r} Option v
mutable-map-set :: forall r k v. MutableMap r k v -> k -> v
  ->{Mut r} Result MapError Unit
mutable-map-delete :: forall r k v. MutableMap r k v -> k
  ->{Mut r} Result MapError (Option v)
mutable-map-clear :: forall r k v. MutableMap r k v ->{Mut r} Unit

mutable-map-all :: forall r k v ef. MutableMap r k v
  -> (k -> v ->{ef} Bool) ->{Mut r | ef} Bool
mutable-map-keys :: forall r k v ef. MutableMap r k v
  -> (k ->{ef} Bool) ->{Mut r | ef} Bool
mutable-map-values :: forall r k v ef. MutableMap r k v
  -> (v ->{ef} Bool) ->{Mut r | ef} Bool

mutable-map-insert :: forall r k v ef. MutableMap r k v
  -> ((k -> v ->{Mut r | ef} Bool) ->{Mut r | ef} Unit)
  ->{Mut r | ef} Result MapError Unit
mutable-map-collect :: forall r k v ef. using (h :: Hash k, eq :: Eq k)
  => ((k -> v ->{Mut r | ef} Bool) ->{Mut r | ef} Unit)
  ->{Mut r | ef} Result MapError (MutableMap r k v)
mutable-map-collect-with :: forall r k v ef. MapKeyPolicy k
  -> ((k -> v ->{Mut r | ef} Bool) ->{Mut r | ef} Unit)
  ->{Mut r | ef} Result MapError (MutableMap r k v)
mutable-map-clone :: forall r k v. MutableMap r k v
  ->{Mut r} Result MapError (MutableMap r k v)
mutable-map-copy :: forall r s k v. MutableMap r k v -> MutableMap s k v
  ->{Mut r, Mut s} Result MapError Unit
mutable-map-delete-if :: forall r k v ef. MutableMap r k v
  -> (k -> v ->{ef} Bool) ->{Mut r | ef} Result MapError Unit
mutable-map-equal :: forall r s k v. using (values :: Eq v)
  => MutableMap r k v -> MutableMap s k v ->{Mut r, Mut s} Bool
mutable-map-equal-by :: forall r s k a b ef.
  (a -> b ->{ef} Bool) -> MutableMap r k a -> MutableMap s k b
  ->{Mut r, Mut s | ef} Bool

map-source-from-list :: forall k v ef. List (k, v)
  -> (k -> v ->{ef} Bool) ->{ef} Unit
mutable-map-source :: forall r k v ef. MutableMap r k v
  -> (k -> v ->{Mut r | ef} Bool) ->{Mut r | ef} Unit
```

All/Keys/Values return True on exhaustion and False on action stop. They
are reusable curried callback traversals, not a new effect-indexed
existential sequence datatype. Source producers return Unit and obey the
yield callback's stop result. The source helpers give direct adapters for
lists and other live façade maps. Separate explicitly named snapshot
source adapters can later expose existing homes without claiming they
have stable live identity. Row-polymorphic callbacks can perform IO or
mutate other regions; a Mut-only callback restriction is not acceptable.
Higher-order row/source use and storing/reusing a partial application
need positive and negative frontend gates before publication.

The pair-source form explicitly includes its own `Mut r` label in the
yield callback's row as well as its result row. This permits two partially
applied map sources/consumers in different regions to unify their open
tails without the self-tail equation exposed by the first frozen check.
The direct All/Keys/Values forms retain their more general callback row;
the source adaptation does not remove IO or other-region effects.

`MapError` distinguishes capacity refusal, an internal identity invariant
failure and an absent optional destination. A list membership error at an
internal deletion is reported as an invariant failure, never quietly
converted to successful deletion. A new binding checks a conservative
target-derived entry bound before growing either store; overwrites at
the bound remain allowed. The bound must account for the index's largest
power-of-two chunk count (eight slots and seven live entries per chunk),
the linked-list physical bound, and allocation headers. Invalid huge
requests cannot be passed unbounded to existing HashTable constructors.
No user capacity constructor is necessary for the ten-operation surface.

Insert/Collect consume pairs in yield order and therefore keep the last
equivalent key/value. On a checked insertion refusal, Insert stops the
producer and returns the error after preserving its already-applied
prefix. A defensive callback wrapper makes any further producer yields
inert after refusal. Before a normally returning source's result is
exposed, its private insertion session is closed; a retained callback
then returns False without mutating the destination. This does not claim
finalization across arbitrary non-returning user effects. Collect releases its unpublished partial map on
error. Arbitrary producers must obey the stop protocol and cannot expect
transactional rollback of their own side effects.

Copy takes destination first and source second; source values overwrite.
Its fixed identity snapshot makes self-copy finite, including NaN keys:
each original NaN entry can add one new unfindable entry, but none of the
new entries is revisited. Clone creates a fresh map with the source
policy, copying each entry once without a source key lookup. Equal checks
sizes first, then right-side key lookup and value comparison; it has no
map identity shortcut. EqualBy permits unrelated value types and stops
value callbacks on the first decisive mismatch.

DeleteIf uses live identity traversal, invokes the predicate, then calls
ordinary key deletion if True. If the predicate deletes/reinserts the
current reflexive key, this later deletion removes the replacement; it
does not merely unlink the old captured identity. A NaN key remains
unfindable and therefore remains present even when its predicate is True.

## Nil and floating policy adapters

The façade has a valid allocated empty map, never a hidden null handle.
An accompanying optional adapter uses `Option (MutableMap r k v)`:
Clone preserves None, equality identifies None and empty, traversal of
None performs no calls, and assignment to None returns
`MapMissingDestination` on the first yielded binding. An empty input is
successful. Collect returns an allocated map. These adapters preserve
observable absence without weakening the core map type.

Explicit F32/F64 IEEE policies use `feq-f32/f64` and a hash that normalizes
both zeros while allowing NaN entries to remain non-reflexive. Private
inline LLVM bitcasts can inspect full float bits without a numeric cast;
any positive-natural digest folding needs independent unsigned boundary
fixtures and must not depend on the known older literal-fold defect.
No global floating Eq/Ord/Hash instance is added. A separately named
reflexive NaN-grouping or bit-pattern policy is a different semantic
choice, not a substitute for these tests.

## Gates and performance review

Before compiled work, preserve the design and source checkpoints in this
isolated lane. Root owns Cargo; a bounded frozen CLI source/control window
will be requested only after static source is ready. First use b004/c4
and a coherent admitted Native pair supporting LinkedList and HashTable;
Ring's recursive Array epoch is unrelated. No full chain is needed here.

Required independent observations include all ten operations; last-wins
sequences; clone/container isolation and shared payload mutation; retained
owners after source release; source/destination self-aliasing; all
current/future entry deletion, clear, overwrite, reinsertion, slot reuse,
growth and nested traversal cases; early callback stop; cross-region/IO
effects and illegal pure use; nil/empty adapters; NaN key multiplicity,
lookup/deletion/clone/self-copy/equality, value NaNs, zero key
representatives and infinities. All normal 32 MiB/8 MiB and separate
poisoned 256 MiB/8 MiB runs must finish with zero leaks, and source-absent
interfaces must retain the exact captured-policy/effect surface.

Benchmark public collect, copy, clone, equality and partial/full traversal
with distinct digests, deliberate full collisions, managed values,
delete-heavy churn and callback mutation. Retain independent output and
owner checksums. Compare against existing homes and the SDK in explicitly
granted timing windows. Inspect generated LLVM/assembly and dynamic
allocation/ARC counts for the two-level index, stable-node value cells
and identity snapshot. Reuse HashTable's eight-byte SWAR control matcher;
do not vectorize arbitrary user callbacks. Wider inline LLVM work is
justified only by an actual raw-control kernel bottleneck. Existing
ordered-map balance and Ring's selected/candidate performance work remain
separate tasks.
