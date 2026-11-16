# URL performance and generated-code review

These samples exercise actual public byte pipelines on CPU0 with one untimed
warmup process and five alternating samples. Input construction and process
startup are outside the monotonic timer. Every operation changes one source byte;
length and three output positions remain observable. Independent SDK outputs
agree in all 90 workload/size/pattern cases. The ten workloads cover component
escaping/unescaping, query parse+encode, URL parse+render, reference resolution,
path joining, exact escaped length and an unrelated fresh-copy control.

Patterns are ordinary ASCII, repeated structural/query punctuation with spaces,
and dense valid percent triplets. All selected 32/1024/65536-byte percent inputs
end on complete triplets, so decoding measurements perform the complete work.
Grammar-specific pipelines can interpret punctuation as query structure; these
are complete public pipelines, not claims about isolated path traversal speed.
Byte results always have independent storage. The SDK string API can return
unchanged strings; conversions and allocation behavior are part of the recorded
pipelines. The reference length workload computes `len(QueryEscape(...))`, which
performs different work from an exact counting API.

The initial assembly kept a dynamic byte policy inside every counting block.
Dispatching once per component cut the query counting body from 665 to 264
assembly instructions and improved the 64 KiB counting pipeline by
**1.91–1.94×** across all three patterns. Machine-counter ordinary validation
also improved the 64 KiB ordinary query-unescape pipeline **1.615×**. The generic
baseline component source is recoverable from commit `2ab8327c`; all actual
module/compiler/LLVM/executable identities are retained in the reports.

The next review found two boxed optional digit results for each validated
percent triple and a second call to the complete percent validator while writing
already validated output. The adopted implementation decodes the two hexadecimal
lanes with pure LLVM and a 256 invalid sentinel, using exact owner-held loads.
Its second pass decodes directly and uses checked stores. All 65,536 possible
digit pairs agree with a digest obtained from the independent public decoder.
The percent validator fell from **1,053 instructions / 89 static call sites** to
**331 / 25**; the output decoder no longer calls the full validator. The complete
before/after source manifests differ only in `Text.Url.Codec`.

The alternating comparison at 64 KiB with dense triplets measured:

| Public pipeline | Before / after |
| --- | ---: |
| Query unescape | 2.010× |
| Path unescape | 1.820× |
| Parse and render | 1.420× |
| Query parse and encode | 1.726× |
| Reference parse and resolve | 1.572× |
| Path join and render | 1.475× |

Dense 32-byte component decoders improved 1.64–1.70×. Ordinary cases were near
parity. The retained 64 KiB ordinary path-unescape sample has a **0.942×** ratio;
its raw timing ranges overlap, and its 206-instruction ordinary validation loop
has identical normalized assembly. This does not establish an end-to-end zero
regression: the sample is retained, without a threshold assertion or removal.

Percent-dense pipelines remain slower than the SDK in the separate public report
(roughly 7–13× at 64 KiB for the dense workloads). Checked byte writes and boxed
natural-number index/proof arithmetic still emit runtime calls; immutable raw
path-hint validation causes additional complete decoding passes during rendering.
SIMD scanning cannot remove those output costs. Ordinary ASCII pipelines often
outperform the measured SDK pipelines. The unrelated allocation/copy control is
included, and no claim treats different work or different allocation guarantees
as a universal language speed comparison. There are no hidden inline-assembly
stores, forged FFI signatures or relaxed compiler proof rules.

The first timing window had exclusive CPU0, with parent Cargo work on CPU3,
character Native validation on CPU1 and collections Native work on CPU4. URL
performed no CPU2 work. The final pair comparison and SDK report had exclusive
CPU0 and CSV functional diagnostic compilation on CPU2; no URL CPU2 work ran.
These are coordinated shared-machine samples. Comparisons alternate retained
executables to reduce load drift; do not derive optimization speedups by comparing
absolute medians across separate windows.

Each report records raw samples, checksums and immutable source/executable/LLVM
identities. Supplemental toolchain provenance records the frozen compiler's
actual source snapshot. Legacy `compiler_source_sha256` and
`runtime_source_sha256` fields describe the worktree context, not an assertion
that those sources built the frozen binary. Rust measurements use host runtime
archive `c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac`;
Native functional validation uses provider archive
`80bd5b546742f8374221ac445ec20b7cfd0df0958d5c39313cde077204d6e141`.
Both archives predate every retained executable and remained unchanged. Runtime
or compiler changes were not mixed into optimization comparisons.

The final candidate passed complete component, URL capability and exhaustive
pair-domain suites with both frozen frontends in ordinary and poisoned memory:
302 processes report zero leaks. Actual load, SIMD, SWAR and pair kernels compile
to WebAssembly and AArch64 objects. The shared Rust target also supplies a
big-endian profile gate, awaiting the parent's coordinated Cargo run.
