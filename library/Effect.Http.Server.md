# Effect.Http.Server

HTTP/1.1 and TLS 1.3 HTTPS listener/socket servers with keep-alive, size limits and ServeMux dispatch, plus a TLS/CookieJar/redirect-aware HTTP client.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Http.Server`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Http.Server.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Tls](Crypto.Tls.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.CookieJar](Data.CookieJar.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Clock](Effect.Clock.md), [Effect.Http](Effect.Http.md), [Effect.Net](Effect.Net.md), [Effect.Tls](Effect.Tls.md), [Text.Cookie](Text.Cookie.md), [Text.Http](Text.Http.md), [Text.Show](Text.Show.md), [Text.Url](Text.Url.md).

Non-embedded source closure: [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.Pkix](Crypto.Pkix.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.Tls](Crypto.Tls.md), [Crypto.X25519](Crypto.X25519.md), [Crypto.X509](Crypto.X509.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.CookieJar](Data.CookieJar.md), [Data.IpAddr](Data.IpAddr.md), [Data.NetIp](Data.NetIp.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Http](Effect.Http.md), [Effect.Http.Server](Effect.Http.Server.md), [Effect.Net](Effect.Net.md), [Effect.Random](Effect.Random.md), [Effect.Stream](Effect.Stream.md), [Effect.Tls](Effect.Tls.md), [Text.Asn1](Text.Asn1.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Binary](Text.Binary.md), [Text.Cookie](Text.Cookie.md), [Text.Digest](Text.Digest.md), [Text.Hex](Text.Hex.md), [Text.Http](Text.Http.md), [Text.Md5](Text.Md5.md), [Text.Pem](Text.Pem.md), [Text.Punycode](Text.Punycode.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md), [Text.TextProto](Text.TextProto.md), [Text.Uri](Text.Uri.md), [Text.Url](Text.Url.md), [Text.Url.Codec](Text.Url.Codec.md), [Text.Url.Error](Text.Url.Error.md), [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Model](Text.Url.Model.md), [Text.Url.Parser](Text.Url.Parser.md), [Text.Url.Query](Text.Url.Query.md), [Text.Url.Resolve](Text.Url.Resolve.md), [Text.Url.Scan](Text.Url.Scan.md), [Text.Url.Userinfo](Text.Url.Userinfo.md).

<details><summary>Behaviour and boundary notes from the source</summary>

HTTP/1.1 server over Effect.Net sockets, ServeMux handler dispatch, and an HTTP client with
Data.CookieJar and redirect support built over Effect.Http.

Connections are served with explicit header and body size limits, HTTP/1.1 keep-alive and pipelining
buffer management, graceful single-request and multi-request serving, and an acceptor loop that pairs
directly with socket listeners. The cookie-aware client threads an immutable CookieJar across requests
and redirect chains without ambient mutable state.

</details>

# Types and constructors

<a id="type-48747470436c69656e74"></a>

## `HttpClient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HttpClient</span></code></pre>

Client fields: optional jar, max redirects, max response bytes, optional clock seconds, optional TLS config.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 520

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpClient</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>)))))</code></pre>

<a id="type-48747470436c69656e744572726f72"></a>

## `HttpClientError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HttpClientError</span></code></pre>

Named errors reported by the cookie- and redirect-aware HTTP client.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 501

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ClientHttpError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClientError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ClientTooManyRedirects</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClientError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ClientBadRedirectLocation</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClientError</span>)</code></pre>

<a id="type-4874747048616e646c6572"></a>

## `HttpHandler`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HttpHandler</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Function from an incoming wire request to a wire response performing IO r.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 122

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpHandler</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">r</span> . ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>) -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>)</code></pre>

<a id="type-48747470536572766572436f6e666967"></a>

## `HttpServerConfig`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HttpServerConfig</span></code></pre>

Configuration fields in the order listed above.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 78

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">HttpServerConfig</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span>))))</code></pre>

<a id="type-487474705365727665724572726f72"></a>

## `HttpServerError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">HttpServerError</span></code></pre>

Named errors reported by the HTTP/1.1 server.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 49

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ServerNetError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ServerProtoError</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpProtoError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ServerMaxRequestsReached</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ServerTlsError</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span>)</code></pre>

# Values

<a id="value-687474702d636c69656e742d64656661756c74"></a>

## `http-client-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-default</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span></code></pre>

Default HTTP client (no cookie jar, up to 10 redirects, 1 MiB response limit, live wall clock, plaintext only).

[Source](/reference-sources/Effect.Http.Server.cinch) · line 523

<a id="value-687474702d636c69656e742d646f"></a>

## `http-client-do`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-do</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClientError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))))</code></pre>

Execute an HttpWireRequest with CookieJar persistence and redirect following, returning
the final response paired with the updated client. Uses TLS 1.3 for `https://` URLs when configured.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 569

<a id="value-687474702d636c69656e742d646f2d746c73"></a>

## `http-client-do-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-do-tls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClientError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))))</code></pre>

Execute an HttpWireRequest over HTTP or HTTPS (TLS 1.3), applying cookies and following redirects.
Uses the client's attached `TlsConfig` if set, or `tls-config-default` otherwise.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 584

<a id="value-687474702d636c69656e742d6572726f722d6d657373616765"></a>

## `http-client-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClientError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a readable diagnostic for an HTTP client error.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 510

<a id="value-687474702d636c69656e742d676574"></a>

## `http-client-get`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-get</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClientError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))))</code></pre>

Perform an HTTP/HTTPS GET request to a URL, applying cookies and following redirects.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 557

<a id="value-687474702d636c69656e742d6765742d746c73"></a>

## `http-client-get-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-get-tls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClientError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))))</code></pre>

Perform an HTTP or HTTPS GET request with cookies and redirect following.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 588

<a id="value-687474702d636c69656e742d6a6172"></a>

## `http-client-jar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-jar</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span>))</code></pre>

Retrieve the client's current CookieJar, if attached.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 552

<a id="value-687474702d636c69656e742d706f7374"></a>

## `http-client-post`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-post</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClientError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))))))</code></pre>

Perform an HTTP/HTTPS POST request with a Content-Type and body bytes, applying cookies and following redirects.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 562

<a id="value-687474702d636c69656e742d706f73742d746c73"></a>

## `http-client-post-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-post-tls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClientError</span> (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>, <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))))))</code></pre>

Perform an HTTP or HTTPS POST request with Content-Type, body bytes, cookies, and redirect following.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 593

<a id="value-687474702d636c69656e742d776974682d6a6172"></a>

## `http-client-with-jar`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-with-jar</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">CookieJar</span>.<span class="tok-constructor">CookieJar</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))</code></pre>

Attach a CookieJar to an HTTP client.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 527

<a id="value-687474702d636c69656e742d776974682d6d61782d6279746573"></a>

## `http-client-with-max-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-with-max-bytes</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))</code></pre>

Set the maximum received response size in bytes.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 537

<a id="value-687474702d636c69656e742d776974682d6d61782d726564697265637473"></a>

## `http-client-with-max-redirects`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-with-max-redirects</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))</code></pre>

Set the maximum number of redirects to follow (0 disables automatic redirects).

[Source](/reference-sources/Effect.Http.Server.cinch) · line 532

<a id="value-687474702d636c69656e742d776974682d74696d65"></a>

## `http-client-with-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-with-time</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))</code></pre>

Pin the client's Unix timestamp (seconds) for deterministic cookie expiration checks.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 542

<a id="value-687474702d636c69656e742d776974682d746c73"></a>

## `http-client-with-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-client-with-tls</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpClient</span>))</code></pre>

Attach a TLS 1.3 configuration to an HTTP client, enabling `https://` requests and redirects.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 547

<a id="value-687474702d68616e646c65722d66756e63"></a>

## `http-handler-func`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-handler-func</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>))</code></pre>

Wrap an effectful request handler function.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 127

<a id="value-687474702d68616e646c65722d70757265"></a>

## `http-handler-pure`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-handler-pure</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>))</code></pre>

Lift a pure request-to-response function into an HttpHandler.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 131

<a id="value-687474702d6c697374656e2d616e642d7365727665"></a>

## `http-listen-and-serve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-listen-and-serve</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Bind a TCP listener on `host:port` and serve incoming HTTP/1.1 connections indefinitely.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 368

<a id="value-687474702d6c697374656e2d616e642d73657276652d746c73"></a>

## `http-listen-and-serve-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-listen-and-serve-tls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>)))))</code></pre>

Bind a TCP listener on `host:port` and serve HTTPS (HTTP/1.1 over TLS 1.3) indefinitely.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 482

<a id="value-687474702d6c697374656e2d616e642d73657276652d746c732d77697468"></a>

## `http-listen-and-serve-tls-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-listen-and-serve-tls-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>)))))))</code></pre>

Bind a TCP listener on `host:port`, serve up to `max-conns` HTTPS connections (`0` = unlimited)
over TLS 1.3 with explicit server configuration, and close the listener before returning.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 488

<a id="value-687474702d6c697374656e2d616e642d73657276652d77697468"></a>

## `http-listen-and-serve-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-listen-and-serve-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>))))))</code></pre>

Bind a TCP listener on `host:port`, serve up to `max-conns` connections (`0` = unlimited)
with explicit configuration, and close the listener before returning.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 373

<a id="value-687474702d6d75782d68616e646c6572"></a>

## `http-mux-handler`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-mux-handler</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">ServeMux</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>)) -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>))</code></pre>

Adapt a ServeMux of HttpHandlers into a single HttpHandler, setting matched path values,
issuing canonical/trailing-slash redirects, and returning 405 with an Allow header when needed.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 150

<a id="value-687474702d6e6f742d666f756e642d68616e646c6572"></a>

## `http-not-found-handler`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-not-found-handler</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>)</code></pre>

Standard 404 Not Found handler.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 141

<a id="value-687474702d72656469726563742d68616e646c6572"></a>

## `http-redirect-handler`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-redirect-handler</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>)))</code></pre>

Standard HTTP redirect handler with the given status code and Location target.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 145

<a id="value-687474702d72756e2d68616e646c6572"></a>

## `http-run-handler`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-run-handler</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireRequest</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>))</code></pre>

Invoke an HttpHandler on a request.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 135

<a id="value-687474702d73657276652d616363657074"></a>

## `http-serve-accept`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-serve-accept</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span>))) -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Repeatedly accept sockets from a listener callback until Ok None signals graceful shutdown,
serving each connection with the given configuration and closing it afterwards.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 297

<a id="value-687474702d73657276652d6279746573"></a>

## `http-serve-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-serve-bytes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Nat</span>)))))</code></pre>

Serve one or more HTTP/1.1 requests directly from an in-memory wire byte buffer,
returning the concatenated wire response bytes and total requests served.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 314

<a id="value-687474702d73657276652d6c697374656e6572"></a>

## `http-serve-listener`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-serve-listener</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Accept and serve incoming TCP connections on `listener` indefinitely using the default configuration.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 345

<a id="value-687474702d73657276652d6c697374656e65722d6e"></a>

## `http-serve-listener-n`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-serve-listener-n</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>)))))</code></pre>

Accept and serve up to `max-conns` TCP connections on `listener` (`0` = unlimited),
closing each accepted client socket after serving it and returning total served requests.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 350

<a id="value-687474702d73657276652d736f636b6574"></a>

## `http-serve-socket`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-serve-socket</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Serve HTTP/1.1 requests over an open TCP socket using the default configuration.
Returns the number of requests served before clean EOF or connection close.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 210

<a id="value-687474702d73657276652d736f636b65742d6f6e6365"></a>

## `http-serve-socket-once`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-serve-socket-once</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">HttpWireResponse</span>)))</code></pre>

Serve exactly one HTTP/1.1 request on a socket with Connection: close, close the socket, and return the response.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 273

<a id="value-687474702d73657276652d736f636b65742d77697468"></a>

## `http-serve-socket-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-serve-socket-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Serve HTTP/1.1 requests over an open TCP socket with an explicit server configuration.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 214

<a id="value-687474702d73657276652d746c73"></a>

## `http-serve-tls`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-serve-tls</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Upgrade an accepted TCP socket with a TLS 1.3 server handshake and serve HTTP/1.1 keep-alive requests
using the default server configuration, sending `close_notify` when finished.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 387

<a id="value-687474702d73657276652d746c732d6c697374656e65722d6e"></a>

## `http-serve-tls-listener-n`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-serve-tls-listener-n</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>))))))</code></pre>

Accept up to `max-conns` TCP connections on `listener` (`0` = unlimited), upgrade each via TLS 1.3,
and serve HTTP/1.1 requests on every established connection.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 459

<a id="value-687474702d73657276652d746c732d77697468"></a>

## `http-serve-tls-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-serve-tls-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpHandler</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> <span class="tok-constructor">Nat</span>)))))</code></pre>

Upgrade an accepted TCP socket with a TLS 1.3 server handshake and serve HTTP/1.1 keep-alive requests
with an explicit server configuration, sending `close_notify` and closing the socket on completion.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 392

<a id="value-687474702d7365727665722d636f6e6669672d64656661756c74"></a>

## `http-server-config-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-server-config-default</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span></code></pre>

Default HTTP/1.1 server configuration (1 MiB header limit, 10 MiB body limit, keep-alive enabled, unlimited requests).

[Source](/reference-sources/Effect.Http.Server.cinch) · line 81

<a id="value-687474702d7365727665722d636f6e6669672d776974682d6b6565702d616c697665"></a>

## `http-server-config-with-keep-alive`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-server-config-with-keep-alive</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span>))</code></pre>

Enable or disable HTTP/1.1 persistent keep-alive connections.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 90

<a id="value-687474702d7365727665722d636f6e6669672d776974682d6c696d697473"></a>

## `http-server-config-with-limits`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-server-config-with-limits</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span>)))</code></pre>

Override the maximum header bytes and maximum body bytes.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 85

<a id="value-687474702d7365727665722d636f6e6669672d776974682d6d61782d7265717565737473"></a>

## `http-server-config-with-max-requests`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-server-config-with-max-requests</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerConfig</span>))</code></pre>

Limit the number of requests served per connection (0 means unlimited).

[Source](/reference-sources/Effect.Http.Server.cinch) · line 95

<a id="value-687474702d7365727665722d6572726f722d6d657373616765"></a>

## `http-server-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">http-server-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Http</span>.<span class="tok-constructor">Server</span>.<span class="tok-constructor">HttpServerError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a readable diagnostic for an HTTP server error.

[Source](/reference-sources/Effect.Http.Server.cinch) · line 60

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Http/Server.cinch`. Source SHA-256: `09cf9a306e698b5acb9afd62e576708efe629205fe4807112c1a3ae74d00a080`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
