# Effect.Uuid

Secure UUID generation with explicit serialized version 7 ownership.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Uuid`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Uuid.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Core](Data.Core.md), [Data.Nat](Data.Nat.md), [Data.Uuid](Data.Uuid.md), [Effect.Clock](Effect.Clock.md), [Effect.Concurrent.MVar](Effect.Concurrent.MVar.md), [Effect.Random](Effect.Random.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Uuid](Data.Uuid.md), [Effect.Random](Effect.Random.md), [Effect.Uuid](Effect.Uuid.md), [Text.Hex](Text.Hex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Secure UUID generation with explicit serialized version 7 ownership.
Entropy comes from the system random capability and time from the wall clock.
One shared generator orders timestamp transitions; separate generators define
independent sequences. Entropy acquisition occurs after releasing the state
cell, so overlapping calls are ordered at that transition rather than by
completion time. Plain blocking operations preserve the full effect row.

</details>

# Types and constructors

<a id="type-55756964563747656e657261746f72"></a>

## `UuidV7Generator`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">UuidV7Generator</span></code></pre>

One explicitly shared serialized timestamp sequence with no public access to its state cell.

[Source](/reference-sources/Effect.Uuid.cinch) · line 18

# Values

<a id="value-757569642d6e6577"></a>

## `uuid-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>))</code></pre>

Generate the current default version 4 value using 16 fresh secure random bytes.

[Source](/reference-sources/Effect.Uuid.cinch) · line 23

<a id="value-757569642d6e65772d7634"></a>

## `uuid-new-v4`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-new-v4</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>))</code></pre>

Acquire real secure entropy and preserve 122 random bits after version/variant masking.

[Source](/reference-sources/Effect.Uuid.cinch) · line 27

<a id="value-757569642d6e65772d7637"></a>

## `uuid-new-v7`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-new-v7</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidV7Generator</span> -&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">Diverge</span>, <span class="tok-constructor">IO</span> <span class="tok-variable">r</span>, <span class="tok-constructor">MVar</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">Uuid</span>)))</code></pre>

Sample real wall time under the state lock, commit its transition, then acquire eight secure random bytes.
A whole-second clock rollback resets the ordering constraint. Same-second
rollback and equal timestamps advance the logical fraction monotonically.
Checked clock refusals restore the old state before returning an error.

[Source](/reference-sources/Effect.Uuid.cinch) · line 38

<a id="value-757569642d76372d67656e657261746f72"></a>

## `uuid-v7-generator`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uuid-v7-generator</span> :: (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">MVar</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Uuid</span>.<span class="tok-constructor">UuidV7Generator</span>)</code></pre>

Allocate one serialized version 7 sequence; retain and share this handle for shared ordering.

[Source](/reference-sources/Effect.Uuid.cinch) · line 31

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Uuid.cinch`. Source SHA-256: `fd4fb84c957afb908f2fb7d8cf1fb4ea1416fd09ec64b4a376cfd3a526cc55a3`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
