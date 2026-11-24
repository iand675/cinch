# Image.Color.Palette

Immutable factories for the ordered 256-color Plan 9 and 216-color WebSafe palettes.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Image.Color.Palette</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Image.Color.Palette/source/">View source</a><a href="/packages/search/?q=%2BImage.Color.Palette">Search module</a><a href="/reference-sources/Image.Color.Palette.cinch" download="Image.Color.Palette.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Core](Data.Core.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Image.Color](Image.Color.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Image.Color.Palette`: [Image.Color](Image.Color.md), [Image.Color.Palette](Image.Color.Palette.md).

```sh
cp seed/prelude/Image/Color.cinch ./Image.Color.cinch
cp seed/prelude/Image/Color/Palette.cinch ./Image.Color.Palette.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Construct immutable fixed color palettes in their exact published order.

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-color-palette-plan9">color-palette-plan9</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-palette-plan9</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a></code></li><li><a class="syn-name" href="#v-color-palette-web-safe">color-palette-web-safe</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">color-palette-web-safe</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-color-palette-plan9">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-palette-plan9</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a></code><span class="doc-sig-links"><a href="#v-color-palette-plan9" title="Permalink">#</a><a href="/packages/stdlib/Image.Color.Palette/source/#L11" title="View source at line 11">Source</a></span></div>
<div class="doc-body">

Return all256 Plan9 colors as opaque RGBA8 values in palette order.

</div>
</article>

<article class="doc-entry" id="v-color-palette-web-safe">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">color-palette-web-safe</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Image.Color/#t-ColorPalette" title="Image.Color.ColorPalette">ColorPalette</a></code><span class="doc-sig-links"><a href="#v-color-palette-web-safe" title="Permalink">#</a><a href="/packages/stdlib/Image.Color.Palette/source/#L15" title="View source at line 15">Source</a></span></div>
<div class="doc-body">

Return all216 web-safe colors as opaque RGBA8 values in palette order.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Image/Color/Palette.cinch`](/packages/stdlib/Image.Color.Palette/source/). Source SHA-256: `4d3f13a9a42b56e54be8d4af1c5ef176035b503d83c34dc0cb54671f820ac5dc`.
