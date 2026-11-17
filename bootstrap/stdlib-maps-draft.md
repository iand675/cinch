# Unqualified map façade draft

This checkpoint contains implementation and fixture work in progress.
It is **not published library coverage** and must not be integrated as a
completed package. Its partial source/control/IEEE qualification is
recorded below. Published independent
audit/reference checkpoints remain separate from this local source draft.

The new modules are `Data.Mutable.Map`, `.Optional` and `.Float`. The five
existing map homes are unchanged. Public operations follow
`stdlib-maps-design.md`; the core owns a captured key policy, a SWAR digest
index, and stable LinkedList entry memberships. Identity snapshots are
checked for current membership before reading current values. Callback
growth, clear, deletion, overwrite and reinsertion do not carry raw slots
across callbacks. The index and snapshot allocation costs remain explicit
tradeoffs requiring later measurements.

Copy currently uses a direct identity-bucket worker. This keeps the same
source-preferred, finite self-copy behavior without a temporary insertion
status cell. Optional Copy directly dispatches present maps and checks
empty sources for absent destinations. The generic source composition
API remains present. Its separate cross-region composition source gate
now passes repaired Rust fa3; these specialized workers alone did not
establish that gate.

## Preserved source evidence and limits

- Frozen Rust b004/c4 checks the initial core through Copy/Clone/DeleteIf,
  then rejects EqualBy's second region read. The minimal independent
  `Var r Nat`/`Var s Nat` and HashTable-size probes reproduce the same
  rejection. Parent confirmed the Var failure on current Rust37dc and
  owns the effect-accumulation repair. No cross-region API was narrowed.
- Adding explicit `Region` annotations was tried diagnostically and
  rejected as an unknown type by the older Rust surface; those
  annotations were reverted. Normal inferred-region syntax is retained.
- Full Native2f077/80bd passes both minimal distinct-region probes.
  Its initial core Copy source/consumer partial application reports
  `expected r, found s`. That older-frontend higher-order composition
  limitation is preserved rather than declared repaired.
- The direct core Copy worker allows Native2f to check the whole core,
  including heterogeneous EqualBy. Optional Copy's analogous direct
  implementation first exposed my reserved identifier `target`; it is
  renamed to `destination-map`. A later exact source-selected check now
  accepts the complete current core/Optional/Float closure, including the
  session-closing guard. No linked Native executable or runtime verdict
  follows from this source check. Its exact inputs are recorded separately
  in `stdlib-maps-native-source-results.json`.

Exact inputs and diagnostics remain under `build/maps-source-probe`,
`build/maps-region-probes` and `build/maps-native-source-probe`. The
maintained minimal region fixtures are `maps-region-pair.cinch` and
`maps-region-source.cinch`.

## Repaired source gate and genuine interface refusal

The exact repaired Rust compiler is
`fa3ea3c9be1b74be455cb18ce0f084ef31aafb82e39ccc8fd660f653719531d2`,
source checkpoint `5c664cfd`, with adjacent c4 runtime. It checks the
entire current core/Optional/Float source and both independent distinct-
region read/source-composition fixtures. No API was narrowed to obtain
these source verdicts. Reports are under `build/maps-region-safe`.

A real source-absent consumer uses byte-for-byte copies of the actual
fa3-produced interfaces, with hashes from its generated manifest and no
library sources beside the consumer. Both Native2f and experimental4a
reject a call to imported `mutable-map-size`, exit4/compiler leak0:

```text
imported nominal scheme needs defining-module type identities: Data.Mutable.Map.MutableMap
type constructor is applied to too many arguments
kind mismatch in type inference
```

`build/maps-source-absent-fa3/results.json` preserves the exact artifacts,
consumers and verdicts. Even closed mutable-map-size/copy/new exports have
`resolved:null`. At frozen source5c, the Rust writer's
`resolved_kernel_effect_row` excludes Mut and refuses open row tails;
this is a concrete cause of the missing resolved declarations. The
MutableMap/Var/LinkedList phantom headers also still say Type. Those
header semantics and open-row transport are separate gates; no metadata
was restamped and no reader check was weakened. Cross-producer interface
compatibility is not established by source checking.

## Prepared independent gates

`go_maps_vectors.go` ran on the pinned SDK1.27.4 and generated eight
100-operation traces spanning all ten actual package operations. Every
step carries its full independent post-state; observations do not pin an
unspecified iteration order. It also generated fourteen binary32/64
bit-exact NaN/zero/clone/copy/delete rows and seven nil/empty observations.
The published generation receipt explicitly does not qualify the library.

The trace and float templates render these independent values. All eight
general traces now pass: 800 complete independent post-state/operation
observations across sixteen normal/poison processes on fa3/c4. The separate control has twelve first-failure
groups: binding utilities, views/early stop, clear during traversal,
NaN clone/self-copy, optional absence, growth/overwrite, delete/reinsert
with immediate slot reuse, nested traversal, explicit colliding policy,
owned Bytes after source clear, shallow shared mutable values, and a source
retaining its yield callback after completion. The insertion/absent-source
session is closed before returning; late yields return False without
mutating the former destination. The final control emits/links O2 and
passes normal32M/8M and poison256M/8M on fa3/c4, both zero leaks. Its earlier
import/default-literal/scoped-region annotation fixture failures are
preserved; typed ordinary helpers tie absence and callback state to the
actual map's region.

Both independent binary32 and binary64 consumers also emit/link O2 and
pass normal/poison profiles, four zero-leak processes. All fourteen SDK
observations match complete signaling/quiet NaN/sign/zero key bits,
multiplicity and clone/copy/delete/self-equality behavior. These are real
runtime results. All800 general steps and the actual IO+Mut source, values
and equality-callback control also pass on fa3/c4. The complete Rust matrix
contains twelve consumers and24 normal/poison processes, all zero leaks.
Native map runtime gates remain separate. Reports are under
`build/maps-region-safe`; their exact hashes are recorded in
`stdlib-maps-runtime-results.json`.
Further value-NaN/callback mutation/error-prefix controls, source-absent
interfaces, capacity models and public benchmark gates remain required.

`validate_maps.py` freezes explicit sources and frontend/runtime/provider
identities. It separates source-only checks from actual LLVM/link/runtime
gates; normal32M/8M and separate poison256M/8M must both report zero leaks.
Native linking explicitly selects O0 or O2 and its exact matched provider
object/runtime; Rust whole-source linking uses only its explicit archive.
Source-only, control, IEEE, all general traces and actual IO callback modes
passed on fa3/c4. Native full source selection also passes admitted2f/V2/80bd; its
emission/link/runtime modes remain unqualified.

Ring selected9c and unselected storage candidate0dd remain immutable.
Work switches back to bounded focused Ring qualification when the exact
repaired experimental Native tuple is available; full-family adoption and
matched performance review remain separate gates.

## Provisional closed-Mut interface transport

The distinct provisional Rust writer9dc (`9dcdf0f2ed2aac1248d5ae3439ab7d4e556a2d1d8456a33297ae03b5e4efd2ee`,
source354eef07, adjacent c4 runtime) checks the unchanged full map sources
and now emits a real resolved closed-Mut scheme. Genuine source-absent
interfaces were copied byte for byte from its own generated manifest.
Experimental Native4a admits the pure policy identity consumer. It refuses
closed map-size with a kind mismatch: the function binds r as Region while
the MutableMap nominal header still declares its parameter Type. The open
row map-all export still has resolved:null and is separately refused.
These are remaining semantic/header-role and open-row features; no header,
manifest or metadata bytes were restamped. The provisional writer is not
claimed fully qualified. Exact reports are under `build/maps-provisional-9dc`.

## Source-selected Native consumer and public workload preflight

The first full Native control consumer on admitted2f/V2/80bd refuses
effect-row inclusion while composing the collection/insertion producers
in binding-case, before linking or runtime. Exact source and diagnostic
are retained in `build/maps-source-selected-2f/control-results.json`. The
full source-only closure verdict does not imply this application gate
passed. The public API and its effect rows remain unchanged.

Both public-workload Rust variants now build with ordinary generated RC
and pass60 independent normal/poison fingerprint runs across three
workloads and five sizes. `stdlib-maps-benchmark-results.json` records
exact artifacts, allocation counters and subsequent paired baseline timing. The
construction pipeline at4096 entries/eight iterations allocates754388
blocks/22694906 cumulative bytes/1000943 peak bytes in the façade versus
131597/6305868/246667 in the public HashTable baseline. These process-wide
counters include startup, preparation and teardown. Stable entry owners,
the extra digest index and snapshot behavior are material costs requiring
review, rather than an assumed acceleration claim.

The parent-granted baseline timing has180 warmup/sample processes with
independent fingerprints, zero leaks, allocation counters disabled and no
new shared-cgroup throttling. The4096-entry/eight-iteration construction
pipeline median is30.675ms for the façade versus6.938ms for HashTable.
The separate typed-Var entry candidate remains unselected and unqualified;
its exact source/patch hashes and invariants are documented independently.

Completed fa3 functional ELF bytes were archived losslessly to recover
50378123 bytes during filesystem pressure. Their original/report/archive/
decompressed SHA and byte counts are retained in
`stdlib-maps-functional-archive-results.json` and the immutable build index.
The source, LLVM and benchmark/frozen/provider/runtime inputs are unchanged.
`restore_map_functional_elf.py --original <recorded-relative-path>` restores
one verified owner-executable ELF on demand. The new supplemental value-NaN
and predicate-mutation edge consumer has not yet run; the established24
functional process verdicts remain tied to their original exact fixtures.
