# Text.Strconv

Explicit-base and width-aware integer codecs, exact decimal and hexadecimal float parsing and formatting, complex parsing/formatting and Go string/rune quoting.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Strconv</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Strconv/source/">View source</a><a href="/packages/search/?q=%2BText.Strconv">Search module</a><a href="/reference-sources/Text.Strconv.cinch" download="Text.Strconv.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Float](Data.Float.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Relation.Eq](Relation.Eq.md), [Text.Builder](Text.Builder.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Conversions between strings, byte buffers, and basic scalar values:
booleans, signed and unsigned machine integers in bases 2 through 36 (plus
base-0 prefix and underscore syntax), binary32 and binary64 floating-point
numbers (decimal, hexadecimal, and special values with exact IEEE-754
round-to-nearest-even), complex pairs, literal quoting and unquoting, and
Unicode printable/graphic rune classification.

Every fallible conversion returns an explicit Result with a named
StrconvError rather than panicking, and clamped-pair variants expose the
saturated boundary value alongside a range error when overflow occurs.
Byte-appending functions return fresh buffers without mutating caller
prefixes.

</details>

<details class="doc-synopsis"><summary>Synopsis (53 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-StrconvError">StrconvError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">StrconvError</strong></code></li><li><a class="syn-name" href="#v-append-bool">append-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-append-float32">append-float32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-float32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-append-float64">append-float64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-append-int">append-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-append-quote">append-quote</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-quote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-append-quote-rune">append-quote-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-quote-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-append-quote-rune-to-ascii">append-quote-rune-to-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-quote-rune-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-append-quote-rune-to-graphic">append-quote-rune-to-graphic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-quote-rune-to-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-append-quote-to-ascii">append-quote-to-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-quote-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-append-quote-to-graphic">append-quote-to-graphic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-quote-to-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-append-uint">append-uint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">append-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-atoi">atoi</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atoi</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-atoi-with-clamp">atoi-with-clamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">atoi-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code></li><li><a class="syn-name" href="#v-can-backquote">can-backquote</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">can-backquote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-format-bool">format-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">format-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-format-complex128">format-complex128</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">format-complex128</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-format-complex64">format-complex64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">format-complex64</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-format-float32">format-float32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">format-float32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-format-float64">format-float64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">format-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-format-int">format-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">format-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-format-uint">format-uint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">format-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-int-size">int-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">int-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-is-graphic">is-graphic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-is-print">is-print</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">is-print</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-itoa">itoa</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">itoa</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-parse-bool">parse-bool</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-parse-complex128">parse-complex128</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-complex128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code></li><li><a class="syn-name" href="#v-parse-complex128-with-clamp">parse-complex128-with-clamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-complex128-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> ((<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>), <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code></li><li><a class="syn-name" href="#v-parse-complex64">parse-complex64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-complex64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>)</code></li><li><a class="syn-name" href="#v-parse-complex64-with-clamp">parse-complex64-with-clamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-complex64-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> ((<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>), <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code></li><li><a class="syn-name" href="#v-parse-float32">parse-float32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-float32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code></li><li><a class="syn-name" href="#v-parse-float32-with-clamp">parse-float32-with-clamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-float32-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code></li><li><a class="syn-name" href="#v-parse-float64">parse-float64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-parse-float64-with-clamp">parse-float64-with-clamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-float64-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code></li><li><a class="syn-name" href="#v-parse-int">parse-int</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-parse-int-with-clamp">parse-int-with-clamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-int-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code></li><li><a class="syn-name" href="#v-parse-uint">parse-uint</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-parse-uint-with-clamp">parse-uint-with-clamp</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-uint-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code></li><li><a class="syn-name" href="#v-quote">quote</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quote-bytes">quote-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quote-bytes-to-ascii">quote-bytes-to-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quote-bytes-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quote-bytes-to-graphic">quote-bytes-to-graphic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quote-bytes-to-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quote-rune">quote-rune</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quote-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quote-rune-to-ascii">quote-rune-to-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quote-rune-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quote-rune-to-graphic">quote-rune-to-graphic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quote-rune-to-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quote-to-ascii">quote-to-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quote-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quote-to-graphic">quote-to-graphic</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quote-to-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quoted-prefix">quoted-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-strconv-error-message">strconv-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">strconv-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unquote">unquote</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unquote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-unquote-bytes">unquote-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unquote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-unquote-char">unquote-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unquote-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-StrconvError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">StrconvError</strong></code><span class="doc-sig-links"><a href="#t-StrconvError" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

Why a string conversion failed.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-StrconvSyntax"><code><strong class="tok-constructor">StrconvSyntax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a></code></li><li id="k-StrconvRange"><code><strong class="tok-constructor">StrconvRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a></code></li><li id="k-StrconvBadBase"><code><strong class="tok-constructor">StrconvBadBase</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a></code></li><li id="k-StrconvBadBitSize"><code><strong class="tok-constructor">StrconvBadBitSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a></code></li><li id="k-StrconvBadFormat"><code><strong class="tok-constructor">StrconvBadFormat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Strconv/#instances">Text.Strconv</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-append-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-bool" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L219" title="View source at line 219">Source</a></span></div>
<div class="doc-body">

Append "true" or "false" to a byte buffer, returning a fresh buffer.

</div>
</article>

<article class="doc-entry" id="v-append-float32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-float32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-float32" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L726" title="View source at line 726">Source</a></span></div>
<div class="doc-body">

Append the formatted binary32 string to a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-append-float64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-float64" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L720" title="View source at line 720">Source</a></span></div>
<div class="doc-body">

Append the formatted binary64 string to a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-append-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-int" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L442" title="View source at line 442">Source</a></span></div>
<div class="doc-body">

Append the base-b representation of a signed 64-bit integer to a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-append-quote">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-quote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-quote" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1552" title="View source at line 1552">Source</a></span></div>
<div class="doc-body">

Append the double-quoted literal of a string to a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-append-quote-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-quote-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-quote-rune" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1583" title="View source at line 1583">Source</a></span></div>
<div class="doc-body">

Append the single-quoted literal of a character to a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-append-quote-rune-to-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-quote-rune-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-quote-rune-to-ascii" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1587" title="View source at line 1587">Source</a></span></div>
<div class="doc-body">

Append the ASCII-only single-quoted literal of a character to a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-append-quote-rune-to-graphic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-quote-rune-to-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-quote-rune-to-graphic" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1591" title="View source at line 1591">Source</a></span></div>
<div class="doc-body">

Append the graphic-preserving single-quoted literal of a character to a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-append-quote-to-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-quote-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-quote-to-ascii" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1556" title="View source at line 1556">Source</a></span></div>
<div class="doc-body">

Append the ASCII-only double-quoted literal of a string to a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-append-quote-to-graphic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-quote-to-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-quote-to-graphic" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1560" title="View source at line 1560">Source</a></span></div>
<div class="doc-body">

Append the graphic-preserving double-quoted literal of a string to a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-append-uint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">append-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-append-uint" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L451" title="View source at line 451">Source</a></span></div>
<div class="doc-body">

Append the base-b representation of an unsigned 64-bit integer to a byte buffer.

</div>
</article>

<article class="doc-entry" id="v-atoi">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atoi</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-atoi" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L392" title="View source at line 392">Source</a></span></div>
<div class="doc-body">

Parse a base-10 64-bit signed integer.

</div>
</article>

<article class="doc-entry" id="v-atoi-with-clamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">atoi-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code><span class="doc-sig-links"><a href="#v-atoi-with-clamp" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L388" title="View source at line 388">Source</a></span></div>
<div class="doc-body">

Parse a base-10 64-bit signed integer, returning the clamped value alongside any error.

</div>
</article>

<article class="doc-entry" id="v-can-backquote">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">can-backquote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-can-backquote" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1595" title="View source at line 1595">Source</a></span></div>
<div class="doc-body">

Report whether a string can be represented unchanged as a single-line backquoted literal without control characters other than tab.

</div>
</article>

<article class="doc-entry" id="v-format-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">format-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-format-bool" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L215" title="View source at line 215">Source</a></span></div>
<div class="doc-body">

Format a boolean as "true" or "false".

</div>
</article>

<article class="doc-entry" id="v-format-complex128">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">format-complex128</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-format-complex128" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1406" title="View source at line 1406">Source</a></span></div>
<div class="doc-body">

Format a 128-bit complex number (real, imag) as "(a+bi)".

</div>
</article>

<article class="doc-entry" id="v-format-complex64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">format-complex64</strong> :: (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-format-complex64" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1415" title="View source at line 1415">Source</a></span></div>
<div class="doc-body">

Format a 64-bit complex number (real, imag) as "(a+bi)".

</div>
</article>

<article class="doc-entry" id="v-format-float32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">format-float32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-format-float32" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L702" title="View source at line 702">Source</a></span></div>
<div class="doc-body">

Format a binary32 floating-point value according to format specifier and precision.

</div>
</article>

<article class="doc-entry" id="v-format-float64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">format-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-format-float64" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L684" title="View source at line 684">Source</a></span></div>
<div class="doc-body">

Format a binary64 floating-point value according to format specifier and precision.
Supported formats: 'b', 'e', 'E', 'f', 'g', 'G', 'x', 'X'. Negative precision selects shortest round-trip digits.

</div>
</article>

<article class="doc-entry" id="v-format-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">format-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-format-int" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L426" title="View source at line 426">Source</a></span></div>
<div class="doc-body">

Format a signed 64-bit integer in the given base (2..36) using lowercase letters a..z.

</div>
</article>

<article class="doc-entry" id="v-format-uint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">format-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-format-uint" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L420" title="View source at line 420">Source</a></span></div>
<div class="doc-body">

Format an unsigned 64-bit integer in the given base (2..36) using lowercase letters a..z.

</div>
</article>

<article class="doc-entry" id="v-int-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">int-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-int-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Size in bits of the default machine integer width used when bit size 0 is requested.

</div>
</article>

<article class="doc-entry" id="v-is-graphic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-graphic" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1432" title="View source at line 1432">Source</a></span></div>
<div class="doc-body">

Report whether a Unicode scalar is a Unicode graphic character (categories L, M, N, P, S, and Zs).

</div>
</article>

<article class="doc-entry" id="v-is-print">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">is-print</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-is-print" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1428" title="View source at line 1428">Source</a></span></div>
<div class="doc-body">

Report whether a Unicode scalar is printable (letters, marks, numbers, punctuation, symbols, and ASCII space).

</div>
</article>

<article class="doc-entry" id="v-itoa">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">itoa</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-itoa" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L435" title="View source at line 435">Source</a></span></div>
<div class="doc-body">

Format a signed 64-bit integer in base 10.

</div>
</article>

<article class="doc-entry" id="v-parse-bool">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-bool</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-parse-bool" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L208" title="View source at line 208">Source</a></span></div>
<div class="doc-body">

Parse a boolean string accepting 1, t, T, TRUE, true, True, 0, f, F, FALSE, false, False.

</div>
</article>

<article class="doc-entry" id="v-parse-complex128">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-complex128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code><span class="doc-sig-links"><a href="#v-parse-complex128" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1354" title="View source at line 1354">Source</a></span></div>
<div class="doc-body">

Parse a 128-bit complex number of the form N, Ni, or N±Ni (optionally parenthesized).

</div>
</article>

<article class="doc-entry" id="v-parse-complex128-with-clamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-complex128-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> ((<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>), <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code><span class="doc-sig-links"><a href="#v-parse-complex128-with-clamp" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1326" title="View source at line 1326">Source</a></span></div>
<div class="doc-body">

Parse a 128-bit complex number (pair of F64), returning clamped components alongside StrconvRange on overflow.

</div>
</article>

<article class="doc-entry" id="v-parse-complex64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-complex64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>)</code><span class="doc-sig-links"><a href="#v-parse-complex64" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1388" title="View source at line 1388">Source</a></span></div>
<div class="doc-body">

Parse a 64-bit complex number of the form N, Ni, or N±Ni (optionally parenthesized).

</div>
</article>

<article class="doc-entry" id="v-parse-complex64-with-clamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-complex64-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> ((<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>), <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code><span class="doc-sig-links"><a href="#v-parse-complex64-with-clamp" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1360" title="View source at line 1360">Source</a></span></div>
<div class="doc-body">

Parse a 64-bit complex number (pair of F32), returning clamped components alongside StrconvRange on overflow.

</div>
</article>

<article class="doc-entry" id="v-parse-float32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-float32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code><span class="doc-sig-links"><a href="#v-parse-float32" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1316" title="View source at line 1316">Source</a></span></div>
<div class="doc-body">

Parse a binary32 floating-point number with exact single-rounding IEEE-754 semantics.

</div>
</article>

<article class="doc-entry" id="v-parse-float32-with-clamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-float32-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code><span class="doc-sig-links"><a href="#v-parse-float32-with-clamp" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1304" title="View source at line 1304">Source</a></span></div>
<div class="doc-body">

Parse a binary32 float, returning ±Inf alongside StrconvRange on overflow.

</div>
</article>

<article class="doc-entry" id="v-parse-float64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-float64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-parse-float64" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1298" title="View source at line 1298">Source</a></span></div>
<div class="doc-body">

Parse a binary64 floating-point number (decimal, hexadecimal, NaN, or ±Inf/Infinity).

</div>
</article>

<article class="doc-entry" id="v-parse-float64-with-clamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-float64-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code><span class="doc-sig-links"><a href="#v-parse-float64-with-clamp" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1286" title="View source at line 1286">Source</a></span></div>
<div class="doc-body">

Parse a binary64 float, returning ±Inf alongside StrconvRange on overflow.

</div>
</article>

<article class="doc-entry" id="v-parse-int">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-int</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-parse-int" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L382" title="View source at line 382">Source</a></span></div>
<div class="doc-body">

Parse a signed integer in the given base (0, or 2..36) and bit size (0..64).

</div>
</article>

<article class="doc-entry" id="v-parse-int-with-clamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-int-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code><span class="doc-sig-links"><a href="#v-parse-int-with-clamp" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L350" title="View source at line 350">Source</a></span></div>
<div class="doc-body">

Parse a signed integer, returning both the value (clamped to the bit-size minimum or maximum on overflow) and any error.

</div>
</article>

<article class="doc-entry" id="v-parse-uint">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-uint</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-parse-uint" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L344" title="View source at line 344">Source</a></span></div>
<div class="doc-body">

Parse an unsigned integer in the given base (0, or 2..36) and bit size (0..64).

</div>
</article>

<article class="doc-entry" id="v-parse-uint-with-clamp">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-uint-with-clamp</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a>)</code><span class="doc-sig-links"><a href="#v-parse-uint-with-clamp" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L338" title="View source at line 338">Source</a></span></div>
<div class="doc-body">

Parse an unsigned integer, returning both the value (clamped to the bit-size maximum on overflow) and any error.

</div>
</article>

<article class="doc-entry" id="v-quote">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quote" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1540" title="View source at line 1540">Source</a></span></div>
<div class="doc-body">

Return a double-quoted string literal escaping control and non-printable characters.

</div>
</article>

<article class="doc-entry" id="v-quote-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quote-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1528" title="View source at line 1528">Source</a></span></div>
<div class="doc-body">

Return a double-quoted literal representing raw bytes, escaping control, non-printable, and invalid UTF-8 bytes.

</div>
</article>

<article class="doc-entry" id="v-quote-bytes-to-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quote-bytes-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quote-bytes-to-ascii" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1532" title="View source at line 1532">Source</a></span></div>
<div class="doc-body">

Return an ASCII-only double-quoted literal representing raw bytes.

</div>
</article>

<article class="doc-entry" id="v-quote-bytes-to-graphic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quote-bytes-to-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quote-bytes-to-graphic" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1536" title="View source at line 1536">Source</a></span></div>
<div class="doc-body">

Return a double-quoted literal preserving Unicode graphic characters and escaping non-graphic or invalid bytes.

</div>
</article>

<article class="doc-entry" id="v-quote-rune">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quote-rune</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quote-rune" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1571" title="View source at line 1571">Source</a></span></div>
<div class="doc-body">

Return a single-quoted character literal representing a Unicode scalar.

</div>
</article>

<article class="doc-entry" id="v-quote-rune-to-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quote-rune-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quote-rune-to-ascii" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1575" title="View source at line 1575">Source</a></span></div>
<div class="doc-body">

Return an ASCII-only single-quoted character literal representing a Unicode scalar.

</div>
</article>

<article class="doc-entry" id="v-quote-rune-to-graphic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quote-rune-to-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quote-rune-to-graphic" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1579" title="View source at line 1579">Source</a></span></div>
<div class="doc-body">

Return a single-quoted character literal preserving Unicode graphic scalars.

</div>
</article>

<article class="doc-entry" id="v-quote-to-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quote-to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quote-to-ascii" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1544" title="View source at line 1544">Source</a></span></div>
<div class="doc-body">

Return a double-quoted ASCII-only string literal escaping non-ASCII and non-printable characters.

</div>
</article>

<article class="doc-entry" id="v-quote-to-graphic">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quote-to-graphic</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quote-to-graphic" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1548" title="View source at line 1548">Source</a></span></div>
<div class="doc-body">

Return a double-quoted string literal preserving Unicode graphic characters and escaping non-graphic characters.

</div>
</article>

<article class="doc-entry" id="v-quoted-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quoted-prefix" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1757" title="View source at line 1757">Source</a></span></div>
<div class="doc-body">

Return the verbatim quoted string literal at the prefix of `s`.

</div>
</article>

<article class="doc-entry" id="v-strconv-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">strconv-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-strconv-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Human-readable message describing a conversion error.

</div>
</article>

<article class="doc-entry" id="v-unquote">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unquote</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-unquote" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1778" title="View source at line 1778">Source</a></span></div>
<div class="doc-body">

Interpret a single-quoted, double-quoted, or backquoted literal as a UTF-8 string.

</div>
</article>

<article class="doc-entry" id="v-unquote-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unquote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-unquote-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1768" title="View source at line 1768">Source</a></span></div>
<div class="doc-body">

Interpret a single-quoted, double-quoted, or backquoted literal, returning its exact unescaped bytes.

</div>
</article>

<article class="doc-entry" id="v-unquote-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unquote-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-unquote-char" title="Permalink">#</a><a href="/packages/stdlib/Text.Strconv/source/#L1684" title="View source at line 1684">Source</a></span></div>
<div class="doc-body">

Decode the first character or byte escape in a quoted literal body.
Returns the decoded character, whether it requires multibyte UTF-8 encoding, and the remaining tail string.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_StrconvError"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Strconv/#t-StrconvError" title="Text.Strconv.StrconvError">StrconvError</a></code> <span class="doc-inst-meta">Implicit · <code>Text.Strconv.Eq_StrconvError</code></span><div class="doc-child-prose">Structural equality for conversion errors.</div></li>
</ul>
