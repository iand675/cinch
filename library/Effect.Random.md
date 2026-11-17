# Effect.Random

Runtime entropy/random utilities with explicit effectful access.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Random`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Random.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Effect.Random](Effect.Random.md).

<details><summary>Behaviour and boundary notes from the source</summary>

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

# Types and constructors

<a id="type-52616e646f6d4572726f72"></a>

## `RandomError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">RandomError</span></code></pre>

Invalid bounds and intervals for otherwise well-formed random draws.

[Source](/reference-sources/Effect.Random.cinch) · line 37

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">EmptyBound</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Random</span>.<span class="tok-constructor">RandomError</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">EmptyRange</span> :: <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Random</span>.<span class="tok-constructor">RandomError</span></code></pre>

# Values

<a id="value-6865782d6f662d6279746573"></a>

## `hex-of-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">hex-of-bytes</span> :: (<span class="tok-constructor">Bytes</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Render two lowercase hexadecimal characters per input byte.
Digits are `0`..`9` then `a`..`f`: the
rendering a token, a digest echo or a temp name reads. A `String` is
UTF-8 over `Bytes`, and every hex character is ASCII, so the
conversion cannot fail (the `None` arm is unreachable and only keeps
it total, the `Data.Sym`/`Data.Complex` convention).

[Source](/reference-sources/Effect.Random.cinch) · line 93

<a id="value-72616e646f6d2d6279746573"></a>

## `random-bytes`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">random-bytes</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Bytes</span>)</code></pre>

Request exactly `n` owned bytes from the operating-system entropy source.
`n = 0` is the
legal empty request. Every draw here names this call; nothing holds
state between calls.

[Source](/reference-sources/Effect.Random.cinch) · line 51

<a id="value-72616e646f6d2d686578"></a>

## `random-hex`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">random-hex</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Data</span>.<span class="tok-constructor">String</span>.<span class="tok-constructor">String</span>)</code></pre>

Request `n` entropy bytes and render them as `2 * n` lowercase hex characters.

[Source](/reference-sources/Effect.Random.cinch) · line 102

<a id="value-72616e646f6d2d6e6174"></a>

## `random-nat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">random-nat</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Unit</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} <span class="tok-constructor">Nat</span>)</code></pre>

Draw one full machine word of entropy as a `Nat`, uniform over
`0 .. 2^64 - 1` (eight bytes, so no acceptance region is needed).
The draw is bound before it is read (the `draw-below` shape below):
one binding keeps the value read identical to the value drawn, and
it preserves the explicit lifetime of the buffer during decoding.

[Source](/reference-sources/Effect.Random.cinch) · line 59

<a id="value-72616e646f6d2d6e61742d62656c6f77"></a>

## `random-nat-below`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">random-nat-below</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Random</span>.<span class="tok-constructor">RandomError</span> <span class="tok-constructor">Nat</span>))</code></pre>

Draw a uniform value below `bound` by rejection sampling over
`byte-width bound` bytes, accepting only a whole multiple of the bound.
`bound = 0` is `Err EmptyBound`; bound one makes an empty provider
request and returns zero. The emptiness test makes the
bound the draw's `NonZero` divisor.

[Source](/reference-sources/Effect.Random.cinch) · line 69

<a id="value-72616e646f6d2d6e61742d696e2d72616e6765"></a>

## `random-nat-in-range`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">random-nat-in-range</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <span class="tok-constructor">Io</span> <span class="tok-variable">r</span>) =&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt;{<span class="tok-constructor">IO</span> <span class="tok-variable">r</span>} (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-constructor">Effect</span>.<span class="tok-constructor">Random</span>.<span class="tok-constructor">RandomError</span> <span class="tok-constructor">Nat</span>)))</code></pre>

Draw a uniform value in `lo .. hi - 1` (inclusive lower, exclusive
upper): the offset draw shifted by `lo`. An empty range (`hi <= lo`,
which also covers an inverted one) is `Err EmptyRange`; `hi - lo` is
only formed after that guard, so the subtraction cannot go below
zero. The inner `Err` arm is unreachable (`hi - lo` is at least 1
here) and only keeps the propagation total.

[Source](/reference-sources/Effect.Random.cinch) · line 80

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Random.cinch`. Source SHA-256: `75d3f181771dfa42e5c912f04c9a003e943b3e6eb7ebabe45eefa9e028be9216`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
