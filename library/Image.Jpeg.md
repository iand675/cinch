# Image.Jpeg

Baseline and progressive JPEG decoding to YCbCr, gray, RGB or CMYK planes, and baseline encoding, with vector IDCT, FDCT, Huffman and colour kernels.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Image.Jpeg`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Image.Jpeg.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Image.Raster](Image.Raster.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Image.Jpeg](Image.Jpeg.md), [Image.Raster](Image.Raster.md).

<details><summary>Behaviour and boundary notes from the source</summary>

JPEG images: baseline, extended-sequential and progressive Huffman decoding, and baseline encoding.
Decoding yields the image in its coded colour space: one component as a Gray8 raster, YCbCr as planes
with their chroma subsampling (4:4:4, 4:2:2, 4:2:0, 4:4:0, 4:1:1 or 4:1:0), RGB (an Adobe marker with
transform 0, or components named R, G and B without a JFIF marker) as an Rgb8 raster, and four
components (Adobe CMYK or YCCK) as CMYK samples; jpeg-to-raster converts any of them to a raster.
Restart intervals, non-interleaved scans, spectral selection and successive approximation are
supported; arithmetic coding, the lossless and hierarchical processes and 12-bit samples are refused.
jpeg-decode-config reads the size and colour model without decoding pixels.

Entropy-coded segments are destuffed sixteen bytes at a time and then read by bit offset. Huffman
codes resolve through an eight-bit lookup table, with a branch-free search over all sixteen code
lengths for longer codes, in one kernel call that also extends the magnitude bits. Dequantization and
the two-pass integer inverse DCT (after the MPEG Software Simulation Group's) run in 32-bit vector
lanes with the scalar definition's wrapping arithmetic, so any input decodes to the same samples on
every target; colour conversion runs four pixels per step.

The encoder writes baseline streams with the standard Annex K tables scaled by a quality from 1 to
100 (75 by default): Gray8 rasters as one component and everything else as YCbCr 4:2:0, replicating
edge pixels into partial blocks. Pixels with alpha are premultiplied first, as sixteen-bit colour
readers do. The forward DCT follows the Independent JPEG Group's integer jfdctint; its row pass, its
column pass with rounding quantization, colour conversion and chroma averaging run in vector lanes.

</details>

# Types and constructors

<a id="type-4368726f6d61526174696f"></a>

## `ChromaRatio`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ChromaRatio</span></code></pre>

How chroma planes are subsampled against the luma plane (horizontal then vertical factors).

[Source](/reference-sources/Image.Jpeg.cinch) · line 69

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Chroma444</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Chroma422</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Chroma420</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Chroma440</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Chroma411</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Chroma410</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span></code></pre>

<a id="type-436d796b496d616765"></a>

## `CmykImage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CmykImage</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Image.Jpeg.cinch) · line 229

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CmykImage</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">CmykImage</span>)))</code></pre>

<a id="type-4a706567436f6c6f724d6f64656c"></a>

## `JpegColorModel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JpegColorModel</span></code></pre>

The colour model a JPEG decodes to.

[Source](/reference-sources/Image.Jpeg.cinch) · line 284

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegGrayModel</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegColorModel</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegYcbcrModel</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegColorModel</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegRgbModel</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegColorModel</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegCmykModel</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegColorModel</span></code></pre>

<a id="type-4a706567436f6e666967"></a>

## `JpegConfig`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JpegConfig</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Image.Jpeg.cinch) · line 297

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegConfig</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegColorModel</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegConfig</span>)))</code></pre>

<a id="type-4a7065674572726f72"></a>

## `JpegError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JpegError</span></code></pre>

Why a JPEG could not be decoded or encoded.

[Source](/reference-sources/Image.Jpeg.cinch) · line 50

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegFormatError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegUnsupported</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegTruncated</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegTooLarge</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegError</span></code></pre>

<a id="type-4a706567496d616765"></a>

## `JpegImage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">JpegImage</span></code></pre>

A decoded image in its coded colour space.

[Source](/reference-sources/Image.Jpeg.cinch) · line 265

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegGray</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegImage</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegYcbcr</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegImage</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegRgb</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegImage</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">JpegCmyk</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">CmykImage</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegImage</span>)</code></pre>

<a id="type-5963626372496d616765"></a>

## `YcbcrImage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">YcbcrImage</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Image.Jpeg.cinch) · line 115

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">YcbcrImage</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span>))))))))</code></pre>

# Values

<a id="value-6368726f6d612d686569676874"></a>

## `chroma-height`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chroma-height</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

The height of a chroma plane for a luma height (rounded up).

[Source](/reference-sources/Image.Jpeg.cinch) · line 108

<a id="value-6368726f6d612d7769647468"></a>

## `chroma-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chroma-width</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

The width of a chroma plane for a luma width (rounded up).

[Source](/reference-sources/Image.Jpeg.cinch) · line 104

<a id="value-636d796b2d686569676874"></a>

## `cmyk-height`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cmyk-height</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">CmykImage</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The height in pixels.

[Source](/reference-sources/Image.Jpeg.cinch) · line 240

<a id="value-636d796b2d696d616765"></a>

## `cmyk-image`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cmyk-image</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">CmykImage</span>))))</code></pre>

A CMYK image over existing samples; None when they are too short.

[Source](/reference-sources/Image.Jpeg.cinch) · line 232

<a id="value-636d796b2d706978656c73"></a>

## `cmyk-pixels`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cmyk-pixels</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">CmykImage</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The samples, four bytes per pixel.

[Source](/reference-sources/Image.Jpeg.cinch) · line 244

<a id="value-636d796b2d746f2d726173746572"></a>

## `cmyk-to-raster`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cmyk-to-raster</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">CmykImage</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>)</code></pre>

Convert to an Rgb8 raster: each channel is (65535 - 257c) * (65535 - 257k) / 65535, high byte.

[Source](/reference-sources/Image.Jpeg.cinch) · line 248

<a id="value-636d796b2d7769647468"></a>

## `cmyk-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cmyk-width</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">CmykImage</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The width in pixels.

[Source](/reference-sources/Image.Jpeg.cinch) · line 236

<a id="value-6a7065672d636f6e6669672d686569676874"></a>

## `jpeg-config-height`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jpeg-config-height</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegConfig</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The height in pixels.

[Source](/reference-sources/Image.Jpeg.cinch) · line 308

<a id="value-6a7065672d636f6e6669672d6d6f64656c"></a>

## `jpeg-config-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jpeg-config-model</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegConfig</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegColorModel</span>)</code></pre>

The colour model.

[Source](/reference-sources/Image.Jpeg.cinch) · line 300

<a id="value-6a7065672d636f6e6669672d7769647468"></a>

## `jpeg-config-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jpeg-config-width</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegConfig</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The width in pixels.

[Source](/reference-sources/Image.Jpeg.cinch) · line 304

<a id="value-6a7065672d6465636f6465"></a>

## `jpeg-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jpeg-decode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegError</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegImage</span>))</code></pre>

Decode a whole image.

[Source](/reference-sources/Image.Jpeg.cinch) · line 470

<a id="value-6a7065672d6465636f64652d636f6e666967"></a>

## `jpeg-decode-config`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jpeg-decode-config</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegError</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegConfig</span>))</code></pre>

Read the colour model and size: stops at the frame header after a JFIF marker, otherwise at the first scan.

[Source](/reference-sources/Image.Jpeg.cinch) · line 481

<a id="value-6a7065672d64656661756c742d7175616c697479"></a>

## `jpeg-default-quality`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jpeg-default-quality</span> :: <span class="tok-constructor">Nat</span></code></pre>

The quality used when none is chosen.

[Source](/reference-sources/Image.Jpeg.cinch) · line 1684

<a id="value-6a7065672d656e636f6465"></a>

## `jpeg-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jpeg-encode</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Encode a raster at a quality (clamped to 1..100): Gray8 as one component, every other format as YCbCr
4:2:0 after premultiplying alpha.

[Source](/reference-sources/Image.Jpeg.cinch) · line 1689

<a id="value-6a7065672d656e636f64652d7963626372"></a>

## `jpeg-encode-ycbcr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jpeg-encode-ycbcr</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Encode YCbCr planes at a quality as YCbCr 4:2:0; each pixel takes the chroma sample covering it, and
each 2x2 neighbourhood is averaged.

[Source](/reference-sources/Image.Jpeg.cinch) · line 1700

<a id="value-6a7065672d6572726f722d6d657373616765"></a>

## `jpeg-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jpeg-error-message</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

A readable description of an error.

[Source](/reference-sources/Image.Jpeg.cinch) · line 61

<a id="value-6a7065672d746f2d726173746572"></a>

## `jpeg-to-raster`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jpeg-to-raster</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">JpegImage</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>)</code></pre>

The image as a raster: Gray8 for grayscale, Rgb8 otherwise.

[Source](/reference-sources/Image.Jpeg.cinch) · line 276

<a id="value-79636263722d632d6f6666736574"></a>

## `ycbcr-c-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-c-offset</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)))</code></pre>

The chroma offset of pixel (x, y).

[Source](/reference-sources/Image.Jpeg.cinch) · line 170

<a id="value-79636263722d632d737472696465"></a>

## `ycbcr-c-stride`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-c-stride</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Bytes between vertically adjacent chroma samples.

[Source](/reference-sources/Image.Jpeg.cinch) · line 150

<a id="value-79636263722d6362"></a>

## `ycbcr-cb`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-cb</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The blue-difference chroma plane.

[Source](/reference-sources/Image.Jpeg.cinch) · line 158

<a id="value-79636263722d6372"></a>

## `ycbcr-cr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-cr</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The red-difference chroma plane.

[Source](/reference-sources/Image.Jpeg.cinch) · line 162

<a id="value-79636263722d66726f6d2d706c616e6573"></a>

## `ycbcr-from-planes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-from-planes</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span>)))))))))</code></pre>

A YCbCr image over existing planes; None when a stride is narrower than its plane or a plane is too short.

[Source](/reference-sources/Image.Jpeg.cinch) · line 125

<a id="value-79636263722d686569676874"></a>

## `ycbcr-height`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-height</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The height in pixels.

[Source](/reference-sources/Image.Jpeg.cinch) · line 142

<a id="value-79636263722d696d616765"></a>

## `ycbcr-image`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-image</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span>)))</code></pre>

A zeroed YCbCr image with tight strides.

[Source](/reference-sources/Image.Jpeg.cinch) · line 118

<a id="value-79636263722d726174696f"></a>

## `ycbcr-ratio`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-ratio</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">ChromaRatio</span>)</code></pre>

The chroma ratio.

[Source](/reference-sources/Image.Jpeg.cinch) · line 134

<a id="value-79636263722d746f2d726173746572"></a>

## `ycbcr-to-raster`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-to-raster</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>)</code></pre>

Convert to an Rgb8 raster with the 8-bit YCbCr to RGB conversion, taking each pixel's chroma from the
sample covering it; four pixels per kernel step.

[Source](/reference-sources/Image.Jpeg.cinch) · line 175

<a id="value-79636263722d7769647468"></a>

## `ycbcr-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-width</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The width in pixels.

[Source](/reference-sources/Image.Jpeg.cinch) · line 138

<a id="value-79636263722d79"></a>

## `ycbcr-y`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-y</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The luma plane.

[Source](/reference-sources/Image.Jpeg.cinch) · line 154

<a id="value-79636263722d792d6f6666736574"></a>

## `ycbcr-y-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-y-offset</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)))</code></pre>

The luma offset of pixel (x, y).

[Source](/reference-sources/Image.Jpeg.cinch) · line 166

<a id="value-79636263722d792d737472696465"></a>

## `ycbcr-y-stride`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ycbcr-y-stride</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Jpeg</span>.<span class="tok-constructor">YcbcrImage</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Bytes between vertically adjacent luma samples.

[Source](/reference-sources/Image.Jpeg.cinch) · line 146

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Image/Jpeg.cinch`. Source SHA-256: `1970f9e50cd8c62171a16aa80ceca7fe529dec234f0a90ed81e048529dc14c86`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
