# Effect.Net

TCP client sockets, TCP listeners (listen/accept), UDP send, address values, named errors, and runtime effects.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Net`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Net.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Effect.Net](Effect.Net.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Net: TCP sockets and name resolution over the runtime's
`cinch_net_*` externs (`seed/runtime/src/net.rs`). A `Socket` is
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
codes (`seed/runtime/src/platform.rs`), which `net-error-code`
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

<a id="type-4c697374656e6572"></a>

## `Listener`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Listener</span></code></pre>

Wrap the runtime listener handle word.

[Source](/reference-sources/Effect.Net.cinch) · line 94

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Listener</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span>)</code></pre>

<a id="type-4e65744572726f72"></a>

## `NetError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">NetError</span></code></pre>

What went wrong, named. The constructors correspond one for one to
the platform layer's shared codes.

[Source](/reference-sources/Effect.Net.cinch) · line 75

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NetNotFound</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NetPermissionDenied</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NetInvalidArgument</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NetIoError</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span></code></pre>

<a id="type-536f636b6574"></a>

## `Socket`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Socket</span></code></pre>

An open TCP connection, as the runtime's socket handle word (kept
as the runtime's `I64`: it only ever goes back to the runtime).

[Source](/reference-sources/Effect.Net.cinch) · line 83

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Socket</span> :: (<span class="tok-constructor">I64</span> -&gt; <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span>)</code></pre>

# Values

<a id="value-6c697374656e65722d68616e646c65"></a>

## `listener-handle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">listener-handle</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

Return the underlying runtime handle word of a TCP listener.

[Source](/reference-sources/Effect.Net.cinch) · line 97

<a id="value-6e65742d616363657074"></a>

## `net-accept`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">net-accept</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span>))</code></pre>

Accept one incoming TCP connection on `listener`, returning a connected `Socket`.

[Source](/reference-sources/Effect.Net.cinch) · line 144

<a id="value-6e65742d636c6f7365"></a>

## `net-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">net-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close the socket: the handle is gone afterwards, so every later
operation on it is `Err NetInvalidArgument`.

[Source](/reference-sources/Effect.Net.cinch) · line 192

<a id="value-6e65742d636f6e6e656374"></a>

## `net-connect`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">net-connect</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span>)))</code></pre>

Connect to `host` (UTF-8 host text) at `port` (0..=65535) over TCP:
the open socket, or the named error. The name is resolved by the
platform resolver. A port past the `I64` range is invalid like any
other port past 65535.

[Source](/reference-sources/Effect.Net.cinch) · line 127

<a id="value-6e65742d6c697374656e"></a>

## `net-listen`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">net-listen</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span>)))</code></pre>

Bind a TCP listener on `host` (UTF-8 host text) and `port` (`0` for an
OS-assigned ephemeral port). Returns the bound `Listener` or a named error.

[Source](/reference-sources/Effect.Net.cinch) · line 136

<a id="value-6e65742d6c697374656e65722d61646472"></a>

## `net-listener-addr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">net-listener-addr</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>))</code></pre>

Return the bound local address (`<ip>:<port>`) of `listener` as UTF-8 text.

[Source](/reference-sources/Effect.Net.cinch) · line 151

<a id="value-6e65742d6c697374656e65722d636c6f7365"></a>

## `net-listener-close`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">net-listener-close</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Listener</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> <span class="tok-constructor">Unit</span>))</code></pre>

Close a TCP listener; subsequent operations on it return `Err NetInvalidArgument`.

[Source](/reference-sources/Effect.Net.cinch) · line 160

<a id="value-6e65742d72656164"></a>

## `net-read`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">net-read</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Read up to `max` bytes of the socket: `Ok` of what one platform
call produced (empty at the end of the stream), or the named error.

[Source](/reference-sources/Effect.Net.cinch) · line 176

<a id="value-6e65742d7265736f6c7665"></a>

## `net-resolve`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">net-resolve</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> <span class="tok-constructor">Bytes</span>)))</code></pre>

Resolve `host` (UTF-8 host text) at `port`: `Ok` of one
`<ip>:<port>` line per resolved address (each newline terminated,
an IPv6 address bracketed), or the named error. The empty result is
an empty buffer: the resolver answered with no address.

[Source](/reference-sources/Effect.Net.cinch) · line 202

<a id="value-6e65742d7564702d73656e64"></a>

## `net-udp-send`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">net-udp-send</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> <span class="tok-constructor">Nat</span>))))</code></pre>

Send one UDP datagram containing `payload` to `(host, port)`.

[Source](/reference-sources/Effect.Net.cinch) · line 167

<a id="value-6e65742d7772697465"></a>

## `net-write`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">net-write</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; (<span class="tok-constructor">Bytes</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">NetError</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Write `bs` to the socket: the byte count (the whole buffer goes),
or the named error.

[Source](/reference-sources/Effect.Net.cinch) · line 184

<a id="value-736f636b65742d68616e646c65"></a>

## `socket-handle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">socket-handle</span> :: (<span class="tok-constructor">Effect</span>.<span class="tok-constructor">Net</span>.<span class="tok-constructor">Socket</span> -&gt; <span class="tok-constructor">I64</span>)</code></pre>

The handle word of a socket, for the layers above this one.

[Source](/reference-sources/Effect.Net.cinch) · line 87

# Reference provenance

Generated from the Rust compiler's checked exports and `seed/prelude/Effect/Net.cinch`. Source SHA-256: `126fd1f6a1b7172b7cbac70dbb04431c1a43d2f12ff1dd6ddd2c75c9b0637bf4`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
