# Text.Base64.Encoding

Configurable Base64 byte alphabets, padding and strict trailing bits, SIMD codecs, persistent states and checked caller buffers.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Base64.Encoding</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Base64.Encoding/source/">View source</a><a href="/packages/search/?q=%2BText.Base64.Encoding">Search module</a><a href="/reference-sources/Text.Base64.Encoding.cinch" download="Text.Base64.Encoding.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

<details class="doc-notes"><summary>Module notes</summary>

Configurable Base64 byte encodings with portable 128-bit SIMD quanta and persistent incremental state.
Alphabets and padding preserve exact byte values, including non-UTF-8 values.
Decoders ignore CR/LF, preserve completed output before failures, and count
original input bytes in diagnostics. Strict mode rejects nonzero unused bits.
All returned buffers are fresh; no caller-owned buffer aliases retained state.

</details>

<details class="doc-synopsis"><summary>Synopsis (35 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Base64ConfigError">Base64ConfigError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64ConfigError</strong></code></li><li><a class="syn-name" href="#t-Base64Decoder">Base64Decoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64Decoder</strong></code></li><li><a class="syn-name" href="#t-Base64Encoder">Base64Encoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64Encoder</strong></code></li><li><a class="syn-name" href="#t-Base64Encoding">Base64Encoding</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64Encoding</strong></code></li><li><a class="syn-name" href="#t-Base64InputError">Base64InputError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64InputError</strong></code></li><li><a class="syn-name" href="#v-base64-config-error-message">base64-config-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-config-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-base64-decode-append">base64-decode-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a>)</code></li><li><a class="syn-name" href="#v-base64-decode-bytes">base64-decode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base64-decode-into">base64-decode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a>)</code></li><li><a class="syn-name" href="#v-base64-decode-prefix">base64-decode-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decode-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a>)</code></li><li><a class="syn-name" href="#v-base64-decode-text">base64-decode-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decode-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a>)</code></li><li><a class="syn-name" href="#v-base64-decoded-length">base64-decoded-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-base64-decoder">base64-decoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Decoder" title="Text.Base64.Encoding.Base64Decoder">Base64Decoder</a></code></li><li><a class="syn-name" href="#v-base64-decoder-error">base64-decoder-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decoder-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Decoder" title="Text.Base64.Encoding.Base64Decoder">Base64Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a></code></li><li><a class="syn-name" href="#v-base64-decoder-feed">base64-decoder-feed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Decoder" title="Text.Base64.Encoding.Base64Decoder">Base64Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Decoder" title="Text.Base64.Encoding.Base64Decoder">Base64Decoder</a>)</code></li><li><a class="syn-name" href="#v-base64-decoder-finish">base64-decoder-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-decoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Decoder" title="Text.Base64.Encoding.Base64Decoder">Base64Decoder</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a>)</code></li><li><a class="syn-name" href="#v-base64-encode-append">base64-encode-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base64-encode-bytes">base64-encode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base64-encode-into">base64-encode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-base64-encode-text">base64-encode-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encode-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-base64-encoded-length">base64-encoded-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-base64-encoder">base64-encoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoder" title="Text.Base64.Encoding.Base64Encoder">Base64Encoder</a></code></li><li><a class="syn-name" href="#v-base64-encoder-feed">base64-encoder-feed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoder" title="Text.Base64.Encoding.Base64Encoder">Base64Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoder" title="Text.Base64.Encoding.Base64Encoder">Base64Encoder</a>)</code></li><li><a class="syn-name" href="#v-base64-encoder-finish">base64-encoder-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoder" title="Text.Base64.Encoding.Base64Encoder">Base64Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base64-encoding">base64-encoding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code></li><li><a class="syn-name" href="#v-base64-encoding-alphabet">base64-encoding-alphabet</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encoding-alphabet</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base64-encoding-padding">base64-encoding-padding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-encoding-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-base64-input-error-message">base64-input-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-input-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-base64-input-error-offset">base64-input-error-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-input-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-base64-raw-standard-encoding">base64-raw-standard-encoding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-raw-standard-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code></li><li><a class="syn-name" href="#v-base64-raw-url-encoding">base64-raw-url-encoding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-raw-url-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code></li><li><a class="syn-name" href="#v-base64-standard-encoding">base64-standard-encoding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-standard-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code></li><li><a class="syn-name" href="#v-base64-strict">base64-strict</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-strict</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code></li><li><a class="syn-name" href="#v-base64-url-encoding">base64-url-encoding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-url-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code></li><li><a class="syn-name" href="#v-base64-with-padding">base64-with-padding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base64-with-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Base64ConfigError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64ConfigError</strong></code><span class="doc-sig-links"><a href="#t-Base64ConfigError" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Invalid alphabet or padding configuration, returned without mutating caller storage.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Base64AlphabetLength"><code><strong class="tok-constructor">Base64AlphabetLength</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a></code></li><li id="k-Base64AlphabetNewline"><code><strong class="tok-constructor">Base64AlphabetNewline</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a></code></li><li id="k-Base64AlphabetDuplicate"><code><strong class="tok-constructor">Base64AlphabetDuplicate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a></code></li><li id="k-Base64PaddingValue"><code><strong class="tok-constructor">Base64PaddingValue</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a></code></li><li id="k-Base64PaddingCollision"><code><strong class="tok-constructor">Base64PaddingCollision</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Base64Decoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64Decoder</strong></code><span class="doc-sig-links"><a href="#t-Base64Decoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L335" title="View source at line 335">Source</a></span></div>
<div class="doc-body">

Immutable decoder with a packed partial quantum, absolute offsets and sticky error.

</div>
</article>

<article class="doc-entry" id="t-Base64Encoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64Encoder</strong></code><span class="doc-sig-links"><a href="#t-Base64Encoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L314" title="View source at line 314">Source</a></span></div>
<div class="doc-body">

Immutable encoding continuation with at most two copied pending input bytes.

</div>
</article>

<article class="doc-entry" id="t-Base64Encoding">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64Encoding</strong></code><span class="doc-sig-links"><a href="#t-Base64Encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

Opaque immutable alphabet, optional padding and trailing-bit validation policy.

</div>
</article>

<article class="doc-entry" id="t-Base64InputError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base64InputError</strong></code><span class="doc-sig-links"><a href="#t-Base64InputError" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L278" title="View source at line 278">Source</a></span></div>
<div class="doc-body">

Precise original-byte failures and caller-buffer capacity failures.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidBase64Byte"><code><strong class="tok-constructor">InvalidBase64Byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a></code></li><li id="k-TruncatedBase64Quantum"><code><strong class="tok-constructor">TruncatedBase64Quantum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a></code></li><li id="k-InvalidBase64TrailingBits"><code><strong class="tok-constructor">InvalidBase64TrailingBits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a></code></li><li id="k-Base64ShortDestination"><code><strong class="tok-constructor">Base64ShortDestination</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-base64-config-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-config-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-base64-config-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Describe the first invalid alphabet position or rejected padding value.

</div>
</article>

<article class="doc-entry" id="v-base64-decode-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a>)</code><span class="doc-sig-links"><a href="#v-base64-decode-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L545" title="View source at line 545">Source</a></span></div>
<div class="doc-body">

Append decoded bytes to a copied prefix, preserving any later input failure.

</div>
</article>

<article class="doc-entry" id="v-base64-decode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base64-decode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L535" title="View source at line 535">Source</a></span></div>
<div class="doc-body">

Decode complete input atomically; use the prefix API when progress is needed on failure.

</div>
</article>

<article class="doc-entry" id="v-base64-decode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a>)</code><span class="doc-sig-links"><a href="#v-base64-decode-into" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L550" title="View source at line 550">Source</a></span></div>
<div class="doc-body">

Decode into a checked capacity bound; complete decoding before copying makes overlap safe.
A short range fails before mutation, otherwise completed output is retained beside its input error.

</div>
</article>

<article class="doc-entry" id="v-base64-decode-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decode-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a>)</code><span class="doc-sig-links"><a href="#v-base64-decode-prefix" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L530" title="View source at line 530">Source</a></span></div>
<div class="doc-body">

Decode all bytes while preserving the completed prefix beside a later input error.

</div>
</article>

<article class="doc-entry" id="v-base64-decode-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decode-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a>)</code><span class="doc-sig-links"><a href="#v-base64-decode-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L541" title="View source at line 541">Source</a></span></div>
<div class="doc-body">

Decode text with original UTF-8 byte offsets while preserving completed output on error.

</div>
</article>

<article class="doc-entry" id="v-base64-decoded-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-base64-decoded-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L160" title="View source at line 160">Source</a></span></div>
<div class="doc-body">

Maximum decoded byte count for an encoded byte count; this does not validate the bytes.

</div>
</article>

<article class="doc-entry" id="v-base64-decoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Decoder" title="Text.Base64.Encoding.Base64Decoder">Base64Decoder</a></code><span class="doc-sig-links"><a href="#v-base64-decoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L340" title="View source at line 340">Source</a></span></div>
<div class="doc-body">

Begin an independent decoder; CR/LF remain ignored even in strict mode.

</div>
</article>

<article class="doc-entry" id="v-base64-decoder-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decoder-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Decoder" title="Text.Base64.Encoding.Base64Decoder">Base64Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a></code><span class="doc-sig-links"><a href="#v-base64-decoder-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L344" title="View source at line 344">Source</a></span></div>
<div class="doc-body">

Observe a sticky failure after retaining all output returned with its feed.

</div>
</article>

<article class="doc-entry" id="v-base64-decoder-feed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Decoder" title="Text.Base64.Encoding.Base64Decoder">Base64Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Decoder" title="Text.Base64.Encoding.Base64Decoder">Base64Decoder</a>)</code><span class="doc-sig-links"><a href="#v-base64-decoder-feed" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L361" title="View source at line 361">Source</a></span></div>
<div class="doc-body">

Feed encoded bytes, preserving complete output before any malformed byte or strict-bit error.

</div>
</article>

<article class="doc-entry" id="v-base64-decoder-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-decoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Decoder" title="Text.Base64.Encoding.Base64Decoder">Base64Decoder</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a>)</code><span class="doc-sig-links"><a href="#v-base64-decoder-finish" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L516" title="View source at line 516">Source</a></span></div>
<div class="doc-body">

Validate EOF and render an allowed unpadded tail, retaining strict trailing-bit failures.

</div>
</article>

<article class="doc-entry" id="v-base64-encode-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base64-encode-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L269" title="View source at line 269">Source</a></span></div>
<div class="doc-body">

Append encoded bytes to a copied prefix without changing either input buffer.

</div>
</article>

<article class="doc-entry" id="v-base64-encode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base64-encode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L259" title="View source at line 259">Source</a></span></div>
<div class="doc-body">

Encode into fresh bytes, preserving custom bytes without a UTF-8 conversion.

</div>
</article>

<article class="doc-entry" id="v-base64-encode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-base64-encode-into" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L305" title="View source at line 305">Source</a></span></div>
<div class="doc-body">

Encode into a complete destination range after checking capacity and snapshotting overlap.

</div>
</article>

<article class="doc-entry" id="v-base64-encode-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encode-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-base64-encode-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L265" title="View source at line 265">Source</a></span></div>
<div class="doc-body">

Return text only when the exact encoded byte sequence is valid UTF-8.

</div>
</article>

<article class="doc-entry" id="v-base64-encoded-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-base64-encoded-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L154" title="View source at line 154">Source</a></span></div>
<div class="doc-body">

Exact output byte count, including any final padding.

</div>
</article>

<article class="doc-entry" id="v-base64-encoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoder" title="Text.Base64.Encoding.Base64Encoder">Base64Encoder</a></code><span class="doc-sig-links"><a href="#v-base64-encoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L319" title="View source at line 319">Source</a></span></div>
<div class="doc-body">

Begin an encoding that emits full quanta until explicitly finished.

</div>
</article>

<article class="doc-entry" id="v-base64-encoder-feed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoder" title="Text.Base64.Encoding.Base64Encoder">Base64Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoder" title="Text.Base64.Encoding.Base64Encoder">Base64Encoder</a>)</code><span class="doc-sig-links"><a href="#v-base64-encoder-feed" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L323" title="View source at line 323">Source</a></span></div>
<div class="doc-body">

Emit complete three-byte quanta and copy only the remaining zero to two input bytes.

</div>
</article>

<article class="doc-entry" id="v-base64-encoder-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoder" title="Text.Base64.Encoding.Base64Encoder">Base64Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base64-encoder-finish" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L331" title="View source at line 331">Source</a></span></div>
<div class="doc-body">

Render the final short quantum with the selected padding, preserving the original continuation.

</div>
</article>

<article class="doc-entry" id="v-base64-encoding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code><span class="doc-sig-links"><a href="#v-base64-encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Construct an independent encoding from 64 distinct non-newline bytes.
The default pad is '='. If that byte is in the custom alphabet, change or
disable padding before relying on an invertible final short quantum.

</div>
</article>

<article class="doc-entry" id="v-base64-encoding-alphabet">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encoding-alphabet</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base64-encoding-alphabet" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

Return an independent copy of the complete byte alphabet.

</div>
</article>

<article class="doc-entry" id="v-base64-encoding-padding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-encoding-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-base64-encoding-padding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

Observe the optional one-byte padding policy.

</div>
</article>

<article class="doc-entry" id="v-base64-input-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-input-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-base64-input-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L297" title="View source at line 297">Source</a></span></div>
<div class="doc-body">

Render a diagnostic with an original byte position or complete destination capacity.

</div>
</article>

<article class="doc-entry" id="v-base64-input-error-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-input-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64InputError" title="Text.Base64.Encoding.Base64InputError">Base64InputError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-base64-input-error-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L289" title="View source at line 289">Source</a></span></div>
<div class="doc-body">

Original byte position for an input failure; capacity failures return zero.

</div>
</article>

<article class="doc-entry" id="v-base64-raw-standard-encoding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-raw-standard-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code><span class="doc-sig-links"><a href="#v-base64-raw-standard-encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

Standard alphabet without padding.

</div>
</article>

<article class="doc-entry" id="v-base64-raw-url-encoding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-raw-url-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code><span class="doc-sig-links"><a href="#v-base64-raw-url-encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

URL alphabet without padding.

</div>
</article>

<article class="doc-entry" id="v-base64-standard-encoding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-standard-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code><span class="doc-sig-links"><a href="#v-base64-standard-encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

Standard alphabet with equals-sign padding and permissive unused trailing bits.

</div>
</article>

<article class="doc-entry" id="v-base64-strict">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-strict</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code><span class="doc-sig-links"><a href="#v-base64-strict" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L150" title="View source at line 150">Source</a></span></div>
<div class="doc-body">

Copy the encoding with unused trailing-bit validation enabled; encoding output is unchanged.

</div>
</article>

<article class="doc-entry" id="v-base64-url-encoding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-url-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code><span class="doc-sig-links"><a href="#v-base64-url-encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

URL alphabet with hyphen/underscore symbols and equals-sign padding.

</div>
</article>

<article class="doc-entry" id="v-base64-with-padding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base64-with-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64ConfigError" title="Text.Base64.Encoding.Base64ConfigError">Base64ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base64.Encoding/#t-Base64Encoding" title="Text.Base64.Encoding.Base64Encoding">Base64Encoding</a></code><span class="doc-sig-links"><a href="#v-base64-with-padding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base64.Encoding/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Select a validated padding byte, or None for unpadded output.

</div>
</article>
