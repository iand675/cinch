# Data.IpAddr

Validated IP addresses, parsing/rendering, and address-family vocabulary.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.IpAddr`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.IpAddr.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Data.IpAddr](Data.IpAddr.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

The law shape (exercised in compiler/tests/lib_uri_ipaddr.rs):
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

# Types and constructors

<a id="type-497041646472"></a>

## `IpAddr`

```cinch
data IpAddr
```

[Source](/reference-sources/Data.IpAddr.cinch) · line 82

```cinch
V4Addr :: (Ipv4 -> IpAddr)
```

```cinch
V6Addr :: (Ipv6 -> IpAddr)
```

<a id="type-497046616d696c79"></a>

## `IpFamily`

```cinch
data IpFamily
```

The two families and the sum over them.

Which family an address belongs to (Ruby spells this as a pair of
predicates; one total witness keeps the answer exhaustive).

[Source](/reference-sources/Data.IpAddr.cinch) · line 69

```cinch
Ipv4Family :: IpFamily
```

```cinch
Ipv6Family :: IpFamily
```

<a id="type-49707634"></a>

## `Ipv4`

```cinch
data Ipv4
```

Four octets, each 0..=255. The field type carries the bound, so the
byte form needs no check and no constructor site can exceed it.

[Source](/reference-sources/Data.IpAddr.cinch) · line 75

<a id="type-49707636"></a>

## `Ipv6`

```cinch
data Ipv6
```

Exactly eight 16-bit groups, most significant first.

[Source](/reference-sources/Data.IpAddr.cinch) · line 79

# Values

<a id="value-69702d6279746573"></a>

## `ip-bytes`

```cinch
ip-bytes :: (IpAddr -> Bytes)
```

[Source](/reference-sources/Data.IpAddr.cinch) · line 420

<a id="value-69702d66616d696c79"></a>

## `ip-family`

```cinch
ip-family :: (IpAddr -> IpFamily)
```

The sum over the families.

[Source](/reference-sources/Data.IpAddr.cinch) · line 410

<a id="value-69702d7061727365"></a>

## `ip-parse`

```cinch
ip-parse :: (String -> Option IpAddr)
```

[Source](/reference-sources/Data.IpAddr.cinch) · line 425

<a id="value-69702d746f2d737472696e67"></a>

## `ip-to-string`

```cinch
ip-to-string :: (IpAddr -> String)
```

[Source](/reference-sources/Data.IpAddr.cinch) · line 415

<a id="value-697076342d6279746573"></a>

## `ipv4-bytes`

```cinch
ipv4-bytes :: (Ipv4 -> Bytes)
```

[Source](/reference-sources/Data.IpAddr.cinch) · line 194

<a id="value-697076342d66726f6d2d6f6374657473"></a>

## `ipv4-from-octets`

```cinch
ipv4-from-octets :: (Nat -> (Nat -> (Nat -> (Nat -> Option Ipv4))))
```

The smart constructor: octets outside 0..=255 are refused, so no
caller can build an `Ipv4` the parser would never produce.

[Source](/reference-sources/Data.IpAddr.cinch) · line 179

<a id="value-697076342d7061727365"></a>

## `ipv4-parse`

```cinch
ipv4-parse :: (String -> Option Ipv4)
```

[Source](/reference-sources/Data.IpAddr.cinch) · line 171

<a id="value-697076342d746f2d737472696e67"></a>

## `ipv4-to-string`

```cinch
ipv4-to-string :: (Ipv4 -> String)
```

[Source](/reference-sources/Data.IpAddr.cinch) · line 184

<a id="value-697076362d6279746573"></a>

## `ipv6-bytes`

```cinch
ipv6-bytes :: (Ipv6 -> Bytes)
```

[Source](/reference-sources/Data.IpAddr.cinch) · line 375

<a id="value-697076362d66726f6d2d67726f757073"></a>

## `ipv6-from-groups`

```cinch
ipv6-from-groups :: (List Nat -> Option Ipv6)
```

The smart constructor: exactly eight groups, each 0..=65535.

[Source](/reference-sources/Data.IpAddr.cinch) · line 304

<a id="value-697076362d7061727365"></a>

## `ipv6-parse`

```cinch
ipv6-parse :: (String -> Option Ipv6)
```

[Source](/reference-sources/Data.IpAddr.cinch) · line 291

<a id="value-697076362d746f2d737472696e67"></a>

## `ipv6-to-string`

```cinch
ipv6-to-string :: (Ipv6 -> String)
```

Canonical rendering: lower-case hex, the chosen zero run spelled
`::`, at most one `::` per address.

[Source](/reference-sources/Data.IpAddr.cinch) · line 356

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.IpAddr.Eq_IpAddr` | `Eq IpAddr` | Implicit |
| `Data.IpAddr.Eq_Ipv4` | `Eq Ipv4` | Implicit |
| `Data.IpAddr.Eq_Ipv6` | `Eq Ipv6` | Implicit |
| `Data.IpAddr.Ord_IpAddr` | `Ord IpAddr` | Implicit |
| `Data.IpAddr.Ord_Ipv4` | `Ord Ipv4` | Implicit |
| `Data.IpAddr.Ord_Ipv6` | `Ord Ipv6` | Implicit |

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/IpAddr.cinch`. Source SHA-256: `1315365aa804385982fcb718a6772635be4b70cec5b62aeb30e3478ffa8b39ae`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
