# Image.Png

PNG decoding of every color type, depth and interlace method with SIMD scanline reconstruction, and PNG encoding.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Image.Png</code></div><div class="doc-mod-bar-links"><a href="/library/source/Image.Png/">View source</a><a href="/library/search/?q=%2BImage.Png">Search module</a><a href="/reference-sources/Image.Png.cinch" download="Image.Png.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Image.Raster](Image.Raster.md), [Text.Checksum](Text.Checksum.md), [Text.Zlib](Text.Zlib.md).

<details class="doc-closure"><summary>Ship-source closure (9 files)</summary>

Copy these modules beside your entry file before importing `Image.Png`: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Image.Png](Image.Png.md), [Image.Raster](Image.Raster.md), [Text.Adler32](Text.Adler32.md), [Text.Binary](Text.Binary.md), [Text.Checksum](Text.Checksum.md), [Text.Crc64](Text.Crc64.md), [Text.Zlib](Text.Zlib.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Image/Png.cinch ./Image.Png.cinch
cp seed/prelude/Image/Raster.cinch ./Image.Raster.cinch
cp seed/prelude/Text/Adler32.cinch ./Text.Adler32.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
cp seed/prelude/Text/Checksum.cinch ./Text.Checksum.cinch
cp seed/prelude/Text/Crc64.cinch ./Text.Crc64.cinch
cp seed/prelude/Text/Zlib.cinch ./Text.Zlib.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

PNG images: decode every color type, bit depth and interlace method into rasters, and encode rasters.
Decoding verifies the signature and every chunk CRC, joins the IDAT stream
and inflates it, then reverses each scanline's filter in place: Up adds the
prior row eight bytes at a time in SIMD lanes, and Sub reconstructs eight
bytes per step with a lane-shift prefix scan specialized to the pixel size;
Average and Paeth depend on the byte to their left and run bytewise with a
branchless Paeth predictor. Samples below eight bits are widened (gray scaled
to the full range, palette indices kept), a gray or RGB transparency key
adds an alpha channel, and Adam7 passes are scattered into place. Encoding
writes a non-interlaced image, choosing each row's filter by the smallest sum
of absolute signed residuals.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-PngError">PngError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PngError</strong></code></li><li><a class="syn-name" href="#v-png-config">png-config</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">png-config</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a>)</code></li><li><a class="syn-name" href="#v-png-decode">png-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">png-decode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-png-encode">png-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">png-encode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-png-error-message">png-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">png-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-PngError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PngError</strong></code><span class="doc-sig-links"><a href="#t-PngError" title="Permalink">#</a><a href="/library/source/Image.Png/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

A malformed or unsupported PNG file.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PngBadSignature"><code><strong class="tok-constructor">PngBadSignature</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a></code></li><li id="k-PngBadChunk"><code><strong class="tok-constructor">PngBadChunk</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a></code></li><li id="k-PngBadCrc"><code><strong class="tok-constructor">PngBadCrc</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a></code></li><li id="k-PngBadHeader"><code><strong class="tok-constructor">PngBadHeader</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a></code></li><li id="k-PngMissingData"><code><strong class="tok-constructor">PngMissingData</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a></code></li><li id="k-PngBadData"><code><strong class="tok-constructor">PngBadData</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-png-config">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">png-config</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a>)</code><span class="doc-sig-links"><a href="#v-png-config" title="Permalink">#</a><a href="/library/source/Image.Png/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

Read only the header: width, height and the raster format decoding would produce, without inflating
image data. Transparency is taken into account when a tRNS chunk precedes the first IDAT.

</div>
</article>

<article class="doc-entry" id="v-png-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">png-decode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-png-decode" title="Permalink">#</a><a href="/library/source/Image.Png/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

Decode a complete PNG file.

</div>
</article>

<article class="doc-entry" id="v-png-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">png-encode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-png-encode" title="Permalink">#</a><a href="/library/source/Image.Png/#L473" title="View source at line 473">Source</a></span></div>
<div class="doc-body">

Encode a raster as a non-interlaced PNG.

</div>
</article>

<article class="doc-entry" id="v-png-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">png-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Png/#t-PngError" title="Image.Png.PngError">PngError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-png-error-message" title="Permalink">#</a><a href="/library/source/Image.Png/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Render a PNG error.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Image/Png.cinch`](/library/source/Image.Png/). Source SHA-256: `8040ff6891b589157daeda5d4d55e83daf7543d91591dc202a193f662fe44060`.
