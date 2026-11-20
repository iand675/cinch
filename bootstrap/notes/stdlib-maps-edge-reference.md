# Supplemental independent map reference

The original published reference generator and its 800-step/14-key-IEEE/nil
fixture remain unchanged. `go_maps_edge_vectors.go` is a separate prepared
generator for the new supplemental edge consumer. Its actual pinned SDK
run passed in 0.293s on CPU1 using the existing surface cache. This does
not establish any new library runtime verdict.

The generator calls the pinned SDK package directly, rather than modeling
the Cinch map algorithms. JSON contains finite integers, binary64 bit
patterns, booleans and counts; it never serializes a NaN as a JSON number.
No unspecified key iteration order is recorded.

| Capability | Independent observation | Cinch contract under test |
| --- | --- | --- |
| NaN values | Equal on the same map and its shallow clone; EqualFunc with bit comparison before and after replacing the clone's value | Explicit IEEE value comparison must remain non-reflexive without a map-identity shortcut. Explicit bit comparison is a separate predicate. No global floating Eq instance is introduced. |
| Current-key predicate replacement | DeleteFunc callback verifies, deletes and reinserts its current key, then selects that key for deletion | DeleteIf must delete the current replacement by key after its predicate, rather than only unlinking the old captured identity. |
| Equality callback mutation | EqualFunc's first predicate clears the right map, then records equality, callback count and final sizes | Later right-side lookups must observe the current map and stop before invoking the predicate for a missing key. |

The selected library has its own documented identity-snapshot contract;
these observations do not imply equivalence for every unspecified mutation
or traversal implementation. Both representations must pass the same
supplemental fixture and established full matrix before a candidate timing
can select the private entry-cell representation. The actual source-selected
Native row refusals and genuine interface-header feature remain independent
compiler gates.

The actual SDK is go1.27.4. Generator SHA is
`ca5d896322fbbddbe261eb4e65c6ca48c772f3b84170aa9a47238ab65e70f408`;
raw JSON SHA is
`463cb975559bde910c67ef5aeda54d0958cf1eba2aed6f343723660e8b730c47`.
The exact command, environment, SDK source hashes and observations are
recorded in `../evidence/stdlib/stdlib-maps-edge-reference-results.json`; the raw fixture is
`maps-edge-vectors.json`. An initial 4ms launch failed at the read-only
default cache; its separate receipt is preserved, and only the cache
environment was corrected for the successful run.

`generate_maps_cases.py --edge-control` now renders every expected bit
pattern, equality result, callback count and final size from that actual
JSON into the supplemental consumer template. The maintained Cargo test
and frontend runner both use this renderer. Existing eight trace and two
IEEE-key rendered consumers were checked byte-for-byte against their
original successful receipt hashes and remain unchanged. The new edge
consumer now passes on both baseline0fc and C839 with exact855/c4/LLVM20,
normal32M/8M and poison256M/8M, four zero-leak processes. Its exact separate
qualification is recorded in `../evidence/stdlib/stdlib-maps-855-edge-results.json`. Existing
successful matrices and baseline timings are not rerun or relabeled by
this independent-reference generation.
