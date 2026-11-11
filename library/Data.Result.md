# Data.Result

Success and named failure values, error-aware transformations, partitioning, and sequencing instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Result`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Result.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Category.Applicative](Category.Applicative.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Data.Core](Data.Core.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Result: a computation's outcome: `Err e` or `Ok x`.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over) and
its D22 sequencing provider instance (`ResultSequencing`: `do`
short-circuits on the first `Err`; the class lives in
Effect.Sequencing).

</details>

# Types and constructors

<a id="type-526573756c74"></a>

## `Result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Result</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

`Result` is here for the same reason: `Data.Result`'s
`partition-results` and `results` consume a `List`, and it too is
checked before `Data.List`.

[Source](/reference-sources/Data.Core.cinch) · line 37

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Err</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> . (<span class="tok-variable">e</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Ok</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)</code></pre>

# Values

<a id="value-656974686572"></a>

## `either`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">either</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">e</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">b</span>)))</code></pre>

The two-branch fold: `f` consumes an error, `g` a value. The error
branch leads, matching Haskell's `either` and this type's
constructor order (the error is the first argument everywhere,
including `Result e a` itself).

[Source](/reference-sources/Data.Result.cinch) · line 40

<a id="value-6572726f7273"></a>

## `errors`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">errors</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">e</span>))</code></pre>

The error projection, the values dropped (`Data.Either.lefts`).

[Source](/reference-sources/Data.Result.cinch) · line 103

<a id="value-66726f6d2d726573756c74"></a>

## `from-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-result</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>))</code></pre>

The value, or the default when there is an error
(`Data.Either.fromRight`); the default leads, like `from-option`'s.

[Source](/reference-sources/Data.Result.cinch) · line 74

<a id="value-69732d657272"></a>

## `is-err`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-err</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Result.cinch)

<a id="value-69732d6f6b"></a>

## `is-ok`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-ok</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

The branch tests: the tag read, no payload work, no constraint.

[Source](/reference-sources/Data.Result.cinch) · line 46

<a id="value-6d61702d657272"></a>

## `map-err`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-err</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">e</span> -&gt; <span class="tok-variable">f</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span>)))</code></pre>

The error's map (`Data.Either`'s `first`, `mapLeft` in ML-family
spellings: it maps the LEFT type). The value side is untouched.

[Source](/reference-sources/Data.Result.cinch) · line 67

<a id="value-6d61702d726573756c74"></a>

## `map-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-result</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">b</span>)))</code></pre>

The value's own map, for a caller who has only the prelude: the
`Functor` method of the same name (`map` in Data.List's surface) is
reached through its constraint or with `import Category.Functor`.
The error side is untouched.

[Source](/reference-sources/Data.Result.cinch) · line 60

<a id="value-6f7074696f6e2d746f2d726573756c74"></a>

## `option-to-result`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">option-to-result</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">e</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)))</code></pre>

The option as a result, with the error value supplied for `None`
(`Data.Either.note`); the error leads, as everywhere in this module.

[Source](/reference-sources/Data.Result.cinch) · line 122

<a id="value-706172746974696f6e2d726573756c7473"></a>

## `partition-results`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">partition-results</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">e</span>), (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

The classic `partitionEithers`: every error and every value, each
side in the input order with its relative order preserved. One pass,
and the recursion happens once per element (the returned tuple is
taken apart by pattern, so neither side is walked twice).

[Source](/reference-sources/Data.Result.cinch) · line 83

<a id="value-726573756c742d746f2d6f7074696f6e"></a>

## `result-to-option`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">result-to-option</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

The value as an option, the error dropped. This is the projection a
caller reaches for in this repo because `Ok` is the success; the
error-side twin is `Data.Either`'s `hush` and is deliberately not
added (its name would have to be spelled for the error side, and no
caller has asked for it).

[Source](/reference-sources/Data.Result.cinch) · line 115

<a id="value-726573756c7473"></a>

## `results`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">results</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">e</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Result</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

The value projection, the errors dropped (`Data.Either.rights`).
`partition-results` is the pair form; these two are the single-side
walks a caller reaches for when it wants one side only.

[Source](/reference-sources/Data.Result.cinch) · line 95

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Result.Applicative_Result` | `Category.Applicative.Applicative (Data.Core.Result e)` | Implicit |
| `Data.Result.Functor_Result` | `Category.Functor.Functor (Data.Core.Result e)` | Implicit |
| `Data.Result.Monad_Result` | `Category.Monad.Monad (Data.Core.Result e)` | Implicit |
| `Data.Result.SeqResult` | `Effect.Sequencing.ResultSequencing e` | Implicit |

**`Data.Result.Applicative_Result`**: `pure` is `Ok`; application propagates the FIRST `Err` (the left
operand's error wins), which is what makes `traverse` into `Result`
short-circuit on the first error in traversal order (moved from
Category.Applicative).
**`Data.Result.Functor_Result`**: The error side is untouched: `map f (Err e)` is `Err e` (moved
from Category.Functor at the prelude split).
**`Data.Result.Monad_Result`**: The error case short-circuits: `Err e >>= k` is `Err e` (moved
from Category.Monad).

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Result.cinch`. Source SHA-256: `b35bc2ca2f50d03522fa0e036ee1c735405ed38dce048d7d8e64d54bdc99ef8d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
