# Effect.Rpc

Effectful JSON-RPC and gob-RPC TCP listener servers, clients, and HTTP POST JSON-RPC handler over Effect.Net and Effect.Http.Server.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Rpc`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Rpc.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Http.Server](Effect.Http.Server.md), [Effect.Net](Effect.Net.md), [Text.Gob](Text.Gob.md), [Text.Http](Text.Http.md), [Text.Rpc](Text.Rpc.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.Pkix](Crypto.Pkix.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.Tls](Crypto.Tls.md), [Crypto.X25519](Crypto.X25519.md), [Crypto.X509](Crypto.X509.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.CookieJar](Data.CookieJar.md), [Data.IpAddr](Data.IpAddr.md), [Data.NetIp](Data.NetIp.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Http](Effect.Http.md), [Effect.Http.Server](Effect.Http.Server.md), [Effect.Net](Effect.Net.md), [Effect.Random](Effect.Random.md), [Effect.Rpc](Effect.Rpc.md), [Effect.Stream](Effect.Stream.md), [Effect.Tls](Effect.Tls.md), [Text.Asn1](Text.Asn1.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Binary](Text.Binary.md), [Text.Builder](Text.Builder.md), [Text.Cookie](Text.Cookie.md), [Text.Digest](Text.Digest.md), [Text.Gob](Text.Gob.md), [Text.Hex](Text.Hex.md), [Text.Http](Text.Http.md), [Text.Json](Text.Json.md), [Text.Lex](Text.Lex.md), [Text.Md5](Text.Md5.md), [Text.Parse](Text.Parse.md), [Text.Pem](Text.Pem.md), [Text.Punycode](Text.Punycode.md), [Text.Rpc](Text.Rpc.md), [Text.Scan](Text.Scan.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md), [Text.TextProto](Text.TextProto.md), [Text.Uri](Text.Uri.md), [Text.Url](Text.Url.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effectful RPC client and server over Effect.Net TCP sockets and Effect.Http.Server.
Provides an immutable RPC method registry supporting pure and effectful JSON-RPC 1.0/2.0
and binary gob-RPC method handlers, single and batch JSON-RPC dispatch, streaming TCP
servers for both newline-delimited JSON-RPC and length-prefixed gob-RPC, a listener
acceptor loop, an HTTP POST JSON-RPC 2.0 handler returning 204 for notifications, and
stateful clients threading sequence numbers and read buffers without ambient mutable state.

</details>

# Types and constructors

<a id="type-52706343616c6c4572726f72"></a>

## `RpcCallError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RpcCallError</span></code></pre>

Named errors returned by RPC clients and servers across transport, framing, and remote execution.

[Source](/reference-sources/Effect.Rpc.cinch) · line 45

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcRemoteError</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcProtocolError</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcNetError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcClosed</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span></code></pre>

<a id="type-527063436c69656e74"></a>

## `RpcClient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RpcClient</span></code></pre>

Underlying socket, codec kind, buffered unread bytes, next sequence id, max frame bytes, and closed state.

[Source](/reference-sources/Effect.Rpc.cinch) · line 455

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcClient</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span>))))))</code></pre>

<a id="type-52706348616e646c6572"></a>

## `RpcHandler`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RpcHandler</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

A registered RPC method implementation operating on Json or GobValue under IO region r.

[Source](/reference-sources/Effect.Rpc.cinch) · line 77

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcJsonHandler</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcHandler</span> <span class="tok-variable">r</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcJsonPureHandler</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcHandler</span> <span class="tok-variable">r</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcGobHandler</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcHandler</span> <span class="tok-variable">r</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcGobPureHandler</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcHandler</span> <span class="tok-variable">r</span>)</code></pre>

<a id="type-527063536572766572"></a>

## `RpcServer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RpcServer</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

Registered JSON method table, gob method table, and maximum wire frame size in bytes.

[Source](/reference-sources/Effect.Rpc.cinch) · line 90

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RpcServer</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcHandler</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcHandler</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>)))</code></pre>

# Values

<a id="value-7270632d63616c6c2d6572726f722d6d657373616765"></a>

## `rpc-call-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-call-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a human-readable diagnostic message for an RpcCallError.

[Source](/reference-sources/Effect.Rpc.cinch) · line 64

<a id="value-7270632d63616c6c2d676f62"></a>

## `rpc-call-gob`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-call-gob</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span>))))</code></pre>

Invoke a binary gob-RPC method on an RpcClient and await its length-prefixed gob response.

[Source](/reference-sources/Effect.Rpc.cinch) · line 582

<a id="value-7270632d63616c6c2d6a736f6e"></a>

## `rpc-call-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-call-json</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span>))))</code></pre>

Invoke a JSON-RPC 1.0 or 2.0 method on an RpcClient and await its correlated response.
Returns the call outcome paired with the updated RpcClient carrying incremented sequence and buffer state.

[Source](/reference-sources/Effect.Rpc.cinch) · line 503

<a id="value-7270632d63616c6c2d6a736f6e2d6261746368"></a>

## `rpc-call-json-batch`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-call-json-batch</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span>)), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span>)))</code></pre>

Send a JSON-RPC 2.0 batch array over an RpcClient and decode the batch response array.
If every request in the batch is a notification, sends the batch and returns Ok Nil without awaiting a reply.

[Source](/reference-sources/Effect.Rpc.cinch) · line 560

<a id="value-7270632d636c69656e742d636f646563"></a>

## `rpc-client-codec`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-client-codec</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span>)</code></pre>

Return the wire codec kind configured on an RpcClient.

[Source](/reference-sources/Effect.Rpc.cinch) · line 467

<a id="value-7270632d636c69656e742d66726f6d2d736f636b6574"></a>

## `rpc-client-from-socket`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-client-from-socket</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCodecKind</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span>))</code></pre>

Wrap an already-connected TCP Socket as an RpcClient using the specified wire codec.

[Source](/reference-sources/Effect.Rpc.cinch) · line 458

<a id="value-7270632d636c69656e742d736571"></a>

## `rpc-client-seq`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-client-seq</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the next request sequence number that will be assigned by an RpcClient.

[Source](/reference-sources/Effect.Rpc.cinch) · line 471

<a id="value-7270632d636c69656e742d776974682d6d61782d6672616d65"></a>

## `rpc-client-with-max-frame`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-client-with-max-frame</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span>))</code></pre>

Configure the maximum frame or line size in bytes accepted by an RpcClient.

[Source](/reference-sources/Effect.Rpc.cinch) · line 462

<a id="value-7270632d636c6f7365"></a>

## `rpc-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close the underlying TCP socket of an RpcClient.

[Source](/reference-sources/Effect.Rpc.cinch) · line 610

<a id="value-7270632d6469616c"></a>

## `rpc-dial`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-dial</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span>)))</code></pre>

Connect to a binary gob-RPC server at (host, port) over TCP.

[Source](/reference-sources/Effect.Rpc.cinch) · line 481

<a id="value-7270632d6469616c2d6a736f6e727063"></a>

## `rpc-dial-jsonrpc`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-dial-jsonrpc</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span>)))</code></pre>

Connect to a newline-delimited JSON-RPC 2.0 server at (host, port) over TCP.

[Source](/reference-sources/Effect.Rpc.cinch) · line 485

<a id="value-7270632d6469616c2d6a736f6e72706331"></a>

## `rpc-dial-jsonrpc1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-dial-jsonrpc1</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span>)))</code></pre>

Connect to a newline-delimited JSON-RPC 1.0 server at (host, port) over TCP.

[Source](/reference-sources/Effect.Rpc.cinch) · line 489

<a id="value-7270632d64697370617463682d676f62"></a>

## `rpc-dispatch-gob`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-dispatch-gob</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">GobRpcResponse</span>))</code></pre>

Dispatch a single decoded GobRpcRequest against an RpcServer's gob method table.

[Source](/reference-sources/Effect.Rpc.cinch) · line 245

<a id="value-7270632d64697370617463682d6a736f6e"></a>

## `rpc-dispatch-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-dispatch-json</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcResponse</span>)))</code></pre>

Dispatch a single decoded RpcRequest against an RpcServer.
Returns None for notifications (requests with no correlation id), after executing the method.

[Source](/reference-sources/Effect.Rpc.cinch) · line 199

<a id="value-7270632d64697370617463682d6a736f6e2d6279746573"></a>

## `rpc-dispatch-json-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-dispatch-json-bytes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Dispatch raw UTF-8 JSON-RPC bytes (single JSON-RPC 1.0/2.0 request OR JSON-RPC 2.0 batch array).
Returns None when the request or entire batch consists solely of notifications.

[Source](/reference-sources/Effect.Rpc.cinch) · line 227

<a id="value-7270632d6861732d6d6574686f64"></a>

## `rpc-has-method`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-has-method</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return True if the given method name is registered for either JSON-RPC or gob-RPC.

[Source](/reference-sources/Effect.Rpc.cinch) · line 147

<a id="value-7270632d687474702d68616e646c6572"></a>

## `rpc-http-handler`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-http-handler</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>))</code></pre>

Adapt an RpcServer into an Effect.Http.Server HttpHandler serving HTTP POST JSON-RPC 2.0 requests.
Responds with Content-Type application/json (200 OK) for calls and 204 No Content for notifications.

[Source](/reference-sources/Effect.Rpc.cinch) · line 444

<a id="value-7270632d6d6574686f6473"></a>

## `rpc-methods`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-methods</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return all unique method names registered on an RpcServer.

[Source](/reference-sources/Effect.Rpc.cinch) · line 172

<a id="value-7270632d6e6f746966792d6a736f6e"></a>

## `rpc-notify-json`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-notify-json</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Unit</span>), <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcClient</span>))))</code></pre>

Send a JSON-RPC notification (no response expected) over an RpcClient.

[Source](/reference-sources/Effect.Rpc.cinch) · line 537

<a id="value-7270632d7265676973746572"></a>

## `rpc-register`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-register</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>))))</code></pre>

Register an effectful JSON-RPC method handler on an RpcServer.

[Source](/reference-sources/Effect.Rpc.cinch) · line 123

<a id="value-7270632d72656769737465722d676f62"></a>

## `rpc-register-gob`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-register-gob</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>))))</code></pre>

Register an effectful gob-RPC method handler on an RpcServer.

[Source](/reference-sources/Effect.Rpc.cinch) · line 135

<a id="value-7270632d72656769737465722d676f622d70757265"></a>

## `rpc-register-gob-pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-register-gob-pure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Gob</span>.<span class="tok-constructor">GobValue</span>)) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>))))</code></pre>

Register a pure gob-RPC method handler on an RpcServer.

[Source](/reference-sources/Effect.Rpc.cinch) · line 141

<a id="value-7270632d72656769737465722d70757265"></a>

## `rpc-register-pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-register-pure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">Json</span>.<span class="tok-constructor">Json</span>)) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>))))</code></pre>

Register a pure JSON-RPC method handler on an RpcServer.

[Source](/reference-sources/Effect.Rpc.cinch) · line 129

<a id="value-7270632d73657276652d676f622d6c697374656e6572"></a>

## `rpc-serve-gob-listener`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-serve-gob-listener</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Accept up to max-conns TCP connections on a Listener and serve gob-RPC on each.
Closes each accepted socket on completion and returns total served RPC calls.

[Source](/reference-sources/Effect.Rpc.cinch) · line 430

<a id="value-7270632d73657276652d676f622d736f636b6574"></a>

## `rpc-serve-gob-socket`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-serve-gob-socket</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Serve length-prefixed binary gob-RPC requests over a TCP socket until clean EOF.
Returns the total number of served RPC calls.

[Source](/reference-sources/Effect.Rpc.cinch) · line 411

<a id="value-7270632d73657276652d6a736f6e7270632d6c697374656e6572"></a>

## `rpc-serve-jsonrpc-listener`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-serve-jsonrpc-listener</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Accept up to max-conns TCP connections on a Listener and serve JSON-RPC 1.0/2.0 on each.
Closes each accepted socket on completion and returns total served request lines.

[Source](/reference-sources/Effect.Rpc.cinch) · line 388

<a id="value-7270632d73657276652d6a736f6e7270632d736f636b6574"></a>

## `rpc-serve-jsonrpc-socket`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-serve-jsonrpc-socket</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcCallError</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Serve newline-delimited JSON-RPC 1.0 and 2.0 requests (including batches and notifications) over a TCP socket until EOF.
Returns the total number of dispatched request lines.

[Source](/reference-sources/Effect.Rpc.cinch) · line 369

<a id="value-7270632d7365727665722d6e6577"></a>

## `rpc-server-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-server-new</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>)</code></pre>

Create an empty RpcServer with the default maximum frame size (16 MiB).

[Source](/reference-sources/Effect.Rpc.cinch) · line 93

<a id="value-7270632d7365727665722d776974682d6d61782d6672616d65"></a>

## `rpc-server-with-max-frame`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rpc-server-with-max-frame</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Rpc</span>.<span class="tok-constructor">RpcServer</span> <span class="tok-variable">r</span>)))</code></pre>

Override the maximum frame or line size in bytes accepted by an RpcServer.

[Source](/reference-sources/Effect.Rpc.cinch) · line 97

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Rpc.cinch`. Source SHA-256: `c4ad39642520223349981af9f73d343cfd5bb26aca9dbc7401e0df4ea0d12dc0`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
