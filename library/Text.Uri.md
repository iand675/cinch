# Text.Uri

URI parsing, rendering, components, and validation/error vocabulary.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Text.Uri`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Text.Uri.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.IpAddr](Data.IpAddr.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.IpAddr](Data.IpAddr.md), [Text.Uri](Text.Uri.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

The law shape (exercised in compiler/tests/lib_uri_ipaddr.rs):
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

# Types and constructors

<a id="type-557269"></a>

## `Uri`

```cinch
data Uri
```

The reference and its components.

The seven RFC 3986 components, each `None` when absent (an absent
component and an empty one differ: `http://x` has no query, `http://x?`
has an empty one, and the round trip needs the difference). `host`
carries the authority's host text as written, brackets included, so
`uri-to-string` reprints an IPv6 literal without re-deciding its
brackets; a present-but-empty `host` is the empty authority of
`file:///x`.

[Source](/reference-sources/Text.Uri.cinch) · line 88

```cinch
MkUri :: (Option String -> (Option String -> (Option String -> (Option Nat -> (String -> (Option String -> (Option String -> Uri)))))))
```

<a id="type-5572694572726f72"></a>

## `UriError`

```cinch
data UriError
```

Errors: every malformed reference names the rule it broke.

[Source](/reference-sources/Text.Uri.cinch) · line 124

```cinch
BadPercentEscape :: UriError
```

```cinch
BadScheme :: UriError
```

```cinch
BadPort :: UriError
```

```cinch
BadIpv6Host :: UriError
```

```cinch
BadUtf8 :: UriError
```

# Values

<a id="value-72656e6465722d7572692d6572726f72"></a>

## `render-uri-error`

```cinch
render-uri-error :: (UriError -> String)
```

[Source](/reference-sources/Text.Uri.cinch) · line 131

<a id="value-7572692d6465636f6465"></a>

## `uri-decode`

```cinch
uri-decode :: (String -> Result UriError String)
```

[Source](/reference-sources/Text.Uri.cinch) · line 255

<a id="value-7572692d656e636f6465"></a>

## `uri-encode`

```cinch
uri-encode :: (String -> String)
```

[Source](/reference-sources/Text.Uri.cinch) · line 239

<a id="value-7572692d667261676d656e74"></a>

## `uri-fragment`

```cinch
uri-fragment :: (Uri -> Option String)
```

[Source](/reference-sources/Text.Uri.cinch) · line 115

<a id="value-7572692d686f7374"></a>

## `uri-host`

```cinch
uri-host :: (Uri -> Option String)
```

[Source](/reference-sources/Text.Uri.cinch) · line 99

<a id="value-7572692d686f73742d6970"></a>

## `uri-host-ip`

```cinch
uri-host-ip :: (Uri -> Option IpAddr)
```

The address a host names, when it names one: a bracketed literal is
an IPv6 address, a bare host that parses as IPv4 is one, and any
other host (a registered name, or an unbracketed IPv6) is `None`.

[Source](/reference-sources/Text.Uri.cinch) · line 581

<a id="value-7572692d6e6f726d616c697a65"></a>

## `uri-normalize`

```cinch
uri-normalize :: (Uri -> Uri)
```

[Source](/reference-sources/Text.Uri.cinch) · line 542

<a id="value-7572692d7061727365"></a>

## `uri-parse`

```cinch
uri-parse :: (String -> Result UriError Uri)
```

Parsing (RFC 3986 appendix B's grammar, with the checks the
grammar needs and the appendix's regex cannot carry).

[Source](/reference-sources/Text.Uri.cinch) · line 294

<a id="value-7572692d70617468"></a>

## `uri-path`

```cinch
uri-path :: (Uri -> String)
```

[Source](/reference-sources/Text.Uri.cinch) · line 107

<a id="value-7572692d706f7274"></a>

## `uri-port`

```cinch
uri-port :: (Uri -> Option Nat)
```

[Source](/reference-sources/Text.Uri.cinch) · line 103

<a id="value-7572692d7175657279"></a>

## `uri-query`

```cinch
uri-query :: (Uri -> Option String)
```

[Source](/reference-sources/Text.Uri.cinch) · line 111

<a id="value-7572692d7265736f6c7665"></a>

## `uri-resolve`

```cinch
uri-resolve :: (Uri -> (Uri -> Uri))
```

[Source](/reference-sources/Text.Uri.cinch) · line 652

<a id="value-7572692d736368656d65"></a>

## `uri-scheme`

```cinch
uri-scheme :: (Uri -> Option String)
```

[Source](/reference-sources/Text.Uri.cinch) · line 91

<a id="value-7572692d746f2d737472696e67"></a>

## `uri-to-string`

```cinch
uri-to-string :: (Uri -> String)
```

Printing: the exact inverse of parsing (the round-trip law).

[Source](/reference-sources/Text.Uri.cinch) · line 451

<a id="value-7572692d75736572696e666f"></a>

## `uri-userinfo`

```cinch
uri-userinfo :: (Uri -> Option String)
```

[Source](/reference-sources/Text.Uri.cinch) · line 95

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Text/Uri.cinch`. Source SHA-256: `b1b0e152c068b9b4ef4d47b03b84502cd0fc2b506608bdd006b87405934c50f4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
