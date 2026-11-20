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
portable-object and IR receipts are in `bootstrap/evidence/stdlib/stdlib-byte-search-results.json`.
The frozen Native compiler is not claimed to contain the separate embedded
constructor fixes. Paired public-pipeline timing and assembly review remain open.

Reproduction uses `tooling/validate_byte_search.py` with either the frozen Rust
CLI or the qualified Native wrapper plus its `--native-interfaces` manifest.
The runner copies source-distributed dependencies and retains their exact
identities. Compile with a 64 MiB shell stack and `RUST_MIN_STACK=67108864`;
compiled test programs independently use their explicit 8 MiB stack limit.


## Measured optimization follow-up

The subsequent source preserves all four APIs and exact offsets. It adds a
single scalar first/final-byte check, local straight-line LLVM machine counters
and predicates, and a horizontal sum specialized to the eight exact match bits.
The generic x86 target had expanded population count into a long software
sequence. Native imported arithmetic also remained external object calls inside
each block. Assembly now shows direct arithmetic; the Native O2 count loop
additionally vectorizes into 32-byte groups. No stores or branches are hidden
inside primitive blocks, and checked mutations remain in the caller.

The final candidate passes all 3,681 rows and 100 owner pipelines under frozen
Rust O2, Native O0 and a separately relinked Native O2 corpus. All 408 normal and
poison corpus processes finish with zero leaks. Actual wasm32 and AArch64 kernels
compile, and all 29 implementation declarations have canonical documentation.
The actual-kernel Rust test checks the new horizontal-sum instructions; the
parent still owns the Rust/big-endian gate.

Three variants use the same driver: equivalent local scalar loops, the original
`c41c674d` implementation, and the candidate. The driver mutates three checked
bytes before each query, compares independent checksums and full input digests,
and keeps input reads after the ending clock. All 288 memory-profile smokes and
900 measured processes pass those observations and report zero leaks.

At 65,537 bytes, compared with the original implementation, final count pipelines
improve 1.59–1.72× in Rust and 14.31–16.18× in Native; combined query pipelines
improve 1.50–1.59× in Rust and 9.67–16.12× in Native. Against equivalent local
scalar loops, large combined pipelines improve 2.27–3.08× and 3.47–4.32×
respectively. These are shared-machine observations, not universal guarantees.

Constant-time dense first/last queries initially received only eight iterations
at the largest size. A retained follow-up uses 2,000 iterations at every size;
most results are near parity. Shared-host outliers occur in both directions:
for example 1.58× for one Rust index condition and 0.665× for one Native last
condition. Every corrected and uncorrected sample is retained. Those small-query
outliers do not justify further source changes. The initial reports, driver and
scalar source are also preserved for reproduction. Parent CPU3 gates, a Base
CPU1 chain and possible CPU2/4 functional work continued during measurements;
CPU0 was explicitly assigned exclusively to this lane for each timing window.

Complete source/profile, benchmark and assembly receipts are in
`bootstrap/evidence/stdlib/stdlib-byte-search-optimization-results.json`. The runner is
`tooling/bench_byte_search.py`; Native builds explicitly relink generated IR at
O2 with the frozen provider object and runtime. Source-only validation does not
claim the separate embedded String/Bytes constructors or a fresh Native base.
