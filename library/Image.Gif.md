# Image.Gif

Decode and encode GIF images and animations: palettes, transparency, delays, disposal, looping and interlace.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Image.Gif`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Image.Gif.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Image.Raster](Image.Raster.md), [Text.Lzw](Text.Lzw.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Image.Gif](Image.Gif.md), [Image.Raster](Image.Raster.md), [Text.Lzw](Text.Lzw.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-476966"></a>

## `Gif`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Gif</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Image.Gif.cinch) · line 70

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Gif</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifFrame</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifLoop</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">Gif</span>))))))</code></pre>

<a id="type-476966446973706f73616c"></a>

## `GifDisposal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GifDisposal</span></code></pre>

What happens to a frame's area before the next frame is drawn.

[Source](/reference-sources/Image.Gif.cinch) · line 50

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifDisposalUnspecified</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifDisposal</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifDisposalNone</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifDisposal</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifDisposalBackground</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifDisposal</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifDisposalPrevious</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifDisposal</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifDisposalReserved</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifDisposal</span>)</code></pre>

<a id="type-4769664572726f72"></a>

## `GifError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GifError</span></code></pre>

Why decoding or encoding failed.

[Source](/reference-sources/Image.Gif.cinch) · line 125

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifBadHeader</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifTruncated</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifUnknownBlock</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifUnknownExtension</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifBadGraphicControl</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifFrameOutOfBounds</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifNoColorTable</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifBadLiteralWidth</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifNotEnoughData</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifTooMuchData</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifBadPixel</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifBadImageData</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Lzw</span>.<span class="tok-constructor">LzwError</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifMissingImage</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifNoFrames</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifEmptyPalette</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifPaletteTooLarge</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifNotIndexed</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifTooLarge</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifLoopTooLarge</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span></code></pre>

<a id="type-4769664672616d65"></a>

## `GifFrame`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GifFrame</span></code></pre>

Fields in the order listed above.

[Source](/reference-sources/Image.Gif.cinch) · line 65

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifFrame</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifDisposal</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifFrame</span>)))))</code></pre>

<a id="type-4769664c6f6f70"></a>

## `GifLoop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GifLoop</span></code></pre>

How many times an animation plays.

[Source](/reference-sources/Image.Gif.cinch) · line 41

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifLoopForever</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifLoop</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifPlayOnce</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifLoop</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GifLoopTimes</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifLoop</span>)</code></pre>

# Values

<a id="value-676966"></a>

## `gif`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifFrame</span>) -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifLoop</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">Gif</span>))))))</code></pre>

Make a GIF from its parts.

[Source](/reference-sources/Image.Gif.cinch) · line 73

<a id="value-6769662d6261636b67726f756e642d696e646578"></a>

## `gif-background-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-background-index</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">Gif</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Background palette index.

[Source](/reference-sources/Image.Gif.cinch) · line 97

<a id="value-6769662d636f6e666967"></a>

## `gif-config`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-config</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bytes</span>)))</code></pre>

The screen width, height and global palette, from the header alone.

[Source](/reference-sources/Image.Gif.cinch) · line 214

<a id="value-6769662d6465636f6465"></a>

## `gif-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-decode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>))</code></pre>

Decode the first frame's raster, ignoring the rest of the file.

[Source](/reference-sources/Image.Gif.cinch) · line 208

<a id="value-6769662d6465636f64652d616c6c"></a>

## `gif-decode-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-decode-all</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">Gif</span>))</code></pre>

Decode every frame of a file.

[Source](/reference-sources/Image.Gif.cinch) · line 204

<a id="value-6769662d656e636f6465"></a>

## `gif-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-encode</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Encode one indexed raster as a single-frame GIF whose global palette is the raster's.

[Source](/reference-sources/Image.Gif.cinch) · line 485

<a id="value-6769662d656e636f64652d616c6c"></a>

## `gif-encode-all`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-encode-all</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">Gif</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Encode an animation.

[Source](/reference-sources/Image.Gif.cinch) · line 489

<a id="value-6769662d6572726f722d6d657373616765"></a>

## `gif-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-error-message</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an error.

[Source](/reference-sources/Image.Gif.cinch) · line 166

<a id="value-6769662d6672616d65"></a>

## `gif-frame`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-frame</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifDisposal</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifFrame</span>)))))</code></pre>

Make a frame from an indexed raster, its offsets, delay (hundredths of a second) and disposal.

[Source](/reference-sources/Image.Gif.cinch) · line 77

<a id="value-6769662d6672616d652d64656c6179"></a>

## `gif-frame-delay`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-frame-delay</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifFrame</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

A frame's delay in hundredths of a second.

[Source](/reference-sources/Image.Gif.cinch) · line 117

<a id="value-6769662d6672616d652d646973706f73616c"></a>

## `gif-frame-disposal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-frame-disposal</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifFrame</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifDisposal</span>)</code></pre>

A frame's disposal method.

[Source](/reference-sources/Image.Gif.cinch) · line 121

<a id="value-6769662d6672616d652d6c656674"></a>

## `gif-frame-left`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-frame-left</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifFrame</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

A frame's left offset.

[Source](/reference-sources/Image.Gif.cinch) · line 109

<a id="value-6769662d6672616d652d726173746572"></a>

## `gif-frame-raster`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-frame-raster</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifFrame</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Raster</span>.<span class="tok-constructor">Raster</span>)</code></pre>

A frame's indexed raster (its palette is the frame's effective palette).

[Source](/reference-sources/Image.Gif.cinch) · line 105

<a id="value-6769662d6672616d652d746f70"></a>

## `gif-frame-top`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-frame-top</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifFrame</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

A frame's top offset.

[Source](/reference-sources/Image.Gif.cinch) · line 113

<a id="value-6769662d6672616d6573"></a>

## `gif-frames`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-frames</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">Gif</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifFrame</span>))</code></pre>

Frames in order.

[Source](/reference-sources/Image.Gif.cinch) · line 89

<a id="value-6769662d686569676874"></a>

## `gif-height`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-height</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">Gif</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Screen height.

[Source](/reference-sources/Image.Gif.cinch) · line 85

<a id="value-6769662d6c6f6f70"></a>

## `gif-loop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-loop</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">Gif</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">GifLoop</span>)</code></pre>

Looping.

[Source](/reference-sources/Image.Gif.cinch) · line 93

<a id="value-6769662d70616c65747465"></a>

## `gif-palette`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-palette</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">Gif</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Global palette as RGBA quadruples, empty when there is none.

[Source](/reference-sources/Image.Gif.cinch) · line 101

<a id="value-6769662d7769647468"></a>

## `gif-width`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gif-width</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Gif</span>.<span class="tok-constructor">Gif</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Screen width.

[Source](/reference-sources/Image.Gif.cinch) · line 81

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Image/Gif.cinch`. Source SHA-256: `5e12dd3501c4492b077dfade090b3e6cf9af32f970ad7f99d0a58c7cb8dbfabd`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
