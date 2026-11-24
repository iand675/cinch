# Image.Png

PNG decoding of every color type, depth and interlace method with SIMD scanline reconstruction, and PNG encoding.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Image.Png`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Image.Png.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Image.Raster](Image.Raster.md), [Text.Checksum](Text.Checksum.md), [Text.Zlib](Text.Zlib.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Image.Png](Image.Png.md), [Image.Raster](Image.Raster.md), [Text.Adler32](Text.Adler32.md), [Text.Binary](Text.Binary.md), [Text.Checksum](Text.Checksum.md), [Text.Crc64](Text.Crc64.md), [Text.Flate](Text.Flate.md), [Text.Zlib](Text.Zlib.md).

<details><summary>Behaviour and boundary notes from the source</summary>

PNG images: decode every color type, bit depth and interlace method into rasters, and encode rasters.
Decoding verifies the signature and every chunk CRC, joins the IDAT stream
and inflates it, then reverses each scanline's filter in place: Up adds the
prior row sixteen bytes at a time in SIMD lanes, and Sub reconstructs sixteen
bytes per step with a lane-shift prefix scan specialized to the pixel size;
Average and Paeth depend on the byte to their left and run bytewise with a
branchless Paeth predictor. Samples below eight bits are widened (gray scaled
to the full range, palette indices kept), a gray or RGB transparency key
adds an alpha channel, and Adam7 passes are scattered into place. Encoding
writes a non-interlaced image, choosing each row's filter by the smallest sum
of absolute signed residuals.

</details>

# Types and constructors

<a id="type-506e674572726f72"></a>

## `PngError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PngError</span></code></pre>

A malformed or unsupported PNG file.

[Source](/reference-sources/Image.Png.cinch) · line 31

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PngBadSignature</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Png</span>.<span class="tok-constructor">PngError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PngBadChunk</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Png</span>.<span class="tok-constructor">PngError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PngBadCrc</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Png</span>.<span class="tok-constructor">PngError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PngBadHeader</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Png</span>.<span class="tok-constructor">PngError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PngMissingData</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Png</span>.<span class="tok-constructor">PngError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PngBadData</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Png</span>.<span class="tok-constructor">PngError</span></code></pre>

# Values

<a id="value-706e672d636f6e666967"></a>

## `png-config`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">png-config</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Png</span>.<span class="tok-constructor">PngError</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>, <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span>)))</code></pre>

Read only the header: width, height and the raster format decoding would produce, without inflating
image data. Transparency is taken into account when a tRNS chunk precedes the first IDAT.

[Source](/reference-sources/Image.Png.cinch) · line 103

<a id="value-706e672d6465636f6465"></a>

## `png-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">png-decode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Png</span>.<span class="tok-constructor">PngError</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>))</code></pre>

Decode a complete PNG file.

[Source](/reference-sources/Image.Png.cinch) · line 91

<a id="value-706e672d656e636f6465"></a>

## `png-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">png-encode</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a raster as a non-interlaced PNG.

[Source](/reference-sources/Image.Png.cinch) · line 461

<a id="value-706e672d6572726f722d6d657373616765"></a>

## `png-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">png-error-message</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Png</span>.<span class="tok-constructor">PngError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a PNG error.

[Source](/reference-sources/Image.Png.cinch) · line 46

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Image/Png.cinch`. Source SHA-256: `4365df5473eb069b7c08d3151d5603c6e5475295e52f25e140c1a41d50af4e1b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
