# Image.Color

Color representations, conversion models and immutable palettes with first-nearest lookup.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Image.Color</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Image.Color/source/">View source</a><a href="/packages/search/?q=%2BImage.Color">Search module</a><a href="/reference-sources/Image.Color.cinch" download="Image.Color.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

<details class="doc-notes"><summary>Module notes</summary>

Pure color values, explicit conversion models and immutable palettes.
Standard representations retain raw channel widths and model identity.
Canonical observation yields premultiplied16-bit RGBA in wider machine words.
Conversion rounding, first palette ties and zero-alpha behavior are explicit.

</details>

<details class="doc-synopsis"><summary>Synopsis (58 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Color">Color</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Color</strong></code></li><li><a class="syn-name" href="#t-ColorKind">ColorKind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ColorKind</strong></code></li><li><a class="syn-name" href="#t-ColorModel">ColorModel</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ColorModel</strong></code></li><li><a class="syn-name" href="#t-ColorPalette">ColorPalette</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ColorPalette</strong></code></li><li><a class="syn-name" href="#v-color-alpha16">color-alpha16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-alpha16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-alpha16-components">color-alpha16-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-alpha16-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-color-alpha16-model">color-alpha16-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-alpha16-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-alpha8">color-alpha8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-alpha8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-alpha8-components">color-alpha8-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-alpha8-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-color-alpha8-model">color-alpha8-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-alpha8-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-black">color-black</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-black</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-cmyk">color-cmyk</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-cmyk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-cmyk-components">color-cmyk-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-cmyk-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code></li><li><a class="syn-name" href="#v-color-cmyk-model">color-cmyk-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-cmyk-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-cmyk-to-rgb">color-cmyk-to-rgb</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-cmyk-to-rgb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code></li><li><a class="syn-name" href="#v-color-convert">color-convert</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-convert</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-gray16">color-gray16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-gray16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-gray16-components">color-gray16-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-gray16-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-color-gray16-model">color-gray16-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-gray16-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-gray8">color-gray8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-gray8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-gray8-components">color-gray8-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-gray8-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-color-gray8-model">color-gray8-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-gray8-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-kind">color-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li><a class="syn-name" href="#v-color-model">color-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-model</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-model-kind">color-model-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-model-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li><a class="syn-name" href="#v-color-model-total">color-model-total</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-model-total</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-nrgba16">color-nrgba16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-nrgba16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-nrgba16-components">color-nrgba16-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-nrgba16-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>)</code></li><li><a class="syn-name" href="#v-color-nrgba16-model">color-nrgba16-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-nrgba16-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-nrgba8">color-nrgba8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-nrgba8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-nrgba8-components">color-nrgba8-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-nrgba8-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code></li><li><a class="syn-name" href="#v-color-nrgba8-model">color-nrgba8-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-nrgba8-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-opaque">color-opaque</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-opaque</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-palette">color-palette</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-palette</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a></code></li><li><a class="syn-name" href="#v-color-palette-colors">color-palette-colors</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-palette-colors</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-palette-convert">color-palette-convert</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-palette-convert</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-palette-index">color-palette-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-palette-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-color-palette-model">color-palette-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-palette-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-rgb-to-cmyk">color-rgb-to-cmyk</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-rgb-to-cmyk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code></li><li><a class="syn-name" href="#v-color-rgb-to-ycbcr">color-rgb-to-ycbcr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-rgb-to-ycbcr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code></li><li><a class="syn-name" href="#v-color-rgba">color-rgba</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-rgba</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>)</code></li><li><a class="syn-name" href="#v-color-rgba16">color-rgba16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-rgba16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-rgba16-components">color-rgba16-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-rgba16-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>)</code></li><li><a class="syn-name" href="#v-color-rgba16-model">color-rgba16-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-rgba16-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-rgba8">color-rgba8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-rgba8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-rgba8-components">color-rgba8-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-rgba8-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code></li><li><a class="syn-name" href="#v-color-rgba8-model">color-rgba8-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-rgba8-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-source">color-source</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-source</strong> :: (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>)) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-transparent">color-transparent</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-transparent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-white">color-white</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-white</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-ycbcr">color-ycbcr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-ycbcr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-ycbcr-components">color-ycbcr-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-ycbcr-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code></li><li><a class="syn-name" href="#v-color-ycbcr-model">color-ycbcr-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-ycbcr-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li><li><a class="syn-name" href="#v-color-ycbcr-to-rgb">color-ycbcr-to-rgb</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-ycbcr-to-rgb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code></li><li><a class="syn-name" href="#v-color-ycbcra">color-ycbcra</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-ycbcra</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-ycbcra-base">color-ycbcra-base</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-ycbcra-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code></li><li><a class="syn-name" href="#v-color-ycbcra-components">color-ycbcra-components</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-ycbcra-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code></li><li><a class="syn-name" href="#v-color-ycbcra-model">color-ycbcra-model</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-ycbcra-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Color">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Color</strong></code><span class="doc-sig-links"><a href="#t-Color" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

Opaque immutable color retaining its source identity and exact raw fields.

</div>
</article>

<article class="doc-entry" id="t-ColorKind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ColorKind</strong></code><span class="doc-sig-links"><a href="#t-ColorKind" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

Observable identity of each standard representation or custom channel source.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Rgba8Kind"><code><strong class="tok-constructor">Rgba8Kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-Rgba16Kind"><code><strong class="tok-constructor">Rgba16Kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-Nrgba8Kind"><code><strong class="tok-constructor">Nrgba8Kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-Nrgba16Kind"><code><strong class="tok-constructor">Nrgba16Kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-Alpha8Kind"><code><strong class="tok-constructor">Alpha8Kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-Alpha16Kind"><code><strong class="tok-constructor">Alpha16Kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-Gray8Kind"><code><strong class="tok-constructor">Gray8Kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-Gray16Kind"><code><strong class="tok-constructor">Gray16Kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-YcbcrKind"><code><strong class="tok-constructor">YcbcrKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-YcbcraKind"><code><strong class="tok-constructor">YcbcraKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-CmykKind"><code><strong class="tok-constructor">CmykKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li><li id="k-RgbaSourceKind"><code><strong class="tok-constructor">RgbaSourceKind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ColorModel">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ColorModel</strong></code><span class="doc-sig-links"><a href="#t-ColorModel" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Explicit pure conversion capability with optional output for absent conversions.

</div>
</article>

<article class="doc-entry" id="t-ColorPalette">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ColorPalette</strong></code><span class="doc-sig-links"><a href="#t-ColorPalette" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

An immutable sequence; construction and observation preserve color identities.

</div>
</article>

# Values

<article class="doc-entry" id="v-color-alpha16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-alpha16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-alpha16" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L191" title="View source at line 191">Source</a></span></div>
<div class="doc-body">

Construct 16-bit alpha; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-alpha16-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-alpha16-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-color-alpha16-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L195" title="View source at line 195">Source</a></span></div>
<div class="doc-body">

Observe exact raw alpha16 fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-alpha16-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-alpha16-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-alpha16-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L201" title="View source at line 201">Source</a></span></div>
<div class="doc-body">

Convert to the alpha16 representation; already matching values retain their fields.

</div>
</article>

<article class="doc-entry" id="v-color-alpha8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-alpha8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-alpha8" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

Construct 8-bit alpha; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-alpha8-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-alpha8-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-color-alpha8-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

Observe exact raw alpha8 fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-alpha8-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-alpha8-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-alpha8-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Convert to the alpha8 representation; already matching values retain their fields.

</div>
</article>

<article class="doc-entry" id="v-color-black">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-black</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-black" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L281" title="View source at line 281">Source</a></span></div>
<div class="doc-body">

Preserve the conventional opaque black grayscale representation.

</div>
</article>

<article class="doc-entry" id="v-color-cmyk">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-cmyk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-cmyk" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L261" title="View source at line 261">Source</a></span></div>
<div class="doc-body">

Construct opaque8-bit cyan, magenta, yellow and black; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-cmyk-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-cmyk-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code><span class="doc-sig-links"><a href="#v-color-cmyk-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L265" title="View source at line 265">Source</a></span></div>
<div class="doc-body">

Observe exact raw cmyk fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-cmyk-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-cmyk-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-cmyk-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L271" title="View source at line 271">Source</a></span></div>
<div class="doc-body">

Convert to the cmyk representation; already matching values retain their fields.

</div>
</article>

<article class="doc-entry" id="v-color-cmyk-to-rgb">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-cmyk-to-rgb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code><span class="doc-sig-links"><a href="#v-color-cmyk-to-rgb" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L379" title="View source at line 379">Source</a></span></div>
<div class="doc-body">

Convert8-bit ink channels through16-bit products before reducing precision.

</div>
</article>

<article class="doc-entry" id="v-color-convert">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-convert</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-convert" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L309" title="View source at line 309">Source</a></span></div>
<div class="doc-body">

Dispatch an explicit conversion without adding mutable state or hidden registration.

</div>
</article>

<article class="doc-entry" id="v-color-gray16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-gray16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-gray16" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L219" title="View source at line 219">Source</a></span></div>
<div class="doc-body">

Construct opaque16-bit gray; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-gray16-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-gray16-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-color-gray16-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L223" title="View source at line 223">Source</a></span></div>
<div class="doc-body">

Observe exact raw gray16 fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-gray16-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-gray16-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-gray16-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L229" title="View source at line 229">Source</a></span></div>
<div class="doc-body">

Convert to the gray16 representation; already matching values retain their fields.

</div>
</article>

<article class="doc-entry" id="v-color-gray8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-gray8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-gray8" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

Construct opaque8-bit gray; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-gray8-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-gray8-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-color-gray8-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

Observe exact raw gray8 fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-gray8-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-gray8-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-gray8-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L215" title="View source at line 215">Source</a></span></div>
<div class="doc-body">

Convert to the gray8 representation; already matching values retain their fields.

</div>
</article>

<article class="doc-entry" id="v-color-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code><span class="doc-sig-links"><a href="#v-color-kind" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

Observe the retained representation without forcing a custom channel callback.

</div>
</article>

<article class="doc-entry" id="v-color-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-model</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L297" title="View source at line 297">Source</a></span></div>
<div class="doc-body">

Retain a pure model callback including its possible absent result.

</div>
</article>

<article class="doc-entry" id="v-color-model-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-model-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorKind" title="Image.Color.ColorKind">ColorKind</a></code><span class="doc-sig-links"><a href="#v-color-model-kind" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L305" title="View source at line 305">Source</a></span></div>
<div class="doc-body">

Observe a standard target kind; callbacks and palettes have no standard target tag.

</div>
</article>

<article class="doc-entry" id="v-color-model-total">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-model-total</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-model-total" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L301" title="View source at line 301">Source</a></span></div>
<div class="doc-body">

Retain a total pure model callback, wrapping its returned color as present.

</div>
</article>

<article class="doc-entry" id="v-color-nrgba16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-nrgba16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-nrgba16" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L163" title="View source at line 163">Source</a></span></div>
<div class="doc-body">

Construct straight16-bit red, green, blue and alpha; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-nrgba16-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-nrgba16-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>)</code><span class="doc-sig-links"><a href="#v-color-nrgba16-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Observe exact raw nrgba16 fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-nrgba16-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-nrgba16-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-nrgba16-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L173" title="View source at line 173">Source</a></span></div>
<div class="doc-body">

Convert to the nrgba16 representation; already matching values retain their fields.

</div>
</article>

<article class="doc-entry" id="v-color-nrgba8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-nrgba8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-nrgba8" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L149" title="View source at line 149">Source</a></span></div>
<div class="doc-body">

Construct straight8-bit red, green, blue and alpha; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-nrgba8-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-nrgba8-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code><span class="doc-sig-links"><a href="#v-color-nrgba8-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

Observe exact raw nrgba8 fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-nrgba8-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-nrgba8-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-nrgba8-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

Convert to the nrgba8 representation; already matching values retain their fields.

</div>
</article>

<article class="doc-entry" id="v-color-opaque">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-opaque</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-opaque" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L293" title="View source at line 293">Source</a></span></div>
<div class="doc-body">

Preserve the full-alpha-only representation of opacity.

</div>
</article>

<article class="doc-entry" id="v-color-palette">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-palette</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a></code><span class="doc-sig-links"><a href="#v-color-palette" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L383" title="View source at line 383">Source</a></span></div>
<div class="doc-body">

Capture an immutable color sequence, including an explicit empty palette.

</div>
</article>

<article class="doc-entry" id="v-color-palette-colors">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-palette-colors</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-palette-colors" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L387" title="View source at line 387">Source</a></span></div>
<div class="doc-body">

Observe the same immutable color values in their original order.

</div>
</article>

<article class="doc-entry" id="v-color-palette-convert">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-palette-convert</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-palette-convert" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L402" title="View source at line 402">Source</a></span></div>
<div class="doc-body">

Return the original nearest color, with absence for an empty palette.

</div>
</article>

<article class="doc-entry" id="v-color-palette-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-palette-index</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-color-palette-index" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L395" title="View source at line 395">Source</a></span></div>
<div class="doc-body">

Return the first nearest index using all four premultiplied channels; empty is zero.

</div>
</article>

<article class="doc-entry" id="v-color-palette-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-palette-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-palette-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L391" title="View source at line 391">Source</a></span></div>
<div class="doc-body">

Treat a palette as an explicit model; empty conversion remains absent.

</div>
</article>

<article class="doc-entry" id="v-color-rgb-to-cmyk">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-rgb-to-cmyk</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code><span class="doc-sig-links"><a href="#v-color-rgb-to-cmyk" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L375" title="View source at line 375">Source</a></span></div>
<div class="doc-body">

Convert an8-bit RGB triple with the explicit all-black singularity.

</div>
</article>

<article class="doc-entry" id="v-color-rgb-to-ycbcr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-rgb-to-ycbcr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code><span class="doc-sig-links"><a href="#v-color-rgb-to-ycbcr" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L367" title="View source at line 367">Source</a></span></div>
<div class="doc-body">

Convert an8-bit RGB triple with exact fixed-point luma/chroma rounding.

</div>
</article>

<article class="doc-entry" id="v-color-rgba">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-rgba</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>)</code><span class="doc-sig-links"><a href="#v-color-rgba" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L316" title="View source at line 316">Source</a></span></div>
<div class="doc-body">

Observe sixteen-bit premultiplied channels widened for multiplication without overflow.

</div>
</article>

<article class="doc-entry" id="v-color-rgba16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-rgba16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-rgba16" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Construct premultiplied16-bit red, green, blue and alpha; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-rgba16-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-rgba16-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>)</code><span class="doc-sig-links"><a href="#v-color-rgba16-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L139" title="View source at line 139">Source</a></span></div>
<div class="doc-body">

Observe exact raw rgba16 fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-rgba16-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-rgba16-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-rgba16-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L145" title="View source at line 145">Source</a></span></div>
<div class="doc-body">

Convert to the rgba16 representation; already matching values retain their fields.

</div>
</article>

<article class="doc-entry" id="v-color-rgba8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-rgba8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-rgba8" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

Construct premultiplied8-bit red, green, blue and alpha; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-rgba8-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-rgba8-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code><span class="doc-sig-links"><a href="#v-color-rgba8-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

Observe exact raw rgba8 fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-rgba8-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-rgba8-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-rgba8-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

Convert to the rgba8 representation; already matching values retain their fields.

</div>
</article>

<article class="doc-entry" id="v-color-source">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-source</strong> :: (<a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a>)) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-source" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Retain a pure callback; channels are sixteen-bit premultiplied values, not copied eagerly.

</div>
</article>

<article class="doc-entry" id="v-color-transparent">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-transparent</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-transparent" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L289" title="View source at line 289">Source</a></span></div>
<div class="doc-body">

Preserve the zero-alpha-only representation of transparency.

</div>
</article>

<article class="doc-entry" id="v-color-white">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-white</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-white" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L285" title="View source at line 285">Source</a></span></div>
<div class="doc-body">

Preserve the conventional opaque white grayscale representation.

</div>
</article>

<article class="doc-entry" id="v-color-ycbcr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-ycbcr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-ycbcr" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L233" title="View source at line 233">Source</a></span></div>
<div class="doc-body">

Construct opaque8-bit luma and chroma; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-ycbcr-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-ycbcr-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code><span class="doc-sig-links"><a href="#v-color-ycbcr-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L237" title="View source at line 237">Source</a></span></div>
<div class="doc-body">

Observe exact raw ycbcr fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-ycbcr-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-ycbcr-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-ycbcr-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L243" title="View source at line 243">Source</a></span></div>
<div class="doc-body">

Convert to the ycbcr representation; already matching values retain their fields.

</div>
</article>

<article class="doc-entry" id="v-color-ycbcr-to-rgb">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-ycbcr-to-rgb</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code><span class="doc-sig-links"><a href="#v-color-ycbcr-to-rgb" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L371" title="View source at line 371">Source</a></span></div>
<div class="doc-body">

Convert an8-bit luma/chroma triple with saturated channels and y-dependent rounding.

</div>
</article>

<article class="doc-entry" id="v-color-ycbcra">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-ycbcra</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-ycbcra" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L247" title="View source at line 247">Source</a></span></div>
<div class="doc-body">

Construct straight8-bit luma, chroma and alpha; raw channels are retained without alpha clamping.

</div>
</article>

<article class="doc-entry" id="v-color-ycbcra-base">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-ycbcra-base</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a></code><span class="doc-sig-links"><a href="#v-color-ycbcra-base" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L275" title="View source at line 275">Source</a></span></div>
<div class="doc-body">

Observe the embedded luma/chroma value without changing the independent alpha channel.

</div>
</article>

<article class="doc-entry" id="v-color-ycbcra-components">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-ycbcra-components</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-Color" title="Image.Color.Color">Color</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a>)</code><span class="doc-sig-links"><a href="#v-color-ycbcra-components" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L251" title="View source at line 251">Source</a></span></div>
<div class="doc-body">

Observe exact raw ycbcra fields, returning absent for other representations.

</div>
</article>

<article class="doc-entry" id="v-color-ycbcra-model">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-ycbcra-model</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorModel" title="Image.Color.ColorModel">ColorModel</a></code><span class="doc-sig-links"><a href="#v-color-ycbcra-model" title="Permalink">#</a><a href="/packages/stdlib/Image.Color/source/#L257" title="View source at line 257">Source</a></span></div>
<div class="doc-body">

Convert to the ycbcra representation; already matching values retain their fields.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Image/Color.cinch`](/packages/stdlib/Image.Color/source/). Source SHA-256: `151b5fea2d5e3c899aa72701071b100d62a999d04cc4d07147fe836398ba9707`.
