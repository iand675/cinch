# Effect.Random

Runtime entropy/random utilities with explicit effectful access.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Effect.Random`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Effect.Random.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Bytes](Data.Bytes.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

Non-embedded source closure: [Effect.Random](Effect.Random.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Effect.Random: OS entropy, the uniform draws built on it, and the
hex rendering a token or a temp name wants (P34, the platform layer:
Ruby's `SecureRandom` reduced to what a safe language can state).

`random-bytes` is `n` fresh bytes from the OS entropy source, exactly
the requested length, and every draw below is built from it. An
illegal bound is a NAMED `RandomError` value in a `Result` (the house
style of Effect.Fs and Effect.Process), never a fault.

The entropy word is the platform layer's own read of `/dev/urandom`
into a fresh boxed buffer (`compiler/runtime/src/platform.rs`, the
`cinch_random_bytes` extern), declared through the compiler's
recognized intrinsic `cinch.runtime.randomBytes`. The checker pins
that intrinsic's signature exactly (`Nat -> Bytes`; a mismatch is an
error naming both types), and it is a pure-typed leaf: the word it
returns IS the boxed buffer, so no marshalling sits between the
runtime and the value. The capability therefore lives on this
module's surface, where decision D45 wants it: the OS is consulted at
runtime under `Io r`, never ambient at compile time, and a staged
body has no door to it (the intrinsic itself stays private to this
module, so every base-library use goes through a function whose row
carries the capability; a caller that redeclares the intrinsic in its
own module takes on that statement itself).

LAW SHAPE (what the surface promises, and what the module's test
holds it to):
  length:    `random-bytes n` has exactly `n` bytes. `n = 0` is legal
             and is the empty buffer: the runtime boxes an empty
             `Bytes` and the read of zero bytes succeeds, so an empty
             request has no failure mode to report, and pinning that
             choice here is the point (a caller doing arithmetic on
             lengths never has to special-case it);
  freshness: two draws of the same length differ, because each call
             reads the OS source afresh and nothing is cached between
             calls. The test may pin the inequality of two 32-byte
             draws: a collision has probability 2^-256, which is not a
             plausible event in the life of the universe;
  range:     `random-nat-below bound` is uniform on `0 .. bound - 1`;
             `random-nat-in-range lo hi` is uniform on `lo .. hi - 1`;
             `random-nat` is uniform over one full 64-bit word
             (`0 .. 2^64 - 1`);
  totality:  `bound = 0` is `Err EmptyBound`, and an empty or
             inverted range is `Err EmptyRange`; neither faults, and
             `bound = 1` is 0 without consuming entropy.

Modulo bias, and why the draw is a rejection loop rather than one
remainder: `v mod bound` over a uniform `v < 2^k` is biased whenever
`2^k` is not a multiple of `bound`, because the low residues have one
more preimage than the high ones. So the draw is taken over exactly
`w = byte-width bound` bytes, the smallest width with `256^w >=
bound`, and the acceptance region is `bound`'s largest multiple at or
below `256^w`: `limit = 256^w - (256^w mod bound)`. A draw `v < limit`
is uniform on `0 .. limit - 1`, and `limit mod bound = 0`, so
`v mod bound` is uniform on `0 .. bound - 1`. A draw at or above
`limit` is discarded and the loop draws again. The accepted region is
always more than half of the drawn range (`limit` is a multiple of
`bound` and at least one multiple fits by the width's definition; the
worst case is a bound just above a power of 256, where 129 of the 256
values of a one-byte region are accepted), so the expected number of
draws is below two. `bound = 1` has `w = 0`: no bytes are drawn and
the answer is 0. `bytes-to-nat` reads the whole draw (exactly `w`
bytes) big-endian, which is what makes `v` uniform over `0 .. 256^w
- 1` (the byte order only fixes the numeric value, not its
distribution).

Named refusals (boundaries this module deliberately does not cross):
  seeding, and reproducibility: there is no seed argument, no
    `Random.new`-shaped object and no way to replay a sequence. A seed
    API is where a security bug lives (a reproducible draw is the
    attacker's draw), and determinism in this design belongs to
    `Effect.Clock`, whose readings are declared inputs, not to the
    RNG: a program that needs a repeatable value spells the value or
    reads a recorded input;
  a generator object or a stream of draws: the surface is the
    capability (`using (io :: Io r) => ... ->{IO r}`), not state.
    There is no `Random.new` handle, no iterator of draws, and no
    process-global generator that would leak state between modules:
    every call reads the OS;
  Ruby's own sequences: nothing here reproduces `SecureRandom.hex`,
    `SecureRandom.random_number` or `Random`'s outputs. Ruby's
    `Random` is a Mersenne Twister (seeded and reproducible, refused
    above) and its `SecureRandom` composes hashes over entropy; this
    module draws the bytes and stops, so a caller that wants a recipe
    (a token, a nonce, a shuffled order) names it at its own surface,
    where its properties can be stated;
  entropy failure is not representable here: the intrinsic's
    signature is pinned exactly (`Nat -> Bytes`), so the platform
    layer's failure word, a NEGATED code (the runtime's own probe
    asserts `cinch_random_bytes(-1) < 0`), would arrive where a buffer
    word is expected and this module has nowhere to put it. A
    `/dev/urandom` read that fails on resource exhaustion is therefore
    the boundary's fault, not a `RandomError`. A length beyond the
    `i64` range faults in `Nat`'s own conversion (exit 71), the
    `Bytes` boundary's existing rule, before the source is touched;
  float and unbounded-width draws: only `Nat` draws are here. A
    uniform `F64` needs a rounding rule this slice does not state, and
    an unbounded-width draw has no canonical width; `random-nat` is
    one full 64-bit word, not an unbounded integer;
  unbounded retries: the redraw above is not structurally decreasing,
    so no measure certifies its termination; it stops with probability
    1 and its expected draw count is below two. It is declared
    under the plain `{IO r}` row, the shape `Effect.Fs`'s `dir-names`
    uses for its pipe loop, with no `Diverge`: that label is the
    productivity and cleanup accounting of the codata and structured
    scope rules (core language specification section 9), and nothing
    here loses productivity or leaks cleanup;
  a hex rendering beyond this module's pair: `hex-of-bytes` renders
    any buffer (the renderer `Effect.FileUtils` composes a temp name
    from) and `random-hex` is that rendering over a fresh draw. A
    digest's rendering stays `Text.Digest`'s (`digest-hex`, whose
    byte-level helper is private to that family), and nothing here
    hashes entropy to reach a width: the bytes drawn are the bytes
    rendered.

Bootstrap-compiler note (a tracked compiler finding, not a design
choice): a capability-constrained body that applies a PURE function
to a pure-typed intrinsic's result is miscompiled when the function
is reached through a further `using`-wrapper, faulting with
`cinchruntime: null or wrong-kind word or corrupt generated data`
(exit 71). `random-hex` is exactly that shape
(`hex-of-bytes (random-bytes n)`) and every draw here reaches the
intrinsic, so the working shapes are the ones this module's own test
uses: a `with (io = newIO ()) in` body (and `main`), or binding the
draw and calling the pure `hex-of-bytes` on the binding (the
`Effect.FileUtils` composition) until the finding is closed.

Module scheme (design/base-library.md): `Effect.*` is the effect
system's vocabulary, and this is the platform layer's entropy home.
design/stdlib-parity-ruby.md places `SecureRandom` with the
crypto/digest family (P39, beside Digest and HMAC) rather than with
`Random`, whose seeded generator the section's refused-list discipline
refuses; wall-clock and civil-time draws are `Effect.Clock`'s.

</details>

# Types and constructors

<a id="type-52616e646f6d4572726f72"></a>

## `RandomError`

```cinch
data RandomError
```

What a draw cannot be asked for, named (`Effect.Fs`'s vocabulary
shape: an illegal argument is a value, never a fault).

[Source](/reference-sources/Effect.Random.cinch) · line 151

```cinch
EmptyBound :: RandomError
```

```cinch
EmptyRange :: RandomError
```

# Values

<a id="value-6865782d6f662d6279746573"></a>

## `hex-of-bytes`

```cinch
hex-of-bytes :: (Bytes -> String)
```

Two characters per byte, lower case (`0`..`9` then `a`..`f`): the
rendering a token, a digest echo or a temp name reads. A `String` is
UTF-8 over `Bytes`, and every hex character is ASCII, so the
conversion cannot fail (the `None` arm is unreachable and only keeps
it total, the `Data.Sym`/`Data.Complex` convention).

[Source](/reference-sources/Effect.Random.cinch) · line 208

<a id="value-72616e646f6d2d6279746573"></a>

## `random-bytes`

```cinch
random-bytes :: forall r . using (io :: Io r) => (Nat ->{IO r} Bytes)
```

`n` bytes from the OS entropy source, exactly `n` (`n = 0` is the
legal empty request). Every draw here names this call; nothing holds
state between calls.

[Source](/reference-sources/Effect.Random.cinch) · line 165

<a id="value-72616e646f6d2d686578"></a>

## `random-hex`

```cinch
random-hex :: forall r . using (io :: Io r) => (Nat ->{IO r} String)
```

`n` bytes of OS entropy as 2n lower-case hex characters (Ruby's
`SecureRandom.hex` shape, over this module's own draw rather than a
hash composition).

[Source](/reference-sources/Effect.Random.cinch) · line 219

<a id="value-72616e646f6d2d6e6174"></a>

## `random-nat`

```cinch
random-nat :: forall r . using (io :: Io r) => (Unit ->{IO r} Nat)
```

One full machine word of entropy as a `Nat`: uniform over
`0 .. 2^64 - 1` (eight bytes, so no acceptance region is needed).
The draw is bound before it is read (the `draw-below` shape below):
one binding keeps the value read identical to the value drawn, and
it is the shape the bootstrap compiler compiles correctly (the
header's note on the inlining fault names what the other shape
does).

[Source](/reference-sources/Effect.Random.cinch) · line 175

<a id="value-72616e646f6d2d6e61742d62656c6f77"></a>

## `random-nat-below`

```cinch
random-nat-below :: forall r . using (io :: Io r) => (Nat ->{IO r} Result RandomError Nat)
```

A uniform value below `bound`: rejection sampling over
`byte-width bound` bytes, in the acceptance region the header
derives. `bound = 0` is `Err EmptyBound`; `bound = 1` is 0 and
draws nothing. The emptiness test is the decision that makes the
bound the draw's `NonZero` divisor.

[Source](/reference-sources/Effect.Random.cinch) · line 185

<a id="value-72616e646f6d2d6e61742d696e2d72616e6765"></a>

## `random-nat-in-range`

```cinch
random-nat-in-range :: forall r . using (io :: Io r) => (Nat -> (Nat ->{IO r} Result RandomError Nat))
```

A uniform value in `lo .. hi - 1` (inclusive lower, exclusive
upper): the offset draw shifted by `lo`. An empty range (`hi <= lo`,
which also covers an inverted one) is `Err EmptyRange`; `hi - lo` is
only formed after that guard, so the subtraction cannot go below
zero. The inner `Err` arm is unreachable (`hi - lo` is at least 1
here) and only keeps the propagation total.

[Source](/reference-sources/Effect.Random.cinch) · line 196

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Effect/Random.cinch`. Source SHA-256: `c5d1bcb6f88ed290dd79e70ddf789c1071b9e492a913b54bb62eff3022f012ce`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
