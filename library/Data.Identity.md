# Data.Identity

The identity container and its generic abstraction instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Identity`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Identity.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Category.Applicative](Category.Applicative.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md).

<details><summary>Behaviour and boundary notes from the source</summary>

The identity type: the universal visitor that changes nothing. It is
the unit of the category family (the identity functor and the trivial
monad), so its operations below are the instance method bodies the
Category.* classes carry as this type's instances.

Module name: `Data.Identity` (the module scheme's name,
design/base-library.md; the temporary single-segment header is gone
now that dotted module names resolve).

`do` notation and these classes (decision D22): `do` picks its
provider lexically per definition, through a `using` binder with a
DirectSequencing/ResultSequencing/OptionSequencing head (default:
DirectSequencing). A do-provider is a fixed instance choice: exactly
one container per definition, no polymorphism. The Category.*
classes (Functor/Applicative/Monad) abstract over containers for
polymorphic code; their `map`/`pure`/`>>=` are ordinary class
methods resolved per use site. The two mechanisms do not interact
and neither is changing: `do` never consults the classes, and a class
constraint never selects a do provider.

The functions and the instances: the functions below are the
instance method bodies (Functor `map`, Applicative `pure`/`<*>`/
`lift-a2`, Monad `>>=`/`join`; there is no `return`, decision D152).
The classes have landed (higher-kinded parameters), so the instances
at the bottom are defined through these bodies: the plain functions
stay as the type's direct API and the class methods resolve to them.
The operators `<$>`, `<*>` and `>>=` are class methods per decision
D2 (operators are class methods with per-module fixity
declarations), so no per-type operator stand-ins are defined here.

</details>

# Types and constructors

<a id="type-4964656e74697479"></a>

## `Identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Identity</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Identity.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Identity</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">a</span>)</code></pre>

# Values

<a id="value-61702d6964656e74697479"></a>

## `ap-identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">ap-identity</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">b</span>)))</code></pre>

Applicative body: apply a wrapped function to a wrapped value.

[Source](/reference-sources/Data.Identity.cinch) · line 54

<a id="value-62696e642d6964656e74697479"></a>

## `bind-identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">bind-identity</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">b</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">b</span>)))</code></pre>

Monad body: sequence a dependent computation.

[Source](/reference-sources/Data.Identity.cinch) · line 63

<a id="value-6a6f696e2d6964656e74697479"></a>

## `join-identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">join-identity</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">a</span>))</code></pre>

Monad body: collapse one layer.

[Source](/reference-sources/Data.Identity.cinch) · line 68

<a id="value-6c6966742d61322d6964656e74697479"></a>

## `lift-a2-identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lift-a2-identity</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">c</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-variable">c</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">b</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">c</span>))))</code></pre>

Applicative body: lift a two-argument function.

[Source](/reference-sources/Data.Identity.cinch) · line 59

<a id="value-6d61702d6964656e74697479"></a>

## `map-identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-identity</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">b</span>)))</code></pre>

Functor body: map over the wrapped value.

[Source](/reference-sources/Data.Identity.cinch) · line 45

<a id="value-707572652d6964656e74697479"></a>

## `pure-identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">pure-identity</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">a</span>))</code></pre>

Applicative body.

[Source](/reference-sources/Data.Identity.cinch) · line 50

<a id="value-72756e2d6964656e74697479"></a>

## `run-identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">run-identity</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Identity</span>.<span class="tok-constructor">Identity</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>)</code></pre>

Elimination: the wrapped value.

[Source](/reference-sources/Data.Identity.cinch) · line 40

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Identity.Applicative_Identity` | `Category.Applicative.Applicative Data.Identity.Identity` | Implicit |
| `Data.Identity.Functor_Identity` | `Category.Functor.Functor Data.Identity.Identity` | Implicit |
| `Data.Identity.Monad_Identity` | `Category.Monad.Monad Data.Identity.Identity` | Implicit |

**`Data.Identity.Functor_Identity`**: The class instances (instances live with their type), defined
through the bodies above.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Identity.cinch`. Source SHA-256: `2429ddda85cbdaaef07d640c7df7cb7e78e6537c30ff3920ac0ca1675a5922e9`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
