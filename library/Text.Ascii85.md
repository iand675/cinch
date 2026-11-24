# Text.Ascii85

ASCII85 text, caller-buffer and incremental codecs with consumed-input progress, checked source offsets, SIMD groups and control-run scanning.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Ascii85</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Ascii85/">View source</a><a href="/library/search/?q=%2BText.Ascii85">Search module</a><a href="/reference-sources/Text.Ascii85.cinch" download="Text.Ascii85.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Text.Ascii85`: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Ascii85](Text.Ascii85.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/Ascii85.cinch ./Text.Ascii85.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

ASCII85 byte encoding with incremental encoder and decoder state.
Four bytes become five symbols from ! through u; a complete zero word
becomes z. No document delimiters are added or accepted. Decoding ignores
bytes 0 through 32, counts errors in original input bytes, and accumulates
complete groups modulo 2^32, including noncanonical overflowing groups.
Complete groups use inline LLVM SIMD digit arithmetic. Incremental calls
retain only an unfinished word; finishing emits the final short quantum.

</details>

<details class="doc-synopsis"><summary>Synopsis (22 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Ascii85BufferError">Ascii85BufferError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ascii85BufferError</strong></code></li><li><a class="syn-name" href="#t-Ascii85Decoder">Ascii85Decoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ascii85Decoder</strong></code></li><li><a class="syn-name" href="#t-Ascii85Encoder">Ascii85Encoder</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ascii85Encoder</strong></code></li><li><a class="syn-name" href="#t-Ascii85Error">Ascii85Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ascii85Error</strong></code></li><li><a class="syn-name" href="#v-ascii85-buffer-error-message">ascii85-buffer-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-buffer-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-ascii85-decode">ascii85-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-decode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ascii85-decode-at">ascii85-decode-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-decode-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a>)</code></li><li><a class="syn-name" href="#v-ascii85-decode-bytes">ascii85-decode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-decode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ascii85-decode-into">ascii85-decode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-decode-into</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a>)</code></li><li><a class="syn-name" href="#v-ascii85-decode-prefix">ascii85-decode-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-decode-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a>)</code></li><li><a class="syn-name" href="#v-ascii85-decoder">ascii85-decoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-decoder</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Decoder" title="Text.Ascii85.Ascii85Decoder">Ascii85Decoder</a></code></li><li><a class="syn-name" href="#v-ascii85-decoder-feed">ascii85-decoder-feed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-decoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Decoder" title="Text.Ascii85.Ascii85Decoder">Ascii85Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Decoder" title="Text.Ascii85.Ascii85Decoder">Ascii85Decoder</a>)</code></li><li><a class="syn-name" href="#v-ascii85-decoder-finish">ascii85-decoder-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-decoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Decoder" title="Text.Ascii85.Ascii85Decoder">Ascii85Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ascii85-encode">ascii85-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-encode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-ascii85-encode-bytes">ascii85-encode-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ascii85-encode-into">ascii85-encode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-encode-into</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ascii85-encoder">ascii85-encoder</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-encoder</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Encoder" title="Text.Ascii85.Ascii85Encoder">Ascii85Encoder</a></code></li><li><a class="syn-name" href="#v-ascii85-encoder-feed">ascii85-encoder-feed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-encoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Encoder" title="Text.Ascii85.Ascii85Encoder">Ascii85Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Encoder" title="Text.Ascii85.Ascii85Encoder">Ascii85Encoder</a>)</code></li><li><a class="syn-name" href="#v-ascii85-encoder-finish">ascii85-encoder-finish</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-encoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Encoder" title="Text.Ascii85.Ascii85Encoder">Ascii85Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ascii85-error-message">ascii85-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-ascii85-error-offset">ascii85-error-offset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ascii85-max-encoded-length">ascii85-max-encoded-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ascii85-max-encoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Ascii85BufferError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ascii85BufferError</strong></code><span class="doc-sig-links"><a href="#t-Ascii85BufferError" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L224" title="View source at line 224">Source</a></span></div>
<div class="doc-body">

A source offset or destination interval is outside its accessible buffer.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Ascii85ShortDestination"><code><strong class="tok-constructor">Ascii85ShortDestination</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a></code></li><li id="k-Ascii85InvalidSourceOffset"><code><strong class="tok-constructor">Ascii85InvalidSourceOffset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a></code></li><li id="k-Ascii85InvalidDestinationOffset"><code><strong class="tok-constructor">Ascii85InvalidDestinationOffset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Ascii85Decoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ascii85Decoder</strong></code><span class="doc-sig-links"><a href="#t-Ascii85Decoder" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

An immutable decoding continuation holding at most four pending symbols.

</div>
</article>

<article class="doc-entry" id="t-Ascii85Encoder">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ascii85Encoder</strong></code><span class="doc-sig-links"><a href="#t-Ascii85Encoder" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

An immutable encoding continuation holding at most three pending bytes.

</div>
</article>

<article class="doc-entry" id="t-Ascii85Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ascii85Error</strong></code><span class="doc-sig-links"><a href="#t-Ascii85Error" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L29" title="View source at line 29">Source</a></span></div>
<div class="doc-body">

Malformed encoded input. An unsuccessful feed returns no partial output.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-InvalidAscii85Byte"><code><strong class="tok-constructor">InvalidAscii85Byte</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a></code></li><li id="k-TruncatedAscii85Quantum"><code><strong class="tok-constructor">TruncatedAscii85Quantum</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-ascii85-buffer-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-buffer-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-ascii85-buffer-error-message" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L233" title="View source at line 233">Source</a></span></div>
<div class="doc-body">

Render an interval diagnostic with the requested size or offset and its available bound.

</div>
</article>

<article class="doc-entry" id="v-ascii85-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-decode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ascii85-decode" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L212" title="View source at line 212">Source</a></span></div>
<div class="doc-body">

Decode complete text with error positions measured in UTF-8 input bytes.

</div>
</article>

<article class="doc-entry" id="v-ascii85-decode-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-decode-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a>)</code><span class="doc-sig-links"><a href="#v-ascii85-decode-at" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L305" title="View source at line 305">Source</a></span></div>
<div class="doc-body">

Decode from a checked source offset without copying an unconsumed suffix into a new input buffer.
Return written bytes and the number consumed from that offset. Errors retain their absolute
original-buffer positions. A caller can add the consumed count to its next source offset.
Four available destination bytes permit progress; final short output occupies only its actual bytes.
Overlapping intervals snapshot the source before writing; disjoint buffers keep their original owners.

</div>
</article>

<article class="doc-entry" id="v-ascii85-decode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-decode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ascii85-decode-bytes" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L206" title="View source at line 206">Source</a></span></div>
<div class="doc-body">

Decode a complete byte buffer. Control bytes through space are ignored; framing is invalid.

</div>
</article>

<article class="doc-entry" id="v-ascii85-decode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-decode-into</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a>)</code><span class="doc-sig-links"><a href="#v-ascii85-decode-into" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L297" title="View source at line 297">Source</a></span></div>
<div class="doc-body">

Decode into an existing interval and return written bytes, consumed input and an optional error.
A full quantum requires four available bytes; insufficient space returns completed progress.
With finish False, consumed input ends at the last full group so its unfinished suffix can be retried.
With finish True, valid short tails are emitted and all input is consumed. On malformed input,
completed output and its consumed boundary remain available alongside the absolute byte error.
An offset beyond the destination is rejected before mutation; overlapping input is snapshotted.

</div>
</article>

<article class="doc-entry" id="v-ascii85-decode-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-decode-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a>)</code><span class="doc-sig-links"><a href="#v-ascii85-decode-prefix" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L317" title="View source at line 317">Source</a></span></div>
<div class="doc-body">

Decode to fresh bytes while exposing completed output, consumed input and any malformed suffix.
A non-final call leaves an unfinished quantum in the input suffix rather than hidden state.

</div>
</article>

<article class="doc-entry" id="v-ascii85-decoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-decoder</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Decoder" title="Text.Ascii85.Ascii85Decoder">Ascii85Decoder</a></code><span class="doc-sig-links"><a href="#v-ascii85-decoder" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Begin an independent decoding with absolute error offsets starting at zero.

</div>
</article>

<article class="doc-entry" id="v-ascii85-decoder-feed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-decoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Decoder" title="Text.Ascii85.Ascii85Decoder">Ascii85Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Decoder" title="Text.Ascii85.Ascii85Decoder">Ascii85Decoder</a>)</code><span class="doc-sig-links"><a href="#v-ascii85-decoder-feed" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L182" title="View source at line 182">Source</a></span></div>
<div class="doc-body">

Decode complete groups in arbitrary byte chunks, retaining unfinished symbols.
Invalid bytes return an absolute byte offset and no output for this feed.

</div>
</article>

<article class="doc-entry" id="v-ascii85-decoder-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-decoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Decoder" title="Text.Ascii85.Ascii85Decoder">Ascii85Decoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ascii85-decoder-finish" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L195" title="View source at line 195">Source</a></span></div>
<div class="doc-body">

Finish decoding: a single remaining symbol is invalid; two through four emit a byte tail.

</div>
</article>

<article class="doc-entry" id="v-ascii85-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-encode</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-ascii85-encode" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

Encode a complete buffer without document delimiters or inserted whitespace.

</div>
</article>

<article class="doc-entry" id="v-ascii85-encode-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-encode-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ascii85-encode-bytes" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L240" title="View source at line 240">Source</a></span></div>
<div class="doc-body">

Encode a complete buffer to fresh ASCII bytes, including an exact short final word.

</div>
</article>

<article class="doc-entry" id="v-ascii85-encode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-encode-into</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85BufferError" title="Text.Ascii85.Ascii85BufferError">Ascii85BufferError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ascii85-encode-into" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L248" title="View source at line 248">Source</a></span></div>
<div class="doc-body">

Encode at a destination offset and return the actual byte count, including zero-word shorthand.
Capacity must hold ascii85-max-encoded-length; rejection precedes all writes.
Source and destination may overlap: overlapping input is snapshotted before mutation.

</div>
</article>

<article class="doc-entry" id="v-ascii85-encoder">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-encoder</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Encoder" title="Text.Ascii85.Ascii85Encoder">Ascii85Encoder</a></code><span class="doc-sig-links"><a href="#v-ascii85-encoder" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Begin an independent encoding. Empty feeds preserve the continuation.

</div>
</article>

<article class="doc-entry" id="v-ascii85-encoder-feed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-encoder-feed</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Encoder" title="Text.Ascii85.Ascii85Encoder">Ascii85Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Encoder" title="Text.Ascii85.Ascii85Encoder">Ascii85Encoder</a>)</code><span class="doc-sig-links"><a href="#v-ascii85-encoder-feed" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L118" title="View source at line 118">Source</a></span></div>
<div class="doc-body">

Encode all complete words in a chunk and return their bytes plus the advanced state.
The returned output is independent of subsequent mutation of the input.

</div>
</article>

<article class="doc-entry" id="v-ascii85-encoder-finish">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-encoder-finish</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Encoder" title="Text.Ascii85.Ascii85Encoder">Ascii85Encoder</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ascii85-encoder-finish" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

Finish an encoding, emitting two through four symbols for any pending bytes.
A partial zero word is never shortened to z. Finishing an empty state is empty.

</div>
</article>

<article class="doc-entry" id="v-ascii85-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-ascii85-error-message" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

Render a decoding diagnostic, including the offending byte or incomplete tail position.

</div>
</article>

<article class="doc-entry" id="v-ascii85-error-offset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-error-offset</strong> :: <a class="doc-type-link tok-constructor" href="/library/Text.Ascii85/#t-Ascii85Error" title="Text.Ascii85.Ascii85Error">Ascii85Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ascii85-error-offset" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

The absolute original input byte offset associated with a decoding failure.

</div>
</article>

<article class="doc-entry" id="v-ascii85-max-encoded-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ascii85-max-encoded-length</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-ascii85-max-encoded-length" title="Permalink">#</a><a href="/library/source/Text.Ascii85/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

A safe encoded byte bound, including a full five-symbol final allocation.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Ascii85.cinch`](/library/source/Text.Ascii85/). Source SHA-256: `6550cb8d1b45de4b2d0ca392ad31b6cae716213b003f1789ae7d0281948d222b`.
