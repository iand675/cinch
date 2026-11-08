# Function.Combinators

Small reusable function operations such as identity, composition, argument manipulation, and application.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Function.Combinators`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Function.Combinators.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Relation.Eq](Relation.Eq.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Function.Combinators: combinators over plain functions.

Module scheme (design/base-library.md): `Function.*` is the kind of
combinators over plain functions. The pair projections (`fst`,
`snd`, `swap`, `curry`, `uncurry`) live here too: a tuple is a
language-level type with no home module of its own, and these are
function-level projections over a pair, the same kind of value as
the combinators around them, not list operations.

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-6170706c792d6e2d74696d6573"></a>

## `apply-n-times`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">apply-n-times</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)))</code></pre>

`apply-n-times n f x` applies `f` to `x` `n` times: `f (f x)` at
`n` = 2 and `x` at `n` = 0. The count is a `Nat`, so the base case
is the zero test and the step decrements (the same `Nat` walk as
`Data.List`'s `take`).

[Source](/reference-sources/Function.Combinators.cinch) · line 54

<a id="value-6170706c792d746f"></a>

## `apply-to`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">apply-to</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; <span class="tok-variable">b</span>))</code></pre>

Reverse application: `apply-to x f` is `f x` (the `Data.Function.(&)`
pipe). The name is `apply-to`, not an operator: `&` is bitwise AND
(Algebra.Bits) and `|>` is not in the base fixity table, so an
operator spelling would either collide or need a new table entry.

[Source](/reference-sources/Function.Combinators.cinch) · line 47

<a id="value-626f7468"></a>

## `both`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">both</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">c</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">c</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span>, <span class="tok-variable">c</span>))))</code></pre>

`both f g x` is the pair of both projections of `x` (the fan-out
combinator, the `(&&&)` shape without the arrow machinery).

[Source](/reference-sources/Function.Combinators.cinch) · line 61

<a id="value-636f6d706f7365"></a>

## `compose`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">compose</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">c</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">b</span> -&gt; <span class="tok-variable">c</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">c</span>)))</code></pre>

`compose h g` applies `g` then `h` (the functor composition law's
named composition: a law equation states total function
applications, never a lambda; primitive operations and inline
target IR specification section 5).

[Source](/reference-sources/Function.Combinators.cinch) · line 28

<a id="value-636f6e7374"></a>

## `const`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">const</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Function.Combinators.cinch)

<a id="value-6375727279"></a>

## `curry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">curry</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">c</span> :: <span class="tok-constructor">Type</span>). (((<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) -&gt; <span class="tok-variable">c</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-variable">c</span>)))</code></pre>

[Source](/reference-sources/Function.Combinators.cinch)

<a id="value-666c6970"></a>

## `flip`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">flip</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">c</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-variable">c</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">c</span>)))</code></pre>

`flip f` swaps `f`'s two arguments, so `flip (-) x y` is `y - x`
and `flip const x y` is `y` (`Data.Function.flip`).

[Source](/reference-sources/Function.Combinators.cinch) · line 33

<a id="value-667374"></a>

## `fst`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fst</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) -&gt; <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Function.Combinators.cinch)

<a id="value-6964656e74697479"></a>

## `identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">identity</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)</code></pre>

[Source](/reference-sources/Function.Combinators.cinch)

<a id="value-6f6e"></a>

## `on`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">on</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">c</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-variable">c</span>)) -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">c</span>))))</code></pre>

`on g k` compares two values through `k`'s projections: `on (<)
fst` is the comparator on first components (the `Data.Function.on`
combinator, prefix here since the surface declares no `on`
fixity).

[Source](/reference-sources/Function.Combinators.cinch) · line 40

<a id="value-736e64"></a>

## `snd`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">snd</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) -&gt; <span class="tok-variable">b</span>)</code></pre>

[Source](/reference-sources/Function.Combinators.cinch)

<a id="value-73776170"></a>

## `swap`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">swap</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) -&gt; (<span class="tok-variable">b</span>, <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Function.Combinators.cinch)

<a id="value-756e6375727279"></a>

## `uncurry`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">uncurry</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">c</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-variable">c</span>)) -&gt; ((<span class="tok-variable">a</span>, <span class="tok-variable">b</span>) -&gt; <span class="tok-variable">c</span>))</code></pre>

[Source](/reference-sources/Function.Combinators.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Function/Combinators.cinch`. Source SHA-256: `8d4b3bd4798a326720d7d95f6c632dc4480eb4c825ca7d4a4db1254945ba5eef`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
