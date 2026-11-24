# Text.Pem

Owned byte-preserving envelope blocks, persistent headers, exact malformed-input recovery and bounded payload encoding.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Pem</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Pem/source/">View source</a><a href="/packages/search/?q=%2BText.Pem">Search module</a><a href="/reference-sources/Text.Pem.cinch" download="Text.Pem.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Effect.Stream](Effect.Stream.md), [Text.Base64.Encoding](Text.Base64.Encoding.md).

<details class="doc-notes"><summary>Module notes</summary>

PEM envelopes with immutable owned byte labels, headers and payloads.
Decoding preserves arbitrary byte metadata, recovers the next well-formed
envelope, and returns an independently owned remainder. Encoding validates
header keys before output, orders headers deterministically, and wraps the
standard padded Base64 alphabet at 64 columns. Writers retain continuations.

</details>

<details class="doc-synopsis"><summary>Synopsis (20 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-PemBlock">PemBlock</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PemBlock</strong></code></li><li><a class="syn-name" href="#t-PemError">PemError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PemError</strong></code></li><li><a class="syn-name" href="#t-PemHeader">PemHeader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PemHeader</strong></code></li><li><a class="syn-name" href="#v-pem-block">pem-block</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code></li><li><a class="syn-name" href="#v-pem-block-bytes">pem-block-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-block-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-pem-block-headers">pem-block-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-block-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a></code></li><li><a class="syn-name" href="#v-pem-block-type">pem-block-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-block-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-pem-decode">pem-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-pem-encode-bytes">pem-encode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-pem-error-message">pem-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pem-find-header">pem-find-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-find-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-pem-header">pem-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a></code></li><li><a class="syn-name" href="#v-pem-header-key">pem-header-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-header-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-pem-header-value">pem-header-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-header-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-pem-with-bytes">pem-with-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-with-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code></li><li><a class="syn-name" href="#v-pem-with-header">pem-with-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-with-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code></li><li><a class="syn-name" href="#v-pem-with-headers">pem-with-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-with-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code></li><li><a class="syn-name" href="#v-pem-with-type">pem-with-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-with-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code></li><li><a class="syn-name" href="#v-pem-without-header">pem-without-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-without-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code></li><li><a class="syn-name" href="#v-pem-write">pem-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pem-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-PemBlock">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PemBlock</strong></code><span class="doc-sig-links"><a href="#t-PemBlock" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

An immutable envelope; labels and metadata may contain arbitrary non-UTF8 bytes.

</div>
</article>

<article class="doc-entry" id="t-PemError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PemError</strong></code><span class="doc-sig-links"><a href="#t-PemError" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Encoding validation or transport failure, with no implicit sink closure.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PemInvalidHeaderKey"><code><strong class="tok-constructor">PemInvalidHeaderKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a></code></li><li id="k-PemOutputError"><code><strong class="tok-constructor">PemOutputError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-StreamError" title="Effect.Stream.StreamError">StreamError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PemHeader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PemHeader</strong></code><span class="doc-sig-links"><a href="#t-PemHeader" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

Immutable header metadata; construction and byte observations copy caller storage.

</div>
</article>

# Values

<article class="doc-entry" id="v-pem-block">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code><span class="doc-sig-links"><a href="#v-pem-block" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

Construct an immutable envelope; duplicate byte keys use the final supplied value.

</div>
</article>

<article class="doc-entry" id="v-pem-block-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-block-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pem-block-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

Observe an independently mutable copy of the decoded payload.

</div>
</article>

<article class="doc-entry" id="v-pem-block-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-block-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a></code><span class="doc-sig-links"><a href="#v-pem-block-headers" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Observe immutable unique headers in byte-lexicographic key order.

</div>
</article>

<article class="doc-entry" id="v-pem-block-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-block-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pem-block-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Observe an independently mutable copy of the exact label bytes.

</div>
</article>

<article class="doc-entry" id="v-pem-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-pem-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L579" title="View source at line 579">Source</a></span></div>
<div class="doc-body">

Decode the next well-formed envelope and return a fresh independently mutable remainder.
Candidates start at a line boundary relative to recovery's current slice. Malformed
candidates are skipped by END trailer; absence or terminal malformed metadata returns
None and a fresh copy of the complete original input. Labels and headers preserve bytes.

</div>
</article>

<article class="doc-entry" id="v-pem-encode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pem-encode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L683" title="View source at line 683">Source</a></span></div>
<div class="doc-body">

Encode a complete fresh envelope or refuse a colon-bearing key without returning partial output.
Arbitrary labels and values are retained exactly, including line breaks; only header keys
containing a colon are refused. Empty payloads add no body line. Metadata is deterministic.

</div>
</article>

<article class="doc-entry" id="v-pem-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pem-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Explain a header refusal or identify a retained underlying stream failure.

</div>
</article>

<article class="doc-entry" id="v-pem-find-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-find-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pem-find-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Find an exact case-sensitive byte key, returning a fresh mutable value when present.

</div>
</article>

<article class="doc-entry" id="v-pem-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a></code><span class="doc-sig-links"><a href="#v-pem-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Construct immutable byte metadata without validating or discarding unusual keys or values.

</div>
</article>

<article class="doc-entry" id="v-pem-header-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-header-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pem-header-key" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Observe an independently mutable copy of a header key.

</div>
</article>

<article class="doc-entry" id="v-pem-header-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-header-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pem-header-value" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Observe an independently mutable copy of a header value.

</div>
</article>

<article class="doc-entry" id="v-pem-with-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-with-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code><span class="doc-sig-links"><a href="#v-pem-with-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L211" title="View source at line 211">Source</a></span></div>
<div class="doc-body">

Replace payload bytes with an owned snapshot while preserving metadata and the original block.

</div>
</article>

<article class="doc-entry" id="v-pem-with-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-with-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code><span class="doc-sig-links"><a href="#v-pem-with-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

Replace or add a header persistently; both blocks retain immutable independently observable metadata.

</div>
</article>

<article class="doc-entry" id="v-pem-with-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-with-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemHeader" title="Text.Pem.PemHeader">PemHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code><span class="doc-sig-links"><a href="#v-pem-with-headers" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L203" title="View source at line 203">Source</a></span></div>
<div class="doc-body">

Replace all metadata persistently, with sorted last-key-wins semantics and immutable payload sharing.

</div>
</article>

<article class="doc-entry" id="v-pem-with-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-with-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code><span class="doc-sig-links"><a href="#v-pem-with-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L207" title="View source at line 207">Source</a></span></div>
<div class="doc-body">

Replace a label with an owned snapshot, preserving the original block and shared immutable fields.

</div>
</article>

<article class="doc-entry" id="v-pem-without-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-without-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a></code><span class="doc-sig-links"><a href="#v-pem-without-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L185" title="View source at line 185">Source</a></span></div>
<div class="doc-body">

Remove a byte key persistently without copying unrelated private payload storage.

</div>
</article>

<article class="doc-entry" id="v-pem-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pem-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Pem/#t-PemBlock" title="Text.Pem.PemBlock">PemBlock</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Pem/#t-PemError" title="Text.Pem.PemError">PemError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteWriter" title="Effect.Stream.ByteWriter">ByteWriter</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-pem-write" title="Permalink">#</a><a href="/packages/stdlib/Text.Pem/source/#L753" title="View source at line 753">Source</a></span></div>
<div class="doc-body">

Write an envelope with bounded payload storage and explicit continuation ownership.
Validate all keys before any callback. Positive short writes are retried; zero progress,
invalid counts and sink errors preserve the advanced writer and never replay accepted
output. Chunk boundaries are unspecified. This operation never flushes or closes its sink.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Pem.cinch`](/packages/stdlib/Text.Pem/source/). Source SHA-256: `9478c3c41a71436180df02c7ac175c309f790f39d131440895f0a81c7f733efd`.
