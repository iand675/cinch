# Crypto.Pkix

PKIX distinguished names, OIDs, RFC 4514 formatting, algorithm identifiers, X.509 extensions, times and certificate revocation lists (CRLs).

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Crypto.Pkix</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.Pkix/source/">View source</a><a href="/packages/search/?q=%2BCrypto.Pkix">Search module</a><a href="/reference-sources/Crypto.Pkix.cinch" download="Crypto.Pkix.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Asn1](Text.Asn1.md), [Text.Hex](Text.Hex.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Public Key Infrastructure (PKIX) data structures for X.509 certificates,
certificate signing requests and certificate revocation lists (RFC 5280, RFC 4514).

Provides ASN.1 DER codecs and structured accessors for distinguished names
(`PkixName`, `PkixRdn`, `PkixAttr`), algorithm identifiers (`PkixAlgorithmIdentifier`),
X.509 v3 extensions (`PkixExtension`), revoked certificate entries
(`PkixRevokedCertificate`) and certificate revocation lists (`PkixCrl`),
together with RFC 4514 string formatting, OID utilities, RFC 5280 time codecs
and exact DER TLV boundary helpers.

</details>

<details class="doc-synopsis"><summary>Synopsis (106 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixAlgorithmIdentifier</strong></code></li><li><a class="syn-name" href="#t-PkixAttr">PkixAttr</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixAttr</strong></code></li><li><a class="syn-name" href="#t-PkixCrl">PkixCrl</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixCrl</strong></code></li><li><a class="syn-name" href="#t-PkixError">PkixError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixError</strong></code></li><li><a class="syn-name" href="#t-PkixExtension">PkixExtension</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixExtension</strong></code></li><li><a class="syn-name" href="#t-PkixName">PkixName</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixName</strong></code></li><li><a class="syn-name" href="#t-PkixRdn">PkixRdn</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixRdn</strong></code></li><li><a class="syn-name" href="#t-PkixRevokedCertificate">PkixRevokedCertificate</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixRevokedCertificate</strong></code></li><li><a class="syn-name" href="#v-crl-extensions">crl-extensions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code></li><li><a class="syn-name" href="#v-crl-issuer">crl-issuer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-issuer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-crl-next-update">crl-next-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-next-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-crl-raw">crl-raw</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crl-raw-tbs">crl-raw-tbs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-raw-tbs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crl-revoked-certificates">crl-revoked-certificates</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-revoked-certificates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a></code></li><li><a class="syn-name" href="#v-crl-signature">crl-signature</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-crl-signature-algorithm">crl-signature-algorithm</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-signature-algorithm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code></li><li><a class="syn-name" href="#v-crl-tbs-signature-algorithm">crl-tbs-signature-algorithm</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-tbs-signature-algorithm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code></li><li><a class="syn-name" href="#v-crl-this-update">crl-this-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-this-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-crl-version">crl-version</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">crl-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-der-element-bounds">der-element-bounds</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">der-element-bounds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-der-first-child-bytes">der-first-child-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">der-first-child-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-der-split-sequence">der-split-sequence</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">der-split-sequence</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-marshal-crl">marshal-crl</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">marshal-crl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-marshal-tbs-crl">marshal-tbs-crl</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">marshal-tbs-crl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-oid-common-name">oid-common-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-common-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-oid-country">oid-country</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-country</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-oid-equal">oid-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-oid-from-string">oid-from-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-oid-locality">oid-locality</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-locality</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-oid-organization">oid-organization</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-organization</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-oid-organizational-unit">oid-organizational-unit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-organizational-unit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-oid-postal-code">oid-postal-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-postal-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-oid-province">oid-province</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-province</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-oid-serial-number">oid-serial-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-serial-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-oid-street-address">oid-street-address</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-street-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-oid-to-string">oid-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">oid-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-parse-crl">parse-crl</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">parse-crl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a></code></li><li><a class="syn-name" href="#v-pkix-alg-equal">pkix-alg-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-alg-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-pkix-alg-from-asn1">pkix-alg-from-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-alg-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code></li><li><a class="syn-name" href="#v-pkix-alg-oid">pkix-alg-oid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-alg-oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pkix-alg-params">pkix-alg-params</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-alg-params</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-pkix-alg-to-asn1">pkix-alg-to-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-alg-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-pkix-algorithm-identifier">pkix-algorithm-identifier</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-algorithm-identifier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code></li><li><a class="syn-name" href="#v-pkix-attr">pkix-attr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-attr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code></li><li><a class="syn-name" href="#v-pkix-attr-equal">pkix-attr-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-attr-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-pkix-attr-for-string">pkix-attr-for-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-attr-for-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code></li><li><a class="syn-name" href="#v-pkix-attr-from-asn1">pkix-attr-from-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-attr-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code></li><li><a class="syn-name" href="#v-pkix-attr-oid">pkix-attr-oid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-attr-oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pkix-attr-printable">pkix-attr-printable</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-attr-printable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code></li><li><a class="syn-name" href="#v-pkix-attr-string">pkix-attr-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-attr-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-attr-to-asn1">pkix-attr-to-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-attr-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-pkix-attr-utf8">pkix-attr-utf8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-attr-utf8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code></li><li><a class="syn-name" href="#v-pkix-attr-value">pkix-attr-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-attr-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-pkix-common-name">pkix-common-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-common-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-country">pkix-country</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-country</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-crl-from-parts">pkix-crl-from-parts</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-crl-from-parts</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a></code></li><li><a class="syn-name" href="#v-pkix-decode-time">pkix-decode-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-decode-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-pkix-encode-time">pkix-encode-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-encode-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-pkix-error-message">pkix-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-ext-critical">pkix-ext-critical</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-ext-critical</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-pkix-ext-from-asn1">pkix-ext-from-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-ext-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code></li><li><a class="syn-name" href="#v-pkix-ext-oid">pkix-ext-oid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-ext-oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-pkix-ext-to-asn1">pkix-ext-to-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-ext-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-pkix-ext-value">pkix-ext-value</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-ext-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-pkix-extension">pkix-extension</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-extension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code></li><li><a class="syn-name" href="#v-pkix-find-extension">pkix-find-extension</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-find-extension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code></li><li><a class="syn-name" href="#v-pkix-locality">pkix-locality</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-locality</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-marshal-name">pkix-marshal-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-marshal-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-pkix-name-add-attr">pkix-name-add-attr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-name-add-attr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-name-add-rdn">pkix-name-add-rdn</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-name-add-rdn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-name-empty">pkix-name-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-name-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-name-equal">pkix-name-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-name-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-pkix-name-from-asn1">pkix-name-from-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-name-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-name-from-rdns">pkix-name-from-rdns</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-name-from-rdns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-name-rdns">pkix-name-rdns</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-name-rdns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a></code></li><li><a class="syn-name" href="#v-pkix-name-to-asn1">pkix-name-to-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-name-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-pkix-name-to-canonical-string">pkix-name-to-canonical-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-name-to-canonical-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-name-to-string">pkix-name-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-name-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-organization">pkix-organization</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-organization</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-organizational-unit">pkix-organizational-unit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-organizational-unit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-parse-name">pkix-parse-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-parse-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-postal-code">pkix-postal-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-postal-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-province">pkix-province</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-province</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-rdn">pkix-rdn</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-rdn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a></code></li><li><a class="syn-name" href="#v-pkix-rdn-attrs">pkix-rdn-attrs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-rdn-attrs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code></li><li><a class="syn-name" href="#v-pkix-rdn-equal">pkix-rdn-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-rdn-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-pkix-rdn-from-asn1">pkix-rdn-from-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-rdn-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a></code></li><li><a class="syn-name" href="#v-pkix-rdn-to-asn1">pkix-rdn-to-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-rdn-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-pkix-revoked-certificate">pkix-revoked-certificate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a></code></li><li><a class="syn-name" href="#v-pkix-revoked-extensions">pkix-revoked-extensions</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code></li><li><a class="syn-name" href="#v-pkix-revoked-from-asn1">pkix-revoked-from-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a></code></li><li><a class="syn-name" href="#v-pkix-revoked-serial">pkix-revoked-serial</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-serial</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-pkix-revoked-time">pkix-revoked-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-pkix-revoked-to-asn1">pkix-revoked-to-asn1</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code></li><li><a class="syn-name" href="#v-pkix-serial-number">pkix-serial-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-serial-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-street-address">pkix-street-address</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-street-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pkix-with-common-name">pkix-with-common-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-with-common-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-with-country">pkix-with-country</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-with-country</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-with-extra-attr">pkix-with-extra-attr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-with-extra-attr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-with-locality">pkix-with-locality</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-with-locality</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-with-organization">pkix-with-organization</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-with-organization</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-with-organizational-unit">pkix-with-organizational-unit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-with-organizational-unit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-with-postal-code">pkix-with-postal-code</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-with-postal-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-with-province">pkix-with-province</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-with-province</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-with-serial-number">pkix-with-serial-number</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-with-serial-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li><li><a class="syn-name" href="#v-pkix-with-street-address">pkix-with-street-address</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pkix-with-street-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-PkixAlgorithmIdentifier">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixAlgorithmIdentifier</strong></code><span class="doc-sig-links"><a href="#t-PkixAlgorithmIdentifier" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L807" title="View source at line 807">Source</a></span></div>
<div class="doc-body">

Algorithm OID and optional ASN.1 parameters.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PkixAlgorithmIdentifier"><code><strong class="tok-constructor">PkixAlgorithmIdentifier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PkixAttr">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixAttr</strong></code><span class="doc-sig-links"><a href="#t-PkixAttr" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L359" title="View source at line 359">Source</a></span></div>
<div class="doc-body">

Attribute type OID and ASN.1 value.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PkixAttr"><code><strong class="tok-constructor">PkixAttr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PkixCrl">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixCrl</strong></code><span class="doc-sig-links"><a href="#t-PkixCrl" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L968" title="View source at line 968">Source</a></span></div>
<div class="doc-body">

A parsed or constructed X.509 Certificate Revocation List (RFC 5280 CertificateList).

</div>
</article>

<article class="doc-entry" id="t-PkixError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixError</strong></code><span class="doc-sig-links"><a href="#t-PkixError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

An error encountered while parsing or validating a PKIX structure.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PkixAsn1Error"><code><strong class="tok-constructor">PkixAsn1Error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1Error" title="Text.Asn1.Asn1Error">Asn1Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a></code></li><li id="k-PkixInvalidStructure"><code><strong class="tok-constructor">PkixInvalidStructure</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a></code></li><li id="k-PkixInvalidTime"><code><strong class="tok-constructor">PkixInvalidTime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a></code></li><li id="k-PkixInvalidOid"><code><strong class="tok-constructor">PkixInvalidOid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PkixExtension">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixExtension</strong></code><span class="doc-sig-links"><a href="#t-PkixExtension" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L848" title="View source at line 848">Source</a></span></div>
<div class="doc-body">

Extension OID, critical flag, and raw DER extnValue octets.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PkixExtension"><code><strong class="tok-constructor">PkixExtension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PkixName">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixName</strong></code><span class="doc-sig-links"><a href="#t-PkixName" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L520" title="View source at line 520">Source</a></span></div>
<div class="doc-body">

The RDNSequence preserving exact order and multi-valued RDNs.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PkixName"><code><strong class="tok-constructor">PkixName</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PkixRdn">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixRdn</strong></code><span class="doc-sig-links"><a href="#t-PkixRdn" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L474" title="View source at line 474">Source</a></span></div>
<div class="doc-body">

Attributes belonging to this RDN.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PkixRdn"><code><strong class="tok-constructor">PkixRdn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-PkixRevokedCertificate">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PkixRevokedCertificate</strong></code><span class="doc-sig-links"><a href="#t-PkixRevokedCertificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L911" title="View source at line 911">Source</a></span></div>
<div class="doc-body">

Revoked certificate serial number, revocation Unix timestamp, and entry extensions.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PkixRevokedCertificate"><code><strong class="tok-constructor">PkixRevokedCertificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-crl-extensions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code><span class="doc-sig-links"><a href="#v-crl-extensions" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1016" title="View source at line 1016">Source</a></span></div>
<div class="doc-body">

Return the CRL extensions.

</div>
</article>

<article class="doc-entry" id="v-crl-issuer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-issuer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-crl-issuer" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1000" title="View source at line 1000">Source</a></span></div>
<div class="doc-body">

Return the CRL issuer distinguished name.

</div>
</article>

<article class="doc-entry" id="v-crl-next-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-next-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-crl-next-update" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1008" title="View source at line 1008">Source</a></span></div>
<div class="doc-body">

Return the optional CRL nextUpdate timestamp in Unix seconds.

</div>
</article>

<article class="doc-entry" id="v-crl-raw">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-raw</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crl-raw" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1032" title="View source at line 1032">Source</a></span></div>
<div class="doc-body">

Return the complete DER encoding of the CRL.

</div>
</article>

<article class="doc-entry" id="v-crl-raw-tbs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-raw-tbs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crl-raw-tbs" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1020" title="View source at line 1020">Source</a></span></div>
<div class="doc-body">

Return the exact raw DER bytes of the TBSCertList signed by the issuer.

</div>
</article>

<article class="doc-entry" id="v-crl-revoked-certificates">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-revoked-certificates</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a></code><span class="doc-sig-links"><a href="#v-crl-revoked-certificates" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1012" title="View source at line 1012">Source</a></span></div>
<div class="doc-body">

Return the list of revoked certificate entries.

</div>
</article>

<article class="doc-entry" id="v-crl-signature">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-signature</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-crl-signature" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1028" title="View source at line 1028">Source</a></span></div>
<div class="doc-body">

Return the raw signature bytes.

</div>
</article>

<article class="doc-entry" id="v-crl-signature-algorithm">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-signature-algorithm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code><span class="doc-sig-links"><a href="#v-crl-signature-algorithm" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1024" title="View source at line 1024">Source</a></span></div>
<div class="doc-body">

Return the outer signature algorithm identifier.

</div>
</article>

<article class="doc-entry" id="v-crl-tbs-signature-algorithm">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-tbs-signature-algorithm</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code><span class="doc-sig-links"><a href="#v-crl-tbs-signature-algorithm" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L996" title="View source at line 996">Source</a></span></div>
<div class="doc-body">

Return the inner TBS signature algorithm identifier.

</div>
</article>

<article class="doc-entry" id="v-crl-this-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-this-update</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-crl-this-update" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1004" title="View source at line 1004">Source</a></span></div>
<div class="doc-body">

Return the CRL thisUpdate timestamp in Unix seconds.

</div>
</article>

<article class="doc-entry" id="v-crl-version">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">crl-version</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-crl-version" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L992" title="View source at line 992">Source</a></span></div>
<div class="doc-body">

Return the CRL version (1 for v1, 2 for v2).

</div>
</article>

<article class="doc-entry" id="v-der-element-bounds">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">der-element-bounds</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-der-element-bounds" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L212" title="View source at line 212">Source</a></span></div>
<div class="doc-body">

Locate the DER element starting at offset `start`: returns Some (start, content-start, end).

</div>
</article>

<article class="doc-entry" id="v-der-first-child-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">der-first-child-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-der-first-child-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L263" title="View source at line 263">Source</a></span></div>
<div class="doc-body">

Extract the exact raw DER bytes of the first child element of a top-level DER SEQUENCE.
Used to obtain exact wire TBS bytes for signature verification.

</div>
</article>

<article class="doc-entry" id="v-der-split-sequence">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">der-split-sequence</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-der-split-sequence" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L277" title="View source at line 277">Source</a></span></div>
<div class="doc-body">

Split a buffer of concatenated top-level DER elements into individual raw element slices.

</div>
</article>

<article class="doc-entry" id="v-marshal-crl">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">marshal-crl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-marshal-crl" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1046" title="View source at line 1046">Source</a></span></div>
<div class="doc-body">

Marshal a `PkixCrl` to DER bytes.

</div>
</article>

<article class="doc-entry" id="v-marshal-tbs-crl">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">marshal-tbs-crl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-marshal-tbs-crl" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1036" title="View source at line 1036">Source</a></span></div>
<div class="doc-body">

Marshal the TBSCertList portion of a CRL to DER bytes.

</div>
</article>

<article class="doc-entry" id="v-oid-common-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-common-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-oid-common-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

OID 2.5.4.3 (commonName / CN).

</div>
</article>

<article class="doc-entry" id="v-oid-country">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-country</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-oid-country" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

OID 2.5.4.6 (countryName / C).

</div>
</article>

<article class="doc-entry" id="v-oid-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-oid-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

Return True iff two OID arc lists are identical.

</div>
</article>

<article class="doc-entry" id="v-oid-from-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-from-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-oid-from-string" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Parse a dotted-decimal OID string (at least two valid arcs, first arc 0..2).

</div>
</article>

<article class="doc-entry" id="v-oid-locality">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-locality</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-oid-locality" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

OID 2.5.4.7 (localityName / L).

</div>
</article>

<article class="doc-entry" id="v-oid-organization">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-organization</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-oid-organization" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

OID 2.5.4.10 (organizationName / O).

</div>
</article>

<article class="doc-entry" id="v-oid-organizational-unit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-organizational-unit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-oid-organizational-unit" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L139" title="View source at line 139">Source</a></span></div>
<div class="doc-body">

OID 2.5.4.11 (organizationalUnitName / OU).

</div>
</article>

<article class="doc-entry" id="v-oid-postal-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-postal-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-oid-postal-code" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

OID 2.5.4.17 (postalCode / POSTALCODE).

</div>
</article>

<article class="doc-entry" id="v-oid-province">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-province</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-oid-province" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

OID 2.5.4.8 (stateOrProvinceName / ST).

</div>
</article>

<article class="doc-entry" id="v-oid-serial-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-serial-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-oid-serial-number" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

OID 2.5.4.5 (serialNumber / SERIALNUMBER).

</div>
</article>

<article class="doc-entry" id="v-oid-street-address">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-street-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-oid-street-address" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

OID 2.5.4.9 (streetAddress / STREET).

</div>
</article>

<article class="doc-entry" id="v-oid-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">oid-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-oid-to-string" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

Format an OID arc list in dotted-decimal notation (e.g. "2.5.4.3").

</div>
</article>

<article class="doc-entry" id="v-parse-crl">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">parse-crl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a></code><span class="doc-sig-links"><a href="#v-parse-crl" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L1050" title="View source at line 1050">Source</a></span></div>
<div class="doc-body">

Parse a DER-encoded X.509 Certificate Revocation List (RFC 5280 CertificateList).

</div>
</article>

<article class="doc-entry" id="v-pkix-alg-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-alg-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-pkix-alg-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L838" title="View source at line 838">Source</a></span></div>
<div class="doc-body">

Return True iff two `PkixAlgorithmIdentifier` values have identical DER encodings.

</div>
</article>

<article class="doc-entry" id="v-pkix-alg-from-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-alg-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code><span class="doc-sig-links"><a href="#v-pkix-alg-from-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L829" title="View source at line 829">Source</a></span></div>
<div class="doc-body">

Decode an ASN.1 AlgorithmIdentifier SEQUENCE.

</div>
</article>

<article class="doc-entry" id="v-pkix-alg-oid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-alg-oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pkix-alg-oid" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L814" title="View source at line 814">Source</a></span></div>
<div class="doc-body">

Return the OID of a `PkixAlgorithmIdentifier`.

</div>
</article>

<article class="doc-entry" id="v-pkix-alg-params">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-alg-params</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-pkix-alg-params" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L818" title="View source at line 818">Source</a></span></div>
<div class="doc-body">

Return the optional ASN.1 parameters of a `PkixAlgorithmIdentifier`.

</div>
</article>

<article class="doc-entry" id="v-pkix-alg-to-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-alg-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-pkix-alg-to-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L822" title="View source at line 822">Source</a></span></div>
<div class="doc-body">

Encode a `PkixAlgorithmIdentifier` as an ASN.1 SEQUENCE.

</div>
</article>

<article class="doc-entry" id="v-pkix-algorithm-identifier">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-algorithm-identifier</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a></code><span class="doc-sig-links"><a href="#v-pkix-algorithm-identifier" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L810" title="View source at line 810">Source</a></span></div>
<div class="doc-body">

Construct a `PkixAlgorithmIdentifier`.

</div>
</article>

<article class="doc-entry" id="v-pkix-attr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-attr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code><span class="doc-sig-links"><a href="#v-pkix-attr" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L362" title="View source at line 362">Source</a></span></div>
<div class="doc-body">

Construct a `PkixAttr` from an OID and ASN.1 value.

</div>
</article>

<article class="doc-entry" id="v-pkix-attr-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-attr-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-pkix-attr-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L462" title="View source at line 462">Source</a></span></div>
<div class="doc-body">

Return True iff two `PkixAttr` values have the same OID and identical DER-encoded value.

</div>
</article>

<article class="doc-entry" id="v-pkix-attr-for-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-attr-for-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code><span class="doc-sig-links"><a href="#v-pkix-attr-for-string" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L382" title="View source at line 382">Source</a></span></div>
<div class="doc-body">

Construct a string `PkixAttr` using PrintableString when all bytes are valid PrintableString characters, else UTF8String.

</div>
</article>

<article class="doc-entry" id="v-pkix-attr-from-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-attr-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code><span class="doc-sig-links"><a href="#v-pkix-attr-from-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L454" title="View source at line 454">Source</a></span></div>
<div class="doc-body">

Decode an ASN.1 AttributeTypeAndValue into a `PkixAttr`.

</div>
</article>

<article class="doc-entry" id="v-pkix-attr-oid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-attr-oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pkix-attr-oid" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L366" title="View source at line 366">Source</a></span></div>
<div class="doc-body">

Return the OID of a `PkixAttr`.

</div>
</article>

<article class="doc-entry" id="v-pkix-attr-printable">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-attr-printable</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code><span class="doc-sig-links"><a href="#v-pkix-attr-printable" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L378" title="View source at line 378">Source</a></span></div>
<div class="doc-body">

Construct a PrintableString `PkixAttr`.

</div>
</article>

<article class="doc-entry" id="v-pkix-attr-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-attr-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-attr-string" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L405" title="View source at line 405">Source</a></span></div>
<div class="doc-body">

Extract the UTF-8 string value of a `PkixAttr`, decoding BMPString (UTF-16BE) and Latin-1/T61 strings as well.

</div>
</article>

<article class="doc-entry" id="v-pkix-attr-to-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-attr-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-pkix-attr-to-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L449" title="View source at line 449">Source</a></span></div>
<div class="doc-body">

Encode a `PkixAttr` as an ASN.1 SEQUENCE { OBJECT IDENTIFIER, ANY }.

</div>
</article>

<article class="doc-entry" id="v-pkix-attr-utf8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-attr-utf8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code><span class="doc-sig-links"><a href="#v-pkix-attr-utf8" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L374" title="View source at line 374">Source</a></span></div>
<div class="doc-body">

Construct a UTF8String `PkixAttr`.

</div>
</article>

<article class="doc-entry" id="v-pkix-attr-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-attr-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-pkix-attr-value" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L370" title="View source at line 370">Source</a></span></div>
<div class="doc-body">

Return the raw ASN.1 value of a `PkixAttr`.

</div>
</article>

<article class="doc-entry" id="v-pkix-common-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-common-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-common-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L617" title="View source at line 617">Source</a></span></div>
<div class="doc-body">

Extract the CommonName (OID 2.5.4.3), if present.

</div>
</article>

<article class="doc-entry" id="v-pkix-country">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-country</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-country" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L625" title="View source at line 625">Source</a></span></div>
<div class="doc-body">

Extract all Country (OID 2.5.4.6) values.

</div>
</article>

<article class="doc-entry" id="v-pkix-crl-from-parts">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-crl-from-parts</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAlgorithmIdentifier" title="Crypto.Pkix.PkixAlgorithmIdentifier">PkixAlgorithmIdentifier</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixCrl" title="Crypto.Pkix.PkixCrl">PkixCrl</a></code><span class="doc-sig-links"><a href="#v-pkix-crl-from-parts" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L974" title="View source at line 974">Source</a></span></div>
<div class="doc-body">

Construct a `PkixCrl` from its constituent fields and signature.

</div>
</article>

<article class="doc-entry" id="v-pkix-decode-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-decode-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-pkix-decode-time" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L295" title="View source at line 295">Source</a></span></div>
<div class="doc-body">

Decode an ASN.1 UTCTime or GeneralizedTime into Unix seconds (I64).

</div>
</article>

<article class="doc-entry" id="v-pkix-encode-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-encode-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-pkix-encode-time" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L303" title="View source at line 303">Source</a></span></div>
<div class="doc-body">

Encode Unix seconds (I64) as RFC 5280 Time: UTCTime for years 1950..2049, GeneralizedTime otherwise.

</div>
</article>

<article class="doc-entry" id="v-pkix-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-error-message" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Render a PKIX error as a human-readable diagnostic message.

</div>
</article>

<article class="doc-entry" id="v-pkix-ext-critical">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-ext-critical</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-pkix-ext-critical" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L859" title="View source at line 859">Source</a></span></div>
<div class="doc-body">

Return whether a `PkixExtension` is marked critical.

</div>
</article>

<article class="doc-entry" id="v-pkix-ext-from-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-ext-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code><span class="doc-sig-links"><a href="#v-pkix-ext-from-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L874" title="View source at line 874">Source</a></span></div>
<div class="doc-body">

Decode an ASN.1 Extension SEQUENCE.

</div>
</article>

<article class="doc-entry" id="v-pkix-ext-oid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-ext-oid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-pkix-ext-oid" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L855" title="View source at line 855">Source</a></span></div>
<div class="doc-body">

Return the OID of a `PkixExtension`.

</div>
</article>

<article class="doc-entry" id="v-pkix-ext-to-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-ext-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-pkix-ext-to-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L867" title="View source at line 867">Source</a></span></div>
<div class="doc-body">

Encode a `PkixExtension` as ASN.1 DER (omitting critical BOOLEAN when False per DER DEFAULT).

</div>
</article>

<article class="doc-entry" id="v-pkix-ext-value">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-ext-value</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pkix-ext-value" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L863" title="View source at line 863">Source</a></span></div>
<div class="doc-body">

Return the raw DER value bytes of a `PkixExtension`.

</div>
</article>

<article class="doc-entry" id="v-pkix-extension">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-extension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code><span class="doc-sig-links"><a href="#v-pkix-extension" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L851" title="View source at line 851">Source</a></span></div>
<div class="doc-body">

Construct a `PkixExtension`.

</div>
</article>

<article class="doc-entry" id="v-pkix-find-extension">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-find-extension</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code><span class="doc-sig-links"><a href="#v-pkix-find-extension" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L885" title="View source at line 885">Source</a></span></div>
<div class="doc-body">

Find the first extension with the given OID in a list.

</div>
</article>

<article class="doc-entry" id="v-pkix-locality">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-locality</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-locality" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L637" title="View source at line 637">Source</a></span></div>
<div class="doc-body">

Extract all Locality (OID 2.5.4.7) values.

</div>
</article>

<article class="doc-entry" id="v-pkix-marshal-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-marshal-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pkix-marshal-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L787" title="View source at line 787">Source</a></span></div>
<div class="doc-body">

Marshal a `PkixName` to DER bytes.

</div>
</article>

<article class="doc-entry" id="v-pkix-name-add-attr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-name-add-attr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-name-add-attr" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L540" title="View source at line 540">Source</a></span></div>
<div class="doc-body">

Append a single-attribute RDN to a `PkixName`.

</div>
</article>

<article class="doc-entry" id="v-pkix-name-add-rdn">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-name-add-rdn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-name-add-rdn" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L535" title="View source at line 535">Source</a></span></div>
<div class="doc-body">

Append one `PkixRdn` to the end of a `PkixName`.

</div>
</article>

<article class="doc-entry" id="v-pkix-name-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-name-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-name-empty" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L523" title="View source at line 523">Source</a></span></div>
<div class="doc-body">

An empty distinguished name.

</div>
</article>

<article class="doc-entry" id="v-pkix-name-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-name-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-pkix-name-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L797" title="View source at line 797">Source</a></span></div>
<div class="doc-body">

Return True iff two `PkixName` values have identical canonical DER encodings.

</div>
</article>

<article class="doc-entry" id="v-pkix-name-from-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-name-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-name-from-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L773" title="View source at line 773">Source</a></span></div>
<div class="doc-body">

Decode an ASN.1 RDNSequence into a `PkixName`.

</div>
</article>

<article class="doc-entry" id="v-pkix-name-from-rdns">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-name-from-rdns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-name-from-rdns" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L527" title="View source at line 527">Source</a></span></div>
<div class="doc-body">

Construct a `PkixName` from an explicit RDN sequence.

</div>
</article>

<article class="doc-entry" id="v-pkix-name-rdns">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-name-rdns</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a></code><span class="doc-sig-links"><a href="#v-pkix-name-rdns" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L531" title="View source at line 531">Source</a></span></div>
<div class="doc-body">

Return the RDN sequence of a `PkixName`.

</div>
</article>

<article class="doc-entry" id="v-pkix-name-to-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-name-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-pkix-name-to-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L762" title="View source at line 762">Source</a></span></div>
<div class="doc-body">

Encode a `PkixName` to its ASN.1 RDNSequence tree.

</div>
</article>

<article class="doc-entry" id="v-pkix-name-to-canonical-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-name-to-canonical-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-name-to-canonical-string" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L671" title="View source at line 671">Source</a></span></div>
<div class="doc-body">

Format a `PkixName` in canonical attribute-type order with non-standard attributes placed at the end.

</div>
</article>

<article class="doc-entry" id="v-pkix-name-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-name-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-name-to-string" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L667" title="View source at line 667">Source</a></span></div>
<div class="doc-body">

Format a `PkixName` according to RFC 4514 (RDNs in reverse order, joined by ",", multi-valued RDNs by "+").

</div>
</article>

<article class="doc-entry" id="v-pkix-organization">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-organization</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-organization" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L629" title="View source at line 629">Source</a></span></div>
<div class="doc-body">

Extract all Organization (OID 2.5.4.10) values.

</div>
</article>

<article class="doc-entry" id="v-pkix-organizational-unit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-organizational-unit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-organizational-unit" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L633" title="View source at line 633">Source</a></span></div>
<div class="doc-body">

Extract all OrganizationalUnit (OID 2.5.4.11) values.

</div>
</article>

<article class="doc-entry" id="v-pkix-parse-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-parse-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-parse-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L791" title="View source at line 791">Source</a></span></div>
<div class="doc-body">

Parse a DER-encoded `PkixName`.

</div>
</article>

<article class="doc-entry" id="v-pkix-postal-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-postal-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-postal-code" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L649" title="View source at line 649">Source</a></span></div>
<div class="doc-body">

Extract all PostalCode (OID 2.5.4.17) values.

</div>
</article>

<article class="doc-entry" id="v-pkix-province">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-province</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-province" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L641" title="View source at line 641">Source</a></span></div>
<div class="doc-body">

Extract all Province/State (OID 2.5.4.8) values.

</div>
</article>

<article class="doc-entry" id="v-pkix-rdn">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-rdn</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a></code><span class="doc-sig-links"><a href="#v-pkix-rdn" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L477" title="View source at line 477">Source</a></span></div>
<div class="doc-body">

Construct a `PkixRdn` from a list of attributes.

</div>
</article>

<article class="doc-entry" id="v-pkix-rdn-attrs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-rdn-attrs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a></code><span class="doc-sig-links"><a href="#v-pkix-rdn-attrs" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L481" title="View source at line 481">Source</a></span></div>
<div class="doc-body">

Return the attributes contained in a `PkixRdn`.

</div>
</article>

<article class="doc-entry" id="v-pkix-rdn-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-rdn-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-pkix-rdn-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L510" title="View source at line 510">Source</a></span></div>
<div class="doc-body">

Return True iff two `PkixRdn` values have equal DER encodings.

</div>
</article>

<article class="doc-entry" id="v-pkix-rdn-from-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-rdn-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a></code><span class="doc-sig-links"><a href="#v-pkix-rdn-from-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L496" title="View source at line 496">Source</a></span></div>
<div class="doc-body">

Decode an ASN.1 SET into a `PkixRdn`.

</div>
</article>

<article class="doc-entry" id="v-pkix-rdn-to-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-rdn-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRdn" title="Crypto.Pkix.PkixRdn">PkixRdn</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-pkix-rdn-to-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L485" title="View source at line 485">Source</a></span></div>
<div class="doc-body">

Encode a `PkixRdn` as an ASN.1 SET OF AttributeTypeAndValue.

</div>
</article>

<article class="doc-entry" id="v-pkix-revoked-certificate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-certificate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a></code><span class="doc-sig-links"><a href="#v-pkix-revoked-certificate" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L914" title="View source at line 914">Source</a></span></div>
<div class="doc-body">

Construct a `PkixRevokedCertificate`.

</div>
</article>

<article class="doc-entry" id="v-pkix-revoked-extensions">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-extensions</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixExtension" title="Crypto.Pkix.PkixExtension">PkixExtension</a></code><span class="doc-sig-links"><a href="#v-pkix-revoked-extensions" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L926" title="View source at line 926">Source</a></span></div>
<div class="doc-body">

Return any CRL entry extensions attached to this revoked certificate entry.

</div>
</article>

<article class="doc-entry" id="v-pkix-revoked-from-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-from-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixError" title="Crypto.Pkix.PkixError">PkixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a></code><span class="doc-sig-links"><a href="#v-pkix-revoked-from-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L937" title="View source at line 937">Source</a></span></div>
<div class="doc-body">

Decode an ASN.1 revokedCertificate entry.

</div>
</article>

<article class="doc-entry" id="v-pkix-revoked-serial">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-serial</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-pkix-revoked-serial" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L918" title="View source at line 918">Source</a></span></div>
<div class="doc-body">

Return the revoked certificate's serial number.

</div>
</article>

<article class="doc-entry" id="v-pkix-revoked-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-pkix-revoked-time" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L922" title="View source at line 922">Source</a></span></div>
<div class="doc-body">

Return the revocation time in Unix seconds.

</div>
</article>

<article class="doc-entry" id="v-pkix-revoked-to-asn1">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-revoked-to-asn1</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixRevokedCertificate" title="Crypto.Pkix.PkixRevokedCertificate">PkixRevokedCertificate</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Asn1/#t-Asn1" title="Text.Asn1.Asn1">Asn1</a></code><span class="doc-sig-links"><a href="#v-pkix-revoked-to-asn1" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L930" title="View source at line 930">Source</a></span></div>
<div class="doc-body">

Encode a `PkixRevokedCertificate` to ASN.1.

</div>
</article>

<article class="doc-entry" id="v-pkix-serial-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-serial-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-serial-number" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L621" title="View source at line 621">Source</a></span></div>
<div class="doc-body">

Extract the SerialNumber (OID 2.5.4.5), if present.

</div>
</article>

<article class="doc-entry" id="v-pkix-street-address">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-street-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pkix-street-address" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L645" title="View source at line 645">Source</a></span></div>
<div class="doc-body">

Extract all StreetAddress (OID 2.5.4.9) values.

</div>
</article>

<article class="doc-entry" id="v-pkix-with-common-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-with-common-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-with-common-name" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L544" title="View source at line 544">Source</a></span></div>
<div class="doc-body">

Append a CommonName (CN) RDN.

</div>
</article>

<article class="doc-entry" id="v-pkix-with-country">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-with-country</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-with-country" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L552" title="View source at line 552">Source</a></span></div>
<div class="doc-body">

Append a Country (C) RDN.

</div>
</article>

<article class="doc-entry" id="v-pkix-with-extra-attr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-with-extra-attr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixAttr" title="Crypto.Pkix.PkixAttr">PkixAttr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-with-extra-attr" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L580" title="View source at line 580">Source</a></span></div>
<div class="doc-body">

Append an arbitrary attribute RDN to a `PkixName`.

</div>
</article>

<article class="doc-entry" id="v-pkix-with-locality">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-with-locality</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-with-locality" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L564" title="View source at line 564">Source</a></span></div>
<div class="doc-body">

Append a Locality (L) RDN.

</div>
</article>

<article class="doc-entry" id="v-pkix-with-organization">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-with-organization</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-with-organization" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L556" title="View source at line 556">Source</a></span></div>
<div class="doc-body">

Append an Organization (O) RDN.

</div>
</article>

<article class="doc-entry" id="v-pkix-with-organizational-unit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-with-organizational-unit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-with-organizational-unit" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L560" title="View source at line 560">Source</a></span></div>
<div class="doc-body">

Append an OrganizationalUnit (OU) RDN.

</div>
</article>

<article class="doc-entry" id="v-pkix-with-postal-code">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-with-postal-code</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-with-postal-code" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L576" title="View source at line 576">Source</a></span></div>
<div class="doc-body">

Append a PostalCode (POSTALCODE) RDN.

</div>
</article>

<article class="doc-entry" id="v-pkix-with-province">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-with-province</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-with-province" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L568" title="View source at line 568">Source</a></span></div>
<div class="doc-body">

Append a Province/State (ST) RDN.

</div>
</article>

<article class="doc-entry" id="v-pkix-with-serial-number">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-with-serial-number</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-with-serial-number" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L548" title="View source at line 548">Source</a></span></div>
<div class="doc-body">

Append a SerialNumber (SERIALNUMBER) RDN.

</div>
</article>

<article class="doc-entry" id="v-pkix-with-street-address">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pkix-with-street-address</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.Pkix/#t-PkixName" title="Crypto.Pkix.PkixName">PkixName</a></code><span class="doc-sig-links"><a href="#v-pkix-with-street-address" title="Permalink">#</a><a href="/packages/stdlib/Crypto.Pkix/source/#L572" title="View source at line 572">Source</a></span></div>
<div class="doc-body">

Append a StreetAddress (STREET) RDN.

</div>
</article>
