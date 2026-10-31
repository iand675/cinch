# Effect.Http

A bounded plaintext HTTP client with response framing and named errors. HTTPS is explicitly refused; this module has no TLS implementation.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Http`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Http.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Net](Effect.Net.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md), [Text.Uri](Text.Uri.md).

Non-embedded source closure: [Data.IpAddr](Data.IpAddr.md), [Effect.Http](Effect.Http.md), [Effect.Net](Effect.Net.md), [Text.Uri](Text.Uri.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

THE LAW SHAPE (what this slice's tests pin, compiler/tests/lib_http.rs):
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

# Types and constructors

<a id="type-487474704572726f72"></a>

## `HttpError`

```cinch
data HttpError
```

Errors.

What went wrong: a malformed or refused reply, a bad URL, or the
socket layer's own named failure (wrapped, not flattened).

[Source](/reference-sources/Effect.Http.cinch) · line 112

```cinch
HttpBadStatusLine :: HttpError
```

```cinch
HttpBadHeaderLine :: HttpError
```

```cinch
HttpUnsupportedTransferEncoding :: HttpError
```

```cinch
HttpBodyTooLarge :: HttpError
```

```cinch
HttpTruncatedBody :: HttpError
```

```cinch
HttpBadUrl :: HttpError
```

```cinch
HttpNoTls :: HttpError
```

```cinch
HttpUnsupportedScheme :: HttpError
```

```cinch
HttpNetworkError :: (NetError -> HttpError)
```

<a id="type-4874747052657175657374"></a>

## `HttpRequest`

```cinch
data HttpRequest
```

The request and response values.

A request: method, host, port, request-target path, headers, and an
optional body. The path is sent as given, with a leading `/` supplied
when it is empty or does not start with one.

[Source](/reference-sources/Effect.Http.cinch) · line 149

```cinch
MkHttpRequest :: (String -> (String -> (Nat -> (String -> (List (String, String) -> (Option Bytes -> HttpRequest))))))
```

<a id="type-48747470526573706f6e7365"></a>

## `HttpResponse`

```cinch
data HttpResponse
```

A response: status code, reason phrase, the header list (duplicates
kept, in arrival order) and the decoded body bytes.

[Source](/reference-sources/Effect.Http.cinch) · line 178

```cinch
MkHttpResponse :: (Nat -> (String -> (List (String, String) -> (Bytes -> HttpResponse))))
```

# Values

<a id="value-687474702d626f6479"></a>

## `http-body`

```cinch
http-body :: (HttpResponse -> Bytes)
```

[Source](/reference-sources/Effect.Http.cinch) · line 193

<a id="value-687474702d676574"></a>

## `http-get`

```cinch
http-get :: forall r . using (io :: Io r) => (String ->{IO r} Result HttpError HttpResponse)
```

GET `url` on a fresh connection. The URL is parsed with Text.Uri. An
`https` URL is refused (HttpNoTls); any non-`http` scheme (or none) is
refused (HttpUnsupportedScheme); a URL with no host is HttpBadUrl. A
missing port is 80.

[Source](/reference-sources/Effect.Http.cinch) · line 661

<a id="value-687474702d686561646572"></a>

## `http-header`

```cinch
http-header :: (HttpResponse -> (String -> Option String))
```

The FIRST value of `name`, matched case-insensitively (ASCII folding:
a header name is ASCII, so `Content-Length` and `content-length` are
the same name, and no locale folding is done), or `None`.

[Source](/reference-sources/Effect.Http.cinch) · line 200

<a id="value-687474702d6865616465722d616c6c"></a>

## `http-header-all`

```cinch
http-header-all :: (HttpResponse -> (String -> List String))
```

Every value of `name`, matched case-insensitively, in arrival order
(a repeated header keeps every value).

[Source](/reference-sources/Effect.Http.cinch) · line 205

<a id="value-687474702d68656164657273"></a>

## `http-headers`

```cinch
http-headers :: (HttpResponse -> List (String, String))
```

[Source](/reference-sources/Effect.Http.cinch) · line 189

<a id="value-687474702d726561736f6e"></a>

## `http-reason`

```cinch
http-reason :: (HttpResponse -> String)
```

[Source](/reference-sources/Effect.Http.cinch) · line 185

<a id="value-687474702d72657175657374"></a>

## `http-request`

```cinch
http-request :: forall r . using (io :: Io r) => (HttpRequest ->{IO r} Result HttpError HttpResponse)
```

Sending and reading.

Send `q` on a fresh connection and read the whole reply. The default
bound (max-response-bytes) applies; use http-request-max for an
explicit one.

[Source](/reference-sources/Effect.Http.cinch) · line 589

<a id="value-687474702d726571756573742d626f6479"></a>

## `http-request-body`

```cinch
http-request-body :: (HttpRequest -> Option Bytes)
```

[Source](/reference-sources/Effect.Http.cinch) · line 172

<a id="value-687474702d726571756573742d68656164657273"></a>

## `http-request-headers`

```cinch
http-request-headers :: (HttpRequest -> List (String, String))
```

[Source](/reference-sources/Effect.Http.cinch) · line 168

<a id="value-687474702d726571756573742d686f7374"></a>

## `http-request-host`

```cinch
http-request-host :: (HttpRequest -> String)
```

[Source](/reference-sources/Effect.Http.cinch) · line 156

<a id="value-687474702d726571756573742d6d6178"></a>

## `http-request-max`

```cinch
http-request-max :: forall r . using (io :: Io r) => (Nat -> (HttpRequest ->{IO r} Result HttpError HttpResponse))
```

Send `q` with an explicit bound on the received bytes (headers plus
body). Refuses past it with HttpBodyTooLarge.

[Source](/reference-sources/Effect.Http.cinch) · line 594

<a id="value-687474702d726571756573742d6d6574686f64"></a>

## `http-request-method`

```cinch
http-request-method :: (HttpRequest -> String)
```

[Source](/reference-sources/Effect.Http.cinch) · line 152

<a id="value-687474702d726571756573742d70617468"></a>

## `http-request-path`

```cinch
http-request-path :: (HttpRequest -> String)
```

[Source](/reference-sources/Effect.Http.cinch) · line 164

<a id="value-687474702d726571756573742d706f7274"></a>

## `http-request-port`

```cinch
http-request-port :: (HttpRequest -> Nat)
```

[Source](/reference-sources/Effect.Http.cinch) · line 160

<a id="value-687474702d737461747573"></a>

## `http-status`

```cinch
http-status :: (HttpResponse -> Nat)
```

[Source](/reference-sources/Effect.Http.cinch) · line 181

<a id="value-6d61782d726573706f6e73652d6279746573"></a>

## `max-response-bytes`

```cinch
max-response-bytes :: Nat
```

The default bound on a whole received response, in bytes (the
no-framing case and the header section share it).

[Source](/reference-sources/Effect.Http.cinch) · line 138

<a id="value-72656e6465722d687474702d6572726f72"></a>

## `render-http-error`

```cinch
render-http-error :: (HttpError -> String)
```

The diagnostic line for a named error.

[Source](/reference-sources/Effect.Http.cinch) · line 124

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Http.cinch`. Source SHA-256: `28ce800080f068afac0b699ae592bf9630a2d20f5ea2246d171a0cdea87e774c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
