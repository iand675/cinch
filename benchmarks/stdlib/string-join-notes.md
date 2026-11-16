# Immutable String clone and join

`string-clone` copies into private immutable backing. `string-join` takes a list
of valid Strings and a separator, counts exact bytes with a tail loop, allocates
one backing buffer, then copies pieces and separators through checked bulk
operations. Empty pieces retain adjacent separators; an empty separator shares
the existing concat path. Neither API copies each input into temporary mutable
buffers or exposes private backing. This increment adds no module imports.

All 450 independent reference observations match the pinned SDK. Frozen Rust O0
and O2 source-override consumers pass every exact byte/scalar/digest observation,
plus mutable boundary copies and 100 temporary-owner pipelines per profile.
All 268 final normal/poison processes finish with zero leaks at 32 MiB/8 MiB
and 256 MiB/8 MiB. The compiler shell and Rust worker stacks were separately
64 MiB. Each eight-row batch and ten-pipeline owner batch runs in its own process
to bound deliberate debug quarantine. Both O0 join traversal workers have entry
backedges and no recursive calls. Four new declarations have canonical docs.

The current source SHA is recorded in
`bootstrap/stdlib-string-join-source-override-results.json`, together with
canonical and marked interface hashes, exact fixture/compiler/executable
identities and all process results. The source override has its own receipt
discriminator; no old frozen embedded implementation can satisfy it.

Four Rust tests are supplied with explicit `Text.Hex` and `Data.Bytes.Word`
source closures. Cargo has not run in this lane. Fresh embedded Rust, fresh
Native provider/source receipts and Native corpus checks remain pending with
parent integration. The phase1 repeat/concat and phase2 pack/byte-constructor
source snapshots remain frozen independently. Public pipeline timing is also
pending; this checkpoint claims functional source-override validation only.

Reproduction is `tooling/validate_string_join.py` with the frozen Rust compiler,
selected `--module-source`, separate artifact directories and `--opt-level 0`
or `2`. Reference generation is in `tooling/go_string_join_vectors.go`; a
separate Python exact-byte/scalar implementation produced the committed fixture.
