# Crypto.X509

X.509 v3 certificate parsing, creation and chain verification, RFC 6125 hostname and IP SAN checks, CSRs, and PKIX/PKCS#8/SEC1/PKCS#1 key codecs.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.X509`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.X509.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.P256](Crypto.P256.md), [Crypto.Pkix](Crypto.Pkix.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.X25519](Crypto.X25519.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.NetIp](Data.NetIp.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Asn1](Text.Asn1.md), [Text.Pem](Text.Pem.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md).

Non-embedded source closure: [Crypto.ConstantTime](Crypto.ConstantTime.md), [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.Field25519](Crypto.Field25519.md), [Crypto.Hmac](Crypto.Hmac.md), [Crypto.P256](Crypto.P256.md), [Crypto.Pkix](Crypto.Pkix.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.X25519](Crypto.X25519.md), [Crypto.X509](Crypto.X509.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.IpAddr](Data.IpAddr.md), [Data.NetIp](Data.NetIp.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Stream](Effect.Stream.md), [Text.Asn1](Text.Asn1.md), [Text.Base64.Encoding](Text.Base64.Encoding.md), [Text.Binary](Text.Binary.md), [Text.Digest](Text.Digest.md), [Text.Hex](Text.Hex.md), [Text.Md5](Text.Md5.md), [Text.Pem](Text.Pem.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md).

<details><summary>Behaviour and boundary notes from the source</summary>

X.509 v3 certificates, public/private key codecs (PKIX, PKCS#8, SEC 1, PKCS#1),
certificate signing requests (PKCS#10), hostname verification (RFC 6125),
certificate revocation list verification, and chain validation (RFC 5280).

Supports RSA (PKCS#1 v1.5 and RSASSA-PSS), ECDSA P-256, Ed25519, and X25519 keys.
All parsing, cryptographic signature verification, constraint checking, and DER
generation are pure Cinch over Crypto.Pkix, Text.Asn1, Text.Pem, Crypto.Rsa,
Crypto.Ecdsa, Crypto.P256, Crypto.Ed25519, Crypto.X25519, and Data.NetIp.

</details>

# Types and constructors

<a id="type-43657274506f6f6c"></a>

## `CertPool`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CertPool</span></code></pre>

An immutable set of trusted root or intermediate certificates.

[Source](/reference-sources/Crypto.X509.cinch) · line 1886

<a id="type-4365727469666963617465"></a>

## `Certificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Certificate</span></code></pre>

A parsed X.509 certificate.

[Source](/reference-sources/Crypto.X509.cinch) · line 1069

<a id="type-436572746966696361746552657175657374"></a>

## `CertificateRequest`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CertificateRequest</span></code></pre>

A parsed PKCS#10 Certificate Signing Request.

[Source](/reference-sources/Crypto.X509.cinch) · line 2387

<a id="type-436572746966696361746554656d706c617465"></a>

## `CertificateTemplate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CertificateTemplate</span></code></pre>

Parameters for creating an X.509 v3 certificate with `create-certificate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2151

<a id="type-43737254656d706c617465"></a>

## `CsrTemplate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">CsrTemplate</span></code></pre>

Template for creating a PKCS#10 Certificate Signing Request.

[Source](/reference-sources/Crypto.X509.cinch) · line 2517

<a id="type-456432353531395075626c69634b6579"></a>

## `Ed25519PublicKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Ed25519PublicKey</span></code></pre>

A validated 32-byte Ed25519 public key.

[Source](/reference-sources/Crypto.X509.cinch) · line 355

<a id="type-4578744b65795573616765"></a>

## `ExtKeyUsage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">ExtKeyUsage</span></code></pre>

Extended Key Usage purpose identifiers (RFC 5280 Section 4.2.1.12).

[Source](/reference-sources/Crypto.X509.cinch) · line 978

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageAny</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageServerAuth</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageClientAuth</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageCodeSigning</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageEmailProtection</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageIpsecEndSystem</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageIpsecTunnel</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageIpsecUser</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageTimeStamping</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageOcspSigning</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">ExtKeyUsageOther</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span>)</code></pre>

<a id="type-507269766174654b6579"></a>

## `PrivateKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PrivateKey</span></code></pre>

A supported X.509 private key.

[Source](/reference-sources/Crypto.X509.cinch) · line 396

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PrivRsa</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PrivP256</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PrivEd25519</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Ed25519</span>.<span class="tok-constructor">Ed25519PrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PrivX25519</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X25519</span>.<span class="tok-constructor">X25519PrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span>)</code></pre>

<a id="type-5075626c69634b6579"></a>

## `PublicKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PublicKey</span></code></pre>

A supported X.509 public key.

[Source](/reference-sources/Crypto.X509.cinch) · line 385

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PubRsa</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PubP256</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PublicKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PubEd25519</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Ed25519PublicKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PubX25519</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X25519PublicKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span>)</code></pre>

<a id="type-5369676e6174757265416c676f726974686d"></a>

## `SignatureAlgorithm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">SignatureAlgorithm</span></code></pre>

Supported X.509 signature algorithms.

[Source](/reference-sources/Crypto.X509.cinch) · line 706

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigUnknownAlgorithm</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigSha1WithRsa</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigSha256WithRsa</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigSha384WithRsa</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigSha512WithRsa</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigSha256WithRsaPss</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigSha384WithRsaPss</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigSha512WithRsaPss</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigEcdsaWithSha256</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigEcdsaWithSha384</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigEcdsaWithSha512</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">SigPureEd25519</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span></code></pre>

<a id="type-5665726966794f7074696f6e73"></a>

## `VerifyOptions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">VerifyOptions</span></code></pre>

Parameters controlling X.509 certificate chain verification.

[Source](/reference-sources/Crypto.X509.cinch) · line 1924

<a id="type-5832353531395075626c69634b6579"></a>

## `X25519PublicKey`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">X25519PublicKey</span></code></pre>

A validated 32-byte X25519 public key.

[Source](/reference-sources/Crypto.X509.cinch) · line 370

<a id="type-583530394572726f72"></a>

## `X509Error`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">X509Error</span></code></pre>

Structured error values returned by X.509 parsing, signing, and verification operations.

[Source](/reference-sources/Crypto.X509.cinch) · line 64

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509Asn1Error</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1Error</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509PkixError</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509InvalidEncoding</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509InvalidKey</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509UnsupportedAlgorithm</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509SignatureAlgorithmMismatch</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509SignatureInvalid</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509Expired</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509HostnameMismatch</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509UnknownAuthority</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509NotAuthorizedToSign</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509TooManyIntermediates</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509NameConstraintViolation</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509IncompatibleUsage</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509UnhandledCriticalExtension</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">X509InvalidExtension</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span>)</code></pre>

# Values

<a id="value-636572742d617574686f726974792d6b65792d6964"></a>

## `cert-authority-key-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-authority-key-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Return the Authority Key Identifier keyIdentifier bytes, if present.

[Source](/reference-sources/Crypto.X509.cinch) · line 1158

<a id="value-636572742d62617369632d636f6e73747261696e74732d76616c6964"></a>

## `cert-basic-constraints-valid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-basic-constraints-valid</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return True iff the BasicConstraints extension was present in the certificate.

[Source](/reference-sources/Crypto.X509.cinch) · line 1114

<a id="value-636572742d646e732d6e616d6573"></a>

## `cert-dns-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-dns-names</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the DNS names from the Subject Alternative Name extension.

[Source](/reference-sources/Crypto.X509.cinch) · line 1137

<a id="value-636572742d656d61696c2d616464726573736573"></a>

## `cert-email-addresses`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-email-addresses</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the email addresses from the Subject Alternative Name extension.

[Source](/reference-sources/Crypto.X509.cinch) · line 1141

<a id="value-636572742d6578636c756465642d646e732d646f6d61696e73"></a>

## `cert-excluded-dns-domains`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-excluded-dns-domains</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the excluded DNS domain constraints from the NameConstraints extension.

[Source](/reference-sources/Crypto.X509.cinch) · line 1168

<a id="value-636572742d6578636c756465642d69702d72616e676573"></a>

## `cert-excluded-ip-ranges`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-excluded-ip-ranges</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span>))</code></pre>

Return the excluded IP range constraints from the NameConstraints extension.

[Source](/reference-sources/Crypto.X509.cinch) · line 1178

<a id="value-636572742d6578742d6b65792d7573616765"></a>

## `cert-ext-key-usage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-ext-key-usage</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span>))</code></pre>

Return the list of Extended Key Usages.

[Source](/reference-sources/Crypto.X509.cinch) · line 1133

<a id="value-636572742d657874656e73696f6e73"></a>

## `cert-extensions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-extensions</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>))</code></pre>

Return all raw X.509 v3 extensions present on the certificate.

[Source](/reference-sources/Crypto.X509.cinch) · line 1183

<a id="value-636572742d69702d616464726573736573"></a>

## `cert-ip-addresses`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-ip-addresses</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>))</code></pre>

Return the IP addresses from the Subject Alternative Name extension.

[Source](/reference-sources/Crypto.X509.cinch) · line 1145

<a id="value-636572742d69732d6361"></a>

## `cert-is-ca`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-is-ca</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return True iff BasicConstraints asserts cA = TRUE.

[Source](/reference-sources/Crypto.X509.cinch) · line 1119

<a id="value-636572742d697373756572"></a>

## `cert-issuer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-issuer</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>)</code></pre>

Return the certificate issuer distinguished name.

[Source](/reference-sources/Crypto.X509.cinch) · line 1086

<a id="value-636572742d6b65792d7573616765"></a>

## `cert-key-usage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-key-usage</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">U32</span>)</code></pre>

Return the KeyUsage bitmask (0 if extension is absent).

[Source](/reference-sources/Crypto.X509.cinch) · line 1129

<a id="value-636572742d6d61782d706174682d6c656e"></a>

## `cert-max-path-len`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-max-path-len</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>))</code></pre>

Return the optional BasicConstraints pathLenConstraint (`None` if unconstrained).

[Source](/reference-sources/Crypto.X509.cinch) · line 1124

<a id="value-636572742d6e6f742d6166746572"></a>

## `cert-not-after`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-not-after</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the certificate validity end timestamp in Unix seconds.

[Source](/reference-sources/Crypto.X509.cinch) · line 1098

<a id="value-636572742d6e6f742d6265666f7265"></a>

## `cert-not-before`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-not-before</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the certificate validity start timestamp in Unix seconds.

[Source](/reference-sources/Crypto.X509.cinch) · line 1094

<a id="value-636572742d7065726d69747465642d646e732d646f6d61696e73"></a>

## `cert-permitted-dns-domains`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-permitted-dns-domains</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the permitted DNS domain constraints from the NameConstraints extension.

[Source](/reference-sources/Crypto.X509.cinch) · line 1163

<a id="value-636572742d7065726d69747465642d69702d72616e676573"></a>

## `cert-permitted-ip-ranges`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-permitted-ip-ranges</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span>))</code></pre>

Return the permitted IP range constraints from the NameConstraints extension.

[Source](/reference-sources/Crypto.X509.cinch) · line 1173

<a id="value-636572742d706f6f6c2d616464"></a>

## `cert-pool-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-pool-add</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span>))</code></pre>

Return a new `CertPool` with `cert` added.

[Source](/reference-sources/Crypto.X509.cinch) · line 1895

<a id="value-636572742d706f6f6c2d617070656e642d70656d"></a>

## `cert-pool-append-pem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-pool-append-pem</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span>, <span class="tok-constructor">Bool</span>)))</code></pre>

Parse all valid `"CERTIFICATE"` PEM blocks and add them to `pool`.
Returns `(updated-pool, added-any)`, skipping invalid or non-certificate PEM blocks.

[Source](/reference-sources/Crypto.X509.cinch) · line 1901

<a id="value-636572742d706f6f6c2d6365727473"></a>

## `cert-pool-certs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-pool-certs</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span>))</code></pre>

Return all certificates stored in a `CertPool`.

[Source](/reference-sources/Crypto.X509.cinch) · line 1916

<a id="value-636572742d706f6f6c2d6c656e"></a>

## `cert-pool-len`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-pool-len</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the number of certificates in a `CertPool`.

[Source](/reference-sources/Crypto.X509.cinch) · line 1920

<a id="value-636572742d706f6f6c2d6e6577"></a>

## `cert-pool-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-pool-new</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span></code></pre>

Create an empty `CertPool`.

[Source](/reference-sources/Crypto.X509.cinch) · line 1891

<a id="value-636572742d7075626c69632d6b6579"></a>

## `cert-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-public-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span>)</code></pre>

Return the certificate subject public key.

[Source](/reference-sources/Crypto.X509.cinch) · line 1102

<a id="value-636572742d726177"></a>

## `cert-raw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-raw</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the complete DER encoding of the certificate.

[Source](/reference-sources/Crypto.X509.cinch) · line 1187

<a id="value-636572742d7261772d7462732d6365727469666963617465"></a>

## `cert-raw-tbs-certificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-raw-tbs-certificate</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the exact raw DER bytes of the TBSCertificate signed by the issuer.

[Source](/reference-sources/Crypto.X509.cinch) · line 1106

<a id="value-636572742d73657269616c2d6e756d626572"></a>

## `cert-serial-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-serial-number</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Int</span>)</code></pre>

Return the certificate serial number.

[Source](/reference-sources/Crypto.X509.cinch) · line 1078

<a id="value-636572742d7369676e6174757265"></a>

## `cert-signature`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-signature</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the raw signature bytes.

[Source](/reference-sources/Crypto.X509.cinch) · line 1110

<a id="value-636572742d7369676e61747572652d616c676f726974686d"></a>

## `cert-signature-algorithm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-signature-algorithm</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span>)</code></pre>

Return the certificate signature algorithm.

[Source](/reference-sources/Crypto.X509.cinch) · line 1082

<a id="value-636572742d7375626a656374"></a>

## `cert-subject`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-subject</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>)</code></pre>

Return the certificate subject distinguished name.

[Source](/reference-sources/Crypto.X509.cinch) · line 1090

<a id="value-636572742d7375626a6563742d6b65792d6964"></a>

## `cert-subject-key-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-subject-key-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Return the Subject Key Identifier bytes, if present.

[Source](/reference-sources/Crypto.X509.cinch) · line 1153

<a id="value-636572742d74656d706c6174652d6e6577"></a>

## `cert-template-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-template-new</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))))</code></pre>

Construct a minimal `CertificateTemplate` with serial, subject, notBefore, and notAfter.

[Source](/reference-sources/Crypto.X509.cinch) · line 2156

<a id="value-636572742d75726973"></a>

## `cert-uris`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-uris</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the URIs from the Subject Alternative Name extension.

[Source](/reference-sources/Crypto.X509.cinch) · line 1149

<a id="value-636572742d76657273696f6e"></a>

## `cert-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cert-version</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the X.509 certificate version (1, 2, or 3).

[Source](/reference-sources/Crypto.X509.cinch) · line 1074

<a id="value-636865636b2d63657274696669636174652d726571756573742d7369676e6174757265"></a>

## `check-certificate-request-signature`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">check-certificate-request-signature</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Unit</span>))</code></pre>

Verify the self-signature on a PKCS#10 `CertificateRequest`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2512

<a id="value-636865636b2d63726c2d7369676e61747572652d66726f6d"></a>

## `check-crl-signature-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">check-crl-signature-from</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Verify that `crl` carries a valid signature created by `issuer`.
Checks KeyUsageCRLSign when KeyUsage is present on `issuer`, then verifies the signature.

[Source](/reference-sources/Crypto.X509.cinch) · line 1708

<a id="value-636865636b2d7369676e6174757265"></a>

## `check-signature`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">check-signature</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Unit</span>)))))</code></pre>

Verify `signature` over raw `signed` bytes using `pub` and `alg`.

[Source](/reference-sources/Crypto.X509.cinch) · line 1629

<a id="value-636865636b2d7369676e61747572652d66726f6d"></a>

## `check-signature-from`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">check-signature-from</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Verify that `cert` carries a valid signature created by `parent`.
Checks parent CA authority (BasicConstraints and KeyUsageCertSign when present) and signature.

[Source](/reference-sources/Crypto.X509.cinch) · line 1698

<a id="value-6372656174652d6365727469666963617465"></a>

## `create-certificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">create-certificate</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span>) -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Bytes</span>)))))</code></pre>

Create and sign an X.509 v3 certificate in DER format.
Pass `None` as `parent` for a self-signed certificate, or `Some ca` for a CA-signed certificate.

[Source](/reference-sources/Crypto.X509.cinch) · line 2348

<a id="value-6372656174652d63657274696669636174652d72657175657374"></a>

## `create-certificate-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">create-certificate-request</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Create and sign a PKCS#10 Certificate Signing Request in DER format.

[Source](/reference-sources/Crypto.X509.cinch) · line 2556

<a id="value-6372656174652d7265766f636174696f6e2d6c697374"></a>

## `create-revocation-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">create-revocation-list</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span>) -&gt; (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Bytes</span>))))))))</code></pre>

Create and sign an X.509 v2 Certificate Revocation List in DER format.

[Source](/reference-sources/Crypto.X509.cinch) · line 2597

<a id="value-6373722d646e732d6e616d6573"></a>

## `csr-dns-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-dns-names</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the requested SAN DNS names from the CSR.

[Source](/reference-sources/Crypto.X509.cinch) · line 2420

<a id="value-6373722d656d61696c2d616464726573736573"></a>

## `csr-email-addresses`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-email-addresses</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the requested SAN email addresses from the CSR.

[Source](/reference-sources/Crypto.X509.cinch) · line 2424

<a id="value-6373722d657874656e73696f6e73"></a>

## `csr-extensions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-extensions</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>))</code></pre>

Return all requested X.509 extensions attached to the CSR.

[Source](/reference-sources/Crypto.X509.cinch) · line 2436

<a id="value-6373722d69702d616464726573736573"></a>

## `csr-ip-addresses`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-ip-addresses</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>))</code></pre>

Return the requested SAN IP addresses from the CSR.

[Source](/reference-sources/Crypto.X509.cinch) · line 2428

<a id="value-6373722d7075626c69632d6b6579"></a>

## `csr-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-public-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span>)</code></pre>

Return the requested subject public key.

[Source](/reference-sources/Crypto.X509.cinch) · line 2400

<a id="value-6373722d726177"></a>

## `csr-raw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-raw</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the complete DER encoding of the CSR.

[Source](/reference-sources/Crypto.X509.cinch) · line 2416

<a id="value-6373722d7261772d746273"></a>

## `csr-raw-tbs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-raw-tbs</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the exact raw DER bytes of CertificationRequestInfo.

[Source](/reference-sources/Crypto.X509.cinch) · line 2412

<a id="value-6373722d7369676e6174757265"></a>

## `csr-signature`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-signature</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the raw CSR signature bytes.

[Source](/reference-sources/Crypto.X509.cinch) · line 2408

<a id="value-6373722d7369676e61747572652d616c676f726974686d"></a>

## `csr-signature-algorithm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-signature-algorithm</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span>)</code></pre>

Return the CSR signature algorithm.

[Source](/reference-sources/Crypto.X509.cinch) · line 2404

<a id="value-6373722d7375626a656374"></a>

## `csr-subject`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-subject</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>)</code></pre>

Return the requested subject distinguished name.

[Source](/reference-sources/Crypto.X509.cinch) · line 2396

<a id="value-6373722d74656d706c6174652d6e6577"></a>

## `csr-template-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-template-new</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span>)</code></pre>

Create a minimal `CsrTemplate` for the given subject distinguished name.

[Source](/reference-sources/Crypto.X509.cinch) · line 2522

<a id="value-6373722d74656d706c6174652d776974682d646e732d6e616d6573"></a>

## `csr-template-with-dns-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-template-with-dns-names</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span>))</code></pre>

Set requested DNS names on a `CsrTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2531

<a id="value-6373722d74656d706c6174652d776974682d656d61696c2d616464726573736573"></a>

## `csr-template-with-email-addresses`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-template-with-email-addresses</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span>))</code></pre>

Set requested email addresses on a `CsrTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2536

<a id="value-6373722d74656d706c6174652d776974682d657874656e73696f6e73"></a>

## `csr-template-with-extensions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-template-with-extensions</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span>))</code></pre>

Attach extra requested extensions to a `CsrTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2551

<a id="value-6373722d74656d706c6174652d776974682d69702d616464726573736573"></a>

## `csr-template-with-ip-addresses`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-template-with-ip-addresses</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span>))</code></pre>

Set requested IP addresses on a `CsrTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2541

<a id="value-6373722d74656d706c6174652d776974682d7369672d616c67"></a>

## `csr-template-with-sig-alg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-template-with-sig-alg</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span>))</code></pre>

Override the signature algorithm on a `CsrTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2526

<a id="value-6373722d74656d706c6174652d776974682d75726973"></a>

## `csr-template-with-uris`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-template-with-uris</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CsrTemplate</span>))</code></pre>

Set requested URIs on a `CsrTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2546

<a id="value-6373722d75726973"></a>

## `csr-uris`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-uris</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the requested SAN URIs from the CSR.

[Source](/reference-sources/Crypto.X509.cinch) · line 2432

<a id="value-6373722d76657273696f6e"></a>

## `csr-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">csr-version</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the CSR version (always 0 for PKCS#10).

[Source](/reference-sources/Crypto.X509.cinch) · line 2392

<a id="value-656432353531392d7075626c69632d6b65792d6279746573"></a>

## `ed25519-public-key-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-public-key-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Ed25519PublicKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return an independent copy of an `Ed25519PublicKey`'s 32 bytes.

[Source](/reference-sources/Crypto.X509.cinch) · line 366

<a id="value-656432353531392d7075626c69632d6b65792d66726f6d2d6279746573"></a>

## `ed25519-public-key-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ed25519-public-key-from-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Ed25519PublicKey</span>))</code></pre>

Validate and construct an `Ed25519PublicKey` from 32 raw bytes.

[Source](/reference-sources/Crypto.X509.cinch) · line 360

<a id="value-656b752d657175616c"></a>

## `eku-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eku-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return True iff two `ExtKeyUsage` values represent the same OID.

[Source](/reference-sources/Crypto.X509.cinch) · line 1033

<a id="value-656b752d66726f6d2d6f6964"></a>

## `eku-from-oid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eku-from-oid</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span>)</code></pre>

Map an OID arc list to an `ExtKeyUsage` value.

[Source](/reference-sources/Crypto.X509.cinch) · line 1018

<a id="value-656b752d746f2d6f6964"></a>

## `eku-to-oid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">eku-to-oid</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>))</code></pre>

Convert an `ExtKeyUsage` to its OID arc list.

[Source](/reference-sources/Crypto.X509.cinch) · line 1003

<a id="value-6b65792d75736167652d636572742d7369676e"></a>

## `key-usage-cert-sign`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-cert-sign</span> :: <span class="tok-constructor">U32</span></code></pre>

KeyUsage bit 5: keyCertSign.

[Source](/reference-sources/Crypto.X509.cinch) · line 905

<a id="value-6b65792d75736167652d636f6e74656e742d636f6d6d69746d656e74"></a>

## `key-usage-content-commitment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-content-commitment</span> :: <span class="tok-constructor">U32</span></code></pre>

KeyUsage bit 1: contentCommitment (nonRepudiation).

[Source](/reference-sources/Crypto.X509.cinch) · line 889

<a id="value-6b65792d75736167652d63726c2d7369676e"></a>

## `key-usage-crl-sign`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-crl-sign</span> :: <span class="tok-constructor">U32</span></code></pre>

KeyUsage bit 6: cRLSign.

[Source](/reference-sources/Crypto.X509.cinch) · line 909

<a id="value-6b65792d75736167652d646174612d656e6369706865726d656e74"></a>

## `key-usage-data-encipherment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-data-encipherment</span> :: <span class="tok-constructor">U32</span></code></pre>

KeyUsage bit 3: dataEncipherment.

[Source](/reference-sources/Crypto.X509.cinch) · line 897

<a id="value-6b65792d75736167652d64656369706865722d6f6e6c79"></a>

## `key-usage-decipher-only`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-decipher-only</span> :: <span class="tok-constructor">U32</span></code></pre>

KeyUsage bit 8: decipherOnly.

[Source](/reference-sources/Crypto.X509.cinch) · line 917

<a id="value-6b65792d75736167652d6469676974616c2d7369676e6174757265"></a>

## `key-usage-digital-signature`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-digital-signature</span> :: <span class="tok-constructor">U32</span></code></pre>

KeyUsage bit 0: digitalSignature.

[Source](/reference-sources/Crypto.X509.cinch) · line 885

<a id="value-6b65792d75736167652d656e6369706865722d6f6e6c79"></a>

## `key-usage-encipher-only`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-encipher-only</span> :: <span class="tok-constructor">U32</span></code></pre>

KeyUsage bit 7: encipherOnly.

[Source](/reference-sources/Crypto.X509.cinch) · line 913

<a id="value-6b65792d75736167652d686173"></a>

## `key-usage-has`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-has</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return True iff all flags in `mask` are set in `usage`.

[Source](/reference-sources/Crypto.X509.cinch) · line 921

<a id="value-6b65792d75736167652d6b65792d61677265656d656e74"></a>

## `key-usage-key-agreement`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-key-agreement</span> :: <span class="tok-constructor">U32</span></code></pre>

KeyUsage bit 4: keyAgreement.

[Source](/reference-sources/Crypto.X509.cinch) · line 901

<a id="value-6b65792d75736167652d6b65792d656e6369706865726d656e74"></a>

## `key-usage-key-encipherment`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-key-encipherment</span> :: <span class="tok-constructor">U32</span></code></pre>

KeyUsage bit 2: keyEncipherment.

[Source](/reference-sources/Crypto.X509.cinch) · line 893

<a id="value-6b65792d75736167652d6e6f6e65"></a>

## `key-usage-none`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-none</span> :: <span class="tok-constructor">U32</span></code></pre>

Empty KeyUsage bitmask.

[Source](/reference-sources/Crypto.X509.cinch) · line 881

<a id="value-6b65792d75736167652d756e696f6e"></a>

## `key-usage-union`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">key-usage-union</span> :: (<span class="tok-constructor">U32</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">U32</span>))</code></pre>

Combine two KeyUsage bitmasks.

[Source](/reference-sources/Crypto.X509.cinch) · line 925

<a id="value-6d61727368616c2d65632d707269766174652d6b6579"></a>

## `marshal-ec-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">marshal-ec-private-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Marshal a NIST P-256 private key to standalone SEC 1 ASN.1 DER (including curve OID and public key).

[Source](/reference-sources/Crypto.X509.cinch) · line 544

<a id="value-6d61727368616c2d706b6373312d707269766174652d6b6579"></a>

## `marshal-pkcs1-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">marshal-pkcs1-private-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Marshal an RSA private key to PKCS#1 ASN.1 DER form (delegating to `Crypto.Rsa`).

[Source](/reference-sources/Crypto.X509.cinch) · line 435

<a id="value-6d61727368616c2d706b6373312d7075626c69632d6b6579"></a>

## `marshal-pkcs1-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">marshal-pkcs1-public-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Marshal an RSA public key to PKCS#1 ASN.1 DER form (delegating to `Crypto.Rsa`).

[Source](/reference-sources/Crypto.X509.cinch) · line 425

<a id="value-6d61727368616c2d706b6373382d707269766174652d6b6579"></a>

## `marshal-pkcs8-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">marshal-pkcs8-private-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Marshal a `PrivateKey` to PKCS#8 PrivateKeyInfo DER bytes (RFC 5208 / RFC 5958).

[Source](/reference-sources/Crypto.X509.cinch) · line 617

<a id="value-6d61727368616c2d706b69782d7075626c69632d6b6579"></a>

## `marshal-pkix-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">marshal-pkix-public-key</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Marshal a `PublicKey` to PKIX SubjectPublicKeyInfo DER bytes.

[Source](/reference-sources/Crypto.X509.cinch) · line 467

<a id="value-70617273652d6365727469666963617465"></a>

## `parse-certificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-certificate</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span>))</code></pre>

Parse a single DER-encoded X.509 certificate.

[Source](/reference-sources/Crypto.X509.cinch) · line 1491

<a id="value-70617273652d63657274696669636174652d72657175657374"></a>

## `parse-certificate-request`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-certificate-request</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateRequest</span>))</code></pre>

Parse a DER-encoded PKCS#10 Certificate Signing Request.

[Source](/reference-sources/Crypto.X509.cinch) · line 2440

<a id="value-70617273652d636572746966696361746573"></a>

## `parse-certificates`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-certificates</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span>)))</code></pre>

Parse one or more concatenated DER-encoded X.509 certificates.

[Source](/reference-sources/Crypto.X509.cinch) · line 1593

<a id="value-70617273652d6365727469666963617465732d70656d"></a>

## `parse-certificates-pem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-certificates-pem</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span>)))</code></pre>

Parse all `"CERTIFICATE"` PEM blocks from a PEM byte stream.
Non-certificate PEM blocks are skipped; malformed certificate DER fails with an error.

[Source](/reference-sources/Crypto.X509.cinch) · line 1610

<a id="value-70617273652d65632d707269766174652d6b6579"></a>

## `parse-ec-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-ec-private-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">P256</span>.<span class="tok-constructor">P256PrivateKey</span>))</code></pre>

Parse a SEC 1 ECPrivateKey DER buffer into a NIST P-256 private key.

[Source](/reference-sources/Crypto.X509.cinch) · line 548

<a id="value-70617273652d706b6373312d707269766174652d6b6579"></a>

## `parse-pkcs1-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-pkcs1-private-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPrivateKey</span>))</code></pre>

Parse an RSA private key in PKCS#1 ASN.1 DER form (delegating to `Crypto.Rsa`).

[Source](/reference-sources/Crypto.X509.cinch) · line 429

<a id="value-70617273652d706b6373312d7075626c69632d6b6579"></a>

## `parse-pkcs1-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-pkcs1-public-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Rsa</span>.<span class="tok-constructor">RsaPublicKey</span>))</code></pre>

Parse an RSA public key in PKCS#1 ASN.1 DER form (delegating to `Crypto.Rsa`).

[Source](/reference-sources/Crypto.X509.cinch) · line 419

<a id="value-70617273652d706b6373382d707269766174652d6b6579"></a>

## `parse-pkcs8-private-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-pkcs8-private-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span>))</code></pre>

Parse a PKCS#8 PrivateKeyInfo DER buffer into a `PrivateKey`.

[Source](/reference-sources/Crypto.X509.cinch) · line 645

<a id="value-70617273652d706b69782d7075626c69632d6b6579"></a>

## `parse-pkix-public-key`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-pkix-public-key</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span>))</code></pre>

Parse a PKIX SubjectPublicKeyInfo DER buffer into a `PublicKey`.

[Source](/reference-sources/Crypto.X509.cinch) · line 518

<a id="value-7075626c69632d6b65792d657175616c"></a>

## `public-key-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">public-key-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return True iff two `PublicKey` values are of the same type and equal.

[Source](/reference-sources/Crypto.X509.cinch) · line 415

<a id="value-7075626c69632d6b65792d6f66"></a>

## `public-key-of`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">public-key-of</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PrivateKey</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">PublicKey</span>)</code></pre>

Derive the corresponding `PublicKey` from a `PrivateKey`.

[Source](/reference-sources/Crypto.X509.cinch) · line 407

<a id="value-7369672d616c672d657175616c"></a>

## `sig-alg-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sig-alg-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return True iff two `SignatureAlgorithm` values are equal.

[Source](/reference-sources/Crypto.X509.cinch) · line 733

<a id="value-7369672d616c672d66726f6d2d706b6978"></a>

## `sig-alg-from-pkix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sig-alg-from-pkix</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span>)</code></pre>

Recognize a `SignatureAlgorithm` from a `PkixAlgorithmIdentifier`.

[Source](/reference-sources/Crypto.X509.cinch) · line 780

<a id="value-7369672d616c672d746f2d706b6978"></a>

## `sig-alg-to-pkix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sig-alg-to-pkix</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span>))</code></pre>

Convert a `SignatureAlgorithm` to its PKIX `PkixAlgorithmIdentifier`.

[Source](/reference-sources/Crypto.X509.cinch) · line 764

<a id="value-74656d706c6174652d776974682d617574686f726974792d6b65792d6964"></a>

## `template-with-authority-key-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-authority-key-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set Authority Key Identifier bytes on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2221

<a id="value-74656d706c6174652d776974682d6361"></a>

## `template-with-ca`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-ca</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set BasicConstraints `cA` flag (and mark BasicConstraints valid) on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2167

<a id="value-74656d706c6174652d776974682d646e732d6e616d6573"></a>

## `template-with-dns-names`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-dns-names</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set SAN DNS names on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2191

<a id="value-74656d706c6174652d776974682d656d61696c2d616464726573736573"></a>

## `template-with-email-addresses`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-email-addresses</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set SAN email addresses on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2197

<a id="value-74656d706c6174652d776974682d6578636c756465642d646e73"></a>

## `template-with-excluded-dns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-excluded-dns</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set excluded DNS domain NameConstraints on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2233

<a id="value-74656d706c6174652d776974682d6578636c756465642d697073"></a>

## `template-with-excluded-ips`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-excluded-ips</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set excluded IP range NameConstraints on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2245

<a id="value-74656d706c6174652d776974682d6578742d6b65792d7573616765"></a>

## `template-with-ext-key-usage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-ext-key-usage</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set the Extended Key Usage list on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2185

<a id="value-74656d706c6174652d776974682d65787472612d657874656e73696f6e73"></a>

## `template-with-extra-extensions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-extra-extensions</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Attach additional raw `PkixExtension` entries to a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2251

<a id="value-74656d706c6174652d776974682d69702d616464726573736573"></a>

## `template-with-ip-addresses`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-ip-addresses</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Addr</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set SAN IP addresses on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2203

<a id="value-74656d706c6174652d776974682d6b65792d7573616765"></a>

## `template-with-key-usage`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-key-usage</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; (<span class="tok-constructor">U32</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set the KeyUsage bitmask on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2179

<a id="value-74656d706c6174652d776974682d6d61782d706174682d6c656e"></a>

## `template-with-max-path-len`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-max-path-len</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set BasicConstraints `pathLenConstraint` on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2173

<a id="value-74656d706c6174652d776974682d7065726d69747465642d646e73"></a>

## `template-with-permitted-dns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-permitted-dns</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set permitted DNS domain NameConstraints on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2227

<a id="value-74656d706c6174652d776974682d7065726d69747465642d697073"></a>

## `template-with-permitted-ips`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-permitted-ips</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">NetIp</span>.<span class="tok-constructor">Prefix</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set permitted IP range NameConstraints on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2239

<a id="value-74656d706c6174652d776974682d7369672d616c67"></a>

## `template-with-sig-alg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-sig-alg</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">SignatureAlgorithm</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Override the signature algorithm on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2162

<a id="value-74656d706c6174652d776974682d7375626a6563742d6b65792d6964"></a>

## `template-with-subject-key-id`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-subject-key-id</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set Subject Key Identifier bytes on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2215

<a id="value-74656d706c6174652d776974682d75726973"></a>

## `template-with-uris`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">template-with-uris</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertificateTemplate</span>))</code></pre>

Set SAN URIs on a `CertificateTemplate`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2209

<a id="value-7665726966792d6365727469666963617465"></a>

## `verify-certificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">verify-certificate</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">VerifyOptions</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span>)))))</code></pre>

Verify `leaf` under `opts`, returning all valid certificate chains `[leaf, ..., root]`.

[Source](/reference-sources/Crypto.X509.cinch) · line 2066

<a id="value-7665726966792d686f73746e616d65"></a>

## `verify-hostname`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">verify-hostname</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">Certificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Unit</span>)))</code></pre>

Verify that `cert` is valid for `host` (either a DNS hostname or an IPv4/IPv6 literal).
Follows RFC 6125: matches only Subject Alternative Name entries (never CommonName),
performs case-insensitive ASCII DNS comparison with single-label leftmost wildcards,
and matches IP literals (with optional IPv6 brackets) via `Data.NetIp`.

[Source](/reference-sources/Crypto.X509.cinch) · line 1725

<a id="value-7665726966792d6f7074696f6e732d6e6577"></a>

## `verify-options-new`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">verify-options-new</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">VerifyOptions</span>))</code></pre>

Create default `VerifyOptions` with the given trusted root pool and Unix verification time.

[Source](/reference-sources/Crypto.X509.cinch) · line 1929

<a id="value-7665726966792d776974682d646e732d6e616d65"></a>

## `verify-with-dns-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">verify-with-dns-name</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">VerifyOptions</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">VerifyOptions</span>))</code></pre>

Set the hostname or IP address to verify against the leaf certificate.

[Source](/reference-sources/Crypto.X509.cinch) · line 1938

<a id="value-7665726966792d776974682d696e7465726d65646961746573"></a>

## `verify-with-intermediates`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">verify-with-intermediates</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">VerifyOptions</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">CertPool</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">VerifyOptions</span>))</code></pre>

Attach an intermediate certificate pool to `VerifyOptions`.

[Source](/reference-sources/Crypto.X509.cinch) · line 1933

<a id="value-7665726966792d776974682d6b65792d757361676573"></a>

## `verify-with-key-usages`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">verify-with-key-usages</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">VerifyOptions</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">ExtKeyUsage</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">VerifyOptions</span>))</code></pre>

Set the acceptable extended key usages (empty list defaults to `[ExtKeyUsageServerAuth]`).

[Source](/reference-sources/Crypto.X509.cinch) · line 1943

<a id="value-7832353531392d7075626c69632d6b65792d6279746573"></a>

## `x25519-public-key-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519-public-key-bytes</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X25519PublicKey</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return an independent copy of an `X25519PublicKey`'s 32 bytes.

[Source](/reference-sources/Crypto.X509.cinch) · line 381

<a id="value-7832353531392d7075626c69632d6b65792d66726f6d2d6279746573"></a>

## `x25519-public-key-from-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x25519-public-key-from-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X25519PublicKey</span>))</code></pre>

Validate and construct an `X25519PublicKey` from 32 raw bytes.

[Source](/reference-sources/Crypto.X509.cinch) · line 375

<a id="value-783530392d6572726f722d6d657373616765"></a>

## `x509-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">x509-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">X509</span>.<span class="tok-constructor">X509Error</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render an `X509Error` as a human-readable diagnostic message.

[Source](/reference-sources/Crypto.X509.cinch) · line 99

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/X509.cinch`. Source SHA-256: `f707cda359c92c857d7177e0d1a79d4d02667f86f5cca50ceb5c6ba0e5766f9b`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
