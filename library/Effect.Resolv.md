# Effect.Resolv

Hostname/address resolution through the runtime, with named failure results.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Resolv`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Resolv.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.IpAddr](Data.IpAddr.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Net](Effect.Net.md), [Text.Scan](Text.Scan.md).

Non-embedded source closure: [Data.IpAddr](Data.IpAddr.md), [Effect.Net](Effect.Net.md), [Effect.Resolv](Effect.Resolv.md), [Text.Scan](Text.Scan.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Resolv: name resolution as typed addresses, over the
resolution intrinsic (`Effect.Net`'s `net-resolve`, the runtime's
`cinch_net_resolve` in `compiler/runtime/src/net.rs`). The boundary
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

# Types and constructors

<a id="type-5265736f6c764572726f72"></a>

## `ResolvError`

```cinch
data ResolvError
```

What went wrong, named. The four shared platform names are the
boundary's own vocabulary (`Effect.Net`'s `NetError`, the codes
`compiler/runtime/src/platform.rs` shares across the externs);
`ResolvMalformedAnswer` is this home's extra name for a line the
boundary returned that is not a well-formed `<ip>:<port>`, which a
well-behaved boundary never produces but which this parser refuses
to turn into an invented address.

[Source](/reference-sources/Effect.Resolv.cinch) · line 82

```cinch
ResolvNotFound :: ResolvError
```

```cinch
ResolvPermissionDenied :: ResolvError
```

```cinch
ResolvInvalidArgument :: ResolvError
```

```cinch
ResolvIoError :: ResolvError
```

```cinch
ResolvMalformedAnswer :: ResolvError
```

# Values

<a id="value-616e737765722d616464726573736573"></a>

## `answer-addresses`

```cinch
answer-addresses :: (List (IpAddr, Nat) -> List IpAddr)
```

Classification over an answer list (Ruby's `IPAddr#ipv4?`/
`#ipv6?` shape, one total `IpFamily` witness through
`Data.IpAddr`'s `ip-family`).

The addresses alone, in the resolver's order.

[Source](/reference-sources/Effect.Resolv.cinch) · line 127

<a id="value-616e737765722d66616d696c792d636f756e74"></a>

## `answer-family-count`

```cinch
answer-family-count :: (IpFamily -> (List (IpAddr, Nat) -> Nat))
```

How many answers belong to `fam`.

[Source](/reference-sources/Effect.Resolv.cinch) · line 131

<a id="value-616e737765722d6861732d66616d696c79"></a>

## `answer-has-family`

```cinch
answer-has-family :: (IpFamily -> (List (IpAddr, Nat) -> Bool))
```

Whether any answer belongs to `fam`.

[Source](/reference-sources/Effect.Resolv.cinch) · line 140

<a id="value-7265736f6c7665"></a>

## `resolve`

```cinch
resolve :: forall r . using (io :: Io r) => (String -> (Nat ->{IO r} Result ResolvError (List (IpAddr, Nat))))
```

Resolve `host` (UTF-8 host text) at `port` (0..=65535) into the
typed answer list, or the named error. Each `<ip>:<port>` line the
resolver returned is parsed into an `(IpAddr, Nat)` pair; a line
the parser cannot read is `ResolvMalformedAnswer` rather than a
dropped answer, so the well-formedness of the result is a checked
property and never a silent filter.

[Source](/reference-sources/Effect.Resolv.cinch) · line 104

<a id="value-7265736f6c76652d616464726573736573"></a>

## `resolve-addresses`

```cinch
resolve-addresses :: forall r . using (io :: Io r) => (String ->{IO r} Result ResolvError (List IpAddr))
```

The name-only convenience: `resolve host` at port 0 and the port
discarded, so a caller that wants just the addresses (Ruby's
`Resolv.getaddresses`) reads no port it does not use. The port is
0 because resolution is port-independent (the port is only echoed
onto each line); the addresses are the same for every port.

[Source](/reference-sources/Effect.Resolv.cinch) · line 114

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Resolv.cinch`. Source SHA-256: `7430e1f878fff227f9adfa2717c511959737072897e99c5df85e16294a6d4fa0`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
