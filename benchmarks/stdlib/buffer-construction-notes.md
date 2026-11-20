# Scalar packing and typed byte-list construction

This checkpoint keeps the existing `pack`, `bytes-from-list`,
`bytes-from-u8s` and `FromList Bytes U8` behavior. Each implementation counts
its final size with a tail loop, allocates one output, then fills checked
intervals with an ordinary tail loop. It adds no imports to embedded modules.
Raw byte construction remains independent of UTF-8 validity; packing accepts
scalar `Char` values and preserves immutable String backing.

The exact source and interface receipts, independent SDK comparison and all
functional results are in
`bootstrap/evidence/stdlib/stdlib-buffer-construction-source-override-results.json`.
The 184 packing and 92 raw-byte observations cover scalar-width boundaries,
embedded controls, arbitrary bytes and lengths through 65537. Both frozen
Rust O0/O2 runs pass the full corpus, separate 64 KiB regressions and 32 repeated
4097-element ownership pipelines, normal 32 MiB/8 MiB and poison 256 MiB/8 MiB,
with zero leaks in all 148 profile processes. O0 IR has entry backedges and no
recursive calls in all five traversal workers (including both generic count
specializations).

The compiler ran with a 64 MiB shell stack and `RUST_MIN_STACK=67108864`.
Those compiler settings are distinct from the compiled programs’ 8 MiB stack
limit. Each selected source is prefixed with a receipt discriminator, and
both emitted interfaces must match independently calculated source hashes.
A frozen compiler cannot accidentally satisfy these checks with its old
embedded constructors.

The fixture generator originally produced deep input by unpacking a complete
repeated string, which temporarily retained two deep character lists. The
final consumer constructs exact-order repeated patterns directly in one list.
The poison runtime intentionally retains freed blocks, so the final consumer
selects eight observations per process instead of accumulating the entire
184-case quarantine. The heap limits were unchanged. The natural-byte list
fixture also requires an explicit refined-list annotation for polymorphic
constructor inference.

Four Rust consumer tests are supplied in `lib_buffer_construction.rs`.
They have not been run by this lane. Fresh embedded Rust, fresh Native
provider/source receipts and shared bootstrap gates remain parent-owned;
none is claimed here. The phase1 repeat/concat source remains frozen separately
for the in-progress base epoch. Performance comparison of this second
constructor increment is also pending.

Reproduction, after sourcing the workspace tool environment and setting the
single-job/test-thread environment:

```sh
ulimit -s 65536
taskset -c 0 python3 tooling/validate_buffer_construction.py \
  --compiler /workspace/dasl-stdlib-io/build/prim-wide-frozen/cinch \
  --string-source build/buffer-construction-phase2/sources/Data.String.cinch \
  --bytes-source build/buffer-construction-phase2/sources/Data.Bytes.cinch \
  --opt-level 0 --artifacts build/buffer-construction-phase2/o0 \
  --output build/buffer-construction-phase2/o0/results.json
```

Repeat with `--opt-level 2` and a separate artifact directory. The SDK generator
is `tooling/go_buffer_construction_vectors.go`; the checked-in fixture also
matches an independent Python byte/scalar model.
