# Image.Raster

Gray, gray-alpha, RGB, RGBA and paletted rasters at eight and sixteen bits with pixel access and RGBA conversion.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Image.Raster`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Image.Raster.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md).

Non-embedded source closure: [Image.Raster](Image.Raster.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Raster images: a pixel format, dimensions, a row stride and interleaved sample bytes.
Formats cover grayscale, grayscale with alpha, RGB and RGBA at eight or sixteen
bits per sample (sixteen-bit samples are big-endian, as PNG stores them) and
palette indices with an RGBA palette. Alpha is not premultiplied. Pixel access
is bounds-checked and returns non-premultiplied sixteen-bit RGBA, and
raster-to-rgba8 converts any raster to eight-bit RGBA in fresh storage.

</details>

# Types and constructors

<a id="type-506978656c466f726d6174"></a>

## `PixelFormat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PixelFormat</span></code></pre>

How samples are laid out in each pixel.

[Source](/reference-sources/Image.Raster.cinch) · line 18

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Gray8</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Gray16</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GrayAlpha8</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GrayAlpha16</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rgb8</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rgb16</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rgba8</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rgba16</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Indexed</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span></code></pre>

<a id="type-526173746572"></a>

## `Raster`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Raster</span></code></pre>

Private fields in the order listed above.

[Source](/reference-sources/Image.Raster.cinch) · line 41

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Raster</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>))))))</code></pre>

# Values

<a id="value-706978656c2d6279746573"></a>

## `pixel-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pixel-bytes</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Bytes per pixel for a format.

[Source](/reference-sources/Image.Raster.cinch) · line 44

<a id="value-726173746572"></a>

## `raster`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raster</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>))))))</code></pre>

A raster over existing pixel bytes with a tight stride; None when the bytes are too short.

[Source](/reference-sources/Image.Raster.cinch) · line 57

<a id="value-7261737465722d666f726d6174"></a>

## `raster-format`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raster-format</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span>)</code></pre>

Pixel format.

[Source](/reference-sources/Image.Raster.cinch) · line 67

<a id="value-7261737465722d686569676874"></a>

## `raster-height`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raster-height</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Height in pixels.

[Source](/reference-sources/Image.Raster.cinch) · line 75

<a id="value-7261737465722d6e6577"></a>

## `raster-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raster-new</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">PixelFormat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>)))</code></pre>

A zeroed raster of the given format and size.

[Source](/reference-sources/Image.Raster.cinch) · line 63

<a id="value-7261737465722d70616c65747465"></a>

## `raster-palette`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raster-palette</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The palette as RGBA quadruples (empty unless indexed).

[Source](/reference-sources/Image.Raster.cinch) · line 87

<a id="value-7261737465722d706978656c"></a>

## `raster-pixel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raster-pixel</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>)))))</code></pre>

Non-premultiplied 16-bit RGBA of the pixel at (x, y); None outside the image.

[Source](/reference-sources/Image.Raster.cinch) · line 103

<a id="value-7261737465722d706978656c73"></a>

## `raster-pixels`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raster-pixels</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The pixel bytes (shared; copy before mutating).

[Source](/reference-sources/Image.Raster.cinch) · line 83

<a id="value-7261737465722d737472696465"></a>

## `raster-stride`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raster-stride</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Bytes from one row to the next.

[Source](/reference-sources/Image.Raster.cinch) · line 79

<a id="value-7261737465722d746f2d7267626138"></a>

## `raster-to-rgba8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raster-to-rgba8</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>)</code></pre>

Convert to tightly packed 8-bit RGBA, keeping the high byte of 16-bit samples.

[Source](/reference-sources/Image.Raster.cinch) · line 127

<a id="value-7261737465722d7769647468"></a>

## `raster-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">raster-width</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Width in pixels.

[Source](/reference-sources/Image.Raster.cinch) · line 71

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Image/Raster.cinch`. Source SHA-256: `a4c4c2c6b9f63c021a1654fe120f4746e4266c05a4383b5e6c6581e05b5fbf3b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
