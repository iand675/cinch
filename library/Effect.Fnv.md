# Effect.Fnv

Inspectable FNV hashing sinks, generic byte-writer composition and bounded reader-to-hash copying with advanced continuations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Fnv`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Fnv.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Effect.Stream](Effect.Stream.md), [Text.Fnv.State](Text.Fnv.State.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Effect.Fnv](Effect.Fnv.md), [Effect.Stream](Effect.Stream.md), [Text.Binary](Text.Binary.md), [Text.Fnv](Text.Fnv.md), [Text.Fnv.State](Text.Fnv.State.md).

<details><summary>Behaviour and boundary notes from the source</summary>

FNV hashing sinks with explicit, inspectable continuation states.
Every write accepts its entire input and preserves earlier hash handles.
Stream copying returns the advanced source and hash even on source failure.
No adapter retains input buffers or closes borrowed readers.

</details>

# Types and constructors

<a id="type-466e76577269746572"></a>

## `FnvWriter`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FnvWriter</span></code></pre>

An immutable scalar hash sink with no borrowed runtime resources.
Generic ByteWriter adapters carry their own runtime identity.

[Source](/reference-sources/Effect.Fnv.cinch) · line 17

# Values

<a id="value-666e762d636f7079"></a>

## `fnv-copy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-copy</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">FnvWriter</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">StreamError</span> <span class="tok-constructor">Nat</span>), ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteReader</span> <span class="tok-variable">r</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">FnvWriter</span>)))))</code></pre>

Hash a reader through EOF, returning the accepted count and both advanced handles.
A zero chunk size is rejected without reading. Source errors retain the hash of
every earlier successful chunk and the advanced source returned with the failure.

[Source](/reference-sources/Effect.Fnv.cinch) · line 48

<a id="value-666e762d68617368696e672d777269746572"></a>

## `fnv-hashing-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-hashing-writer</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Stream</span>.<span class="tok-constructor">ByteWriter</span> <span class="tok-variable">r</span>))</code></pre>

Expose a hash-only sink through the generic writer protocol for composition.
Use FnvWriter or fnv-copy when the advanced hash must remain directly inspectable.

[Source](/reference-sources/Effect.Fnv.cinch) · line 37

<a id="value-666e762d7772697465"></a>

## `fnv-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">FnvWriter</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">FnvWriter</span>)))</code></pre>

Hash every input byte and return its accepted count and advanced sink.
Empty input succeeds without changing the digest; resource exhaustion follows runtime policy.

[Source](/reference-sources/Effect.Fnv.cinch) · line 31

<a id="value-666e762d777269746572"></a>

## `fnv-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-writer</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">FnvWriter</span>)</code></pre>

Begin a sink from any initial or restored FNV state.

[Source](/reference-sources/Effect.Fnv.cinch) · line 22

<a id="value-666e762d7772697465722d7374617465"></a>

## `fnv-writer-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-writer-state</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">FnvWriter</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span>)</code></pre>

Inspect the current hash without advancing the sink or changing earlier handles.

[Source](/reference-sources/Effect.Fnv.cinch) · line 26

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Fnv.cinch`. Source SHA-256: `7253c58aac971fe5a2879b5cfda24593dbc52cd69bdbaff1d7ddfdf1c9db5464`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
