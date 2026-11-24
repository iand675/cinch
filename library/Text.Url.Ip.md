# Text.Url.Ip

URL host IP-literal and port validation with explicit IPv6 and zone rules.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Url.Ip</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Url.Ip/source/">View source</a><a href="/packages/search/?q=%2BText.Url.Ip">Search module</a><a href="/reference-sources/Text.Url.Ip.cinch" download="Text.Url.Ip.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Text.Url.Scan](Text.Url.Scan.md).

<details class="doc-notes"><summary>Module notes</summary>

Strict bracketed IPv6 syntax, including terminal IPv4 and nonempty zones.
Validation preserves spelling and performs no network resolution.

</details>

<details class="doc-synopsis" open><summary>Synopsis (1 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-url-valid-ipv6">url-valid-ipv6</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">url-valid-ipv6</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-url-valid-ipv6">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">url-valid-ipv6</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-url-valid-ipv6" title="Permalink">#</a><a href="/packages/stdlib/Text.Url.Ip/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Validate an unbracketed decoded IPv6 address, allowing one nonempty percent zone.

</div>
</article>
