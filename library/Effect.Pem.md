# Effect.Pem

Validated envelope output through explicit advanced writer continuations, bounded chunks and linear short-write draining.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Pem`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Pem.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Text.Pem](Text.Pem.md).

Non-embedded source closure: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Pem](Effect.Pem.md), [Effect.Stream](Effect.Stream.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Pem](Text.Pem.md).

<details><summary>Behaviour and boundary notes from the source</summary>

One-shot PEM writing with bounded payload chunks and explicit advanced sink continuations.
Validation happens before output. Positive short writes are drained; transport
failure returns the advanced sink without closing or flushing borrowed resources.

</details>

# Types and constructors

<a id="type-50656d4572726f72"></a>

## `PemError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PemError</span></code></pre>

Encoding validation or transport failure, with no implicit sink closure.

[Source](/reference-sources/Text.Pem.cinch) · line 38

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PemInvalidHeaderKey</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PemOutputError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemError</span>)</code></pre>

# Values

<a id="value-70656d2d6572726f722d6d657373616765"></a>

## `pem-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Explain a header refusal or identify a retained underlying stream failure.

[Source](/reference-sources/Text.Pem.cinch) · line 45

<a id="value-70656d2d7772697465"></a>

## `pem-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pem-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemBlock</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Pem</span>.<span class="tok-constructor">PemError</span> <span class="tok-constructor">Unit</span>), (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))))</code></pre>

Write an envelope with bounded payload storage and explicit continuation ownership.
Validate all keys before any callback. Positive short writes are retried; zero progress,
invalid counts and sink errors preserve the advanced writer and never replay accepted
output. Chunk boundaries are unspecified. This operation never flushes or closes its sink.

[Source](/reference-sources/Text.Pem.cinch) · line 753

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Pem.cinch`. Source SHA-256: `47dfe3101d486d89c2b6e2b78700b36c4c4c3ed80cf467de347ef7ba6ffc2bb4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
