# Crypto.Tls

Pure sans-I/O RFC 8446 TLS 1.3 client and server handshake state machine, key schedule, record AEAD framing, ALPN/SNI and X.509 verification.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.Tls</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Tls/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Tls">Search module</a><a href="/reference-sources/Crypto.Tls.cinch" download="Crypto.Tls.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.Aes](Crypto.Aes.md), [Crypto.ChaCha20Poly1305](Crypto.ChaCha20Poly1305.md), [Crypto.Cipher](Crypto.Cipher.md), [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.Kdf](Crypto.Kdf.md), [Crypto.P256](Crypto.P256.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.X25519](Crypto.X25519.md), [Crypto.X509](Crypto.X509.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.NetIp](Data.NetIp.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Pem](Text.Pem.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md).

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis"><summary>Synopsis (88 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TlsAlert">TlsAlert</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsAlert</strong></code></li><li><a class="syn-name" href="#t-TlsCertificate">TlsCertificate</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsCertificate</strong></code></li><li><a class="syn-name" href="#t-TlsCipherSuite">TlsCipherSuite</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsCipherSuite</strong></code></li><li><a class="syn-name" href="#t-TlsClient">TlsClient</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsClient</strong></code></li><li><a class="syn-name" href="#t-TlsConfig">TlsConfig</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsConfig</strong></code></li><li><a class="syn-name" href="#t-TlsConnectionState">TlsConnectionState</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsConnectionState</strong></code></li><li><a class="syn-name" href="#t-TlsCurve">TlsCurve</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsCurve</strong></code></li><li><a class="syn-name" href="#t-TlsError">TlsError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsError</strong></code></li><li><a class="syn-name" href="#t-TlsOutput">TlsOutput</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsOutput</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-TlsServer">TlsServer</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsServer</strong></code></li><li><a class="syn-name" href="#t-TlsSignatureScheme">TlsSignatureScheme</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsSignatureScheme</strong></code></li><li><a class="syn-name" href="#t-TlsVersion">TlsVersion</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsVersion</strong></code></li><li><a class="syn-name" href="#v-tls-alert-code">tls-alert-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-alert-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tls-alert-from-code">tls-alert-from-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-alert-from-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li><a class="syn-name" href="#v-tls-alert-message">tls-alert-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-alert-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-tls-certificate">tls-certificate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a></code></li><li><a class="syn-name" href="#v-tls-certificate-chain">tls-certificate-chain</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-certificate-chain</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a></code></li><li><a class="syn-name" href="#v-tls-certificate-key">tls-certificate-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-certificate-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a></code></li><li><a class="syn-name" href="#v-tls-cipher-suite-equal">tls-cipher-suite-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-cipher-suite-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tls-cipher-suite-from-id">tls-cipher-suite-from-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-cipher-suite-from-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a></code></li><li><a class="syn-name" href="#v-tls-cipher-suite-id">tls-cipher-suite-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-cipher-suite-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tls-cipher-suite-name">tls-cipher-suite-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-cipher-suite-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-tls-client-close">tls-client-close</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-client-close</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-tls-client-export-keying-material">tls-client-export-keying-material</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-client-export-keying-material</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-client-feed">tls-client-feed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-client-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a>)</code></li><li><a class="syn-name" href="#v-tls-client-handshake-complete">tls-client-handshake-complete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-client-handshake-complete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tls-client-key-update">tls-client-key-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-client-key-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-tls-client-send">tls-client-send</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-client-send</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-tls-client-start">tls-client-start</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-client-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-tls-client-state">tls-client-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-client-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a></code></li><li><a class="syn-name" href="#v-tls-config-alpn">tls-config-alpn</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-alpn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-tls-config-certificate">tls-config-certificate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a></code></li><li><a class="syn-name" href="#v-tls-config-cipher-suites">tls-config-cipher-suites</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-cipher-suites</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a></code></li><li><a class="syn-name" href="#v-tls-config-curves">tls-config-curves</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-curves</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a></code></li><li><a class="syn-name" href="#v-tls-config-default">tls-config-default</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code></li><li><a class="syn-name" href="#v-tls-config-insecure-skip-verify">tls-config-insecure-skip-verify</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-insecure-skip-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tls-config-now">tls-config-now</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-now</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-tls-config-roots">tls-config-roots</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-roots</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a></code></li><li><a class="syn-name" href="#v-tls-config-server-name">tls-config-server-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-server-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-tls-config-with-alpn">tls-config-with-alpn</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-with-alpn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code></li><li><a class="syn-name" href="#v-tls-config-with-certificate">tls-config-with-certificate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-with-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code></li><li><a class="syn-name" href="#v-tls-config-with-cipher-suites">tls-config-with-cipher-suites</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-with-cipher-suites</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code></li><li><a class="syn-name" href="#v-tls-config-with-curves">tls-config-with-curves</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-with-curves</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code></li><li><a class="syn-name" href="#v-tls-config-with-insecure-skip-verify">tls-config-with-insecure-skip-verify</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-with-insecure-skip-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code></li><li><a class="syn-name" href="#v-tls-config-with-now">tls-config-with-now</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-with-now</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code></li><li><a class="syn-name" href="#v-tls-config-with-roots">tls-config-with-roots</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-with-roots</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code></li><li><a class="syn-name" href="#v-tls-config-with-server-name">tls-config-with-server-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-config-with-server-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code></li><li><a class="syn-name" href="#v-tls-curve-equal">tls-curve-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-curve-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tls-curve-from-id">tls-curve-from-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-curve-from-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a></code></li><li><a class="syn-name" href="#v-tls-curve-id">tls-curve-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-curve-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tls-derive-secret">tls-derive-secret</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-derive-secret</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-error-alert">tls-error-alert</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-error-alert</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li><a class="syn-name" href="#v-tls-error-message">tls-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-tls-export-keying-material">tls-export-keying-material</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-export-keying-material</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-finished-verify-data">tls-finished-verify-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-finished-verify-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-hkdf-expand-label">tls-hkdf-expand-label</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-hkdf-expand-label</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-hkdf-extract">tls-hkdf-extract</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-hkdf-extract</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-load-x509-key-pair">tls-load-x509-key-pair</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-load-x509-key-pair</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a></code></li><li><a class="syn-name" href="#v-tls-out-data">tls-out-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-out-data</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-out-eof">tls-out-eof</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-out-eof</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tls-out-state">tls-out-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-out-state</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-tls-out-wire">tls-out-wire</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-out-wire</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-record-nonce">tls-record-nonce</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-record-nonce</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-record-open">tls-record-open</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-record-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-tls-record-seal">tls-record-seal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-record-seal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-server-close">tls-server-close</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-server-close</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-tls-server-export-keying-material">tls-server-export-keying-material</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-server-export-keying-material</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-server-feed">tls-server-feed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-server-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a>)</code></li><li><a class="syn-name" href="#v-tls-server-handshake-complete">tls-server-handshake-complete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-server-handshake-complete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tls-server-key-update">tls-server-key-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-server-key-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-tls-server-new">tls-server-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-server-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a></code></li><li><a class="syn-name" href="#v-tls-server-send">tls-server-send</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-server-send</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-tls-server-state">tls-server-state</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-server-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a></code></li><li><a class="syn-name" href="#v-tls-signature-scheme-equal">tls-signature-scheme-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-signature-scheme-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tls-signature-scheme-from-id">tls-signature-scheme-from-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-signature-scheme-from-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a></code></li><li><a class="syn-name" href="#v-tls-signature-scheme-id">tls-signature-scheme-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-signature-scheme-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tls-state-cipher-suite">tls-state-cipher-suite</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-state-cipher-suite</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a></code></li><li><a class="syn-name" href="#v-tls-state-handshake-complete">tls-state-handshake-complete</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-state-handshake-complete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tls-state-negotiated-protocol">tls-state-negotiated-protocol</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-state-negotiated-protocol</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-tls-state-peer-certificates">tls-state-peer-certificates</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-state-peer-certificates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a></code></li><li><a class="syn-name" href="#v-tls-state-peer-closed">tls-state-peer-closed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-state-peer-closed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tls-state-server-name">tls-state-server-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-state-server-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-tls-state-version">tls-state-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-state-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a></code></li><li><a class="syn-name" href="#v-tls-traffic-keys">tls-traffic-keys</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-traffic-keys</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-tls-update-traffic-secret">tls-update-traffic-secret</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-update-traffic-secret</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tls-version-equal">tls-version-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-version-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-tls-version-id">tls-version-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-version-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tls-version-name">tls-version-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tls-version-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TlsAlert">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsAlert</strong></code><span class="doc-sig-links"><a href="#t-TlsAlert" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

TLS 1.3 alert descriptions (RFC 8446 section 6).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-AlertCloseNotify"><code><strong class="tok-constructor">AlertCloseNotify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertUnexpectedMessage"><code><strong class="tok-constructor">AlertUnexpectedMessage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertBadRecordMac"><code><strong class="tok-constructor">AlertBadRecordMac</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertRecordOverflow"><code><strong class="tok-constructor">AlertRecordOverflow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertHandshakeFailure"><code><strong class="tok-constructor">AlertHandshakeFailure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertBadCertificate"><code><strong class="tok-constructor">AlertBadCertificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertUnsupportedCertificate"><code><strong class="tok-constructor">AlertUnsupportedCertificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertCertificateExpired"><code><strong class="tok-constructor">AlertCertificateExpired</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertCertificateUnknown"><code><strong class="tok-constructor">AlertCertificateUnknown</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertIllegalParameter"><code><strong class="tok-constructor">AlertIllegalParameter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertUnknownCa"><code><strong class="tok-constructor">AlertUnknownCa</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertDecodeError"><code><strong class="tok-constructor">AlertDecodeError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertDecryptError"><code><strong class="tok-constructor">AlertDecryptError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertProtocolVersion"><code><strong class="tok-constructor">AlertProtocolVersion</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertInsufficientSecurity"><code><strong class="tok-constructor">AlertInsufficientSecurity</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertInternalError"><code><strong class="tok-constructor">AlertInternalError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertUserCanceled"><code><strong class="tok-constructor">AlertUserCanceled</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertMissingExtension"><code><strong class="tok-constructor">AlertMissingExtension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertUnsupportedExtension"><code><strong class="tok-constructor">AlertUnsupportedExtension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertUnrecognizedName"><code><strong class="tok-constructor">AlertUnrecognizedName</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertCertificateRequired"><code><strong class="tok-constructor">AlertCertificateRequired</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertNoApplicationProtocol"><code><strong class="tok-constructor">AlertNoApplicationProtocol</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li><li id="k-AlertOther"><code><strong class="tok-constructor">AlertOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TlsCertificate">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsCertificate</strong></code><span class="doc-sig-links"><a href="#t-TlsCertificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L400" title="View source at line 400">Source</a></span></div>
<div class="doc-body">

A leaf certificate chain paired with its private signing key.

</div>
</article>

<article class="doc-entry" id="t-TlsCipherSuite">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsCipherSuite</strong></code><span class="doc-sig-links"><a href="#t-TlsCipherSuite" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

TLS 1.3 AEAD and hash cipher suites (RFC 8446 section B.4).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TlsAes128GcmSha256"><code><strong class="tok-constructor">TlsAes128GcmSha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a></code></li><li id="k-TlsAes256GcmSha384"><code><strong class="tok-constructor">TlsAes256GcmSha384</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a></code></li><li id="k-TlsChacha20Poly1305Sha256"><code><strong class="tok-constructor">TlsChacha20Poly1305Sha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TlsClient">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsClient</strong></code><span class="doc-sig-links"><a href="#t-TlsClient" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1696" title="View source at line 1696">Source</a></span></div>
<div class="doc-body">

Immutable sans-I/O TLS 1.3 client state machine.

</div>
</article>

<article class="doc-entry" id="t-TlsConfig">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsConfig</strong></code><span class="doc-sig-links"><a href="#t-TlsConfig" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L454" title="View source at line 454">Source</a></span></div>
<div class="doc-body">

Configuration for TLS 1.3 client and server handshakes.

</div>
</article>

<article class="doc-entry" id="t-TlsConnectionState">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsConnectionState</strong></code><span class="doc-sig-links"><a href="#t-TlsConnectionState" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L552" title="View source at line 552">Source</a></span></div>
<div class="doc-body">

Snapshot of negotiated parameters for a TLS 1.3 connection.

</div>
</article>

<article class="doc-entry" id="t-TlsCurve">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsCurve</strong></code><span class="doc-sig-links"><a href="#t-TlsCurve" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

Named elliptic curves supported for TLS 1.3 ECDHE key exchange.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-CurveX25519"><code><strong class="tok-constructor">CurveX25519</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a></code></li><li id="k-CurveP256"><code><strong class="tok-constructor">CurveP256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TlsError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsError</strong></code><span class="doc-sig-links"><a href="#t-TlsError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L313" title="View source at line 313">Source</a></span></div>
<div class="doc-body">

Structured errors returned by TLS 1.3 key schedule, record, and handshake operations.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TlsProtocolError"><code><strong class="tok-constructor">TlsProtocolError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsAlertReceived"><code><strong class="tok-constructor">TlsAlertReceived</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsBadRecordMac"><code><strong class="tok-constructor">TlsBadRecordMac</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsRecordOverflow"><code><strong class="tok-constructor">TlsRecordOverflow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsUnsupportedVersion"><code><strong class="tok-constructor">TlsUnsupportedVersion</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsNoCommonCipherSuite"><code><strong class="tok-constructor">TlsNoCommonCipherSuite</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsNoCommonCurve"><code><strong class="tok-constructor">TlsNoCommonCurve</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsNoCommonSignatureScheme"><code><strong class="tok-constructor">TlsNoCommonSignatureScheme</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsBadSignature"><code><strong class="tok-constructor">TlsBadSignature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsVerifyDataMismatch"><code><strong class="tok-constructor">TlsVerifyDataMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsCertVerifyFailed"><code><strong class="tok-constructor">TlsCertVerifyFailed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsInvalidConfig"><code><strong class="tok-constructor">TlsInvalidConfig</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsInvalidEntropy"><code><strong class="tok-constructor">TlsInvalidEntropy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsHandshakeNotComplete"><code><strong class="tok-constructor">TlsHandshakeNotComplete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsConnectionClosed"><code><strong class="tok-constructor">TlsConnectionClosed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li><li id="k-TlsKeyExchangeFailed"><code><strong class="tok-constructor">TlsKeyExchangeFailed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TlsOutput">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsOutput</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-TlsOutput" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L585" title="View source at line 585">Source</a></span></div>
<div class="doc-body">

Output produced by feeding incoming wire bytes into a client or server state machine.

</div>
</article>

<article class="doc-entry" id="t-TlsServer">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsServer</strong></code><span class="doc-sig-links"><a href="#t-TlsServer" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L2209" title="View source at line 2209">Source</a></span></div>
<div class="doc-body">

Immutable sans-I/O TLS 1.3 server state machine.

</div>
</article>

<article class="doc-entry" id="t-TlsSignatureScheme">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsSignatureScheme</strong></code><span class="doc-sig-links"><a href="#t-TlsSignatureScheme" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

TLS 1.3 `SignatureScheme` values supported for `CertificateVerify`.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SigRsaPssRsaeSha256"><code><strong class="tok-constructor">SigRsaPssRsaeSha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a></code></li><li id="k-SigRsaPssRsaeSha384"><code><strong class="tok-constructor">SigRsaPssRsaeSha384</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a></code></li><li id="k-SigRsaPssRsaeSha512"><code><strong class="tok-constructor">SigRsaPssRsaeSha512</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a></code></li><li id="k-SigEcdsaSecp256r1Sha256"><code><strong class="tok-constructor">SigEcdsaSecp256r1Sha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a></code></li><li id="k-SigEd25519"><code><strong class="tok-constructor">SigEd25519</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TlsVersion">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TlsVersion</strong></code><span class="doc-sig-links"><a href="#t-TlsVersion" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Supported TLS protocol versions.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Tls13"><code><strong class="tok-constructor">Tls13</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-tls-alert-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-alert-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tls-alert-code" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L232" title="View source at line 232">Source</a></span></div>
<div class="doc-body">

Return the 8-bit wire code of a `TlsAlert`.

</div>
</article>

<article class="doc-entry" id="v-tls-alert-from-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-alert-from-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code><span class="doc-sig-links"><a href="#v-tls-alert-from-code" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L259" title="View source at line 259">Source</a></span></div>
<div class="doc-body">

Decode an 8-bit wire alert description code into a `TlsAlert`.

</div>
</article>

<article class="doc-entry" id="v-tls-alert-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-alert-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-tls-alert-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L286" title="View source at line 286">Source</a></span></div>
<div class="doc-body">

Render a readable name for a `TlsAlert`.

</div>
</article>

<article class="doc-entry" id="v-tls-certificate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a></code><span class="doc-sig-links"><a href="#v-tls-certificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L405" title="View source at line 405">Source</a></span></div>
<div class="doc-body">

Construct a `TlsCertificate` from a certificate chain (leaf first) and private key.

</div>
</article>

<article class="doc-entry" id="v-tls-certificate-chain">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-certificate-chain</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a></code><span class="doc-sig-links"><a href="#v-tls-certificate-chain" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L409" title="View source at line 409">Source</a></span></div>
<div class="doc-body">

Return the certificate chain (leaf first) of a `TlsCertificate`.

</div>
</article>

<article class="doc-entry" id="v-tls-certificate-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-certificate-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a></code><span class="doc-sig-links"><a href="#v-tls-certificate-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L414" title="View source at line 414">Source</a></span></div>
<div class="doc-body">

Return the private signing key of a `TlsCertificate`.

</div>
</article>

<article class="doc-entry" id="v-tls-cipher-suite-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-cipher-suite-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tls-cipher-suite-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Return `True` when two `TlsCipherSuite` values are equal.

</div>
</article>

<article class="doc-entry" id="v-tls-cipher-suite-from-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-cipher-suite-from-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a></code><span class="doc-sig-links"><a href="#v-tls-cipher-suite-from-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

Decode a 16-bit wire identifier into a supported `TlsCipherSuite`.

</div>
</article>

<article class="doc-entry" id="v-tls-cipher-suite-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-cipher-suite-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tls-cipher-suite-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

Return the 16-bit wire identifier of a `TlsCipherSuite`.

</div>
</article>

<article class="doc-entry" id="v-tls-cipher-suite-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-cipher-suite-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-tls-cipher-suite-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Return the IANA standard name of a `TlsCipherSuite`.

</div>
</article>

<article class="doc-entry" id="v-tls-client-close">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-client-close</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-tls-client-close" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1770" title="View source at line 1770">Source</a></span></div>
<div class="doc-body">

Emit an encrypted `close_notify` alert from a `TlsClient` and mark it closed.

</div>
</article>

<article class="doc-entry" id="v-tls-client-export-keying-material">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-client-export-keying-material</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-client-export-keying-material" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1717" title="View source at line 1717">Source</a></span></div>
<div class="doc-body">

Export RFC 8446 section 7.5 keying material from an established `TlsClient`.

</div>
</article>

<article class="doc-entry" id="v-tls-client-feed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-client-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a>)</code><span class="doc-sig-links"><a href="#v-tls-client-feed" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1783" title="View source at line 1783">Source</a></span></div>
<div class="doc-body">

Feed incoming wire bytes into a `TlsClient`, processing all complete TLS records.

</div>
</article>

<article class="doc-entry" id="v-tls-client-handshake-complete">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-client-handshake-complete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tls-client-handshake-complete" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1701" title="View source at line 1701">Source</a></span></div>
<div class="doc-body">

Return `True` if a `TlsClient` has completed the TLS 1.3 handshake.

</div>
</article>

<article class="doc-entry" id="v-tls-client-key-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-client-key-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-tls-client-key-update" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1756" title="View source at line 1756">Source</a></span></div>
<div class="doc-body">

Initiate a TLS 1.3 `KeyUpdate` from an established `TlsClient` and advance its write keys.

</div>
</article>

<article class="doc-entry" id="v-tls-client-send">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-client-send</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-tls-client-send" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1746" title="View source at line 1746">Source</a></span></div>
<div class="doc-body">

Encrypt application data for transmission by an established `TlsClient`.

</div>
</article>

<article class="doc-entry" id="v-tls-client-start">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-client-start</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-tls-client-start" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1725" title="View source at line 1725">Source</a></span></div>
<div class="doc-body">

Start a new TLS 1.3 client handshake using `cfg` and at least 32 bytes of `entropy`, returning `(client, client_hello_wire_bytes)`.

</div>
</article>

<article class="doc-entry" id="v-tls-client-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-client-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsClient" title="Crypto.Tls.TlsClient">TlsClient</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a></code><span class="doc-sig-links"><a href="#v-tls-client-state" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1709" title="View source at line 1709">Source</a></span></div>
<div class="doc-body">

Inspect the negotiated `TlsConnectionState` of a `TlsClient`.

</div>
</article>

<article class="doc-entry" id="v-tls-config-alpn">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-alpn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-tls-config-alpn" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L544" title="View source at line 544">Source</a></span></div>
<div class="doc-body">

Return the configured list of ALPN protocols.

</div>
</article>

<article class="doc-entry" id="v-tls-config-certificate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a></code><span class="doc-sig-links"><a href="#v-tls-config-certificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L532" title="View source at line 532">Source</a></span></div>
<div class="doc-body">

Return the configured local certificate chain and key, if any.

</div>
</article>

<article class="doc-entry" id="v-tls-config-cipher-suites">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-cipher-suites</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a></code><span class="doc-sig-links"><a href="#v-tls-config-cipher-suites" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L536" title="View source at line 536">Source</a></span></div>
<div class="doc-body">

Return the configured list of cipher suites.

</div>
</article>

<article class="doc-entry" id="v-tls-config-curves">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-curves</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a></code><span class="doc-sig-links"><a href="#v-tls-config-curves" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L540" title="View source at line 540">Source</a></span></div>
<div class="doc-body">

Return the configured list of ECDHE curves.

</div>
</article>

<article class="doc-entry" id="v-tls-config-default">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-default</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code><span class="doc-sig-links"><a href="#v-tls-config-default" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L459" title="View source at line 459">Source</a></span></div>
<div class="doc-body">

Default TLS 1.3 configuration with all three RFC 8446 cipher suites, X25519 and P-256, and strict certificate verification.

</div>
</article>

<article class="doc-entry" id="v-tls-config-insecure-skip-verify">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-insecure-skip-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tls-config-insecure-skip-verify" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L528" title="View source at line 528">Source</a></span></div>
<div class="doc-body">

Return whether certificate verification is skipped.

</div>
</article>

<article class="doc-entry" id="v-tls-config-now">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-now</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-tls-config-now" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L548" title="View source at line 548">Source</a></span></div>
<div class="doc-body">

Return the configured Unix timestamp in seconds.

</div>
</article>

<article class="doc-entry" id="v-tls-config-roots">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-roots</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a></code><span class="doc-sig-links"><a href="#v-tls-config-roots" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L524" title="View source at line 524">Source</a></span></div>
<div class="doc-body">

Return the configured root CA pool.

</div>
</article>

<article class="doc-entry" id="v-tls-config-server-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-server-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-tls-config-server-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L520" title="View source at line 520">Source</a></span></div>
<div class="doc-body">

Return the configured server name.

</div>
</article>

<article class="doc-entry" id="v-tls-config-with-alpn">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-with-alpn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code><span class="doc-sig-links"><a href="#v-tls-config-with-alpn" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L508" title="View source at line 508">Source</a></span></div>
<div class="doc-body">

Set the ordered list of supported ALPN application protocols (e.g. `["h2", "http/1.1"]`).

</div>
</article>

<article class="doc-entry" id="v-tls-config-with-certificate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-with-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code><span class="doc-sig-links"><a href="#v-tls-config-with-certificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L490" title="View source at line 490">Source</a></span></div>
<div class="doc-body">

Attach a local certificate chain and private key (required for servers, optional for mTLS clients).

</div>
</article>

<article class="doc-entry" id="v-tls-config-with-cipher-suites">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-with-cipher-suites</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code><span class="doc-sig-links"><a href="#v-tls-config-with-cipher-suites" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L496" title="View source at line 496">Source</a></span></div>
<div class="doc-body">

Override the ordered list of allowed TLS 1.3 cipher suites.

</div>
</article>

<article class="doc-entry" id="v-tls-config-with-curves">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-with-curves</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code><span class="doc-sig-links"><a href="#v-tls-config-with-curves" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L502" title="View source at line 502">Source</a></span></div>
<div class="doc-body">

Override the ordered list of allowed ECDHE curves.

</div>
</article>

<article class="doc-entry" id="v-tls-config-with-insecure-skip-verify">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-with-insecure-skip-verify</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code><span class="doc-sig-links"><a href="#v-tls-config-with-insecure-skip-verify" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L484" title="View source at line 484">Source</a></span></div>
<div class="doc-body">

Enable or disable certificate chain and hostname verification (testing only).

</div>
</article>

<article class="doc-entry" id="v-tls-config-with-now">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-with-now</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code><span class="doc-sig-links"><a href="#v-tls-config-with-now" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L514" title="View source at line 514">Source</a></span></div>
<div class="doc-body">

Set the verification wall-clock timestamp in Unix seconds.

</div>
</article>

<article class="doc-entry" id="v-tls-config-with-roots">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-with-roots</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code><span class="doc-sig-links"><a href="#v-tls-config-with-roots" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L478" title="View source at line 478">Source</a></span></div>
<div class="doc-body">

Set the trusted root CA `CertPool` used to verify peer certificates.

</div>
</article>

<article class="doc-entry" id="v-tls-config-with-server-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-config-with-server-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a></code><span class="doc-sig-links"><a href="#v-tls-config-with-server-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L472" title="View source at line 472">Source</a></span></div>
<div class="doc-body">

Set the expected server hostname (used for SNI when non-IP and X.509 hostname verification).

</div>
</article>

<article class="doc-entry" id="v-tls-curve-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-curve-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tls-curve-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L139" title="View source at line 139">Source</a></span></div>
<div class="doc-body">

Return `True` when two `TlsCurve` values are equal.

</div>
</article>

<article class="doc-entry" id="v-tls-curve-from-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-curve-from-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a></code><span class="doc-sig-links"><a href="#v-tls-curve-from-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Decode a 16-bit NamedGroup wire identifier into a supported `TlsCurve`.

</div>
</article>

<article class="doc-entry" id="v-tls-curve-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-curve-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCurve" title="Crypto.Tls.TlsCurve">TlsCurve</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tls-curve-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

Return the 16-bit NamedGroup wire identifier of a `TlsCurve`.

</div>
</article>

<article class="doc-entry" id="v-tls-derive-secret">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-derive-secret</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-derive-secret" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L823" title="View source at line 823">Source</a></span></div>
<div class="doc-body">

RFC 8446 section 7.1 `Derive-Secret(secret, label, transcript_hash)`.

</div>
</article>

<article class="doc-entry" id="v-tls-error-alert">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-error-alert</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsAlert" title="Crypto.Tls.TlsAlert">TlsAlert</a></code><span class="doc-sig-links"><a href="#v-tls-error-alert" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L368" title="View source at line 368">Source</a></span></div>
<div class="doc-body">

Map a `TlsError` to the RFC 8446 alert description suitable for notifying a peer.

</div>
</article>

<article class="doc-entry" id="v-tls-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-tls-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L348" title="View source at line 348">Source</a></span></div>
<div class="doc-body">

Render a readable diagnostic for a `TlsError`.

</div>
</article>

<article class="doc-entry" id="v-tls-export-keying-material">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-export-keying-material</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-export-keying-material" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L850" title="View source at line 850">Source</a></span></div>
<div class="doc-body">

Derive RFC 8446 section 7.5 TLS-Exporter keying material from an exporter master secret.

</div>
</article>

<article class="doc-entry" id="v-tls-finished-verify-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-finished-verify-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-finished-verify-data" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L838" title="View source at line 838">Source</a></span></div>
<div class="doc-body">

Compute the RFC 8446 section 4.4.4 `Finished` verify_data from a traffic secret and transcript hash.

</div>
</article>

<article class="doc-entry" id="v-tls-hkdf-expand-label">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-hkdf-expand-label</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-hkdf-expand-label" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L811" title="View source at line 811">Source</a></span></div>
<div class="doc-body">

RFC 8446 section 7.1 `HKDF-Expand-Label(secret, label, context, length)`.

</div>
</article>

<article class="doc-entry" id="v-tls-hkdf-extract">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-hkdf-extract</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-hkdf-extract" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L805" title="View source at line 805">Source</a></span></div>
<div class="doc-body">

RFC 5869 `HKDF-Extract(salt, ikm)` using the cipher suite's hash function.

</div>
</article>

<article class="doc-entry" id="v-tls-load-x509-key-pair">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-load-x509-key-pair</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCertificate" title="Crypto.Tls.TlsCertificate">TlsCertificate</a></code><span class="doc-sig-links"><a href="#v-tls-load-x509-key-pair" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L419" title="View source at line 419">Source</a></span></div>
<div class="doc-body">

Parse a PEM-encoded certificate chain and PEM-encoded private key into a `TlsCertificate`.

</div>
</article>

<article class="doc-entry" id="v-tls-out-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-out-data</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-out-data" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L598" title="View source at line 598">Source</a></span></div>
<div class="doc-body">

Extract decrypted application payload bytes received from the peer from a `TlsOutput`.

</div>
</article>

<article class="doc-entry" id="v-tls-out-eof">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-out-eof</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tls-out-eof" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L602" title="View source at line 602">Source</a></span></div>
<div class="doc-body">

Return `True` if the peer has sent `close_notify`.

</div>
</article>

<article class="doc-entry" id="v-tls-out-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-out-state</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#v-tls-out-state" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L590" title="View source at line 590">Source</a></span></div>
<div class="doc-body">

Extract the updated state machine from a `TlsOutput`.

</div>
</article>

<article class="doc-entry" id="v-tls-out-wire">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-out-wire</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-out-wire" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L594" title="View source at line 594">Source</a></span></div>
<div class="doc-body">

Extract outbound TLS wire bytes to send to the peer from a `TlsOutput`.

</div>
</article>

<article class="doc-entry" id="v-tls-record-nonce">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-record-nonce</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-record-nonce" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L897" title="View source at line 897">Source</a></span></div>
<div class="doc-body">

Compute the 12-byte per-record AEAD nonce by XORing the static IV with the big-endian sequence number.

</div>
</article>

<article class="doc-entry" id="v-tls-record-open">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-record-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-tls-record-open" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1024" title="View source at line 1024">Source</a></span></div>
<div class="doc-body">

Open a single TLS 1.3 encrypted record (5-byte header + ciphertext + tag), returning `(inner_content_type, plaintext)`.

</div>
</article>

<article class="doc-entry" id="v-tls-record-seal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-record-seal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-record-seal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L1015" title="View source at line 1015">Source</a></span></div>
<div class="doc-body">

Seal a single TLS 1.3 encrypted record from explicit key, IV, sequence number, inner content type, and plaintext.

</div>
</article>

<article class="doc-entry" id="v-tls-server-close">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-server-close</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-tls-server-close" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L2278" title="View source at line 2278">Source</a></span></div>
<div class="doc-body">

Emit an encrypted `close_notify` alert from a `TlsServer` and mark it closed.

</div>
</article>

<article class="doc-entry" id="v-tls-server-export-keying-material">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-server-export-keying-material</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-server-export-keying-material" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L2229" title="View source at line 2229">Source</a></span></div>
<div class="doc-body">

Export RFC 8446 section 7.5 keying material from an established `TlsServer`.

</div>
</article>

<article class="doc-entry" id="v-tls-server-feed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-server-feed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsOutput" title="Crypto.Tls.TlsOutput">TlsOutput</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a>)</code><span class="doc-sig-links"><a href="#v-tls-server-feed" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L2291" title="View source at line 2291">Source</a></span></div>
<div class="doc-body">

Feed incoming wire bytes into a `TlsServer`, processing all complete TLS records.

</div>
</article>

<article class="doc-entry" id="v-tls-server-handshake-complete">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-server-handshake-complete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tls-server-handshake-complete" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L2214" title="View source at line 2214">Source</a></span></div>
<div class="doc-body">

Return `True` if a `TlsServer` has completed the TLS 1.3 handshake.

</div>
</article>

<article class="doc-entry" id="v-tls-server-key-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-server-key-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-tls-server-key-update" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L2264" title="View source at line 2264">Source</a></span></div>
<div class="doc-body">

Initiate a TLS 1.3 `KeyUpdate` from an established `TlsServer` and advance its write keys.

</div>
</article>

<article class="doc-entry" id="v-tls-server-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-server-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConfig" title="Crypto.Tls.TlsConfig">TlsConfig</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a></code><span class="doc-sig-links"><a href="#v-tls-server-new" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L2237" title="View source at line 2237">Source</a></span></div>
<div class="doc-body">

Create a new sans-I/O TLS 1.3 server ready to accept a `ClientHello`.

</div>
</article>

<article class="doc-entry" id="v-tls-server-send">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-server-send</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-tls-server-send" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L2254" title="View source at line 2254">Source</a></span></div>
<div class="doc-body">

Encrypt application data for transmission by an established `TlsServer`.

</div>
</article>

<article class="doc-entry" id="v-tls-server-state">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-server-state</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsServer" title="Crypto.Tls.TlsServer">TlsServer</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a></code><span class="doc-sig-links"><a href="#v-tls-server-state" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L2222" title="View source at line 2222">Source</a></span></div>
<div class="doc-body">

Inspect the negotiated `TlsConnectionState` of a `TlsServer`.

</div>
</article>

<article class="doc-entry" id="v-tls-signature-scheme-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-signature-scheme-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tls-signature-scheme-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L175" title="View source at line 175">Source</a></span></div>
<div class="doc-body">

Return `True` when two `TlsSignatureScheme` values are equal.

</div>
</article>

<article class="doc-entry" id="v-tls-signature-scheme-from-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-signature-scheme-from-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a></code><span class="doc-sig-links"><a href="#v-tls-signature-scheme-from-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Decode a 16-bit wire identifier into a supported `TlsSignatureScheme`.

</div>
</article>

<article class="doc-entry" id="v-tls-signature-scheme-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-signature-scheme-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsSignatureScheme" title="Crypto.Tls.TlsSignatureScheme">TlsSignatureScheme</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tls-signature-scheme-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L156" title="View source at line 156">Source</a></span></div>
<div class="doc-body">

Return the 16-bit wire identifier of a `TlsSignatureScheme`.

</div>
</article>

<article class="doc-entry" id="v-tls-state-cipher-suite">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-state-cipher-suite</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a></code><span class="doc-sig-links"><a href="#v-tls-state-cipher-suite" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L561" title="View source at line 561">Source</a></span></div>
<div class="doc-body">

Return the negotiated cipher suite.

</div>
</article>

<article class="doc-entry" id="v-tls-state-handshake-complete">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-state-handshake-complete</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tls-state-handshake-complete" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L577" title="View source at line 577">Source</a></span></div>
<div class="doc-body">

Return `True` once the TLS 1.3 handshake has completed.

</div>
</article>

<article class="doc-entry" id="v-tls-state-negotiated-protocol">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-state-negotiated-protocol</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-tls-state-negotiated-protocol" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L569" title="View source at line 569">Source</a></span></div>
<div class="doc-body">

Return the negotiated ALPN protocol, or empty string if none was negotiated.

</div>
</article>

<article class="doc-entry" id="v-tls-state-peer-certificates">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-state-peer-certificates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a></code><span class="doc-sig-links"><a href="#v-tls-state-peer-certificates" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L573" title="View source at line 573">Source</a></span></div>
<div class="doc-body">

Return the peer's presented certificate chain (leaf first).

</div>
</article>

<article class="doc-entry" id="v-tls-state-peer-closed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-state-peer-closed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tls-state-peer-closed" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L581" title="View source at line 581">Source</a></span></div>
<div class="doc-body">

Return `True` once the remote peer has sent `close_notify`.

</div>
</article>

<article class="doc-entry" id="v-tls-state-server-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-state-server-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-tls-state-server-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L565" title="View source at line 565">Source</a></span></div>
<div class="doc-body">

Return the SNI server name negotiated or requested on the connection.

</div>
</article>

<article class="doc-entry" id="v-tls-state-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-state-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsConnectionState" title="Crypto.Tls.TlsConnectionState">TlsConnectionState</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a></code><span class="doc-sig-links"><a href="#v-tls-state-version" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L557" title="View source at line 557">Source</a></span></div>
<div class="doc-body">

Return the negotiated TLS version.

</div>
</article>

<article class="doc-entry" id="v-tls-traffic-keys">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-traffic-keys</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-tls-traffic-keys" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L828" title="View source at line 828">Source</a></span></div>
<div class="doc-body">

Derive the `(key, iv)` pair from a TLS 1.3 traffic secret (RFC 8446 section 7.3).

</div>
</article>

<article class="doc-entry" id="v-tls-update-traffic-secret">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-update-traffic-secret</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsCipherSuite" title="Crypto.Tls.TlsCipherSuite">TlsCipherSuite</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsError" title="Crypto.Tls.TlsError">TlsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tls-update-traffic-secret" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L845" title="View source at line 845">Source</a></span></div>
<div class="doc-body">

Advance an application traffic secret via `HKDF-Expand-Label(secret, "traffic upd", "", Hash.length)`.

</div>
</article>

<article class="doc-entry" id="v-tls-version-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-version-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-tls-version-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

Return `True` when two `TlsVersion` values are equal.

</div>
</article>

<article class="doc-entry" id="v-tls-version-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-version-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tls-version-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Return the 16-bit wire identifier of a `TlsVersion`.

</div>
</article>

<article class="doc-entry" id="v-tls-version-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tls-version-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Tls/#t-TlsVersion" title="Crypto.Tls.TlsVersion">TlsVersion</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-tls-version-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Tls/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

Return the standard display name of a `TlsVersion`.

</div>
</article>
