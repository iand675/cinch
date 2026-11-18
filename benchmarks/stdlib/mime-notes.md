# MIME capability checkpoint (in preparation)

Source checkpoint `cbff6190` implements all 16 pinned declaration records.
Both admitted frozen frontend/provider pairs pass the full 3,119-row corpus,
ten callback policies and nine ownership/continuation controls under normal and
poisoned-free profiles with zero leaks. Parent integration passed the current Rust Cargo regression: both tests
(2/2) passed in 90.53 seconds. The newer Native family remains a separate
admission-dependent representative/benchmark scope.

The pinned `mime` audit contains 16 declaration records. The repeated literal and
typed constant records are distinct audit entries for the same two values.

| SDK records | Explicit Cinch capability |
| --- | --- |
| `BEncoding = 98`, `BEncoding WordEncoder` | `mime-b-encoding`, exact U8 value 98 |
| `QEncoding = 113`, `QEncoding WordEncoder` | `mime-q-encoding`, exact U8 value 113 |
| `WordEncoder uint8` | Public `MimeWordEncoder U8`; every selector is retained |
| `WordEncoder.Encode` | `mime-word-encode-bytes`; checked String convenience is separate |
| `ParseMediaType` | `mime-parse-media-type` plus base/nullable-parameters/error observations |
| `FormatMediaType` | `mime-format-media-type` over exact-key owned parameter contexts |
| `ErrInvalidMediaParameter` | Named `MimeInvalidMediaParameter` failure constructor |
| `AddExtensionType` | `mime-registry-add` returns a new explicit context or the exact error |
| `TypeByExtension` | `mime-registry-type`, exact spelling before Unicode lowercase fallback |
| `ExtensionsByType` | `mime-registry-extensions`, sorted historical lowercase associations |
| `WordDecoder struct` | `MimeWordDecoder r e`, explicit region and arbitrary caller error type |
| `WordDecoder.CharsetReader` | Inspectable/replaceable/clearable optional callback receiving lowercase label and a fresh input reader |
| `WordDecoder.Decode` | `mime-word-decode`, with checked String convenience |
| `WordDecoder.DecodeHeader` | `mime-header-decode`, preserving literal recovery, whitespace and callback error rules |

Wire values use Bytes because the source language's strings can contain malformed
UTF-8. Charset content labelled UTF-8 is passed through without validation; only
checked String conveniences reject invalid output. Error diagnostic spellings
are immutable quoted text, including distinctions between invalid source bytes
and valid nonprinting Unicode scalars. No input/output buffer aliases retained
parameter or registry state.

The media parser distinguishes a valid base with a parameter-syntax failure from
base failure and conflicting duplicates. Success exposes a non-null empty map;
parameter failures expose no map. RFC 2231 single-part precedence, simple versus
encoded segment precedence, gaps, ignored language tags, unsupported charset
behavior and malformed-piece omission preserve the observed package behavior.
Formatting sorts original exact keys before lowercasing their spellings. Keys
that differ only by case remain distinct inputs to formatting.

The extension registry has 65 deterministic built-in facts and accepts explicit
association imports. It has no hidden process-wide mutations or ambient host
initialization. Host-specific mappings must be seeded into the context explicitly;
compatibility claims compare matching association contexts, not an unseeded host
MIME database. Exact-case bindings can retain an earlier value while lowercase
fallbacks update. Inverse associations remain historical rather than deleting a
previous type's extension on reassignment.

Charset callbacks preserve arbitrary typed creation/read errors. Converted readers
can return bytes with EOF or failure in the same observation, or an empty
nonterminal result. An optional complete-copy capability models readers with a
whole-copy path in addition to pull reads. Both capabilities are explicit, and
returned callback buffers are copied before retention or exposure. The decoder
uses 32768-byte generic read requests, never closes returned sources, and discards
all returned header output on conversion/read failure. Complete header input is
snapshotted before any callback can mutate its caller-owned byte input.

The supplemental writer conveniences encode/decode a complete value before
output, use the separately gated shared stream drain helper, and retain the
advanced sink on partial failure. Header conversion failure leaves the sink
untouched. Short writes, caller error identities, repeated handles, callback
read/copy observations and ownership changes are fixture concerns outside timing.

SIMD byte classes cover token/plain/Q/attribute/quoted/ASCII spans with exact
scalar fringes. Pure packed-word lowercase kernels feed existing checked little-endian word
stores, processing sixteen proved bytes per wrapper iteration. Separate endian bodies preserve physical input order in
lane masks; the packed-word transform is endian independent. Actual selected kernel
objects are linked to an independent scalar oracle, including every physical byte lane and bounded word transforms. The oracle C object disables loop/SLP vectorization. Only the
actual host target is executed; no big-endian execution is implied.

The frozen frontend/runtime/provider epochs will remain explicit. Compiler host
stacks are 64 MiB; consumer stacks are 8 MiB. Normal 32 MiB heap and poisoned-free
256 MiB heap profiles are separate, require zero leaks, and retain every command,
source hash, diagnostic and sample. Rust CLI has no `--rc-debug` emission flag;
Native emission uses it in functional profiles. No Cargo/full chain runs occur
in the lane, and parent owns shared inventory, docs/site and provider integration.

The independent committed corpus currently contains 551 media observations,
60 formats, 816 encoder observations (including every selector byte), 1,538
word/header/callback observations, 18 ordered registry steps and 136 complete
codec-to-sink observations. Both admitted frozen pairs cover every observation. Supplemental
controls cover buffer ownership, legal buffer reuse, advanced writer handles,
zero reads, oversized converted results and optional whole-copy behavior.

The paired harness prepares seven public workloads: parse-and-format, format,
B/Q encoding, B/Q header decoding and lowercase-fallback registry lookup. Every
output byte contributes to an observable checksum; the retained input is read
only after the ending clock. SIMD and scalar source snapshots preserve the same
public operation and ownership contract. The first b004/c4 interleaved SIMD/scalar/reference sample set is retained;
Native and newly admitted frontend benchmark scope remains pending.

Registry performance has an explicit context limit: the Cinch benchmark owns
one custom association in an empty explicit context, while the public reference
retains its ambient host database plus the same association. Queries and returned
bytes agree, but total index populations differ. Those rows describe these two
public pipelines, not equal-population index complexity. Cinch returns fresh
mutable byte outputs; the reference uses immutable strings and can reuse an
unchanged input. Both allocation policies are recorded rather than hidden.


The first Rust b004/c4 window used exclusive CPU0 with Codegen1/JSON2
background recorded, five samples of ten complete operations per condition, and
rotated/reversed SIMD/scalar/reference order. Every checksum and leak check
passed. This is one shared-machine window; no samples or regressions were
filtered. At 4 KiB, the retained medians are:

| Public pipeline | SIMD µs | Scalar µs | Reference µs | SIMD/scalar |
| --- | ---: | ---: | ---: | ---: |
| parse | 28.97 | 229.13 | 41.29 | 0.126 |
| format | 384.24 | 432.25 | 64.97 | 0.889 |
| b-encode | 365.35 | 375.26 | 49.09 | 0.974 |
| q-encode | 730.09 | 740.75 | 93.83 | 0.986 |
| header | 493.86 | 472.21 | 33.00 | 1.046 |
| header-q | 1098.99 | 1064.45 | 68.06 | 1.032 |
| registry | 32.29 | 76.41 | 18.44 | 0.423 |

The parse/registry scans benefit from long classified spans. Most codec rows
remain substantially slower than the reference; 4 KiB B-header and Q-header
SIMD rows regress about 4.6% and 3.2% versus scalar in this window. Short-input
and escape-heavy work does not automatically benefit from wider classification.
Registry rows also retain the context-population limit described above. The
optimized host object contains packed byte arithmetic/comparisons and
`pmovmskb`; lowercase uses a pure packed-word transform followed by existing
checked byte-word stores. Actual AArch64 and wasm32 kernel objects were emitted;
only the host target has been executed.


The full functional epochs are explicit: Rust compiler
`b004049baa8e2614dd35f5484811bbf2cd24b137923a8e3bc36bf08c9c53fc93`
with runtime `c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac`,
and Native compiler `2f07707e0022b55e92bd8c2412962d06d2e4455022261c3e36dfa27e57fb231f`
with admitted V2 provider and runtime
`80bd5b546742f8374221ac445ec20b7cfd0df0958d5c39313cde077204d6e141`.
The V2 provider adds a real emitted `HashMap String String` enumeration OO
specialization. The prior provider's exact missing-variant refusal remains in
the evidence; no library rewrite, source shadow or metadata invention bypassed
it. A newer frontend is not credited with this complete corpus.

Canonical extraction covers 192 new library declarations with no missing docs,
diagnostics or orphans. The full-corpus source and final source differ only in
complete leading-comment lines: three instance implementation blocks become ordinary prose because the method
signatures belong to existing classes, while each new instance has canonical
documentation; one wrapper comment clarifies checked word stores. The exact original source, normalization patch and both
hashes are retained. Final 32-media/32-encoder/10-callback/nine-owner smoke
passes on both admitted pairs cover the normalized source.

The SDK-free Rust-only `lib_mime` target parses canonical docs and executes seven
representative committed groups through the current `CARGO_BIN_EXE_cinch`, an
ordinary compiler runtime-initialization build, the captured exact host archive,
and actual selected kernel/scalar C objects. It refuses external Native
selectors rather than silently falling back. Parent owns Cargo execution and reports both tests green (2/2, 90.53 seconds).


A separate old Native 2f077/V2/80bd baseline also passed both 168-run SIMD and
scalar preflights. Its own exclusive-CPU0 window retained all 210 raw samples
and 42 warmups, with quota counters and identical output/input/context
checksums. This is the admitted old-family baseline, not the newer Native family.
The 4 KiB medians are:

| Public pipeline | SIMD µs | Scalar µs | Reference µs | SIMD/scalar |
| --- | ---: | ---: | ---: | ---: |
| parse | 371.37 | 768.66 | 40.68 | 0.483 |
| format | 2212.43 | 2130.99 | 64.96 | 1.038 |
| b-encode | 1407.99 | 1335.47 | 48.22 | 1.054 |
| q-encode | 3048.02 | 2896.86 | 85.31 | 1.052 |
| header | 1091.25 | 1031.40 | 26.81 | 1.058 |
| header-q | 1882.24 | 1802.81 | 49.26 | 1.044 |
| registry | 102.28 | 512.56 | 17.14 | 0.200 |

Long parse and registry scans improve versus scalar, while codec rows regress
about 4–6% at 4 KiB in this window and remain much slower than the reference.
All short-input regressions, raw measurements and original control evidence
remain visible. No speed parity or newer-family adoption is claimed.

To recover disk headroom after qualification, 152 completed cold functional
ELF/LLVM clients were losslessly archived with original and compressed SHA-256
verification, freeing 717,771,989 bytes. Source, frozen compiler, provider,
runtime and all benchmark inputs remain untouched. The coverage verifier reads
exact decompressed artifacts transparently; each archive receipt records its
restore command. This does not replace an executed observation or change its
source epoch.
