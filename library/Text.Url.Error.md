# Text.Url.Error

Structured URL failures with original byte offsets, context, diagnostics and explicit causes.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Url.Error</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Url.Error/source/">View source</a><a href="/packages/search/?q=%2BText.Url.Error">Search module</a><a href="/reference-sources/Text.Url.Error.cinch" download="Text.Url.Error.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Byte-preserving URL failures and typed contextual errors.
URL failures own their diagnostic byte detail and expose copied buffers.
Contextual errors retain a typed cause, an operation and a copied original
input. Temporary and timeout classification is explicit for custom causes;
pure parsing and escaping failures use false for both classifications.

</details>

<details class="doc-synopsis" open><summary>Synopsis (18 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-UrlError">UrlError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlError</strong> <span class="tok-variable">cause</span></code></li><li><a class="syn-name" href="#t-UrlFailure">UrlFailure</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlFailure</strong></code></li><li><a class="syn-name" href="#t-UrlFailureKind">UrlFailureKind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlFailureKind</strong></code></li><li><a class="syn-name" href="#v-url-error">url-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span></code></li><li><a class="syn-name" href="#v-url-error-cause">url-error-cause</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-cause</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">cause</span></code></li><li><a class="syn-name" href="#v-url-error-input">url-error-input</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-input</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-error-message">url-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-error-operation">url-error-operation</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-operation</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-error-render">url-error-render</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-render</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. (<span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-error-temporary">url-error-temporary</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-temporary</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-error-timeout">url-error-timeout</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-error-timeout</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-url-failure">url-failure</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a></code></li><li><a class="syn-name" href="#v-url-failure-detail">url-failure-detail</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure-detail</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-url-failure-kind">url-failure-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li><a class="syn-name" href="#v-url-failure-message">url-failure-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-url-failure-offset">url-failure-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-url-failure-shift">url-failure-shift</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-failure-shift</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a></code></li><li><a class="syn-name" href="#v-url-quote-bytes">url-quote-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-quote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-UrlError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlError</strong> <span class="tok-variable">cause</span></code><span class="doc-sig-links"><a href="#t-UrlError" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L158" title="View source at line 158">Source</a></span></div>
<div class="doc-body">

A typed cause and the original input for one named URL operation.

</div>
</article>

<article class="doc-entry" id="t-UrlFailure">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlFailure</strong></code><span class="doc-sig-links"><a href="#t-UrlFailure" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

An immutable failure class, component-relative byte offset and exact copied byte detail.

</div>
</article>

<article class="doc-entry" id="t-UrlFailureKind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UrlFailureKind</strong></code><span class="doc-sig-links"><a href="#t-UrlFailureKind" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

A precise class of malformed URL, percent escape or query input.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-UrlInvalidEscape"><code><strong class="tok-constructor">UrlInvalidEscape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlInvalidHost"><code><strong class="tok-constructor">UrlInvalidHost</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlControlByte"><code><strong class="tok-constructor">UrlControlByte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlMissingScheme"><code><strong class="tok-constructor">UrlMissingScheme</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlInvalidRequest"><code><strong class="tok-constructor">UrlInvalidRequest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlEmptyRequest"><code><strong class="tok-constructor">UrlEmptyRequest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlPathColon"><code><strong class="tok-constructor">UrlPathColon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlInvalidPort"><code><strong class="tok-constructor">UrlInvalidPort</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlMissingBracket"><code><strong class="tok-constructor">UrlMissingBracket</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlInvalidIpLiteral"><code><strong class="tok-constructor">UrlInvalidIpLiteral</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlInvalidUserinfo"><code><strong class="tok-constructor">UrlInvalidUserinfo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlQuerySemicolon"><code><strong class="tok-constructor">UrlQuerySemicolon</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li><li id="k-UrlTooManyParameters"><code><strong class="tok-constructor">UrlTooManyParameters</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-url-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span></code><span class="doc-sig-links"><a href="#v-url-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L163" title="View source at line 163">Source</a></span></div>
<div class="doc-body">

Wrap any typed cause with copied input and caller-supplied temporary/timeout flags.

</div>
</article>

<article class="doc-entry" id="v-url-error-cause">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-cause</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">cause</span></code><span class="doc-sig-links"><a href="#v-url-error-cause" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L175" title="View source at line 175">Source</a></span></div>
<div class="doc-body">

Unwrap the original typed cause without erasing its type or information.

</div>
</article>

<article class="doc-entry" id="v-url-error-input">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-input</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-error-input" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L171" title="View source at line 171">Source</a></span></div>
<div class="doc-body">

Return a fresh copy of the original operation input, without requiring valid UTF-8.

</div>
</article>

<article class="doc-entry" id="v-url-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

Render a URL parsing or escaping context with its structured built-in failure.

</div>
</article>

<article class="doc-entry" id="v-url-error-operation">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-operation</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-error-operation" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Observe the operation name associated with the failure.

</div>
</article>

<article class="doc-entry" id="v-url-error-render">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-render</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. (<span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-error-render" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Render any typed contextual error using its supplied cause formatter.

</div>
</article>

<article class="doc-entry" id="v-url-error-temporary">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-temporary</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-error-temporary" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Observe explicit temporary classification; pure parsing failures are not temporary.

</div>
</article>

<article class="doc-entry" id="v-url-error-timeout">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-error-timeout</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">cause</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlError" title="Text.Url.Error.UrlError">UrlError</a> <span class="tok-variable">cause</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-error-timeout" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

Observe explicit timeout classification; pure parsing failures do not time out.

</div>
</article>

<article class="doc-entry" id="v-url-failure">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a></code><span class="doc-sig-links"><a href="#v-url-failure" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Construct a byte-precise failure without losing non-UTF-8 detail.

</div>
</article>

<article class="doc-entry" id="v-url-failure-detail">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure-detail</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-url-failure-detail" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Return a fresh copy of the exact offending bytes, including non-UTF-8 bytes.

</div>
</article>

<article class="doc-entry" id="v-url-failure-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailureKind" title="Text.Url.Error.UrlFailureKind">UrlFailureKind</a></code><span class="doc-sig-links"><a href="#v-url-failure-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Observe the named failure class.

</div>
</article>

<article class="doc-entry" id="v-url-failure-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-failure-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

Render a named failure while retaining its raw detail separately for programmatic handling.

</div>
</article>

<article class="doc-entry" id="v-url-failure-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-url-failure-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Observe the zero-based byte position within the component being processed.

</div>
</article>

<article class="doc-entry" id="v-url-failure-shift">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-failure-shift</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Url/#t-UrlFailure" title="Text.Url.Error.UrlFailure">UrlFailure</a></code><span class="doc-sig-links"><a href="#v-url-failure-shift" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Rebase a component-relative failure into a larger original byte interval.

</div>
</article>

<article class="doc-entry" id="v-url-quote-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-quote-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-url-quote-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Error/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

Produce an ASCII byte-exact quoted diagnostic; non-ASCII bytes use hexadecimal escapes.

</div>
</article>
