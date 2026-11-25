# Text.Rpc

Pure JSON-RPC 1.0/2.0 (single, notification, batch) and length-prefixed gob-RPC wire codecs, envelopes, and standard error codes.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Rpc</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Rpc/source/">View source</a><a href="/packages/search/?q=%2BText.Rpc">Search module</a><a href="/reference-sources/Text.Rpc.cinch" download="Text.Rpc.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Gob](Text.Gob.md), [Text.Json](Text.Json.md), [Text.Parse](Text.Parse.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Pure sans-I/O RPC framing and wire codecs for JSON-RPC 1.0, JSON-RPC 2.0, and binary gob-RPC.
Provides request and response models, standard JSON-RPC 2.0 error codes, single and batch
JSON-RPC encoding and decoding with automatic version detection and notification handling,
4-byte big-endian length-prefixed binary framing with configurable size limits, and typed
gob-RPC request/response framing over Text.Gob values.

</details>

<details class="doc-synopsis"><summary>Synopsis (71 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-GobRpcRequest">GobRpcRequest</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobRpcRequest</strong></code></li><li><a class="syn-name" href="#t-GobRpcResponse">GobRpcResponse</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobRpcResponse</strong></code></li><li><a class="syn-name" href="#t-RpcCodecKind">RpcCodecKind</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcCodecKind</strong></code></li><li><a class="syn-name" href="#t-RpcError">RpcError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcError</strong></code></li><li><a class="syn-name" href="#t-RpcFrameError">RpcFrameError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcFrameError</strong></code></li><li><a class="syn-name" href="#t-RpcId">RpcId</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcId</strong></code></li><li><a class="syn-name" href="#t-RpcRequest">RpcRequest</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcRequest</strong></code></li><li><a class="syn-name" href="#t-RpcResponse">RpcResponse</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcResponse</strong></code></li><li><a class="syn-name" href="#v-gob-infer-type">gob-infer-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gob-infer-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code></li><li><a class="syn-name" href="#v-gobrpc-decode-request">gobrpc-decode-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-decode-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-gobrpc-decode-request-payload">gobrpc-decode-request-payload</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-decode-request-payload</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a></code></li><li><a class="syn-name" href="#v-gobrpc-decode-response">gobrpc-decode-response</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-decode-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-gobrpc-decode-response-payload">gobrpc-decode-response-payload</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-decode-response-payload</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a></code></li><li><a class="syn-name" href="#v-gobrpc-encode-request">gobrpc-encode-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-encode-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gobrpc-encode-request-limit">gobrpc-encode-request-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-encode-request-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gobrpc-encode-response">gobrpc-encode-response</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-encode-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gobrpc-encode-response-limit">gobrpc-encode-response-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-encode-response-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-gobrpc-request-body">gobrpc-request-body</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-request-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li><a class="syn-name" href="#v-gobrpc-request-method">gobrpc-request-method</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-request-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-gobrpc-request-seq">gobrpc-request-seq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-request-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-gobrpc-response-result">gobrpc-response-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-response-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code></li><li><a class="syn-name" href="#v-gobrpc-response-seq">gobrpc-response-seq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">gobrpc-response-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-jsonrpc-decode-batch">jsonrpc-decode-batch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-decode-batch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a>)</code></li><li><a class="syn-name" href="#v-jsonrpc-decode-batch-items">jsonrpc-decode-batch-items</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-decode-batch-items</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a>))</code></li><li><a class="syn-name" href="#v-jsonrpc-decode-batch-responses">jsonrpc-decode-batch-responses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-decode-batch-responses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a>)</code></li><li><a class="syn-name" href="#v-jsonrpc-decode-request">jsonrpc-decode-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-decode-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a></code></li><li><a class="syn-name" href="#v-jsonrpc-decode-response">jsonrpc-decode-response</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-decode-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a></code></li><li><a class="syn-name" href="#v-jsonrpc-encode-batch">jsonrpc-encode-batch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-encode-batch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-jsonrpc-encode-batch-responses">jsonrpc-encode-batch-responses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-encode-batch-responses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-jsonrpc-encode-request">jsonrpc-encode-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-encode-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-jsonrpc-encode-response">jsonrpc-encode-response</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-encode-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-jsonrpc-encode-response-with">jsonrpc-encode-response-with</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-encode-response-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-jsonrpc-is-batch">jsonrpc-is-batch</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">jsonrpc-is-batch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rpc-codec-eq">rpc-codec-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-codec-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rpc-default-max-frame-bytes">rpc-default-max-frame-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-default-max-frame-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-rpc-err-internal-error">rpc-err-internal-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-err-internal-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-rpc-err-invalid-params">rpc-err-invalid-params</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-err-invalid-params</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-rpc-err-invalid-request">rpc-err-invalid-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-err-invalid-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-rpc-err-method-not-found">rpc-err-method-not-found</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-err-method-not-found</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-rpc-err-parse-error">rpc-err-parse-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-err-parse-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-rpc-error">rpc-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code></li><li><a class="syn-name" href="#v-rpc-error-code">rpc-error-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-error-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-rpc-error-data">rpc-error-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-error-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-rpc-error-eq">rpc-error-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-error-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rpc-error-message">rpc-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-rpc-error-with-data">rpc-error-with-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-error-with-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code></li><li><a class="syn-name" href="#v-rpc-frame-decode">rpc-frame-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-frame-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-rpc-frame-encode">rpc-frame-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-frame-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-rpc-frame-error-message">rpc-frame-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-frame-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-rpc-id-eq">rpc-id-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-id-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rpc-id-from-json">rpc-id-from-json</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-id-from-json</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a></code></li><li><a class="syn-name" href="#v-rpc-id-to-json">rpc-id-to-json</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-id-to-json</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-rpc-internal-error">rpc-internal-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-internal-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code></li><li><a class="syn-name" href="#v-rpc-invalid-params">rpc-invalid-params</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-invalid-params</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code></li><li><a class="syn-name" href="#v-rpc-invalid-request">rpc-invalid-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-invalid-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code></li><li><a class="syn-name" href="#v-rpc-method-not-found">rpc-method-not-found</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-method-not-found</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code></li><li><a class="syn-name" href="#v-rpc-notification">rpc-notification</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-notification</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a></code></li><li><a class="syn-name" href="#v-rpc-parse-error">rpc-parse-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-parse-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code></li><li><a class="syn-name" href="#v-rpc-request">rpc-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a></code></li><li><a class="syn-name" href="#v-rpc-request-codec">rpc-request-codec</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-request-codec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a></code></li><li><a class="syn-name" href="#v-rpc-request-eq">rpc-request-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-request-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rpc-request-id">rpc-request-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-request-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a></code></li><li><a class="syn-name" href="#v-rpc-request-is-notification">rpc-request-is-notification</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-request-is-notification</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rpc-request-method">rpc-request-method</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-request-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-rpc-request-params">rpc-request-params</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-request-params</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li><a class="syn-name" href="#v-rpc-request-v1">rpc-request-v1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-request-v1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a></code></li><li><a class="syn-name" href="#v-rpc-response-eq">rpc-response-eq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-response-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-rpc-response-err">rpc-response-err</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-response-err</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a></code></li><li><a class="syn-name" href="#v-rpc-response-id">rpc-response-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-response-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a></code></li><li><a class="syn-name" href="#v-rpc-response-ok">rpc-response-ok</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-response-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a></code></li><li><a class="syn-name" href="#v-rpc-response-result">rpc-response-result</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rpc-response-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-GobRpcRequest">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobRpcRequest</strong></code><span class="doc-sig-links"><a href="#t-GobRpcRequest" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L654" title="View source at line 654">Source</a></span></div>
<div class="doc-body">

Construct a gob-RPC request from method name, sequence identifier, and argument value.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GobRpcRequest"><code><strong class="tok-constructor">GobRpcRequest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-GobRpcResponse">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">GobRpcResponse</strong></code><span class="doc-sig-links"><a href="#t-GobRpcResponse" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L671" title="View source at line 671">Source</a></span></div>
<div class="doc-body">

Construct a gob-RPC response from sequence identifier and result outcome.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-GobRpcResponse"><code><strong class="tok-constructor">GobRpcResponse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-RpcCodecKind">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcCodecKind</strong></code><span class="doc-sig-links"><a href="#t-RpcCodecKind" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

Wire protocol and serialization format for an RPC channel or message.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RpcCodecJson1"><code><strong class="tok-constructor">RpcCodecJson1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a></code></li><li id="k-RpcCodecJson2"><code><strong class="tok-constructor">RpcCodecJson2</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a></code></li><li id="k-RpcCodecGob"><code><strong class="tok-constructor">RpcCodecGob</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-RpcError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcError</strong></code><span class="doc-sig-links"><a href="#t-RpcError" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

Structured RPC error carrying a numeric error code, message, and optional JSON detail payload.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RpcErrorDetail"><code><strong class="tok-constructor">RpcErrorDetail</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-RpcFrameError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcFrameError</strong></code><span class="doc-sig-links"><a href="#t-RpcFrameError" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

Named errors produced by binary frame encoding, frame decoding, or gob-RPC serialization.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RpcFrameIncomplete"><code><strong class="tok-constructor">RpcFrameIncomplete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a></code></li><li id="k-RpcFrameTooLarge"><code><strong class="tok-constructor">RpcFrameTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a></code></li><li id="k-RpcFrameMalformed"><code><strong class="tok-constructor">RpcFrameMalformed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-RpcId">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcId</strong></code><span class="doc-sig-links"><a href="#t-RpcId" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Correlation identifier matching an RPC response back to its originating request.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RpcIdInt"><code><strong class="tok-constructor">RpcIdInt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a></code></li><li id="k-RpcIdString"><code><strong class="tok-constructor">RpcIdString</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a></code></li><li id="k-RpcIdNull"><code><strong class="tok-constructor">RpcIdNull</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-RpcRequest">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcRequest</strong></code><span class="doc-sig-links"><a href="#t-RpcRequest" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L203" title="View source at line 203">Source</a></span></div>
<div class="doc-body">

Construct an RPC request with method name, JSON params, optional id (None for notification), and codec kind.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RpcRequest"><code><strong class="tok-constructor">RpcRequest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-RpcResponse">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RpcResponse</strong></code><span class="doc-sig-links"><a href="#t-RpcResponse" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L255" title="View source at line 255">Source</a></span></div>
<div class="doc-body">

Construct an RPC response from its correlation identifier and result outcome.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RpcResponse"><code><strong class="tok-constructor">RpcResponse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-gob-infer-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gob-infer-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobType" title="Text.Gob.GobType">GobType</a></code><span class="doc-sig-links"><a href="#v-gob-infer-type" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L695" title="View source at line 695">Source</a></span></div>
<div class="doc-body">

Infer a compatible GobType descriptor directly from a runtime GobValue.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-decode-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-decode-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-gobrpc-decode-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L774" title="View source at line 774">Source</a></span></div>
<div class="doc-body">

Decode one 4-byte length-prefixed GobRpcRequest frame from a buffer, enforcing max-size.
Returns the decoded request paired with any remaining unread bytes.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-decode-request-payload">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-decode-request-payload</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a></code><span class="doc-sig-links"><a href="#v-gobrpc-decode-request-payload" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L759" title="View source at line 759">Source</a></span></div>
<div class="doc-body">

Decode a single already-deframed GobRpcRequest gob payload.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-decode-response">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-decode-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-gobrpc-decode-response" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L854" title="View source at line 854">Source</a></span></div>
<div class="doc-body">

Decode one 4-byte length-prefixed GobRpcResponse frame from a buffer, enforcing max-size.
Returns the decoded response paired with any remaining unread bytes.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-decode-response-payload">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-decode-response-payload</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a></code><span class="doc-sig-links"><a href="#v-gobrpc-decode-response-payload" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L822" title="View source at line 822">Source</a></span></div>
<div class="doc-body">

Decode a single already-deframed GobRpcResponse gob payload.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-encode-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-encode-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gobrpc-encode-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L740" title="View source at line 740">Source</a></span></div>
<div class="doc-body">

Encode a GobRpcRequest into a 4-byte length-prefixed gob frame with the default maximum frame size.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-encode-request-limit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-encode-request-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gobrpc-encode-request-limit" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L744" title="View source at line 744">Source</a></span></div>
<div class="doc-body">

Encode a GobRpcRequest into a 4-byte length-prefixed gob frame enforcing max-size.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-encode-response">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-encode-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gobrpc-encode-response" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L782" title="View source at line 782">Source</a></span></div>
<div class="doc-body">

Encode a GobRpcResponse into a 4-byte length-prefixed gob frame with the default maximum frame size.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-encode-response-limit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-encode-response-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-gobrpc-encode-response-limit" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L786" title="View source at line 786">Source</a></span></div>
<div class="doc-body">

Encode a GobRpcResponse into a 4-byte length-prefixed gob frame enforcing max-size.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-request-body">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-request-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code><span class="doc-sig-links"><a href="#v-gobrpc-request-body" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L665" title="View source at line 665">Source</a></span></div>
<div class="doc-body">

Return the argument GobValue of a GobRpcRequest.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-request-method">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-request-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-gobrpc-request-method" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L657" title="View source at line 657">Source</a></span></div>
<div class="doc-body">

Return the method name of a GobRpcRequest.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-request-seq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-request-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcRequest" title="Text.Rpc.GobRpcRequest">GobRpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-gobrpc-request-seq" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L661" title="View source at line 661">Source</a></span></div>
<div class="doc-body">

Return the sequence identifier of a GobRpcRequest.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-response-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-response-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Gob/#t-GobValue" title="Text.Gob.GobValue">GobValue</a></code><span class="doc-sig-links"><a href="#v-gobrpc-response-result" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L678" title="View source at line 678">Source</a></span></div>
<div class="doc-body">

Return the result outcome of a GobRpcResponse.

</div>
</article>

<article class="doc-entry" id="v-gobrpc-response-seq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">gobrpc-response-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-GobRpcResponse" title="Text.Rpc.GobRpcResponse">GobRpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-gobrpc-response-seq" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L674" title="View source at line 674">Source</a></span></div>
<div class="doc-body">

Return the sequence identifier of a GobRpcResponse.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-decode-batch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-decode-batch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a>)</code><span class="doc-sig-links"><a href="#v-jsonrpc-decode-batch" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L576" title="View source at line 576">Source</a></span></div>
<div class="doc-body">

Decode a non-empty JSON-RPC 2.0 batch array where every element must be a valid request.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-decode-batch-items">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-decode-batch-items</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a>))</code><span class="doc-sig-links"><a href="#v-jsonrpc-decode-batch-items" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L557" title="View source at line 557">Source</a></span></div>
<div class="doc-body">

Decode a JSON-RPC 2.0 batch array preserving per-item validation errors for server dispatch.
Returns Parse error on invalid JSON and Invalid Request on empty arrays or non-array inputs.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-decode-batch-responses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-decode-batch-responses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a>)</code><span class="doc-sig-links"><a href="#v-jsonrpc-decode-batch-responses" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L595" title="View source at line 595">Source</a></span></div>
<div class="doc-body">

Decode a JSON-RPC 2.0 batch response array from UTF-8 JSON bytes.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-decode-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-decode-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a></code><span class="doc-sig-links"><a href="#v-jsonrpc-decode-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L472" title="View source at line 472">Source</a></span></div>
<div class="doc-body">

Decode a single JSON-RPC 1.0 or 2.0 request from UTF-8 JSON bytes.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-decode-response">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-decode-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a></code><span class="doc-sig-links"><a href="#v-jsonrpc-decode-response" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L540" title="View source at line 540">Source</a></span></div>
<div class="doc-body">

Decode a single JSON-RPC 1.0 or 2.0 response from UTF-8 JSON bytes.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-encode-batch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-encode-batch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-jsonrpc-encode-batch" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L546" title="View source at line 546">Source</a></span></div>
<div class="doc-body">

Encode a list of JSON-RPC requests into a newline-terminated JSON-RPC 2.0 batch array.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-encode-batch-responses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-encode-batch-responses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-jsonrpc-encode-batch-responses" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L582" title="View source at line 582">Source</a></span></div>
<div class="doc-body">

Encode a list of JSON-RPC 2.0 responses into a newline-terminated JSON batch array.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-encode-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-encode-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-jsonrpc-encode-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L418" title="View source at line 418">Source</a></span></div>
<div class="doc-body">

Encode an RpcRequest as newline-delimited JSON-RPC 1.0 or 2.0 bytes.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-encode-response">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-encode-response</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-jsonrpc-encode-response" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L489" title="View source at line 489">Source</a></span></div>
<div class="doc-body">

Encode an RpcResponse as newline-delimited JSON-RPC 2.0 bytes.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-encode-response-with">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-encode-response-with</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-jsonrpc-encode-response-with" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L493" title="View source at line 493">Source</a></span></div>
<div class="doc-body">

Encode an RpcResponse using either JSON-RPC 1.0 or JSON-RPC 2.0 wire format.

</div>
</article>

<article class="doc-entry" id="v-jsonrpc-is-batch">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">jsonrpc-is-batch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-jsonrpc-is-batch" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L340" title="View source at line 340">Source</a></span></div>
<div class="doc-body">

Return True when the first non-whitespace byte of the payload is '[' (indicating a JSON-RPC batch).

</div>
</article>

<article class="doc-entry" id="v-rpc-codec-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-codec-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-rpc-codec-eq" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

Compare two RPC codec kinds for equality.

</div>
</article>

<article class="doc-entry" id="v-rpc-default-max-frame-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-default-max-frame-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rpc-default-max-frame-bytes" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

Default maximum binary frame size in bytes (16 MiB).

</div>
</article>

<article class="doc-entry" id="v-rpc-err-internal-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-err-internal-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-rpc-err-internal-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

Standard JSON-RPC 2.0 internal error code (-32603).

</div>
</article>

<article class="doc-entry" id="v-rpc-err-invalid-params">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-err-invalid-params</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-rpc-err-invalid-params" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

Standard JSON-RPC 2.0 invalid params error code (-32602).

</div>
</article>

<article class="doc-entry" id="v-rpc-err-invalid-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-err-invalid-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-rpc-err-invalid-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

Standard JSON-RPC 2.0 invalid request error code (-32600).

</div>
</article>

<article class="doc-entry" id="v-rpc-err-method-not-found">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-err-method-not-found</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-rpc-err-method-not-found" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Standard JSON-RPC 2.0 method not found error code (-32601).

</div>
</article>

<article class="doc-entry" id="v-rpc-err-parse-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-err-parse-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-rpc-err-parse-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

Standard JSON-RPC 2.0 parse error code (-32700).

</div>
</article>

<article class="doc-entry" id="v-rpc-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code><span class="doc-sig-links"><a href="#v-rpc-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

Construct an RpcError with no attached detail payload.

</div>
</article>

<article class="doc-entry" id="v-rpc-error-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-error-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-rpc-error-code" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

Return the signed 64-bit error code of an RpcError.

</div>
</article>

<article class="doc-entry" id="v-rpc-error-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-error-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-rpc-error-data" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

Return the optional JSON detail payload of an RpcError.

</div>
</article>

<article class="doc-entry" id="v-rpc-error-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-error-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-rpc-error-eq" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L171" title="View source at line 171">Source</a></span></div>
<div class="doc-body">

Compare two RpcError values for structural equality.

</div>
</article>

<article class="doc-entry" id="v-rpc-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-rpc-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Return the human-readable error message of an RpcError.

</div>
</article>

<article class="doc-entry" id="v-rpc-error-with-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-error-with-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code><span class="doc-sig-links"><a href="#v-rpc-error-with-data" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

Construct an RpcError with an attached JSON detail payload.

</div>
</article>

<article class="doc-entry" id="v-rpc-frame-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-frame-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-rpc-frame-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L630" title="View source at line 630">Source</a></span></div>
<div class="doc-body">

Decode one 4-byte big-endian length-prefixed frame from a streaming byte buffer, enforcing max-size.
Returns the extracted payload paired with any trailing unread bytes.

</div>
</article>

<article class="doc-entry" id="v-rpc-frame-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-frame-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-rpc-frame-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L614" title="View source at line 614">Source</a></span></div>
<div class="doc-body">

Wrap a payload in a 4-byte big-endian length-prefixed frame, enforcing max-size (0 disables custom cap).

</div>
</article>

<article class="doc-entry" id="v-rpc-frame-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-frame-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcFrameError" title="Text.Rpc.RpcFrameError">RpcFrameError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-rpc-frame-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L186" title="View source at line 186">Source</a></span></div>
<div class="doc-body">

Render a readable diagnostic message for an RpcFrameError.

</div>
</article>

<article class="doc-entry" id="v-rpc-id-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-id-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-rpc-id-eq" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

Compare two RPC correlation identifiers for exact equality.

</div>
</article>

<article class="doc-entry" id="v-rpc-id-from-json">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-id-from-json</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a></code><span class="doc-sig-links"><a href="#v-rpc-id-from-json" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Parse a Json scalar into an RpcId, rejecting floats, booleans, arrays, and objects.

</div>
</article>

<article class="doc-entry" id="v-rpc-id-to-json">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-id-to-json</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-rpc-id-to-json" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Convert an RpcId into its wire Json representation.

</div>
</article>

<article class="doc-entry" id="v-rpc-internal-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-internal-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code><span class="doc-sig-links"><a href="#v-rpc-internal-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L149" title="View source at line 149">Source</a></span></div>
<div class="doc-body">

Construct a standard JSON-RPC 2.0 internal error (-32603).

</div>
</article>

<article class="doc-entry" id="v-rpc-invalid-params">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-invalid-params</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code><span class="doc-sig-links"><a href="#v-rpc-invalid-params" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L145" title="View source at line 145">Source</a></span></div>
<div class="doc-body">

Construct a standard JSON-RPC 2.0 invalid params error (-32602).

</div>
</article>

<article class="doc-entry" id="v-rpc-invalid-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-invalid-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code><span class="doc-sig-links"><a href="#v-rpc-invalid-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

Construct a standard JSON-RPC 2.0 invalid request error (-32600).

</div>
</article>

<article class="doc-entry" id="v-rpc-method-not-found">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-method-not-found</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code><span class="doc-sig-links"><a href="#v-rpc-method-not-found" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Construct a standard JSON-RPC 2.0 method not found error (-32601).

</div>
</article>

<article class="doc-entry" id="v-rpc-notification">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-notification</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a></code><span class="doc-sig-links"><a href="#v-rpc-notification" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L214" title="View source at line 214">Source</a></span></div>
<div class="doc-body">

Construct a JSON-RPC 2.0 notification (carrying no correlation identifier).

</div>
</article>

<article class="doc-entry" id="v-rpc-parse-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-parse-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a></code><span class="doc-sig-links"><a href="#v-rpc-parse-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L133" title="View source at line 133">Source</a></span></div>
<div class="doc-body">

Construct a standard JSON-RPC 2.0 parse error (-32700).

</div>
</article>

<article class="doc-entry" id="v-rpc-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a></code><span class="doc-sig-links"><a href="#v-rpc-request" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L206" title="View source at line 206">Source</a></span></div>
<div class="doc-body">

Construct a JSON-RPC 2.0 request with an explicit correlation identifier.

</div>
</article>

<article class="doc-entry" id="v-rpc-request-codec">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-request-codec</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcCodecKind" title="Text.Rpc.RpcCodecKind">RpcCodecKind</a></code><span class="doc-sig-links"><a href="#v-rpc-request-codec" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L230" title="View source at line 230">Source</a></span></div>
<div class="doc-body">

Return the wire codec kind associated with an RpcRequest.

</div>
</article>

<article class="doc-entry" id="v-rpc-request-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-request-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-rpc-request-eq" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L246" title="View source at line 246">Source</a></span></div>
<div class="doc-body">

Compare two RpcRequest values for structural equality.

</div>
</article>

<article class="doc-entry" id="v-rpc-request-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-request-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a></code><span class="doc-sig-links"><a href="#v-rpc-request-id" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L226" title="View source at line 226">Source</a></span></div>
<div class="doc-body">

Return the optional correlation identifier of an RpcRequest (None indicates a notification).

</div>
</article>

<article class="doc-entry" id="v-rpc-request-is-notification">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-request-is-notification</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-rpc-request-is-notification" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L234" title="View source at line 234">Source</a></span></div>
<div class="doc-body">

Return True when the request is a notification expecting no response.

</div>
</article>

<article class="doc-entry" id="v-rpc-request-method">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-request-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-rpc-request-method" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L218" title="View source at line 218">Source</a></span></div>
<div class="doc-body">

Return the method name of an RpcRequest.

</div>
</article>

<article class="doc-entry" id="v-rpc-request-params">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-request-params</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-rpc-request-params" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L222" title="View source at line 222">Source</a></span></div>
<div class="doc-body">

Return the JSON parameters payload of an RpcRequest.

</div>
</article>

<article class="doc-entry" id="v-rpc-request-v1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-request-v1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcRequest" title="Text.Rpc.RpcRequest">RpcRequest</a></code><span class="doc-sig-links"><a href="#v-rpc-request-v1" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L210" title="View source at line 210">Source</a></span></div>
<div class="doc-body">

Construct a JSON-RPC 1.0 request with an explicit correlation identifier.

</div>
</article>

<article class="doc-entry" id="v-rpc-response-eq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-response-eq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-rpc-response-eq" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L274" title="View source at line 274">Source</a></span></div>
<div class="doc-body">

Compare two RpcResponse values for structural equality.

</div>
</article>

<article class="doc-entry" id="v-rpc-response-err">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-response-err</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a></code><span class="doc-sig-links"><a href="#v-rpc-response-err" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L262" title="View source at line 262">Source</a></span></div>
<div class="doc-body">

Construct an error RpcResponse carrying a structured RpcError.

</div>
</article>

<article class="doc-entry" id="v-rpc-response-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-response-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a></code><span class="doc-sig-links"><a href="#v-rpc-response-id" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L266" title="View source at line 266">Source</a></span></div>
<div class="doc-body">

Return the correlation identifier of an RpcResponse.

</div>
</article>

<article class="doc-entry" id="v-rpc-response-ok">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-response-ok</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcId" title="Text.Rpc.RpcId">RpcId</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a></code><span class="doc-sig-links"><a href="#v-rpc-response-ok" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L258" title="View source at line 258">Source</a></span></div>
<div class="doc-body">

Construct a successful RpcResponse carrying a JSON result value.

</div>
</article>

<article class="doc-entry" id="v-rpc-response-result">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rpc-response-result</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcResponse" title="Text.Rpc.RpcResponse">RpcResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Rpc/#t-RpcError" title="Text.Rpc.RpcError">RpcError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code><span class="doc-sig-links"><a href="#v-rpc-response-result" title="Permalink">#</a><a href="/packages/stdlib/Text.Rpc/source/#L270" title="View source at line 270">Source</a></span></div>
<div class="doc-body">

Return the Result outcome of an RpcResponse.

</div>
</article>
