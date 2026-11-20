# Text.Url.Ip

URL host IP-literal and port validation with explicit IPv6 and zone rules.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Url.Ip`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Url.Ip.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Text.Url.Scan](Text.Url.Scan.md).

Non-embedded source closure: [Text.Url.Ip](Text.Url.Ip.md), [Text.Url.Scan](Text.Url.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Strict bracketed IPv6 syntax, including terminal IPv4 and nonempty zones.
Validation preserves spelling and performs no network resolution.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-75726c2d76616c69642d69707636"></a>

## `url-valid-ipv6`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">url-valid-ipv6</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

Validate an unbracketed decoded IPv6 address, allowing one nonempty percent zone.

[Source](/reference-sources/Text.Url.Ip.cinch) · line 59

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Text/Url/Ip.cinch`. Source SHA-256: `8bbe1f04258c0bece6823836b336e71009122463e79eb8eef923ff4e550a55be`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
