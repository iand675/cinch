# Data.Char

Unicode scalar values and checked character/code conversions, with range information for safe use.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Char`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Char.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Hash](Data.Hash.md), [Data.Option](Data.Option.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Char: Unicode scalar values.

`Char` is a core runtime type (primitive types specification
section 1): one scalar value. `u32-to-char` rejects values outside
the scalar range (surrogates and above 0x10FFFF) as `None`.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-636861722d636f6465"></a>

## `char-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">char-code</span> :: (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">InRange</span> <span class="tok-number">0</span> <span class="tok-number">1114111</span> <span class="tok-constructor">Nat</span>))</code></pre>

The scalar value as a `Nat`: the conversion the text and data
homes all spelled out by hand before this was its home. A scalar
value is at most 0x10FFFF, and the result says so.

[Source](/reference-sources/Data.Char.cinch) · line 37

<a id="value-636861722d66726f6d2d636f6465"></a>

## `char-from-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">char-from-code</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Char</span>))</code></pre>

The inverse, partial exactly as `u32-to-char` is: a value outside
the scalar range (a surrogate, or above 0x10FFFF) is `None`. The
range test comes first, so a value past `U32` is `None` rather than
a wrapped scalar.

[Source](/reference-sources/Data.Char.cinch) · line 44

<a id="value-636861722d746f2d753332"></a>

## `char-to-u32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">char-to-u32</span> :: ((<span class="tok-variable">a0</span> :: <span class="tok-constructor">Char</span>) -&gt; <span class="tok-variable">refined</span> (((<span class="tok-number">0</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">r</span> <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">1114111</span>) <span class="tok-operator">&amp;&amp;</span> <span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">a0</span>)) <span class="tok-constructor">U32</span>)</code></pre>

Primitive types specification, section 6: Char values are Unicode
scalar values, in code point order. `u32-to-char` rejects surrogates
and values above 0x10FFFF.

[Source](/reference-sources/Data.Char.cinch) · line 25

<a id="value-64696769742d76616c7565"></a>

## `digit-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">digit-value</span> :: (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

`Some 0..9` for the ASCII digits, `None` otherwise (the caller
has already decided what a non-digit means).

[Source](/reference-sources/Data.Char.cinch) · line 62

<a id="value-65712d63686172"></a>

## `eq-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eq-char</span> :: (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-67652d63686172"></a>

## `ge-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ge-char</span> :: (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-67742d63686172"></a>

## `gt-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gt-char</span> :: (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-6865782d64696769742d76616c7565"></a>

## `hex-digit-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-digit-value</span> :: (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

`Some 0..15` for `0`-`9`, `a`-`f` and `A`-`F`; `None` otherwise.

[Source](/reference-sources/Data.Char.cinch) · line 68

<a id="value-69732d61736369692d616c706861"></a>

## `is-ascii-alpha`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-ascii-alpha</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-69732d61736369692d616c7068612d6e756d65726963"></a>

## `is-ascii-alpha-numeric`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-ascii-alpha-numeric</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-69732d61736369692d6469676974"></a>

## `is-ascii-digit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-ascii-digit</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

The ASCII digit and hex-digit test/value pairs. Every text and
data home that parses numeric text had written its own (under
`is-digit-char`, `digit-of`, `is-hex`, `hex-of`, ...), and this is
their one home. The names say ASCII because that is the whole
class they accept: there is no Unicode digit or decimal-digit
database here, and none is claimed.

[Source](/reference-sources/Data.Char.cinch) · line 55

<a id="value-69732d61736369692d6865782d6469676974"></a>

## `is-ascii-hex-digit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-ascii-hex-digit</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-69732d61736369692d6c6f776572"></a>

## `is-ascii-lower`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-ascii-lower</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-69732d61736369692d7370616365"></a>

## `is-ascii-space`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-ascii-space</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

ASCII whitespace: space, tab, LF, VT, FF, CR (the ASCII subset of
base's `isSpace`; the other three controls are the whitespace
classes that exist outside ASCII).

[Source](/reference-sources/Data.Char.cinch) · line 111

<a id="value-69732d61736369692d7570706572"></a>

## `is-ascii-upper`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-ascii-upper</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

A-Z is `65..90`, `a`-`z` is `97..122` in code point order.

[Source](/reference-sources/Data.Char.cinch) · line 92

<a id="value-6c652d63686172"></a>

## `le-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">le-char</span> :: (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-6c742d63686172"></a>

## `lt-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lt-char</span> :: (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-6e652d63686172"></a>

## `ne-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ne-char</span> :: (<span class="tok-constructor">Char</span> -&gt; (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-746f2d6c6f7765722d6173636969"></a>

## `to-lower-ascii`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-lower-ascii</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>)</code></pre>

[Source](/reference-sources/Data.Char.cinch)

<a id="value-746f2d75707065722d6173636969"></a>

## `to-upper-ascii`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-upper-ascii</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Char</span>)</code></pre>

The ASCII case fold. A scalar outside the affected range is
returned unchanged, which is the documented behaviour for every
non-ASCII scalar and every non-letter. `char-from-code` cannot
fail on a code an upper/lower test just admitted, so the `None`
arm returns the input unchanged (never a magic placeholder).

[Source](/reference-sources/Data.Char.cinch) · line 121

<a id="value-7533322d746f2d63686172"></a>

## `u32-to-char`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">u32-to-char</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Char</span>))</code></pre>

[Source](/reference-sources/Data.Char.cinch)

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Char.Eq_Char` | `Relation.Eq.Eq Char` | Implicit |
| `Data.Char.Hash_Char` | `Data.Hash.Hash Char` | Implicit |
| `Data.Char.Ord_Char` | `Relation.Ord.Ord Char` | Implicit |

**`Data.Char.Hash_Char`**: The scalar value is the digest (instances live with their type).

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Char.cinch`. Source SHA-256: `217d21faf8b93a9606f779e720aca8c012fdea1ecbc745a95298e96896d783fb`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
