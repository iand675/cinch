# Crypto.X509

X.509 v3 certificate parsing, creation and chain verification, RFC 6125 hostname and IP SAN checks, CSRs, and PKIX/PKCS#8/SEC1/PKCS#1 key codecs.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.X509</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.X509/source/">View source</a><a href="/packages/search/?q=%2BCrypto.X509">Search module</a><a href="/reference-sources/Crypto.X509.cinch" download="Crypto.X509.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Crypto.Ecdsa](Crypto.Ecdsa.md), [Crypto.Ed25519](Crypto.Ed25519.md), [Crypto.P256](Crypto.P256.md), [Crypto.Pkix](Crypto.Pkix.md), [Crypto.Rsa](Crypto.Rsa.md), [Crypto.X25519](Crypto.X25519.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.NetIp](Data.NetIp.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Text.Asn1](Text.Asn1.md), [Text.Pem](Text.Pem.md), [Text.Sha1](Text.Sha1.md), [Text.Sha256](Text.Sha256.md), [Text.Sha512](Text.Sha512.md).

<details class="doc-notes"><summary>Module notes</summary>

X.509 v3 certificates, public/private key codecs (PKIX, PKCS#8, SEC 1, PKCS#1),
certificate signing requests (PKCS#10), hostname verification (RFC 6125),
certificate revocation list verification, and chain validation (RFC 5280).

Supports RSA (PKCS#1 v1.5 and RSASSA-PSS), ECDSA P-256, Ed25519, and X25519 keys.
All parsing, cryptographic signature verification, constraint checking, and DER
generation are pure Cinch over Crypto.Pkix, Text.Asn1, Text.Pem, Crypto.Rsa,
Crypto.Ecdsa, Crypto.P256, Crypto.Ed25519, Crypto.X25519, and Data.NetIp.

</details>

<details class="doc-synopsis"><summary>Synopsis (133 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-CertPool">CertPool</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CertPool</strong></code></li><li><a class="syn-name" href="#t-Certificate">Certificate</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Certificate</strong></code></li><li><a class="syn-name" href="#t-CertificateRequest">CertificateRequest</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CertificateRequest</strong></code></li><li><a class="syn-name" href="#t-CertificateTemplate">CertificateTemplate</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CertificateTemplate</strong></code></li><li><a class="syn-name" href="#t-CsrTemplate">CsrTemplate</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsrTemplate</strong></code></li><li><a class="syn-name" href="#t-Ed25519PublicKey">Ed25519PublicKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ed25519PublicKey</strong></code></li><li><a class="syn-name" href="#t-ExtKeyUsage">ExtKeyUsage</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ExtKeyUsage</strong></code></li><li><a class="syn-name" href="#t-PrivateKey">PrivateKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PrivateKey</strong></code></li><li><a class="syn-name" href="#t-PublicKey">PublicKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PublicKey</strong></code></li><li><a class="syn-name" href="#t-SignatureAlgorithm">SignatureAlgorithm</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SignatureAlgorithm</strong></code></li><li><a class="syn-name" href="#t-VerifyOptions">VerifyOptions</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">VerifyOptions</strong></code></li><li><a class="syn-name" href="#t-X25519PublicKey">X25519PublicKey</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">X25519PublicKey</strong></code></li><li><a class="syn-name" href="#t-X509Error">X509Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">X509Error</strong></code></li><li><a class="syn-name" href="#v-cert-authority-key-id">cert-authority-key-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-authority-key-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cert-basic-constraints-valid">cert-basic-constraints-valid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-basic-constraints-valid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-cert-dns-names">cert-dns-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-dns-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cert-email-addresses">cert-email-addresses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-email-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cert-excluded-dns-domains">cert-excluded-dns-domains</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-excluded-dns-domains</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cert-excluded-ip-ranges">cert-excluded-ip-ranges</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-excluded-ip-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code></li><li><a class="syn-name" href="#v-cert-ext-key-usage">cert-ext-key-usage</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-ext-key-usage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li><a class="syn-name" href="#v-cert-extensions">cert-extensions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code></li><li><a class="syn-name" href="#v-cert-ip-addresses">cert-ip-addresses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-ip-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-cert-is-ca">cert-is-ca</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-is-ca</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-cert-issuer">cert-issuer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-issuer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-cert-key-usage">cert-key-usage</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-key-usage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-cert-max-path-len">cert-max-path-len</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-max-path-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-cert-not-after">cert-not-after</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-not-after</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-cert-not-before">cert-not-before</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-not-before</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-cert-permitted-dns-domains">cert-permitted-dns-domains</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-permitted-dns-domains</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cert-permitted-ip-ranges">cert-permitted-ip-ranges</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-permitted-ip-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code></li><li><a class="syn-name" href="#v-cert-pool-add">cert-pool-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-pool-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a></code></li><li><a class="syn-name" href="#v-cert-pool-append-pem">cert-pool-append-pem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-pool-append-pem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-cert-pool-certs">cert-pool-certs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-pool-certs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a></code></li><li><a class="syn-name" href="#v-cert-pool-len">cert-pool-len</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-pool-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-cert-pool-new">cert-pool-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-pool-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a></code></li><li><a class="syn-name" href="#v-cert-public-key">cert-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code></li><li><a class="syn-name" href="#v-cert-raw">cert-raw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cert-raw-tbs-certificate">cert-raw-tbs-certificate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-raw-tbs-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cert-serial-number">cert-serial-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-serial-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-cert-signature">cert-signature</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cert-signature-algorithm">cert-signature-algorithm</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-signature-algorithm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li><a class="syn-name" href="#v-cert-subject">cert-subject</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-subject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-cert-subject-key-id">cert-subject-key-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-subject-key-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-cert-template-new">cert-template-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-template-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-cert-uris">cert-uris</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-uris</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-cert-version">cert-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cert-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-check-certificate-request-signature">check-certificate-request-signature</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">check-certificate-request-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-check-crl-signature-from">check-crl-signature-from</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">check-crl-signature-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-check-signature">check-signature</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">check-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-check-signature-from">check-signature-from</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">check-signature-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-create-certificate">create-certificate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">create-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-create-certificate-request">create-certificate-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">create-certificate-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-create-revocation-list">create-revocation-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">create-revocation-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-csr-dns-names">csr-dns-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-dns-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-csr-email-addresses">csr-email-addresses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-email-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-csr-extensions">csr-extensions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code></li><li><a class="syn-name" href="#v-csr-ip-addresses">csr-ip-addresses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-ip-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code></li><li><a class="syn-name" href="#v-csr-public-key">csr-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code></li><li><a class="syn-name" href="#v-csr-raw">csr-raw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-csr-raw-tbs">csr-raw-tbs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-raw-tbs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-csr-signature">csr-signature</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-csr-signature-algorithm">csr-signature-algorithm</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-signature-algorithm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li><a class="syn-name" href="#v-csr-subject">csr-subject</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-subject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-csr-template-new">csr-template-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-template-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code></li><li><a class="syn-name" href="#v-csr-template-with-dns-names">csr-template-with-dns-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-template-with-dns-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code></li><li><a class="syn-name" href="#v-csr-template-with-email-addresses">csr-template-with-email-addresses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-template-with-email-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code></li><li><a class="syn-name" href="#v-csr-template-with-extensions">csr-template-with-extensions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-template-with-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code></li><li><a class="syn-name" href="#v-csr-template-with-ip-addresses">csr-template-with-ip-addresses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-template-with-ip-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code></li><li><a class="syn-name" href="#v-csr-template-with-sig-alg">csr-template-with-sig-alg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-template-with-sig-alg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code></li><li><a class="syn-name" href="#v-csr-template-with-uris">csr-template-with-uris</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-template-with-uris</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code></li><li><a class="syn-name" href="#v-csr-uris">csr-uris</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-uris</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-csr-version">csr-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">csr-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-ed25519-public-key-bytes">ed25519-public-key-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-public-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Ed25519PublicKey" title="Crypto.X509.Ed25519PublicKey">Ed25519PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ed25519-public-key-from-bytes">ed25519-public-key-from-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ed25519-public-key-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Ed25519PublicKey" title="Crypto.X509.Ed25519PublicKey">Ed25519PublicKey</a></code></li><li><a class="syn-name" href="#v-eku-equal">eku-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">eku-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-eku-from-oid">eku-from-oid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">eku-from-oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li><a class="syn-name" href="#v-eku-to-oid">eku-to-oid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">eku-to-oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-key-usage-cert-sign">key-usage-cert-sign</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-cert-sign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-key-usage-content-commitment">key-usage-content-commitment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-content-commitment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-key-usage-crl-sign">key-usage-crl-sign</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-crl-sign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-key-usage-data-encipherment">key-usage-data-encipherment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-data-encipherment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-key-usage-decipher-only">key-usage-decipher-only</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-decipher-only</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-key-usage-digital-signature">key-usage-digital-signature</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-digital-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-key-usage-encipher-only">key-usage-encipher-only</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-encipher-only</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-key-usage-has">key-usage-has</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-has</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-key-usage-key-agreement">key-usage-key-agreement</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-key-agreement</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-key-usage-key-encipherment">key-usage-key-encipherment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-key-encipherment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-key-usage-none">key-usage-none</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-none</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-key-usage-union">key-usage-union</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">key-usage-union</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-marshal-ec-private-key">marshal-ec-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">marshal-ec-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-marshal-pkcs1-private-key">marshal-pkcs1-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">marshal-pkcs1-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-marshal-pkcs1-public-key">marshal-pkcs1-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">marshal-pkcs1-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-marshal-pkcs8-private-key">marshal-pkcs8-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">marshal-pkcs8-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-marshal-pkix-public-key">marshal-pkix-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">marshal-pkix-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-parse-certificate">parse-certificate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a></code></li><li><a class="syn-name" href="#v-parse-certificate-request">parse-certificate-request</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-certificate-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a></code></li><li><a class="syn-name" href="#v-parse-certificates">parse-certificates</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-certificates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a>)</code></li><li><a class="syn-name" href="#v-parse-certificates-pem">parse-certificates-pem</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-certificates-pem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a>)</code></li><li><a class="syn-name" href="#v-parse-ec-private-key">parse-ec-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-ec-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a></code></li><li><a class="syn-name" href="#v-parse-pkcs1-private-key">parse-pkcs1-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-pkcs1-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a></code></li><li><a class="syn-name" href="#v-parse-pkcs1-public-key">parse-pkcs1-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-pkcs1-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a></code></li><li><a class="syn-name" href="#v-parse-pkcs8-private-key">parse-pkcs8-private-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-pkcs8-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a></code></li><li><a class="syn-name" href="#v-parse-pkix-public-key">parse-pkix-public-key</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-pkix-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code></li><li><a class="syn-name" href="#v-public-key-equal">public-key-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">public-key-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-public-key-of">public-key-of</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">public-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code></li><li><a class="syn-name" href="#v-sig-alg-equal">sig-alg-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sig-alg-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-sig-alg-from-pkix">sig-alg-from-pkix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sig-alg-from-pkix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li><a class="syn-name" href="#v-sig-alg-to-pkix">sig-alg-to-pkix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sig-alg-to-pkix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code></li><li><a class="syn-name" href="#v-template-with-authority-key-id">template-with-authority-key-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-authority-key-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-ca">template-with-ca</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-ca</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-dns-names">template-with-dns-names</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-dns-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-email-addresses">template-with-email-addresses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-email-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-excluded-dns">template-with-excluded-dns</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-excluded-dns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-excluded-ips">template-with-excluded-ips</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-excluded-ips</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-ext-key-usage">template-with-ext-key-usage</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-ext-key-usage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-extra-extensions">template-with-extra-extensions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-extra-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-ip-addresses">template-with-ip-addresses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-ip-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-key-usage">template-with-key-usage</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-key-usage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-max-path-len">template-with-max-path-len</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-max-path-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-permitted-dns">template-with-permitted-dns</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-permitted-dns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-permitted-ips">template-with-permitted-ips</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-permitted-ips</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-sig-alg">template-with-sig-alg</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-sig-alg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-subject-key-id">template-with-subject-key-id</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-subject-key-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-template-with-uris">template-with-uris</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">template-with-uris</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code></li><li><a class="syn-name" href="#v-verify-certificate">verify-certificate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">verify-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a>))</code></li><li><a class="syn-name" href="#v-verify-hostname">verify-hostname</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">verify-hostname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-verify-options-new">verify-options-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">verify-options-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a></code></li><li><a class="syn-name" href="#v-verify-with-dns-name">verify-with-dns-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">verify-with-dns-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a></code></li><li><a class="syn-name" href="#v-verify-with-intermediates">verify-with-intermediates</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">verify-with-intermediates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a></code></li><li><a class="syn-name" href="#v-verify-with-key-usages">verify-with-key-usages</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">verify-with-key-usages</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a></code></li><li><a class="syn-name" href="#v-x25519-public-key-bytes">x25519-public-key-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x25519-public-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X25519PublicKey" title="Crypto.X509.X25519PublicKey">X25519PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-x25519-public-key-from-bytes">x25519-public-key-from-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x25519-public-key-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X25519PublicKey" title="Crypto.X509.X25519PublicKey">X25519PublicKey</a></code></li><li><a class="syn-name" href="#v-x509-error-message">x509-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">x509-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-CertPool">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CertPool</strong></code><span class="doc-sig-links"><a href="#t-CertPool" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1886" title="View source at line 1886">Source</a></span></div>
<div class="doc-body">

An immutable set of trusted root or intermediate certificates.

</div>
</article>

<article class="doc-entry" id="t-Certificate">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Certificate</strong></code><span class="doc-sig-links"><a href="#t-Certificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1069" title="View source at line 1069">Source</a></span></div>
<div class="doc-body">

A parsed X.509 certificate.

</div>
</article>

<article class="doc-entry" id="t-CertificateRequest">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CertificateRequest</strong></code><span class="doc-sig-links"><a href="#t-CertificateRequest" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2387" title="View source at line 2387">Source</a></span></div>
<div class="doc-body">

A parsed PKCS#10 Certificate Signing Request.

</div>
</article>

<article class="doc-entry" id="t-CertificateTemplate">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CertificateTemplate</strong></code><span class="doc-sig-links"><a href="#t-CertificateTemplate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2151" title="View source at line 2151">Source</a></span></div>
<div class="doc-body">

Parameters for creating an X.509 v3 certificate with `create-certificate`.

</div>
</article>

<article class="doc-entry" id="t-CsrTemplate">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CsrTemplate</strong></code><span class="doc-sig-links"><a href="#t-CsrTemplate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2517" title="View source at line 2517">Source</a></span></div>
<div class="doc-body">

Template for creating a PKCS#10 Certificate Signing Request.

</div>
</article>

<article class="doc-entry" id="t-Ed25519PublicKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ed25519PublicKey</strong></code><span class="doc-sig-links"><a href="#t-Ed25519PublicKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L355" title="View source at line 355">Source</a></span></div>
<div class="doc-body">

A validated 32-byte Ed25519 public key.

</div>
</article>

<article class="doc-entry" id="t-ExtKeyUsage">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ExtKeyUsage</strong></code><span class="doc-sig-links"><a href="#t-ExtKeyUsage" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L978" title="View source at line 978">Source</a></span></div>
<div class="doc-body">

Extended Key Usage purpose identifiers (RFC 5280 Section 4.2.1.12).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ExtKeyUsageAny"><code><strong class="tok-constructor">ExtKeyUsageAny</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li id="k-ExtKeyUsageServerAuth"><code><strong class="tok-constructor">ExtKeyUsageServerAuth</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li id="k-ExtKeyUsageClientAuth"><code><strong class="tok-constructor">ExtKeyUsageClientAuth</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li id="k-ExtKeyUsageCodeSigning"><code><strong class="tok-constructor">ExtKeyUsageCodeSigning</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li id="k-ExtKeyUsageEmailProtection"><code><strong class="tok-constructor">ExtKeyUsageEmailProtection</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li id="k-ExtKeyUsageIpsecEndSystem"><code><strong class="tok-constructor">ExtKeyUsageIpsecEndSystem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li id="k-ExtKeyUsageIpsecTunnel"><code><strong class="tok-constructor">ExtKeyUsageIpsecTunnel</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li id="k-ExtKeyUsageIpsecUser"><code><strong class="tok-constructor">ExtKeyUsageIpsecUser</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li id="k-ExtKeyUsageTimeStamping"><code><strong class="tok-constructor">ExtKeyUsageTimeStamping</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li id="k-ExtKeyUsageOcspSigning"><code><strong class="tok-constructor">ExtKeyUsageOcspSigning</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li><li id="k-ExtKeyUsageOther"><code><strong class="tok-constructor">ExtKeyUsageOther</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PrivateKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PrivateKey</strong></code><span class="doc-sig-links"><a href="#t-PrivateKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L396" title="View source at line 396">Source</a></span></div>
<div class="doc-body">

A supported X.509 private key.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PrivRsa"><code><strong class="tok-constructor">PrivRsa</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a></code></li><li id="k-PrivP256"><code><strong class="tok-constructor">PrivP256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a></code></li><li id="k-PrivEd25519"><code><strong class="tok-constructor">PrivEd25519</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Ed25519/#t-Ed25519PrivateKey" title="Crypto.Ed25519.Ed25519PrivateKey">Ed25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a></code></li><li id="k-PrivX25519"><code><strong class="tok-constructor">PrivX25519</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X25519/#t-X25519PrivateKey" title="Crypto.X25519.X25519PrivateKey">X25519PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PublicKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PublicKey</strong></code><span class="doc-sig-links"><a href="#t-PublicKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L385" title="View source at line 385">Source</a></span></div>
<div class="doc-body">

A supported X.509 public key.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PubRsa"><code><strong class="tok-constructor">PubRsa</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code></li><li id="k-PubP256"><code><strong class="tok-constructor">PubP256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PublicKey" title="Crypto.P256.P256PublicKey">P256PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code></li><li id="k-PubEd25519"><code><strong class="tok-constructor">PubEd25519</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Ed25519PublicKey" title="Crypto.X509.Ed25519PublicKey">Ed25519PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code></li><li id="k-PubX25519"><code><strong class="tok-constructor">PubX25519</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X25519PublicKey" title="Crypto.X509.X25519PublicKey">X25519PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-SignatureAlgorithm">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">SignatureAlgorithm</strong></code><span class="doc-sig-links"><a href="#t-SignatureAlgorithm" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L706" title="View source at line 706">Source</a></span></div>
<div class="doc-body">

Supported X.509 signature algorithms.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SigUnknownAlgorithm"><code><strong class="tok-constructor">SigUnknownAlgorithm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigSha1WithRsa"><code><strong class="tok-constructor">SigSha1WithRsa</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigSha256WithRsa"><code><strong class="tok-constructor">SigSha256WithRsa</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigSha384WithRsa"><code><strong class="tok-constructor">SigSha384WithRsa</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigSha512WithRsa"><code><strong class="tok-constructor">SigSha512WithRsa</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigSha256WithRsaPss"><code><strong class="tok-constructor">SigSha256WithRsaPss</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigSha384WithRsaPss"><code><strong class="tok-constructor">SigSha384WithRsaPss</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigSha512WithRsaPss"><code><strong class="tok-constructor">SigSha512WithRsaPss</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigEcdsaWithSha256"><code><strong class="tok-constructor">SigEcdsaWithSha256</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigEcdsaWithSha384"><code><strong class="tok-constructor">SigEcdsaWithSha384</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigEcdsaWithSha512"><code><strong class="tok-constructor">SigEcdsaWithSha512</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li><li id="k-SigPureEd25519"><code><strong class="tok-constructor">SigPureEd25519</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-VerifyOptions">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">VerifyOptions</strong></code><span class="doc-sig-links"><a href="#t-VerifyOptions" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1924" title="View source at line 1924">Source</a></span></div>
<div class="doc-body">

Parameters controlling X.509 certificate chain verification.

</div>
</article>

<article class="doc-entry" id="t-X25519PublicKey">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">X25519PublicKey</strong></code><span class="doc-sig-links"><a href="#t-X25519PublicKey" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L370" title="View source at line 370">Source</a></span></div>
<div class="doc-body">

A validated 32-byte X25519 public key.

</div>
</article>

<article class="doc-entry" id="t-X509Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">X509Error</strong></code><span class="doc-sig-links"><a href="#t-X509Error" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L64" title="View source at line 64">Source</a></span></div>
<div class="doc-body">

Structured error values returned by X.509 parsing, signing, and verification operations.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-X509Asn1Error"><code><strong class="tok-constructor">X509Asn1Error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1Error" title="Text.Asn1.Asn1Error">Asn1Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509PkixError"><code><strong class="tok-constructor">X509PkixError</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509InvalidEncoding"><code><strong class="tok-constructor">X509InvalidEncoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509InvalidKey"><code><strong class="tok-constructor">X509InvalidKey</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509UnsupportedAlgorithm"><code><strong class="tok-constructor">X509UnsupportedAlgorithm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509SignatureAlgorithmMismatch"><code><strong class="tok-constructor">X509SignatureAlgorithmMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509SignatureInvalid"><code><strong class="tok-constructor">X509SignatureInvalid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509Expired"><code><strong class="tok-constructor">X509Expired</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509HostnameMismatch"><code><strong class="tok-constructor">X509HostnameMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509UnknownAuthority"><code><strong class="tok-constructor">X509UnknownAuthority</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509NotAuthorizedToSign"><code><strong class="tok-constructor">X509NotAuthorizedToSign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509TooManyIntermediates"><code><strong class="tok-constructor">X509TooManyIntermediates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509NameConstraintViolation"><code><strong class="tok-constructor">X509NameConstraintViolation</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509IncompatibleUsage"><code><strong class="tok-constructor">X509IncompatibleUsage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509UnhandledCriticalExtension"><code><strong class="tok-constructor">X509UnhandledCriticalExtension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li><li id="k-X509InvalidExtension"><code><strong class="tok-constructor">X509InvalidExtension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-cert-authority-key-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-authority-key-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-cert-authority-key-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1158" title="View source at line 1158">Source</a></span></div>
<div class="doc-body">

Return the Authority Key Identifier keyIdentifier bytes, if present.

</div>
</article>

<article class="doc-entry" id="v-cert-basic-constraints-valid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-basic-constraints-valid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-cert-basic-constraints-valid" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1114" title="View source at line 1114">Source</a></span></div>
<div class="doc-body">

Return True iff the BasicConstraints extension was present in the certificate.

</div>
</article>

<article class="doc-entry" id="v-cert-dns-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-dns-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cert-dns-names" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1137" title="View source at line 1137">Source</a></span></div>
<div class="doc-body">

Return the DNS names from the Subject Alternative Name extension.

</div>
</article>

<article class="doc-entry" id="v-cert-email-addresses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-email-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cert-email-addresses" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1141" title="View source at line 1141">Source</a></span></div>
<div class="doc-body">

Return the email addresses from the Subject Alternative Name extension.

</div>
</article>

<article class="doc-entry" id="v-cert-excluded-dns-domains">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-excluded-dns-domains</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cert-excluded-dns-domains" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1168" title="View source at line 1168">Source</a></span></div>
<div class="doc-body">

Return the excluded DNS domain constraints from the NameConstraints extension.

</div>
</article>

<article class="doc-entry" id="v-cert-excluded-ip-ranges">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-excluded-ip-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code><span class="doc-sig-links"><a href="#v-cert-excluded-ip-ranges" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1178" title="View source at line 1178">Source</a></span></div>
<div class="doc-body">

Return the excluded IP range constraints from the NameConstraints extension.

</div>
</article>

<article class="doc-entry" id="v-cert-ext-key-usage">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-ext-key-usage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code><span class="doc-sig-links"><a href="#v-cert-ext-key-usage" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1133" title="View source at line 1133">Source</a></span></div>
<div class="doc-body">

Return the list of Extended Key Usages.

</div>
</article>

<article class="doc-entry" id="v-cert-extensions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code><span class="doc-sig-links"><a href="#v-cert-extensions" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1183" title="View source at line 1183">Source</a></span></div>
<div class="doc-body">

Return all raw X.509 v3 extensions present on the certificate.

</div>
</article>

<article class="doc-entry" id="v-cert-ip-addresses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-ip-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-cert-ip-addresses" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1145" title="View source at line 1145">Source</a></span></div>
<div class="doc-body">

Return the IP addresses from the Subject Alternative Name extension.

</div>
</article>

<article class="doc-entry" id="v-cert-is-ca">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-is-ca</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-cert-is-ca" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1119" title="View source at line 1119">Source</a></span></div>
<div class="doc-body">

Return True iff BasicConstraints asserts cA = TRUE.

</div>
</article>

<article class="doc-entry" id="v-cert-issuer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-issuer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-cert-issuer" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1086" title="View source at line 1086">Source</a></span></div>
<div class="doc-body">

Return the certificate issuer distinguished name.

</div>
</article>

<article class="doc-entry" id="v-cert-key-usage">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-key-usage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-cert-key-usage" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1129" title="View source at line 1129">Source</a></span></div>
<div class="doc-body">

Return the KeyUsage bitmask (0 if extension is absent).

</div>
</article>

<article class="doc-entry" id="v-cert-max-path-len">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-max-path-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-cert-max-path-len" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1124" title="View source at line 1124">Source</a></span></div>
<div class="doc-body">

Return the optional BasicConstraints pathLenConstraint (`None` if unconstrained).

</div>
</article>

<article class="doc-entry" id="v-cert-not-after">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-not-after</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-cert-not-after" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1098" title="View source at line 1098">Source</a></span></div>
<div class="doc-body">

Return the certificate validity end timestamp in Unix seconds.

</div>
</article>

<article class="doc-entry" id="v-cert-not-before">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-not-before</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-cert-not-before" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1094" title="View source at line 1094">Source</a></span></div>
<div class="doc-body">

Return the certificate validity start timestamp in Unix seconds.

</div>
</article>

<article class="doc-entry" id="v-cert-permitted-dns-domains">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-permitted-dns-domains</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cert-permitted-dns-domains" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1163" title="View source at line 1163">Source</a></span></div>
<div class="doc-body">

Return the permitted DNS domain constraints from the NameConstraints extension.

</div>
</article>

<article class="doc-entry" id="v-cert-permitted-ip-ranges">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-permitted-ip-ranges</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a></code><span class="doc-sig-links"><a href="#v-cert-permitted-ip-ranges" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1173" title="View source at line 1173">Source</a></span></div>
<div class="doc-body">

Return the permitted IP range constraints from the NameConstraints extension.

</div>
</article>

<article class="doc-entry" id="v-cert-pool-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-pool-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a></code><span class="doc-sig-links"><a href="#v-cert-pool-add" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1895" title="View source at line 1895">Source</a></span></div>
<div class="doc-body">

Return a new `CertPool` with `cert` added.

</div>
</article>

<article class="doc-entry" id="v-cert-pool-append-pem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-pool-append-pem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-cert-pool-append-pem" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1901" title="View source at line 1901">Source</a></span></div>
<div class="doc-body">

Parse all valid `"CERTIFICATE"` PEM blocks and add them to `pool`.
Returns `(updated-pool, added-any)`, skipping invalid or non-certificate PEM blocks.

</div>
</article>

<article class="doc-entry" id="v-cert-pool-certs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-pool-certs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a></code><span class="doc-sig-links"><a href="#v-cert-pool-certs" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1916" title="View source at line 1916">Source</a></span></div>
<div class="doc-body">

Return all certificates stored in a `CertPool`.

</div>
</article>

<article class="doc-entry" id="v-cert-pool-len">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-pool-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-cert-pool-len" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1920" title="View source at line 1920">Source</a></span></div>
<div class="doc-body">

Return the number of certificates in a `CertPool`.

</div>
</article>

<article class="doc-entry" id="v-cert-pool-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-pool-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a></code><span class="doc-sig-links"><a href="#v-cert-pool-new" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1891" title="View source at line 1891">Source</a></span></div>
<div class="doc-body">

Create an empty `CertPool`.

</div>
</article>

<article class="doc-entry" id="v-cert-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code><span class="doc-sig-links"><a href="#v-cert-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1102" title="View source at line 1102">Source</a></span></div>
<div class="doc-body">

Return the certificate subject public key.

</div>
</article>

<article class="doc-entry" id="v-cert-raw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-cert-raw" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1187" title="View source at line 1187">Source</a></span></div>
<div class="doc-body">

Return the complete DER encoding of the certificate.

</div>
</article>

<article class="doc-entry" id="v-cert-raw-tbs-certificate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-raw-tbs-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-cert-raw-tbs-certificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1106" title="View source at line 1106">Source</a></span></div>
<div class="doc-body">

Return the exact raw DER bytes of the TBSCertificate signed by the issuer.

</div>
</article>

<article class="doc-entry" id="v-cert-serial-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-serial-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-cert-serial-number" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1078" title="View source at line 1078">Source</a></span></div>
<div class="doc-body">

Return the certificate serial number.

</div>
</article>

<article class="doc-entry" id="v-cert-signature">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-cert-signature" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1110" title="View source at line 1110">Source</a></span></div>
<div class="doc-body">

Return the raw signature bytes.

</div>
</article>

<article class="doc-entry" id="v-cert-signature-algorithm">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-signature-algorithm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code><span class="doc-sig-links"><a href="#v-cert-signature-algorithm" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1082" title="View source at line 1082">Source</a></span></div>
<div class="doc-body">

Return the certificate signature algorithm.

</div>
</article>

<article class="doc-entry" id="v-cert-subject">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-subject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-cert-subject" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1090" title="View source at line 1090">Source</a></span></div>
<div class="doc-body">

Return the certificate subject distinguished name.

</div>
</article>

<article class="doc-entry" id="v-cert-subject-key-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-subject-key-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-cert-subject-key-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1153" title="View source at line 1153">Source</a></span></div>
<div class="doc-body">

Return the Subject Key Identifier bytes, if present.

</div>
</article>

<article class="doc-entry" id="v-cert-template-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-template-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-cert-template-new" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2156" title="View source at line 2156">Source</a></span></div>
<div class="doc-body">

Construct a minimal `CertificateTemplate` with serial, subject, notBefore, and notAfter.

</div>
</article>

<article class="doc-entry" id="v-cert-uris">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-uris</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-cert-uris" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1149" title="View source at line 1149">Source</a></span></div>
<div class="doc-body">

Return the URIs from the Subject Alternative Name extension.

</div>
</article>

<article class="doc-entry" id="v-cert-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cert-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-cert-version" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1074" title="View source at line 1074">Source</a></span></div>
<div class="doc-body">

Return the X.509 certificate version (1, 2, or 3).

</div>
</article>

<article class="doc-entry" id="v-check-certificate-request-signature">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">check-certificate-request-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-check-certificate-request-signature" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2512" title="View source at line 2512">Source</a></span></div>
<div class="doc-body">

Verify the self-signature on a PKCS#10 `CertificateRequest`.

</div>
</article>

<article class="doc-entry" id="v-check-crl-signature-from">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">check-crl-signature-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-check-crl-signature-from" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1708" title="View source at line 1708">Source</a></span></div>
<div class="doc-body">

Verify that `crl` carries a valid signature created by `issuer`.
Checks KeyUsageCRLSign when KeyUsage is present on `issuer`, then verifies the signature.

</div>
</article>

<article class="doc-entry" id="v-check-signature">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">check-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-check-signature" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1629" title="View source at line 1629">Source</a></span></div>
<div class="doc-body">

Verify `signature` over raw `signed` bytes using `pub` and `alg`.

</div>
</article>

<article class="doc-entry" id="v-check-signature-from">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">check-signature-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-check-signature-from" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1698" title="View source at line 1698">Source</a></span></div>
<div class="doc-body">

Verify that `cert` carries a valid signature created by `parent`.
Checks parent CA authority (BasicConstraints and KeyUsageCertSign when present) and signature.

</div>
</article>

<article class="doc-entry" id="v-create-certificate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">create-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-create-certificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2348" title="View source at line 2348">Source</a></span></div>
<div class="doc-body">

Create and sign an X.509 v3 certificate in DER format.
Pass `None` as `parent` for a self-signed certificate, or `Some ca` for a CA-signed certificate.

</div>
</article>

<article class="doc-entry" id="v-create-certificate-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">create-certificate-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-create-certificate-request" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2556" title="View source at line 2556">Source</a></span></div>
<div class="doc-body">

Create and sign a PKCS#10 Certificate Signing Request in DER format.

</div>
</article>

<article class="doc-entry" id="v-create-revocation-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">create-revocation-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-create-revocation-list" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2597" title="View source at line 2597">Source</a></span></div>
<div class="doc-body">

Create and sign an X.509 v2 Certificate Revocation List in DER format.

</div>
</article>

<article class="doc-entry" id="v-csr-dns-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-dns-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-csr-dns-names" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2420" title="View source at line 2420">Source</a></span></div>
<div class="doc-body">

Return the requested SAN DNS names from the CSR.

</div>
</article>

<article class="doc-entry" id="v-csr-email-addresses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-email-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-csr-email-addresses" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2424" title="View source at line 2424">Source</a></span></div>
<div class="doc-body">

Return the requested SAN email addresses from the CSR.

</div>
</article>

<article class="doc-entry" id="v-csr-extensions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code><span class="doc-sig-links"><a href="#v-csr-extensions" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2436" title="View source at line 2436">Source</a></span></div>
<div class="doc-body">

Return all requested X.509 extensions attached to the CSR.

</div>
</article>

<article class="doc-entry" id="v-csr-ip-addresses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-ip-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a></code><span class="doc-sig-links"><a href="#v-csr-ip-addresses" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2428" title="View source at line 2428">Source</a></span></div>
<div class="doc-body">

Return the requested SAN IP addresses from the CSR.

</div>
</article>

<article class="doc-entry" id="v-csr-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code><span class="doc-sig-links"><a href="#v-csr-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2400" title="View source at line 2400">Source</a></span></div>
<div class="doc-body">

Return the requested subject public key.

</div>
</article>

<article class="doc-entry" id="v-csr-raw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-csr-raw" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2416" title="View source at line 2416">Source</a></span></div>
<div class="doc-body">

Return the complete DER encoding of the CSR.

</div>
</article>

<article class="doc-entry" id="v-csr-raw-tbs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-raw-tbs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-csr-raw-tbs" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2412" title="View source at line 2412">Source</a></span></div>
<div class="doc-body">

Return the exact raw DER bytes of CertificationRequestInfo.

</div>
</article>

<article class="doc-entry" id="v-csr-signature">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-csr-signature" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2408" title="View source at line 2408">Source</a></span></div>
<div class="doc-body">

Return the raw CSR signature bytes.

</div>
</article>

<article class="doc-entry" id="v-csr-signature-algorithm">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-signature-algorithm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code><span class="doc-sig-links"><a href="#v-csr-signature-algorithm" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2404" title="View source at line 2404">Source</a></span></div>
<div class="doc-body">

Return the CSR signature algorithm.

</div>
</article>

<article class="doc-entry" id="v-csr-subject">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-subject</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-csr-subject" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2396" title="View source at line 2396">Source</a></span></div>
<div class="doc-body">

Return the requested subject distinguished name.

</div>
</article>

<article class="doc-entry" id="v-csr-template-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-template-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code><span class="doc-sig-links"><a href="#v-csr-template-new" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2522" title="View source at line 2522">Source</a></span></div>
<div class="doc-body">

Create a minimal `CsrTemplate` for the given subject distinguished name.

</div>
</article>

<article class="doc-entry" id="v-csr-template-with-dns-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-template-with-dns-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code><span class="doc-sig-links"><a href="#v-csr-template-with-dns-names" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2531" title="View source at line 2531">Source</a></span></div>
<div class="doc-body">

Set requested DNS names on a `CsrTemplate`.

</div>
</article>

<article class="doc-entry" id="v-csr-template-with-email-addresses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-template-with-email-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code><span class="doc-sig-links"><a href="#v-csr-template-with-email-addresses" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2536" title="View source at line 2536">Source</a></span></div>
<div class="doc-body">

Set requested email addresses on a `CsrTemplate`.

</div>
</article>

<article class="doc-entry" id="v-csr-template-with-extensions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-template-with-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code><span class="doc-sig-links"><a href="#v-csr-template-with-extensions" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2551" title="View source at line 2551">Source</a></span></div>
<div class="doc-body">

Attach extra requested extensions to a `CsrTemplate`.

</div>
</article>

<article class="doc-entry" id="v-csr-template-with-ip-addresses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-template-with-ip-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code><span class="doc-sig-links"><a href="#v-csr-template-with-ip-addresses" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2541" title="View source at line 2541">Source</a></span></div>
<div class="doc-body">

Set requested IP addresses on a `CsrTemplate`.

</div>
</article>

<article class="doc-entry" id="v-csr-template-with-sig-alg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-template-with-sig-alg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code><span class="doc-sig-links"><a href="#v-csr-template-with-sig-alg" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2526" title="View source at line 2526">Source</a></span></div>
<div class="doc-body">

Override the signature algorithm on a `CsrTemplate`.

</div>
</article>

<article class="doc-entry" id="v-csr-template-with-uris">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-template-with-uris</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CsrTemplate" title="Crypto.X509.CsrTemplate">CsrTemplate</a></code><span class="doc-sig-links"><a href="#v-csr-template-with-uris" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2546" title="View source at line 2546">Source</a></span></div>
<div class="doc-body">

Set requested URIs on a `CsrTemplate`.

</div>
</article>

<article class="doc-entry" id="v-csr-uris">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-uris</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-csr-uris" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2432" title="View source at line 2432">Source</a></span></div>
<div class="doc-body">

Return the requested SAN URIs from the CSR.

</div>
</article>

<article class="doc-entry" id="v-csr-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">csr-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-csr-version" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2392" title="View source at line 2392">Source</a></span></div>
<div class="doc-body">

Return the CSR version (always 0 for PKCS#10).

</div>
</article>

<article class="doc-entry" id="v-ed25519-public-key-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-public-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Ed25519PublicKey" title="Crypto.X509.Ed25519PublicKey">Ed25519PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ed25519-public-key-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L366" title="View source at line 366">Source</a></span></div>
<div class="doc-body">

Return an independent copy of an `Ed25519PublicKey`'s 32 bytes.

</div>
</article>

<article class="doc-entry" id="v-ed25519-public-key-from-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ed25519-public-key-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Ed25519PublicKey" title="Crypto.X509.Ed25519PublicKey">Ed25519PublicKey</a></code><span class="doc-sig-links"><a href="#v-ed25519-public-key-from-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L360" title="View source at line 360">Source</a></span></div>
<div class="doc-body">

Validate and construct an `Ed25519PublicKey` from 32 raw bytes.

</div>
</article>

<article class="doc-entry" id="v-eku-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">eku-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-eku-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1033" title="View source at line 1033">Source</a></span></div>
<div class="doc-body">

Return True iff two `ExtKeyUsage` values represent the same OID.

</div>
</article>

<article class="doc-entry" id="v-eku-from-oid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">eku-from-oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a></code><span class="doc-sig-links"><a href="#v-eku-from-oid" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1018" title="View source at line 1018">Source</a></span></div>
<div class="doc-body">

Map an OID arc list to an `ExtKeyUsage` value.

</div>
</article>

<article class="doc-entry" id="v-eku-to-oid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">eku-to-oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-eku-to-oid" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1003" title="View source at line 1003">Source</a></span></div>
<div class="doc-body">

Convert an `ExtKeyUsage` to its OID arc list.

</div>
</article>

<article class="doc-entry" id="v-key-usage-cert-sign">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-cert-sign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-cert-sign" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L905" title="View source at line 905">Source</a></span></div>
<div class="doc-body">

KeyUsage bit 5: keyCertSign.

</div>
</article>

<article class="doc-entry" id="v-key-usage-content-commitment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-content-commitment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-content-commitment" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L889" title="View source at line 889">Source</a></span></div>
<div class="doc-body">

KeyUsage bit 1: contentCommitment (nonRepudiation).

</div>
</article>

<article class="doc-entry" id="v-key-usage-crl-sign">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-crl-sign</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-crl-sign" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L909" title="View source at line 909">Source</a></span></div>
<div class="doc-body">

KeyUsage bit 6: cRLSign.

</div>
</article>

<article class="doc-entry" id="v-key-usage-data-encipherment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-data-encipherment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-data-encipherment" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L897" title="View source at line 897">Source</a></span></div>
<div class="doc-body">

KeyUsage bit 3: dataEncipherment.

</div>
</article>

<article class="doc-entry" id="v-key-usage-decipher-only">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-decipher-only</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-decipher-only" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L917" title="View source at line 917">Source</a></span></div>
<div class="doc-body">

KeyUsage bit 8: decipherOnly.

</div>
</article>

<article class="doc-entry" id="v-key-usage-digital-signature">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-digital-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-digital-signature" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L885" title="View source at line 885">Source</a></span></div>
<div class="doc-body">

KeyUsage bit 0: digitalSignature.

</div>
</article>

<article class="doc-entry" id="v-key-usage-encipher-only">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-encipher-only</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-encipher-only" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L913" title="View source at line 913">Source</a></span></div>
<div class="doc-body">

KeyUsage bit 7: encipherOnly.

</div>
</article>

<article class="doc-entry" id="v-key-usage-has">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-has</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-key-usage-has" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L921" title="View source at line 921">Source</a></span></div>
<div class="doc-body">

Return True iff all flags in `mask` are set in `usage`.

</div>
</article>

<article class="doc-entry" id="v-key-usage-key-agreement">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-key-agreement</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-key-agreement" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L901" title="View source at line 901">Source</a></span></div>
<div class="doc-body">

KeyUsage bit 4: keyAgreement.

</div>
</article>

<article class="doc-entry" id="v-key-usage-key-encipherment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-key-encipherment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-key-encipherment" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L893" title="View source at line 893">Source</a></span></div>
<div class="doc-body">

KeyUsage bit 2: keyEncipherment.

</div>
</article>

<article class="doc-entry" id="v-key-usage-none">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-none</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-none" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L881" title="View source at line 881">Source</a></span></div>
<div class="doc-body">

Empty KeyUsage bitmask.

</div>
</article>

<article class="doc-entry" id="v-key-usage-union">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">key-usage-union</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-key-usage-union" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L925" title="View source at line 925">Source</a></span></div>
<div class="doc-body">

Combine two KeyUsage bitmasks.

</div>
</article>

<article class="doc-entry" id="v-marshal-ec-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">marshal-ec-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-marshal-ec-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L544" title="View source at line 544">Source</a></span></div>
<div class="doc-body">

Marshal a NIST P-256 private key to standalone SEC 1 ASN.1 DER (including curve OID and public key).

</div>
</article>

<article class="doc-entry" id="v-marshal-pkcs1-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">marshal-pkcs1-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-marshal-pkcs1-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L435" title="View source at line 435">Source</a></span></div>
<div class="doc-body">

Marshal an RSA private key to PKCS#1 ASN.1 DER form (delegating to `Crypto.Rsa`).

</div>
</article>

<article class="doc-entry" id="v-marshal-pkcs1-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">marshal-pkcs1-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-marshal-pkcs1-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L425" title="View source at line 425">Source</a></span></div>
<div class="doc-body">

Marshal an RSA public key to PKCS#1 ASN.1 DER form (delegating to `Crypto.Rsa`).

</div>
</article>

<article class="doc-entry" id="v-marshal-pkcs8-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">marshal-pkcs8-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-marshal-pkcs8-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L617" title="View source at line 617">Source</a></span></div>
<div class="doc-body">

Marshal a `PrivateKey` to PKCS#8 PrivateKeyInfo DER bytes (RFC 5208 / RFC 5958).

</div>
</article>

<article class="doc-entry" id="v-marshal-pkix-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">marshal-pkix-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-marshal-pkix-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L467" title="View source at line 467">Source</a></span></div>
<div class="doc-body">

Marshal a `PublicKey` to PKIX SubjectPublicKeyInfo DER bytes.

</div>
</article>

<article class="doc-entry" id="v-parse-certificate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a></code><span class="doc-sig-links"><a href="#v-parse-certificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1491" title="View source at line 1491">Source</a></span></div>
<div class="doc-body">

Parse a single DER-encoded X.509 certificate.

</div>
</article>

<article class="doc-entry" id="v-parse-certificate-request">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-certificate-request</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateRequest" title="Crypto.X509.CertificateRequest">CertificateRequest</a></code><span class="doc-sig-links"><a href="#v-parse-certificate-request" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2440" title="View source at line 2440">Source</a></span></div>
<div class="doc-body">

Parse a DER-encoded PKCS#10 Certificate Signing Request.

</div>
</article>

<article class="doc-entry" id="v-parse-certificates">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-certificates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a>)</code><span class="doc-sig-links"><a href="#v-parse-certificates" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1593" title="View source at line 1593">Source</a></span></div>
<div class="doc-body">

Parse one or more concatenated DER-encoded X.509 certificates.

</div>
</article>

<article class="doc-entry" id="v-parse-certificates-pem">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-certificates-pem</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a>)</code><span class="doc-sig-links"><a href="#v-parse-certificates-pem" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1610" title="View source at line 1610">Source</a></span></div>
<div class="doc-body">

Parse all `"CERTIFICATE"` PEM blocks from a PEM byte stream.
Non-certificate PEM blocks are skipped; malformed certificate DER fails with an error.

</div>
</article>

<article class="doc-entry" id="v-parse-ec-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-ec-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.P256/#t-P256PrivateKey" title="Crypto.P256.P256PrivateKey">P256PrivateKey</a></code><span class="doc-sig-links"><a href="#v-parse-ec-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L548" title="View source at line 548">Source</a></span></div>
<div class="doc-body">

Parse a SEC 1 ECPrivateKey DER buffer into a NIST P-256 private key.

</div>
</article>

<article class="doc-entry" id="v-parse-pkcs1-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-pkcs1-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPrivateKey" title="Crypto.Rsa.RsaPrivateKey">RsaPrivateKey</a></code><span class="doc-sig-links"><a href="#v-parse-pkcs1-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L429" title="View source at line 429">Source</a></span></div>
<div class="doc-body">

Parse an RSA private key in PKCS#1 ASN.1 DER form (delegating to `Crypto.Rsa`).

</div>
</article>

<article class="doc-entry" id="v-parse-pkcs1-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-pkcs1-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Rsa/#t-RsaPublicKey" title="Crypto.Rsa.RsaPublicKey">RsaPublicKey</a></code><span class="doc-sig-links"><a href="#v-parse-pkcs1-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L419" title="View source at line 419">Source</a></span></div>
<div class="doc-body">

Parse an RSA public key in PKCS#1 ASN.1 DER form (delegating to `Crypto.Rsa`).

</div>
</article>

<article class="doc-entry" id="v-parse-pkcs8-private-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-pkcs8-private-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a></code><span class="doc-sig-links"><a href="#v-parse-pkcs8-private-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L645" title="View source at line 645">Source</a></span></div>
<div class="doc-body">

Parse a PKCS#8 PrivateKeyInfo DER buffer into a `PrivateKey`.

</div>
</article>

<article class="doc-entry" id="v-parse-pkix-public-key">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-pkix-public-key</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code><span class="doc-sig-links"><a href="#v-parse-pkix-public-key" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L518" title="View source at line 518">Source</a></span></div>
<div class="doc-body">

Parse a PKIX SubjectPublicKeyInfo DER buffer into a `PublicKey`.

</div>
</article>

<article class="doc-entry" id="v-public-key-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">public-key-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-public-key-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L415" title="View source at line 415">Source</a></span></div>
<div class="doc-body">

Return True iff two `PublicKey` values are of the same type and equal.

</div>
</article>

<article class="doc-entry" id="v-public-key-of">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">public-key-of</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PrivateKey" title="Crypto.X509.PrivateKey">PrivateKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-PublicKey" title="Crypto.X509.PublicKey">PublicKey</a></code><span class="doc-sig-links"><a href="#v-public-key-of" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L407" title="View source at line 407">Source</a></span></div>
<div class="doc-body">

Derive the corresponding `PublicKey` from a `PrivateKey`.

</div>
</article>

<article class="doc-entry" id="v-sig-alg-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sig-alg-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-sig-alg-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L733" title="View source at line 733">Source</a></span></div>
<div class="doc-body">

Return True iff two `SignatureAlgorithm` values are equal.

</div>
</article>

<article class="doc-entry" id="v-sig-alg-from-pkix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sig-alg-from-pkix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a></code><span class="doc-sig-links"><a href="#v-sig-alg-from-pkix" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L780" title="View source at line 780">Source</a></span></div>
<div class="doc-body">

Recognize a `SignatureAlgorithm` from a `PkixAlgorithmIdentifier`.

</div>
</article>

<article class="doc-entry" id="v-sig-alg-to-pkix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sig-alg-to-pkix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code><span class="doc-sig-links"><a href="#v-sig-alg-to-pkix" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L764" title="View source at line 764">Source</a></span></div>
<div class="doc-body">

Convert a `SignatureAlgorithm` to its PKIX `PkixAlgorithmIdentifier`.

</div>
</article>

<article class="doc-entry" id="v-template-with-authority-key-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-authority-key-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-authority-key-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2221" title="View source at line 2221">Source</a></span></div>
<div class="doc-body">

Set Authority Key Identifier bytes on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-ca">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-ca</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-ca" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2167" title="View source at line 2167">Source</a></span></div>
<div class="doc-body">

Set BasicConstraints `cA` flag (and mark BasicConstraints valid) on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-dns-names">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-dns-names</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-dns-names" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2191" title="View source at line 2191">Source</a></span></div>
<div class="doc-body">

Set SAN DNS names on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-email-addresses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-email-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-email-addresses" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2197" title="View source at line 2197">Source</a></span></div>
<div class="doc-body">

Set SAN email addresses on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-excluded-dns">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-excluded-dns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-excluded-dns" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2233" title="View source at line 2233">Source</a></span></div>
<div class="doc-body">

Set excluded DNS domain NameConstraints on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-excluded-ips">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-excluded-ips</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-excluded-ips" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2245" title="View source at line 2245">Source</a></span></div>
<div class="doc-body">

Set excluded IP range NameConstraints on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-ext-key-usage">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-ext-key-usage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-ext-key-usage" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2185" title="View source at line 2185">Source</a></span></div>
<div class="doc-body">

Set the Extended Key Usage list on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-extra-extensions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-extra-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-extra-extensions" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2251" title="View source at line 2251">Source</a></span></div>
<div class="doc-body">

Attach additional raw `PkixExtension` entries to a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-ip-addresses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-ip-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Addr" title="Data.NetIp.Addr">Addr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-ip-addresses" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2203" title="View source at line 2203">Source</a></span></div>
<div class="doc-body">

Set SAN IP addresses on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-key-usage">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-key-usage</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-key-usage" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2179" title="View source at line 2179">Source</a></span></div>
<div class="doc-body">

Set the KeyUsage bitmask on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-max-path-len">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-max-path-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-max-path-len" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2173" title="View source at line 2173">Source</a></span></div>
<div class="doc-body">

Set BasicConstraints `pathLenConstraint` on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-permitted-dns">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-permitted-dns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-permitted-dns" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2227" title="View source at line 2227">Source</a></span></div>
<div class="doc-body">

Set permitted DNS domain NameConstraints on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-permitted-ips">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-permitted-ips</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.NetIp/#t-Prefix" title="Data.NetIp.Prefix">Prefix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-permitted-ips" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2239" title="View source at line 2239">Source</a></span></div>
<div class="doc-body">

Set permitted IP range NameConstraints on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-sig-alg">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-sig-alg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-SignatureAlgorithm" title="Crypto.X509.SignatureAlgorithm">SignatureAlgorithm</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-sig-alg" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2162" title="View source at line 2162">Source</a></span></div>
<div class="doc-body">

Override the signature algorithm on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-subject-key-id">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-subject-key-id</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-subject-key-id" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2215" title="View source at line 2215">Source</a></span></div>
<div class="doc-body">

Set Subject Key Identifier bytes on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-template-with-uris">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">template-with-uris</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertificateTemplate" title="Crypto.X509.CertificateTemplate">CertificateTemplate</a></code><span class="doc-sig-links"><a href="#v-template-with-uris" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2209" title="View source at line 2209">Source</a></span></div>
<div class="doc-body">

Set SAN URIs on a `CertificateTemplate`.

</div>
</article>

<article class="doc-entry" id="v-verify-certificate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">verify-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a>))</code><span class="doc-sig-links"><a href="#v-verify-certificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L2066" title="View source at line 2066">Source</a></span></div>
<div class="doc-body">

Verify `leaf` under `opts`, returning all valid certificate chains `[leaf, ..., root]`.

</div>
</article>

<article class="doc-entry" id="v-verify-hostname">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">verify-hostname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-Certificate" title="Crypto.X509.Certificate">Certificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-verify-hostname" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1725" title="View source at line 1725">Source</a></span></div>
<div class="doc-body">

Verify that `cert` is valid for `host` (either a DNS hostname or an IPv4/IPv6 literal).
Follows RFC 6125: matches only Subject Alternative Name entries (never CommonName),
performs case-insensitive ASCII DNS comparison with single-label leftmost wildcards,
and matches IP literals (with optional IPv6 brackets) via `Data.NetIp`.

</div>
</article>

<article class="doc-entry" id="v-verify-options-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">verify-options-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a></code><span class="doc-sig-links"><a href="#v-verify-options-new" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1929" title="View source at line 1929">Source</a></span></div>
<div class="doc-body">

Create default `VerifyOptions` with the given trusted root pool and Unix verification time.

</div>
</article>

<article class="doc-entry" id="v-verify-with-dns-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">verify-with-dns-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a></code><span class="doc-sig-links"><a href="#v-verify-with-dns-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1938" title="View source at line 1938">Source</a></span></div>
<div class="doc-body">

Set the hostname or IP address to verify against the leaf certificate.

</div>
</article>

<article class="doc-entry" id="v-verify-with-intermediates">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">verify-with-intermediates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-CertPool" title="Crypto.X509.CertPool">CertPool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a></code><span class="doc-sig-links"><a href="#v-verify-with-intermediates" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1933" title="View source at line 1933">Source</a></span></div>
<div class="doc-body">

Attach an intermediate certificate pool to `VerifyOptions`.

</div>
</article>

<article class="doc-entry" id="v-verify-with-key-usages">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">verify-with-key-usages</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-ExtKeyUsage" title="Crypto.X509.ExtKeyUsage">ExtKeyUsage</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-VerifyOptions" title="Crypto.X509.VerifyOptions">VerifyOptions</a></code><span class="doc-sig-links"><a href="#v-verify-with-key-usages" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L1943" title="View source at line 1943">Source</a></span></div>
<div class="doc-body">

Set the acceptable extended key usages (empty list defaults to `[ExtKeyUsageServerAuth]`).

</div>
</article>

<article class="doc-entry" id="v-x25519-public-key-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x25519-public-key-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X25519PublicKey" title="Crypto.X509.X25519PublicKey">X25519PublicKey</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-x25519-public-key-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L381" title="View source at line 381">Source</a></span></div>
<div class="doc-body">

Return an independent copy of an `X25519PublicKey`'s 32 bytes.

</div>
</article>

<article class="doc-entry" id="v-x25519-public-key-from-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x25519-public-key-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X25519PublicKey" title="Crypto.X509.X25519PublicKey">X25519PublicKey</a></code><span class="doc-sig-links"><a href="#v-x25519-public-key-from-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L375" title="View source at line 375">Source</a></span></div>
<div class="doc-body">

Validate and construct an `X25519PublicKey` from 32 raw bytes.

</div>
</article>

<article class="doc-entry" id="v-x509-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">x509-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.X509/#t-X509Error" title="Crypto.X509.X509Error">X509Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-x509-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.X509/source/#L99" title="View source at line 99">Source</a></span></div>
<div class="doc-body">

Render an `X509Error` as a human-readable diagnostic message.

</div>
</article>
