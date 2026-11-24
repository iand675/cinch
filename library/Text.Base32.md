# Text.Base32

Configurable Base32 byte alphabets and padding, SIMD codecs, prefix/caller-buffer operations and persistent incremental states.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Base32</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Base32/source/">View source</a><a href="/packages/search/?q=%2BText.Base32">Search module</a><a href="/reference-sources/Text.Base32.cinch" download="Text.Base32.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Text.Base32`: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Base32](Text.Base32.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/Base32.cinch ./Text.Base32.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

RFC 4648 Base32 with fixed or custom byte alphabets and optional byte padding.
Fixed-alphabet text conveniences accept uppercase symbols and return atomic
errors. Configurable byte APIs preserve complete output before a later error,
count positions in original bytes and provide persistent encoder/decoder state.
CR and LF are ignored. Final lengths, padding and trailing data are validated;
unused low symbol bits are discarded. Complete quanta use inline LLVM SIMD
and SWAR; custom lookup tables are private, immutable copies of caller bytes.

</details>

<details class="doc-synopsis"><summary>Synopsis (38 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Base32Alphabet">Base32Alphabet</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Alphabet</strong></code></li><li><a class="syn-name" href="#t-Base32ConfigError">Base32ConfigError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32ConfigError</strong></code></li><li><a class="syn-name" href="#t-Base32Decoder">Base32Decoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Decoder</strong></code></li><li><a class="syn-name" href="#t-Base32Encoder">Base32Encoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Encoder</strong></code></li><li><a class="syn-name" href="#t-Base32Encoding">Base32Encoding</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Encoding</strong></code></li><li><a class="syn-name" href="#t-Base32Error">Base32Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Error</strong></code></li><li><a class="syn-name" href="#t-Base32InputError">Base32InputError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32InputError</strong></code></li><li><a class="syn-name" href="#v-base32-config-error-message">base32-config-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-config-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-base32-decode">base32-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Alphabet" title="Text.Base32.Base32Alphabet">Base32Alphabet</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Error" title="Text.Base32.Base32Error">Base32Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base32-decode-append">base32-decode-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a>)</code></li><li><a class="syn-name" href="#v-base32-decode-bytes">base32-decode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base32-decode-into">base32-decode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a>)</code></li><li><a class="syn-name" href="#v-base32-decode-prefix">base32-decode-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decode-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a>)</code></li><li><a class="syn-name" href="#v-base32-decode-text">base32-decode-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decode-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a>)</code></li><li><a class="syn-name" href="#v-base32-decoded-length">base32-decoded-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-base32-decoder">base32-decoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Decoder" title="Text.Base32.Base32Decoder">Base32Decoder</a></code></li><li><a class="syn-name" href="#v-base32-decoder-error">base32-decoder-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decoder-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Decoder" title="Text.Base32.Base32Decoder">Base32Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a></code></li><li><a class="syn-name" href="#v-base32-decoder-feed">base32-decoder-feed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Decoder" title="Text.Base32.Base32Decoder">Base32Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Decoder" title="Text.Base32.Base32Decoder">Base32Decoder</a>)</code></li><li><a class="syn-name" href="#v-base32-decoder-finish">base32-decoder-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-decoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Decoder" title="Text.Base32.Base32Decoder">Base32Decoder</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a>)</code></li><li><a class="syn-name" href="#v-base32-encode">base32-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Alphabet" title="Text.Base32.Base32Alphabet">Base32Alphabet</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-base32-encode-append">base32-encode-append</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base32-encode-bytes">base32-encode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base32-encode-into">base32-encode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-base32-encode-text">base32-encode-text</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encode-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-base32-encoded-length">base32-encoded-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-base32-encoder">base32-encoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoder" title="Text.Base32.Base32Encoder">Base32Encoder</a></code></li><li><a class="syn-name" href="#v-base32-encoder-feed">base32-encoder-feed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoder" title="Text.Base32.Base32Encoder">Base32Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoder" title="Text.Base32.Base32Encoder">Base32Encoder</a>)</code></li><li><a class="syn-name" href="#v-base32-encoder-finish">base32-encoder-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoder" title="Text.Base32.Base32Encoder">Base32Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base32-encoding">base32-encoding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a></code></li><li><a class="syn-name" href="#v-base32-encoding-alphabet">base32-encoding-alphabet</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encoding-alphabet</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base32-encoding-padding">base32-encoding-padding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-encoding-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-base32-hex-encoding">base32-hex-encoding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-hex-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a></code></li><li><a class="syn-name" href="#v-base32-input-error-message">base32-input-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-input-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-base32-input-error-offset">base32-input-error-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-input-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-base32-raw-decode">base32-raw-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-raw-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Alphabet" title="Text.Base32.Base32Alphabet">Base32Alphabet</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Error" title="Text.Base32.Base32Error">Base32Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-base32-raw-encode">base32-raw-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-raw-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Alphabet" title="Text.Base32.Base32Alphabet">Base32Alphabet</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-base32-standard-encoding">base32-standard-encoding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-standard-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a></code></li><li><a class="syn-name" href="#v-base32-with-padding">base32-with-padding</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">base32-with-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Base32Alphabet">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Alphabet</strong></code><span class="doc-sig-links"><a href="#t-Base32Alphabet" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

The fixed 32-symbol alphabet for a Base32 encoding.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Base32Standard"><code><strong class="tok-constructor">Base32Standard</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Alphabet" title="Text.Base32.Base32Alphabet">Base32Alphabet</a></code></li><li id="k-Base32Hex"><code><strong class="tok-constructor">Base32Hex</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Alphabet" title="Text.Base32.Base32Alphabet">Base32Alphabet</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Base32ConfigError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32ConfigError</strong></code><span class="doc-sig-links"><a href="#t-Base32ConfigError" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L450" title="View source at line 450">Source</a></span></div>
<div class="doc-body">

Invalid alphabet or padding configuration; construction never changes caller storage.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Base32AlphabetLength"><code><strong class="tok-constructor">Base32AlphabetLength</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a></code></li><li id="k-Base32AlphabetNewline"><code><strong class="tok-constructor">Base32AlphabetNewline</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a></code></li><li id="k-Base32AlphabetDuplicate"><code><strong class="tok-constructor">Base32AlphabetDuplicate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a></code></li><li id="k-Base32PaddingValue"><code><strong class="tok-constructor">Base32PaddingValue</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a></code></li><li id="k-Base32PaddingCollision"><code><strong class="tok-constructor">Base32PaddingCollision</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Base32Decoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Decoder</strong></code><span class="doc-sig-links"><a href="#t-Base32Decoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L714" title="View source at line 714">Source</a></span></div>
<div class="doc-body">

Persistent decoding continuation with a packed unfinished quantum and sticky failure.

</div>
</article>

<article class="doc-entry" id="t-Base32Encoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Encoder</strong></code><span class="doc-sig-links"><a href="#t-Base32Encoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L690" title="View source at line 690">Source</a></span></div>
<div class="doc-body">

Persistent encoding continuation owning fewer than five pending input bytes.

</div>
</article>

<article class="doc-entry" id="t-Base32Encoding">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Encoding</strong></code><span class="doc-sig-links"><a href="#t-Base32Encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L445" title="View source at line 445">Source</a></span></div>
<div class="doc-body">

An immutable byte alphabet, inverse lookup and optional one-byte padding policy.
Custom alphabets are copied. Fixed configurations use the existing SIMD classifiers.

</div>
</article>

<article class="doc-entry" id="t-Base32Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32Error</strong></code><span class="doc-sig-links"><a href="#t-Base32Error" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Malformed input for the fixed-alphabet text conveniences; errors carry no partial output.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidBase32Character"><code><strong class="tok-constructor">InvalidBase32Character</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Error" title="Text.Base32.Base32Error">Base32Error</a></code></li><li id="k-InvalidBase32Length"><code><strong class="tok-constructor">InvalidBase32Length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Error" title="Text.Base32.Base32Error">Base32Error</a></code></li><li id="k-InvalidBase32Padding"><code><strong class="tok-constructor">InvalidBase32Padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Error" title="Text.Base32.Base32Error">Base32Error</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Base32InputError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Base32InputError</strong></code><span class="doc-sig-links"><a href="#t-Base32InputError" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L657" title="View source at line 657">Source</a></span></div>
<div class="doc-body">

Invalid encoded bytes, an incomplete final quantum, or insufficient destination capacity.
Input positions count original bytes, including ignored CR and LF.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidBase32Byte"><code><strong class="tok-constructor">InvalidBase32Byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a></code></li><li id="k-TruncatedBase32Quantum"><code><strong class="tok-constructor">TruncatedBase32Quantum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a></code></li><li id="k-Base32ShortDestination"><code><strong class="tok-constructor">Base32ShortDestination</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-base32-config-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-config-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-base32-config-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L463" title="View source at line 463">Source</a></span></div>
<div class="doc-body">

Describe the precise alphabet offset or rejected padding value.

</div>
</article>

<article class="doc-entry" id="v-base32-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Alphabet" title="Text.Base32.Base32Alphabet">Base32Alphabet</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Error" title="Text.Base32.Base32Error">Base32Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base32-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L198" title="View source at line 198">Source</a></span></div>
<div class="doc-body">

Decode padded Base32, ignoring CR/LF. Incomplete quanta, misplaced padding, lowercase and unknown symbols return named errors.

</div>
</article>

<article class="doc-entry" id="v-base32-decode-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a>)</code><span class="doc-sig-links"><a href="#v-base32-decode-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L831" title="View source at line 831">Source</a></span></div>
<div class="doc-body">

Append completed decoded bytes to a copied prefix, retaining a later malformed-input error.

</div>
</article>

<article class="doc-entry" id="v-base32-decode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base32-decode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L821" title="View source at line 821">Source</a></span></div>
<div class="doc-body">

Decode complete bytes atomically; use the prefix API when malformed-input progress is needed.

</div>
</article>

<article class="doc-entry" id="v-base32-decode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a>)</code><span class="doc-sig-links"><a href="#v-base32-decode-into" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L838" title="View source at line 838">Source</a></span></div>
<div class="doc-body">

Decode into a checked destination and return progress plus a possible input error.
The actual completed output must fit before mutation. Decoding completes before
copying, so overlap is safe and valid padding never requires excess capacity.

</div>
</article>

<article class="doc-entry" id="v-base32-decode-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decode-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a>)</code><span class="doc-sig-links"><a href="#v-base32-decode-prefix" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L815" title="View source at line 815">Source</a></span></div>
<div class="doc-body">

Decode bytes with precise original offsets and retain all preceding complete output on error.

</div>
</article>

<article class="doc-entry" id="v-base32-decode-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decode-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a>)</code><span class="doc-sig-links"><a href="#v-base32-decode-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L827" title="View source at line 827">Source</a></span></div>
<div class="doc-body">

Decode UTF-8 text while preserving complete output and counting original UTF-8 byte offsets.

</div>
</article>

<article class="doc-entry" id="v-base32-decoded-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-base32-decoded-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L563" title="View source at line 563">Source</a></span></div>
<div class="doc-body">

Maximum decoded byte count from an encoded byte count; this does not validate input.
Newlines and padding can make the actual result shorter than this capacity bound.

</div>
</article>

<article class="doc-entry" id="v-base32-decoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Decoder" title="Text.Base32.Base32Decoder">Base32Decoder</a></code><span class="doc-sig-links"><a href="#v-base32-decoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L719" title="View source at line 719">Source</a></span></div>
<div class="doc-body">

Begin a strict byte decoder. CR/LF are ignored; padding and final lengths are validated.

</div>
</article>

<article class="doc-entry" id="v-base32-decoder-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decoder-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Decoder" title="Text.Base32.Base32Decoder">Base32Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a></code><span class="doc-sig-links"><a href="#v-base32-decoder-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L723" title="View source at line 723">Source</a></span></div>
<div class="doc-body">

Observe a sticky feed failure while retaining any output returned with that feed.

</div>
</article>

<article class="doc-entry" id="v-base32-decoder-feed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Decoder" title="Text.Base32.Base32Decoder">Base32Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Decoder" title="Text.Base32.Base32Decoder">Base32Decoder</a>)</code><span class="doc-sig-links"><a href="#v-base32-decoder-feed" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L728" title="View source at line 728">Source</a></span></div>
<div class="doc-body">

Emit complete quanta, preserving their output alongside a later sticky decoding failure.
An incomplete padded quantum is held until every required pad is validated.

</div>
</article>

<article class="doc-entry" id="v-base32-decoder-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-decoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Decoder" title="Text.Base32.Base32Decoder">Base32Decoder</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a>)</code><span class="doc-sig-links"><a href="#v-base32-decoder-finish" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L801" title="View source at line 801">Source</a></span></div>
<div class="doc-body">

Validate EOF and render an allowed unpadded tail. Lengths 1, 3 and 6 are rejected.
Padded input requires every final pad; any data after a padded final quantum is invalid.

</div>
</article>

<article class="doc-entry" id="v-base32-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Alphabet" title="Text.Base32.Base32Alphabet">Base32Alphabet</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-base32-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

Encode with the selected alphabet and enough equals signs to complete an eight-character quantum. Empty input stays empty.

</div>
</article>

<article class="doc-entry" id="v-base32-encode-append">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encode-append</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base32-encode-append" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L647" title="View source at line 647">Source</a></span></div>
<div class="doc-body">

Append encoded bytes to a copied prefix without mutating or retaining either caller buffer.

</div>
</article>

<article class="doc-entry" id="v-base32-encode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base32-encode-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L637" title="View source at line 637">Source</a></span></div>
<div class="doc-body">

Encode directly into fresh byte storage; arbitrary alphabet and padding bytes are preserved.

</div>
</article>

<article class="doc-entry" id="v-base32-encode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encode-into</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-base32-encode-into" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L681" title="View source at line 681">Source</a></span></div>
<div class="doc-body">

Encode into a complete checked destination interval, returning the written byte count.
Capacity is checked before mutation; a source snapshot makes overlap safe.

</div>
</article>

<article class="doc-entry" id="v-base32-encode-text">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encode-text</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-base32-encode-text" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L643" title="View source at line 643">Source</a></span></div>
<div class="doc-body">

Encode as text only when the exact encoded bytes form valid UTF-8.

</div>
</article>

<article class="doc-entry" id="v-base32-encoded-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-base32-encoded-length" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L556" title="View source at line 556">Source</a></span></div>
<div class="doc-body">

Exact encoded byte count for an input length, including the selected padding policy.

</div>
</article>

<article class="doc-entry" id="v-base32-encoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encoder</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoder" title="Text.Base32.Base32Encoder">Base32Encoder</a></code><span class="doc-sig-links"><a href="#v-base32-encoder" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L695" title="View source at line 695">Source</a></span></div>
<div class="doc-body">

Begin an independent encoding; finish explicitly to emit its final short quantum.

</div>
</article>

<article class="doc-entry" id="v-base32-encoder-feed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoder" title="Text.Base32.Base32Encoder">Base32Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoder" title="Text.Base32.Base32Encoder">Base32Encoder</a>)</code><span class="doc-sig-links"><a href="#v-base32-encoder-feed" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L699" title="View source at line 699">Source</a></span></div>
<div class="doc-body">

Emit all complete five-byte quanta, retaining only a copied tail of at most four bytes.

</div>
</article>

<article class="doc-entry" id="v-base32-encoder-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoder" title="Text.Base32.Base32Encoder">Base32Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base32-encoder-finish" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L710" title="View source at line 710">Source</a></span></div>
<div class="doc-body">

Render the final short quantum and selected padding without changing the retained state.

</div>
</article>

<article class="doc-entry" id="v-base32-encoding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a></code><span class="doc-sig-links"><a href="#v-base32-encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L495" title="View source at line 495">Source</a></span></div>
<div class="doc-body">

Validate 32 distinct non-newline bytes and create an independent padded encoding.
Bytes above 127 retain their exact values. An equals-sign symbol is allowed;
select another padding byte or no padding before requiring an invertible tail.

</div>
</article>

<article class="doc-entry" id="v-base32-encoding-alphabet">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encoding-alphabet</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base32-encoding-alphabet" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L515" title="View source at line 515">Source</a></span></div>
<div class="doc-body">

Return a fresh complete alphabet buffer, including for custom non-UTF-8 alphabets.

</div>
</article>

<article class="doc-entry" id="v-base32-encoding-padding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-encoding-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-base32-encoding-padding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L523" title="View source at line 523">Source</a></span></div>
<div class="doc-body">

Observe the optional padding byte; None selects an unpadded encoding.

</div>
</article>

<article class="doc-entry" id="v-base32-hex-encoding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-hex-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a></code><span class="doc-sig-links"><a href="#v-base32-hex-encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L476" title="View source at line 476">Source</a></span></div>
<div class="doc-body">

The extended-hex uppercase byte alphabet with equals-sign padding.

</div>
</article>

<article class="doc-entry" id="v-base32-input-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-input-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-base32-input-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L673" title="View source at line 673">Source</a></span></div>
<div class="doc-body">

Describe original-byte decoding failures or the complete required destination capacity.

</div>
</article>

<article class="doc-entry" id="v-base32-input-error-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-input-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32InputError" title="Text.Base32.Base32InputError">Base32InputError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-base32-input-error-offset" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L666" title="View source at line 666">Source</a></span></div>
<div class="doc-body">

Original input position associated with a malformed quantum; capacity errors return zero.

</div>
</article>

<article class="doc-entry" id="v-base32-raw-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-raw-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Alphabet" title="Text.Base32.Base32Alphabet">Base32Alphabet</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Error" title="Text.Base32.Base32Error">Base32Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-base32-raw-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L202" title="View source at line 202">Source</a></span></div>
<div class="doc-body">

Decode unpadded Base32, ignoring CR/LF. Padding and impossible final-quantum lengths are errors.

</div>
</article>

<article class="doc-entry" id="v-base32-raw-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-raw-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Alphabet" title="Text.Base32.Base32Alphabet">Base32Alphabet</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-base32-raw-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

Encode with the selected alphabet and no padding. The output length is ceil(8 * byte-count / 5).

</div>
</article>

<article class="doc-entry" id="v-base32-standard-encoding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-standard-encoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a></code><span class="doc-sig-links"><a href="#v-base32-standard-encoding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L472" title="View source at line 472">Source</a></span></div>
<div class="doc-body">

The standard uppercase byte alphabet with equals-sign padding.

</div>
</article>

<article class="doc-entry" id="v-base32-with-padding">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">base32-with-padding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32ConfigError" title="Text.Base32.Base32ConfigError">Base32ConfigError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Base32/#t-Base32Encoding" title="Text.Base32.Base32Encoding">Base32Encoding</a></code><span class="doc-sig-links"><a href="#v-base32-with-padding" title="Permalink">#</a><a href="/packages/stdlib/Text.Base32/source/#L542" title="View source at line 542">Source</a></span></div>
<div class="doc-body">

Copy an encoding with validated one-byte padding, or disable padding with None.
Newline values, values above 255 and alphabet collisions return named errors.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Base32.cinch`](/packages/stdlib/Text.Base32/source/). Source SHA-256: `e901c7d08842d6580c765eefac5ad5d7ab48078c6fc46dbb963e3abafd55acbd`.
