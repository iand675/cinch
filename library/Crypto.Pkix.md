# Crypto.Pkix

PKIX distinguished names, OIDs, RFC 4514 formatting, algorithm identifiers, X.509 extensions, times and certificate revocation lists (CRLs).

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Crypto.Pkix`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Crypto.Pkix.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Asn1](Text.Asn1.md), [Text.Hex](Text.Hex.md), [Text.Show](Text.Show.md).

Non-embedded source closure: [Crypto.Pkix](Crypto.Pkix.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Text.Asn1](Text.Asn1.md), [Text.Hex](Text.Hex.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Public Key Infrastructure (PKIX) data structures for X.509 certificates,
certificate signing requests and certificate revocation lists (RFC 5280, RFC 4514).

Provides ASN.1 DER codecs and structured accessors for distinguished names
(`PkixName`, `PkixRdn`, `PkixAttr`), algorithm identifiers (`PkixAlgorithmIdentifier`),
X.509 v3 extensions (`PkixExtension`), revoked certificate entries
(`PkixRevokedCertificate`) and certificate revocation lists (`PkixCrl`),
together with RFC 4514 string formatting, OID utilities, RFC 5280 time codecs
and exact DER TLV boundary helpers.

</details>

# Types and constructors

<a id="type-506b6978416c676f726974686d4964656e746966696572"></a>

## `PkixAlgorithmIdentifier`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PkixAlgorithmIdentifier</span></code></pre>

Algorithm OID and optional ASN.1 parameters.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 807

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PkixAlgorithmIdentifier</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span>))</code></pre>

<a id="type-506b697841747472"></a>

## `PkixAttr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PkixAttr</span></code></pre>

Attribute type OID and ASN.1 value.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 359

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PkixAttr</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span>))</code></pre>

<a id="type-506b697843726c"></a>

## `PkixCrl`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PkixCrl</span></code></pre>

A parsed or constructed X.509 Certificate Revocation List (RFC 5280 CertificateList).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 968

<a id="type-506b69784572726f72"></a>

## `PkixError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PkixError</span></code></pre>

An error encountered while parsing or validating a PKIX structure.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 48

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PkixAsn1Error</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1Error</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PkixInvalidStructure</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PkixInvalidTime</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PkixInvalidOid</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span></code></pre>

<a id="type-506b6978457874656e73696f6e"></a>

## `PkixExtension`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PkixExtension</span></code></pre>

Extension OID, critical flag, and raw DER extnValue octets.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 848

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PkixExtension</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>)))</code></pre>

<a id="type-506b69784e616d65"></a>

## `PkixName`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PkixName</span></code></pre>

The RDNSequence preserving exact order and multi-valued RDNs.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 520

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PkixName</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>)</code></pre>

<a id="type-506b697852646e"></a>

## `PkixRdn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PkixRdn</span></code></pre>

Attributes belonging to this RDN.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 474

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PkixRdn</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span>)</code></pre>

<a id="type-506b69785265766f6b65644365727469666963617465"></a>

## `PkixRevokedCertificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">PkixRevokedCertificate</span></code></pre>

Revoked certificate serial number, revocation Unix timestamp, and entry extensions.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 911

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">PkixRevokedCertificate</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span>)))</code></pre>

# Values

<a id="value-63726c2d657874656e73696f6e73"></a>

## `crl-extensions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-extensions</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>))</code></pre>

Return the CRL extensions.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1016

<a id="value-63726c2d697373756572"></a>

## `crl-issuer`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-issuer</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>)</code></pre>

Return the CRL issuer distinguished name.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1000

<a id="value-63726c2d6e6578742d757064617465"></a>

## `crl-next-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-next-update</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

Return the optional CRL nextUpdate timestamp in Unix seconds.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1008

<a id="value-63726c2d726177"></a>

## `crl-raw`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-raw</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the complete DER encoding of the CRL.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1032

<a id="value-63726c2d7261772d746273"></a>

## `crl-raw-tbs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-raw-tbs</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the exact raw DER bytes of the TBSCertList signed by the issuer.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1020

<a id="value-63726c2d7265766f6b65642d636572746966696361746573"></a>

## `crl-revoked-certificates`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-revoked-certificates</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span>))</code></pre>

Return the list of revoked certificate entries.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1012

<a id="value-63726c2d7369676e6174757265"></a>

## `crl-signature`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-signature</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the raw signature bytes.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1028

<a id="value-63726c2d7369676e61747572652d616c676f726974686d"></a>

## `crl-signature-algorithm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-signature-algorithm</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span>)</code></pre>

Return the outer signature algorithm identifier.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1024

<a id="value-63726c2d7462732d7369676e61747572652d616c676f726974686d"></a>

## `crl-tbs-signature-algorithm`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-tbs-signature-algorithm</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span>)</code></pre>

Return the inner TBS signature algorithm identifier.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 996

<a id="value-63726c2d746869732d757064617465"></a>

## `crl-this-update`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-this-update</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the CRL thisUpdate timestamp in Unix seconds.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1004

<a id="value-63726c2d76657273696f6e"></a>

## `crl-version`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">crl-version</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

Return the CRL version (1 for v1, 2 for v2).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 992

<a id="value-6465722d656c656d656e742d626f756e6473"></a>

## `der-element-bounds`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">der-element-bounds</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>, <span class="tok-constructor">Nat</span>))))</code></pre>

Locate the DER element starting at offset `start`: returns Some (start, content-start, end).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 212

<a id="value-6465722d66697273742d6368696c642d6279746573"></a>

## `der-first-child-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">der-first-child-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Bytes</span>))</code></pre>

Extract the exact raw DER bytes of the first child element of a top-level DER SEQUENCE.
Used to obtain exact wire TBS bytes for signature verification.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 263

<a id="value-6465722d73706c69742d73657175656e6365"></a>

## `der-split-sequence`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">der-split-sequence</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Split a buffer of concatenated top-level DER elements into individual raw element slices.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 277

<a id="value-6d61727368616c2d63726c"></a>

## `marshal-crl`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">marshal-crl</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Marshal a `PkixCrl` to DER bytes.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1046

<a id="value-6d61727368616c2d7462732d63726c"></a>

## `marshal-tbs-crl`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">marshal-tbs-crl</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>) -&gt; <span class="tok-constructor">Bytes</span>)))))))</code></pre>

Marshal the TBSCertList portion of a CRL to DER bytes.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1036

<a id="value-6f69642d636f6d6d6f6e2d6e616d65"></a>

## `oid-common-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-common-name</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

OID 2.5.4.3 (commonName / CN).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 111

<a id="value-6f69642d636f756e747279"></a>

## `oid-country`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-country</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

OID 2.5.4.6 (countryName / C).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 119

<a id="value-6f69642d657175616c"></a>

## `oid-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-equal</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return True iff two OID arc lists are identical.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 147

<a id="value-6f69642d66726f6d2d737472696e67"></a>

## `oid-from-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-from-string</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Parse a dotted-decimal OID string (at least two valid arcs, first arc 0..2).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 165

<a id="value-6f69642d6c6f63616c697479"></a>

## `oid-locality`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-locality</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

OID 2.5.4.7 (localityName / L).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 123

<a id="value-6f69642d6f7267616e697a6174696f6e"></a>

## `oid-organization`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-organization</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

OID 2.5.4.10 (organizationName / O).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 135

<a id="value-6f69642d6f7267616e697a6174696f6e616c2d756e6974"></a>

## `oid-organizational-unit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-organizational-unit</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

OID 2.5.4.11 (organizationalUnitName / OU).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 139

<a id="value-6f69642d706f7374616c2d636f6465"></a>

## `oid-postal-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-postal-code</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

OID 2.5.4.17 (postalCode / POSTALCODE).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 143

<a id="value-6f69642d70726f76696e6365"></a>

## `oid-province`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-province</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

OID 2.5.4.8 (stateOrProvinceName / ST).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 127

<a id="value-6f69642d73657269616c2d6e756d626572"></a>

## `oid-serial-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-serial-number</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

OID 2.5.4.5 (serialNumber / SERIALNUMBER).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 115

<a id="value-6f69642d7374726565742d61646472657373"></a>

## `oid-street-address`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-street-address</span> :: (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>)</code></pre>

OID 2.5.4.9 (streetAddress / STREET).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 131

<a id="value-6f69642d746f2d737472696e67"></a>

## `oid-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">oid-to-string</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format an OID arc list in dotted-decimal notation (e.g. "2.5.4.3").

[Source](/reference-sources/Crypto.Pkix.cinch) · line 155

<a id="value-70617273652d63726c"></a>

## `parse-crl`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">parse-crl</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span>))</code></pre>

Parse a DER-encoded X.509 Certificate Revocation List (RFC 5280 CertificateList).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 1050

<a id="value-706b69782d616c672d657175616c"></a>

## `pkix-alg-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-alg-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return True iff two `PkixAlgorithmIdentifier` values have identical DER encodings.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 838

<a id="value-706b69782d616c672d66726f6d2d61736e31"></a>

## `pkix-alg-from-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-alg-from-asn1</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span>))</code></pre>

Decode an ASN.1 AlgorithmIdentifier SEQUENCE.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 829

<a id="value-706b69782d616c672d6f6964"></a>

## `pkix-alg-oid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-alg-oid</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>))</code></pre>

Return the OID of a `PkixAlgorithmIdentifier`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 814

<a id="value-706b69782d616c672d706172616d73"></a>

## `pkix-alg-params`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-alg-params</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>))</code></pre>

Return the optional ASN.1 parameters of a `PkixAlgorithmIdentifier`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 818

<a id="value-706b69782d616c672d746f2d61736e31"></a>

## `pkix-alg-to-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-alg-to-asn1</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

Encode a `PkixAlgorithmIdentifier` as an ASN.1 SEQUENCE.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 822

<a id="value-706b69782d616c676f726974686d2d6964656e746966696572"></a>

## `pkix-algorithm-identifier`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-algorithm-identifier</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span>))</code></pre>

Construct a `PkixAlgorithmIdentifier`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 810

<a id="value-706b69782d61747472"></a>

## `pkix-attr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-attr</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span>))</code></pre>

Construct a `PkixAttr` from an OID and ASN.1 value.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 362

<a id="value-706b69782d617474722d657175616c"></a>

## `pkix-attr-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-attr-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return True iff two `PkixAttr` values have the same OID and identical DER-encoded value.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 462

<a id="value-706b69782d617474722d666f722d737472696e67"></a>

## `pkix-attr-for-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-attr-for-string</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span>))</code></pre>

Construct a string `PkixAttr` using PrintableString when all bytes are valid PrintableString characters, else UTF8String.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 382

<a id="value-706b69782d617474722d66726f6d2d61736e31"></a>

## `pkix-attr-from-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-attr-from-asn1</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span>))</code></pre>

Decode an ASN.1 AttributeTypeAndValue into a `PkixAttr`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 454

<a id="value-706b69782d617474722d6f6964"></a>

## `pkix-attr-oid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-attr-oid</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>))</code></pre>

Return the OID of a `PkixAttr`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 366

<a id="value-706b69782d617474722d7072696e7461626c65"></a>

## `pkix-attr-printable`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-attr-printable</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span>))</code></pre>

Construct a PrintableString `PkixAttr`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 378

<a id="value-706b69782d617474722d737472696e67"></a>

## `pkix-attr-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-attr-string</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Extract the UTF-8 string value of a `PkixAttr`, decoding BMPString (UTF-16BE) and Latin-1/T61 strings as well.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 405

<a id="value-706b69782d617474722d746f2d61736e31"></a>

## `pkix-attr-to-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-attr-to-asn1</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

Encode a `PkixAttr` as an ASN.1 SEQUENCE { OBJECT IDENTIFIER, ANY }.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 449

<a id="value-706b69782d617474722d75746638"></a>

## `pkix-attr-utf8`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-attr-utf8</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span>))</code></pre>

Construct a UTF8String `PkixAttr`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 374

<a id="value-706b69782d617474722d76616c7565"></a>

## `pkix-attr-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-attr-value</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

Return the raw ASN.1 value of a `PkixAttr`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 370

<a id="value-706b69782d636f6d6d6f6e2d6e616d65"></a>

## `pkix-common-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-common-name</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Extract the CommonName (OID 2.5.4.3), if present.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 617

<a id="value-706b69782d636f756e747279"></a>

## `pkix-country`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-country</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Extract all Country (OID 2.5.4.6) values.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 625

<a id="value-706b69782d63726c2d66726f6d2d7061727473"></a>

## `pkix-crl-from-parts`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-crl-from-parts</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAlgorithmIdentifier</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>) -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixCrl</span>))))))))</code></pre>

Construct a `PkixCrl` from its constituent fields and signature.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 974

<a id="value-706b69782d6465636f64652d74696d65"></a>

## `pkix-decode-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-decode-time</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> <span class="tok-constructor">I64</span>))</code></pre>

Decode an ASN.1 UTCTime or GeneralizedTime into Unix seconds (I64).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 295

<a id="value-706b69782d656e636f64652d74696d65"></a>

## `pkix-encode-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-encode-time</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

Encode Unix seconds (I64) as RFC 5280 Time: UTCTime for years 1950..2049, GeneralizedTime otherwise.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 303

<a id="value-706b69782d6572726f722d6d657373616765"></a>

## `pkix-error-message`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-error-message</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render a PKIX error as a human-readable diagnostic message.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 59

<a id="value-706b69782d6578742d637269746963616c"></a>

## `pkix-ext-critical`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-ext-critical</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Return whether a `PkixExtension` is marked critical.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 859

<a id="value-706b69782d6578742d66726f6d2d61736e31"></a>

## `pkix-ext-from-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-ext-from-asn1</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>))</code></pre>

Decode an ASN.1 Extension SEQUENCE.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 874

<a id="value-706b69782d6578742d6f6964"></a>

## `pkix-ext-oid`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-ext-oid</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>))</code></pre>

Return the OID of a `PkixExtension`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 855

<a id="value-706b69782d6578742d746f2d61736e31"></a>

## `pkix-ext-to-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-ext-to-asn1</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

Encode a `PkixExtension` as ASN.1 DER (omitting critical BOOLEAN when False per DER DEFAULT).

[Source](/reference-sources/Crypto.Pkix.cinch) · line 867

<a id="value-706b69782d6578742d76616c7565"></a>

## `pkix-ext-value`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-ext-value</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Return the raw DER value bytes of a `PkixExtension`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 863

<a id="value-706b69782d657874656e73696f6e"></a>

## `pkix-extension`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-extension</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; (<span class="tok-constructor">Bool</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>)))</code></pre>

Construct a `PkixExtension`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 851

<a id="value-706b69782d66696e642d657874656e73696f6e"></a>

## `pkix-find-extension`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-find-extension</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Nat</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>)))</code></pre>

Find the first extension with the given OID in a list.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 885

<a id="value-706b69782d6c6f63616c697479"></a>

## `pkix-locality`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-locality</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Extract all Locality (OID 2.5.4.7) values.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 637

<a id="value-706b69782d6d61727368616c2d6e616d65"></a>

## `pkix-marshal-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-marshal-name</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; <span class="tok-constructor">Bytes</span>)</code></pre>

Marshal a `PkixName` to DER bytes.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 787

<a id="value-706b69782d6e616d652d6164642d61747472"></a>

## `pkix-name-add-attr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-name-add-attr</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append a single-attribute RDN to a `PkixName`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 540

<a id="value-706b69782d6e616d652d6164642d72646e"></a>

## `pkix-name-add-rdn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-name-add-rdn</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append one `PkixRdn` to the end of a `PkixName`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 535

<a id="value-706b69782d6e616d652d656d707479"></a>

## `pkix-name-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-name-empty</span> :: <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span></code></pre>

An empty distinguished name.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 523

<a id="value-706b69782d6e616d652d657175616c"></a>

## `pkix-name-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-name-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return True iff two `PkixName` values have identical canonical DER encodings.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 797

<a id="value-706b69782d6e616d652d66726f6d2d61736e31"></a>

## `pkix-name-from-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-name-from-asn1</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Decode an ASN.1 RDNSequence into a `PkixName`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 773

<a id="value-706b69782d6e616d652d66726f6d2d72646e73"></a>

## `pkix-name-from-rdns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-name-from-rdns</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>)</code></pre>

Construct a `PkixName` from an explicit RDN sequence.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 527

<a id="value-706b69782d6e616d652d72646e73"></a>

## `pkix-name-rdns`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-name-rdns</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span>))</code></pre>

Return the RDN sequence of a `PkixName`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 531

<a id="value-706b69782d6e616d652d746f2d61736e31"></a>

## `pkix-name-to-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-name-to-asn1</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

Encode a `PkixName` to its ASN.1 RDNSequence tree.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 762

<a id="value-706b69782d6e616d652d746f2d63616e6f6e6963616c2d737472696e67"></a>

## `pkix-name-to-canonical-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-name-to-canonical-string</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a `PkixName` in canonical attribute-type order with non-standard attributes placed at the end.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 671

<a id="value-706b69782d6e616d652d746f2d737472696e67"></a>

## `pkix-name-to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-name-to-string</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Format a `PkixName` according to RFC 4514 (RDNs in reverse order, joined by ",", multi-valued RDNs by "+").

[Source](/reference-sources/Crypto.Pkix.cinch) · line 667

<a id="value-706b69782d6f7267616e697a6174696f6e"></a>

## `pkix-organization`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-organization</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Extract all Organization (OID 2.5.4.10) values.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 629

<a id="value-706b69782d6f7267616e697a6174696f6e616c2d756e6974"></a>

## `pkix-organizational-unit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-organizational-unit</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Extract all OrganizationalUnit (OID 2.5.4.11) values.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 633

<a id="value-706b69782d70617273652d6e616d65"></a>

## `pkix-parse-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-parse-name</span> :: (<span class="tok-constructor">Bytes</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Parse a DER-encoded `PkixName`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 791

<a id="value-706b69782d706f7374616c2d636f6465"></a>

## `pkix-postal-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-postal-code</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Extract all PostalCode (OID 2.5.4.17) values.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 649

<a id="value-706b69782d70726f76696e6365"></a>

## `pkix-province`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-province</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Extract all Province/State (OID 2.5.4.8) values.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 641

<a id="value-706b69782d72646e"></a>

## `pkix-rdn`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-rdn</span> :: ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span>)</code></pre>

Construct a `PkixRdn` from a list of attributes.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 477

<a id="value-706b69782d72646e2d6174747273"></a>

## `pkix-rdn-attrs`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-rdn-attrs</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span>))</code></pre>

Return the attributes contained in a `PkixRdn`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 481

<a id="value-706b69782d72646e2d657175616c"></a>

## `pkix-rdn-equal`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-rdn-equal</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

Return True iff two `PkixRdn` values have equal DER encodings.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 510

<a id="value-706b69782d72646e2d66726f6d2d61736e31"></a>

## `pkix-rdn-from-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-rdn-from-asn1</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span>))</code></pre>

Decode an ASN.1 SET into a `PkixRdn`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 496

<a id="value-706b69782d72646e2d746f2d61736e31"></a>

## `pkix-rdn-to-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-rdn-to-asn1</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRdn</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

Encode a `PkixRdn` as an ASN.1 SET OF AttributeTypeAndValue.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 485

<a id="value-706b69782d7265766f6b65642d6365727469666963617465"></a>

## `pkix-revoked-certificate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-revoked-certificate</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">I64</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>) -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span>)))</code></pre>

Construct a `PkixRevokedCertificate`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 914

<a id="value-706b69782d7265766f6b65642d657874656e73696f6e73"></a>

## `pkix-revoked-extensions`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-revoked-extensions</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixExtension</span>))</code></pre>

Return any CRL entry extensions attached to this revoked certificate entry.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 926

<a id="value-706b69782d7265766f6b65642d66726f6d2d61736e31"></a>

## `pkix-revoked-from-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-revoked-from-asn1</span> :: (<span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixError</span> <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span>))</code></pre>

Decode an ASN.1 revokedCertificate entry.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 937

<a id="value-706b69782d7265766f6b65642d73657269616c"></a>

## `pkix-revoked-serial`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-revoked-serial</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span> -&gt; <span class="tok-constructor">Int</span>)</code></pre>

Return the revoked certificate's serial number.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 918

<a id="value-706b69782d7265766f6b65642d74696d65"></a>

## `pkix-revoked-time`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-revoked-time</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the revocation time in Unix seconds.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 922

<a id="value-706b69782d7265766f6b65642d746f2d61736e31"></a>

## `pkix-revoked-to-asn1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-revoked-to-asn1</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixRevokedCertificate</span> -&gt; <span class="tok-constructor">Text</span>.<span class="tok-constructor">Asn1</span>.<span class="tok-constructor">Asn1</span>)</code></pre>

Encode a `PkixRevokedCertificate` to ASN.1.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 930

<a id="value-706b69782d73657269616c2d6e756d626572"></a>

## `pkix-serial-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-serial-number</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Extract the SerialNumber (OID 2.5.4.5), if present.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 621

<a id="value-706b69782d7374726565742d61646472657373"></a>

## `pkix-street-address`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-street-address</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Extract all StreetAddress (OID 2.5.4.9) values.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 645

<a id="value-706b69782d776974682d636f6d6d6f6e2d6e616d65"></a>

## `pkix-with-common-name`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-with-common-name</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append a CommonName (CN) RDN.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 544

<a id="value-706b69782d776974682d636f756e747279"></a>

## `pkix-with-country`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-with-country</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append a Country (C) RDN.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 552

<a id="value-706b69782d776974682d65787472612d61747472"></a>

## `pkix-with-extra-attr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-with-extra-attr</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixAttr</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append an arbitrary attribute RDN to a `PkixName`.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 580

<a id="value-706b69782d776974682d6c6f63616c697479"></a>

## `pkix-with-locality`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-with-locality</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append a Locality (L) RDN.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 564

<a id="value-706b69782d776974682d6f7267616e697a6174696f6e"></a>

## `pkix-with-organization`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-with-organization</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append an Organization (O) RDN.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 556

<a id="value-706b69782d776974682d6f7267616e697a6174696f6e616c2d756e6974"></a>

## `pkix-with-organizational-unit`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-with-organizational-unit</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append an OrganizationalUnit (OU) RDN.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 560

<a id="value-706b69782d776974682d706f7374616c2d636f6465"></a>

## `pkix-with-postal-code`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-with-postal-code</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append a PostalCode (POSTALCODE) RDN.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 576

<a id="value-706b69782d776974682d70726f76696e6365"></a>

## `pkix-with-province`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-with-province</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append a Province/State (ST) RDN.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 568

<a id="value-706b69782d776974682d73657269616c2d6e756d626572"></a>

## `pkix-with-serial-number`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-with-serial-number</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append a SerialNumber (SERIALNUMBER) RDN.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 548

<a id="value-706b69782d776974682d7374726565742d61646472657373"></a>

## `pkix-with-street-address`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pkix-with-street-address</span> :: (<span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; <span class="tok-constructor">Crypto</span>.<span class="tok-constructor">Pkix</span>.<span class="tok-constructor">PkixName</span>))</code></pre>

Append a StreetAddress (STREET) RDN.

[Source](/reference-sources/Crypto.Pkix.cinch) · line 572

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Crypto/Pkix.cinch`. Source SHA-256: `27018e645b601e7a13ee4cdcb5d0adce24a93ee350240a4902d24055f2d7a4a0`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
