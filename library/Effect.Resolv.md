# Effect.Resolv

Hostname/address resolution through the runtime, with named failure results.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Resolv</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Resolv/">View source</a><a href="/library/search/?q=%2BEffect.Resolv">Search module</a><a href="/reference-sources/Effect.Resolv.cinch" download="Effect.Resolv.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.IpAddr](Data.IpAddr.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Net](Effect.Net.md), [Text.Scan](Text.Scan.md).

<details class="doc-closure"><summary>Ship-source closure (4 files)</summary>

Copy these modules beside your entry file before importing `Effect.Resolv`: [Data.IpAddr](Data.IpAddr.md), [Effect.Net](Effect.Net.md), [Effect.Resolv](Effect.Resolv.md), [Text.Scan](Text.Scan.md).

```sh
cp seed/prelude/Data/IpAddr.cinch ./Data.IpAddr.cinch
cp seed/prelude/Effect/Net.cinch ./Effect.Net.cinch
cp seed/prelude/Effect/Resolv.cinch ./Effect.Resolv.cinch
cp seed/prelude/Text/Scan.cinch ./Text.Scan.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Effect.Resolv: name resolution as typed addresses, over the
resolution intrinsic (`Effect.Net`'s `net-resolve`, the runtime's
`cinch_net_resolve` in `seed/runtime/src/net.rs`). The boundary
answers one `<ip>:<port>` line per resolved address; this home is
the DNS vocabulary over those lines, parsing each into a
`Data.IpAddr` address paired with its port, so a caller holds
addresses, never text.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary, and a resolution is a runtime capability
consulted under the `Io r` row, so its home is an `Effect.*` module.
This is the DNS half design/stdlib-parity-ruby.md's networking
family names (Ruby's `Resolv`), the piece D363 leaves open beside
`Net::HTTP` ("a DNS home over `net-resolve`'s lines remain"): the
socket, URI and IPAddr halves of the family landed as `Effect.Net`
(D363), `Text.Uri` and `Data.IpAddr` (D347). The module has no
runtime externs and needs no compiler edit; its only boundary is
`Effect.Net`'s `net-resolve`.

THE LAW SHAPE (what the slice's tests pin):
  1. typed answers: `resolve host port` answers a `List` of
     `(IpAddr, Nat)` pairs for the lines the resolver returned, so
     an address is a value with a family, a rendering and a byte
     view, not a slice of the resolver's text;
  2. renderer round trip: for every answer `(a, _)`,
     `ip-parse (ip-to-string a)` is `Some a` (the address a caller
     renders is the address the resolver described, because
     `Data.IpAddr`'s rendering is a fixed point of its parsing);
  3. port carried: every answer's port is the port the caller asked
     for, because resolution adds no address the caller did not
     name a port for;
  4. well formed: every line the boundary returned is an address
     and no empty line survives (the newline is a terminator, not an
     answer);
  5. refusal is a value: a name the platform cannot resolve and a
     port outside 0..=65535 are named `ResolvError` values, never a
     fault and never a hang.

Named refusals (boundaries this module deliberately does not cross):
  address records only: the boundary answers address lookups
    (A/AAAA) and nothing else, so there is no MX, TXT, NS or any
    other record type here; a general DNS client that answers those
    is a hand-rolled UDP query and its own slice;
  no caching and no TTL: every call asks the resolver afresh and no
    answer is remembered, so no TTL is observed or honoured;
  no `/etc/hosts` and no search-domain handling: that policy is the
    platform resolver's business and this home neither reimplements
    nor bypasses it (a name resolves the way the platform resolves
    it, hosts file and search domains included or not, by the
    platform's own rules);
  forward lookups only: the boundary maps a name to addresses, so
    there is no reverse (address to name) lookup here, because the
    boundary does not support one;
  no ordering guarantee: the answers are the resolver's own order,
    and nothing here sorts or deduplicates them.

</details>

<details class="doc-synopsis" open><summary>Synopsis (6 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ResolvError">ResolvError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ResolvError</strong></code></li><li><a class="syn-name" href="#v-answer-addresses">answer-addresses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">answer-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code></li><li><a class="syn-name" href="#v-answer-family-count">answer-family-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">answer-family-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpFamily" title="Data.IpAddr.IpFamily">IpFamily</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-answer-has-family">answer-has-family</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">answer-has-family</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpFamily" title="Data.IpAddr.IpFamily">IpFamily</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-resolve">resolve</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resolve</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Resolv/#t-ResolvError" title="Effect.Resolv.ResolvError">ResolvError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>))</code></li><li><a class="syn-name" href="#v-resolve-addresses">resolve-addresses</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">resolve-addresses</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Resolv/#t-ResolvError" title="Effect.Resolv.ResolvError">ResolvError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ResolvError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ResolvError</strong></code><span class="doc-sig-links"><a href="#t-ResolvError" title="Permalink">#</a><a href="/library/source/Effect.Resolv/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

What went wrong, named. The four shared platform names are the
boundary's own vocabulary (`Effect.Net`'s `NetError`, the codes
`seed/runtime/src/platform.rs` shares across the externs);
`ResolvMalformedAnswer` is this home's extra name for a line the
boundary returned that is not a well-formed `<ip>:<port>`, which a
well-behaved boundary never produces but which this parser refuses
to turn into an invented address.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ResolvNotFound"><code><strong class="tok-constructor">ResolvNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Resolv/#t-ResolvError" title="Effect.Resolv.ResolvError">ResolvError</a></code></li><li id="k-ResolvPermissionDenied"><code><strong class="tok-constructor">ResolvPermissionDenied</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Resolv/#t-ResolvError" title="Effect.Resolv.ResolvError">ResolvError</a></code></li><li id="k-ResolvInvalidArgument"><code><strong class="tok-constructor">ResolvInvalidArgument</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Resolv/#t-ResolvError" title="Effect.Resolv.ResolvError">ResolvError</a></code></li><li id="k-ResolvIoError"><code><strong class="tok-constructor">ResolvIoError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Resolv/#t-ResolvError" title="Effect.Resolv.ResolvError">ResolvError</a></code></li><li id="k-ResolvMalformedAnswer"><code><strong class="tok-constructor">ResolvMalformedAnswer</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Resolv/#t-ResolvError" title="Effect.Resolv.ResolvError">ResolvError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-answer-addresses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">answer-addresses</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code><span class="doc-sig-links"><a href="#v-answer-addresses" title="Permalink">#</a><a href="/library/source/Effect.Resolv/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

The addresses alone, in the resolver's order.

</div>
</article>

<article class="doc-entry" id="v-answer-family-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">answer-family-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpFamily" title="Data.IpAddr.IpFamily">IpFamily</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-answer-family-count" title="Permalink">#</a><a href="/library/source/Effect.Resolv/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

How many answers belong to `fam`.

</div>
</article>

<article class="doc-entry" id="v-answer-has-family">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">answer-has-family</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpFamily" title="Data.IpAddr.IpFamily">IpFamily</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-answer-has-family" title="Permalink">#</a><a href="/library/source/Effect.Resolv/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

Whether any answer belongs to `fam`.

</div>
</article>

<article class="doc-entry" id="v-resolve">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resolve</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Resolv/#t-ResolvError" title="Effect.Resolv.ResolvError">ResolvError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>))</code><span class="doc-sig-links"><a href="#v-resolve" title="Permalink">#</a><a href="/library/source/Effect.Resolv/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

Resolve `host` (UTF-8 host text) at `port` (0..=65535) into the
typed answer list, or the named error. Each `<ip>:<port>` line the
resolver returned is parsed into an `(IpAddr, Nat)` pair; a line
the parser cannot read is `ResolvMalformedAnswer` rather than a
dropped answer, so the well-formedness of the result is a checked
property and never a silent filter.

</div>
</article>

<article class="doc-entry" id="v-resolve-addresses">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">resolve-addresses</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Resolv/#t-ResolvError" title="Effect.Resolv.ResolvError">ResolvError</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a>)</code><span class="doc-sig-links"><a href="#v-resolve-addresses" title="Permalink">#</a><a href="/library/source/Effect.Resolv/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

The name-only convenience: `resolve host` at port 0 and the port
discarded, so a caller that wants just the addresses (Ruby's
`Resolv.getaddresses`) reads no port it does not use. The port is
0 because resolution is port-independent (the port is only echoed
onto each line); the addresses are the same for every port.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Resolv.cinch`](/library/source/Effect.Resolv/). Source SHA-256: `a965d0886171f2b2cb753518e0dc2dedf3442cb80fe6384d16b92804c61a05f0`.
