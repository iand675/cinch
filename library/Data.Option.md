# Data.Option

Optional values with None and Some, plus combinators for propagating or consuming absence.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Option`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Option.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Category.Alternative](Category.Alternative.md), [Category.Applicative](Category.Applicative.md), [Category.Foldable](Category.Foldable.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Category.Traversable](Category.Traversable.md), [Data.Core](Data.Core.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Option: the optional value: `None` or `Some x`.

Module scheme (design/base-library.md): `Data.*` homes are the
concrete types; every instance lives with its type, so the instance
family for `Option` is here (the parked-instance deviation of D159
is over). The class modules own only their structural instances.

The D22 sequencing provider (`OptionSequencing`) is a property of
the container, so its provider instance is here too: one instance
per provider class keeps `do` selection order-independent (decision
D22; the classes live in Effect.Sequencing).

</details>

# Types and constructors

<a id="type-4f7074696f6e"></a>

## `Option`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Option</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Core.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">None</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Some</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)</code></pre>

# Values

<a id="value-6361742d6f7074696f6e73"></a>

## `cat-options`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cat-options</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

The present values, in order; `None`s drop (`Data.Maybe.catMaybes`).
`List` is nameable here because `Data.Core` declares it ahead of
every home (this function's signature is exactly the cycle that
module exists to break).

[Source](/reference-sources/Data.Option.cinch) · line 75

<a id="value-66726f6d2d6f7074696f6e"></a>

## `from-option`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">from-option</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>))</code></pre>

The value, or the default when there is none
(`Data.Maybe.fromMaybe`); the default leads for the same reason, and
because the result's type is the default's type.

[Source](/reference-sources/Data.Option.cinch) · line 55

<a id="value-69732d6e6f6e65"></a>

## `is-none`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-none</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.Option.cinch)

<a id="value-69732d736f6d65"></a>

## `is-some`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-some</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

The presence tests: the tag read, no payload work, no constraint.

[Source](/reference-sources/Data.Option.cinch) · line 61

<a id="value-6c6973742d746f2d6f7074696f6e"></a>

## `list-to-option`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-to-option</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

The first element, or `None` for the empty list
(`Data.Maybe.listToMaybe`); the tail is deliberately not inspected,
so this is O(1) on any list.

[Source](/reference-sources/Data.Option.cinch) · line 110

<a id="value-6d61702d6f7074696f6e"></a>

## `map-option`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map-option</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">b</span>)))</code></pre>

The option's own map, for a caller who has only the prelude: the
`Functor` method of the same name (`map` in Data.List's surface) is
reached through its constraint or with `import Category.Functor`.

```cinch test name=map-some
--- input
main :: Int
main = case map-option (\x -> x + 1) (Some 1) of
  Some n -> n - 2
  None -> 1
--- exit
0
```

[Source](/reference-sources/Data.Option.cinch) · line 95

<a id="value-6d61796265"></a>

## `maybe`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">maybe</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">b</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">b</span>)))</code></pre>

The fold: the default `d` unless there is a value, which `f`
consumes. The default leads and the function follows, matching
Haskell's `maybe` (`maybe d f o`); parity of argument order is the
point of these combinators, and the option is the thing opened last.

[Source](/reference-sources/Data.Option.cinch) · line 47

<a id="value-6f7074696f6e2d746f2d6c697374"></a>

## `option-to-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">option-to-list</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

A `Some` is a one-element list and a `None` the empty list
(`Data.Maybe.maybeToList`); `list-to-option` is its left inverse.

[Source](/reference-sources/Data.Option.cinch) · line 102

<a id="value-6f722d656c7365"></a>

## `or-else`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">or-else</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

First success: `None` falls through to the second option, `Some`
keeps the first (`Control.Applicative`'s `(<|>)` for `Option`, which
a prelude-only caller cannot reach: `Alternative` is not in the
implicit surface). The name is the value-fallback one, as in Scala's
`orElse` and this repo's `Text.Json.or-else`; Rust's closure-taking
`or_else` is deliberately not the shape, so an eager second option
is never built.

[Source](/reference-sources/Data.Option.cinch) · line 122

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Option.Alternative_Option` | `Category.Alternative.Alternative Data.Core.Option` | Implicit |
| `Data.Option.Applicative_Option` | `Category.Applicative.Applicative Data.Core.Option` | Implicit |
| `Data.Option.Foldable_Option` | `Category.Foldable.Foldable Data.Core.Option` | Implicit |
| `Data.Option.Functor_Option` | `Category.Functor.Functor Data.Core.Option` | Implicit |
| `Data.Option.Monad_Option` | `Category.Monad.Monad Data.Core.Option` | Implicit |
| `Data.Option.Monoid_Option` | `Algebra.Monoid.Monoid (Data.Core.Option t)` | Implicit |
| `Data.Option.Semigroup_Option` | `Algebra.Semigroup.Semigroup (Data.Core.Option t)` | Implicit |
| `Data.Option.SeqOption` | `Effect.Sequencing.OptionSequencing` | Implicit |
| `Data.Option.Traversable_Option` | `Category.Traversable.Traversable Data.Core.Option` | Implicit |

**`Data.Option.Alternative_Option`**: First success wins: `None <|> y` is `y`, `Some x <|> y` keeps `x`
(moved from Category.Alternative).
**`Data.Option.Applicative_Option`**: `pure` is `Some`; application is `None` if either side is (moved
from Category.Applicative).
**`Data.Option.Foldable_Option`**: A `Some` is a one-element container; `None` folds as empty (moved
from Category.Foldable).
**`Data.Option.Functor_Option`**: `map` is `Some` under the function; `None` is fixed (moved from
Category.Functor).
**`Data.Option.Monad_Option`**: The failing case short-circuits: `None >>= k` is `None` (moved
from Category.Monad).
**`Data.Option.Monoid_Option`**: `None` is the identity (moved from Algebra.Monoid).
**`Data.Option.Semigroup_Option`**: Option first-some: the leftmost `Some` wins, `None` is ignored
(moved from Algebra.Semigroup at the prelude split; the instance
lives with its type).
**`Data.Option.Traversable_Option`**: `None` traverses to `pure None`; `Some` carries one effect (moved
from Category.Traversable).

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Option.cinch`. Source SHA-256: `907ea837397289a0b3b672cdfd458c1f8a5d474c8387c5a1bd8ed2823463da01`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
