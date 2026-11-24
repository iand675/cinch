# Effect.HttpTest

In-memory HTTP ResponseRecorder, request builder and loopback wire/socket testing helpers.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.HttpTest`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.HttpTest.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Http.Server](Effect.Http.Server.md), [Effect.Net](Effect.Net.md), [Text.Http](Text.Http.md).

Non-embedded source closure: [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.Pkix](Crypto.Pkix.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.Tls](Crypto.Tls.md), [Crypto.X25519](Crypto.X25519.md), [Crypto.X509](Crypto.X509.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.CookieJar](Data.CookieJar.md), [Data.IpAddr](Data.IpAddr.md), [Data.NetIp](Data.NetIp.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Http](Effect.Http.md), [Effect.Http.Server](Effect.Http.Server.md), [Effect.HttpTest](Effect.HttpTest.md), [Effect.Net](Effect.Net.md), [Effect.Random](Effect.Random.md), [Effect.Stream](Effect.Stream.md), [Effect.Tls](Effect.Tls.md), [Text.Asn1](Text.Asn1.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Binary](Text.Binary.md), [Text.Cookie](Text.Cookie.md), [Text.Digest](Text.Digest.md), [Text.Hex](Text.Hex.md), [Text.Http](Text.Http.md), [Text.Md5](Text.Md5.md), [Text.Pem](Text.Pem.md), [Text.Punycode](Text.Punycode.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md), [Text.TextProto](Text.TextProto.md), [Text.Uri](Text.Uri.md), [Text.Url](Text.Url.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

HTTP testing utilities: in-memory ResponseRecorder, synthetic request construction,
pure and effectful handler recording, in-memory wire roundtrips, and loopback Effect.Net helpers.

ResponseRecorder captures status code, canonical header snapshots (taken at the first header write
so later header mutations do not alter the finalized result), body chunks, and flush state without
opening sockets. Loopback helpers pair directly with Effect.Http.Server and Effect.Net for live
TCP integration tests.

</details>

# Types and constructors

<a id="type-526573706f6e73655265636f72646572"></a>

## `ResponseRecorder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ResponseRecorder</span></code></pre>

Recorder fields in the order listed above.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 36

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ResponseRecorder</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>))))))</code></pre>

# Values

<a id="value-68747470746573742d65786368616e67652d6c6f6f706261636b"></a>

## `httptest-exchange-loopback`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">httptest-exchange-loopback</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Send raw HTTP/1.1 wire bytes to host:port over Effect.Net and read all response bytes until EOF.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 217

<a id="value-68747470746573742d6e65772d7265636f72646572"></a>

## `httptest-new-recorder`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">httptest-new-recorder</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span></code></pre>

Create a fresh ResponseRecorder initialized with status 200 and empty headers and body.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 39

<a id="value-68747470746573742d6e65772d72657175657374"></a>

## `httptest-new-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">httptest-new-request</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>)))</code></pre>

Construct a synthetic incoming HTTP/1.1 request suitable for passing directly to handlers.
Defaults empty method to "GET", missing host to "example.com", and remote address to "192.0.2.1:1234".

[Source](/reference-sources/Effect.HttpTest.cinch) · line 151

<a id="value-68747470746573742d7265636f7264"></a>

## `httptest-record`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">httptest-record</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>))</code></pre>

Execute an effectful HttpHandler in memory against a request and capture its response in a ResponseRecorder.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 158

<a id="value-68747470746573742d7265636f72642d6d7578"></a>

## `httptest-record-mux`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">httptest-record-mux</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">ServeMux</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>))</code></pre>

Execute a ServeMux of HttpHandlers in memory against a request and capture the result in a ResponseRecorder.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 164

<a id="value-68747470746573742d7265636f72642d777269746572"></a>

## `httptest-record-writer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">httptest-record-writer</span> :: ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>)) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>))</code></pre>

Execute a pure ResponseRecorder-transforming handler against a request without IO effects.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 184

<a id="value-68747470746573742d73657276652d6c6f6f706261636b"></a>

## `httptest-serve-loopback`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">httptest-serve-loopback</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Connect to a loopback TCP test peer at host:port, serve HTTP/1.1 with default config, and close the socket.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 203

<a id="value-68747470746573742d73657276652d6c6f6f706261636b2d77697468"></a>

## `httptest-serve-loopback-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">httptest-serve-loopback-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>)))))</code></pre>

Connect to a loopback TCP test peer at host:port, serve HTTP/1.1 with explicit config, and close the socket.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 208

<a id="value-68747470746573742d73657276652d77697265"></a>

## `httptest-serve-wire`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">httptest-serve-wire</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>)))</code></pre>

Perform a full HTTP/1.1 wire serialization, server parsing/dispatch, and response parsing roundtrip in memory.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 188

<a id="value-7265636f726465722d6164642d686561646572"></a>

## `recorder-add-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-add-header</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>)))</code></pre>

Append a header value on the recorder's staging header map.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 53

<a id="value-7265636f726465722d626f6479"></a>

## `recorder-body`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-body</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

The concatenated body bytes written to the recorder.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 98

<a id="value-7265636f726465722d626f64792d737472696e67"></a>

## `recorder-body-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-body-string</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

The recorded body decoded as UTF-8 text (empty string on invalid UTF-8).

[Source](/reference-sources/Effect.HttpTest.cinch) · line 102

<a id="value-7265636f726465722d636f6465"></a>

## `recorder-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-code</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The recorded HTTP status code (200 by default).

[Source](/reference-sources/Effect.HttpTest.cinch) · line 94

<a id="value-7265636f726465722d64656c2d686561646572"></a>

## `recorder-del-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-del-header</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>))</code></pre>

Delete a header from the recorder's staging header map.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 59

<a id="value-7265636f726465722d666c757368"></a>

## `recorder-flush`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-flush</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>)</code></pre>

Mark the recorder as flushed, implicitly writing status 200 if headers have not been written yet.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 87

<a id="value-7265636f726465722d666c7573686564"></a>

## `recorder-flushed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-flushed</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Whether recorder-flush was called.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 108

<a id="value-7265636f726465722d686561646572"></a>

## `recorder-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-header</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpHeader</span>)</code></pre>

Read the recorder's current staging header map.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 43

<a id="value-7265636f726465722d726573756c74"></a>

## `recorder-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-result</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>)</code></pre>

Finalize the recorder into an immutable HttpWireResponse using the snapshotted headers.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 112

<a id="value-7265636f726465722d7365742d686561646572"></a>

## `recorder-set-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-set-header</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>)))</code></pre>

Set a header on the recorder's staging header map.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 47

<a id="value-7265636f726465722d7772697465"></a>

## `recorder-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-write</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>))</code></pre>

Append body bytes to the recorder, implicitly writing status 200 and detecting Content-Type on the first write.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 72

<a id="value-7265636f726465722d77726974652d686561646572"></a>

## `recorder-write-header`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-write-header</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>))</code></pre>

Record the HTTP status code and snapshot the current headers on the first call; subsequent calls are ignored.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 65

<a id="value-7265636f726465722d77726974652d737472696e67"></a>

## `recorder-write-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">recorder-write-string</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">HttpTest</span>.<span class="tok-constructor">ResponseRecorder</span>))</code></pre>

Append UTF-8 string bytes to the recorder.

[Source](/reference-sources/Effect.HttpTest.cinch) · line 83

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/HttpTest.cinch`. Source SHA-256: `89e36e2100b642f5a8f5f3efc98bb1b70be82c0953e9bd8e1b0533392fbf497d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
