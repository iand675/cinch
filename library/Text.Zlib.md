# Text.Zlib

Compression/decompression operations and named malformed/unsupported-stream results.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Zlib`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Zlib.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md).

Non-embedded source closure: [Text.Zlib](Text.Zlib.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Zlib: DEFLATE and its zlib wrapper (RFC 1951 under RFC 1950).

`inflate` is the complete decoder: stored blocks, fixed-Huffman
blocks and dynamic Huffman blocks (the code-length alphabet's
16/17/18 repeat codes included), LZ77 back-references over the full
length and distance tables, the header's CM/CINFO/FDICT/FCHECK
fields, and the trailing Adler-32. `deflate` is the other direction
only, and deliberately asymmetric: it is a STORED-BLOCK encoder
(RFC 1951 3.2.4), so what it writes is a legal zlib stream that
every decoder accepts, never a compressed one. Bit-exactness with
any particular encoder was never the shape; `inflate (deflate x) ==
x` is, and the other direction is pinned by handing the encoder's
output to a real decoder (python's `zlib`, in
compiler/tests/lib_zlib.rs).

THE DESIGN LAW SHAPE: decoding never guesses. Nothing here produces
bytes it cannot justify, and every refusal names itself:
  `BadHeader`       the two header bytes do not describe a zlib
                    stream: CM other than 8, CINFO above 7 (a
                    window past 32K), FDICT set (no dictionary
                    support here), or the CMF/FLG pair not a
                    multiple of 31 (FCHECK);
  `BadChecksum`     the trailing Adler-32 does not match the decoded
                    bytes;
  `InvalidCode`     the stream's structure is not one DEFLATE
                    defines: the reserved block type (BTYPE 3), a
                    stored block whose NLEN is not LEN's complement,
                    a length or distance code past its alphabet, a
                    distance reaching back past the output's start,
                    a repeat code overrunning the two length
                    arrays, or a Huffman code no code matches;
  `TruncatedStream` the input ends before the stream does: a read
                    past the last byte, or fewer than four bytes
                    where the Adler-32 should be.

NAMED REFUSALS at the boundary (what this module deliberately does
not do): no gzip and no raw-DEFLATE wrapper (`windowBits` picks them
at a stream API; another wrapper is another decoder, not a flag), no
preset dictionary (an FDICT stream refuses as `BadHeader`), no
streaming or incremental API (inflate takes the whole stream and
returns the whole result, and so does the encoder), no compression
on the encode side (a fixed-Huffman encoder needs match finding,
which is its own module's work), and no trailing-data check (bytes
after the Adler-32 belong to no stream this module reads).

The window claimed is CINFO 7 in both directions: the decoder
accepts the 32K maximum and the encoder declares exactly what it
writes. `crc32` sits beside the wrapper's Adler-32 because the rest
of the family wants it (gzip's container, PNG), not because zlib
reads it; both are pinned to published vectors in
compiler/tests/lib_zlib.rs.

The two spellings are the Ruby ones (`Zlib.adler32`, `Zlib.crc32`)
and, here, also the only legal ones: a digit cannot continue an
identifier across a dash (surface syntax local amendments section
8, so `x-1` is subtraction), which rules out `adler-32`. The
neighbours resolve the same collision the same way by keeping the
digits whole (`sha256-bytes` in Data.Bytes, `f64-to-decimal` in
Data.Machine); the checksum names keep them whole too.

Module scheme (design/base-library.md): `Text.*` is text processing
over String and Bytes, and a byte-stream codec is that family's;
`Bytes` and `Nat` are `Data.*`'s. Ruby item ported: Zlib
(design/stdlib-parity-ruby.md, P39), the inflate/deflate/adler32/
crc32 surface, without the stream object.

Bit order (RFC 1951 3.1.1) is the one place a decoder can be subtly
wrong, so it is stated once and relied on everywhere below: bits
are consumed in stream order, the least significant bit of each
byte first, and every numeric field (LEN, NLEN, HLIT/HDIST/HCLEN,
the extra bits of a length or a distance) is packed least
significant bit first, while a Huffman code is packed most
significant bit first. The single reader below delivers stream
order; the numeric fields accumulate little-endian (`bits-read`)
and the codes accumulate high-bit-first (`huff-walk`).

</details>

# Types and constructors

<a id="type-5a6c69624572726f72"></a>

## `ZlibError`

```cinch
data ZlibError
```

Errors.

The named failure vocabulary (the header's four refusals). Every
one is a value, never a fault: a malformed stream is an outcome
the caller reads, and the whole result is discarded (no partial
bytes cross the boundary).

[Source](/reference-sources/Text.Zlib.cinch) · line 100

```cinch
BadHeader :: ZlibError
```

```cinch
BadChecksum :: ZlibError
```

```cinch
InvalidCode :: ZlibError
```

```cinch
TruncatedStream :: ZlibError
```

# Values

<a id="value-61646c65723332"></a>

## `adler32`

```cinch
adler32 :: (Bytes -> U32)
```

The checksum's low 32 bits (both halves are below 65521, so the
residue is the value itself; it is the conversion's evidence).

[Source](/reference-sources/Text.Zlib.cinch) · line 154

<a id="value-6372633332"></a>

## `crc32`

```cinch
crc32 :: (Bytes -> U32)
```

CRC-32: the IEEE 802.3 polynomial reflected (0xEDB88320), seeded
all ones and finished inverted (the checksum gzip and PNG carry).

[Source](/reference-sources/Text.Zlib.cinch) · line 168

<a id="value-6465666c617465"></a>

## `deflate`

```cinch
deflate :: (Bytes -> Bytes)
```

Encoding. Stored blocks only (see the header): the encoder's
obligation is a legal stream that round-trips, and its output is
exactly the stream a decoder would have produced had the input
arrived as a stored block already.

The zlib stream for `src`: header 0x78 0x01 (CM 8, CINFO 7,
FLEVEL 0, FDICT clear, FCHECK 1, the pair a multiple of 31), one
stored block per 65535-byte chunk with the last one final (one
empty final block for no bytes: a stream still needs one), and the
Adler-32 of `src` big-endian. The buffer is sized exactly, so
nothing is copied twice.

[Source](/reference-sources/Text.Zlib.cinch) · line 735

<a id="value-696e666c617465"></a>

## `inflate`

```cinch
inflate :: (Bytes -> Result ZlibError Bytes)
```

The whole stream: header, blocks, trailing Adler-32.

[Source](/reference-sources/Text.Zlib.cinch) · line 456

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Zlib.cinch`. Source SHA-256: `106c7d309df7a045f78b61737fc4a0a0ff14c821b1fe9f1777284549f042c55d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
