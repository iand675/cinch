# Exact byte search kernels

`Data.Bytes.Search` provides `bytes-index-byte`, `bytes-last-index-byte`,
`bytes-contains-byte` and `bytes-count-byte`. Offsets are byte offsets. Every
byte value is compared exactly, including malformed UTF-8. Missing offsets
are `None`; containment is false and count is zero for empty input.

Complete eight-byte blocks use an unaligned private LLVM load and an exact
SWAR matching mask. The masked lane addition cannot carry between bytes, so
first/last/count never inherit cross-lane borrow false positives. Big-endian
loads reverse word order before lane classification. Ordinary machine
leading/trailing-zero and population-count intrinsics consume the resulting
mask. These emit the same LLVM operations while satisfying the frozen Native
frontend's older inline-call containment contract. No safety check is bypassed.
Scalar fringes load exactly one available byte at a time. Signed machine
counters and one backing-owner hold surround the private loops. Both frontend
IR families have no owner, byte-runtime or allocation calls inside those loops.

All 3,681 independently generated rows match the pinned SDK and pass both
frozen frontends. The corpus includes every byte value, zero/one adjacent lanes,
all word fringes and lengths through 65,537. Each frontend also passes 100
pipelines with temporary 16,385-byte inputs. All 272 final normal/poison profile
processes finish with zero leaks, using 32 MiB/8 MiB and 256 MiB/8 MiB respectively.
Separate processes bound the debug runtime's deliberate freed-block quarantine;
all rows and pipeline iterations still run under the original positive limits.

Actual private kernels compile to wasm32 and AArch64 objects. Five supplied Rust
tests also include a big-endian object gate; Cargo was not run in this lane.
The documentation extractor checks all 19 declarations and module documentation
without diagnostics or orphans. Exact compiler, source, fixture, executable,
portable-object and IR receipts are in `bootstrap/stdlib-byte-search-results.json`.
The frozen Native compiler is not claimed to contain the separate embedded
constructor fixes. Paired public-pipeline timing and assembly review remain open.

Reproduction uses `tooling/validate_byte_search.py` with either the frozen Rust
CLI or the qualified Native wrapper plus its `--native-interfaces` manifest.
The runner copies source-distributed dependencies and retains their exact
identities. Compile with a 64 MiB shell stack and `RUST_MIN_STACK=67108864`;
compiled test programs independently use their explicit 8 MiB stack limit.
