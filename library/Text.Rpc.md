# Text.Rpc

Pure JSON-RPC 1.0/2.0 (single, notification, batch) and length-prefixed gob-RPC wire codecs, envelopes, and standard error codes.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Rpc`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Rpc.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Gob](Text.Gob.md), [Text.Json](Text.Json.md), [Text.Parse](Text.Parse.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Builder](Text.Builder.md), [Text.Gob](Text.Gob.md), [Text.Json](Text.Json.md), [Text.Lex](Text.Lex.md), [Text.Parse](Text.Parse.md), [Text.Rpc](Text.Rpc.md), [Text.Scan](Text.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pure sans-I/O RPC framing and wire codecs for JSON-RPC 1.0, JSON-RPC 2.0, and binary gob-RPC.
Provides request and response models, standard JSON-RPC 2.0 error codes, single and batch
JSON-RPC encoding and decoding with automatic version detection and notification handling,
4-byte big-endian length-prefixed binary framing with configurable size limits, and typed
gob-RPC request/response framing over Text.Gob values.

</details>

# Types and constructors

<a id="type-476f6252706352657175657374"></a>

## `GobRpcRequest`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobRpcRequest</span></code></pre>

Construct a gob-RPC request from method name, sequence identifier, and argument value.

[Source](/reference-sources/Text.Rpc.cinch) · line 654

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobRpcRequest</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcRequest</span>)))</code></pre>

<a id="type-476f62527063526573706f6e7365"></a>

## `GobRpcResponse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">GobRpcResponse</span></code></pre>

Construct a gob-RPC response from sequence identifier and result outcome.

[Source](/reference-sources/Text.Rpc.cinch) · line 671

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">GobRpcResponse</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcResponse</span>))</code></pre>

<a id="type-527063436f6465634b696e64"></a>

## `RpcCodecKind`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RpcCodecKind</span></code></pre>

Wire protocol and serialization format for an RPC channel or message.

[Source](/reference-sources/Text.Rpc.cinch) · line 84

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcCodecJson1</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcCodecJson2</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcCodecGob</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span></code></pre>

<a id="type-5270634572726f72"></a>

## `RpcError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RpcError</span></code></pre>

Structured RPC error carrying a numeric error code, message, and optional JSON detail payload.

[Source](/reference-sources/Text.Rpc.cinch) · line 120

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcErrorDetail</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span>)))</code></pre>

<a id="type-5270634672616d654572726f72"></a>

## `RpcFrameError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RpcFrameError</span></code></pre>

Named errors produced by binary frame encoding, frame decoding, or gob-RPC serialization.

[Source](/reference-sources/Text.Rpc.cinch) · line 177

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcFrameIncomplete</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcFrameTooLarge</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcFrameMalformed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span>)</code></pre>

<a id="type-5270634964"></a>

## `RpcId`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RpcId</span></code></pre>

Correlation identifier matching an RPC response back to its originating request.

[Source](/reference-sources/Text.Rpc.cinch) · line 51

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcIdInt</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcIdString</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcIdNull</span> :: <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span></code></pre>

<a id="type-52706352657175657374"></a>

## `RpcRequest`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RpcRequest</span></code></pre>

Construct an RPC request with method name, JSON params, optional id (None for notification), and codec kind.

[Source](/reference-sources/Text.Rpc.cinch) · line 203

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcRequest</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span>))))</code></pre>

<a id="type-527063526573706f6e7365"></a>

## `RpcResponse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RpcResponse</span></code></pre>

Construct an RPC response from its correlation identifier and result outcome.

[Source](/reference-sources/Text.Rpc.cinch) · line 255

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcResponse</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span>))</code></pre>

# Values

<a id="value-676f622d696e6665722d74797065"></a>

## `gob-infer-type`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gob-infer-type</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobType</span>)</code></pre>

Infer a compatible GobType descriptor directly from a runtime GobValue.

[Source](/reference-sources/Text.Rpc.cinch) · line 695

<a id="value-676f627270632d6465636f64652d72657175657374"></a>

## `gobrpc-decode-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-decode-request</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcRequest</span>, <span class="tok-constructor">Bytes</span>))))</code></pre>

Decode one 4-byte length-prefixed GobRpcRequest frame from a buffer, enforcing max-size.
Returns the decoded request paired with any remaining unread bytes.

[Source](/reference-sources/Text.Rpc.cinch) · line 774

<a id="value-676f627270632d6465636f64652d726571756573742d7061796c6f6164"></a>

## `gobrpc-decode-request-payload`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-decode-request-payload</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcRequest</span>))</code></pre>

Decode a single already-deframed GobRpcRequest gob payload.

[Source](/reference-sources/Text.Rpc.cinch) · line 759

<a id="value-676f627270632d6465636f64652d726573706f6e7365"></a>

## `gobrpc-decode-response`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-decode-response</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcResponse</span>, <span class="tok-constructor">Bytes</span>))))</code></pre>

Decode one 4-byte length-prefixed GobRpcResponse frame from a buffer, enforcing max-size.
Returns the decoded response paired with any remaining unread bytes.

[Source](/reference-sources/Text.Rpc.cinch) · line 854

<a id="value-676f627270632d6465636f64652d726573706f6e73652d7061796c6f6164"></a>

## `gobrpc-decode-response-payload`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-decode-response-payload</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcResponse</span>))</code></pre>

Decode a single already-deframed GobRpcResponse gob payload.

[Source](/reference-sources/Text.Rpc.cinch) · line 822

<a id="value-676f627270632d656e636f64652d72657175657374"></a>

## `gobrpc-encode-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-encode-request</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Encode a GobRpcRequest into a 4-byte length-prefixed gob frame with the default maximum frame size.

[Source](/reference-sources/Text.Rpc.cinch) · line 740

<a id="value-676f627270632d656e636f64652d726571756573742d6c696d6974"></a>

## `gobrpc-encode-request-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-encode-request-limit</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Encode a GobRpcRequest into a 4-byte length-prefixed gob frame enforcing max-size.

[Source](/reference-sources/Text.Rpc.cinch) · line 744

<a id="value-676f627270632d656e636f64652d726573706f6e7365"></a>

## `gobrpc-encode-response`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-encode-response</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcResponse</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Encode a GobRpcResponse into a 4-byte length-prefixed gob frame with the default maximum frame size.

[Source](/reference-sources/Text.Rpc.cinch) · line 782

<a id="value-676f627270632d656e636f64652d726573706f6e73652d6c696d6974"></a>

## `gobrpc-encode-response-limit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-encode-response-limit</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcResponse</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Encode a GobRpcResponse into a 4-byte length-prefixed gob frame enforcing max-size.

[Source](/reference-sources/Text.Rpc.cinch) · line 786

<a id="value-676f627270632d726571756573742d626f6479"></a>

## `gobrpc-request-body`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-request-body</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcRequest</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)</code></pre>

Return the argument GobValue of a GobRpcRequest.

[Source](/reference-sources/Text.Rpc.cinch) · line 665

<a id="value-676f627270632d726571756573742d6d6574686f64"></a>

## `gobrpc-request-method`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-request-method</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the method name of a GobRpcRequest.

[Source](/reference-sources/Text.Rpc.cinch) · line 657

<a id="value-676f627270632d726571756573742d736571"></a>

## `gobrpc-request-seq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-request-seq</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcRequest</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the sequence identifier of a GobRpcRequest.

[Source](/reference-sources/Text.Rpc.cinch) · line 661

<a id="value-676f627270632d726573706f6e73652d726573756c74"></a>

## `gobrpc-response-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-response-result</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcResponse</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>))</code></pre>

Return the result outcome of a GobRpcResponse.

[Source](/reference-sources/Text.Rpc.cinch) · line 678

<a id="value-676f627270632d726573706f6e73652d736571"></a>

## `gobrpc-response-seq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">gobrpc-response-seq</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcResponse</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the sequence identifier of a GobRpcResponse.

[Source](/reference-sources/Text.Rpc.cinch) · line 674

<a id="value-6a736f6e7270632d6465636f64652d6261746368"></a>

## `jsonrpc-decode-batch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-decode-batch</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span>)))</code></pre>

Decode a non-empty JSON-RPC 2.0 batch array where every element must be a valid request.

[Source](/reference-sources/Text.Rpc.cinch) · line 576

<a id="value-6a736f6e7270632d6465636f64652d62617463682d6974656d73"></a>

## `jsonrpc-decode-batch-items`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-decode-batch-items</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span>))))</code></pre>

Decode a JSON-RPC 2.0 batch array preserving per-item validation errors for server dispatch.
Returns Parse error on invalid JSON and Invalid Request on empty arrays or non-array inputs.

[Source](/reference-sources/Text.Rpc.cinch) · line 557

<a id="value-6a736f6e7270632d6465636f64652d62617463682d726573706f6e736573"></a>

## `jsonrpc-decode-batch-responses`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-decode-batch-responses</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span>)))</code></pre>

Decode a JSON-RPC 2.0 batch response array from UTF-8 JSON bytes.

[Source](/reference-sources/Text.Rpc.cinch) · line 595

<a id="value-6a736f6e7270632d6465636f64652d72657175657374"></a>

## `jsonrpc-decode-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-decode-request</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span>))</code></pre>

Decode a single JSON-RPC 1.0 or 2.0 request from UTF-8 JSON bytes.

[Source](/reference-sources/Text.Rpc.cinch) · line 472

<a id="value-6a736f6e7270632d6465636f64652d726573706f6e7365"></a>

## `jsonrpc-decode-response`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-decode-response</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span>))</code></pre>

Decode a single JSON-RPC 1.0 or 2.0 response from UTF-8 JSON bytes.

[Source](/reference-sources/Text.Rpc.cinch) · line 540

<a id="value-6a736f6e7270632d656e636f64652d6261746368"></a>

## `jsonrpc-encode-batch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-encode-batch</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a list of JSON-RPC requests into a newline-terminated JSON-RPC 2.0 batch array.

[Source](/reference-sources/Text.Rpc.cinch) · line 546

<a id="value-6a736f6e7270632d656e636f64652d62617463682d726573706f6e736573"></a>

## `jsonrpc-encode-batch-responses`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-encode-batch-responses</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode a list of JSON-RPC 2.0 responses into a newline-terminated JSON batch array.

[Source](/reference-sources/Text.Rpc.cinch) · line 582

<a id="value-6a736f6e7270632d656e636f64652d72657175657374"></a>

## `jsonrpc-encode-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-encode-request</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode an RpcRequest as newline-delimited JSON-RPC 1.0 or 2.0 bytes.

[Source](/reference-sources/Text.Rpc.cinch) · line 418

<a id="value-6a736f6e7270632d656e636f64652d726573706f6e7365"></a>

## `jsonrpc-encode-response`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-encode-response</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Encode an RpcResponse as newline-delimited JSON-RPC 2.0 bytes.

[Source](/reference-sources/Text.Rpc.cinch) · line 489

<a id="value-6a736f6e7270632d656e636f64652d726573706f6e73652d77697468"></a>

## `jsonrpc-encode-response-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-encode-response-with</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode an RpcResponse using either JSON-RPC 1.0 or JSON-RPC 2.0 wire format.

[Source](/reference-sources/Text.Rpc.cinch) · line 493

<a id="value-6a736f6e7270632d69732d6261746368"></a>

## `jsonrpc-is-batch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">jsonrpc-is-batch</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return True when the first non-whitespace byte of the payload is '[' (indicating a JSON-RPC batch).

[Source](/reference-sources/Text.Rpc.cinch) · line 340

<a id="value-7270632d636f6465632d6571"></a>

## `rpc-codec-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-codec-eq</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare two RPC codec kinds for equality.

[Source](/reference-sources/Text.Rpc.cinch) · line 93

<a id="value-7270632d64656661756c742d6d61782d6672616d652d6279746573"></a>

## `rpc-default-max-frame-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-default-max-frame-bytes</span> :: <span class="tok-constructor">Nat</span></code></pre>

Default maximum binary frame size in bytes (16 MiB).

[Source](/reference-sources/Text.Rpc.cinch) · line 193

<a id="value-7270632d6572722d696e7465726e616c2d6572726f72"></a>

## `rpc-err-internal-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-err-internal-error</span> :: <span class="tok-constructor">I64</span></code></pre>

Standard JSON-RPC 2.0 internal error code (-32603).

[Source](/reference-sources/Text.Rpc.cinch) · line 116

<a id="value-7270632d6572722d696e76616c69642d706172616d73"></a>

## `rpc-err-invalid-params`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-err-invalid-params</span> :: <span class="tok-constructor">I64</span></code></pre>

Standard JSON-RPC 2.0 invalid params error code (-32602).

[Source](/reference-sources/Text.Rpc.cinch) · line 112

<a id="value-7270632d6572722d696e76616c69642d72657175657374"></a>

## `rpc-err-invalid-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-err-invalid-request</span> :: <span class="tok-constructor">I64</span></code></pre>

Standard JSON-RPC 2.0 invalid request error code (-32600).

[Source](/reference-sources/Text.Rpc.cinch) · line 104

<a id="value-7270632d6572722d6d6574686f642d6e6f742d666f756e64"></a>

## `rpc-err-method-not-found`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-err-method-not-found</span> :: <span class="tok-constructor">I64</span></code></pre>

Standard JSON-RPC 2.0 method not found error code (-32601).

[Source](/reference-sources/Text.Rpc.cinch) · line 108

<a id="value-7270632d6572722d70617273652d6572726f72"></a>

## `rpc-err-parse-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-err-parse-error</span> :: <span class="tok-constructor">I64</span></code></pre>

Standard JSON-RPC 2.0 parse error code (-32700).

[Source](/reference-sources/Text.Rpc.cinch) · line 100

<a id="value-7270632d6572726f72"></a>

## `rpc-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-error</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span>))</code></pre>

Construct an RpcError with no attached detail payload.

[Source](/reference-sources/Text.Rpc.cinch) · line 125

<a id="value-7270632d6572726f722d636f6465"></a>

## `rpc-error-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-error-code</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the signed 64-bit error code of an RpcError.

[Source](/reference-sources/Text.Rpc.cinch) · line 153

<a id="value-7270632d6572726f722d64617461"></a>

## `rpc-error-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-error-data</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>))</code></pre>

Return the optional JSON detail payload of an RpcError.

[Source](/reference-sources/Text.Rpc.cinch) · line 161

<a id="value-7270632d6572726f722d6571"></a>

## `rpc-error-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-error-eq</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare two RpcError values for structural equality.

[Source](/reference-sources/Text.Rpc.cinch) · line 171

<a id="value-7270632d6572726f722d6d657373616765"></a>

## `rpc-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the human-readable error message of an RpcError.

[Source](/reference-sources/Text.Rpc.cinch) · line 157

<a id="value-7270632d6572726f722d776974682d64617461"></a>

## `rpc-error-with-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-error-with-data</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span>)))</code></pre>

Construct an RpcError with an attached JSON detail payload.

[Source](/reference-sources/Text.Rpc.cinch) · line 129

<a id="value-7270632d6672616d652d6465636f6465"></a>

## `rpc-frame-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-frame-decode</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>))))</code></pre>

Decode one 4-byte big-endian length-prefixed frame from a streaming byte buffer, enforcing max-size.
Returns the extracted payload paired with any trailing unread bytes.

[Source](/reference-sources/Text.Rpc.cinch) · line 630

<a id="value-7270632d6672616d652d656e636f6465"></a>

## `rpc-frame-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-frame-encode</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Wrap a payload in a 4-byte big-endian length-prefixed frame, enforcing max-size (0 disables custom cap).

[Source](/reference-sources/Text.Rpc.cinch) · line 614

<a id="value-7270632d6672616d652d6572726f722d6d657373616765"></a>

## `rpc-frame-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-frame-error-message</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcFrameError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a readable diagnostic message for an RpcFrameError.

[Source](/reference-sources/Text.Rpc.cinch) · line 186

<a id="value-7270632d69642d6571"></a>

## `rpc-id-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-id-eq</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare two RPC correlation identifiers for exact equality.

[Source](/reference-sources/Text.Rpc.cinch) · line 60

<a id="value-7270632d69642d66726f6d2d6a736f6e"></a>

## `rpc-id-from-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-id-from-json</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span>))</code></pre>

Parse a Json scalar into an RpcId, rejecting floats, booleans, arrays, and objects.

[Source](/reference-sources/Text.Rpc.cinch) · line 74

<a id="value-7270632d69642d746f2d6a736f6e"></a>

## `rpc-id-to-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-id-to-json</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

Convert an RpcId into its wire Json representation.

[Source](/reference-sources/Text.Rpc.cinch) · line 67

<a id="value-7270632d696e7465726e616c2d6572726f72"></a>

## `rpc-internal-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-internal-error</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span>)</code></pre>

Construct a standard JSON-RPC 2.0 internal error (-32603).

[Source](/reference-sources/Text.Rpc.cinch) · line 149

<a id="value-7270632d696e76616c69642d706172616d73"></a>

## `rpc-invalid-params`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-invalid-params</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span>)</code></pre>

Construct a standard JSON-RPC 2.0 invalid params error (-32602).

[Source](/reference-sources/Text.Rpc.cinch) · line 145

<a id="value-7270632d696e76616c69642d72657175657374"></a>

## `rpc-invalid-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-invalid-request</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span>)</code></pre>

Construct a standard JSON-RPC 2.0 invalid request error (-32600).

[Source](/reference-sources/Text.Rpc.cinch) · line 137

<a id="value-7270632d6d6574686f642d6e6f742d666f756e64"></a>

## `rpc-method-not-found`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-method-not-found</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span>)</code></pre>

Construct a standard JSON-RPC 2.0 method not found error (-32601).

[Source](/reference-sources/Text.Rpc.cinch) · line 141

<a id="value-7270632d6e6f74696669636174696f6e"></a>

## `rpc-notification`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-notification</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span>))</code></pre>

Construct a JSON-RPC 2.0 notification (carrying no correlation identifier).

[Source](/reference-sources/Text.Rpc.cinch) · line 214

<a id="value-7270632d70617273652d6572726f72"></a>

## `rpc-parse-error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-parse-error</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span>)</code></pre>

Construct a standard JSON-RPC 2.0 parse error (-32700).

[Source](/reference-sources/Text.Rpc.cinch) · line 133

<a id="value-7270632d72657175657374"></a>

## `rpc-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-request</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span>)))</code></pre>

Construct a JSON-RPC 2.0 request with an explicit correlation identifier.

[Source](/reference-sources/Text.Rpc.cinch) · line 206

<a id="value-7270632d726571756573742d636f646563"></a>

## `rpc-request-codec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-request-codec</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span>)</code></pre>

Return the wire codec kind associated with an RpcRequest.

[Source](/reference-sources/Text.Rpc.cinch) · line 230

<a id="value-7270632d726571756573742d6571"></a>

## `rpc-request-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-request-eq</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare two RpcRequest values for structural equality.

[Source](/reference-sources/Text.Rpc.cinch) · line 246

<a id="value-7270632d726571756573742d6964"></a>

## `rpc-request-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-request-id</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span>))</code></pre>

Return the optional correlation identifier of an RpcRequest (None indicates a notification).

[Source](/reference-sources/Text.Rpc.cinch) · line 226

<a id="value-7270632d726571756573742d69732d6e6f74696669636174696f6e"></a>

## `rpc-request-is-notification`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-request-is-notification</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return True when the request is a notification expecting no response.

[Source](/reference-sources/Text.Rpc.cinch) · line 234

<a id="value-7270632d726571756573742d6d6574686f64"></a>

## `rpc-request-method`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-request-method</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the method name of an RpcRequest.

[Source](/reference-sources/Text.Rpc.cinch) · line 218

<a id="value-7270632d726571756573742d706172616d73"></a>

## `rpc-request-params`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-request-params</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)</code></pre>

Return the JSON parameters payload of an RpcRequest.

[Source](/reference-sources/Text.Rpc.cinch) · line 222

<a id="value-7270632d726571756573742d7631"></a>

## `rpc-request-v1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-request-v1</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span>)))</code></pre>

Construct a JSON-RPC 1.0 request with an explicit correlation identifier.

[Source](/reference-sources/Text.Rpc.cinch) · line 210

<a id="value-7270632d726573706f6e73652d6571"></a>

## `rpc-response-eq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-response-eq</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Compare two RpcResponse values for structural equality.

[Source](/reference-sources/Text.Rpc.cinch) · line 274

<a id="value-7270632d726573706f6e73652d657272"></a>

## `rpc-response-err`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-response-err</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span>))</code></pre>

Construct an error RpcResponse carrying a structured RpcError.

[Source](/reference-sources/Text.Rpc.cinch) · line 262

<a id="value-7270632d726573706f6e73652d6964"></a>

## `rpc-response-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-response-id</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span>)</code></pre>

Return the correlation identifier of an RpcResponse.

[Source](/reference-sources/Text.Rpc.cinch) · line 266

<a id="value-7270632d726573706f6e73652d6f6b"></a>

## `rpc-response-ok`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-response-ok</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcId</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span>))</code></pre>

Construct a successful RpcResponse carrying a JSON result value.

[Source](/reference-sources/Text.Rpc.cinch) · line 258

<a id="value-7270632d726573706f6e73652d726573756c74"></a>

## `rpc-response-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-response-result</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>))</code></pre>

Return the Result outcome of an RpcResponse.

[Source](/reference-sources/Text.Rpc.cinch) · line 270

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Rpc.cinch`. Source SHA-256: `6076b5e4850f3562a4e36066ec654fcad9d571b41fcd2cb1f07425e37cc55cdb`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
