# Text.Show

Rendering values as String. The concrete formatting helpers are re-exported by Prelude; import this module for Show itself.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Text.Show`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Show.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.Nat](Data.Nat.md), [Data.Ordering](Data.Ordering.md), [Data.String](Data.String.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Text.Show: rendering values as `String`.

Module scheme (design/base-library.md): `Text.*` is conversion to
and from String/Bytes. The `Show` class is the conversion family's
class; there was no earlier `Show` to move (nothing in `Relation`
or elsewhere carried one), so it lands here in its scheme home,
refined to the bootstrap's minimal shape: `show` renders, with no
`shows`/`showList` zoo.

The concrete conversions below (`nat-to-string`, `int-to-string`,
`char-string`) are the prelude surface's formatting family (their
names are decision D106's dash style and unchanged by the split).
Their digits are raw ASCII bytes (48..=57), so no character
literals are needed.

Instance homes (documented deviation from "instances live with
their type"): the `Show` instances for the primitive carriers live
HERE with the conversion family, not in their Data.* vocabulary
modules. `show`'s result type `String` sits above the bignum and
byte operations in the module order (Data.String's UTF-8 engine
needs the `Nat` operations), so `Show Nat` in Data.Nat would close
an import cycle. `Show` for `Bool`/`Ordering` renders their fixed
text through string literals, which ordinary terms now carry.

</details>

# Types and constructors

# Signatures and methods

<a id="class-53686f77"></a>

## `Show`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">signature</span> <span class="tok-constructor">Show</span> <span class="tok-variable">a</span></code></pre>

[Source](/reference-sources/Text.Show.cinch)

<a id="method-53686f772e73686f77"></a>

## `Show.show`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">show</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Show</span>.<span class="tok-constructor">Show</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>



# Values

<a id="value-636861722d737472696e67"></a>

## `char-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">char-string</span> :: (<span class="tok-constructor">Char</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Show.cinch)

<a id="value-696e742d746f2d737472696e67"></a>

## `int-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-to-string</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Signed decimal for `Int`, built from bignum division so values
beyond i64 format correctly: the magnitude is a `Nat` (`0 - x` for
a negative `x`) and renders as one, after a raw ASCII 45 byte for
the sign.

[Source](/reference-sources/Text.Show.cinch) · line 113

<a id="value-6e61742d64696769742d636f756e74"></a>

## `nat-digit-count`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nat-digit-count</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Show.cinch)

<a id="value-6e61742d6469676974732d696e746f"></a>

## `nat-digits-into`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nat-digits-into</span> :: <span class="tok-keyword">using</span> ((<span class="tok-variable">arg1</span> <span class="tok-operator">+</span> <span class="tok-variable">arg3</span>) <span class="tok-operator">&lt;=</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Bytes</span>.<span class="tok-variable">bytes-len</span> <span class="tok-variable">arg0</span>)) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Bytes</span>) -&gt; ((<span class="tok-variable">arg1</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg2</span> :: <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-variable">arg3</span> :: <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Unit</span>))))</code></pre>

Decimal formatting: digits are raw ASCII bytes (48..=57), so no
character literals are needed. The low `k` digits of `n` fill
[pos, pos + k) from the END, one division per digit; extracting each
digit from the front as `n / 10^(k - 1)` would rebuild the power and
divide the whole number at every digit, quadratic in `k`. The span
lies inside the buffer, and each digit is `48 + n % 10 <= 57`.

[Source](/reference-sources/Text.Show.cinch) · line 89

<a id="value-6e61742d746f2d737472696e67"></a>

## `nat-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nat-to-string</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Text.Show.cinch)

<a id="value-706f773130"></a>

## `pow10`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pow10</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Text.Show.cinch)

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Text.Show.Show_Bool` | `Text.Show.Show Bool` | Implicit |
| `Text.Show.Show_Char` | `Text.Show.Show Char` | Implicit |
| `Text.Show.Show_Int` | `Text.Show.Show Int` | Implicit |
| `Text.Show.Show_Nat` | `Text.Show.Show Nat` | Implicit |
| `Text.Show.Show_Ordering` | `Text.Show.Show Data.Ordering.Ordering` | Implicit |
| `Text.Show.Show_String` | `Text.Show.Show Data.String.String` | Implicit |

**`Text.Show.Show_Bool`**: The fixed text: each renders as its literal constructor spelling
(probed in compiler/tests/string_literals.rs).
**`Text.Show.Show_Nat`**: The decimal rendering agrees with the concrete conversions below
(the class's law shape: `show` is the rendering; asserted over
sample values in compiler/tests/prelude_split.rs).
**`Text.Show.Show_String`**: The raw text: `String` renders as itself (no quoting in the
bootstrap).

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Show.cinch`. Source SHA-256: `0b9010a0b6d7ee66abb481699b59c2cc0493a42f8a45621d087c8bc202db2fe7`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
