# Image.Draw

Composite rasters with Src and Over, masks and uniform colors, and dither onto palettes, with a vector Over kernel.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Image.Draw</code></div><div class="doc-mod-bar-links"><a href="/library/source/Image.Draw/">View source</a><a href="/library/search/?q=%2BImage.Draw">Search module</a><a href="/reference-sources/Image.Draw.cinch" download="Image.Draw.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Image.Raster](Image.Raster.md).

<details class="doc-closure"><summary>Ship-source closure (3 files)</summary>

Copy these modules beside your entry file before importing `Image.Draw`: [Data.Bytes.Word](Data.Bytes.Word.md), [Image.Draw](Image.Draw.md), [Image.Raster](Image.Raster.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Image/Draw.cinch ./Image.Draw.cinch
cp seed/prelude/Image/Raster.cinch ./Image.Raster.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis" open><summary>Synopsis (7 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-DrawOp">DrawOp</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DrawOp</strong></code></li><li><a class="syn-name" href="#t-DrawRect">DrawRect</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DrawRect</strong></code></li><li><a class="syn-name" href="#v-draw">draw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">draw</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawOp" title="Image.Draw.DrawOp">DrawOp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-draw-floyd-steinberg">draw-floyd-steinberg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">draw-floyd-steinberg</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-draw-mask">draw-mask</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">draw-mask</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawOp" title="Image.Draw.DrawOp">DrawOp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-draw-rect">draw-rect</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">draw-rect</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a></code></li><li><a class="syn-name" href="#v-draw-uniform">draw-uniform</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">draw-uniform</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawOp" title="Image.Draw.DrawOp">DrawOp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-DrawOp">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DrawOp</strong></code><span class="doc-sig-links"><a href="#t-DrawOp" title="Permalink">#</a><a href="/library/source/Image.Draw/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

How source pixels combine with the destination.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-DrawOver"><code><strong class="tok-constructor">DrawOver</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawOp" title="Image.Draw.DrawOp">DrawOp</a></code></li><li id="k-DrawSrc"><code><strong class="tok-constructor">DrawSrc</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawOp" title="Image.Draw.DrawOp">DrawOp</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-DrawRect">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">DrawRect</strong></code><span class="doc-sig-links"><a href="#t-DrawRect" title="Permalink">#</a><a href="/library/source/Image.Draw/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

Corners in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-DrawRect"><code><strong class="tok-constructor">DrawRect</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-draw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">draw</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawOp" title="Image.Draw.DrawOp">DrawOp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-draw" title="Permalink">#</a><a href="/library/source/Image.Draw/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

Draw the source onto the destination within the rectangle, source point sp aligned with its corner.

</div>
</article>

<article class="doc-entry" id="v-draw-floyd-steinberg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">draw-floyd-steinberg</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-draw-floyd-steinberg" title="Permalink">#</a><a href="/library/source/Image.Draw/#L324" title="View source at line 324">Source</a></span></div>
<div class="doc-body">

Draw a source onto an indexed raster with Floyd-Steinberg error diffusion (Src compositing). Other rasters are
returned unchanged.

</div>
</article>

<article class="doc-entry" id="v-draw-mask">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">draw-mask</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawOp" title="Image.Draw.DrawOp">DrawOp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-draw-mask" title="Permalink">#</a><a href="/library/source/Image.Draw/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

Draw through a mask: the mask's alpha at point mp onward scales the source.

</div>
</article>

<article class="doc-entry" id="v-draw-rect">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">draw-rect</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a></code><span class="doc-sig-links"><a href="#v-draw-rect" title="Permalink">#</a><a href="/library/source/Image.Draw/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Make a rectangle from its corners.

</div>
</article>

<article class="doc-entry" id="v-draw-uniform">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">draw-uniform</strong> :: <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawRect" title="Image.Draw.DrawRect">DrawRect</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Draw/#t-DrawOp" title="Image.Draw.DrawOp">DrawOp</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-draw-uniform" title="Permalink">#</a><a href="/library/source/Image.Draw/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

Draw one non-premultiplied 8-bit color (red, green, blue, alpha) over the rectangle.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Image/Draw.cinch`](/library/source/Image.Draw/). Source SHA-256: `bf28ed300fee4c128ae7405b125f650ee0d60dc7c7f41dd42f915a34b549e48f`.
