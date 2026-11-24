# Text.Zlib

Compression/decompression operations and named malformed/unsupported-stream results.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Zlib</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Zlib/source/">View source</a><a href="/packages/search/?q=%2BText.Zlib">Search module</a><a href="/reference-sources/Text.Zlib.cinch" download="Text.Zlib.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md).

<details class="doc-notes"><summary>Module notes</summary>

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
seed/tests/lib_zlib.rs).

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
through inflate-raw-at and deflate-raw. There is no
preset dictionary (an FDICT stream refuses as `BadHeader`), no
streaming or incremental API (inflate takes the whole stream and
returns the whole result, and so does the encoder). The optional
deflate-raw-rle encoder compresses repeated bytes with fixed Huffman
codes. The zlib wrapper has no trailing-data check (bytes
after the Adler-32 belong to no stream this module reads).

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

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ZlibError">ZlibError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZlibError</strong></code></li><li><a class="syn-name" href="#v-adler32">adler32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">adler32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-crc32">crc32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crc32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-deflate">deflate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">deflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-deflate-raw">deflate-raw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">deflate-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-deflate-raw-rle">deflate-raw-rle</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">deflate-raw-rle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-inflate">inflate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">inflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zlib/#t-ZlibError" title="Text.Zlib.ZlibError">ZlibError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-inflate-raw-at">inflate-raw-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">inflate-raw-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zlib/#t-ZlibError" title="Text.Zlib.ZlibError">ZlibError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ZlibError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZlibError</strong></code><span class="doc-sig-links"><a href="#t-ZlibError" title="Permalink">#</a><a href="/packages/stdlib/Text.Zlib/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

The named failure vocabulary (the header's four refusals). Every
one is a value, never a fault: a malformed stream is an outcome
the caller reads, and the whole result is discarded (no partial
bytes cross the boundary).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-BadHeader"><code><strong class="tok-constructor">BadHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zlib/#t-ZlibError" title="Text.Zlib.ZlibError">ZlibError</a></code></li><li id="k-BadChecksum"><code><strong class="tok-constructor">BadChecksum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zlib/#t-ZlibError" title="Text.Zlib.ZlibError">ZlibError</a></code></li><li id="k-InvalidCode"><code><strong class="tok-constructor">InvalidCode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zlib/#t-ZlibError" title="Text.Zlib.ZlibError">ZlibError</a></code></li><li id="k-TruncatedStream"><code><strong class="tok-constructor">TruncatedStream</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zlib/#t-ZlibError" title="Text.Zlib.ZlibError">ZlibError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-adler32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">adler32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-adler32" title="Permalink">#</a><a href="/packages/stdlib/Text.Zlib/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

The checksum's low 32 bits (both halves are below 65521, so the
residue is the value itself; it is the conversion's evidence).

</div>
</article>

<article class="doc-entry" id="v-crc32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crc32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-crc32" title="Permalink">#</a><a href="/packages/stdlib/Text.Zlib/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

CRC-32: the IEEE 802.3 polynomial reflected (0xEDB88320), seeded
all ones and finished inverted (the checksum gzip and PNG carry).

</div>
</article>

<article class="doc-entry" id="v-deflate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">deflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-deflate" title="Permalink">#</a><a href="/packages/stdlib/Text.Zlib/source/#L743" title="View source at line 743">Source</a></span></div>
<div class="doc-body">

The zlib stream for `src`: header 0x78 0x01 (CM 8, CINFO 7,
FLEVEL 0, FDICT clear, FCHECK 1, the pair a multiple of 31), one
stored block per 65535-byte chunk with the last one final (one
empty final block for no bytes: a stream still needs one), and the
Adler-32 of `src` big-endian. The buffer is sized exactly, so
nothing is copied twice.

</div>
</article>

<article class="doc-entry" id="v-deflate-raw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">deflate-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-deflate-raw" title="Permalink">#</a><a href="/packages/stdlib/Text.Zlib/source/#L751" title="View source at line 751">Source</a></span></div>
<div class="doc-body">

Encode raw DEFLATE using stored blocks of up to 65535 bytes each.

</div>
</article>

<article class="doc-entry" id="v-deflate-raw-rle">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">deflate-raw-rle</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-deflate-raw-rle" title="Permalink">#</a><a href="/packages/stdlib/Text.Zlib/source/#L756" title="View source at line 756">Source</a></span></div>
<div class="doc-body">

Encode a raw DEFLATE stream using fixed Huffman codes and distance-one
runs of up to 258 bytes. Choose stored blocks when they would be smaller.

</div>
</article>

<article class="doc-entry" id="v-inflate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">inflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zlib/#t-ZlibError" title="Text.Zlib.ZlibError">ZlibError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-inflate" title="Permalink">#</a><a href="/packages/stdlib/Text.Zlib/source/#L457" title="View source at line 457">Source</a></span></div>
<div class="doc-body">

The whole stream: header, blocks, trailing Adler-32.

</div>
</article>

<article class="doc-entry" id="v-inflate-raw-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">inflate-raw-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zlib/#t-ZlibError" title="Text.Zlib.ZlibError">ZlibError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-inflate-raw-at" title="Permalink">#</a><a href="/packages/stdlib/Text.Zlib/source/#L466" title="View source at line 466">Source</a></span></div>
<div class="doc-body">

Decode one raw DEFLATE stream at a byte offset. Return its bytes and the
absolute offset following the final padded block. Leave trailing bytes unread.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Zlib.cinch`](/packages/stdlib/Text.Zlib/source/). Source SHA-256: `c39f929789f55ade95b2557d1311b7adf0fa2990be85cce753021af98bcdbeb4`.
