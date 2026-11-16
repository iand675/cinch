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
API remains present and needs a separate cross-region composition gate;
these specialized workers do not establish that gate.

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

Exact inputs and diagnostics remain under `build/maps-source-probe`,
`build/maps-region-probes` and `build/maps-native-source-probe`. The
maintained minimal region fixture is `maps-region-pair.cinch`.

## Prepared independent gates

`go_maps_vectors.go` ran on the pinned SDK1.27.4 and generated eight
100-operation traces spanning all ten actual package operations. Every
step carries its full independent post-state; observations do not pin an
unspecified iteration order. It also generated fourteen binary32/64
bit-exact NaN/zero/clone/copy/delete rows and seven nil/empty observations.
The published generation receipt explicitly does not qualify the library.

The prepared trace and float templates render these independent values;
they were generated and Python syntax-checked, but no Cinch consumer was
compiled or executed. The separate control has eleven first-failure
groups: binding utilities, views/early stop, clear during traversal,
NaN clone/self-copy, optional absence, growth/overwrite, delete/reinsert
with immediate slot reuse, nested traversal, explicit colliding policy,
owned Bytes after source clear, and shallow shared mutable values.
Further value-NaN/callback mutation/error-prefix controls, source-absent
interfaces, capacity models and public benchmark gates remain required.

`validate_maps.py` freezes explicit sources and frontend/runtime/provider
identities. It separates source-only checks from actual LLVM/link/runtime
gates; normal32M/8M and separate poison256M/8M must both report zero leaks.
Native linking explicitly selects O0 or O2 and its exact matched provider
object/runtime; Rust whole-source linking uses only its explicit archive.
The full validator has not yet been exercised.

Ring selected9c and unselected storage candidate0dd remain immutable.
Work switches back to bounded focused Ring qualification when the exact
repaired experimental Native tuple is available; full-family adoption and
matched performance review remain separate gates.
