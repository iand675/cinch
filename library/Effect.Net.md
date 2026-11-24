# Effect.Net

Socket operations with explicit handles, address values, errors, and runtime effects.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Net</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Net/">View source</a><a href="/library/search/?q=%2BEffect.Net">Search module</a><a href="/reference-sources/Effect.Net.cinch" download="Effect.Net.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Effect.Net`: [Effect.Net](Effect.Net.md).

```sh
cp seed/prelude/Effect/Net.cinch ./Effect.Net.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-NetError">NetError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">NetError</strong></code></li><li><a class="syn-name" href="#t-Socket">Socket</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Socket</strong></code></li><li><a class="syn-name" href="#v-net-close">net-close</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">net-close</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-net-connect">net-connect</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">net-connect</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a></code></li><li><a class="syn-name" href="#v-net-read">net-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">net-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-net-resolve">net-resolve</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">net-resolve</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-net-write">net-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">net-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-socket-handle">socket-handle</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">socket-handle</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-NetError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">NetError</strong></code><span class="doc-sig-links"><a href="#t-NetError" title="Permalink">#</a><a href="/library/source/Effect.Net/#L73" title="View source at line 73">Source</a></span></div>
<div class="doc-body">

What went wrong, named. The constructors correspond one for one to
the platform layer's shared codes.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-NetNotFound"><code><strong class="tok-constructor">NetNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a></code></li><li id="k-NetPermissionDenied"><code><strong class="tok-constructor">NetPermissionDenied</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a></code></li><li id="k-NetInvalidArgument"><code><strong class="tok-constructor">NetInvalidArgument</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a></code></li><li id="k-NetIoError"><code><strong class="tok-constructor">NetIoError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Socket">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Socket</strong></code><span class="doc-sig-links"><a href="#t-Socket" title="Permalink">#</a><a href="/library/source/Effect.Net/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

An open TCP connection, as the runtime's socket handle word (kept
as the runtime's `I64`: it only ever goes back to the runtime).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Socket"><code><strong class="tok-constructor">Socket</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-net-close">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">net-close</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-net-close" title="Permalink">#</a><a href="/library/source/Effect.Net/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Close the socket: the handle is gone afterwards, so every later
operation on it is `Err NetInvalidArgument`.

</div>
</article>

<article class="doc-entry" id="v-net-connect">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">net-connect</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a></code><span class="doc-sig-links"><a href="#v-net-connect" title="Permalink">#</a><a href="/library/source/Effect.Net/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

Connect to `host` (UTF-8 host text) at `port` (0..=65535) over TCP:
the open socket, or the named error. The name is resolved by the
platform resolver. A port past the `I64` range is invalid like any
other port past 65535.

</div>
</article>

<article class="doc-entry" id="v-net-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">net-read</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-net-read" title="Permalink">#</a><a href="/library/source/Effect.Net/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

Read up to `max` bytes of the socket: `Ok` of what one platform
call produced (empty at the end of the stream), or the named error.

</div>
</article>

<article class="doc-entry" id="v-net-resolve">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">net-resolve</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-net-resolve" title="Permalink">#</a><a href="/library/source/Effect.Net/#L145" title="View source at line 145">Source</a></span></div>
<div class="doc-body">

Resolve `host` (UTF-8 host text) at `port`: `Ok` of one
`<ip>:<port>` line per resolved address (each newline terminated,
an IPv6 address bracketed), or the named error. The empty result is
an empty buffer: the resolver answered with no address.

</div>
</article>

<article class="doc-entry" id="v-net-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">net-write</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-NetError" title="Effect.Net.NetError">NetError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-net-write" title="Permalink">#</a><a href="/library/source/Effect.Net/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

Write `bs` to the socket: the byte count (the whole buffer goes),
or the named error.

</div>
</article>

<article class="doc-entry" id="v-socket-handle">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">socket-handle</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Net/#t-Socket" title="Effect.Net.Socket">Socket</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-socket-handle" title="Permalink">#</a><a href="/library/source/Effect.Net/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

The handle word of a socket, for the layers above this one.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Net.cinch`](/library/source/Effect.Net/). Source SHA-256: `9bac11640aa56817a5722b1336568f7d1ad80e034c105313934b44c8a7e9cc30`.
