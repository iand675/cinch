# Standard library coverage against Go

Go's standard library is a coverage target for Cinch: programmers should be
able to build command-line tools, services, and data-processing programs from
maintained libraries shipped with the compiler. Cinch keeps its module names,
explicit effects, bounds, overflow policies, and `Result` errors. Matching a
Go wire format does not require copying its object model or panic behavior.

The coverage inventory is checked with the Rust compiler. Targeted ASCII85,
UTF-16, stream, word utilities, word/UTF-8 and constructor-identity suites also run with the updated
self-built compiler. An existing module does
not imply full parity with the corresponding Go package. Planned work below
is not an available API.

## Implemented additions

| Go package | Cinch module | Implemented surface | Remaining scope |
| --- | --- | --- | --- |
| `compress/gzip` | [Text.Gzip](../library/Text.Gzip.md), [Text.Zlib](../library/Text.Zlib.md) | Whole-buffer member and concatenated decoding, checked headers and trailers, stored or run-compressed writing, raw DEFLATE with consumed offsets | Streaming, header metadata, compression levels, second-frontend validation and benchmarks |
| `image/color`, `image/color/palette` | [Image.Color](../library/Image.Color.md), [Image.Color.Palette](../library/Image.Color.Palette.md) | Standard color representations, conversion models, nearest-color matching, Plan 9 and WebSafe palette factories | Current self-built qualification and fixed-palette pipeline timings |
| `math/bits` | [Data.Word](../library/Data.Word.md), `Data.Machine`, `Algebra.Bits` | All fixed/native-width count and reversal families, significant length, signed rotations, carry/borrow, exact wide multiply, checked and proof-bearing division/remainder | All 52 reference declarations mapped; portable 128-bit division helpers and checked-result allocations remain performance costs |
| `unicode/utf16` | [Text.Utf16](../library/Text.Utf16.md) | Scalar length, surrogate classification/pairs, sequence encode/decode, append, explicit-order byte codecs, replacement policy | All seven reference capabilities mapped and tested; byte-codec throughput and immutable-list append costs are documented |
| `encoding/binary` | [Text.Binary](../library/Text.Binary.md), [Text.Binary.Layout](../library/Text.Binary.Layout.md), [Effect.Binary](../library/Effect.Binary.md) | Concrete scalar/float/complex layouts, explicit/native byte order, fixed arrays/products/records/bytes/padding, atomic offset/append operations, exact varints and progress-preserving streams | All 36 recorded declarations mapped; generic callback, counter and checked-store costs are documented |
| `encoding/csv` | [Text.Csv.Record](../library/Text.Csv.Record.md), [Effect.Csv](../library/Effect.Csv.md), [Text.Csv](../library/Text.Csv.md) | Configurable text/lossless-byte records, source positions, partial errors, reconfiguration and bounded stream adapters | All 34 recorded declarations mapped; unoptimized bounded producer fixed, two-frontends and parent 11 tests pass, and small writer overhead/rejected compiler experiments are retained |
| `encoding/hex` | [Text.Hex](../library/Text.Hex.md), [Effect.Hex](../library/Effect.Hex.md) | Text/byte codecs, prefix and append APIs, checked caller buffers, formatted dumps, encoding writers, decoding readers and explicitly finished dump sinks | All 15 declarations mapped; immutable continuation, partial-row latency and allocation differences are documented |
| `encoding/ascii85` | [Text.Ascii85](../library/Text.Ascii85.md), [Effect.Ascii85](../library/Effect.Ascii85.md) | Text/byte codecs, checked caller buffers and source offsets, retained prefix/progress, incremental states, stream adapters, shorthand, control runs and wrapped arithmetic | All seven declarations mapped; immutable continuation, buffered-read copying and allocation costs are documented |
| `encoding/base32` | [Text.Base32](../library/Text.Base32.md), [Effect.Base32](../library/Effect.Base32.md) | Fixed/custom byte alphabets, optional byte padding, exact lengths, prefix/append/caller-buffer operations, persistent states and finished writers/bounded readers | All 21 recorded declarations mapped; strict malformed-input, original-byte error and optional UTF-8 text policies are explicit |
| `encoding/base64` | [Text.Base64.Encoding](../library/Text.Base64.Encoding.md), [Effect.Base64](../library/Effect.Base64.md), [Text.Base64](../library/Text.Base64.md) | Fixed/custom byte alphabets, raw/padded and strict trailing-bit modes, exact lengths, prefix/append/caller-buffer operations, persistent states and finished writers/bounded readers | All 24 recorded declarations have idiomatic mappings; arbitrary-byte text conversion, error offsets and copied-buffer costs are explicit |
| `html` | [Text.Html](../library/Text.Html.md) | Text/byte escaping, exact escaped sizing, complete named/numeric HTML5 character references and malformed-reference handling | Both recorded declarations mapped; two expanding HTML5 names intentionally extend the reference table |
| `path` | [Text.Path](../library/Text.Path.md), [Data.Path](../library/Data.Path.md) | Portable slash-path lexical operations, complete checked shell patterns, reusable pattern values and precise pattern errors | All nine recorded declarations mapped; UTF-8 text carrier and bytewise star/Unicode-class behavior are explicit |
| `net/url` | [Text.Url](../library/Text.Url.md) and its component modules, [Text.Uri](../library/Text.Uri.md) | Lossless immutable URL fields, checked parsing/rendering, ordered query multimaps, credentials/errors, binary operations, reference resolution and joining | All 62 recorded declarations mapped; optional UTF-8 views, copied ownership, query limits and remaining percent-decoding costs are explicit |
| `crypto/hmac` | [Crypto.Hmac](../library/Crypto.Hmac.md) | Reusable SHA-256/SHA-512 keys, persistent messages, full tags and verification | Generic digest construction, bounded-memory updates, second frontend and benchmarks |
| `crypto/hkdf`, `crypto/pbkdf2` | [Crypto.Kdf](../library/Crypto.Kdf.md) | Extract, expand, combined HKDF and iterated PBKDF2 with HMAC-SHA-256 or a checked fixed-width keyed callback | Second frontend and benchmarks |
| `crypto/sha256` | [Text.Sha256](../library/Text.Sha256.md), [Effect.Sha256](../library/Effect.Sha256.md) | SHA-224/SHA-256 whole-buffer digests, persistent states, reset, prefix-preserving append, checked binary checkpoints and stream sinks/copies | All ten recorded declarations and returned hash/clone/state capabilities mapped; portable SIMD schedule, copied buffers and hardware-acceleration limits are documented |
| `crypto/sha512` | [Text.Sha512](../library/Text.Sha512.md), [Effect.Sha512](../library/Effect.Sha512.md) | SHA-384, SHA-512, SHA-512/224 and SHA-512/256 whole-buffer digests, persistent states, 204-byte checkpoints and stream sinks/copies | All 18 declarations mapped; two-lane SIMD schedule; hardware SHA-512 instructions and self-built qualification remain |
| `crypto/sha1`, `crypto/md5` | [Text.Sha1](../library/Text.Sha1.md), [Text.Md5](../library/Text.Md5.md), [Effect.Sha1](../library/Effect.Sha1.md), [Effect.Md5](../library/Effect.Md5.md) | Legacy digests with persistent states, checkpoints and stream sinks/copies; four-lane SIMD SHA-1 schedule | All declarations mapped; both are documented as broken for collision resistance |
| `crypto/sha3` | [Text.Sha3](../library/Text.Sha3.md), [Effect.Sha3](../library/Effect.Sha3.md) | SHA3-224/256/384/512, SHAKE128/256 and cSHAKE128/256, persistent sponges, incremental output readers, 573-byte checkpoints and stream adapters | All capabilities mapped; register-resident generated Keccak permutation; self-built qualification remains |
| `crypto/aes`, `crypto/cipher` | [Crypto.Aes](../library/Crypto.Aes.md), [Crypto.Cipher](../library/Crypto.Cipher.md) | AES-128/192/256 with AES-NI/ARMv8 dispatch and a constant-time table-free SIMD fallback; CTR, OFB, CFB, CBC and GCM with PCLMULQDQ or constant-time GHASH | Random-nonce GCM, reader/writer adapters and machine-word block loops remain |
| `text/tabwriter` | [Text.TabWriter](../library/Text.TabWriter.md) | Elastic tabstop alignment with every formatting flag, incremental writes and flushes | Output is returned to the caller; a sink adapter is optional |
| `compress/lzw` | [Text.Lzw](../library/Text.Lzw.md) | Whole-buffer LZW in both bit orders and literal widths 2-8, byte-identical to reference encodings, plus decompression bounded by an output limit that reports how the stream ended | Incremental reader/writer adapters remain |
| `archive/tar` | [Text.Tar](../library/Text.Tar.md) | Whole-archive reading of V7, USTAR, PAX and GNU formats; USTAR/PAX writing byte-identical to the reference writer | Streaming, sparse files, filesystem helpers and sub-second times remain |
| `crypto/ed25519`, `crypto/ecdh` | [Crypto.Ed25519](../library/Crypto.Ed25519.md), [Crypto.X25519](../library/Crypto.X25519.md), [Crypto.Field25519](../library/Crypto.Field25519.md) | Constant-time Ed25519 signing/verification and X25519 agreement over 51-bit limbs with 128-bit column products | Ed25519ph/ctx remain; P-256 agreement lives in Crypto.P256 and key generation in Effect.Crypto |
| `net/textproto` | [Text.TextProto](../library/Text.TextProto.md) | SWAR canonical header keys, ordered MIME headers, continuation lines, status responses and state-machine dot encoding | Connection-level helpers over Effect.Net remain |
| `archive/zip` | [Text.Zip](../library/Text.Zip.md) | Whole-archive reading (stored, DEFLATE, Zip64, extended timestamps) with CRC verification; writing byte-identical to the reference for stored archives | Streaming, pluggable methods and LZ77 compression remain |
| `encoding/asn1` | [Text.Asn1](../library/Text.Asn1.md) | Explicit ASN.1 value trees with strict DER decoding and canonical encoding, tags, OIDs, big integers and times | Staged record codecs replace reflection-based struct marshaling |
| `image/png`, `image` | [Image.Png](../library/Image.Png.md), [Image.Raster](../library/Image.Raster.md) | PNG decode of every color type, depth, filter and Adam7 layout with SIMD Up/Sub reconstruction; adaptive-filter encoding; native-format rasters | Compression levels, ancillary chunks and the general image interface remain |
| `compress/bzip2` | [Text.Bzip2](../library/Text.Bzip2.md) | Whole-buffer decoding of concatenated streams with verified block and stream CRCs, table-driven Huffman decoding, carry-only counters and branch-free word-at-a-time CRCs; checked against libbzip2 encodings | An incremental reader over Effect.Stream remains; throughput (about 2.5 MB/s) is bound by per-call byte-store costs in generated code |
| `net/mail` | [Text.Mail](../library/Text.Mail.md) | RFC 5322 address lists and single addresses (groups, comments, quoted strings, IP domain literals, UTF-8 and encoded-word display names) from text or raw bytes, address formatting with quoting and encoded words, lenient header blocks with body, and dates with obsolete forms; SIMD atext/qtext scanning | A caller-supplied charset converter for encoded words and an incremental body reader remain; obsolete zone names follow RFC 5322 offsets |
| `crypto/des` | [Crypto.Des](../library/Crypto.Des.md) | DES and Triple DES (EDE, three keys) with single-block, ECB, CBC and CTR operations; one generated straight-line kernel per block with permutation networks, rotation windows and SP tables; checked against the cryptography library | Table lookups are not constant time (legacy interoperation only); the 64-bit modes live beside the cipher rather than in Crypto.Cipher's 128-bit modes |
| `crypto/rc4` | [Crypto.Rc4](../library/Crypto.Rc4.md) | Keying for 1 to 256 bytes and keystream XOR as an explicit, persistent stream state that replays when retained | None for the reference surface; the cipher is broken and offered for legacy data only |
| `image/gif` | [Image.Gif](../library/Image.Gif.md) | GIF87a/GIF89a decoding of every frame or the first, screen configuration, palettes with transparency, delays, disposal methods, loop counts and interlaced frames; encoding of indexed animations byte-identical to the reference encoder; SIMD palette-index validation | Encoding non-indexed rasters needs a quantizer and ditherer, which remain |
| `encoding/xml` | [Text.Xml](../library/Text.Xml.md) | Strict token decoding as an explicit state machine (namespace translation, matched end tags, CDATA, entities, line normalization, UTF-8 and character-range checks, reference error messages and lines), raw tokens, an optional depth limit, a token encoder with indentation and generated attribute prefixes, escaping, and element trees; SIMD text and escape scanning | Reflection-style Marshal/Unmarshal has no counterpart; non-strict mode (AutoClose, HTML entities), custom entity maps and charset conversion remain |
| `crypto/mlkem` | [Crypto.MlKem](../library/Crypto.MlKem.md), [Effect.Crypto](../library/Effect.Crypto.md) | ML-KEM-768 and ML-KEM-1024: seed-derived decapsulation keys, checked encapsulation keys, encapsulation with explicit or system randomness, and decapsulation with constant-time implicit rejection; polynomials are immutable 128-bit lanes driven by generated branch-free `<8 x i32>` transform, product and compression kernels; checked against a definition-level reference confirmed by the reference SDK | None for the reference surface |
| `text/template` | [Text.Template](../library/Text.Template.md) | Template sets over a dynamic value (nil, bool, int, float, string, list and map spines, from JSON): the full action language (fields, variables and scoping, pipelines, if/else-if, range with break/continue over lists, maps and integers, with/else-with, define, block, template calls, comments, trim markers, custom delimiters, missing-key modes), every builtin, printf with exactly rounded floats, escapers and user functions; SIMD delimiter search and HTML escaping | Values are dynamic rather than reflected structs and methods (`call` has no function values); parse-tree access (text/template/parse) and exact error message wording remain |
| `image/draw` | [Image.Draw](../library/Image.Draw.md) | Src and Over with optional masks and uniform sources over clipped rectangles for every raster format (premultiplied 16-bit combination, per-format stores, nearest-entry indexed stores), byte copies for same-format Src, a four-pixel `<16 x i32>` Over kernel for 8-bit RGBA, and Floyd-Steinberg dithering; checked pixel for pixel against the reference | The Drawer and Quantizer interfaces have no counterpart beyond the Floyd-Steinberg drawer; formats without a reference type (RGB, gray with alpha) follow documented conversions |
| `crypto/ecdsa`, `crypto/elliptic`, `crypto/ecdh` (P-256) | [Crypto.P256](../library/Crypto.P256.md), [Crypto.Ecdsa](../library/Crypto.Ecdsa.md), [Effect.Crypto](../library/Effect.Crypto.md) | Constant-time P-256 keys, SEC 1 compressed/uncompressed encodings, ECDH, scalar/point arithmetic, deterministic RFC 6979 and hedged ECDSA in raw r‖s and strict DER form, and system-random key generation and signing; field and scalar residues are two-`U128` Montgomery values driven by generated branch-free CIOS kernels, points use complete formulas and a masked fixed-window ladder; checked against an independent library | P-224, P-384, P-521 and the deprecated big-integer `Curve` arithmetic remain |
| `image/jpeg` | [Image.Jpeg](../library/Image.Jpeg.md), [Image.Raster](../library/Image.Raster.md) | Baseline, extended and progressive decoding (restart intervals, non-interleaved scans, 4:4:4 to 4:1:0 chroma, gray, RGB, Adobe CMYK and YCCK) to coded-colour planes or rasters, configuration reading, and baseline encoding of every raster format and of YCbCr planes byte-identical to the reference; the integer IDCT and FDCT with quantization, Huffman and AC run decoding, zig-zag scanning, segment scanning and colour conversion run in generated vector kernels | None for the reference surface; decoding takes whole files as bytes |
| `hash/adler32` | [Text.Adler32](../library/Text.Adler32.md), [Effect.Adler32](../library/Effect.Adler32.md), [Text.Checksum](../library/Text.Checksum.md) | SIMD bounded-segment checksums, persistent states, numeric/byte digests, reset, copied-prefix append, checked checkpoints and stream sinks/copies | All four recorded declarations and returned hash/clone/state capabilities mapped; ownership and streaming costs are documented |
| `hash/crc32` | [Text.Checksum](../library/Text.Checksum.md) | IEEE, Castagnoli, Koopman and custom reflected polynomials, incremental update | Prepared tables, persistent accumulators, checked 12-byte checkpoints and stream adapters in `Text.Crc32` and `Effect.Crc32`; cross-frontend qualification and benchmarking remain |
| `hash/crc64` | [Text.Crc64](../library/Text.Crc64.md), [Effect.Crc64](../library/Effect.Crc64.md), [Text.Checksum](../library/Text.Checksum.md) | Immutable SIMD-built tables, packed slicing-by-eight updates, persistent states, numeric/byte digests, reset, prefix-preserving append, table-checked checkpoints and stream sinks/copies | All eleven recorded declarations and returned hash/clone/state capabilities mapped; prepared-table setup and stream-copy costs are documented |
| `hash/fnv` | [Text.Fnv](../library/Text.Fnv.md), [Text.Fnv.State](../library/Text.Fnv.State.md), [Effect.Fnv](../library/Effect.Fnv.md) | All six variants, numeric and byte digests, persistent states, reset, copied-prefix append, checked checkpoints and inspectable stream sinks/copies | All six constructors and returned hash/clone/state capabilities mapped; allocation and stream-copy costs are documented |
| `io` | [Effect.Stream](../library/Effect.Stream.md) | Explicit reader/writer continuations, memory and borrowed-descriptor adapters, limited/concatenated/mirrored sources, broadcast sinks, exact/minimum/all reads, full writes, bulk and limited copies | Positional readers, seekable sections and offset writers available in `Effect.Stream.Section` and `Effect.Stream.Offset`; descriptor seeking, close/flush, pipes and cancellation remain |

The format/hash modules and their shared `Data.Bytes.Word` layer are
Cinch source over existing base operations and inline LLVM/ASM, with no new
compiler intrinsics or foreign libraries. They follow the existing
[source-distributed library loading rules](../library/00-index.md).
`Text.Zlib` retains its standalone `adler32` and `crc32` exports; the new
incremental checksum API does not depend on compression.

UTF-16 uses SWAR to widen four ASCII bytes or narrow four ASCII code units;
non-ASCII scalars and surrogate pairs retain their precise scalar behavior.
ASCII85, Hex and Base32 use LLVM SIMD lanes for complete encoding/decoding quanta.
Adler-32 reduces sixteen byte lanes and their weights in parallel. Fixed
binary fields use native word assembly and LLVM byte swaps, and varints use
SWAR continuation-bit detection and payload compaction. CRCs process native
eight-byte shift registers; CRC-32C additionally dispatches to a hardware
instruction after x86 feature detection. The explicit AArch64 `ArmCrc32`
capability accelerates IEEE and Castagnoli. FNV unrolls eight ordered steps; its recurrence cannot
be reassociated into independent lanes for a single message.

`Data.Bytes.Word` proves complete read ranges before unaligned native LLVM
loads and retains the backing owner until each load completes. Writes use
checked byte access. SIMD/SWAR format kernels accept machine words; the raw
load helper stays private. Short tails retain the scalar semantics, and no
block overreads its input. CRC-64 has prepared slicing tables; CRC-32 exposes reusable 256-entry tables.

`Effect.Stream` preserves advanced handles after failures and snapshots buffers
retained by memory sources and sinks. Its native consumers run with reference
count validation, poisoned freed memory, a bounded heap, and a zero-leak check.
The callback representation required constructor schemes and substitutions to
retain the callback's actual effect-region identity; cross-runtime handles are
rejected by negative tests.

The machine-readable [surface inventory](stdlib-surface.json) covers all 180
public packages in the pinned reference SDK. It currently records 32 functionally complete, 74 partial
and 74 unreviewed packages. ASCII85, Base32, Base64, binary layouts, CSV, hexadecimal, HTML, paths, URLs, UTF-16, FNV, Adler-32, CRC-64, SHA-224/SHA-256 and word bit/arithmetic utilities have
reviewed capability mappings, independent runtime fixtures and self-built
compiler checks; completion does not assert matching allocation complexity or speed. Upstream mutable
containers are included as implementation homes, not assumed full coverage.
The [benchmark notes](../benchmarks/stdlib/README.md) describe measured public
API pipelines and their limits. The full standard library goal remains open.

Hexadecimal dump formatting uses SIMD nibble encoding and eight-lane printable
ASCII classification, plus SWAR masks to interleave four digit pairs and spaces
per store block. `Effect.Hex` snapshots retained tails, retries odd short writes,
delays decoder errors until completed output is drained, and retains sticky
failures. Its dump sink buffers up to fifteen input bytes until a full row or
`hex-dump-finish`; finishing does not close or flush the underlying sink.

Base32 discards unused low bits in the final symbol, like Go. The `hex-decode` text convenience and Base32 decoders return an error for the
entire input. Hex prefix, append and caller-buffer APIs retain completed pairs
alongside an error; short destination ranges fail before mutation. Short binary reads return `UnexpectedBinaryEnd`, and
overflowing varints return `BinaryVarintOverflow`. Fixed-width put operations
validate the complete destination range before writing any bytes.

ASCII85 now also exposes finite-destination decoding and explicit source offsets.
The returned source count identifies complete quanta for retrying a non-final
suffix. Completed output remains available alongside malformed-input errors;
whole-buffer conveniences keep their all-or-error result. Separate buffer errors
name invalid source/destination offsets and insufficient encoding capacity.
Overlapping buffers snapshot input before writing; disjoint buffers avoid that
copy. A caller can reuse a small destination and advance through one original
source with `ascii85-decode-at`. SIMD skips eight ignored control bytes at once.

ASCII85 has independent buffer and stream measurements against the reference
SDK. SIMD digit kernels lower to short, call-free arithmetic; generated buffer
loops exposed repeated runtime byte-access calls. Checked payload fast paths
remove those calls for valid buffers while preserving bounds, byte range,
read-only, wrong-kind and null faults through the original runtime fallback.
Both heap and static empty buffers preserve their data-pointer contract.
Natural-index conversions and ownership work still dominate much of the loop;
current measurements do not establish performance parity.

FNV now has explicit persistent hash states in `Text.Fnv.State`. All six
variants expose digest size, block size, reset, matching-width numeric values,
network-order digest bytes, and prefix-preserving append. Keeping a state value
branches the hash without a mutable clone operation. Binary checkpoints contain
`fnv`, a variant tag from 1 through 6, and every hash bit. Loading checks the
variant before exact length and leaves the template unchanged on errors.
`Effect.Fnv` provides inspectable sinks, a generic `ByteWriter` adapter, and
bounded reader-to-hash copying. Source errors and invalid read counts return
advanced sources and the last valid hash. Scalar sinks have no runtime resource;
the generic writer carries its own runtime identity. Independent state fixtures
and all 54 public benchmark rows cover every variant. Small-read copying,
per-chunk continuations and fresh serialization buffers remain performance costs.

Adler-32 now has its own portable SIMD home, `Text.Adler32`, with the previous
`Text.Checksum` names preserved as delegating functions. Sixteen-byte weighted
reductions accumulate in machine words and normalize after at most 5552 bytes,
replacing natural-number modular operations after every vector block. Persistent
states expose numeric/network-order digests, reset, size/block size, copied-prefix
append and checked eight-byte checkpoints. `Effect.Adler32` carries the scalar
state directly through bounded stream copies, including after source failures.
Both compilers validate this standalone API. The separate CRC assembly definitions
in `Text.Checksum` remain unsupported by the self-built frontend; this increment
does not repair or promote CRC-32. CRC-64 now has an independently validated standalone home. The benchmark notes retain matched
before/after pipelines and controls. Runtime fixture setup also avoids a known
Rust compiler stack overflow on large constant corpus initialization; that
compiler limitation remains open.

CRC-64 now has a standalone `Text.Crc64` home. `crc64-table` builds an
immutable base table and seven derived slices using pairs of SIMD polynomial
entries; read-only indexed and list views expose all 256 base values. Reusing a
table avoids its 16 KiB allocation and construction cost. Updates consume packed
eight-byte words with masked table lookups and exact tails. The private traversal
retains both backing owners around its machine-counter loop, replacing generated
natural-number tag, overflow and ownership checks inside each block. Polynomial
conveniences prepare a table at 16 KiB and above, where measured setup costs are
amortized; shorter inputs keep the direct shift-register path.

Persistent CRC-64 states expose numeric and network-order digests, reset without
changing polynomial, copied-prefix append and twenty-byte binary checkpoints.
The checkpoint contains `crc`, version 2, the ISO checksum of the base entries in
network order and the full checksum. Loading validates identity before exact
length and then table matching. `Effect.Crc64` preserves the advanced reader and
last valid checksum on failed copies. Independent vectors cover every base entry
for seven polynomials, 350 complete digest/checkpoint cases, 525 arbitrary restored
updates and 791 malformed checkpoints, including the 16 KiB selection boundary.
An optimized poisoned-free case repeatedly releases ephemeral input/table owners.
Prepared tables, fresh serialization buffers and owned stream chunks have explicit
allocation costs; equivalent results do not imply matching allocation behavior.

### Using the additions

Copy the needed module beside a consumer entry:

```sh
mkdir -p /tmp/cinch-binary-example
cp seed/prelude/Text/Binary.cinch /tmp/cinch-binary-example/Text.Binary.cinch
cp seed/prelude/Data/Bytes/Word.cinch /tmp/cinch-binary-example/Data.Bytes.Word.cinch
```

Save this as `/tmp/cinch-binary-example/Main.cinch` and run it with
`cinch run /tmp/cinch-binary-example/Main.cinch`:

```cinch
module Main where
  import Text.Binary
  import Data.Machine
  import Data.Result

  main :: Int
  main = case binary-read-u32 BigEndian (binary-u32 BigEndian 0x12345678) 0 of
    Ok n -> if eq-u32 n 0x12345678 then 0 else 1
    Err _ -> 2
```

Start incremental Adler-32 at `1`, either CRC at `0`, and FNV at the result
of hashing an empty buffer. Updates accept the finished hash of the previous
chunk, with no manual inversion. For example,
`crc32-update crc32-ieee (crc32-sum first) second` equals hashing the
concatenated bytes. CRC polynomial arguments use Go's reflected representation.

### Verification

[Native consumer tests](../seed/tests/lib_go_stdlib.rs) compile and run
real imports against [Go-generated fixtures](../seed/tests/fixtures/go-stdlib-vectors.json).
The corpus includes RFC 4648 examples, all 256 byte values, a deterministic
257-byte buffer, integer width truncations, and signed/unsigned 64-bit limits.
[ASCII85 tests](../seed/tests/lib_ascii85.rs) additionally cover every
two-chunk split, empty feeds, malformed input after complete words, streaming
short writes, sticky failures and mutable-alias isolation. They inspect the
emitted SIMD and compile its kernels for AArch64 and big-endian PowerPC; those
cross-target checks do not claim execution on that hardware.
[Hexadecimal buffer tests](../seed/tests/lib_hex.rs) verify prefix progress,
all byte values and lane positions, overlap, short destinations and exact dump
columns. [Stream tests](../seed/tests/lib_hex_stream.rs) exercise every source
split, one-byte reads, short/failing sinks, mutable aliases and dump finalization.
Ordinary bounded-memory runs are separate from debug poisoning runs: the debug
runtime deliberately quarantines freed blocks, so its heap peak measures retained
diagnostic storage as well as live values.
[UTF-16 tests](../seed/tests/lib_utf16.rs) compare all scalar/pair APIs
against independent fixtures, exhaust all 65,536 individual code units, and
check both byte orders, odd byte counts, marks and SWAR block tails.
[Word-operation tests](../seed/tests/lib_word_ops.rs) compare fixed/native
widths against independent edge and random fixtures, exhaust eight-bit reversal,
and verify carry/borrow, signed rotations, division faults and proof preconditions.
Cross compilation covers 32-bit Wasm, ARM and big-endian PowerPC kernels.
Separate checks cover offsets, truncation, failed writes without mutation,
malformed encodings, and updates at every split of a check string, including
empty chunks. The fixture generator calls Go's actual standard packages.
Go is needed only for fixture regeneration:

```sh
go run tooling/go_stdlib_vectors.go > seed/tests/fixtures/go-stdlib-vectors.json
cd compiler
cargo test --test lib_go_stdlib
```

## Existing coverage and remaining work

| Go area | Existing Cinch modules | Next work / limits |
| --- | --- | --- |
| `bytes`, `strings` | `Data.Bytes`, `Data.String`, `Text.Builder` | Search/split/replace coverage audit; streaming buffers |
| `slices`, `sort`, `cmp`, `maps` | `Data.List`, `Data.Array`, `Data.Map`, `Data.HashMap`, `Data.Mutable.Vec`, `Data.Mutable.HashTable`, `Data.Mutable.BTree`, `Data.Mutable.IndexMap`, `Relation.Ord` | Operation and complexity audit |
| `strconv`, `fmt` | `Text.Show`, `Text.Scan`, `Data.Machine` | Explicit-base, bit-width-aware number parsing and quoting |
| `unicode`, `unicode/utf8` | `Data.Char`, `Data.String` | Unicode property and case tables; current classification helpers are ASCII |
| `encoding/base64`, `encoding/csv`, `encoding/json` | `Text.Base64`, `Text.Csv`, `Text.Json` | Format/options compatibility audit and streaming APIs |
| `compress/zlib`, `compress/flate` | `Text.Zlib` | Existing DEFLATE decoder within zlib and stored-block encoder; raw wrapper and actual compression remain |
| `crypto/hmac` | `Text.Digest`, `Crypto.Hmac` | SHA-256/SHA-512 keys and states exist; generic digest construction over the standalone digest states remains |
| `crypto/rand`, `math/rand` | `Effect.Random` | Existing OS entropy and bounded draws; deterministic PRNG state remains |
| RSA, ECDSA, X.509, TLS | None | Key/certificate types and TLS; AES, Ed25519 and X25519 now exist |
| `io`, `bufio`, `io/fs` | `Effect.Stream`, `Effect.Io`, `Effect.Fs`, `Data.StringIo` | Shared byte protocols and bounded bulk transfers now exist; buffered readers/writers, scanners, random access and lifecycle capabilities remain |
| `os`, `os/exec`, `os/signal` | `Effect.Process`, `Effect.Fs`, `Effect.Signal` | Portability, environment and process lifecycle audit |
| `path`, `path/filepath`, walking | `Data.Path`, `Effect.Find`, `Effect.FileUtils` | Existing POSIX lexical paths and walking; globbing and portability remain; Go path edge cases differ |
| `time` | `Data.Date`, `Data.Time`, `Effect.Clock` | Layout parsing/formatting, timezone database and timers |
| `context`, `sync`, `sync/atomic` | `Effect.Concurrent`, `Effect.Stm` | Existing structured concurrency and STM; cancellation/deadline propagation and synchronization audit remain |
| `net`, `net/netip`, `net/url`, DNS | `Effect.Net`, `Data.IpAddr`, `Text.Uri`, `Effect.Resolv` | Address/prefix, escaping and timeout coverage |
| `mime` | `Text.Mime`, `Effect.Mime` | Complete media types, encoded words, explicit charset callbacks and extension contexts; benchmark limits are recorded |
| `net/http`, multipart | `Effect.Http` | Existing plaintext HTTP; message/body framing, multipart, server lifecycle and TLS remain |
| `flag`, `log`, `testing` | `Text.OptionParser`, `Effect.Log`, `Effect.Test`, `Effect.Benchmark`, `Effect.Coverage` | Usability and integration audit |
| `regexp`, `text/scanner` | `Text.Regex`, `Text.Lex`, `Text.Scan` | Syntax and resource-behavior audit |
| Templates and HTML | None | HTML escaping first; templates with explicit escaping policy later |
| `math`, `math/bits`, `math/big`, `math/cmplx` | `Data.Machine`, `Algebra.Bits`, `Data.Int`, `Data.Rational`, `Data.Complex` | Transcendental functions, IEEE edge cases, numerical validation |
| `container/heap`, `container/list`, `container/ring` | `Data.List`, `Data.Mutable.Heap`, `Data.Mutable.Deque` | Audit priority queue, mutable sequence and ring capabilities against the reference operations |

See the [checked library index](../library/00-index.md) for the existing
modules' actual exports and limits.

## Implementation order

1. **Wire primitives**: the additions above supply byte encodings, integer
   fields and incremental checksums.
2. **Streaming I/O**: establish readers/writers, EOF, short reads, buffering,
   bounded consumption and capability ownership. Reuse these in codecs,
   compression, archives and HTTP.
3. **Everyday tools**: number conversion, ASCII85, globbing, HTML escaping,
   deterministic random state, gzip and tar.
4. **Services**: cancellation/deadlines, HTTP framing and serving,
   multipart, TLS and certificate handling through vetted implementations.
5. **Breadth**: timezone data, templates, XML, zip, numerical functions and
   specialist collections.

Each increment should ship working source, checked reference pages, consumer
tests, independent compatibility vectors where applicable, and explicit
format/API limits. A package-name placeholder does not count as coverage.

SHA-224 and SHA-256 have a standalone source implementation in `Text.Sha256`.
Both variants share a four-word SIMD schedule and exactly sixty-four ordered
rounds per block. Private remaining-length machine counters consume exact
64-byte blocks while retaining their backing owners. Finalization copies fewer
than sixty-four pending bytes and appends the wrapping 64-bit bit count. The
108-byte checkpoint checks the variant identifier before its exact size,
accepts arbitrary chaining words and wrapping counts, and canonicalizes unused
tail storage. Caller-owned input, digest and checkpoint buffers never alias a
retained state. `Effect.Sha256` uses the immutable state as an inspectable sink
and preserves advanced readers and the last valid digest after source failures.
The existing `Text.Digest` runtime facade remains available.

Independent SHA fixtures cover 132 ordinary digest/checkpoint cases, 160
arbitrary restored states and 912 identifier/size cases. Every split through
129 bytes and selected larger padding/block boundaries is exercised. Both
compilers check normal and poisoned-free execution with zero leaks; Rust also
checks optimized temporary-buffer ownership and compiles the actual kernels
for WebAssembly, AArch64 and big-endian PowerPC. Foreign execution is not
claimed. Generated constants are derived exactly from prime roots, and the
private generated suffix has a reproducibility check. See the benchmark notes
for measured small-input allocation/rendering improvements and the remaining
gap against hardware SHA instructions; functional completion does not assert
hardware throughput parity.

Configurable Base32/Base64 APIs preserve arbitrary alphabet and padding bytes,
including non-UTF-8 encoded output. Text conversion therefore returns an option;
byte APIs retain all data. New prefix APIs return complete decoded output with a
later error, and finite destinations check capacity before mutation. Immutable
lookup tables and pending state never alias caller-owned buffers. Stream writers
finish their final quantum explicitly and return the advanced sink; readers use
bounded source chunks, private surplus cursors and sticky delayed errors. Original
byte offsets include ignored CR/LF. Base32 rejects impossible raw tail lengths
and data following padding even when a reference implementation accepts them.
The legacy fixed-text Base32 path now collects and materializes scalar output
with bounded stack usage; a large early-newline regression exercises that path.

`Text.Path` provides slash semantics independent of disk access and preserves
existing `Data.Path` behavior. Patterns are fully checked before matching;
classes and question marks use Unicode scalars while stars advance by byte,
including replacement-scalar matching after a skipped UTF-8 continuation byte.
Private SIMD scans use machine counters and retain owners around complete exact
traversals. `Text.Html` uses a reproducibly generated complete HTML5 entity table,
packed-name routing with full-key verification, and numeric replacement rules.
The two names whose replacements expand the input are supported explicitly.
Its byte APIs preserve non-UTF-8 data outside converted references. Matched
benchmarks retain unsuccessful intermediates and controls; dense conversion and
checked output-store costs remain throughput limitations.
