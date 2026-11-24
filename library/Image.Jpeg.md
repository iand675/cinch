# Image.Jpeg

Baseline and progressive JPEG decoding to YCbCr, gray, RGB or CMYK planes, and baseline encoding, with vector IDCT, FDCT, Huffman and colour kernels.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Image.Jpeg</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Image.Jpeg/source/">View source</a><a href="/packages/search/?q=%2BImage.Jpeg">Search module</a><a href="/reference-sources/Image.Jpeg.cinch" download="Image.Jpeg.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Image.Raster](Image.Raster.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (37 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ChromaRatio">ChromaRatio</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ChromaRatio</strong></code></li><li><a class="syn-name" href="#t-CmykImage">CmykImage</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CmykImage</strong></code></li><li><a class="syn-name" href="#t-JpegColorModel">JpegColorModel</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JpegColorModel</strong></code></li><li><a class="syn-name" href="#t-JpegConfig">JpegConfig</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JpegConfig</strong></code></li><li><a class="syn-name" href="#t-JpegError">JpegError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JpegError</strong></code></li><li><a class="syn-name" href="#t-JpegImage">JpegImage</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JpegImage</strong></code></li><li><a class="syn-name" href="#t-YcbcrImage">YcbcrImage</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">YcbcrImage</strong></code></li><li><a class="syn-name" href="#v-chroma-height">chroma-height</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chroma-height</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-chroma-width">chroma-width</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chroma-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-cmyk-height">cmyk-height</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmyk-height</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-cmyk-image">cmyk-image</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmyk-image</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a></code></li><li><a class="syn-name" href="#v-cmyk-pixels">cmyk-pixels</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmyk-pixels</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cmyk-to-raster">cmyk-to-raster</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmyk-to-raster</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-cmyk-width">cmyk-width</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cmyk-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-jpeg-config-height">jpeg-config-height</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jpeg-config-height</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegConfig" title="Image.Jpeg.JpegConfig">JpegConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-jpeg-config-model">jpeg-config-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jpeg-config-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegConfig" title="Image.Jpeg.JpegConfig">JpegConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegColorModel" title="Image.Jpeg.JpegColorModel">JpegColorModel</a></code></li><li><a class="syn-name" href="#v-jpeg-config-width">jpeg-config-width</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jpeg-config-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegConfig" title="Image.Jpeg.JpegConfig">JpegConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-jpeg-decode">jpeg-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jpeg-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegImage" title="Image.Jpeg.JpegImage">JpegImage</a></code></li><li><a class="syn-name" href="#v-jpeg-decode-config">jpeg-decode-config</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jpeg-decode-config</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegConfig" title="Image.Jpeg.JpegConfig">JpegConfig</a></code></li><li><a class="syn-name" href="#v-jpeg-default-quality">jpeg-default-quality</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jpeg-default-quality</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-jpeg-encode">jpeg-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jpeg-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-jpeg-encode-ycbcr">jpeg-encode-ycbcr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jpeg-encode-ycbcr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-jpeg-error-message">jpeg-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jpeg-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-jpeg-to-raster">jpeg-to-raster</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jpeg-to-raster</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegImage" title="Image.Jpeg.JpegImage">JpegImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-ycbcr-c-offset">ycbcr-c-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-c-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ycbcr-c-stride">ycbcr-c-stride</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-c-stride</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ycbcr-cb">ycbcr-cb</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-cb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ycbcr-cr">ycbcr-cr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-cr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ycbcr-from-planes">ycbcr-from-planes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-from-planes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a></code></li><li><a class="syn-name" href="#v-ycbcr-height">ycbcr-height</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-height</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ycbcr-image">ycbcr-image</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-image</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a></code></li><li><a class="syn-name" href="#v-ycbcr-ratio">ycbcr-ratio</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-ratio</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a></code></li><li><a class="syn-name" href="#v-ycbcr-to-raster">ycbcr-to-raster</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-to-raster</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-ycbcr-width">ycbcr-width</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ycbcr-y">ycbcr-y</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-y</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ycbcr-y-offset">ycbcr-y-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-y-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ycbcr-y-stride">ycbcr-y-stride</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ycbcr-y-stride</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ChromaRatio">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ChromaRatio</strong></code><span class="doc-sig-links"><a href="#t-ChromaRatio" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

How chroma planes are subsampled against the luma plane (horizontal then vertical factors).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Chroma444"><code><strong class="tok-constructor">Chroma444</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a></code></li><li id="k-Chroma422"><code><strong class="tok-constructor">Chroma422</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a></code></li><li id="k-Chroma420"><code><strong class="tok-constructor">Chroma420</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a></code></li><li id="k-Chroma440"><code><strong class="tok-constructor">Chroma440</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a></code></li><li id="k-Chroma411"><code><strong class="tok-constructor">Chroma411</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a></code></li><li id="k-Chroma410"><code><strong class="tok-constructor">Chroma410</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-CmykImage">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CmykImage</strong></code><span class="doc-sig-links"><a href="#t-CmykImage" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L229" title="View source at line 229">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CmykImage"><code><strong class="tok-constructor">CmykImage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JpegColorModel">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JpegColorModel</strong></code><span class="doc-sig-links"><a href="#t-JpegColorModel" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L284" title="View source at line 284">Source</a></span></div>
<div class="doc-body">

The colour model a JPEG decodes to.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JpegGrayModel"><code><strong class="tok-constructor">JpegGrayModel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegColorModel" title="Image.Jpeg.JpegColorModel">JpegColorModel</a></code></li><li id="k-JpegYcbcrModel"><code><strong class="tok-constructor">JpegYcbcrModel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegColorModel" title="Image.Jpeg.JpegColorModel">JpegColorModel</a></code></li><li id="k-JpegRgbModel"><code><strong class="tok-constructor">JpegRgbModel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegColorModel" title="Image.Jpeg.JpegColorModel">JpegColorModel</a></code></li><li id="k-JpegCmykModel"><code><strong class="tok-constructor">JpegCmykModel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegColorModel" title="Image.Jpeg.JpegColorModel">JpegColorModel</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JpegConfig">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JpegConfig</strong></code><span class="doc-sig-links"><a href="#t-JpegConfig" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L297" title="View source at line 297">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JpegConfig"><code><strong class="tok-constructor">JpegConfig</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegColorModel" title="Image.Jpeg.JpegColorModel">JpegColorModel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegConfig" title="Image.Jpeg.JpegConfig">JpegConfig</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JpegError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JpegError</strong></code><span class="doc-sig-links"><a href="#t-JpegError" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Why a JPEG could not be decoded or encoded.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JpegFormatError"><code><strong class="tok-constructor">JpegFormatError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a></code></li><li id="k-JpegUnsupported"><code><strong class="tok-constructor">JpegUnsupported</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a></code></li><li id="k-JpegTruncated"><code><strong class="tok-constructor">JpegTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a></code></li><li id="k-JpegTooLarge"><code><strong class="tok-constructor">JpegTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-JpegImage">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">JpegImage</strong></code><span class="doc-sig-links"><a href="#t-JpegImage" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L265" title="View source at line 265">Source</a></span></div>
<div class="doc-body">

A decoded image in its coded colour space.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JpegGray"><code><strong class="tok-constructor">JpegGray</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegImage" title="Image.Jpeg.JpegImage">JpegImage</a></code></li><li id="k-JpegYcbcr"><code><strong class="tok-constructor">JpegYcbcr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegImage" title="Image.Jpeg.JpegImage">JpegImage</a></code></li><li id="k-JpegRgb"><code><strong class="tok-constructor">JpegRgb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegImage" title="Image.Jpeg.JpegImage">JpegImage</a></code></li><li id="k-JpegCmyk"><code><strong class="tok-constructor">JpegCmyk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegImage" title="Image.Jpeg.JpegImage">JpegImage</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-YcbcrImage">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">YcbcrImage</strong></code><span class="doc-sig-links"><a href="#t-YcbcrImage" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-YcbcrImage"><code><strong class="tok-constructor">YcbcrImage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-chroma-height">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chroma-height</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-chroma-height" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

The height of a chroma plane for a luma height (rounded up).

</div>
</article>

<article class="doc-entry" id="v-chroma-width">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chroma-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-chroma-width" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

The width of a chroma plane for a luma width (rounded up).

</div>
</article>

<article class="doc-entry" id="v-cmyk-height">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmyk-height</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-cmyk-height" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L240" title="View source at line 240">Source</a></span></div>
<div class="doc-body">

The height in pixels.

</div>
</article>

<article class="doc-entry" id="v-cmyk-image">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmyk-image</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a></code><span class="doc-sig-links"><a href="#v-cmyk-image" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

A CMYK image over existing samples; None when they are too short.

</div>
</article>

<article class="doc-entry" id="v-cmyk-pixels">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmyk-pixels</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-cmyk-pixels" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L244" title="View source at line 244">Source</a></span></div>
<div class="doc-body">

The samples, four bytes per pixel.

</div>
</article>

<article class="doc-entry" id="v-cmyk-to-raster">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmyk-to-raster</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-cmyk-to-raster" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L248" title="View source at line 248">Source</a></span></div>
<div class="doc-body">

Convert to an Rgb8 raster: each channel is (65535 - 257c) * (65535 - 257k) / 65535, high byte.

</div>
</article>

<article class="doc-entry" id="v-cmyk-width">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cmyk-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-CmykImage" title="Image.Jpeg.CmykImage">CmykImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-cmyk-width" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L236" title="View source at line 236">Source</a></span></div>
<div class="doc-body">

The width in pixels.

</div>
</article>

<article class="doc-entry" id="v-jpeg-config-height">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jpeg-config-height</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegConfig" title="Image.Jpeg.JpegConfig">JpegConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-jpeg-config-height" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L308" title="View source at line 308">Source</a></span></div>
<div class="doc-body">

The height in pixels.

</div>
</article>

<article class="doc-entry" id="v-jpeg-config-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jpeg-config-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegConfig" title="Image.Jpeg.JpegConfig">JpegConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegColorModel" title="Image.Jpeg.JpegColorModel">JpegColorModel</a></code><span class="doc-sig-links"><a href="#v-jpeg-config-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L300" title="View source at line 300">Source</a></span></div>
<div class="doc-body">

The colour model.

</div>
</article>

<article class="doc-entry" id="v-jpeg-config-width">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jpeg-config-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegConfig" title="Image.Jpeg.JpegConfig">JpegConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-jpeg-config-width" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L304" title="View source at line 304">Source</a></span></div>
<div class="doc-body">

The width in pixels.

</div>
</article>

<article class="doc-entry" id="v-jpeg-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jpeg-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegImage" title="Image.Jpeg.JpegImage">JpegImage</a></code><span class="doc-sig-links"><a href="#v-jpeg-decode" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L470" title="View source at line 470">Source</a></span></div>
<div class="doc-body">

Decode a whole image.

</div>
</article>

<article class="doc-entry" id="v-jpeg-decode-config">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jpeg-decode-config</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegConfig" title="Image.Jpeg.JpegConfig">JpegConfig</a></code><span class="doc-sig-links"><a href="#v-jpeg-decode-config" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L481" title="View source at line 481">Source</a></span></div>
<div class="doc-body">

Read the colour model and size: stops at the frame header after a JFIF marker, otherwise at the first scan.

</div>
</article>

<article class="doc-entry" id="v-jpeg-default-quality">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jpeg-default-quality</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-jpeg-default-quality" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L1718" title="View source at line 1718">Source</a></span></div>
<div class="doc-body">

The quality used when none is chosen.

</div>
</article>

<article class="doc-entry" id="v-jpeg-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jpeg-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-jpeg-encode" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L1723" title="View source at line 1723">Source</a></span></div>
<div class="doc-body">

Encode a raster at a quality (clamped to 1..100): Gray8 as one component, every other format as YCbCr
4:2:0 after premultiplying alpha.

</div>
</article>

<article class="doc-entry" id="v-jpeg-encode-ycbcr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jpeg-encode-ycbcr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-jpeg-encode-ycbcr" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L1734" title="View source at line 1734">Source</a></span></div>
<div class="doc-body">

Encode YCbCr planes at a quality as YCbCr 4:2:0; each pixel takes the chroma sample covering it, and
each 2x2 neighbourhood is averaged.

</div>
</article>

<article class="doc-entry" id="v-jpeg-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jpeg-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegError" title="Image.Jpeg.JpegError">JpegError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-jpeg-error-message" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

A readable description of an error.

</div>
</article>

<article class="doc-entry" id="v-jpeg-to-raster">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jpeg-to-raster</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-JpegImage" title="Image.Jpeg.JpegImage">JpegImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-jpeg-to-raster" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L276" title="View source at line 276">Source</a></span></div>
<div class="doc-body">

The image as a raster: Gray8 for grayscale, Rgb8 otherwise.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-c-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-c-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ycbcr-c-offset" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L170" title="View source at line 170">Source</a></span></div>
<div class="doc-body">

The chroma offset of pixel (x, y).

</div>
</article>

<article class="doc-entry" id="v-ycbcr-c-stride">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-c-stride</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ycbcr-c-stride" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L150" title="View source at line 150">Source</a></span></div>
<div class="doc-body">

Bytes between vertically adjacent chroma samples.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-cb">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-cb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ycbcr-cb" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

The blue-difference chroma plane.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-cr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-cr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ycbcr-cr" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L162" title="View source at line 162">Source</a></span></div>
<div class="doc-body">

The red-difference chroma plane.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-from-planes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-from-planes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a></code><span class="doc-sig-links"><a href="#v-ycbcr-from-planes" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

A YCbCr image over existing planes; None when a stride is narrower than its plane or a plane is too short.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-height">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-height</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ycbcr-height" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L142" title="View source at line 142">Source</a></span></div>
<div class="doc-body">

The height in pixels.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-image">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-image</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a></code><span class="doc-sig-links"><a href="#v-ycbcr-image" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L118" title="View source at line 118">Source</a></span></div>
<div class="doc-body">

A zeroed YCbCr image with tight strides.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-ratio">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-ratio</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-ChromaRatio" title="Image.Jpeg.ChromaRatio">ChromaRatio</a></code><span class="doc-sig-links"><a href="#v-ycbcr-ratio" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L134" title="View source at line 134">Source</a></span></div>
<div class="doc-body">

The chroma ratio.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-to-raster">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-to-raster</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-ycbcr-to-raster" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L175" title="View source at line 175">Source</a></span></div>
<div class="doc-body">

Convert to an Rgb8 raster with the 8-bit YCbCr to RGB conversion, taking each pixel's chroma from the
sample covering it; four pixels per kernel step.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-width">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ycbcr-width" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

The width in pixels.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-y">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-y</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ycbcr-y" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L154" title="View source at line 154">Source</a></span></div>
<div class="doc-body">

The luma plane.

</div>
</article>

<article class="doc-entry" id="v-ycbcr-y-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-y-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ycbcr-y-offset" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L166" title="View source at line 166">Source</a></span></div>
<div class="doc-body">

The luma offset of pixel (x, y).

</div>
</article>

<article class="doc-entry" id="v-ycbcr-y-stride">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ycbcr-y-stride</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Jpeg/#t-YcbcrImage" title="Image.Jpeg.YcbcrImage">YcbcrImage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ycbcr-y-stride" title="Permalink">#</a><a href="/packages/stdlib/Image.Jpeg/source/#L146" title="View source at line 146">Source</a></span></div>
<div class="doc-body">

Bytes between vertically adjacent luma samples.

</div>
</article>
