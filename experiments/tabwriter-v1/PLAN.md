# Tabwriter package plan

Status: independent partial candidate. The actual SDK protocol fixture and checked configuration source are now captured in `OBSERVATIONS.md` and `reference-gates.json`. The formatter, streaming API, full operation qualification and public performance review remain open. This directory is separate from the frozen buffered and cycle experiments; it adds no provider, inventory, website or production collector change.

## Actual declaration map

The installed SDK's cumulative compatibility files contain **19 exact records**, mapping to **12 current exported identities**. Seven additional records describe historical constant typing, not seven extra operations. `reference-surface.json`, `reference-provenance.json` and `capability-plan.json` preserve every record, its installed source location and its planned native operation. The current identities are six flag constants, the escape byte, `Writer`, `NewWriter` and the three methods `Init`, `Write`, `Flush`. No operation is invented to fill the count.

The SDK is consulted for behavior and used as an independent observer. Its implementation is not vendored or copied. Source/license/API-file hashes are retained. The elastic-tabstops concept is described at <https://nickgravgaard.com/elastictabstops/>; the planned implementation uses owned byte chunks and an explicit iterative layout stack.

## Proposed homes and public contract

`Text.Tabwriter` owns checked configuration, flags, a byte-preserving layout engine and a pure complete formatter. `Effect.Tabwriter` owns a mutable, region-indexed writer handle over an explicit output capability. Public Cinch doc comments will describe this library directly, without reference-language names.

| Native surface | Contract |
| --- | --- |
| `TabConfig`, `tab-config` | Check signed minimum width, tab width and padding before constructing a valid configuration; reject negatives and values outside the reference machine interval with named errors. Padding accepts every byte value; the documentation's ASCII recommendation is not a runtime restriction. |
| `tab-filter-html`, `tab-strip-escape`, `tab-align-right`, `tab-discard-empty-columns`, `tab-indent`, `tab-debug` | Explicit `U64` flag bits1,2,4,8,16,32. Unknown bits have no formatting effect. Tab padding forces left alignment; tab width zero emits no tab padding. |
| `tab-escape-byte` | Raw byte255. It is not the UTF-8 encoding of code point255. |
| `tab-format-bytes` | Format complete arbitrary bytes and return unconditionally fresh bytes, including empty/no-change input. No invalid-byte loss or implicit repair. |
| `tab-format-string` | Explicit valid-text facade. A non-ASCII padding byte or retained raw sentinel can make output invalid; return a checked error with lossless raw output access rather than discard or replace data. The raw byte formatter remains authoritative. |
| `TabWriter (r :: Region) e`, `tab-writer` | Initialized opaque handle containing a private initialized IO-indexed cell. Aliases share formatting/progress state, matching a mutable writer protocol. No backing array/pointer is public. |
| `tab-writer-init` | Atomically replace configuration/output and reset buffered layout without flushing or closing the old output. Earlier aliases observe that same logical handle. Invalid configuration is refused before mutation. |
| `tab-writer-write` | Return consumed **input** count plus optional output failure; this count is distinct from bytes emitted. Preserve automatic flush checkpoints and advanced output state on errors. No invented sticky-error policy. |
| `tab-writer-flush` | Complete buffered formatting, treating an unfinished escape as complete for formatting, then reset even if output fails. The writer remains reusable. It never closes a borrowed output. |
| `TabOutput (r :: Region) e`, `tab-output`, `tab-output-from-stream` | Explicit generic output capability returning signed accepted count, optional original error and advanced output handle. This preserves data-plus-error outcomes and arbitrary error payloads. A bridge to established `Effect.Stream.ByteWriter` retains its original `StreamError`; no shared Stream edit. |
| `tab-output-map-error`, `tab-writer-output` | Explicit error-union adaptation for changing output families, and observation of the actual advanced output. No implicit stringification of arbitrary errors. |

Names are the proposed API to review, not declarations already implemented. The language has no implicit zero-value construction for an opaque initialized handle; required initialization is represented by its checked constructor. A busy/reentrant operation needs a named checked refusal rather than corrupting internal buffers. Thread confinement is not inferred merely from the Region parameter; accepted sharing/transfer contracts must be checked on the actual compiler.

The output capability has no reader/transfer callback lending. Its stream adapters retain a cell but the cell stores only the underlying output, never its own returned adapter. Normal nested writers form a directed ownership graph. This is independent of the known buffered source/sink cross-retention cycle; no callback expiry or disposal requirement is introduced.

## Layout and incremental parsing

Input cells end at horizontal tab, vertical tab, newline or formfeed. Hard/soft tabs end aligned column cells; the final trailing cell in a line is not aligned. Alignment groups are maximal contiguous rows containing a column. A final tab without another cell behaves differently from a tab followed by newline, so finalization must preserve actual cell structure.

Every Unicode scalar occupies one width unit, including combining and wide characters; malformed UTF-8 follows per-byte replacement-rune progress for width counting while preserving original bytes. Chunk boundaries must not prematurely count an incomplete multibyte prefix. The parser keeps bounded UTF-8 carry and completes it only at a semantic boundary or flush.

HTML filtering is a byte protocol: tags close at the first `>`, entities at the first `;`; no HTML validation, quoting parser, entity decoding or Unicode terminal-width algorithm is implied. They pass through byte-for-byte with widths zero and one. Escape-byte-delimited segments ignore internal tabs/newlines/formfeeds and HTML markers; strip mode removes only delimiters. Incomplete sequences at flush and every possible chunk split need captured observations.

The layout engine uses an explicit task stack over row tails. At column depth `j`, contiguous rows with an aligned cell form a block; compute that block's maximum width/discardability, then process its tails at depth `j+1`. A leaf emits its complete original row using the accumulated widths. Each cell is classified a bounded number of times and each output cell is emitted once; a single wide row among many short rows must not cause a rows-times-maximum-columns scan or recursive native-stack growth.

Soft-only empty columns may have zero width; hard tabs prevent discarding. Emptiness for indentation uses the cell's byte size, whereas discardability uses width and hard-tab status. HTML-only cells demonstrate why these are distinct. Debug output includes separators for zero-width columns and section bars after an outside-escape formfeed.

## Exact output and error sequencing

The observer must capture underlying output **offers**, not only final bytes. The SDK can issue empty offers, eight-byte padding runs, individual separators/newlines and cell slices. Short success is an error, not a request to retry. Calling a generic write-all adapter would change offer/error behavior.

A cell fragmented across input Writes must still be coalesced into its proper single cell offer; emitting each input chunk separately would change short-write/failure behavior. Padding calls with zero length and the final buffered-line empty offer remain observable too. The established `Data.Bytes.View` is mutable and therefore cannot be substituted as a readonly output lease.

The implementation should use a pure formatting program with explicit output steps. Each step identifies its input-consumption checkpoint and active width path. The effect interpreter snapshots the offered bytes, calls output once, records its advanced continuation and either continues or returns the original failure. Retained parsing state must never expose writable backing to a custom output callback.

Automatic flush during `Write` and explicit `Flush` have different failure cleanup. The installed implementation resets on explicit Flush failure, but not on a Write-triggered failure. A failed layout can leave an active width path that affects subsequent calls. Do not silently substitute a resumable cursor, sticky failure or unconditional reset. Captured failure/retry/Init traces must determine the precise observable policy before selecting internal state transitions.

Invalid negative/excess sink counts are a named native checked refusal; they must not become an unchecked index or silent accepted prefix. Genuine output failures retain their opaque error payload. Host panics and concurrent/reentrant misuse are not converted into successful formatting results.

Extreme nonnegative widths also need bounded oracle observations: the installed machine arithmetic can overflow during padding calculations. The native implementation must use exact/checked arithmetic and a named layout-size refusal rather than reproduce an unchecked wrap or slice fault. A bounded observer sink must stop enormous output attempts and record that limit explicitly; it must not claim an unbounded reference case terminated normally. Plain trailing text that never needs padding must not be rejected merely because an unused configuration value is large.

## Ownership, acceleration and dependency gates

Each Write snapshots caller bytes before retaining them. Cell/row spans hold their owned chunks; public raw output and error-byte observations are fresh. No whole-input recopy per delimiter, repeated list append, output byte-at-a-time spine, or callback exposing mutable internal storage is acceptable.

The private IO cell may reuse only the already vetted slotsNew/Read/Write signatures and initialization pattern; no new ABI, pointer access or checker exception. Generic state is stored as an object-valued carrier, avoiding a generic wide-slot representation assumption. Actual public/private defining Region contracts are required. The exact Stream dependency must be admitted and recorded; historical Type-header metadata cannot be relabeled as Region qualification. A separate source copy is appropriate until that family is promoted.

Readonly SIMD/SWAR scans should find ordinary control bytes, escape delimiters and optional HTML starts in bounded runs. ASCII runs advance/count in bulk; non-ASCII fringes use the qualified UTF-8 layer. Primitive bounds are ordinary proved owner-relative spans, variable shifts are bounded, and Native primitive arguments are computed in locals. Inspect actual O2 calls/assembly and qualify production kernels at every alignment/tail before any performance claim. Scalar width arithmetic does not need artificial SIMD.

## Independent validation and review sequence

1. Capture a new pinned-SDK fixture set, including final bytes, each Write's consumed count/error, every underlying offer, Flush/Init recovery, and explicit panic/contract-violation markers. No vector set has run yet.
2. Cover all flag combinations, zero/nonzero/negative and boundary configurations, all padding byte values, hard/soft ragged columns, final tab/newline/formfeed distinctions, debug markers, raw NUL/invalid UTF-8, Unicode and every split of UTF-8/HTML/escape fragments.
3. Include short/error/empty-output sinks, positive-prefix-plus-error, automatic versus explicit flush failure, recovery/reinitialization, retained aliases, hostile input/output mutation, advanced output observation, nested writers and repeated reclamation. Keep every original failure epoch plain.
4. Source/type/proof first on actual matched compiler families, then bounded normal32M/poison256M/8M functional consumers with zero leaks; actual readonly kernels and supported foreign objects; SDK-free Rust consumers supplied for parent serial integration. No local Cargo or provider changes.
5. Preflight complete public formatting/writer pipelines with independent checksums, then coordinate paired ordinary-RC measurements. Use aligned/plain, ragged, HTML, escape, Unicode, fragmented and failure-free writer workloads plus fresh-copy controls. Review allocations/hot calls and preserve regressions. Module presence or a declaration map alone is not completion.
