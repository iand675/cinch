# Effect.Http

A bounded plaintext HTTP client with response framing and named errors. HTTPS is explicitly refused; this module has no TLS implementation.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Http</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Http/source/">View source</a><a href="/packages/search/?q=%2BEffect.Http">Search module</a><a href="/reference-sources/Effect.Http.cinch" download="Effect.Http.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Net](Effect.Net.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md), [Text.Uri](Text.Uri.md).

<details class="doc-notes"><summary>Module notes</summary>

Effect.Http: an HTTP/1.1 client over the Effect.Net socket capability.

The last item of the networking sweep in design/stdlib-parity-ruby.md
("Net::HTTP, Socket, URI, IPAddr, Resolv/DNS"): a port of Ruby's
Net::HTTP as a request/response value pair sent and read over the
Effect.Net capability's plain TCP sockets. There are no runtime
externs here and no compiler changes: the whole request is rendered to
bytes, written with `net-write`, and the whole reply is read back with
`net-read` to end of stream, then parsed as bytes.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary, and an HTTP client is a runtime capability over
sockets. The URL parser is Text.Uri's `uri-parse`, reused rather than
re-written (D-family: one parser per grammar). Identifiers are
kebab-case per D106.

THE LAW SHAPE (what this slice's tests pin, seed/tests/lib_http.rs):
  1. a 200 with `Content-Length` reads its body whole, and the code,
     reason phrase and headers survive the round trip;
  2. a 404 is a RESPONSE, not an error: the status is data (the caller
     decides what a 4xx means), never an `Err`;
  3. a `Transfer-Encoding: chunked` body reassembles across several
     reads (a short read is legal, so the client accumulates);
  4. a response with no framing at all reads to end of stream, bounded
     by an explicit maximum;
  5. a header name matches case-insensitively (ASCII folding, so
     `Content-Length` and `content-length` are the same name), and a
     repeated name keeps every value in arrival order.

BODY FRAMING. Three framings, chosen in RFC 7230 order: a
`Transfer-Encoding` naming `chunked` (chunk-size hex line, chunk, CRLF,
terminating zero chunk whose trailer is not parsed), else a decimal
`Content-Length` (exactly that many bytes), else no framing (all
remaining bytes). `Transfer-Encoding` wins over `Content-Length`, as
the RFC requires. A no-framing body is bounded by the request's
maximum: the client refuses past it (`HttpBodyTooLarge`) rather than
growing forever.

NAMED ERRORS (a value in every case, the house style; the socket layer
is WRAPPED, not flattened, so its own vocabulary survives as
`HttpNetworkError`'s payload):
  `HttpBadStatusLine`   the status line is not `HTTP/<v> <ddd>[ <reason>]`;
  `HttpBadHeaderLine`   a head line is malformed (no colon, an empty
                        name), or a framing line is malformed (a
                        non-decimal `Content-Length`, a non-hex
                        chunk-size line; the framing grammar of the
                        head section shares this error);
  `HttpUnsupportedTransferEncoding` a `Transfer-Encoding` that is not
                        `chunked`;
  `HttpBodyTooLarge`    the received bytes passed the maximum;
  `HttpTruncatedBody`   a `Content-Length` body or a chunk ran past the
                        bytes that arrived (the peer closed early);
  `HttpBadUrl`          `http-get`'s URL did not parse, or it named no
                        host (a request needs one);
  `HttpNoTls`           an `https` URL: the runtime has no TLS, and a
                        plaintext fallback would be a security bug, so
                        it is refused by name, never attempted;
  `HttpUnsupportedScheme` any other scheme (or a scheme-less reference);
  `HttpNetworkError`    the socket layer failed (connect, read, write).

NAMED REFUSALS (boundaries this client deliberately does not cross, each
an explicit decision rather than a silent gap):
  no TLS: `https` is refused (above); there is no cipher, no handshake,
    and no plaintext downgrade;
  no redirect following: a 3xx is returned as data; the caller decides
    whether and where to re-request (Ruby's Net::HTTP raises or follows
    depending on `follow_redirects`; here it is always the caller's
    move);
  no keep-alive or connection reuse: every request opens a fresh
    connection and sends `Connection: close`; there is no pool;
  no cookies, no proxies, no HTTP/2 (HTTP/1.1 text only), and no
    automatic content decoding: a body is the bytes as sent. A caller
    wanting `Content-Encoding: gzip` decoded uses Text.Zlib's `inflate`
    on the body itself;
  no request headers of the caller's choosing beyond the list given:
    `Host` and `Connection: close` are always sent, a `Content-Length`
    is computed from the body when one is present, and a caller-supplied
    `Content-Length` in the header list is NOT consulted (the computed
    one is appended);
  a chunked request body is not sent: `http-request` sends a body with a
    computed `Content-Length` only (request-side chunking is the caller's
    to avoid, and no API here produces it).

</details>

<details class="doc-synopsis"><summary>Synopsis (20 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-HttpError">HttpError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpError</strong></code></li><li><a class="syn-name" href="#t-HttpRequest">HttpRequest</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpRequest</strong></code></li><li><a class="syn-name" href="#t-HttpResponse">HttpResponse</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpResponse</strong></code></li><li><a class="syn-name" href="#v-http-body">http-body</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-http-get">http-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-get</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a></code></li><li><a class="syn-name" href="#v-http-header">http-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-header-all">http-header-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-header-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-headers">http-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-http-reason">http-reason</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-reason</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-request">http-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-request</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a></code></li><li><a class="syn-name" href="#v-http-request-body">http-request-body</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-request-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-http-request-headers">http-request-headers</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-request-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-http-request-host">http-request-host</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-request-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-request-max">http-request-max</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-request-max</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a></code></li><li><a class="syn-name" href="#v-http-request-method">http-request-method</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-request-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-request-path">http-request-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-request-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-http-request-port">http-request-port</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-request-port</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-http-status">http-status</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">http-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-max-response-bytes">max-response-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">max-response-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-render-http-error">render-http-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-http-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-HttpError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpError</strong></code><span class="doc-sig-links"><a href="#t-HttpError" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

What went wrong: a malformed or refused reply, a bad URL, or the
socket layer's own named failure (wrapped, not flattened).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-HttpBadStatusLine"><code><strong class="tok-constructor">HttpBadStatusLine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a></code></li><li id="k-HttpBadHeaderLine"><code><strong class="tok-constructor">HttpBadHeaderLine</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a></code></li><li id="k-HttpUnsupportedTransferEncoding"><code><strong class="tok-constructor">HttpUnsupportedTransferEncoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a></code></li><li id="k-HttpBodyTooLarge"><code><strong class="tok-constructor">HttpBodyTooLarge</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a></code></li><li id="k-HttpTruncatedBody"><code><strong class="tok-constructor">HttpTruncatedBody</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a></code></li><li id="k-HttpBadUrl"><code><strong class="tok-constructor">HttpBadUrl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a></code></li><li id="k-HttpNoTls"><code><strong class="tok-constructor">HttpNoTls</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a></code></li><li id="k-HttpUnsupportedScheme"><code><strong class="tok-constructor">HttpUnsupportedScheme</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a></code></li><li id="k-HttpNetworkError"><code><strong class="tok-constructor">HttpNetworkError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HttpRequest">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpRequest</strong></code><span class="doc-sig-links"><a href="#t-HttpRequest" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L149" title="View source at line 149">Source</a></span></div>
<div class="doc-body">

A request: method, host, port, request-target path, headers, and an
optional body. The path is sent as given, with a leading `/` supplied
when it is empty or does not start with one.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkHttpRequest"><code><strong class="tok-constructor">MkHttpRequest</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-HttpResponse">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">HttpResponse</strong></code><span class="doc-sig-links"><a href="#t-HttpResponse" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L178" title="View source at line 178">Source</a></span></div>
<div class="doc-body">

A response: status code, reason phrase, the header list (duplicates
kept, in arrival order) and the decoded body bytes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkHttpResponse"><code><strong class="tok-constructor">MkHttpResponse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-http-body">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-http-body" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L193" title="View source at line 193">Source</a></span></div>
</article>

<article class="doc-entry" id="v-http-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-get</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a></code><span class="doc-sig-links"><a href="#v-http-get" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L661" title="View source at line 661">Source</a></span></div>
<div class="doc-body">

GET `url` on a fresh connection. The URL is parsed with Text.Uri. An
`https` URL is refused (HttpNoTls); any non-`http` scheme (or none) is
refused (HttpUnsupportedScheme); a URL with no host is HttpBadUrl. A
missing port is 80.

</div>
</article>

<article class="doc-entry" id="v-http-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-header" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L200" title="View source at line 200">Source</a></span></div>
<div class="doc-body">

The FIRST value of `name`, matched case-insensitively (ASCII folding:
a header name is ASCII, so `Content-Length` and `content-length` are
the same name, and no locale folding is done), or `None`.

</div>
</article>

<article class="doc-entry" id="v-http-header-all">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-header-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-header-all" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

Every value of `name`, matched case-insensitively, in arrival order
(a repeated header keeps every value).

</div>
</article>

<article class="doc-entry" id="v-http-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-http-headers" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L189" title="View source at line 189">Source</a></span></div>
</article>

<article class="doc-entry" id="v-http-reason">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-reason</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-reason" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L185" title="View source at line 185">Source</a></span></div>
</article>

<article class="doc-entry" id="v-http-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-request</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a></code><span class="doc-sig-links"><a href="#v-http-request" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L589" title="View source at line 589">Source</a></span></div>
<div class="doc-body">

Send `q` on a fresh connection and read the whole reply. The default
bound (max-response-bytes) applies; use http-request-max for an
explicit one.

</div>
</article>

<article class="doc-entry" id="v-http-request-body">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-request-body</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-http-request-body" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L172" title="View source at line 172">Source</a></span></div>
</article>

<article class="doc-entry" id="v-http-request-headers">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-request-headers</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-http-request-headers" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L168" title="View source at line 168">Source</a></span></div>
</article>

<article class="doc-entry" id="v-http-request-host">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-request-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-request-host" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L156" title="View source at line 156">Source</a></span></div>
</article>

<article class="doc-entry" id="v-http-request-max">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-request-max</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a></code><span class="doc-sig-links"><a href="#v-http-request-max" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L594" title="View source at line 594">Source</a></span></div>
<div class="doc-body">

Send `q` with an explicit bound on the received bytes (headers plus
body). Refuses past it with HttpBodyTooLarge.

</div>
</article>

<article class="doc-entry" id="v-http-request-method">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-request-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-request-method" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L152" title="View source at line 152">Source</a></span></div>
</article>

<article class="doc-entry" id="v-http-request-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-request-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-http-request-path" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L164" title="View source at line 164">Source</a></span></div>
</article>

<article class="doc-entry" id="v-http-request-port">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-request-port</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpRequest" title="Effect.Http.HttpRequest">HttpRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-http-request-port" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L160" title="View source at line 160">Source</a></span></div>
</article>

<article class="doc-entry" id="v-http-status">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">http-status</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpResponse" title="Effect.Http.HttpResponse">HttpResponse</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-http-status" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L181" title="View source at line 181">Source</a></span></div>
</article>

<article class="doc-entry" id="v-max-response-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">max-response-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-max-response-bytes" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

The default bound on a whole received response, in bytes (the
no-framing case and the header section share it).

</div>
</article>

<article class="doc-entry" id="v-render-http-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-http-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Http/#t-HttpError" title="Effect.Http.HttpError">HttpError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-http-error" title="Permalink">#</a><a href="/packages/stdlib/Effect.Http/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

The diagnostic line for a named error.

</div>
</article>
