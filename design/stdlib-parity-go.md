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
| `math/bits` | [Data.Word](../library/Data.Word.md), `Data.Machine`, `Algebra.Bits` | All fixed/native-width count and reversal families, significant length, signed rotations, carry/borrow, exact wide multiply, checked and proof-bearing division/remainder | All 52 reference declarations mapped; portable 128-bit division helpers and checked-result allocations remain performance costs |
| `unicode/utf16` | [Text.Utf16](../library/Text.Utf16.md) | Scalar length, surrogate classification/pairs, sequence encode/decode, append, explicit-order byte codecs, replacement policy | All seven reference capabilities mapped and tested; byte-codec throughput and immutable-list append costs are documented |
| `encoding/binary` | [Text.Binary](../library/Text.Binary.md) | U16/U32/U64, both byte orders, offset reads, checked writes, unsigned and zigzag I64 varints, consumed byte count | Float/aggregate codecs, stream and append operations |
| `encoding/hex` | [Text.Hex](../library/Text.Hex.md), [Effect.Hex](../library/Effect.Hex.md) | Text/byte codecs, prefix and append APIs, checked caller buffers, formatted dumps, encoding writers, decoding readers and explicitly finished dump sinks | All 15 declarations mapped; immutable continuation, partial-row latency and allocation differences are documented |
| `encoding/ascii85` | [Text.Ascii85](../library/Text.Ascii85.md), [Effect.Ascii85](../library/Effect.Ascii85.md) | Text/byte codecs, checked caller buffers and source offsets, retained prefix/progress, incremental states, stream adapters, shorthand, control runs and wrapped arithmetic | All seven declarations mapped; immutable continuation, buffered-read copying and allocation costs are documented |
| `encoding/base32` | [Text.Base32](../library/Text.Base32.md) | RFC 4648 standard and extended-hex alphabets, padded and raw forms, CR/LF tolerance, final-quantum validation | Custom alphabets/padding, stream adapters |
| `hash/adler32` | [Text.Adler32](../library/Text.Adler32.md), [Effect.Adler32](../library/Effect.Adler32.md), [Text.Checksum](../library/Text.Checksum.md) | SIMD bounded-segment checksums, persistent states, numeric/byte digests, reset, copied-prefix append, checked checkpoints and stream sinks/copies | All four recorded declarations and returned hash/clone/state capabilities mapped; ownership and streaming costs are documented |
| `hash/crc32` | [Text.Checksum](../library/Text.Checksum.md) | IEEE, Castagnoli, Koopman and custom reflected polynomials, incremental update | Table acceleration, stream adapters, serialized state |
| `hash/crc64` | [Text.Checksum](../library/Text.Checksum.md) | ISO, ECMA and custom reflected polynomials, incremental update | Table acceleration, stream adapters, serialized state |
| `hash/fnv` | [Text.Fnv](../library/Text.Fnv.md), [Text.Fnv.State](../library/Text.Fnv.State.md), [Effect.Fnv](../library/Effect.Fnv.md) | All six variants, numeric and byte digests, persistent states, reset, copied-prefix append, checked checkpoints and inspectable stream sinks/copies | All six constructors and returned hash/clone/state capabilities mapped; allocation and stream-copy costs are documented |
| `io` | [Effect.Stream](../library/Effect.Stream.md) | Explicit reader/writer continuations, memory and borrowed-descriptor adapters, limited/concatenated/mirrored sources, broadcast sinks, exact/minimum/all reads, full writes, bulk and limited copies | Random access, seek/close/flush capabilities, pipes and cancellation, section/offset adapters |

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
block overreads its input. Table acceleration is still open.

`Effect.Stream` preserves advanced handles after failures and snapshots buffers
retained by memory sources and sinks. Its native consumers run with reference
count validation, poisoned freed memory, a bounded heap, and a zero-leak check.
The callback representation required constructor schemes and substitutions to
retain the callback's actual effect-region identity; cross-runtime handles are
rejected by negative tests.

The machine-readable [surface inventory](stdlib-surface.json) covers all 180
public packages in the pinned reference SDK. It currently records six functionally complete, 62 partial
and 112 unreviewed packages. ASCII85, hexadecimal, UTF-16, FNV, Adler-32 and word bit/arithmetic utilities have
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
does not repair or promote the CRC packages. The benchmark notes retain matched
before/after pipelines and controls. Runtime fixture setup also avoids a known
Rust compiler stack overflow on large constant corpus initialization; that
compiler limitation remains open.

### Using the additions

Copy the needed module beside a consumer entry:

```sh
mkdir -p /tmp/cinch-binary-example
cp compiler/prelude/Text/Binary.cinch /tmp/cinch-binary-example/Text.Binary.cinch
cp compiler/prelude/Data/Bytes/Word.cinch /tmp/cinch-binary-example/Data.Bytes.Word.cinch
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

[Native consumer tests](../compiler/tests/lib_go_stdlib.rs) compile and run
real imports against [Go-generated fixtures](../compiler/tests/fixtures/go-stdlib-vectors.json).
The corpus includes RFC 4648 examples, all 256 byte values, a deterministic
257-byte buffer, integer width truncations, and signed/unsigned 64-bit limits.
[ASCII85 tests](../compiler/tests/lib_ascii85.rs) additionally cover every
two-chunk split, empty feeds, malformed input after complete words, streaming
short writes, sticky failures and mutable-alias isolation. They inspect the
emitted SIMD and compile its kernels for AArch64 and big-endian PowerPC; those
cross-target checks do not claim execution on that hardware.
[Hexadecimal buffer tests](../compiler/tests/lib_hex.rs) verify prefix progress,
all byte values and lane positions, overlap, short destinations and exact dump
columns. [Stream tests](../compiler/tests/lib_hex_stream.rs) exercise every source
split, one-byte reads, short/failing sinks, mutable aliases and dump finalization.
Ordinary bounded-memory runs are separate from debug poisoning runs: the debug
runtime deliberately quarantines freed blocks, so its heap peak measures retained
diagnostic storage as well as live values.
[UTF-16 tests](../compiler/tests/lib_utf16.rs) compare all scalar/pair APIs
against independent fixtures, exhaust all 65,536 individual code units, and
check both byte orders, odd byte counts, marks and SWAR block tails.
[Word-operation tests](../compiler/tests/lib_word_ops.rs) compare fixed/native
widths against independent edge and random fixtures, exhaust eight-bit reversal,
and verify carry/borrow, signed rotations, division faults and proof preconditions.
Cross compilation covers 32-bit Wasm, ARM and big-endian PowerPC kernels.
Separate checks cover offsets, truncation, failed writes without mutation,
malformed encodings, and updates at every split of a check string, including
empty chunks. The fixture generator calls Go's actual standard packages.
Go is needed only for fixture regeneration:

```sh
go run tooling/go_stdlib_vectors.go > compiler/tests/fixtures/go-stdlib-vectors.json
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
| `encoding/xml`, `encoding/pem` | None | PEM first; XML with explicit resource limits later |
| `compress/zlib`, `compress/flate` | `Text.Zlib` | Existing DEFLATE decoder within zlib and stored-block encoder; raw wrapper and actual compression remain |
| `compress/gzip`, `compress/bzip2`, `archive/tar`, `archive/zip` | None | Gzip and tar first; zip and bzip2 later |
| `crypto/sha256`, `crypto/sha512`, `crypto/sha1`, `crypto/md5`, `crypto/hmac` | `Text.Digest` | Existing whole-buffer digests and HMAC-SHA256; streaming and other HMAC variants remain |
| `crypto/rand`, `math/rand` | `Effect.Random` | Existing OS entropy and bounded draws; deterministic PRNG state remains |
| Ciphers, signatures, X.509, TLS | None | Vetted backend integration, key/certificate types, AEAD, signing and TLS |
| `io`, `bufio`, `io/fs` | `Effect.Stream`, `Effect.Io`, `Effect.Fs`, `Data.StringIo` | Shared byte protocols and bounded bulk transfers now exist; buffered readers/writers, scanners, random access and lifecycle capabilities remain |
| `os`, `os/exec`, `os/signal` | `Effect.Process`, `Effect.Fs`, `Effect.Signal` | Portability, environment and process lifecycle audit |
| `path`, `path/filepath`, walking | `Data.Path`, `Effect.Find`, `Effect.FileUtils` | Existing POSIX lexical paths and walking; globbing and portability remain; Go path edge cases differ |
| `time` | `Data.Date`, `Data.Time`, `Effect.Clock` | Layout parsing/formatting, timezone database and timers |
| `context`, `sync`, `sync/atomic` | `Effect.Concurrent`, `Effect.Stm` | Existing structured concurrency and STM; cancellation/deadline propagation and synchronization audit remain |
| `net`, `net/netip`, `net/url`, DNS | `Effect.Net`, `Data.IpAddr`, `Text.Uri`, `Effect.Resolv` | Address/prefix, escaping and timeout coverage |
| `net/http`, MIME, multipart | `Effect.Http` | Existing plaintext HTTP; message/body framing, MIME, multipart, server lifecycle and TLS remain |
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
4. **Services**: cancellation/deadlines, HTTP framing and serving, MIME,
   multipart, TLS and certificate handling through vetted implementations.
5. **Breadth**: timezone data, templates, XML, zip, numerical functions and
   specialist collections.

Each increment should ship working source, checked reference pages, consumer
tests, independent compatibility vectors where applicable, and explicit
format/API limits. A package-name placeholder does not count as coverage.
