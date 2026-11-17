# Supplemental independent map reference

The original published reference generator and its800-step/14-key-IEEE/nil
fixture remain unchanged. `go_maps_edge_vectors.go` is a separate prepared
generator for the new supplemental edge consumer. It has not yet run and
does not establish any new library runtime verdict.

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

After a caller-granted tiny SDK window, preserve the exact SDK version,
generator/output SHA, raw JSON and command receipt. Verify the three groups
against `mutable-map-edge-control.cinch` before calling that new fixture an
independent reference gate. Existing successful matrices and baseline
timings must not be rerun or relabeled by this preparation.
