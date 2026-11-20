# Text.Url.Error

Structured URL failures with original byte offsets, context, diagnostics and explicit causes.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Url.Error`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Url.Error.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md).

Non-embedded source closure: [Text.Url.Error](Text.Url.Error.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Byte-preserving URL failures and typed contextual errors.
URL failures own their diagnostic byte detail and expose copied buffers.
Contextual errors retain a typed cause, an operation and a copied original
input. Temporary and timeout classification is explicit for custom causes;
pure parsing and escaping failures use false for both classifications.

</details>

# Types and constructors

<a id="type-55726c4572726f72"></a>

## `UrlError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlError</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>)</code></pre>

A typed cause and the original input for one named URL operation.

[Source](/reference-sources/Text.Url.Error.cinch) · line 158

<a id="type-55726c4661696c757265"></a>

## `UrlFailure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlFailure</span></code></pre>

An immutable failure class, component-relative byte offset and exact copied byte detail.

[Source](/reference-sources/Text.Url.Error.cinch) · line 52

<a id="type-55726c4661696c7572654b696e64"></a>

## `UrlFailureKind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlFailureKind</span></code></pre>

A precise class of malformed URL, percent escape or query input.

[Source](/reference-sources/Text.Url.Error.cinch) · line 23

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidEscape</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidHost</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlControlByte</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlMissingScheme</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidRequest</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlEmptyRequest</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlPathColon</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidPort</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlMissingBracket</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidIpLiteral</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlInvalidUserinfo</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlQuerySemicolon</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlTooManyParameters</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span></code></pre>

# Values

<a id="value-75726c2d6572726f72"></a>

## `url-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-variable">cause</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>))))))</code></pre>

Wrap any typed cause with copied input and caller-supplied temporary/timeout flags.

[Source](/reference-sources/Text.Url.Error.cinch) · line 163

<a id="value-75726c2d6572726f722d6361757365"></a>

## `url-error-cause`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-cause</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-variable">cause</span>)</code></pre>

Unwrap the original typed cause without erasing its type or information.

[Source](/reference-sources/Text.Url.Error.cinch) · line 175

<a id="value-75726c2d6572726f722d696e707574"></a>

## `url-error-input`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-input</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a fresh copy of the original operation input, without requiring valid UTF-8.

[Source](/reference-sources/Text.Url.Error.cinch) · line 171

<a id="value-75726c2d6572726f722d6d657373616765"></a>

## `url-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-message</span> :: ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a URL parsing or escaping context with its structured built-in failure.

[Source](/reference-sources/Text.Url.Error.cinch) · line 193

<a id="value-75726c2d6572726f722d6f7065726174696f6e"></a>

## `url-error-operation`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-operation</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Observe the operation name associated with the failure.

[Source](/reference-sources/Text.Url.Error.cinch) · line 167

<a id="value-75726c2d6572726f722d72656e646572"></a>

## `url-error-render`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-render</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">cause</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Render any typed contextual error using its supplied cause formatter.

[Source](/reference-sources/Text.Url.Error.cinch) · line 187

<a id="value-75726c2d6572726f722d74656d706f72617279"></a>

## `url-error-temporary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-temporary</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Observe explicit temporary classification; pure parsing failures are not temporary.

[Source](/reference-sources/Text.Url.Error.cinch) · line 179

<a id="value-75726c2d6572726f722d74696d656f7574"></a>

## `url-error-timeout`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-error-timeout</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">cause</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlError</span> <span class="tok-variable">cause</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Observe explicit timeout classification; pure parsing failures do not time out.

[Source](/reference-sources/Text.Url.Error.cinch) · line 183

<a id="value-75726c2d6661696c757265"></a>

## `url-failure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>)))</code></pre>

Construct a byte-precise failure without losing non-UTF-8 detail.

[Source](/reference-sources/Text.Url.Error.cinch) · line 62

<a id="value-75726c2d6661696c7572652d64657461696c"></a>

## `url-failure-detail`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure-detail</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return a fresh copy of the exact offending bytes, including non-UTF-8 bytes.

[Source](/reference-sources/Text.Url.Error.cinch) · line 74

<a id="value-75726c2d6661696c7572652d6b696e64"></a>

## `url-failure-kind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure-kind</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailureKind</span>)</code></pre>

Observe the named failure class.

[Source](/reference-sources/Text.Url.Error.cinch) · line 66

<a id="value-75726c2d6661696c7572652d6d657373616765"></a>

## `url-failure-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a named failure while retaining its raw detail separately for programmatic handling.

[Source](/reference-sources/Text.Url.Error.cinch) · line 140

<a id="value-75726c2d6661696c7572652d6f6666736574"></a>

## `url-failure-offset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure-offset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Observe the zero-based byte position within the component being processed.

[Source](/reference-sources/Text.Url.Error.cinch) · line 70

<a id="value-75726c2d6661696c7572652d7368696674"></a>

## `url-failure-shift`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-failure-shift</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span>))</code></pre>

Rebase a component-relative failure into a larger original byte interval.

[Source](/reference-sources/Text.Url.Error.cinch) · line 78

<a id="value-75726c2d71756f74652d6279746573"></a>

## `url-quote-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-quote-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Produce an ASCII byte-exact quoted diagnostic; non-ASCII bytes use hexadecimal escapes.

[Source](/reference-sources/Text.Url.Error.cinch) · line 129

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Url/Error.cinch`. Source SHA-256: `93da213a73d120c7bb2d0b5651b52a28646a13727582d49bb0d70600a92fc56f`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
