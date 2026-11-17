# Unselected private entry candidate

The baseline source remains byte-for-byte unchanged at
`0fc095f0bdcbe26f9ba3f26e5db8cb76891698784b744bb76e2e420dc14eb7c6`.
The separate snapshot under `build/maps-entry-candidate/sources` is
reconstructible from `tooling/map-entry-cells-candidate.patch`; its hash
is recorded in `stdlib-maps-entry-candidate-results.json`. It is not a
selected production representation and has not passed source/runtime or
performance gates.

The baseline pipeline's measured allocation and construction costs motivate
this change. The map never uses public linked-list navigation, ordering or
cross-list handle operations. The candidate keeps the existing HashTable
digest index and captured pure key policy, while replacing the unused list
machinery with private entries and the existing typed Var API. It introduces
no foreign declaration or raw-slot ABI and changes no public export or
function signature. The five existing map homes remain unchanged. SIMD/SWAR
continues in the underlying HashTable control-byte matcher; membership and
ownership checks do not provide independent packed lanes.

Each map owns three independent Nat cells: live count, clear generation,
and next entry serial. Each private Entry contains its immutable generation
and serial plus one Var holding Current(active,key,value). Nat counters are
unbounded. Deletion and clear never reset the next serial; no identity can
be reused after removal. Entry has no map-root backlink, so a captured bucket
cannot create a root/entry ownership cycle. User payloads retain their
ordinary shallow ownership semantics.

Snapshots still capture immutable lists of entry identities at invocation.
Before each callback, traversal reads the map's current generation, then
that entry's current active flag and binding. It retains no cached generation
or binding across callbacks. Delete marks the old entry inactive; a replacement
has a new serial and cell. Clear advances the unbounded generation and empties
the index without rewriting captured entries. Consequently old generations,
deleted identities and reinserted replacements are excluded, while an
overwrite changes the current key representative and value of the existing
identity. Nested and reusable traversals each capture their own identities.
Captured deleted payloads remain alive until the invocation releases its
snapshot, as documented by the baseline API.

Copy and clone continue to create fresh destination identities, preserve
non-reflexive key multiplicity and visit only the captured source identities.
Self-copy stays finite. DeleteIf continues to call ordinary current-key
deletion after the predicate; it does not directly unlink a captured entry.
The existing conservative live-entry bound remains, and a broken private
count/active invariant still produces MapIdentityViolation. No effect-row
capability is reduced to accommodate either compiler.

Required gates are source selection, all800 independent steps, all12 live
mutation/owner/policy/late-callback groups,14 IEEE observations, actual mixed
IO callbacks and independent-region source composition, in bounded normal
and separate poisoned profiles. Only after those pass may a new immutable
workload artifact be compared with the preserved baseline. No Native gain
or general source-absent interface admission follows from this candidate.
The actual Region-header/open-row transport gaps and the older Native
producer-composition refusal remain independent compiler gates.
