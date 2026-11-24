# Effect.Random

Runtime entropy/random utilities with explicit effectful access.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Random</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Random/source/">View source</a><a href="/packages/search/?q=%2BEffect.Random">Search module</a><a href="/reference-sources/Effect.Random.cinch" download="Effect.Random.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Effect.Random`: [Effect.Random](Effect.Random.md).

```sh
cp seed/prelude/Effect/Random.cinch ./Effect.Random.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Cryptographic random bytes, uniform integer draws, and hexadecimal tokens.

Every public draw requires an explicit `Io r` capability. Successful byte
requests return a fresh owned buffer of exactly the requested length.
Independent draws may have equal contents; buffer ownership is independent.

Nonempty Linux draws use the initialized kernel generator, with the device
provider retained for kernels without that interface. Empty draws preserve
the device-open check. Other hosted platforms retain their device provider.
Provider and allocation failures belong to the runtime boundary; they are
not represented by `RandomError`, including for an empty byte request.

Bounded draws reject excess values before taking a remainder, avoiding
modulo bias. An empty bound returns `Err EmptyBound`; an empty or inverted
interval returns `Err EmptyRange`. Bound one returns zero after an empty draw.
`random-nat` draws one full 64-bit word. Hexadecimal output uses lowercase.

This module has no seed or replay state. Deterministic generators belong
to separate APIs; draws use the operating-system provider.

</details>

<details class="doc-synopsis" open><summary>Synopsis (7 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-RandomError">RandomError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RandomError</strong></code></li><li><a class="syn-name" href="#v-hex-of-bytes">hex-of-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">hex-of-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-random-bytes">random-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">random-bytes</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-random-hex">random-hex</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">random-hex</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-random-nat">random-nat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">random-nat</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-random-nat-below">random-nat-below</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">random-nat-below</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Random/#t-RandomError" title="Effect.Random.RandomError">RandomError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-random-nat-in-range">random-nat-in-range</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">random-nat-in-range</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Random/#t-RandomError" title="Effect.Random.RandomError">RandomError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-RandomError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">RandomError</strong></code><span class="doc-sig-links"><a href="#t-RandomError" title="Permalink">#</a><a href="/packages/stdlib/Effect.Random/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Invalid bounds and intervals for otherwise well-formed random draws.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-EmptyBound"><code><strong class="tok-constructor">EmptyBound</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Random/#t-RandomError" title="Effect.Random.RandomError">RandomError</a></code></li><li id="k-EmptyRange"><code><strong class="tok-constructor">EmptyRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Random/#t-RandomError" title="Effect.Random.RandomError">RandomError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-hex-of-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">hex-of-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-hex-of-bytes" title="Permalink">#</a><a href="/packages/stdlib/Effect.Random/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

Render two lowercase hexadecimal characters per input byte.
Digits are `0`..`9` then `a`..`f`: the
rendering a token, a digest echo or a temp name reads. A `String` is
UTF-8 over `Bytes`, and every hex character is ASCII, so the
conversion cannot fail (the `None` arm is unreachable and only keeps
it total, the `Data.Sym`/`Data.Complex` convention).

</div>
</article>

<article class="doc-entry" id="v-random-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">random-bytes</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-random-bytes" title="Permalink">#</a><a href="/packages/stdlib/Effect.Random/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Request exactly `n` owned bytes from the operating-system entropy source.
`n = 0` is the
legal empty request. Every draw here names this call; nothing holds
state between calls.

</div>
</article>

<article class="doc-entry" id="v-random-hex">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">random-hex</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-random-hex" title="Permalink">#</a><a href="/packages/stdlib/Effect.Random/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

Request `n` entropy bytes and render them as `2 * n` lowercase hex characters.

</div>
</article>

<article class="doc-entry" id="v-random-nat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">random-nat</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-random-nat" title="Permalink">#</a><a href="/packages/stdlib/Effect.Random/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Draw one full machine word of entropy as a `Nat`, uniform over
`0 .. 2^64 - 1` (eight bytes, so no acceptance region is needed).
The draw is bound before it is read (the `draw-below` shape below):
one binding keeps the value read identical to the value drawn, and
it preserves the explicit lifetime of the buffer during decoding.

</div>
</article>

<article class="doc-entry" id="v-random-nat-below">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">random-nat-below</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Random/#t-RandomError" title="Effect.Random.RandomError">RandomError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-random-nat-below" title="Permalink">#</a><a href="/packages/stdlib/Effect.Random/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Draw a uniform value below `bound` by rejection sampling over
`byte-width bound` bytes, accepting only a whole multiple of the bound.
`bound = 0` is `Err EmptyBound`; bound one makes an empty provider
request and returns zero. The emptiness test makes the
bound the draw's `NonZero` divisor.

</div>
</article>

<article class="doc-entry" id="v-random-nat-in-range">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">random-nat-in-range</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span>{<a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="IO">IO</a> r} <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Random/#t-RandomError" title="Effect.Random.RandomError">RandomError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-random-nat-in-range" title="Permalink">#</a><a href="/packages/stdlib/Effect.Random/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

Draw a uniform value in `lo .. hi - 1` (inclusive lower, exclusive
upper): the offset draw shifted by `lo`. An empty range (`hi <= lo`,
which also covers an inverted one) is `Err EmptyRange`; `hi - lo` is
only formed after that guard, so the subtraction cannot go below
zero. The inner `Err` arm is unreachable (`hi - lo` is at least 1
here) and only keeps the propagation total.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Random.cinch`](/packages/stdlib/Effect.Random/source/). Source SHA-256: `75d3f181771dfa42e5c912f04c9a003e943b3e6eb7ebabe45eefa9e028be9216`.
