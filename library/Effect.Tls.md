# Effect.Tls

Effectful TLS 1.3 client and server connections over Effect.Net sockets with OS entropy and clock-checked certificate verification.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Tls`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Tls.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Tls](Crypto.Tls.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Clock](Effect.Clock.md), [Effect.Net](Effect.Net.md), [Effect.Random](Effect.Random.md).

Non-embedded source closure: [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.Pkix](Crypto.Pkix.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.Tls](Crypto.Tls.md), [Crypto.X25519](Crypto.X25519.md), [Crypto.X509](Crypto.X509.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.IpAddr](Data.IpAddr.md), [Data.NetIp](Data.NetIp.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Net](Effect.Net.md), [Effect.Random](Effect.Random.md), [Effect.Stream](Effect.Stream.md), [Effect.Tls](Effect.Tls.md), [Text.Asn1](Text.Asn1.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Binary](Text.Binary.md), [Text.Digest](Text.Digest.md), [Text.Hex](Text.Hex.md), [Text.Md5](Text.Md5.md), [Text.Pem](Text.Pem.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effectful TLS 1.3 client and server connections over `Effect.Net` TCP sockets,
using operating-system entropy (`Effect.Random`) and the wall clock (`Effect.Clock`)
for X.509 certificate validity checks.

Wraps the pure RFC 8446 state machines from `Crypto.Tls` with buffered record framing,
full-buffer socket writes, automatic `KeyUpdate` replies, `NewSessionTicket` handling,
and graceful `close_notify` shutdown.

</details>

# Types and constructors

<a id="type-546c73436f6e6e"></a>

## `TlsConn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsConn</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>)</code></pre>

An established TLS 1.3 connection over a TCP `Socket` scoped to region `r`.

[Source](/reference-sources/Effect.Tls.cinch) · line 65

<a id="type-546c73496f4572726f72"></a>

## `TlsIoError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsIoError</span></code></pre>

Structured errors returned by effectful TLS 1.3 socket operations.

[Source](/reference-sources/Effect.Tls.cinch) · line 34

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsIoNet</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsIoProtocol</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsIoUnexpectedEof</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span></code></pre>

# Values

<a id="value-746c732d616363657074"></a>

## `tls-accept`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-accept</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; ((<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span>))) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>)))))</code></pre>

Accept one socket from an acceptor callback and upgrade it to a TLS 1.3 server connection.
Returns `Ok None` when the acceptor signals graceful shutdown.

[Source](/reference-sources/Effect.Tls.cinch) · line 251

<a id="value-746c732d636c69656e74"></a>

## `tls-client`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-client</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>))))</code></pre>

Upgrade an already-connected TCP `Socket` by running the TLS 1.3 client handshake.

[Source](/reference-sources/Effect.Tls.cinch) · line 230

<a id="value-746c732d636c6f7365"></a>

## `tls-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>) -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Send an encrypted `close_notify` alert and close the underlying TCP `Socket`.

[Source](/reference-sources/Effect.Tls.cinch) · line 314

<a id="value-746c732d636f6e6e656374696f6e2d7374617465"></a>

## `tls-connection-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-connection-state</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConnectionState</span>)</code></pre>

Inspect the negotiated `TlsConnectionState` of an established `TlsConn`.

[Source](/reference-sources/Effect.Tls.cinch) · line 327

<a id="value-746c732d6469616c"></a>

## `tls-dial`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-dial</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>)))))</code></pre>

Connect over TCP to `host:port` and perform a TLS 1.3 client handshake.
Defaults `server-name` to `host` when left empty in `cfg`, and populates `now-unix`
from the system wall clock when left `0`.

[Source](/reference-sources/Effect.Tls.cinch) · line 218

<a id="value-746c732d696f2d6572726f722d6d657373616765"></a>

## `tls-io-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-io-error-message</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a readable diagnostic message for a `TlsIoError`.

[Source](/reference-sources/Effect.Tls.cinch) · line 43

<a id="value-746c732d6b65792d757064617465"></a>

## `tls-key-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-key-update</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bool</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>))))</code></pre>

Initiate a TLS 1.3 `KeyUpdate` on `conn`, advancing outbound traffic keys and optionally requesting the peer to update.

[Source](/reference-sources/Effect.Tls.cinch) · line 304

<a id="value-746c732d72656164"></a>

## `tls-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span> (<span class="tok-constructor">Bytes</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>)))))</code></pre>

Read up to `max-bytes` of decrypted application data from `conn`.
Returns an empty buffer at clean end-of-stream (`close_notify` or peer socket close).

[Source](/reference-sources/Effect.Tls.cinch) · line 263

<a id="value-746c732d736572766572"></a>

## `tls-server`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-server</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span> (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>))))</code></pre>

Upgrade an already-connected TCP `Socket` by running the TLS 1.3 server handshake.

[Source](/reference-sources/Effect.Tls.cinch) · line 241

<a id="value-746c732d7772697465"></a>

## `tls-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; ((<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsIoError</span> (<span class="tok-constructor">Nat</span>, (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConn</span> <span class="tok-variable">r</span>)))))</code></pre>

Encrypt and transmit all bytes of `payload` over `conn`, returning the plaintext byte count written.

[Source](/reference-sources/Effect.Tls.cinch) · line 294

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Tls.cinch`. Source SHA-256: `789ab18ee78175b70b84b53221581600a6bfe7c6deb928a6c02ec23147a8de6e`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
