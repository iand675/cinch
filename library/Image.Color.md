# Image.Color

Color representations, conversion models and immutable palettes with first-nearest lookup.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Image.Color`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Image.Color.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

Non-embedded source closure: [Image.Color](Image.Color.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pure color values, explicit conversion models and immutable palettes.
Standard representations retain raw channel widths and model identity.
Canonical observation yields premultiplied16-bit RGBA in wider machine words.
Conversion rounding, first palette ties and zero-alpha behavior are explicit.

</details>

# Types and constructors

<a id="type-436f6c6f72"></a>

## `Color`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Color</span></code></pre>

Opaque immutable color retaining its source identity and exact raw fields.

[Source](/reference-sources/Image.Color.cinch) · line 60

<a id="type-436f6c6f724b696e64"></a>

## `ColorKind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ColorKind</span></code></pre>

Observable identity of each standard representation or custom channel source.

[Source](/reference-sources/Image.Color.cinch) · line 33

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rgba8Kind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Rgba16Kind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Nrgba8Kind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Nrgba16Kind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Alpha8Kind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Alpha16Kind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Gray8Kind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Gray16Kind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">YcbcrKind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">YcbcraKind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CmykKind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RgbaSourceKind</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span></code></pre>

<a id="type-436f6c6f724d6f64656c"></a>

## `ColorModel`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ColorModel</span></code></pre>

Explicit pure conversion capability with optional output for absent conversions.

[Source](/reference-sources/Image.Color.cinch) · line 67

<a id="type-436f6c6f7250616c65747465"></a>

## `ColorPalette`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ColorPalette</span></code></pre>

An immutable sequence; construction and observation preserve color identities.

[Source](/reference-sources/Image.Color.cinch) · line 76

# Values

<a id="value-636f6c6f722d616c7068613136"></a>

## `color-alpha16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-alpha16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>)</code></pre>

Construct 16-bit alpha; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 191

<a id="value-636f6c6f722d616c70686131362d636f6d706f6e656e7473"></a>

## `color-alpha16-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-alpha16-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

Observe exact raw alpha16 fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 195

<a id="value-636f6c6f722d616c70686131362d6d6f64656c"></a>

## `color-alpha16-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-alpha16-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the alpha16 representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 201

<a id="value-636f6c6f722d616c70686138"></a>

## `color-alpha8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-alpha8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>)</code></pre>

Construct 8-bit alpha; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 177

<a id="value-636f6c6f722d616c706861382d636f6d706f6e656e7473"></a>

## `color-alpha8-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-alpha8-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

Observe exact raw alpha8 fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 181

<a id="value-636f6c6f722d616c706861382d6d6f64656c"></a>

## `color-alpha8-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-alpha8-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the alpha8 representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 187

<a id="value-636f6c6f722d626c61636b"></a>

## `color-black`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-black</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span></code></pre>

Preserve the conventional opaque black grayscale representation.

[Source](/reference-sources/Image.Color.cinch) · line 281

<a id="value-636f6c6f722d636d796b"></a>

## `color-cmyk`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-cmyk</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>))))</code></pre>

Construct opaque8-bit cyan, magenta, yellow and black; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 261

<a id="value-636f6c6f722d636d796b2d636f6d706f6e656e7473"></a>

## `color-cmyk-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-cmyk-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>)))</code></pre>

Observe exact raw cmyk fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 265

<a id="value-636f6c6f722d636d796b2d6d6f64656c"></a>

## `color-cmyk-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-cmyk-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the cmyk representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 271

<a id="value-636f6c6f722d636d796b2d746f2d726762"></a>

## `color-cmyk-to-rgb`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-cmyk-to-rgb</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>)))))</code></pre>

Convert8-bit ink channels through16-bit products before reducing precision.

[Source](/reference-sources/Image.Color.cinch) · line 379

<a id="value-636f6c6f722d636f6e76657274"></a>

## `color-convert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-convert</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>)))</code></pre>

Dispatch an explicit conversion without adding mutable state or hidden registration.

[Source](/reference-sources/Image.Color.cinch) · line 309

<a id="value-636f6c6f722d677261793136"></a>

## `color-gray16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-gray16</span> :: (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>)</code></pre>

Construct opaque16-bit gray; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 219

<a id="value-636f6c6f722d6772617931362d636f6d706f6e656e7473"></a>

## `color-gray16-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-gray16-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U16</span>))</code></pre>

Observe exact raw gray16 fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 223

<a id="value-636f6c6f722d6772617931362d6d6f64656c"></a>

## `color-gray16-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-gray16-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the gray16 representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 229

<a id="value-636f6c6f722d6772617938"></a>

## `color-gray8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-gray8</span> :: (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>)</code></pre>

Construct opaque8-bit gray; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 205

<a id="value-636f6c6f722d67726179382d636f6d706f6e656e7473"></a>

## `color-gray8-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-gray8-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U8</span>))</code></pre>

Observe exact raw gray8 fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 209

<a id="value-636f6c6f722d67726179382d6d6f64656c"></a>

## `color-gray8-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-gray8-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the gray8 representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 215

<a id="value-636f6c6f722d6b696e64"></a>

## `color-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-kind</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span>)</code></pre>

Observe the retained representation without forcing a custom channel callback.

[Source](/reference-sources/Image.Color.cinch) · line 97

<a id="value-636f6c6f722d6d6f64656c"></a>

## `color-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-model</span> :: ((<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>)) -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span>)</code></pre>

Retain a pure model callback including its possible absent result.

[Source](/reference-sources/Image.Color.cinch) · line 297

<a id="value-636f6c6f722d6d6f64656c2d6b696e64"></a>

## `color-model-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-model-kind</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorKind</span>))</code></pre>

Observe a standard target kind; callbacks and palettes have no standard target tag.

[Source](/reference-sources/Image.Color.cinch) · line 305

<a id="value-636f6c6f722d6d6f64656c2d746f74616c"></a>

## `color-model-total`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-model-total</span> :: ((<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>) -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span>)</code></pre>

Retain a total pure model callback, wrapping its returned color as present.

[Source](/reference-sources/Image.Color.cinch) · line 301

<a id="value-636f6c6f722d6e726762613136"></a>

## `color-nrgba16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-nrgba16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>))))</code></pre>

Construct straight16-bit red, green, blue and alpha; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 163

<a id="value-636f6c6f722d6e7267626131362d636f6d706f6e656e7473"></a>

## `color-nrgba16-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-nrgba16-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">U16</span>, <span class="tok-constructor">U16</span>, <span class="tok-constructor">U16</span>, <span class="tok-constructor">U16</span>)))</code></pre>

Observe exact raw nrgba16 fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 167

<a id="value-636f6c6f722d6e7267626131362d6d6f64656c"></a>

## `color-nrgba16-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-nrgba16-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the nrgba16 representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 173

<a id="value-636f6c6f722d6e7267626138"></a>

## `color-nrgba8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-nrgba8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>))))</code></pre>

Construct straight8-bit red, green, blue and alpha; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 149

<a id="value-636f6c6f722d6e72676261382d636f6d706f6e656e7473"></a>

## `color-nrgba8-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-nrgba8-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>)))</code></pre>

Observe exact raw nrgba8 fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 153

<a id="value-636f6c6f722d6e72676261382d6d6f64656c"></a>

## `color-nrgba8-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-nrgba8-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the nrgba8 representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 159

<a id="value-636f6c6f722d6f7061717565"></a>

## `color-opaque`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-opaque</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span></code></pre>

Preserve the full-alpha-only representation of opacity.

[Source](/reference-sources/Image.Color.cinch) · line 293

<a id="value-636f6c6f722d70616c65747465"></a>

## `color-palette`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-palette</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>) -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorPalette</span>)</code></pre>

Capture an immutable color sequence, including an explicit empty palette.

[Source](/reference-sources/Image.Color.cinch) · line 383

<a id="value-636f6c6f722d70616c657474652d636f6c6f7273"></a>

## `color-palette-colors`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-palette-colors</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorPalette</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>))</code></pre>

Observe the same immutable color values in their original order.

[Source](/reference-sources/Image.Color.cinch) · line 387

<a id="value-636f6c6f722d70616c657474652d636f6e76657274"></a>

## `color-palette-convert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-palette-convert</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorPalette</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>)))</code></pre>

Return the original nearest color, with absence for an empty palette.

[Source](/reference-sources/Image.Color.cinch) · line 402

<a id="value-636f6c6f722d70616c657474652d696e646578"></a>

## `color-palette-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-palette-index</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorPalette</span> -&gt; (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

Return the first nearest index using all four premultiplied channels; empty is zero.

[Source](/reference-sources/Image.Color.cinch) · line 395

<a id="value-636f6c6f722d70616c657474652d6d6f64656c"></a>

## `color-palette-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-palette-model</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorPalette</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span>)</code></pre>

Treat a palette as an explicit model; empty conversion remains absent.

[Source](/reference-sources/Image.Color.cinch) · line 391

<a id="value-636f6c6f722d7267622d746f2d636d796b"></a>

## `color-rgb-to-cmyk`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-rgb-to-cmyk</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>))))</code></pre>

Convert an8-bit RGB triple with the explicit all-black singularity.

[Source](/reference-sources/Image.Color.cinch) · line 375

<a id="value-636f6c6f722d7267622d746f2d7963626372"></a>

## `color-rgb-to-ycbcr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-rgb-to-ycbcr</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>))))</code></pre>

Convert an8-bit RGB triple with exact fixed-point luma/chroma rounding.

[Source](/reference-sources/Image.Color.cinch) · line 367

<a id="value-636f6c6f722d72676261"></a>

## `color-rgba`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-rgba</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">U32</span>, <span class="tok-constructor">U32</span>, <span class="tok-constructor">U32</span>, <span class="tok-constructor">U32</span>))</code></pre>

Observe sixteen-bit premultiplied channels widened for multiplication without overflow.

[Source](/reference-sources/Image.Color.cinch) · line 316

<a id="value-636f6c6f722d726762613136"></a>

## `color-rgba16`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-rgba16</span> :: (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; (<span class="tok-constructor">U16</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>))))</code></pre>

Construct premultiplied16-bit red, green, blue and alpha; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 135

<a id="value-636f6c6f722d7267626131362d636f6d706f6e656e7473"></a>

## `color-rgba16-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-rgba16-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">U16</span>, <span class="tok-constructor">U16</span>, <span class="tok-constructor">U16</span>, <span class="tok-constructor">U16</span>)))</code></pre>

Observe exact raw rgba16 fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 139

<a id="value-636f6c6f722d7267626131362d6d6f64656c"></a>

## `color-rgba16-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-rgba16-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the rgba16 representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 145

<a id="value-636f6c6f722d7267626138"></a>

## `color-rgba8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-rgba8</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>))))</code></pre>

Construct premultiplied8-bit red, green, blue and alpha; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 121

<a id="value-636f6c6f722d72676261382d636f6d706f6e656e7473"></a>

## `color-rgba8-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-rgba8-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>)))</code></pre>

Observe exact raw rgba8 fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 125

<a id="value-636f6c6f722d72676261382d6d6f64656c"></a>

## `color-rgba8-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-rgba8-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the rgba8 representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 131

<a id="value-636f6c6f722d736f75726365"></a>

## `color-source`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-source</span> :: ((<span class="tok-constructor">Unit</span> -&gt; (<span class="tok-constructor">U16</span>, <span class="tok-constructor">U16</span>, <span class="tok-constructor">U16</span>, <span class="tok-constructor">U16</span>)) -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>)</code></pre>

Retain a pure callback; channels are sixteen-bit premultiplied values, not copied eagerly.

[Source](/reference-sources/Image.Color.cinch) · line 101

<a id="value-636f6c6f722d7472616e73706172656e74"></a>

## `color-transparent`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-transparent</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span></code></pre>

Preserve the zero-alpha-only representation of transparency.

[Source](/reference-sources/Image.Color.cinch) · line 289

<a id="value-636f6c6f722d7768697465"></a>

## `color-white`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-white</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span></code></pre>

Preserve the conventional opaque white grayscale representation.

[Source](/reference-sources/Image.Color.cinch) · line 285

<a id="value-636f6c6f722d7963626372"></a>

## `color-ycbcr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-ycbcr</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>)))</code></pre>

Construct opaque8-bit luma and chroma; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 233

<a id="value-636f6c6f722d79636263722d636f6d706f6e656e7473"></a>

## `color-ycbcr-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-ycbcr-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>)))</code></pre>

Observe exact raw ycbcr fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 237

<a id="value-636f6c6f722d79636263722d6d6f64656c"></a>

## `color-ycbcr-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-ycbcr-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the ycbcr representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 243

<a id="value-636f6c6f722d79636263722d746f2d726762"></a>

## `color-ycbcr-to-rgb`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-ycbcr-to-rgb</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>))))</code></pre>

Convert an8-bit luma/chroma triple with saturated channels and y-dependent rounding.

[Source](/reference-sources/Image.Color.cinch) · line 371

<a id="value-636f6c6f722d796362637261"></a>

## `color-ycbcra`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-ycbcra</span> :: (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; (<span class="tok-constructor">U8</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>))))</code></pre>

Construct straight8-bit luma, chroma and alpha; raw channels are retained without alpha clamping.

[Source](/reference-sources/Image.Color.cinch) · line 247

<a id="value-636f6c6f722d7963626372612d62617365"></a>

## `color-ycbcra-base`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-ycbcra-base</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span>))</code></pre>

Observe the embedded luma/chroma value without changing the independent alpha channel.

[Source](/reference-sources/Image.Color.cinch) · line 275

<a id="value-636f6c6f722d7963626372612d636f6d706f6e656e7473"></a>

## `color-ycbcra-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-ycbcra-components</span> :: (<span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">Color</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>, <span class="tok-constructor">U8</span>)))</code></pre>

Observe exact raw ycbcra fields, returning absent for other representations.

[Source](/reference-sources/Image.Color.cinch) · line 251

<a id="value-636f6c6f722d7963626372612d6d6f64656c"></a>

## `color-ycbcra-model`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-ycbcra-model</span> :: <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorModel</span></code></pre>

Convert to the ycbcra representation; already matching values retain their fields.

[Source](/reference-sources/Image.Color.cinch) · line 257

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Image/Color.cinch`. Source SHA-256: `c5fb57d446ef5e35f7fbd02404069b2f2b82b0f9c4563f894fc2e21f06da1932`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
