# Image.Raster

Gray, gray-alpha, RGB, RGBA and paletted rasters at eight and sixteen bits with pixel access and RGBA conversion.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Image.Raster</code></div><div class="doc-mod-bar-links"><a href="/library/source/Image.Raster/">View source</a><a href="/library/search/?q=%2BImage.Raster">Search module</a><a href="/reference-sources/Image.Raster.cinch" download="Image.Raster.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Image.Raster`: [Image.Raster](Image.Raster.md).

```sh
cp seed/prelude/Image/Raster.cinch ./Image.Raster.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Raster images: a pixel format, dimensions, a row stride and interleaved sample bytes.
Formats cover grayscale, grayscale with alpha, RGB and RGBA at eight or sixteen
bits per sample (sixteen-bit samples are big-endian, as PNG stores them) and
palette indices with an RGBA palette. Alpha is not premultiplied. Pixel access
is bounds-checked and returns non-premultiplied sixteen-bit RGBA, and
raster-to-rgba8 converts any raster to eight-bit RGBA in fresh storage.

</details>

<details class="doc-synopsis" open><summary>Synopsis (13 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-PixelFormat">PixelFormat</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PixelFormat</strong></code></li><li><a class="syn-name" href="#t-Raster">Raster</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Raster</strong></code></li><li><a class="syn-name" href="#v-pixel-bytes">pixel-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pixel-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-raster">raster</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">raster</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-raster-format">raster-format</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">raster-format</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code></li><li><a class="syn-name" href="#v-raster-height">raster-height</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">raster-height</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-raster-new">raster-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">raster-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-raster-palette">raster-palette</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">raster-palette</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-raster-pixel">raster-pixel</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">raster-pixel</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-raster-pixels">raster-pixels</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">raster-pixels</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-raster-stride">raster-stride</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">raster-stride</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-raster-to-rgba8">raster-to-rgba8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">raster-to-rgba8</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-raster-width">raster-width</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">raster-width</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-PixelFormat">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PixelFormat</strong></code><span class="doc-sig-links"><a href="#t-PixelFormat" title="Permalink">#</a><a href="/library/source/Image.Raster/#L18" title="View source at line 18">Source</a></span></div>
<div class="doc-body">

How samples are laid out in each pixel.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Gray8"><code><strong class="tok-constructor">Gray8</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code></li><li id="k-Gray16"><code><strong class="tok-constructor">Gray16</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code></li><li id="k-GrayAlpha8"><code><strong class="tok-constructor">GrayAlpha8</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code></li><li id="k-GrayAlpha16"><code><strong class="tok-constructor">GrayAlpha16</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code></li><li id="k-Rgb8"><code><strong class="tok-constructor">Rgb8</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code></li><li id="k-Rgb16"><code><strong class="tok-constructor">Rgb16</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code></li><li id="k-Rgba8"><code><strong class="tok-constructor">Rgba8</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code></li><li id="k-Rgba16"><code><strong class="tok-constructor">Rgba16</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code></li><li id="k-Indexed"><code><strong class="tok-constructor">Indexed</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Raster">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Raster</strong></code><span class="doc-sig-links"><a href="#t-Raster" title="Permalink">#</a><a href="/library/source/Image.Raster/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Private fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Raster"><code><strong class="tok-constructor">Raster</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-pixel-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pixel-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pixel-bytes" title="Permalink">#</a><a href="/library/source/Image.Raster/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Bytes per pixel for a format.

</div>
</article>

<article class="doc-entry" id="v-raster">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">raster</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-raster" title="Permalink">#</a><a href="/library/source/Image.Raster/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

A raster over existing pixel bytes with a tight stride; None when the bytes are too short.

</div>
</article>

<article class="doc-entry" id="v-raster-format">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">raster-format</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a></code><span class="doc-sig-links"><a href="#v-raster-format" title="Permalink">#</a><a href="/library/source/Image.Raster/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Pixel format.

</div>
</article>

<article class="doc-entry" id="v-raster-height">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">raster-height</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-raster-height" title="Permalink">#</a><a href="/library/source/Image.Raster/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

Height in pixels.

</div>
</article>

<article class="doc-entry" id="v-raster-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">raster-new</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-PixelFormat" title="Image.Raster.PixelFormat">PixelFormat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-raster-new" title="Permalink">#</a><a href="/library/source/Image.Raster/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

A zeroed raster of the given format and size.

</div>
</article>

<article class="doc-entry" id="v-raster-palette">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">raster-palette</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-raster-palette" title="Permalink">#</a><a href="/library/source/Image.Raster/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

The palette as RGBA quadruples (empty unless indexed).

</div>
</article>

<article class="doc-entry" id="v-raster-pixel">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">raster-pixel</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-raster-pixel" title="Permalink">#</a><a href="/library/source/Image.Raster/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

Non-premultiplied 16-bit RGBA of the pixel at (x, y); None outside the image.

</div>
</article>

<article class="doc-entry" id="v-raster-pixels">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">raster-pixels</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-raster-pixels" title="Permalink">#</a><a href="/library/source/Image.Raster/#L83" title="View source at line 83">Source</a></span></div>
<div class="doc-body">

The pixel bytes (shared; copy before mutating).

</div>
</article>

<article class="doc-entry" id="v-raster-stride">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">raster-stride</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-raster-stride" title="Permalink">#</a><a href="/library/source/Image.Raster/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Bytes from one row to the next.

</div>
</article>

<article class="doc-entry" id="v-raster-to-rgba8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">raster-to-rgba8</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-raster-to-rgba8" title="Permalink">#</a><a href="/library/source/Image.Raster/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

Convert to tightly packed 8-bit RGBA, keeping the high byte of 16-bit samples.

</div>
</article>

<article class="doc-entry" id="v-raster-width">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">raster-width</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-raster-width" title="Permalink">#</a><a href="/library/source/Image.Raster/#L71" title="View source at line 71">Source</a></span></div>
<div class="doc-body">

Width in pixels.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Image/Raster.cinch`](/library/source/Image.Raster/). Source SHA-256: `a4c4c2c6b9f63c021a1654fe120f4746e4266c05a4383b5e6c6581e05b5fbf3b`.
