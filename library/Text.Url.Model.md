# Text.Url.Model

Immutable URL components, raw encoding hints, rendering and independent copied observations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Url.Model`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Url.Model.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

Non-embedded source closure: [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Immutable byte-preserving URL components, rendering and independent observations.
Decoded fields remain bytes, including invalid UTF-8. Rendering returns bytes;
the optional text view succeeds only when the complete result is valid UTF-8.
Manual components are copied, raw encoding hints are checked before use, and
all byte getters return fresh storage. Retained versions share private owners.

</details>

# Types and constructors

<a id="type-55726c"></a>

## `Url`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Url</span></code></pre>

An immutable URL whose component byte buffers have no external mutable aliases.

[Source](/reference-sources/Text.Url.Model.cinch) · line 34

<a id="type-55726c436f6d706f6e656e7473"></a>

## `UrlComponents`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UrlComponents</span></code></pre>

Explicit URL fields: scheme, opaque, user, host, path, fragment, raw query,
raw path hint, raw fragment hint, forced query delimiter and omitted host.
Byte fields may contain arbitrary bytes; constructing a URL snapshots them.

[Source](/reference-sources/Text.Url.Model.cinch) · line 29

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">UrlParts</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Userinfo</span>.<span class="tok-constructor">UrlUserinfo</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">UrlComponents</span>)))))))))))</code></pre>

# Values

<a id="value-75726c2d617070656e642d62696e617279"></a>

## `url-append-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-append-binary</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append serialized URL bytes to a copied prefix without mutating either input.

[Source](/reference-sources/Text.Url.Model.cinch) · line 232

<a id="value-75726c2d636c6f6e65"></a>

## `url-clone`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-clone</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)</code></pre>

Retain an independent immutable URL version without copying private buffers.

[Source](/reference-sources/Text.Url.Model.cinch) · line 54

<a id="value-75726c2d636f6d706f6e656e7473"></a>

## `url-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-components</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">UrlComponents</span>)</code></pre>

Return a fresh editable component snapshot without exposing the private owners.

[Source](/reference-sources/Text.Url.Model.cinch) · line 44

<a id="value-75726c2d656d707479"></a>

## `url-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-empty</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span></code></pre>

Construct the empty relative reference.

[Source](/reference-sources/Text.Url.Model.cinch) · line 50

<a id="value-75726c2d657363617065642d667261676d656e74"></a>

## `url-escaped-fragment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-escaped-fragment</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Choose a valid raw fragment spelling only when it decodes to the stored fragment.

[Source](/reference-sources/Text.Url.Model.cinch) · line 111

<a id="value-75726c2d657363617065642d70617468"></a>

## `url-escaped-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-escaped-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Choose a valid raw path spelling only when it decodes to the stored path.

[Source](/reference-sources/Text.Url.Model.cinch) · line 102

<a id="value-75726c2d666f7263652d7175657279"></a>

## `url-force-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-force-query</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Observe force query without exposing mutable storage.

[Source](/reference-sources/Text.Url.Model.cinch) · line 94

<a id="value-75726c2d667261676d656e74"></a>

## `url-fragment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-fragment</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe fragment as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 78

<a id="value-75726c2d66726f6d2d636f6d706f6e656e7473"></a>

## `url-from-components`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-from-components</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">UrlComponents</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)</code></pre>

Snapshot every externally mutable byte field in a component bundle.

[Source](/reference-sources/Text.Url.Model.cinch) · line 39

<a id="value-75726c2d686f7374"></a>

## `url-host`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-host</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe host as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 70

<a id="value-75726c2d686f73746e616d65"></a>

## `url-hostname`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-hostname</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return host bytes without a valid optional port and without enclosing IP brackets.

[Source](/reference-sources/Text.Url.Model.cinch) · line 198

<a id="value-75726c2d69732d6162736f6c757465"></a>

## `url-is-absolute`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-is-absolute</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

A URL is absolute precisely when its scheme is nonempty.

[Source](/reference-sources/Text.Url.Model.cinch) · line 183

<a id="value-75726c2d6d61727368616c2d62696e617279"></a>

## `url-marshal-binary`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-marshal-binary</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Serialize a URL to its fresh byte representation.

[Source](/reference-sources/Text.Url.Model.cinch) · line 228

<a id="value-75726c2d6f6d69742d686f7374"></a>

## `url-omit-host`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-omit-host</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Observe omit host without exposing mutable storage.

[Source](/reference-sources/Text.Url.Model.cinch) · line 98

<a id="value-75726c2d6f7061717565"></a>

## `url-opaque`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-opaque</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe opaque as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 62

<a id="value-75726c2d70617468"></a>

## `url-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe path as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 74

<a id="value-75726c2d706f7274"></a>

## `url-port`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-port</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return lexical decimal port bytes, including an empty result for an absent or empty port.

[Source](/reference-sources/Text.Url.Model.cinch) · line 205

<a id="value-75726c2d7175657279"></a>

## `url-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-query</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Query</span>.<span class="tok-constructor">UrlValues</span>)</code></pre>

Parse raw query fields with the default bound, discarding malformed pairs and their diagnostics.

[Source](/reference-sources/Text.Url.Model.cinch) · line 209

<a id="value-75726c2d7261772d667261676d656e74"></a>

## `url-raw-fragment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-raw-fragment</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe raw fragment as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 90

<a id="value-75726c2d7261772d70617468"></a>

## `url-raw-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-raw-path</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe raw path as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 86

<a id="value-75726c2d7261772d7175657279"></a>

## `url-raw-query`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-raw-query</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe raw query as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 82

<a id="value-75726c2d7265646163746564"></a>

## `url-redacted`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-redacted</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return fresh URL bytes with a present password replaced by five lowercase x bytes.

[Source](/reference-sources/Text.Url.Model.cinch) · line 213

<a id="value-75726c2d726571756573742d757269"></a>

## `url-request-uri`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-request-uri</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Render a request target, excluding fragment and using slash for an empty hierarchical path.

[Source](/reference-sources/Text.Url.Model.cinch) · line 220

<a id="value-75726c2d736368656d65"></a>

## `url-scheme`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-scheme</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Observe scheme as a fresh byte buffer.

[Source](/reference-sources/Text.Url.Model.cinch) · line 58

<a id="value-75726c2d746f2d6279746573"></a>

## `url-to-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-to-bytes</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Reassemble fresh URL bytes, preserving raw query and opaque bytes exactly.

[Source](/reference-sources/Text.Url.Model.cinch) · line 164

<a id="value-75726c2d746f2d74657874"></a>

## `url-to-text`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-to-text</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Decode rendered bytes as UTF-8 only when every preserved byte is valid text.

[Source](/reference-sources/Text.Url.Model.cinch) · line 179

<a id="value-75726c2d75736572696e666f"></a>

## `url-userinfo`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-userinfo</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Userinfo</span>.<span class="tok-constructor">UrlUserinfo</span>))</code></pre>

Observe user without exposing mutable storage.

[Source](/reference-sources/Text.Url.Model.cinch) · line 66

<a id="value-75726c2d776974682d667261676d656e74"></a>

## `url-with-fragment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-with-fragment</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)))</code></pre>

Set an escaped fragment, preserving a noncanonical raw hint and decoded bytes.

[Source](/reference-sources/Text.Url.Model.cinch) · line 128

<a id="value-75726c2d776974682d70617468"></a>

## `url-with-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-with-path</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Error</span>.<span class="tok-constructor">UrlFailure</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Url</span>.<span class="tok-constructor">Model</span>.<span class="tok-constructor">Url</span>)))</code></pre>

Set an escaped path on a copied field snapshot, preserving a noncanonical raw hint.

[Source](/reference-sources/Text.Url.Model.cinch) · line 120

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Url/Model.cinch`. Source SHA-256: `7fee092c7305cda028b8ec71763d8f1c76189cb35c76d1a02bb068db1aca7f75`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
