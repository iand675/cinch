# Crypto.Tls

Pure sans-I/O RFC 8446 TLS 1.3 client and server handshake state machine, key schedule, record AEAD framing, ALPN/SNI and X.509 verification.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Tls`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Tls.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.X25519](Crypto.X25519.md), [Crypto.X509](Crypto.X509.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.NetIp](Data.NetIp.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Pem](Text.Pem.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md).

Non-embedded source closure: [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.Pkix](Crypto.Pkix.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.Tls](Crypto.Tls.md), [Crypto.X25519](Crypto.X25519.md), [Crypto.X509](Crypto.X509.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.IpAddr](Data.IpAddr.md), [Data.NetIp](Data.NetIp.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Stream](Effect.Stream.md), [Text.Asn1](Text.Asn1.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Binary](Text.Binary.md), [Text.Digest](Text.Digest.md), [Text.Hex](Text.Hex.md), [Text.Md5](Text.Md5.md), [Text.Pem](Text.Pem.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Pure sans-I/O TLS 1.3 (RFC 8446) client and server state machines, key schedule,
record AEAD protection, certificate chain and signature verification, ALPN, SNI,
HelloRetryRequest, KeyUpdate, and clean close_notify alerts.

Supports all three RFC 8446 mandatory cipher suites (`TLS_AES_128_GCM_SHA256`,
`TLS_AES_256_GCM_SHA384`, `TLS_CHACHA20_POLY1305_SHA256`), both X25519 and P-256
ephemeral ECDH key exchange, and Ed25519, ECDSA P-256, and RSA-PSS (SHA-256/384/512)
certificate authentication over `Crypto.X509`.

All state transitions are pure and immutable; callers supply explicit entropy and
timestamps and drive network I/O externally (see `Effect.Tls` for sockets).

</details>

# Types and constructors

<a id="type-546c73416c657274"></a>

## `TlsAlert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsAlert</span></code></pre>

TLS 1.3 alert descriptions (RFC 8446 section 6).

[Source](/reference-sources/Crypto.Tls.cinch) · line 183

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertCloseNotify</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertUnexpectedMessage</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertBadRecordMac</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertRecordOverflow</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertHandshakeFailure</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertBadCertificate</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertUnsupportedCertificate</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertCertificateExpired</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertCertificateUnknown</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertIllegalParameter</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertUnknownCa</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertDecodeError</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertDecryptError</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertProtocolVersion</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertInsufficientSecurity</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertInternalError</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertUserCanceled</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertMissingExtension</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertUnsupportedExtension</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertUnrecognizedName</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertCertificateRequired</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertNoApplicationProtocol</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">AlertOther</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span>)</code></pre>

<a id="type-546c734365727469666963617465"></a>

## `TlsCertificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsCertificate</span></code></pre>

A leaf certificate chain paired with its private signing key.

[Source](/reference-sources/Crypto.Tls.cinch) · line 400

<a id="type-546c734369706865725375697465"></a>

## `TlsCipherSuite`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsCipherSuite</span></code></pre>

TLS 1.3 AEAD and hash cipher suites (RFC 8446 section B.4).

[Source](/reference-sources/Crypto.Tls.cinch) · line 84

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsAes128GcmSha256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsAes256GcmSha384</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsChacha20Poly1305Sha256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span></code></pre>

<a id="type-546c73436c69656e74"></a>

## `TlsClient`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsClient</span></code></pre>

Immutable sans-I/O TLS 1.3 client state machine.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1696

<a id="type-546c73436f6e666967"></a>

## `TlsConfig`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsConfig</span></code></pre>

Configuration for TLS 1.3 client and server handshakes.

[Source](/reference-sources/Crypto.Tls.cinch) · line 454

<a id="type-546c73436f6e6e656374696f6e5374617465"></a>

## `TlsConnectionState`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsConnectionState</span></code></pre>

Snapshot of negotiated parameters for a TLS 1.3 connection.

[Source](/reference-sources/Crypto.Tls.cinch) · line 552

<a id="type-546c734375727665"></a>

## `TlsCurve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsCurve</span></code></pre>

Named elliptic curves supported for TLS 1.3 ECDHE key exchange.

[Source](/reference-sources/Crypto.Tls.cinch) · line 119

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CurveX25519</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCurve</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">CurveP256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCurve</span></code></pre>

<a id="type-546c734572726f72"></a>

## `TlsError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsError</span></code></pre>

Structured errors returned by TLS 1.3 key schedule, record, and handshake operations.

[Source](/reference-sources/Crypto.Tls.cinch) · line 313

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsProtocolError</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsAlertReceived</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsBadRecordMac</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsRecordOverflow</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsUnsupportedVersion</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsNoCommonCipherSuite</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsNoCommonCurve</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsNoCommonSignatureScheme</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsBadSignature</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsVerifyDataMismatch</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsCertVerifyFailed</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsInvalidConfig</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsInvalidEntropy</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsHandshakeNotComplete</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsConnectionClosed</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">TlsKeyExchangeFailed</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span>)</code></pre>

<a id="type-546c734f7574707574"></a>

## `TlsOutput`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsOutput</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

Output produced by feeding incoming wire bytes into a client or server state machine.

[Source](/reference-sources/Crypto.Tls.cinch) · line 585

<a id="type-546c73536572766572"></a>

## `TlsServer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsServer</span></code></pre>

Immutable sans-I/O TLS 1.3 server state machine.

[Source](/reference-sources/Crypto.Tls.cinch) · line 2209

<a id="type-546c735369676e6174757265536368656d65"></a>

## `TlsSignatureScheme`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsSignatureScheme</span></code></pre>

TLS 1.3 `SignatureScheme` values supported for `CertificateVerify`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 143

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigRsaPssRsaeSha256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsSignatureScheme</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigRsaPssRsaeSha384</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsSignatureScheme</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigRsaPssRsaeSha512</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsSignatureScheme</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigEcdsaSecp256r1Sha256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsSignatureScheme</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigEd25519</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsSignatureScheme</span></code></pre>

<a id="type-546c7356657273696f6e"></a>

## `TlsVersion`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">TlsVersion</span></code></pre>

Supported TLS protocol versions.

[Source](/reference-sources/Crypto.Tls.cinch) · line 65

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Tls13</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsVersion</span></code></pre>

# Values

<a id="value-746c732d616c6572742d636f6465"></a>

## `tls-alert-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-alert-code</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the 8-bit wire code of a `TlsAlert`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 232

<a id="value-746c732d616c6572742d66726f6d2d636f6465"></a>

## `tls-alert-from-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-alert-from-code</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span>)</code></pre>

Decode an 8-bit wire alert description code into a `TlsAlert`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 259

<a id="value-746c732d616c6572742d6d657373616765"></a>

## `tls-alert-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-alert-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a readable name for a `TlsAlert`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 286

<a id="value-746c732d6365727469666963617465"></a>

## `tls-certificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-certificate</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span>) -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCertificate</span>))</code></pre>

Construct a `TlsCertificate` from a certificate chain (leaf first) and private key.

[Source](/reference-sources/Crypto.Tls.cinch) · line 405

<a id="value-746c732d63657274696669636174652d636861696e"></a>

## `tls-certificate-chain`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-certificate-chain</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCertificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span>))</code></pre>

Return the certificate chain (leaf first) of a `TlsCertificate`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 409

<a id="value-746c732d63657274696669636174652d6b6579"></a>

## `tls-certificate-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-certificate-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCertificate</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span>)</code></pre>

Return the private signing key of a `TlsCertificate`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 414

<a id="value-746c732d6369706865722d73756974652d657175616c"></a>

## `tls-cipher-suite-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-cipher-suite-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return `True` when two `TlsCipherSuite` values are equal.

[Source](/reference-sources/Crypto.Tls.cinch) · line 115

<a id="value-746c732d6369706865722d73756974652d66726f6d2d6964"></a>

## `tls-cipher-suite-from-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-cipher-suite-from-id</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span>))</code></pre>

Decode a 16-bit wire identifier into a supported `TlsCipherSuite`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 100

<a id="value-746c732d6369706865722d73756974652d6964"></a>

## `tls-cipher-suite-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-cipher-suite-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the 16-bit wire identifier of a `TlsCipherSuite`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 93

<a id="value-746c732d6369706865722d73756974652d6e616d65"></a>

## `tls-cipher-suite-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-cipher-suite-name</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the IANA standard name of a `TlsCipherSuite`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 108

<a id="value-746c732d636c69656e742d636c6f7365"></a>

## `tls-client-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-client-close</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span>, <span class="tok-constructor">Bytes</span>)))</code></pre>

Emit an encrypted `close_notify` alert from a `TlsClient` and mark it closed.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1770

<a id="value-746c732d636c69656e742d6578706f72742d6b6579696e672d6d6174657269616c"></a>

## `tls-client-export-keying-material`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-client-export-keying-material</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Export RFC 8446 section 7.5 keying material from an established `TlsClient`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1717

<a id="value-746c732d636c69656e742d66656564"></a>

## `tls-client-feed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-client-feed</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsOutput</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span>))))</code></pre>

Feed incoming wire bytes into a `TlsClient`, processing all complete TLS records.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1783

<a id="value-746c732d636c69656e742d68616e647368616b652d636f6d706c657465"></a>

## `tls-client-handshake-complete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-client-handshake-complete</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return `True` if a `TlsClient` has completed the TLS 1.3 handshake.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1701

<a id="value-746c732d636c69656e742d6b65792d757064617465"></a>

## `tls-client-key-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-client-key-update</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span>, <span class="tok-constructor">Bytes</span>))))</code></pre>

Initiate a TLS 1.3 `KeyUpdate` from an established `TlsClient` and advance its write keys.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1756

<a id="value-746c732d636c69656e742d73656e64"></a>

## `tls-client-send`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-client-send</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span>, <span class="tok-constructor">Bytes</span>))))</code></pre>

Encrypt application data for transmission by an established `TlsClient`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1746

<a id="value-746c732d636c69656e742d7374617274"></a>

## `tls-client-start`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-client-start</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span>, <span class="tok-constructor">Bytes</span>))))</code></pre>

Start a new TLS 1.3 client handshake using `cfg` and at least 32 bytes of `entropy`, returning `(client, client_hello_wire_bytes)`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1725

<a id="value-746c732d636c69656e742d7374617465"></a>

## `tls-client-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-client-state</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsClient</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConnectionState</span>)</code></pre>

Inspect the negotiated `TlsConnectionState` of a `TlsClient`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1709

<a id="value-746c732d636f6e6669672d616c706e"></a>

## `tls-config-alpn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-alpn</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the configured list of ALPN protocols.

[Source](/reference-sources/Crypto.Tls.cinch) · line 544

<a id="value-746c732d636f6e6669672d6365727469666963617465"></a>

## `tls-config-certificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-certificate</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCertificate</span>))</code></pre>

Return the configured local certificate chain and key, if any.

[Source](/reference-sources/Crypto.Tls.cinch) · line 532

<a id="value-746c732d636f6e6669672d6369706865722d737569746573"></a>

## `tls-config-cipher-suites`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-cipher-suites</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span>))</code></pre>

Return the configured list of cipher suites.

[Source](/reference-sources/Crypto.Tls.cinch) · line 536

<a id="value-746c732d636f6e6669672d637572766573"></a>

## `tls-config-curves`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-curves</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCurve</span>))</code></pre>

Return the configured list of ECDHE curves.

[Source](/reference-sources/Crypto.Tls.cinch) · line 540

<a id="value-746c732d636f6e6669672d64656661756c74"></a>

## `tls-config-default`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-default</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span></code></pre>

Default TLS 1.3 configuration with all three RFC 8446 cipher suites, X25519 and P-256, and strict certificate verification.

[Source](/reference-sources/Crypto.Tls.cinch) · line 459

<a id="value-746c732d636f6e6669672d696e7365637572652d736b69702d766572696679"></a>

## `tls-config-insecure-skip-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-insecure-skip-verify</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return whether certificate verification is skipped.

[Source](/reference-sources/Crypto.Tls.cinch) · line 528

<a id="value-746c732d636f6e6669672d6e6f77"></a>

## `tls-config-now`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-now</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the configured Unix timestamp in seconds.

[Source](/reference-sources/Crypto.Tls.cinch) · line 548

<a id="value-746c732d636f6e6669672d726f6f7473"></a>

## `tls-config-roots`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-roots</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span>)</code></pre>

Return the configured root CA pool.

[Source](/reference-sources/Crypto.Tls.cinch) · line 524

<a id="value-746c732d636f6e6669672d7365727665722d6e616d65"></a>

## `tls-config-server-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-server-name</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the configured server name.

[Source](/reference-sources/Crypto.Tls.cinch) · line 520

<a id="value-746c732d636f6e6669672d776974682d616c706e"></a>

## `tls-config-with-alpn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-with-alpn</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span>))</code></pre>

Set the ordered list of supported ALPN application protocols (e.g. `["h2", "http/1.1"]`).

[Source](/reference-sources/Crypto.Tls.cinch) · line 508

<a id="value-746c732d636f6e6669672d776974682d6365727469666963617465"></a>

## `tls-config-with-certificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-with-certificate</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCertificate</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span>))</code></pre>

Attach a local certificate chain and private key (required for servers, optional for mTLS clients).

[Source](/reference-sources/Crypto.Tls.cinch) · line 490

<a id="value-746c732d636f6e6669672d776974682d6369706865722d737569746573"></a>

## `tls-config-with-cipher-suites`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-with-cipher-suites</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span>) -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span>))</code></pre>

Override the ordered list of allowed TLS 1.3 cipher suites.

[Source](/reference-sources/Crypto.Tls.cinch) · line 496

<a id="value-746c732d636f6e6669672d776974682d637572766573"></a>

## `tls-config-with-curves`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-with-curves</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCurve</span>) -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span>))</code></pre>

Override the ordered list of allowed ECDHE curves.

[Source](/reference-sources/Crypto.Tls.cinch) · line 502

<a id="value-746c732d636f6e6669672d776974682d696e7365637572652d736b69702d766572696679"></a>

## `tls-config-with-insecure-skip-verify`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-with-insecure-skip-verify</span> :: (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span>))</code></pre>

Enable or disable certificate chain and hostname verification (testing only).

[Source](/reference-sources/Crypto.Tls.cinch) · line 484

<a id="value-746c732d636f6e6669672d776974682d6e6f77"></a>

## `tls-config-with-now`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-with-now</span> :: (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span>))</code></pre>

Set the verification wall-clock timestamp in Unix seconds.

[Source](/reference-sources/Crypto.Tls.cinch) · line 514

<a id="value-746c732d636f6e6669672d776974682d726f6f7473"></a>

## `tls-config-with-roots`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-with-roots</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span>))</code></pre>

Set the trusted root CA `CertPool` used to verify peer certificates.

[Source](/reference-sources/Crypto.Tls.cinch) · line 478

<a id="value-746c732d636f6e6669672d776974682d7365727665722d6e616d65"></a>

## `tls-config-with-server-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-config-with-server-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span>))</code></pre>

Set the expected server hostname (used for SNI when non-IP and X.509 hostname verification).

[Source](/reference-sources/Crypto.Tls.cinch) · line 472

<a id="value-746c732d63757276652d657175616c"></a>

## `tls-curve-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-curve-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCurve</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCurve</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return `True` when two `TlsCurve` values are equal.

[Source](/reference-sources/Crypto.Tls.cinch) · line 139

<a id="value-746c732d63757276652d66726f6d2d6964"></a>

## `tls-curve-from-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-curve-from-id</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCurve</span>))</code></pre>

Decode a 16-bit NamedGroup wire identifier into a supported `TlsCurve`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 132

<a id="value-746c732d63757276652d6964"></a>

## `tls-curve-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-curve-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCurve</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the 16-bit NamedGroup wire identifier of a `TlsCurve`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 126

<a id="value-746c732d6465726976652d736563726574"></a>

## `tls-derive-secret`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-derive-secret</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

RFC 8446 section 7.1 `Derive-Secret(secret, label, transcript_hash)`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 823

<a id="value-746c732d6572726f722d616c657274"></a>

## `tls-error-alert`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-error-alert</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsAlert</span>)</code></pre>

Map a `TlsError` to the RFC 8446 alert description suitable for notifying a peer.

[Source](/reference-sources/Crypto.Tls.cinch) · line 368

<a id="value-746c732d6572726f722d6d657373616765"></a>

## `tls-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a readable diagnostic for a `TlsError`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 348

<a id="value-746c732d6578706f72742d6b6579696e672d6d6174657269616c"></a>

## `tls-export-keying-material`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-export-keying-material</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> <span class="tok-constructor">Bytes</span>))))))</code></pre>

Derive RFC 8446 section 7.5 TLS-Exporter keying material from an exporter master secret.

[Source](/reference-sources/Crypto.Tls.cinch) · line 850

<a id="value-746c732d66696e69736865642d7665726966792d64617461"></a>

## `tls-finished-verify-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-finished-verify-data</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> <span class="tok-constructor">Bytes</span>))))</code></pre>

Compute the RFC 8446 section 4.4.4 `Finished` verify_data from a traffic secret and transcript hash.

[Source](/reference-sources/Crypto.Tls.cinch) · line 838

<a id="value-746c732d686b64662d657870616e642d6c6162656c"></a>

## `tls-hkdf-expand-label`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-hkdf-expand-label</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> <span class="tok-constructor">Bytes</span>))))))</code></pre>

RFC 8446 section 7.1 `HKDF-Expand-Label(secret, label, context, length)`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 811

<a id="value-746c732d686b64662d65787472616374"></a>

## `tls-hkdf-extract`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-hkdf-extract</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bytes</span>)))</code></pre>

RFC 5869 `HKDF-Extract(salt, ikm)` using the cipher suite's hash function.

[Source](/reference-sources/Crypto.Tls.cinch) · line 805

<a id="value-746c732d6c6f61642d783530392d6b65792d70616972"></a>

## `tls-load-x509-key-pair`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-load-x509-key-pair</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCertificate</span>)))</code></pre>

Parse a PEM-encoded certificate chain and PEM-encoded private key into a `TlsCertificate`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 419

<a id="value-746c732d6f75742d64617461"></a>

## `tls-out-data`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-out-data</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsOutput</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Extract decrypted application payload bytes received from the peer from a `TlsOutput`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 598

<a id="value-746c732d6f75742d656f66"></a>

## `tls-out-eof`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-out-eof</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsOutput</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return `True` if the peer has sent `close_notify`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 602

<a id="value-746c732d6f75742d7374617465"></a>

## `tls-out-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-out-state</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsOutput</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>)</code></pre>

Extract the updated state machine from a `TlsOutput`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 590

<a id="value-746c732d6f75742d77697265"></a>

## `tls-out-wire`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-out-wire</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsOutput</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Extract outbound TLS wire bytes to send to the peer from a `TlsOutput`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 594

<a id="value-746c732d7265636f72642d6e6f6e6365"></a>

## `tls-record-nonce`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-record-nonce</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Bytes</span>))</code></pre>

Compute the 12-byte per-record AEAD nonce by XORing the static IV with the big-endian sequence number.

[Source](/reference-sources/Crypto.Tls.cinch) · line 897

<a id="value-746c732d7265636f72642d6f70656e"></a>

## `tls-record-open`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-record-open</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Bytes</span>)))))))</code></pre>

Open a single TLS 1.3 encrypted record (5-byte header + ciphertext + tag), returning `(inner_content_type, plaintext)`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1024

<a id="value-746c732d7265636f72642d7365616c"></a>

## `tls-record-seal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-record-seal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> <span class="tok-constructor">Bytes</span>)))))))</code></pre>

Seal a single TLS 1.3 encrypted record from explicit key, IV, sequence number, inner content type, and plaintext.

[Source](/reference-sources/Crypto.Tls.cinch) · line 1015

<a id="value-746c732d7365727665722d636c6f7365"></a>

## `tls-server-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-server-close</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span>, <span class="tok-constructor">Bytes</span>)))</code></pre>

Emit an encrypted `close_notify` alert from a `TlsServer` and mark it closed.

[Source](/reference-sources/Crypto.Tls.cinch) · line 2278

<a id="value-746c732d7365727665722d6578706f72742d6b6579696e672d6d6174657269616c"></a>

## `tls-server-export-keying-material`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-server-export-keying-material</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Export RFC 8446 section 7.5 keying material from an established `TlsServer`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 2229

<a id="value-746c732d7365727665722d66656564"></a>

## `tls-server-feed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-server-feed</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsOutput</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span>))))</code></pre>

Feed incoming wire bytes into a `TlsServer`, processing all complete TLS records.

[Source](/reference-sources/Crypto.Tls.cinch) · line 2291

<a id="value-746c732d7365727665722d68616e647368616b652d636f6d706c657465"></a>

## `tls-server-handshake-complete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-server-handshake-complete</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return `True` if a `TlsServer` has completed the TLS 1.3 handshake.

[Source](/reference-sources/Crypto.Tls.cinch) · line 2214

<a id="value-746c732d7365727665722d6b65792d757064617465"></a>

## `tls-server-key-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-server-key-update</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span>, <span class="tok-constructor">Bytes</span>))))</code></pre>

Initiate a TLS 1.3 `KeyUpdate` from an established `TlsServer` and advance its write keys.

[Source](/reference-sources/Crypto.Tls.cinch) · line 2264

<a id="value-746c732d7365727665722d6e6577"></a>

## `tls-server-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-server-new</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConfig</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span>)))</code></pre>

Create a new sans-I/O TLS 1.3 server ready to accept a `ClientHello`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 2237

<a id="value-746c732d7365727665722d73656e64"></a>

## `tls-server-send`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-server-send</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span>, <span class="tok-constructor">Bytes</span>))))</code></pre>

Encrypt application data for transmission by an established `TlsServer`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 2254

<a id="value-746c732d7365727665722d7374617465"></a>

## `tls-server-state`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-server-state</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsServer</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConnectionState</span>)</code></pre>

Inspect the negotiated `TlsConnectionState` of a `TlsServer`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 2222

<a id="value-746c732d7369676e61747572652d736368656d652d657175616c"></a>

## `tls-signature-scheme-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-signature-scheme-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsSignatureScheme</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsSignatureScheme</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return `True` when two `TlsSignatureScheme` values are equal.

[Source](/reference-sources/Crypto.Tls.cinch) · line 175

<a id="value-746c732d7369676e61747572652d736368656d652d66726f6d2d6964"></a>

## `tls-signature-scheme-from-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-signature-scheme-from-id</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsSignatureScheme</span>))</code></pre>

Decode a 16-bit wire identifier into a supported `TlsSignatureScheme`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 165

<a id="value-746c732d7369676e61747572652d736368656d652d6964"></a>

## `tls-signature-scheme-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-signature-scheme-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsSignatureScheme</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the 16-bit wire identifier of a `TlsSignatureScheme`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 156

<a id="value-746c732d73746174652d6369706865722d7375697465"></a>

## `tls-state-cipher-suite`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-state-cipher-suite</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConnectionState</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span>)</code></pre>

Return the negotiated cipher suite.

[Source](/reference-sources/Crypto.Tls.cinch) · line 561

<a id="value-746c732d73746174652d68616e647368616b652d636f6d706c657465"></a>

## `tls-state-handshake-complete`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-state-handshake-complete</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConnectionState</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return `True` once the TLS 1.3 handshake has completed.

[Source](/reference-sources/Crypto.Tls.cinch) · line 577

<a id="value-746c732d73746174652d6e65676f7469617465642d70726f746f636f6c"></a>

## `tls-state-negotiated-protocol`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-state-negotiated-protocol</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConnectionState</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the negotiated ALPN protocol, or empty string if none was negotiated.

[Source](/reference-sources/Crypto.Tls.cinch) · line 569

<a id="value-746c732d73746174652d706565722d636572746966696361746573"></a>

## `tls-state-peer-certificates`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-state-peer-certificates</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConnectionState</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span>))</code></pre>

Return the peer's presented certificate chain (leaf first).

[Source](/reference-sources/Crypto.Tls.cinch) · line 573

<a id="value-746c732d73746174652d706565722d636c6f736564"></a>

## `tls-state-peer-closed`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-state-peer-closed</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConnectionState</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return `True` once the remote peer has sent `close_notify`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 581

<a id="value-746c732d73746174652d7365727665722d6e616d65"></a>

## `tls-state-server-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-state-server-name</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConnectionState</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the SNI server name negotiated or requested on the connection.

[Source](/reference-sources/Crypto.Tls.cinch) · line 565

<a id="value-746c732d73746174652d76657273696f6e"></a>

## `tls-state-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-state-version</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsConnectionState</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsVersion</span>)</code></pre>

Return the negotiated TLS version.

[Source](/reference-sources/Crypto.Tls.cinch) · line 557

<a id="value-746c732d747261666669632d6b657973"></a>

## `tls-traffic-keys`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-traffic-keys</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> (<span class="tok-constructor">Bytes</span>, <span class="tok-constructor">Bytes</span>))))</code></pre>

Derive the `(key, iv)` pair from a TLS 1.3 traffic secret (RFC 8446 section 7.3).

[Source](/reference-sources/Crypto.Tls.cinch) · line 828

<a id="value-746c732d7570646174652d747261666669632d736563726574"></a>

## `tls-update-traffic-secret`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-update-traffic-secret</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsCipherSuite</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Advance an application traffic secret via `HKDF-Expand-Label(secret, "traffic upd", "", Hash.length)`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 845

<a id="value-746c732d76657273696f6e2d657175616c"></a>

## `tls-version-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-version-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsVersion</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsVersion</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return `True` when two `TlsVersion` values are equal.

[Source](/reference-sources/Crypto.Tls.cinch) · line 80

<a id="value-746c732d76657273696f6e2d6964"></a>

## `tls-version-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-version-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsVersion</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the 16-bit wire identifier of a `TlsVersion`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 70

<a id="value-746c732d76657273696f6e2d6e616d65"></a>

## `tls-version-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tls-version-name</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Tls</span>.<span class="tok-constructor">TlsVersion</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Return the standard display name of a `TlsVersion`.

[Source](/reference-sources/Crypto.Tls.cinch) · line 75

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Tls.cinch`. Source SHA-256: `0d9ae9bf5935ed9b70345c27de9ed89711a2f1e0036c703200baf44b67d28efb`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
