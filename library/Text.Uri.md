# Text.Uri

URI parsing, rendering, components, and validation/error vocabulary.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Uri</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Uri/source/">View source</a><a href="/packages/search/?q=%2BText.Uri">Search module</a><a href="/reference-sources/Text.Uri.cinch" download="Text.Uri.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.IpAddr](Data.IpAddr.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Text.Uri`: [Data.IpAddr](Data.IpAddr.md), [Text.Uri](Text.Uri.md).

```sh
cp seed/prelude/Data/IpAddr.cinch ./Data.IpAddr.cinch
cp seed/prelude/Text/Uri.cinch ./Text.Uri.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Text.Uri: URI references (RFC 3986) as components, parsed, printed,
normalized and resolved.

A port of Ruby's `URI` (design/stdlib-parity-ruby.md, the networking
family: "`Net::HTTP`, `Socket`, `URI`, `IPAddr`, `Resolv`/DNS").
Ruby's `URI` is a class hierarchy with per-scheme subclasses and a
`parse` that can raise; here one record-free data type carries every
reference and the parse returns a named error.

Module scheme (design/base-library.md): `Text.*` is conversion to and
from String/Bytes, and a URI's whole job is the text form, so its home
is `Text.Uri`. Host classification is what pulls in `Data.IpAddr`:
a bracketed host is checked as an IPv6 literal at parse time
(`ipv6-parse`), and `uri-host-ip` reports the address a host names
when it names one. Identifiers are kebab-case per D106; the component
accessors keep the surface readable and the constructor is exported so
a caller can build a reference it did not parse (D143's surface rule
leaves a constructor out only when it carries an invariant, and a URI
reference has none beyond the percent rule, which `uri-to-string`
does not depend on).

The law shape (exercised in seed/tests/lib_uri_ipaddr.rs):
  parse/print round trip: `uri-to-string (uri-parse s)` is `s` for
    every reference `uri-parse` accepts, so printing never invents or
    drops structure;
  percent round trip: `uri-decode (uri-encode s)` is `Ok s` for every
    `String s`, non-ASCII included, over the documented unreserved
    set below;
  resolution: `uri-resolve` implements RFC 3986 section 5.2 with the
    dot-segment removal of section 5.2.4, so the section 5.4 normal
    and abnormal examples reproduce exactly. Where the RFC offers a
    choice, this is the strict-parser reading: `http:g` against the
    base resolves to `http:g`, never the non-strict `http://a/b/c/g`.

Named refusals (unproductions this module deliberately does not
implement; each is a named boundary, never a silent gap):
  `%`-encoding of the unreserved set only, and upper-case hex on
    output: `uri-encode` is not a general "escape everything" (see
    `unreserved` for the exact set);
  scheme-specific normalization beyond section 6.2.2 (no path case
    folding, no default-filename handling, no `http://x` -> `http://x/`):
    that is the scheme registry's business, not this module's;
  IRI support (RFC 3987): no conversion between an IRI and its URI
    form lives here. Non-ASCII text in the host or path is kept as
    written, the scheme is ASCII-only by `valid-scheme`, and
    `uri-decode` validates its result as UTF-8;
  `mailto:`-style opaque bodies and scheme-specific query grammars:
    the generic syntax keeps them as path and query text, never
    interpreted;
  HTML/XML entity escaping and other serializations: printing is the
    RFC 3986 text form, nothing else.

</details>

<details class="doc-synopsis" open><summary>Synopsis (17 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Uri">Uri</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Uri</strong></code></li><li><a class="syn-name" href="#t-UriError">UriError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UriError</strong></code></li><li><a class="syn-name" href="#v-render-uri-error">render-uri-error</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">render-uri-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-uri-decode">uri-decode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-uri-encode">uri-encode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-uri-fragment">uri-fragment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-uri-host">uri-host</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-uri-host-ip">uri-host-ip</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-host-ip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code></li><li><a class="syn-name" href="#v-uri-normalize">uri-normalize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-normalize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a></code></li><li><a class="syn-name" href="#v-uri-parse">uri-parse</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a></code></li><li><a class="syn-name" href="#v-uri-path">uri-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-uri-port">uri-port</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-port</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-uri-query">uri-query</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-uri-resolve">uri-resolve</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-resolve</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a></code></li><li><a class="syn-name" href="#v-uri-scheme">uri-scheme</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-scheme</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-uri-to-string">uri-to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-uri-userinfo">uri-userinfo</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">uri-userinfo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Uri">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Uri</strong></code><span class="doc-sig-links"><a href="#t-Uri" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

The reference and its components.

The seven RFC 3986 components, each `None` when absent (an absent
component and an empty one differ: `http://x` has no query, `http://x?`
has an empty one, and the round trip needs the difference). `host`
carries the authority's host text as written, brackets included, so
`uri-to-string` reprints an IPv6 literal without re-deciding its
brackets; a present-but-empty `host` is the empty authority of
`file:///x`.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MkUri"><code><strong class="tok-constructor">MkUri</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-UriError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">UriError</strong></code><span class="doc-sig-links"><a href="#t-UriError" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

Errors: every malformed reference names the rule it broke.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-BadPercentEscape"><code><strong class="tok-constructor">BadPercentEscape</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a></code></li><li id="k-BadScheme"><code><strong class="tok-constructor">BadScheme</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a></code></li><li id="k-BadPort"><code><strong class="tok-constructor">BadPort</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a></code></li><li id="k-BadIpv6Host"><code><strong class="tok-constructor">BadIpv6Host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a></code></li><li id="k-BadUtf8"><code><strong class="tok-constructor">BadUtf8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-render-uri-error">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">render-uri-error</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-render-uri-error" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L131" title="View source at line 131">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-decode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-decode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-uri-decode" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L255" title="View source at line 255">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-encode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-encode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-uri-encode" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L239" title="View source at line 239">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-fragment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-fragment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-uri-fragment" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L115" title="View source at line 115">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-host">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-host</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-uri-host" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L99" title="View source at line 99">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-host-ip">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-host-ip</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code><span class="doc-sig-links"><a href="#v-uri-host-ip" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L581" title="View source at line 581">Source</a></span></div>
<div class="doc-body">

The address a host names, when it names one: a bracketed literal is
an IPv6 address, a bare host that parses as IPv4 is one, and any
other host (a registered name, or an unbracketed IPv6) is `None`.

</div>
</article>

<article class="doc-entry" id="v-uri-normalize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-normalize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a></code><span class="doc-sig-links"><a href="#v-uri-normalize" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L542" title="View source at line 542">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-parse">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-parse</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-UriError" title="Text.Uri.UriError">UriError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a></code><span class="doc-sig-links"><a href="#v-uri-parse" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L294" title="View source at line 294">Source</a></span></div>
<div class="doc-body">

Parsing (RFC 3986 appendix B's grammar, with the checks the
grammar needs and the appendix's regex cannot carry).

</div>
</article>

<article class="doc-entry" id="v-uri-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-uri-path" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L107" title="View source at line 107">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-port">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-port</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-uri-port" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L103" title="View source at line 103">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-query">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-query</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-uri-query" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L111" title="View source at line 111">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-resolve">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-resolve</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a></code><span class="doc-sig-links"><a href="#v-uri-resolve" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L652" title="View source at line 652">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-scheme">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-scheme</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-uri-scheme" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L91" title="View source at line 91">Source</a></span></div>
</article>

<article class="doc-entry" id="v-uri-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-uri-to-string" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L451" title="View source at line 451">Source</a></span></div>
<div class="doc-body">

Printing: the exact inverse of parsing (the round-trip law).

</div>
</article>

<article class="doc-entry" id="v-uri-userinfo">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">uri-userinfo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Uri/#t-Uri" title="Text.Uri.Uri">Uri</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-uri-userinfo" title="Permalink">#</a><a href="/packages/stdlib/Text.Uri/source/#L95" title="View source at line 95">Source</a></span></div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Uri.cinch`](/packages/stdlib/Text.Uri/source/). Source SHA-256: `e3098cfb52bdee5813cb985d730d5865cb01c2b687d42464f821b73121bc1749`.
