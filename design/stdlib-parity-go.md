# Standard library coverage against Go

Go's standard library is a coverage target for Cinch: programmers should be
able to build command-line tools, services, and data-processing programs from
maintained libraries shipped with the compiler. Cinch keeps its module names,
explicit effects, bounds, overflow policies, and `Result` errors. Matching a
Go wire format does not require copying its object model or panic behavior.

This inventory describes the current Rust compiler. An existing module does
not imply full parity with the corresponding Go package. Planned work below
is not an available API.

## Implemented additions

| Go package | Cinch module | Implemented surface | Remaining scope |
| --- | --- | --- | --- |
| `encoding/binary` | [Text.Binary](../library/Text.Binary.md) | U16/U32/U64, both byte orders, offset reads, checked writes, unsigned and zigzag I64 varints, consumed byte count | Float/aggregate codecs, stream and append operations |
| `encoding/hex` | [Text.Hex](../library/Text.Hex.md) | Lowercase encoder, either-case decoder, length helpers, invalid-byte offsets | Dumps, stream and append operations |
| `encoding/base32` | [Text.Base32](../library/Text.Base32.md) | RFC 4648 standard and extended-hex alphabets, padded and raw forms, CR/LF tolerance, final-quantum validation | Custom alphabets/padding, stream adapters |
| `hash/adler32` | [Text.Checksum](../library/Text.Checksum.md) | Whole-buffer checksum and incremental update | Stream adapters, serialized state |
| `hash/crc32` | [Text.Checksum](../library/Text.Checksum.md) | IEEE, Castagnoli, Koopman and custom reflected polynomials, incremental update | Table acceleration, stream adapters, serialized state |
| `hash/crc64` | [Text.Checksum](../library/Text.Checksum.md) | ISO, ECMA and custom reflected polynomials, incremental update | Table acceleration, stream adapters, serialized state |
| `hash/fnv` | [Text.Fnv](../library/Text.Fnv.md) | FNV-1 and FNV-1a, 32 and 64 bits, incremental update | 128-bit variants, stream adapters, serialized state |

The five format/hash modules and their shared `Data.Bytes.Word` layer are
Cinch source over existing base operations and inline LLVM/ASM, with no new
compiler intrinsics or foreign libraries. They follow the existing
[source-distributed library loading rules](../library/00-index.md).
`Text.Zlib` retains its standalone `adler32` and `crc32` exports; the new
incremental checksum API does not depend on compression.

Hex and Base32 use LLVM SIMD lanes for complete encoding/decoding quanta.
Adler-32 reduces sixteen byte lanes and their weights in parallel. Fixed
binary fields use native word assembly and LLVM byte swaps, and varints use
SWAR continuation-bit detection and payload compaction. CRCs process native
eight-byte shift registers; CRC-32C additionally dispatches to a hardware
instruction after x86 feature detection. The explicit AArch64 `ArmCrc32`
capability accelerates IEEE and Castagnoli. FNV unrolls eight ordered steps; its recurrence cannot
be reassociated into independent lanes for a single message.

`Data.Bytes.Word` retains checked byte access and buffer ownership at the
boundary; the SIMD/SWAR kernels accept machine words rather than escaping
raw pointers. Short tails retain the scalar semantics, and no block overreads
its input. No benchmark speedup or table acceleration is claimed.

Base32 discards unused low bits in the final symbol, like Go. Hex
and Base32 return an error for the entire input, rather than Go's partial
decoded prefix. Short binary reads return `UnexpectedBinaryEnd`, and
overflowing varints return `BinaryVarintOverflow`. Fixed-width put operations
validate the complete destination range before writing any bytes.

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
| `slices`, `sort`, `cmp`, `maps` | `Data.List`, `Data.Array`, `Data.Map`, `Data.HashMap`, `Relation.Ord` | Operation and complexity audit |
| `strconv`, `fmt` | `Text.Show`, `Text.Scan`, `Data.Machine` | Explicit-base, bit-width-aware number parsing and quoting |
| `unicode`, `unicode/utf8` | `Data.Char`, `Data.String` | Unicode property and case tables; current classification helpers are ASCII |
| `encoding/base64`, `encoding/csv`, `encoding/json` | `Text.Base64`, `Text.Csv`, `Text.Json` | Format/options compatibility audit and streaming APIs |
| `encoding/ascii85`, `encoding/xml`, `encoding/pem` | None | ASCII85 and PEM first; XML with explicit resource limits later |
| `compress/zlib`, `compress/flate` | `Text.Zlib` | Existing DEFLATE decoder within zlib and stored-block encoder; raw wrapper and actual compression remain |
| `compress/gzip`, `compress/bzip2`, `archive/tar`, `archive/zip` | None | Gzip and tar first; zip and bzip2 later |
| `crypto/sha256`, `crypto/sha512`, `crypto/sha1`, `crypto/md5`, `crypto/hmac` | `Text.Digest` | Existing whole-buffer digests and HMAC-SHA256; streaming and other HMAC variants remain |
| `crypto/rand`, `math/rand` | `Effect.Random` | Existing OS entropy and bounded draws; deterministic PRNG state remains |
| Ciphers, signatures, X.509, TLS | None | Vetted backend integration, key/certificate types, AEAD, signing and TLS |
| `io`, `bufio`, `io/fs` | `Effect.Io`, `Effect.Fs`, `Data.StringIo` | Shared readers/writers, EOF, short reads, buffering and bounded consumption |
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
| `container/heap`, `container/list`, `container/ring` | `Data.List` | Priority queues and deque/ring collections |

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
