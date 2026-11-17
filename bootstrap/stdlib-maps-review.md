# Mutable map source checkpoint for review

This is a partial source-distributed library checkpoint, not completed
package coverage. The proposed façade implements the ten audited operation
capabilities in `Data.Mutable.Map`, `.Optional` and `.Float`; the five
existing map homes are unchanged. The selected baseline core source is
`0fc095f0bdcbe26f9ba3f26e5db8cb76891698784b744bb76e2e420dc14eb7c6`.
Its captured key policy, stable entry snapshots and mutation contracts are
described in `stdlib-maps-design.md`. No public effect row was reduced and
no generated interface metadata was restamped.

| Gate | Exact epoch | Actual verdict |
| --- | --- | --- |
| Complete source and independent-region composition | Rust fa3/c4 | Passed current core/Optional/Float source plus distinct-region reads and higher-order source composition. |
| Independent runtime and ownership matrix | Rust fa3/c4, O2 | All 800 independent steps, 12 control groups, 14 IEEE key observations and actual IO callbacks passed: 24 normal/poison processes, zero leaks. |
| Source-selected Native declarations/type/proof check | Native 2f/1d22/22ab/80bd | Passed complete current core/Optional/Float source. This does not establish consumer emission. |
| Source-selected full Native control | Same admitted Native tuple | Refused before linking in binding-case, which composes generic collection/insertion producers: effect inclusion needs resolved rows. No map runtime verdict. |
| Genuine source-absent import | Provisional Rust 9dc writer, experimental Native 4a checker | Pure policy identity admitted; closed map-size refused at genuine Region-scheme/Type-header contradiction; open map-all still has resolved:null. No admission claim or metadata rewrite. |
| Public workload preflight and baseline measurement | Rust fa3/c4, O2 ordinary generated RC | All 60 normal/poison preflight processes and 180 paired warmup/sample processes matched independent fingerprints and zero leaks. Allocation counters were disabled during timing. |

The exact input/report hashes and separate verdicts are retained in
`stdlib-maps-runtime-results.json`, `stdlib-maps-native-source-results.json`,
`stdlib-maps-interface-results.json` and `stdlib-maps-benchmark-results.json`.
Compiler host leak lines in source-only checks are not map consumer runtime
evidence. The explicit tuple identities, rather than this checkout's current
compiler sources, define each result.

## Standalone Native row diagnosis

Two tiny probes use only the actual source of `Data.Mutable.Var`, with no
map implementation. On raw Native 2f and its exact 1d22 interface manifest:

- Generic `consume (supply 7)` refuses with `effect inclusion needs resolved
  rows` at Bundle.cinch:81:15.
- An ordinary client producer explicitly closed to `Mut r` also refuses
  at Bundle.cinch:85:26: `type mismatch: expected ({(Mut ?91)} union ?92), found
  {(Mut ?93)}`.

The actual checks took 1.31s and 1.29s, both compiler leak0, with no linking
or consumer execution. These isolate unresolved row inclusion/union matching
from the map representation and from general interface-header transport.
The initial tooling failure is retained separately: the base manifest did
not contain Var, so its source needed to be included; the raw CLI also
creates an empty requested output on a refused check. Neither probe fixture
was changed to obtain the final observations. Full diagnostics, exact source
and bundle hashes are in `stdlib-maps-native-row-results.json`.

## Measured baseline and unselected candidate

The public HashTable and façade workloads use identical bounded natural
bindings without callback mutation, with independent scalar fingerprints.
HashTable traversal includes its public value snapshot; façade traversal
includes its identity snapshot. Their stronger mutation capabilities and
representations are different, and both real setup costs remain measured.

At 4096 entries/eight iterations, pipeline medians are 30.675ms for the
façade versus 6.938ms for HashTable (4.42 times). Complete traversal is
4.537ms versus 3.924ms; immediate-stop is 2.886ms versus 3.723ms. Each variant
has one warmup and five rotating samples. The shared four-CPU cgroup recorded
no new throttled periods or microseconds; background workloads were not
isolated. Process-wide normal allocation profiles for that pipeline are
754388 blocks/22694906 cumulative bytes/1000943 peak bytes in the façade,
versus131597/6305868/246667 in HashTable. These counters include startup,
preparation and teardown, separately from the measured interval.

This identifies a material construction/allocation cost. The separate
typed-Var Entry candidate removes unused linked-list navigation/arena
machinery while preserving captured policy, live identity snapshots,
unbounded generations/serials and shallow ownership. Its source is
`c839d9b0868aaddfc9914fe515eeb613ba23bd934cc07dc4fa58e44463c00078`;
the baseline source and all measured artifacts remain unchanged. No foreign
declaration, raw-slot ABI or public signature is added or changed. The
candidate is unselected. A separate exact855/c4 epoch now passes its full
source and12 established ownership/live-mutation controls, with both
normal/poison profiles zero leaks. The full matrix and candidate measurement
gates remain pending; no complete qualification or speed gain follows.
Its invariants, exact patch and required matrices are recorded separately in
`stdlib-maps-entry-candidate.md` and its result JSON.

Actual baseline LLVM also shows consecutive single-argument dynamic calls
for captured equality and pair callbacks. A separate compiler review lead
is batching only already-evaluated strict arguments while preserving saved
partial applications, effect order and exact dynamic arity. No compiler
change or gain is claimed here. Existing raw U64 slots are already correctly
specialized, so their words are not being blamed for boxing allocations.

## Remaining gates

Native consumer row inference needs an actual repair or separately recorded
capability-qualified source clients; the existing refusal remains unchanged.
The genuine Region-header/open-row transport work is independent. The new
supplemental value-NaN, callback reinsertion/equality-mutation and pure-read
refusal fixtures are prepared but unexecuted; the maintained Cargo harness
has not run. The Entry candidate must pass source, all 800 observations,
owners, IEEE, mixed IO, callback mutation and negative capability checks
before a separate preflight and paired measurement can select it. Capacity
model/target checks and parent integration checks remain open.

During disk recovery, twelve completed functional ELF byte streams were
archived with original/report/archive/decompressed hashes and a durable
recovery index, reclaiming 50378123 bytes. Source, LLVM and all benchmark,
frozen compiler/provider/runtime inputs were preserved. The archive receipt
and restore helper retain reproducibility; no successful matrix was rerun
to produce this review checkpoint.

## Subsequent boxed-Nat compiler epoch

The exact frozen Rust855/c4 pair accepts both unchanged baseline0fc and
separate C839 full source closures. Their12-control ownership/live-
mutation/policy/late-callback consumers also pass independently in normal
32M/8M and poison256M/8M, four zero-leak processes, with explicit LLVM20.
These reports are in `stdlib-maps-855-source-results.json` and
`stdlib-maps-855-control-results.json`. They do not relabel the earlier
fa3 full matrix or performance measurements. The new baseline SDK-edge
consumer paused at the disk guard before any compilation. Full per-
representation800/IEEE/edge/IO/region/negative and remaining Native gates
are still required before representation selection.
