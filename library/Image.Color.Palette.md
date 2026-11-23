# Image.Color.Palette

Immutable factories for the ordered 256-color Plan 9 and 216-color WebSafe palettes.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Image.Color.Palette`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Image.Color.Palette.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Core](Data.Core.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Image.Color](Image.Color.md).

Non-embedded source closure: [Image.Color](Image.Color.md), [Image.Color.Palette](Image.Color.Palette.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Construct immutable fixed color palettes in their exact published order.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-636f6c6f722d70616c657474652d706c616e39"></a>

## `color-palette-plan9`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-palette-plan9</span> :: (<span class="tok-constructor">Unit</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorPalette</span>)</code></pre>

Return all256 Plan9 colors as opaque RGBA8 values in palette order.

[Source](/reference-sources/Image.Color.Palette.cinch) · line 11

<a id="value-636f6c6f722d70616c657474652d7765622d73616665"></a>

## `color-palette-web-safe`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">color-palette-web-safe</span> :: (<span class="tok-constructor">Unit</span> -&gt; <span class="tok-constructor">Image</span>.<span class="tok-constructor">Color</span>.<span class="tok-constructor">ColorPalette</span>)</code></pre>

Return all216 web-safe colors as opaque RGBA8 values in palette order.

[Source](/reference-sources/Image.Color.Palette.cinch) · line 15

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Image/Color/Palette.cinch`. Source SHA-256: `4d3f13a9a42b56e54be8d4af1c5ef176035b503d83c34dc0cb54671f820ac5dc`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
