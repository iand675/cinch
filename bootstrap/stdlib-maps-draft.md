# Unqualified map façade draft

This checkpoint contains implementation and fixture work in progress.
It is **not published library coverage**, has no runtime qualification,
and must not be integrated as a completed package. Published independent
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
  renamed to `destination-map`. The final complete Optional/Float check
  has not yet run. No linked executable or runtime verdict is claimed.
  The later session-closing guard and its retained-callback control also
  remain unqualified; the earlier source verdict is tied to its retained
  input hashes, not to subsequent edits.

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

The prepared trace and float templates render these independent values;
they were generated and Python syntax-checked, but no Cinch consumer was
linked or executed. The first control build found an invalid selective
import of builtin type Bytes from its operations module; that fixture
import is corrected, with retry pending. The separate control has twelve first-failure
groups: binding utilities, views/early stop, clear during traversal,
NaN clone/self-copy, optional absence, growth/overwrite, delete/reinsert
with immediate slot reuse, nested traversal, explicit colliding policy,
owned Bytes after source clear, shallow shared mutable values, and a source
retaining its yield callback after completion. The insertion/absent-source
session is closed before returning; late yields return False without
mutating the former destination.
Further value-NaN/callback mutation/error-prefix controls, source-absent
interfaces, capacity models and public benchmark gates remain required.

`validate_maps.py` freezes explicit sources and frontend/runtime/provider
identities. It separates source-only checks from actual LLVM/link/runtime
gates; normal32M/8M and separate poison256M/8M must both report zero leaks.
Native linking explicitly selects O0 or O2 and its exact matched provider
object/runtime; Rust whole-source linking uses only its explicit archive.
Source-only modes passed; emission/link/runtime modes remain unqualified.

Ring selected9c and unselected storage candidate0dd remain immutable.
Work switches back to bounded focused Ring qualification when the exact
repaired experimental Native tuple is available; full-family adoption and
matched performance review remain separate gates.
