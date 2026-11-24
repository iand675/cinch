# Effect.Http.Util

HTTP/1.1 wire request/response dumping, strict chunked codecs, hop-by-hop header stripping, and HTTP/HTTPS ReverseProxy forwarding.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Http.Util`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Http.Util.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Tls](Crypto.Tls.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Http](Effect.Http.md), [Effect.Http.Server](Effect.Http.Server.md), [Effect.Tls](Effect.Tls.md), [Text.Http](Text.Http.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.Pkix](Crypto.Pkix.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.Tls](Crypto.Tls.md), [Crypto.X25519](Crypto.X25519.md), [Crypto.X509](Crypto.X509.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.CookieJar](Data.CookieJar.md), [Data.IpAddr](Data.IpAddr.md), [Data.NetIp](Data.NetIp.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Http](Effect.Http.md), [Effect.Http.Server](Effect.Http.Server.md), [Effect.Http.Util](Effect.Http.Util.md), [Effect.Net](Effect.Net.md), [Effect.Random](Effect.Random.md), [Effect.Stream](Effect.Stream.md), [Effect.Tls](Effect.Tls.md), [Text.Asn1](Text.Asn1.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Binary](Text.Binary.md), [Text.Cookie](Text.Cookie.md), [Text.Digest](Text.Digest.md), [Text.Hex](Text.Hex.md), [Text.Http](Text.Http.md), [Text.Md5](Text.Md5.md), [Text.Pem](Text.Pem.md), [Text.Punycode](Text.Punycode.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md), [Text.TextProto](Text.TextProto.md), [Text.Uri](Text.Uri.md), [Text.Url](Text.Url.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

HTTP/1.1 wire diagnostics, strict chunked transfer-encoding helpers, RFC 7230 hop-by-hop
header filtering, and a configurable HTTP/HTTPS reverse proxy built over Effect.Http,
Effect.Http.Server, and Effect.Tls.

Pure wire helpers (`dump-request`, `dump-response`, `chunked-encode`, `chunked-decode`, and
`strip-hop-by-hop-headers`) operate directly on `Text.Http` values and byte buffers without
I/O. `ReverseProxy` forwards incoming `HttpWireRequest` values to HTTP or TLS 1.3 HTTPS
upstreams, stripping hop-by-hop headers in both directions, joining base paths, populating
`X-Forwarded-*` headers, applying optional request rewrites and response modifiers, and
plugging directly into `Effect.Http.Server` handlers and `ServeMux`.

</details>

# Types and constructors

<a id="type-50726f7879546172676574"></a>

## `ProxyTarget`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ProxyTarget</span></code></pre>

Construct a target from scheme, host, port, and base path.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 376

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ProxyTarget</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ProxyTarget</span>))))</code></pre>

<a id="type-5265766572736550726f7879"></a>

## `ReverseProxy`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ReverseProxy</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

Proxy configuration: upstream target, optional request rewriter, optional response modifier,
optional error handler, optional TLS 1.3 configuration, and optional client IP override.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 398

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ReverseProxy</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ProxyTarget</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>))))))</code></pre>

# Values

<a id="value-6368756e6b65642d6465636f6465"></a>

## `chunked-decode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chunked-decode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Decode an HTTP/1.1 chunked transfer-encoded payload with strict hex and CRLF validation.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 263

<a id="value-6368756e6b65642d656e636f6465"></a>

## `chunked-encode`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">chunked-encode</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Encode `payload` into HTTP/1.1 chunked transfer-encoding using chunks of at most `chunk-size` bytes
(`0` defaults to `4096`).

[Source](/reference-sources/Effect.Http.Util.cinch) · line 226

<a id="value-64756d702d72657175657374"></a>

## `dump-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dump-request</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Render an `HttpWireRequest` in RFC 7230 HTTP/1.1 wire format, including body bytes when `include-body` is `True`.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 179

<a id="value-64756d702d726573706f6e7365"></a>

## `dump-response`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">dump-response</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Render an `HttpWireResponse` in RFC 7230 HTTP/1.1 wire format, including body bytes when `include-body` is `True`.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 201

<a id="value-70726f78792d7461726765742d626173652d70617468"></a>

## `proxy-target-base-path`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">proxy-target-base-path</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ProxyTarget</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the upstream target base path prefix.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 391

<a id="value-70726f78792d7461726765742d686f7374"></a>

## `proxy-target-host`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">proxy-target-host</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ProxyTarget</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the upstream target hostname or IP literal.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 383

<a id="value-70726f78792d7461726765742d706f7274"></a>

## `proxy-target-port`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">proxy-target-port</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ProxyTarget</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the upstream target TCP port.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 387

<a id="value-70726f78792d7461726765742d736368656d65"></a>

## `proxy-target-scheme`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">proxy-target-scheme</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ProxyTarget</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the upstream target scheme (`"http"` or `"https"`).

[Source](/reference-sources/Effect.Http.Util.cinch) · line 379

<a id="value-726576657273652d70726f78792d666f7277617264"></a>

## `reverse-proxy-forward`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-proxy-forward</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Forward an incoming `HttpWireRequest` to the configured upstream (HTTP or TLS 1.3 HTTPS),
stripping hop-by-hop headers in both directions, applying rewrite and response-modifier hooks,
and returning a 502 Bad Gateway (or custom error-handler) response on upstream failure.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 574

<a id="value-726576657273652d70726f78792d666f72776172642d746c73"></a>

## `reverse-proxy-forward-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-proxy-forward-tls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Forward an incoming `HttpWireRequest` to an HTTP or TLS 1.3 HTTPS upstream target.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 578

<a id="value-726576657273652d70726f78792d68616e646c6572"></a>

## `reverse-proxy-handler`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-proxy-handler</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>))</code></pre>

Adapt a `ReverseProxy` into an `HttpHandler` that plugs directly into `Effect.Http.Server` and `ServeMux`.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 602

<a id="value-726576657273652d70726f78792d73696e676c652d686f7374"></a>

## `reverse-proxy-single-host`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-proxy-single-host</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>)))))</code></pre>

Create a single-host `ReverseProxy` forwarding to `scheme://host:port<base-path>`.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 401

<a id="value-726576657273652d70726f78792d746172676574"></a>

## `reverse-proxy-target`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-proxy-target</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ProxyTarget</span>)</code></pre>

Return the configured `ProxyTarget` of a reverse proxy.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 406

<a id="value-726576657273652d70726f78792d776974682d636c69656e742d6970"></a>

## `reverse-proxy-with-client-ip`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-proxy-with-client-ip</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>)))</code></pre>

Configure an explicit client IP address appended to `X-Forwarded-For` on forwarded requests.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 430

<a id="value-726576657273652d70726f78792d776974682d6572726f722d68616e646c6572"></a>

## `reverse-proxy-with-error-handler`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-proxy-with-error-handler</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>)))</code></pre>

Attach a custom error handler mapping an upstream failure diagnostic to an `HttpWireResponse`.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 420

<a id="value-726576657273652d70726f78792d776974682d6d6f646966792d726573706f6e7365"></a>

## `reverse-proxy-with-modify-response`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-proxy-with-modify-response</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>)))</code></pre>

Attach a response modifier invoked on each upstream response after hop-by-hop stripping.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 415

<a id="value-726576657273652d70726f78792d776974682d72657772697465"></a>

## `reverse-proxy-with-rewrite`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-proxy-with-rewrite</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>) -&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>)))</code></pre>

Attach a request rewrite function invoked on each outbound proxy request before forwarding.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 410

<a id="value-726576657273652d70726f78792d776974682d746c73"></a>

## `reverse-proxy-with-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse-proxy-with-tls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Util</span>.<span class="tok-constructor">ReverseProxy</span> <span class="tok-variable">r</span>)))</code></pre>

Attach a TLS 1.3 `TlsConfig` used when forwarding to `https` upstream targets.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 425

<a id="value-73747269702d686f702d62792d686f702d68656164657273"></a>

## `strip-hop-by-hop-headers`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">strip-hop-by-hop-headers</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>, <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)))</code></pre>

Remove RFC 7230 §6.1 hop-by-hop headers (`Connection`, `Keep-Alive`, `Proxy-Authenticate`,
`Proxy-Authorization`, `Proxy-Connection`, `Te`, `Trailers`, `Transfer-Encoding`, `Upgrade`)
plus any additional header field names listed inside `Connection` header values.

[Source](/reference-sources/Effect.Http.Util.cinch) · line 353

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Http/Util.cinch`. Source SHA-256: `30551bd754a3e43c4029cbb58cd8fb8f301a3f03ac908446873a69fe67b91db3`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
