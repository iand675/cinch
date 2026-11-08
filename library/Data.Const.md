# Data.Const

A constant container that retains its first parameter while ignoring the mapped value parameter.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Const`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Const.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Category.Applicative](Category.Applicative.md), [Category.Functor](Category.Functor.md).

<details><summary>Behaviour and boundary notes from the source</summary>

The constant functor: `Const a b` carries an `a` and erases the `b`.
It is the universal visitor for computations whose result type is
fixed, and the composing piece the Category.* classes use as their
lawful constant functor.

Module name: `Data.Const` (the module scheme's name,
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

The functions and the instances: `map-const` is the Functor `map`
body for `Const` (the class name is `map`, decision D152), and the
Functor instance at the bottom is defined through it (instances live
with their type). `Const`'s Applicative lands with the prelude
split (D157's waiter is discharged): its `pure` fabricates the
`Monoid` payload and its `<*>` combines payloads under `Semigroup`,
so the instance states both constraints (a class constraint entails
nothing; the standing no-superbundle rule). The operator `<$>` is a
class method per decision D2 (operators are class methods with
per-module fixity declarations), so no per-type operator stand-in is
defined here.

</details>

# Types and constructors

<a id="type-436f6e7374"></a>

## `Const`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Const</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Const.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Const</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> . (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Const</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>)</code></pre>

# Values

<a id="value-6765742d636f6e7374"></a>

## `get-const`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">get-const</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Const</span>.<span class="tok-constructor">Const</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span>) -&gt; <span class="tok-variable">a</span>)</code></pre>

Elimination: the carried value.

[Source](/reference-sources/Data.Const.cinch) · line 43

<a id="value-6d61702d636f6e7374"></a>

## `map-const`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-const</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">m</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Const</span>.<span class="tok-constructor">Const</span> <span class="tok-variable">m</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Const</span>.<span class="tok-constructor">Const</span> <span class="tok-variable">m</span> <span class="tok-variable">b</span>)))</code></pre>

Functor body: the carried value is unchanged and the function is
ignored (the constant functor).

[Source](/reference-sources/Data.Const.cinch) · line 49

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Const.Applicative_Const` | `Category.Applicative.Applicative (Data.Const.Const m)` | Implicit |
| `Data.Const.Functor_Const` | `Category.Functor.Functor (Data.Const.Const m)` | Implicit |

**`Data.Const.Applicative_Const`**: The applicative (D157's waiter is discharged at the prelude
split): `pure` fabricates the `Monoid` payload and `<*>` combines
payloads under `Semigroup`. The instance states both constraints
because a class constraint entails nothing (the standing
no-superbundle rule). The combination is written in prefix form
like the other instance bodies (decision D2's per-module fixity
style keeps `<>` out of this module's parse).
**`Data.Const.Functor_Const`**: The class instance (instances live with their type), defined
through the body above.

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Const.cinch`. Source SHA-256: `6ab8e9bf59d407a77e3a2f78ded46730596593352c4c948f9896d9fa0917a26c`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
