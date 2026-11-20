# PEM implementation review

`Text.Pem` and the `Effect.Pem` writer facade implement all seven declarations in the pinned `encoding/pem` surface. The exact mapping and reference-source/oracle hashes are in [pem-capabilities.json](pem-capabilities.json). No reference implementation source was copied.

Blocks and headers own private immutable byte storage. Label, key, value and payload bytes can contain malformed UTF-8. Construction snapshots mutable inputs; getters, lookups and decoded remainders return fresh mutable storage. Persistent field/header updates preserve earlier blocks. Headers have exact byte keys, stable last-occurrence precedence and deterministic byte order; exact `Proc-Type` precedes other encoded headers. Metadata canonicalization uses stable merge sorting rather than quadratic insertion.

Decoding implements first-END/final-BEGIN recovery, relative recovery-slice anchoring, Unicode header-edge trimming, exact line/CR handling, terminal malformed-header cases and permissive standard padded Base64 after removing spaces/tabs. Encoding refuses colon-bearing header keys before output. Arbitrary label/value bytes are retained, including line breaks; such metadata need not decode back to the same block.

The idiomatic adaptations are explicit: blocks are non-null values; empty/nil headers have one logical empty representation; memory encoding returns a typed refusal instead of an absent buffer; decoded remainders are fresh; and writing returns the advanced sink on success or failure. Positive short writes are drained, zero progress and invalid counts are refused, and borrowed sinks are neither flushed nor closed. Callback chunk boundaries are unspecified.

## Selected implementation

The functional checkpoint is `94e9b83b`, source SHA `35690bf1e13e59bef3337d31b122dd3dcaed2be6cd7b8c1611634de823d32cb4`. The selected source is SHA `9478c3c41a71436180df02c7ac175c309f790f39d131440895f0a81c7f733efd`:

* Payload work batches 768 input bytes, retaining canonical 64-column output and at most 1,040 callback-visible body bytes per batch. All writes and copies remain checked language operations.
* Marker loops use bounded machine positions under an owner hold. Sixteen-byte read-only LLVM scans have exact scalar fringes; marker comparisons read exactly 11, 10 or 9 bytes.
* END search retains a dash prefilter. A discarded newline-prefilter variant tested every Base64 line and regressed ordinary decoding. Its source identity and measurements remain in the experiment report.
* Priority-header matching uses an exact bounded nine-byte LLVM read instead of allocating a comparison buffer for every metadata visit.

The reproducible candidate generators take the original checkpoint source via `--baseline`; the maintained source already contains the selected changes. Only five existing helper implementations changed, one redundant helper was removed, and four helpers were added. Public function implementations, validation, ownership, ordering and error policies remain unchanged.

## Validation

The checked-in independent corpus contains 473 exact encodings, 1,716 decoder/field/remainder observations and 51 actual writer-budget prefixes. It covers every truncation/mutated framing byte, nested/ambiguous recovery, malformed Base64, Unicode and invalid-byte metadata, CR/LF edges, alignments, duplicate headers and 1,000-header cases. Decoder consumers mutate their source after parsing and recheck fields/remainders.

Separate controls cover getter/input mutation, persistent edits, retained callback buffers, one-byte/seven-byte short writes, every failure budget, zero/invalid-count sinks, advanced continuations and a 64 KiB public writer pipeline. The selected source passes the complete corpus and controls under both frozen frontends at O0 and O2: 1,112 memory processes with zero leaks. O0 receipts explicitly identify reused, source/compiler/provider-checked IR and distinct clang relinks.

The actual eleven private primitives have exhaustive marker/lane/byte mutations, including all 2,304 priority-key substitutions. Host O0/O2 normal/poison runs and wasm/ARM object compilation are recorded separately. The SDK-free `lib_pem` target also supplies a big-endian PowerPC object profile; local frozen-CLI receipts do not claim that parent Cargo gate ran.

Normal consumers use 32 MiB heap and 8 MiB stack; poisoned consumers use 256 MiB heap and 8 MiB stack. Compiler-host stack is separately fixed at 64 MiB. Every new declaration has canonical documentation. Four SDK-free Python fixture freshness/shape tests pass.

## Public performance review

[pem-public-benchmarks.json](pem-public-benchmarks.json) retains six experiments, all samples and regressions. The final exact-source comparison covers both frontends, six public pipelines, three sizes and three metadata/preamble patterns: 54 conditions per frontend, five samples plus warmup, rotating baseline/candidate/reference order. Each candidate and baseline first passed 972 reference-fingerprint/zero-leak preflight conditions. Fingerprints preserve output allocation length and selected bytes; they are not cryptographic full-output digests. Complete-byte correctness comes from the separate corpus and large ownership/writer controls.

Inputs are prepared outside timing; bounded machine iteration/checksum loops avoid unrelated optional-remainder allocation. Decode includes fresh mutable label/payload observations; the reference makes matching snapshots. Header-count observation traverses the immutable list, while the reference map exposes length directly. The separate fresh-copy pipeline is a control. These results describe these public ownership policies and this host, rather than interchangeable representation costs.

Final Rust 64 KiB encode medians improve about 2.15–2.23 times without the large-header pattern, writer medians about 2.27–2.58 times, and dash-heavy recovery about 2.69 times. Ordinary 64 KiB decoding remains roughly 3.5–3.6% slower; the large-header decode row is about 7.4% slower. These regressions are retained. Copy-control rows vary substantially in some runs. The selected dash strategy avoids per-line END tests and retains the much larger encoding/writer/adversarial gains; no universal speedup is claimed.

Final Native 64 KiB encode improves about 1.25–1.28 times, writer about 1.65–2.53 times, and dash-heavy recovery about 5.35 times. Several small/ordinary rows regress. Native absolute large pipelines remain substantially slower than the reference and Rust frontend; the actual assembly retains 24 imported `Data.*` call sites in the Base64 encode tail, 50 in its decode tail, and 13 in PEM line copying. In contrast, selected private marker/blank machine loops retain zero imported `Data.*` calls. No Base64 source was edited. The parent owns comparison against a later admitted scalar-lowering compiler epoch.

CPU0 was granted exclusively for executable sampling; own CPU2 builds and queued JSON work were held. The host exposes five cores but has a four-CPU cgroup quota. Final Rust and Native runs record `cpu.max` and `cpu.stat` before/after, with zero throttling delta; earlier runs did not collect those counters. No samples are corrected or dropped. Background seed/target activity and exact artifacts are recorded in each experiment.

## Integration boundary

The Rust producer is `96c4f542…`; the fully self-built Native frontend is `2f07707e…`, source `e494aac8…`, with exact matched String e5/provider object 46a7/runtime 80bd payloads. Source-distributed consumers use explicit admitted manifests and no embedded-source shadows. The Native epoch does not include later array certificates or scalar-import lowering. Parent Cargo/integration gates remain separate from these standalone receipts. This review does not claim the entire standard-library initiative is complete.
