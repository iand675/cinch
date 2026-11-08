# Data.Bool

Boolean operations and their comparison instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Bool`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Bool.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Hash](Data.Hash.md), [Text.Codec](Text.Codec.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Bool: the booleans and their strict operations.

`Bool` is a core runtime type; the `cinch.prim.*Bool` foreign
imports are the machine operations. `&&` and `||` are strict
functions in the bootstrap slice; the surface short-circuiting form
lands with the effects correction (primitive types specification
section 6).

Module scheme (design/base-library.md): the type's home carries its
instance family, including the `Text.Codec` instance (the A6
codec generator's BOOLEAN column is exactly one byte, 0 or 1).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-2626"></a>

## `&&`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">&amp;&amp;</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

These are strict functions in the bootstrap slice; the surface `&&`
and `||` short-circuit once that deviation is corrected (primitive
types specification, section 6).

[Source](/reference-sources/Data.Bool.cinch) · line 32

<a id="value-616e64"></a>

## `and`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">and</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

The strict boolean algebra (primitive types specification, section 6).

[Source](/reference-sources/Data.Bool.cinch) · line 42

<a id="value-626f6f6c"></a>

## `bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bool</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-variable">a</span>)))</code></pre>

The boolean fold: `bool f t b` is `t` when `b` is `True` and `f`
otherwise (`Data.Bool.bool`), the elimination form matching the
constructors' order.

[Source](/reference-sources/Data.Bool.cinch) · line 54

<a id="value-6e6f74"></a>

## `not`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">not</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Bool.cinch)

<a id="value-6f72"></a>

## `or`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">or</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Bool.cinch)

<a id="value-7072696d2d616e642d626f6f6c"></a>

## `prim-and-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prim-and-bool</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Bool.cinch)

<a id="value-7072696d2d65712d626f6f6c"></a>

## `prim-eq-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prim-eq-bool</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Bool.cinch)

<a id="value-7072696d2d6e652d626f6f6c"></a>

## `prim-ne-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prim-ne-bool</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Bool.cinch)

<a id="value-7072696d2d6e6f742d626f6f6c"></a>

## `prim-not-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prim-not-bool</span> :: (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Bool.cinch)

<a id="value-7072696d2d6f722d626f6f6c"></a>

## `prim-or-bool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">prim-or-bool</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Bool.cinch)

<a id="value-786f72"></a>

## `xor`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">xor</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Bool.cinch)

<a id="value-7c7c"></a>

## `||`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-operator">||</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Bool.cinch)

# Operator fixities

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">infixr</span> <span class="tok-number">2</span> <span class="tok-operator">&amp;&amp;</span>
<span class="tok-keyword">infixr</span> <span class="tok-number">1</span> <span class="tok-operator">||</span></code></pre>

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Bool.Codec_Bool` | `Text.Codec.Codec Bool` | Implicit |
| `Data.Bool.Eq_Bool` | `Relation.Eq.Eq Bool` | Implicit |
| `Data.Bool.Hash_Bool` | `Data.Hash.Hash Bool` | Implicit |
| `Data.Bool.JsonCodec_Bool` | `Text.Codec.JsonCodec Bool` | Implicit |
| `Data.Bool.Ord_Bool` | `Relation.Ord.Ord Bool` | Implicit |

**`Data.Bool.Codec_Bool`**: The A6 column codec for booleans (modules specification section
11's `CodecGenerator.derive` vocabulary, decision D65's covered
set): one byte, 0 for False and 1 for True.
**`Data.Bool.Hash_Bool`**: Two buckets (instances live with their type).
**`Data.Bool.JsonCodec_Bool`**: The A6 JSON column codec for booleans (decision D65's covered
set): `JsonCodec`'s round-trip law (`decode-json (encode-json x)`
is `Ok x`).

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Bool.cinch`. Source SHA-256: `b5d776700b7c67f7ad33736bfa1d78edc8d8e94070852c59b22175b637fd81f5`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
