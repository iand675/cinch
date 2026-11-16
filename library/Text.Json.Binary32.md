# Text.Json.Binary32

Exact decimal rounding and shortest finite binary32 JSON coefficients, with preserved signed zero and midpoint comparisons.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Json.Binary32`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Json.Binary32.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Json.Binary32](Text.Json.Binary32.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Exact decimal rounding and shortest JSON coefficients for finite binary32 values.
Parsing consumes an already validated decimal spelling. Decimal midpoint
comparisons repair binary64 double rounding without accumulating input bignums.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-62696e61727933322d66726f6d2d646563696d616c"></a>

## `binary32-from-decimal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary32-from-decimal</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">F32</span>)</code></pre>

Round a validated nonempty JSON decimal exactly to binary32, including signed underflow and overflow.

[Source](/reference-sources/Text.Json.Binary32.cinch) · line 118

<a id="value-62696e61727933322d746f2d646563696d616c"></a>

## `binary32-to-decimal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">binary32-to-decimal</span> :: (<span class="tok-constructor">F32</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Format a finite binary32 as its shortest exact JSON decimal, preserving negative zero.

[Source](/reference-sources/Text.Json.Binary32.cinch) · line 189

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Json/Binary32.cinch`. Source SHA-256: `ad2ac932fc7da1befe03fefc9fdf84b98c7276f7af996ed9d3617bfd635d2693`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
