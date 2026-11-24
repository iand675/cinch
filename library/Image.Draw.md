# Image.Draw

Composite rasters with Src and Over, masks and uniform colors, and dither onto palettes, with a vector Over kernel.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Image.Draw`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Image.Draw.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Wide](Data.Wide.md), [Image.Raster](Image.Raster.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Image.Draw](Image.Draw.md), [Image.Raster](Image.Raster.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Image compositing: Porter-Duff Src and Over, with optional masks and uniform sources, and Floyd-Steinberg dithering.
Drawing fills a rectangle of the destination from a source starting at a
point (and a mask starting at another), after clipping the rectangle to the
destination, source and mask. Pixels are combined in 16-bit premultiplied
RGBA: sources are read in that form and results are stored back in the
destination's format (un-premultiplied for RGBA formats, as luma for gray,
as the nearest palette entry for indexed rasters). Src between rasters of one
non-indexed format copies the stored bytes, so no precision is lost to the
premultiplied round trip, and Over of non-premultiplied 8-bit RGBA runs four
pixels per vector step. Dithering spreads each
pixel's quantization error to its neighbours (7/16 right, 3/16, 5/16 and
1/16 below). Every operation returns a fresh raster; inputs are unchanged.

</details>

# Types and constructors

<a id="type-447261774f70"></a>

## `DrawOp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">DrawOp</span></code></pre>

How source pixels combine with the destination.

[Source](/reference-sources/Image.Draw.cinch) · line 27

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DrawOver</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawOp</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DrawSrc</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawOp</span></code></pre>

<a id="type-4472617752656374"></a>

## `DrawRect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">DrawRect</span></code></pre>

Corners in the order listed above.

[Source](/reference-sources/Image.Draw.cinch) · line 36

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DrawRect</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawRect</span>))))</code></pre>

# Values

<a id="value-64726177"></a>

## `draw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">draw</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawRect</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; ((<span class="tok-constructor">Int</span>, <span class="tok-constructor">Int</span>) -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawOp</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>)))))</code></pre>

Draw the source onto the destination within the rectangle, source point sp aligned with its corner.

[Source](/reference-sources/Image.Draw.cinch) · line 50

<a id="value-647261772d666c6f79642d737465696e62657267"></a>

## `draw-floyd-steinberg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">draw-floyd-steinberg</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawRect</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; ((<span class="tok-constructor">Int</span>, <span class="tok-constructor">Int</span>) -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>))))</code></pre>

Draw a source onto an indexed raster with Floyd-Steinberg error diffusion (Src compositing). Other rasters are
returned unchanged.

[Source](/reference-sources/Image.Draw.cinch) · line 318

<a id="value-647261772d6d61736b"></a>

## `draw-mask`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">draw-mask</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawRect</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; ((<span class="tok-constructor">Int</span>, <span class="tok-constructor">Int</span>) -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; ((<span class="tok-constructor">Int</span>, <span class="tok-constructor">Int</span>) -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawOp</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>)))))))</code></pre>

Draw through a mask: the mask's alpha at point mp onward scales the source.

[Source](/reference-sources/Image.Draw.cinch) · line 54

<a id="value-647261772d72656374"></a>

## `draw-rect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">draw-rect</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawRect</span>))))</code></pre>

Make a rectangle from its corners.

[Source](/reference-sources/Image.Draw.cinch) · line 39

<a id="value-647261772d756e69666f726d"></a>

## `draw-uniform`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">draw-uniform</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawRect</span> -&gt; ((<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Draw</span>.<span class="tok-constructor">DrawOp</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>))))</code></pre>

Draw one non-premultiplied 8-bit color (red, green, blue, alpha) over the rectangle.

[Source](/reference-sources/Image.Draw.cinch) · line 58

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Image/Draw.cinch`. Source SHA-256: `815418a8361b6fec8f7809d64e582123c122c5bf70d49dcac5e2af847f2b03f6`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
