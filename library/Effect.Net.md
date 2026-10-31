# Effect.Net

Socket operations with explicit handles, address values, errors, and runtime effects.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Net`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Net.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Effect.Net](Effect.Net.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Net: TCP sockets and name resolution over the runtime's
`cinch_net_*` externs (`compiler/runtime/src/net.rs`). A `Socket` is
a live connection the runtime's handle table owns; `net-read` and
`net-write` move bytes over it; `net-resolve` is the DNS primitive
(one `<ip>:<port>` line per resolved address), which the name
resolution home presents.

The operations run under the `Io r` capability (decision D45's
determinism rule: a socket is consulted when a program with the
capability calls it, never ambient at compile time, so a build
cannot depend on what the network answered). Every failure is a
NAMED `NetError` value in a `Result` (the house style), never a
fault: the boundary's raw word carries the shared platform error
codes (`compiler/runtime/src/platform.rs`), which `net-error-code`
maps one for one.

The boundary words themselves are the `cinch.runtime.net*` units
(machine scalars and `Bytes`; see the runtime module's header for the
return-code convention). Those units are pure at the boundary, as
every `cinch.runtime.*` intrinsic is (a primitive's declared type is
its `sig()`, which carries no effect row); this module's wrappers are
where the `IO r` discipline lives, exactly as the primitive-type
home wraps `bytesNew`.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary, and a socket is a runtime capability.

THE LAW SHAPE (what the slice's tests pin):
  1. round trip: bytes written to a socket are what the peer reads,
     and `net-read` answers what the peer wrote (a stream is a byte
     sequence, not a message boundary: a short read is legal, and a
     caller that wants a length loops);
  2. end of stream: `net-read` answers `Ok` of the EMPTY buffer once
     the peer closed its side, and that is not an error;
  3. the handle is single-use on close: every operation after
     `net-close` is `Err NetInvalidArgument`, and so is a handle the
     caller invented;
  4. refusal is a value: a connection nothing listens on answers
     `Err` (the documented code), never a fault and never a hang on a
     name that does not resolve;
  5. `net-resolve` answers at least one `<ip>:<port>` line for a name
     the platform resolves, each line newline terminated.

Named refusals (boundaries this module deliberately does not cross):
  plain TCP only: no TLS (a caller that needs it wraps the stream),
    no UDP, and no listening or accepting (this slice is the client
    side);
  no timeouts: connect and read block until the platform call
    returns and do not observe the concurrency cancellation token of
    section 4 (a cancellation-aware read needs the poll shim this
    slice does not ship);
  UTF-8 host text only: a host crosses as text, so a name that is not
    UTF-8 is not expressible here;
  no address parsing: `net-resolve` hands back the resolver's own
    `<ip>:<port>` lines; an `IPAddr` value type over them is the
    address home's work, not this module's;
  one read, not a filled length: `net-read` returns what one platform
    call produced, never a loop to fill the requested size.

</details>

# Types and constructors

<a id="type-4e65744572726f72"></a>

## `NetError`

```cinch
data NetError
```

What went wrong, named. The constructors correspond one for one to
the platform layer's shared codes.

[Source](/reference-sources/Effect.Net.cinch) · line 73

```cinch
NetNotFound :: NetError
```

```cinch
NetPermissionDenied :: NetError
```

```cinch
NetInvalidArgument :: NetError
```

```cinch
NetIoError :: NetError
```

<a id="type-536f636b6574"></a>

## `Socket`

```cinch
data Socket
```

An open TCP connection, as the runtime's socket handle word (kept
as the runtime's `I64`: it only ever goes back to the runtime).

[Source](/reference-sources/Effect.Net.cinch) · line 81

```cinch
Socket :: (I64 -> Socket)
```

An open TCP connection, as the runtime's socket handle word (kept
as the runtime's `I64`: it only ever goes back to the runtime).

# Values

<a id="value-6e65742d636c6f7365"></a>

## `net-close`

```cinch
net-close :: forall r . using (io :: Io r) => (Socket ->{IO r} Result NetError Unit)
```

Close the socket: the handle is gone afterwards, so every later
operation on it is `Err NetInvalidArgument`.

[Source](/reference-sources/Effect.Net.cinch) · line 135

<a id="value-6e65742d636f6e6e656374"></a>

## `net-connect`

```cinch
net-connect :: forall r . using (io :: Io r) => (String -> (Nat ->{IO r} Result NetError Socket))
```

Connect to `host` (UTF-8 host text) at `port` (0..=65535) over TCP:
the open socket, or the named error. The name is resolved by the
platform resolver. A port past the `I64` range is invalid like any
other port past 65535.

[Source](/reference-sources/Effect.Net.cinch) · line 110

<a id="value-6e65742d72656164"></a>

## `net-read`

```cinch
net-read :: forall r . using (io :: Io r) => (Socket -> (Nat ->{IO r} Result NetError Bytes))
```

Read up to `max` bytes of the socket: `Ok` of what one platform
call produced (empty at the end of the stream), or the named error.

[Source](/reference-sources/Effect.Net.cinch) · line 119

<a id="value-6e65742d7265736f6c7665"></a>

## `net-resolve`

```cinch
net-resolve :: forall r . using (io :: Io r) => (String -> (Nat ->{IO r} Result NetError Bytes))
```

Resolve `host` (UTF-8 host text) at `port`: `Ok` of one
`<ip>:<port>` line per resolved address (each newline terminated,
an IPv6 address bracketed), or the named error. The empty result is
an empty buffer: the resolver answered with no address.

[Source](/reference-sources/Effect.Net.cinch) · line 145

<a id="value-6e65742d7772697465"></a>

## `net-write`

```cinch
net-write :: forall r . using (io :: Io r) => (Socket -> (Bytes ->{IO r} Result NetError Nat))
```

Write `bs` to the socket: the byte count (the whole buffer goes),
or the named error.

[Source](/reference-sources/Effect.Net.cinch) · line 127

<a id="value-736f636b65742d68616e646c65"></a>

## `socket-handle`

```cinch
socket-handle :: (Socket -> I64)
```

The handle word of a socket, for the layers above this one.

[Source](/reference-sources/Effect.Net.cinch) · line 85

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Net.cinch`. Source SHA-256: `a2cb8234289a30c321ff66c01032badecda44207e910c715466821db78321a46`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
