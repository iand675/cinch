# Text.Zlib

RFC 1950 zlib and raw DEFLATE compression (backed by Text.Flate LZ77 + dynamic Huffman) and decompression with named error results.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Zlib`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Zlib.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Text.Flate](Text.Flate.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Flate](Text.Flate.md), [Text.Zlib](Text.Zlib.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Zlib: DEFLATE and its zlib wrapper (RFC 1951 under RFC 1950).

`inflate` is the complete decoder: stored blocks, fixed-Huffman
blocks and dynamic Huffman blocks (the code-length alphabet's
16/17/18 repeat codes included), LZ77 back-references over the full
length and distance tables, the header's CM/CINFO/FDICT/FCHECK
fields, and the trailing Adler-32. `deflate` and `deflate-raw`
perform real RFC 1951 DEFLATE compression via `Text.Flate` (LZ77
matching with fixed/dynamic/stored block selection), with explicit
`deflate-level` / `deflate-raw-level` variants for choosing a
`FlateCompression` level and `deflate-stored` / `deflate-raw-stored`
preserved for uncompressed stored-block output. `inflate (deflate x)
== x` holds for every input, and interoperability is verified against
independent decoders in seed/tests/lib_zlib.rs.

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
not do): gzip framing lives with the gzip codec; raw blocks are available
through inflate-raw-at, deflate-raw, deflate-raw-level, and
deflate-raw-stored. There is no preset dictionary (an FDICT stream
refuses as `BadHeader`), no streaming or incremental API (inflate takes
the whole stream and returns the whole result, and so does the encoder).
The optional deflate-raw-rle encoder compresses repeated bytes with
fixed Huffman codes and falls back to stored blocks. The zlib wrapper
has no trailing-data check (bytes after the Adler-32 belong to no
stream this module reads).

The window claimed is CINFO 7 in both directions: the decoder
accepts the 32K maximum and the encoder declares exactly what it
writes. `crc32` sits beside the wrapper's Adler-32 because the rest
of the family wants it (gzip's container, PNG), not because zlib
reads it; both are pinned to published vectors in
seed/tests/lib_zlib.rs.

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

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ZlibError</span></code></pre>

The named failure vocabulary (the header's four refusals). Every
one is a value, never a fault: a malformed stream is an outcome
the caller reads, and the whole result is discarded (no partial
bytes cross the boundary).

[Source](/reference-sources/Text.Zlib.cinch) · line 103

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BadHeader</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zlib</span>.<span class="tok-constructor">ZlibError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">BadChecksum</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zlib</span>.<span class="tok-constructor">ZlibError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidCode</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zlib</span>.<span class="tok-constructor">ZlibError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TruncatedStream</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zlib</span>.<span class="tok-constructor">ZlibError</span></code></pre>

# Values

<a id="value-61646c65723332"></a>

## `adler32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">adler32</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

The checksum's low 32 bits (both halves are below 65521, so the
residue is the value itself; it is the conversion's evidence).

[Source](/reference-sources/Text.Zlib.cinch) · line 157

<a id="value-6372633332"></a>

## `crc32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crc32</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

CRC-32: the IEEE 802.3 polynomial reflected (0xEDB88320), seeded
all ones and finished inverted (the checksum gzip and PNG carry).

[Source](/reference-sources/Text.Zlib.cinch) · line 171

<a id="value-6465666c617465"></a>

## `deflate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deflate</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a zlib stream (RFC 1950) using default DEFLATE compression.

[Source](/reference-sources/Text.Zlib.cinch) · line 767

<a id="value-6465666c6174652d6c6576656c"></a>

## `deflate-level`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deflate-level</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode a zlib stream (RFC 1950) at the given DEFLATE compression level,
setting the header FLEVEL bits accordingly and falling back to stored
blocks if the level is invalid.

[Source](/reference-sources/Text.Zlib.cinch) · line 773

<a id="value-6465666c6174652d726177"></a>

## `deflate-raw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deflate-raw</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a raw DEFLATE stream (RFC 1951) using default compression.

[Source](/reference-sources/Text.Zlib.cinch) · line 790

<a id="value-6465666c6174652d7261772d6c6576656c"></a>

## `deflate-raw-level`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deflate-raw-level</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Flate</span>.<span class="tok-constructor">FlateCompression</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode a raw DEFLATE stream (RFC 1951) at the given compression level,
falling back to stored blocks if the level is invalid.

[Source](/reference-sources/Text.Zlib.cinch) · line 795

<a id="value-6465666c6174652d7261772d726c65"></a>

## `deflate-raw-rle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deflate-raw-rle</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a raw DEFLATE stream using fixed Huffman codes and distance-one
runs of up to 258 bytes. Choose stored blocks when they would be smaller.

[Source](/reference-sources/Text.Zlib.cinch) · line 809

<a id="value-6465666c6174652d7261772d73746f726564"></a>

## `deflate-raw-stored`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deflate-raw-stored</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a raw DEFLATE stream (RFC 1951) using stored blocks of up to
65535 bytes each.

[Source](/reference-sources/Text.Zlib.cinch) · line 804

<a id="value-6465666c6174652d73746f726564"></a>

## `deflate-stored`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">deflate-stored</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a zlib stream (RFC 1950) using uncompressed stored blocks
(header 0x78 0x01, one stored block per 65535-byte chunk, trailing Adler-32).

[Source](/reference-sources/Text.Zlib.cinch) · line 782

<a id="value-696e666c617465"></a>

## `inflate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">inflate</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zlib</span>.<span class="tok-constructor">ZlibError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

The whole stream: header, blocks, trailing Adler-32.

[Source](/reference-sources/Text.Zlib.cinch) · line 459

<a id="value-696e666c6174652d7261772d6174"></a>

## `inflate-raw-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">inflate-raw-at</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Zlib</span>.<span class="tok-constructor">ZlibError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Decode one raw DEFLATE stream at a byte offset. Return its bytes and the
absolute offset following the final padded block. Leave trailing bytes unread.

[Source](/reference-sources/Text.Zlib.cinch) · line 468

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Zlib.cinch`. Source SHA-256: `e1f1d44960a4ab98906525dea42b6fd086b091f827b23b4b5792343cc94c7ff9`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
