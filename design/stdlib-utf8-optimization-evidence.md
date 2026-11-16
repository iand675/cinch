# UTF-8 scan optimization

The selected source preserves all public byte/scalar/error/ownership contracts from `be3fbdaf`. Complete ASCII blocks use a sixteen-byte read-only LLVM SIMD sign-bit mask, with eight-byte SWAR and exact byte fringes. Non-ASCII bulk count/valid/repair scans classify ASCII and impossible leading bytes immediately, then use packed four-byte SWAR continuation/range checks returning only width and validity. Public scalar decoding and full-prefix queries retain the complete decoder. All stores remain checked language writes or bulk copies, with source ownership held across raw loads.

Two codegen findings drove the change. In the Native source-client epoch, imported `Data.Machine` arithmetic/comparison/conversion operations remained external PLT calls on each scan iteration. Small source-local straight-line LLVM helpers let O2 fold those operations into direct counters and addresses. Second, bulk scanning computed decoded scalar values and incomplete-prefix metadata that its callers never used. The dedicated width/validity kernel removes that work. SIMD-only and fused-but-full-decoder candidates were retained because they regressed some Rust mixed/malformed paths; they were not selected.

All controls and candidates use the same public operation driver. The eight-byte patterns are ASCII `abcdefgh`, valid mixed-width `aéΣ€`, and malformed hex `61ff8000c0f080bf`. Every iteration mutates the first ASCII byte and retains a checksum. Each result records three raw monotonic-clock samples plus warmup, 32 iterations, 32-byte and 65,536-byte inputs. Preparation/startup are excluded; mutation, public calls, natural-counter/checksum loop and output fingerprint are included. Copy is a separate allocation control. Grouped reference repair includes text/byte conversions and forces its fresh byte output to escape; per-byte repair composes standard decoding, and is covered by functional preflight rather than the measured rows. Runtime allocation/reclamation strategies differ, so repair/copy comparisons describe the complete pipelines, not identical instruction workloads. Tiny-input results include substantial fixed driver/API cost.

Both coordinated CPU0 windows completed without another CPU0 job: 16.8 seconds for 192 rows, then 9.5 seconds for 144 rows. Other cores ran the explicitly recorded parent/JSON/Base work. The eight initial artifacts and two minimal-step artifacts passed 2,400 boundary/preflight cases, including 0..33, 64 and 65,536-byte inputs, against independent pinned SDK checksums. Raw timing samples for all 336 rows, including regressions and both compiler epochs, are retained under `benchmarks/stdlib/utf8-scans`.

Second-window medians in microseconds per 64 KiB public operation:

| Public operation/pattern | Rust SWAR8 | Rust selected | Native SWAR8 | Native selected |
|---|---:|---:|---:|---:|
| count ASCII |5.95|3.37|75.29|3.63|
| count mixed Unicode |446.93|214.36|937.66|230.67|
| count malformed |854.30|248.14|1717.44|258.80|
| valid ASCII |5.49|2.63|57.00|3.05|
| valid mixed Unicode |411.98|224.84|976.96|228.49|
| repair ASCII |11.03|7.34|120.86|8.50|
| repair mixed Unicode |821.26|441.63|1887.12|487.53|
| repair malformed |2902.43|822.47|5758.43|2310.48|

The early-invalid Native validity case is a retained regression: 342→481 ns in that window. It exits after a short prefix, so it is not a whole-input throughput measure. The pinned reference remains faster on several mixed/malformed paths, and no universal speed claim is made. The selected source improves every measured 64 KiB count/repair and full-input validity path relative to the validated SWAR baseline. ASCII count now lowers to `movdqu`/`pmovmskb` plus direct machine counters; it has no imported arithmetic calls in that hot branch. There is no per-iteration vector/table stack materialization.

Measured minimal source SHA256 is `8abadc90c6e95de90716f0173e237bdcb38a67395399891f1384ea3800d7e3c0`. Final source SHA256 is `59ab5f5cecd21e4802b0850447e03d1390073887d598dbef22680da39cf00d71`; its only subsequent changes clarify two comments, and the driver clarifies one comment. Exact final-source rebuilds emit byte-identical LLVM and assembly to the measured variant under both frozen frontends. `prepare_utf8_scan_candidates.py` reproduces every retained candidate from the frozen baseline, with exact hashes.

The exact final source passes both complete independent corpora: all 1,114,112 numeric code points, 65,536 byte pairs and 12,773 malformed/Unicode/alignment rows; 2,002 memory processes, all zero leaks. Normal runs use 32 MiB heap/8 MiB stack and all bounded poisoned groups use 256 MiB/8 MiB. Separate ownership/alias/atomic-capacity/refusal consumers pass both modes on both frontends. All 22 actual read-only primitives compile to wasm32 and AArch64 objects. The expanded `lib_utf8` Rust target also supplies the powerpc64 big-endian object gate, reserved for parent Cargo integration. Ten generator/corpus tests pass; 70 byte-module declarations and 59 named-table declarations extract canonical docs with zero diagnostics/orphans. The named-table change only clarifies category `Cs`/`Cn` and missing-script documentation.

Rust remains compiler `f2d5ae4c…`/runtime `c4b1097b…`. Native remains gated Char-contract compiler `9e2a64e0…`/source `0b11df4d…`/runtime `12945b71…`. Functional Native corpora use the wrapper's default O0 link. Every measured Native executable is separately and explicitly linked with `clang -O2` and the matched frozen provider/object/runtime; its O0 executable is retained alongside it. Exact identities, commands, samples, corpus receipts, foreign objects and comment/codegen equivalence are recorded in `benchmarks/stdlib/utf8-optimization-evidence.json`.

A separate benchmark setup probe found that the 9e epoch rejects a dynamic `nat-rem i (bytes-len pattern)` refinement inside the inherited fill helper. All benchmark alphabets have exactly eight bytes, so the helper uses the equivalent constant divisor 8. The original failing source and diagnostic are retained in `build/utf8-bench-native-dynamic-rem-regression`. No checker rule, ABI, Char contract, memory limit or mutation rule was weakened.

The five valid-String facade adapters remain separate work. The approved plan uses import-free immutable `Data.String` first/last observations and an optimized sealed-valid scalar count, then a `Text.Utf8` facade. Integration waits for the exact helper checkpoint and fresh combined phase2/3/4 provider gate; `Data.Bytes.Utf8` keeps its no-String dependency.
