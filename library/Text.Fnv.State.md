# Text.Fnv.State

Persistent FNV states with reset, network-order digests, numeric observations and checked versioned binary checkpoints.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Fnv.State`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Fnv.State.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Binary](Text.Binary.md), [Text.Fnv](Text.Fnv.md).

Non-embedded source closure: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Binary](Text.Binary.md), [Text.Fnv](Text.Fnv.md), [Text.Fnv.State](Text.Fnv.State.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Persistent FNV hash states, digest bytes and versioned binary checkpoints.
All six width/order variants retain their identity across updates and reset.
Digests use network byte order. Retaining a state branches a computation:
later updates never change earlier states, and returned buffers are fresh.
Checkpoints contain a four-byte variant identifier followed by the digest;
loading requires a matching variant and exact size. These hashes do not
authenticate data, including their serialized checkpoints.

</details>

# Types and constructors

<a id="type-466e765374617465"></a>

## `FnvState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FnvState</span></code></pre>

An immutable hash accumulator with a fixed width and multiplication/xor order.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 27

<a id="type-466e7653746174654572726f72"></a>

## `FnvStateError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">FnvStateError</span></code></pre>

A checkpoint's variant identity or complete byte length is invalid.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 141

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidFnvStateIdentifier</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvStateError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">InvalidFnvStateSize</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvStateError</span>))</code></pre>

# Values

<a id="value-666e762d73746174652d617070656e64"></a>

## `fnv-state-append`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-append</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append digest bytes to a copied prefix, leaving both the prefix and state unchanged.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 120

<a id="value-666e762d73746174652d617070656e642d73617665"></a>

## `fnv-state-append-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-append-save</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Append a binary checkpoint to a copied prefix without changing either argument.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 137

<a id="value-666e762d73746174652d626c6f636b2d73697a65"></a>

## `fnv-state-block-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-block-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Preferred input granularity in bytes. FNV accepts every individual byte.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 83

<a id="value-666e762d73746174652d6572726f722d6d657373616765"></a>

## `fnv-state-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvStateError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a state-loading diagnostic without exposing implementation-specific type names.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 148

<a id="value-666e762d73746174652d6c6f6164"></a>

## `fnv-state-load`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-load</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvStateError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span>)))</code></pre>

Restore a checkpoint of the template's variant. The template and input remain unchanged.
All bit patterns are valid hash states. Identifier checks precede exact-size checks;
loading one variant into another is an identifier error even when their widths agree.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 156

<a id="value-666e762d73746174652d7265736574"></a>

## `fnv-state-reset`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-reset</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span>)</code></pre>

Return the initial offset basis of the same variant, preserving the original state.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 69

<a id="value-666e762d73746174652d73617665"></a>

## `fnv-state-save`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-save</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Serialize the variant identifier and all hash bits in fresh network-order bytes.
The identifier is fnv followed by a tag from 1 through 6, ordered by width
and then by FNV-1 versus FNV-1a. Checkpoints have lengths 8, 12 or 20.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 133

<a id="value-666e762d73746174652d73697a65"></a>

## `fnv-state-size`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-size</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Number of digest bytes: four, eight or sixteen according to the fixed width.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 76

<a id="value-666e762d73746174652d73756d"></a>

## `fnv-state-sum`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-sum</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return fresh digest bytes in network order without advancing or resetting the state.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 113

<a id="value-666e762d73746174652d757064617465"></a>

## `fnv-state-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-update</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span>))</code></pre>

Continue a state with all input bytes using the ordered native-word kernels.
Input is consumed during this call; no mutable input buffer is retained.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 61

<a id="value-666e762d73746174652d776f72643332"></a>

## `fnv-state-word32`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-word32</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U32</span>))</code></pre>

Observe a 32-bit state as a machine word; other widths return None.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 87

<a id="value-666e762d73746174652d776f72643634"></a>

## `fnv-state-word64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-word64</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">U64</span>))</code></pre>

Observe a 64-bit state as a machine word; other widths return None.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 93

<a id="value-666e762d73746174652d776f726473313238"></a>

## `fnv-state-words128`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv-state-words128</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">U64</span>, <span class="tok-constructor">U64</span>)))</code></pre>

Observe a 128-bit state's high and low words; other widths return None.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 99

<a id="value-666e763132382d7374617465"></a>

## `fnv128-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv128-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span></code></pre>

Initial 128-bit FNV-1 state, retaining both words without truncation.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 52

<a id="value-666e76313238612d7374617465"></a>

## `fnv128a-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv128a-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span></code></pre>

Initial 128-bit FNV-1a state, retaining both words without truncation.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 56

<a id="value-666e7633322d7374617465"></a>

## `fnv32-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv32-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span></code></pre>

Initial 32-bit FNV-1 state, multiplying before xor.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 36

<a id="value-666e763332612d7374617465"></a>

## `fnv32a-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv32a-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span></code></pre>

Initial 32-bit FNV-1a state, applying xor before multiplication.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 40

<a id="value-666e7636342d7374617465"></a>

## `fnv64-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv64-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span></code></pre>

Initial 64-bit FNV-1 state, multiplying before xor.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 44

<a id="value-666e763634612d7374617465"></a>

## `fnv64a-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fnv64a-state</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Fnv</span>.<span class="tok-constructor">State</span>.<span class="tok-constructor">FnvState</span></code></pre>

Initial 64-bit FNV-1a state, applying xor before multiplication.

[Source](/reference-sources/Text.Fnv.State.cinch) · line 48

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Fnv/State.cinch`. Source SHA-256: `32143ed6a2a86719094a7ff3dffcd7811a51d0b7c8aae7630fe1fe0487ecd673`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
