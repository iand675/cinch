# Image.Gif

Decode and encode GIF images and animations: palettes, transparency, delays, disposal, looping and interlace.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Image.Gif</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Image.Gif/source/">View source</a><a href="/packages/search/?q=%2BImage.Gif">Search module</a><a href="/reference-sources/Image.Gif.cinch" download="Image.Gif.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Image.Raster](Image.Raster.md), [Text.Lzw](Text.Lzw.md), [Text.Show](Text.Show.md).

<details class="doc-closure"><summary>Ship-source closure (5 files)</summary>

Copy these modules beside your entry file before importing `Image.Gif`: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Image.Gif](Image.Gif.md), [Image.Raster](Image.Raster.md), [Text.Lzw](Text.Lzw.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Image/Gif.cinch ./Image.Gif.cinch
cp seed/prelude/Image/Raster.cinch ./Image.Raster.cinch
cp seed/prelude/Text/Lzw.cinch ./Text.Lzw.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

GIF images and animations (GIF87a and GIF89a): decoding and encoding of palette-indexed frames.
A file is a logical screen (width, height, an optional global palette and a
background index) holding frames. Each frame is an indexed raster placed at
an offset, with its own or the global palette, a delay in hundredths of a
second and a disposal method. A palette's transparent entry (from the
graphic control extension) decodes with alpha zero, and encoding marks the
first entry with alpha zero as transparent. Pixel data is LZW-compressed
(Text.Lzw, least significant bit first) in sub-blocks of up to 255 bytes;
interlaced frames are reordered on decoding, and encoding writes them
progressively. Decoding validates palette indices sixteen bytes at a time.
The encoder writes GIF89a with a global palette when one is given, reusing
it for frames whose palettes match it apart from the transparent entry.

</details>

<details class="doc-synopsis"><summary>Synopsis (24 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Gif">Gif</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Gif</strong></code></li><li><a class="syn-name" href="#t-GifDisposal">GifDisposal</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GifDisposal</strong></code></li><li><a class="syn-name" href="#t-GifError">GifError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GifError</strong></code></li><li><a class="syn-name" href="#t-GifFrame">GifFrame</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GifFrame</strong></code></li><li><a class="syn-name" href="#t-GifLoop">GifLoop</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GifLoop</strong></code></li><li><a class="syn-name" href="#v-gif">gif</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifLoop" title="Image.Gif.GifLoop">GifLoop</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a></code></li><li><a class="syn-name" href="#v-gif-background-index">gif-background-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-background-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-gif-config">gif-config</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-config</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-gif-decode">gif-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-gif-decode-all">gif-decode-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-decode-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a></code></li><li><a class="syn-name" href="#v-gif-encode">gif-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gif-encode-all">gif-encode-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-encode-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gif-error-message">gif-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-gif-frame">gif-frame</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-frame</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifDisposal" title="Image.Gif.GifDisposal">GifDisposal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a></code></li><li><a class="syn-name" href="#v-gif-frame-delay">gif-frame-delay</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-frame-delay</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-gif-frame-disposal">gif-frame-disposal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-frame-disposal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifDisposal" title="Image.Gif.GifDisposal">GifDisposal</a></code></li><li><a class="syn-name" href="#v-gif-frame-left">gif-frame-left</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-frame-left</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-gif-frame-raster">gif-frame-raster</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-frame-raster</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code></li><li><a class="syn-name" href="#v-gif-frame-top">gif-frame-top</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-frame-top</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-gif-frames">gif-frames</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-frames</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a></code></li><li><a class="syn-name" href="#v-gif-height">gif-height</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-height</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-gif-loop">gif-loop</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-loop</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifLoop" title="Image.Gif.GifLoop">GifLoop</a></code></li><li><a class="syn-name" href="#v-gif-palette">gif-palette</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-palette</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gif-width">gif-width</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gif-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Gif">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Gif</strong></code><span class="doc-sig-links"><a href="#t-Gif" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Gif"><code><strong class="tok-constructor">Gif</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifLoop" title="Image.Gif.GifLoop">GifLoop</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GifDisposal">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GifDisposal</strong></code><span class="doc-sig-links"><a href="#t-GifDisposal" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

What happens to a frame's area before the next frame is drawn.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GifDisposalUnspecified"><code><strong class="tok-constructor">GifDisposalUnspecified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifDisposal" title="Image.Gif.GifDisposal">GifDisposal</a></code></li><li id="k-GifDisposalNone"><code><strong class="tok-constructor">GifDisposalNone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifDisposal" title="Image.Gif.GifDisposal">GifDisposal</a></code></li><li id="k-GifDisposalBackground"><code><strong class="tok-constructor">GifDisposalBackground</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifDisposal" title="Image.Gif.GifDisposal">GifDisposal</a></code></li><li id="k-GifDisposalPrevious"><code><strong class="tok-constructor">GifDisposalPrevious</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifDisposal" title="Image.Gif.GifDisposal">GifDisposal</a></code></li><li id="k-GifDisposalReserved"><code><strong class="tok-constructor">GifDisposalReserved</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifDisposal" title="Image.Gif.GifDisposal">GifDisposal</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GifError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GifError</strong></code><span class="doc-sig-links"><a href="#t-GifError" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

Why decoding or encoding failed.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GifBadHeader"><code><strong class="tok-constructor">GifBadHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifTruncated"><code><strong class="tok-constructor">GifTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifUnknownBlock"><code><strong class="tok-constructor">GifUnknownBlock</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifUnknownExtension"><code><strong class="tok-constructor">GifUnknownExtension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifBadGraphicControl"><code><strong class="tok-constructor">GifBadGraphicControl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifFrameOutOfBounds"><code><strong class="tok-constructor">GifFrameOutOfBounds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifNoColorTable"><code><strong class="tok-constructor">GifNoColorTable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifBadLiteralWidth"><code><strong class="tok-constructor">GifBadLiteralWidth</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifNotEnoughData"><code><strong class="tok-constructor">GifNotEnoughData</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifTooMuchData"><code><strong class="tok-constructor">GifTooMuchData</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifBadPixel"><code><strong class="tok-constructor">GifBadPixel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifBadImageData"><code><strong class="tok-constructor">GifBadImageData</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Lzw/#t-LzwError" title="Text.Lzw.LzwError">LzwError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifMissingImage"><code><strong class="tok-constructor">GifMissingImage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifNoFrames"><code><strong class="tok-constructor">GifNoFrames</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifEmptyPalette"><code><strong class="tok-constructor">GifEmptyPalette</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifPaletteTooLarge"><code><strong class="tok-constructor">GifPaletteTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifNotIndexed"><code><strong class="tok-constructor">GifNotIndexed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifTooLarge"><code><strong class="tok-constructor">GifTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li><li id="k-GifLoopTooLarge"><code><strong class="tok-constructor">GifLoopTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GifFrame">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GifFrame</strong></code><span class="doc-sig-links"><a href="#t-GifFrame" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L64" title="View source at line 64">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GifFrame"><code><strong class="tok-constructor">GifFrame</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifDisposal" title="Image.Gif.GifDisposal">GifDisposal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GifLoop">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GifLoop</strong></code><span class="doc-sig-links"><a href="#t-GifLoop" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

How many times an animation plays.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GifLoopForever"><code><strong class="tok-constructor">GifLoopForever</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifLoop" title="Image.Gif.GifLoop">GifLoop</a></code></li><li id="k-GifPlayOnce"><code><strong class="tok-constructor">GifPlayOnce</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifLoop" title="Image.Gif.GifLoop">GifLoop</a></code></li><li id="k-GifLoopTimes"><code><strong class="tok-constructor">GifLoopTimes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifLoop" title="Image.Gif.GifLoop">GifLoop</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-gif">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifLoop" title="Image.Gif.GifLoop">GifLoop</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a></code><span class="doc-sig-links"><a href="#v-gif" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L72" title="View source at line 72">Source</a></span></div>
<div class="doc-body">

Make a GIF from its parts.

</div>
</article>

<article class="doc-entry" id="v-gif-background-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-background-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-gif-background-index" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

Background palette index.

</div>
</article>

<article class="doc-entry" id="v-gif-config">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-config</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-gif-config" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

The screen width, height and global palette, from the header alone.

</div>
</article>

<article class="doc-entry" id="v-gif-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-gif-decode" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L207" title="View source at line 207">Source</a></span></div>
<div class="doc-body">

Decode the first frame's raster, ignoring the rest of the file.

</div>
</article>

<article class="doc-entry" id="v-gif-decode-all">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-decode-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a></code><span class="doc-sig-links"><a href="#v-gif-decode-all" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L203" title="View source at line 203">Source</a></span></div>
<div class="doc-body">

Decode every frame of a file.

</div>
</article>

<article class="doc-entry" id="v-gif-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gif-encode" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L465" title="View source at line 465">Source</a></span></div>
<div class="doc-body">

Encode one indexed raster as a single-frame GIF whose global palette is the raster's.

</div>
</article>

<article class="doc-entry" id="v-gif-encode-all">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-encode-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gif-encode-all" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L469" title="View source at line 469">Source</a></span></div>
<div class="doc-body">

Encode an animation.

</div>
</article>

<article class="doc-entry" id="v-gif-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifError" title="Image.Gif.GifError">GifError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-gif-error-message" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Render an error.

</div>
</article>

<article class="doc-entry" id="v-gif-frame">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-frame</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifDisposal" title="Image.Gif.GifDisposal">GifDisposal</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a></code><span class="doc-sig-links"><a href="#v-gif-frame" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

Make a frame from an indexed raster, its offsets, delay (hundredths of a second) and disposal.

</div>
</article>

<article class="doc-entry" id="v-gif-frame-delay">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-frame-delay</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-gif-frame-delay" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

A frame's delay in hundredths of a second.

</div>
</article>

<article class="doc-entry" id="v-gif-frame-disposal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-frame-disposal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifDisposal" title="Image.Gif.GifDisposal">GifDisposal</a></code><span class="doc-sig-links"><a href="#v-gif-frame-disposal" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

A frame's disposal method.

</div>
</article>

<article class="doc-entry" id="v-gif-frame-left">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-frame-left</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-gif-frame-left" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

A frame's left offset.

</div>
</article>

<article class="doc-entry" id="v-gif-frame-raster">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-frame-raster</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Raster/#t-Raster" title="Image.Raster.Raster">Raster</a></code><span class="doc-sig-links"><a href="#v-gif-frame-raster" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

A frame's indexed raster (its palette is the frame's effective palette).

</div>
</article>

<article class="doc-entry" id="v-gif-frame-top">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-frame-top</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-gif-frame-top" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

A frame's top offset.

</div>
</article>

<article class="doc-entry" id="v-gif-frames">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-frames</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifFrame" title="Image.Gif.GifFrame">GifFrame</a></code><span class="doc-sig-links"><a href="#v-gif-frames" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

Frames in order.

</div>
</article>

<article class="doc-entry" id="v-gif-height">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-height</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-gif-height" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

Screen height.

</div>
</article>

<article class="doc-entry" id="v-gif-loop">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-loop</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-GifLoop" title="Image.Gif.GifLoop">GifLoop</a></code><span class="doc-sig-links"><a href="#v-gif-loop" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

Looping.

</div>
</article>

<article class="doc-entry" id="v-gif-palette">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-palette</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gif-palette" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

Global palette as RGBA quadruples, empty when there is none.

</div>
</article>

<article class="doc-entry" id="v-gif-width">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gif-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Gif/#t-Gif" title="Image.Gif.Gif">Gif</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-gif-width" title="Permalink">#</a><a href="/packages/stdlib/Image.Gif/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

Screen width.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Image/Gif.cinch`](/packages/stdlib/Image.Gif/source/). Source SHA-256: `ff3c8851f3f303ba6dd255d062414ba3fb90b555e403a852f245d25589aeaeeb`.
