# Data.IpAddr

Validated IP addresses, parsing/rendering, and address-family vocabulary.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.IpAddr</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.IpAddr/source/">View source</a><a href="/packages/search/?q=%2BData.IpAddr">Search module</a><a href="/reference-sources/Data.IpAddr.cinch" download="Data.IpAddr.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.IpAddr: IPv4 and IPv6 addresses as data, not names to resolve.

A port of Ruby's `IPAddr` (design/stdlib-parity-ruby.md, the
networking family: "`Net::HTTP`, `Socket`, `URI`, `IPAddr`,
`Resolv`/DNS"). Ruby folds both families into one class and lets
construction reach the network; this module keeps the two families
as distinct types (`Ipv4`, `Ipv6`) with a sum (`IpAddr`) over them,
because the two have different widths, different byte counts and
different text grammars, and because a value that has been parsed
must never depend on a resolver.

Module scheme (design/base-library.md): `Data.*` homes are concrete
types, and an address is a concrete value with its parsing,
rendering, byte view and comparison, so its home is `Data.IpAddr`.
Identifiers are kebab-case per D106. The range invariants (four
octets at most 255, exactly eight 16-bit groups) live at the smart
constructors and at `ipv4-parse`/`ipv6-parse`; the raw constructor
stays out of the export list per D143's surface rule.

The law shape (exercised in seed/tests/lib_uri_ipaddr.rs):
  parse/render round trip: `ipv4-parse (ipv4-to-string a) == Some a`
    and `ipv6-parse (ipv6-to-string a) == Some a`, so the canonical
    rendering is a fixed point of parsing;
  the byte view is width-exact and most significant byte first:
    four bytes for `Ipv4`, sixteen for `Ipv6`;
  `Eq`/`Ord` agree with numeric address order within a family and
    place every `Ipv4` before every `Ipv6` across families.

Named refusals (unproductions this module deliberately does not
implement; each is a named boundary, never a silent gap):
  name resolution: no DNS, no `/etc/hosts`, no lookup of any kind
    lives here (Ruby's `IPAddr#new` resolves a hostname). Resolution
    belongs to the socket slice, which will call it out explicitly;
  IPv4-embedded and IPv4-compatible IPv6 (`::ffff:1.2.3.4`,
    `::1.2.3.4`) and NAT64 spellings: refused, not guessed, since
    the mixed text grammar hides which family a reader meant;
  scope and zone identifiers (`fe80::1%eth0`): refused (the zone is
    a property of the interface, not of the address);
  prefixes and masks (`10.0.0.0/8`, `2001:db8::/32`): refused (the
    network prefix is the prefix family's business, not this one's);
  IPv4 short and leading-zero forms (`1.2.3`, `010.1.1.1`, `0x7f.1`):
    refused, following RFC 3986's `dec-octet` (three digits at most,
    no leading zero), so one text spells exactly one address.

</details>

<details class="doc-synopsis" open><summary>Synopsis (16 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-IpAddr">IpAddr</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IpAddr</strong></code></li><li><a class="syn-name" href="#t-IpFamily">IpFamily</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IpFamily</strong></code></li><li><a class="syn-name" href="#t-Ipv4">Ipv4</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ipv4</strong></code></li><li><a class="syn-name" href="#t-Ipv6">Ipv6</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ipv6</strong></code></li><li><a class="syn-name" href="#v-ip-bytes">ip-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ip-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ip-family">ip-family</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ip-family</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpFamily" title="Data.IpAddr.IpFamily">IpFamily</a></code></li><li><a class="syn-name" href="#v-ip-parse">ip-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ip-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code></li><li><a class="syn-name" href="#v-ip-to-string">ip-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ip-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-ipv4-bytes">ipv4-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv4-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ipv4-from-octets">ipv4-from-octets</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv4-from-octets</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a></code></li><li><a class="syn-name" href="#v-ipv4-parse">ipv4-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv4-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a></code></li><li><a class="syn-name" href="#v-ipv4-to-string">ipv4-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv4-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-ipv6-bytes">ipv6-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv6-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-ipv6-from-groups">ipv6-from-groups</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv6-from-groups</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a></code></li><li><a class="syn-name" href="#v-ipv6-parse">ipv6-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv6-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a></code></li><li><a class="syn-name" href="#v-ipv6-to-string">ipv6-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">ipv6-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-IpAddr">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IpAddr</strong></code><span class="doc-sig-links"><a href="#t-IpAddr" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-V4Addr"><code><strong class="tok-constructor">V4Addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code></li><li id="k-V6Addr"><code><strong class="tok-constructor">V6Addr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.IpAddr/#instances">Data.IpAddr</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.IpAddr/#instances">Data.IpAddr</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-IpFamily">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">IpFamily</strong></code><span class="doc-sig-links"><a href="#t-IpFamily" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Which family an address belongs to (Ruby spells this as a pair of
predicates; one total witness keeps the answer exhaustive).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Ipv4Family"><code><strong class="tok-constructor">Ipv4Family</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpFamily" title="Data.IpAddr.IpFamily">IpFamily</a></code></li><li id="k-Ipv6Family"><code><strong class="tok-constructor">Ipv6Family</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpFamily" title="Data.IpAddr.IpFamily">IpFamily</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Ipv4">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ipv4</strong></code><span class="doc-sig-links"><a href="#t-Ipv4" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

Four octets, each 0..=255. The field type carries the bound, so the
byte form needs no check and no constructor site can exceed it.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.IpAddr/#instances">Data.IpAddr</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.IpAddr/#instances">Data.IpAddr</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-Ipv6">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ipv6</strong></code><span class="doc-sig-links"><a href="#t-Ipv6" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L79" title="View source at line 79">Source</a></span></div>
<div class="doc-body">

Exactly eight 16-bit groups, most significant first.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.IpAddr/#instances">Data.IpAddr</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.IpAddr/#instances">Data.IpAddr</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-ip-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ip-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ip-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L420" title="View source at line 420">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ip-family">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ip-family</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpFamily" title="Data.IpAddr.IpFamily">IpFamily</a></code><span class="doc-sig-links"><a href="#v-ip-family" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L410" title="View source at line 410">Source</a></span></div>
<div class="doc-body">

The sum over the families.

</div>
</article>

<article class="doc-entry" id="v-ip-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ip-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code><span class="doc-sig-links"><a href="#v-ip-parse" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L425" title="View source at line 425">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ip-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ip-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-ip-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L415" title="View source at line 415">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ipv4-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv4-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ipv4-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L194" title="View source at line 194">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ipv4-from-octets">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv4-from-octets</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a></code><span class="doc-sig-links"><a href="#v-ipv4-from-octets" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

The smart constructor: octets outside 0..=255 are refused, so no
caller can build an `Ipv4` the parser would never produce.

</div>
</article>

<article class="doc-entry" id="v-ipv4-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv4-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a></code><span class="doc-sig-links"><a href="#v-ipv4-parse" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L171" title="View source at line 171">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ipv4-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv4-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-ipv4-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L184" title="View source at line 184">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ipv6-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv6-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-ipv6-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L375" title="View source at line 375">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ipv6-from-groups">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv6-from-groups</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a></code><span class="doc-sig-links"><a href="#v-ipv6-from-groups" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L304" title="View source at line 304">Source</a></span></div>
<div class="doc-body">

The smart constructor: exactly eight groups, each 0..=65535.

</div>
</article>

<article class="doc-entry" id="v-ipv6-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv6-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a></code><span class="doc-sig-links"><a href="#v-ipv6-parse" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L291" title="View source at line 291">Source</a></span></div>
</article>

<article class="doc-entry" id="v-ipv6-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">ipv6-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-ipv6-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.IpAddr/source/#L356" title="View source at line 356">Source</a></span></div>
<div class="doc-body">

Canonical rendering: lower-case hex, the chosen zero run spelled
`::`, at most one `::` per address.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_IpAddr"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code> <span class="doc-inst-meta">Implicit · <code>Data.IpAddr.Eq_IpAddr</code></span></li>
<li id="i-Eq_Ipv4"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a></code> <span class="doc-inst-meta">Implicit · <code>Data.IpAddr.Eq_Ipv4</code></span><div class="doc-child-prose">Instances (they live with their type; D159's parked deviation is over).</div></li>
<li id="i-Eq_Ipv6"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a></code> <span class="doc-inst-meta">Implicit · <code>Data.IpAddr.Eq_Ipv6</code></span></li>
<li id="i-Ord_IpAddr"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code> <span class="doc-inst-meta">Implicit · <code>Data.IpAddr.Ord_IpAddr</code></span></li>
<li id="i-Ord_Ipv4"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a></code> <span class="doc-inst-meta">Implicit · <code>Data.IpAddr.Ord_Ipv4</code></span></li>
<li id="i-Ord_Ipv6"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a></code> <span class="doc-inst-meta">Implicit · <code>Data.IpAddr.Ord_Ipv6</code></span></li>
</ul>
