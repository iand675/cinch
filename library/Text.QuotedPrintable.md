# Text.QuotedPrintable

Quoted-printable byte encoding and decoding with checked line wrapping, escape and malformed-input policies.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.QuotedPrintable</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.QuotedPrintable/source/">View source</a><a href="/packages/search/?q=%2BText.QuotedPrintable">Search module</a><a href="/reference-sources/Text.QuotedPrintable.cinch" download="Text.QuotedPrintable.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Quoted-printable byte codecs with explicit newline modes and continuation state.
Text mode normalizes CR, LF and CRLF to CRLF; binary mode escapes them.
Encoded lines contain at most 76 symbols, including a soft-break marker.
Continuations snapshot unfinished input. Returned byte buffers are fresh and
may be mutated without changing a continuation or another returned buffer.
Decoding accepts lowercase hex, raw high bytes and malformed non-newline
escapes as literals; precise errors retain the successfully decoded prefix.

</details>

<details class="doc-synopsis"><summary>Synopsis (24 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-QuotedPrintableDecoder">QuotedPrintableDecoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">QuotedPrintableDecoder</strong></code></li><li><a class="syn-name" href="#t-QuotedPrintableEncoder">QuotedPrintableEncoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">QuotedPrintableEncoder</strong></code></li><li><a class="syn-name" href="#t-QuotedPrintableError">QuotedPrintableError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">QuotedPrintableError</strong></code></li><li><a class="syn-name" href="#t-QuotedPrintableMode">QuotedPrintableMode</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">QuotedPrintableMode</strong></code></li><li><a class="syn-name" href="#v-quoted-printable-decode-bytes">quoted-printable-decode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-quoted-printable-decode-prefix">quoted-printable-decode-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decode-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a>)</code></li><li><a class="syn-name" href="#v-quoted-printable-decoder">quoted-printable-decoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a></code></li><li><a class="syn-name" href="#v-quoted-printable-decoder-capacity">quoted-printable-decoder-capacity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-capacity</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-quoted-printable-decoder-end">quoted-printable-decoder-end</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-end</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a>)</code></li><li><a class="syn-name" href="#v-quoted-printable-decoder-error">quoted-printable-decoder-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a></code></li><li><a class="syn-name" href="#v-quoted-printable-decoder-feed">quoted-printable-decoder-feed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a>))</code></li><li><a class="syn-name" href="#v-quoted-printable-decoder-finish">quoted-printable-decoder-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a>)</code></li><li><a class="syn-name" href="#v-quoted-printable-decoder-position">quoted-printable-decoder-position</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-position</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-quoted-printable-encode">quoted-printable-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quoted-printable-encode-bytes">quoted-printable-encode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-quoted-printable-encoder">quoted-printable-encoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a></code></li><li><a class="syn-name" href="#v-quoted-printable-encoder-events">quoted-printable-encoder-events</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-events</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>), <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a>)</code></li><li><a class="syn-name" href="#v-quoted-printable-encoder-feed">quoted-printable-encoder-feed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a>)</code></li><li><a class="syn-name" href="#v-quoted-printable-encoder-finish">quoted-printable-encoder-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-quoted-printable-encoder-finish-events">quoted-printable-encoder-finish-events</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-finish-events</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-quoted-printable-encoder-mode">quoted-printable-encoder-mode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a></code></li><li><a class="syn-name" href="#v-quoted-printable-encoder-policy">quoted-printable-encoder-policy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-policy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a></code></li><li><a class="syn-name" href="#v-quoted-printable-error-message">quoted-printable-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-quoted-printable-error-offset">quoted-printable-error-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">quoted-printable-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-QuotedPrintableDecoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">QuotedPrintableDecoder</strong></code><span class="doc-sig-links"><a href="#t-QuotedPrintableDecoder" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L324" title="View source at line 324">Source</a></span></div>
<div class="doc-body">

Immutable decoder continuation owning its unfinished physical-line snapshot.
Feed results count consumed input separately from the failure's precise offset.

</div>
</article>

<article class="doc-entry" id="t-QuotedPrintableEncoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">QuotedPrintableEncoder</strong></code><span class="doc-sig-links"><a href="#t-QuotedPrintableEncoder" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Immutable encoding continuation. Feeds clone its private line before writing.

</div>
</article>

<article class="doc-entry" id="t-QuotedPrintableError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">QuotedPrintableError</strong></code><span class="doc-sig-links"><a href="#t-QuotedPrintableError" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L268" title="View source at line 268">Source</a></span></div>
<div class="doc-body">

Malformed input or an overlong physical line. Prefix APIs retain all
decoded bytes preceding the failure; offsets count original input bytes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-QuotedPrintableInvalidByte"><code><strong class="tok-constructor">QuotedPrintableInvalidByte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a></code></li><li id="k-QuotedPrintableInvalidHex"><code><strong class="tok-constructor">QuotedPrintableInvalidHex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a></code></li><li id="k-QuotedPrintableIncompleteEscape"><code><strong class="tok-constructor">QuotedPrintableIncompleteEscape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a></code></li><li id="k-QuotedPrintableInvalidSoftBreak"><code><strong class="tok-constructor">QuotedPrintableInvalidSoftBreak</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a></code></li><li id="k-QuotedPrintableLineTooLong"><code><strong class="tok-constructor">QuotedPrintableLineTooLong</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-QuotedPrintableMode">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">QuotedPrintableMode</strong></code><span class="doc-sig-links"><a href="#t-QuotedPrintableMode" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Newline policy for subsequent input; mode changes preserve pending bytes and CR carry.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-QuotedPrintableText"><code><strong class="tok-constructor">QuotedPrintableText</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a></code></li><li id="k-QuotedPrintableBinary"><code><strong class="tok-constructor">QuotedPrintableBinary</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-quoted-printable-decode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-decode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L551" title="View source at line 551">Source</a></span></div>
<div class="doc-body">

Return fresh decoded bytes or the first failure under the decoder
acceptance rules. Lowercase hex and malformed non-newline escapes are accepted.
Use decode-prefix when the successfully decoded bytes before an error matter.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-decode-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decode-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a>)</code><span class="doc-sig-links"><a href="#v-quoted-printable-decode-prefix" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L542" title="View source at line 542">Source</a></span></div>
<div class="doc-body">

Decode a complete buffer to fresh bytes and an optional first error.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-decoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-decoder" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L334" title="View source at line 334">Source</a></span></div>
<div class="doc-body">

Begin independent decoding with original offsets starting at zero.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-decoder-capacity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-capacity</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-decoder-capacity" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L344" title="View source at line 344">Source</a></span></div>
<div class="doc-body">

Number of bytes that may be fetched without reading past the decoder's
current physical-line buffer capacity. Failed decoders request no input.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-decoder-end">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-end</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a>)</code><span class="doc-sig-links"><a href="#v-quoted-printable-decoder-end" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L531" title="View source at line 531">Source</a></span></div>
<div class="doc-body">

End an unfinished physical line, distinguishing genuine EOF from an
interrupted source. A codec error may supersede a pending source failure.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-decoder-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-decoder-error" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L338" title="View source at line 338">Source</a></span></div>
<div class="doc-body">

Observe the first sticky codec failure without advancing the continuation.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-decoder-feed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a>))</code><span class="doc-sig-links"><a href="#v-quoted-printable-decoder-feed" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L523" title="View source at line 523">Source</a></span></div>
<div class="doc-body">

Decode complete physical lines from a feed, returning fresh prefix bytes,
the consumed input count, and an advanced immutable continuation. Input after
a terminal error remains unconsumed; previously returned buffers never alias state.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-decoder-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a>)</code><span class="doc-sig-links"><a href="#v-quoted-printable-decoder-finish" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L538" title="View source at line 538">Source</a></span></div>
<div class="doc-body">

Finish at genuine EOF, preserving decoded prefix bytes before a final error.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-decoder-position">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-decoder-position</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableDecoder" title="Text.QuotedPrintable.QuotedPrintableDecoder">QuotedPrintableDecoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-decoder-position" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L351" title="View source at line 351">Source</a></span></div>
<div class="doc-body">

Original input bytes consumed so far, including the unfinished physical line.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L261" title="View source at line 261">Source</a></span></div>
<div class="doc-body">

Encode a complete buffer as ASCII text; all possible input bytes are representable.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-encode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-encode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L256" title="View source at line 256">Source</a></span></div>
<div class="doc-body">

Encode a complete buffer as fresh ASCII bytes using an explicit newline policy.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-encoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-encoder" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

Begin encoding with empty line state under an explicit newline policy.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-encoder-events">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-events</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>), <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a>)</code><span class="doc-sig-links"><a href="#v-quoted-printable-encoder-events" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

Encode a feed as complete sink-write events and an advanced continuation.
Each event carries the input prefix reportable if its sink write fails.
No event or retained state aliases the caller's mutable input.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-encoder-feed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a>)</code><span class="doc-sig-links"><a href="#v-quoted-printable-encoder-feed" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L247" title="View source at line 247">Source</a></span></div>
<div class="doc-body">

Encode complete lines from one feed and retain only the unfinished private line.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-encoder-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-encoder-finish" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L252" title="View source at line 252">Source</a></span></div>
<div class="doc-body">

Finish an immutable continuation and return the independent final encoded bytes.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-encoder-finish-events">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-finish-events</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-quoted-printable-encoder-finish-events" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L216" title="View source at line 216">Source</a></span></div>
<div class="doc-body">

Flush the final line as fresh events, quoting its last space or tab.
The zero-length final event records a logical flush; byte streams may
treat it as an inert empty write under their existing protocol.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-encoder-mode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-encoder-mode" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Change the policy for later feeds without flushing or changing pending input.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-encoder-policy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-encoder-policy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableEncoder" title="Text.QuotedPrintable.QuotedPrintableEncoder">QuotedPrintableEncoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableMode" title="Text.QuotedPrintable.QuotedPrintableMode">QuotedPrintableMode</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-encoder-policy" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Observe the policy used for subsequent feeds without advancing or copying state.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L308" title="View source at line 308">Source</a></span></div>
<div class="doc-body">

Human-readable diagnostics preserving the exact offending byte or transport suffix.

</div>
</article>

<article class="doc-entry" id="v-quoted-printable-error-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">quoted-printable-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.QuotedPrintable/#t-QuotedPrintableError" title="Text.QuotedPrintable.QuotedPrintableError">QuotedPrintableError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-quoted-printable-error-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.QuotedPrintable/source/#L281" title="View source at line 281">Source</a></span></div>
<div class="doc-body">

The absolute original input position associated with a codec failure.

</div>
</article>
