# Remaining mutable-container nominal Region proposal

This is a declaration-only static proposal, based on Root's genuine aggregate-interface advisory. It changes no defining source or historical interface and invokes no compiler. The advisory input is `bootstrap/stdlib-nominal-region-migration-audit.json` in the Root lane, SHA256 `b4909fcb7e2822dbb77ebf246f720577bac25e4c66c13815d2f0906d30a6e62e`. The scoped subset contains17 nominal parameters across16 defining modules; each recorded public `Type` parameter is applied to an explicitly Region-quantified variable in an actual complete exported scheme.

`stdlib-mutable-region-migration-results.json` records each exact defining source SHA256, actual interface SHA256, a complete defining-module scheme witness, an available facade witness, the actual historical public header, and the proposed source hash. The preparation script recomputes the interface's FNV source identity from the actual defining source bytes; all16 match the genuine audit input. The witness schemes are checked against the same unmodified interfaces. An advisory variable spelling alone never authorizes a change.

| Defining module | Public declaration(s) proposed | Mutable dependency that must agree |
| --- | --- | --- |
| `Data.Mutable.Array` | `MutArray (r :: Region) a` | None |
| `Data.Mutable.BTree` | `BTreeMap (r :: Region) k v` | None |
| `Data.Mutable.BTreeSet` | `BTreeSet (r :: Region) a` | `BTreeMap` |
| `Data.Mutable.BitSet` | `BitSet (r :: Region)` | None |
| `Data.Mutable.Deque` | `Deque (r :: Region) a` | None |
| `Data.Mutable.HashTable` | `HashTable (r :: Region) k v` | None |
| `Data.Mutable.HashSet` | `HashSet (r :: Region) a` | `HashTable` |
| `Data.Mutable.Heap` | `Heap (r :: Region) a` | None |
| `Data.Mutable.HeapView` | `HeapView (r :: Region) a` | Scoped application callbacks |
| `Data.Mutable.IndexMap` | `IndexMap (r :: Region) k v` | None |
| `Data.Mutable.LinkedList` | `LinkedList (r :: Region) a`, `ListElement (r :: Region) a` | Both declarations together |
| `Data.Mutable.LruCache` | `LruCache (r :: Region) k v` | `HashTable` |
| `Data.Mutable.Ring` | `Ring (r :: Region) a` | None |
| `Data.Mutable.UnionFind` | `UnionFind (r :: Region)` | None |
| `Data.Mutable.Var` | `Var (r :: Region) a` | None |
| `Data.Mutable.Vec` | `Vec (r :: Region) a` | None |

The actual public headers export these type identities with hidden constructors. `HeapView` carries the role through the mutation rows of its private callback fields, rather than through raw storage alone. The proposed role does not add a runtime field. Key/value/element parameters remain ordinary `Type`; constructor field order, bodies, exports, rows and function signatures are byte-identical. Exact inverse declaration substitutions recover every original source byte.

The seven private nominal types remain unchanged: `BTree.Pool`, `HashTable.Table`, `IndexMap.Dense`, `IndexMap.Index`, `LinkedList.ListNode`, `Ring.RingNode` and `Ring.RingArena`. Their parameters describe payloads or metadata, with no regional parameter to annotate. Public `LinkedListError`/`RingError` and their exported error constructors also remain unchanged. The private C839 `MapEntry (r :: Region)` is an additional local experiment already covered in the separate map proposal; it is not invented as a public header in this aggregate audit. Root's published modules do not yet include that map facade.

`Data.Mutable` reexports the defining identities, so no replacement datatype is added there. HashTable/Var already have independently checked annotated copies in the qualified330/d91 C839 experiment, but the current Root defining sources are still implicit and this proposal does not promote those copies. Wrapper modules must be rechecked with coherent newly generated defining headers; combining a migrated wrapper with an authoritative historical `Type` dependency is not a valid shortcut.

The isolated copies occupy238,291 bytes under `build/mutable-region-migration-static`. The exact17-line annotation proposal is `tooling/mutable-container-nominal-region-proposal.patch`; `python3 tooling/prepare_mutable_region_migration.py` reconstructs it and checks source/witness linkage without invoking a compiler. Frozen providers, runtime inputs, historical source snapshots and interfaces remain plain and unchanged.

Qualification remains required before migration: source-check each dependency closure, inspect actual new defining headers and pure selector binders, regenerate genuine source-absent interfaces, and test aliases/reexports plus explicit Type, concrete-type, shadowed-head and foreign-identity refusals. Preserve old genuine Type headers as authority tests rather than rewriting their kinds. Recheck existing runtime/owner/callback matrices on an exact matched tuple, including live list/ring aliases, wrapper stores and effectful HeapView callbacks. Preserve regular-recursion, function-variance and Array authority gates for Ring. The advisory and this patch establish no runtime, Native-family, performance or production-adoption claim.
