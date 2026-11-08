# Data.List

Finite strict lists: construction, transformation, filtering, splitting, searching, folds, and collection instances.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.List`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.List.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Category.Alternative](Category.Alternative.md), [Category.Applicative](Category.Applicative.md), [Category.Foldable](Category.Foldable.md), [Category.Functor](Category.Functor.md), [Category.Monad](Category.Monad.md), [Category.Traversable](Category.Traversable.md), [Data.Core](Data.Core.md), [Data.FromList](Data.FromList.md), [Data.Ordering](Data.Ordering.md), [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.List: the inductive list: `Nil` or `Cons x xs`.

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over) and
its `FromList` instance (collection literals at `List` are the
identity). `append-list` is the list append the instance family
shares; it is deliberately not in the export list (the curated
surface spells append through `Semigroup`'s `<>`).

`map` here is the concrete list map (the prelude surface's `map`,
decision D106); the `Functor` class method of the same name is
reached through its constraint or an explicit `import
Category.Functor`, which shadows this one per the import shadowing
rule (decision D145).

The concrete `base` combinators live here beside the type: the four
partial projections, the pairing and splitting family, the searching
family, the bounded generators (`iterate`/`unfold`/`cycle` take their
bound as a parameter, because every list-producing walk in this
language must end), the scans and the element-seeded folds,
`transpose`, the stable merge sort, the run grouping, the joining
family, and `nub`/`subsequences`. The container-generic twins
(`sum`, `elem`, the folds with an element seed, ...) live in
`Category.Foldable`, which is reached with an explicit `import`; a
program that wants the list operation without a class constraint uses
the name here, which is the `map` situation over again.

</details>

# Types and constructors

<a id="type-4c697374"></a>

## `List`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">List</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>)</code></pre>

[Source](/reference-sources/Data.Core.cinch)

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Nil</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . <span class="tok-constructor">List</span> <span class="tok-variable">a</span></code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">Cons</span> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> . (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">List</span> <span class="tok-variable">a</span> -&gt; <span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

# Values

<a id="value-627265616b"></a>

## `break`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">break</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>), (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

The longest prefix whose elements do NOT satisfy `p`, and the rest.
Written out rather than spelled `span` of a negated predicate:
`not` lives in `Data.Bool`, which is checked after this module.

[Source](/reference-sources/Data.List.cinch) · line 220

<a id="value-6379636c65"></a>

## `cycle`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cycle</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

The list repeated `n` times. `base`'s `cycle` on an empty list
never returns; here the empty list stays empty, because every
list-producing walk in this language must end.

[Source](/reference-sources/Data.List.cinch) · line 299

<a id="value-64726f70"></a>

## `drop`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">drop</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

Everything but the first `n` elements.

[Source](/reference-sources/Data.List.cinch) · line 107

<a id="value-656c656d2d696e646578"></a>

## `elem-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">elem-index</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

The position of the first element equal to `x` (`None` when
absent).

[Source](/reference-sources/Data.List.cinch) · line 263

<a id="value-66696e64"></a>

## `find`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">find</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

The first element `p` accepts (`None` when none does).

[Source](/reference-sources/Data.List.cinch) · line 240

<a id="value-66696e642d696e646578"></a>

## `find-index`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">find-index</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">Nat</span>)))</code></pre>

The position of the first element `p` accepts (`None` when none
does). The position is a `Nat`: it counts from zero, so it cannot
be negative.

[Source](/reference-sources/Data.List.cinch) · line 250

<a id="value-666f6c642d6c65667431"></a>

## `fold-left1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fold-left1</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

The left fold seeded with the first element, so an empty list has
nothing to return (`None`; `base`'s `foldl1` faults).

[Source](/reference-sources/Data.List.cinch) · line 330

<a id="value-666f6c642d726967687431"></a>

## `fold-right1`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">fold-right1</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

The right fold seeded with the first element (`None` on an empty
list).

[Source](/reference-sources/Data.List.cinch) · line 343

<a id="value-666f6c6472"></a>

## `foldr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">foldr</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">b</span>)))</code></pre>

[Source](/reference-sources/Data.List.cinch)

<a id="value-67726f75702d6279"></a>

## `group-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">group-by</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

The runs of adjacent elements the `same` test groups together: a
run starts at an element and grows while `same` holds of that
element and the next candidate, so `same` is the same-group test
rather than an equality (`base`'s `groupBy`).

[Source](/reference-sources/Data.List.cinch) · line 428

<a id="value-68656164"></a>

## `head`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">head</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

The partial projections of a list: the first element, everything
but the first, the last element, and everything but the last. Each
reports the empty list as `None` (the prelude's partiality
convention: a partial accessor returns `Option`, never a fault).
`head` is `nth 0` spelled for the reader.

[Source](/reference-sources/Data.List.cinch) · line 140

<a id="value-696e6974"></a>

## `init`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">init</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Data.List.cinch)

<a id="value-696e74657263616c617465"></a>

## `intercalate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">intercalate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

The lists joined with `sep` between them (`base`'s `intercalate`,
which is `concat` of `intersperse sep`); `Nil` and a singleton come
back unchanged. The join is the private `join-lists` below rather
than `Category.Foldable`'s class-level `concat`: this is a list
operation, its join is four lines, and a home needs no class
constraint to concatenate its own type.

[Source](/reference-sources/Data.List.cinch) · line 454

<a id="value-696e746572737065727365"></a>

## `intersperse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">intersperse</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-variable">a</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

`sep` between the elements, one separator fewer than the list has
elements, so `Nil` and a singleton come back unchanged (`base`'s
`intersperse`).

[Source](/reference-sources/Data.List.cinch) · line 437

<a id="value-69732d656d707479"></a>

## `is-empty`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">is-empty</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Bool</span>)</code></pre>

[Source](/reference-sources/Data.List.cinch)

<a id="value-69746572617465"></a>

## `iterate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">iterate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">a</span>) -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

The enumeration `x, f x, f (f x), ...` cut to `n` elements. The
bound is what keeps the walk total (`base`'s `iterate` is
infinite).

[Source](/reference-sources/Data.List.cinch) · line 281

<a id="value-6c617374"></a>

## `last`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">last</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.List.cinch)

<a id="value-6c656e677468"></a>

## `length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">length</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Int</span>)</code></pre>

[Source](/reference-sources/Data.List.cinch)

<a id="value-6c6973742d6c656e677468"></a>

## `list-length`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">list-length</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

The same count as a `Nat`, for the indexing and sizing arithmetic
that works in `Nat` (`take`/`drop`/`nth` below, offsets, widths).
`length` stays `Int` because the prelude surface has always
returned `Int` there.

[Source](/reference-sources/Data.List.cinch) · line 82

<a id="value-6c6f6f6b7570"></a>

## `lookup`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">lookup</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">k</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">v</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">k</span>) =&gt; (<span class="tok-variable">k</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">v</span>)))</code></pre>

The value paired with `key`: the first pair whose key matches wins,
so a repeated key keeps the earliest value (`None` when absent).
`lookup` is the association-list read `Data.Map`/`Data.HashMap`
build their trees over.

[Source](/reference-sources/Data.List.cinch) · line 270

<a id="value-6d6170"></a>

## `map`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">map</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">b</span>)))</code></pre>

`List`, `Nil` and `Cons` are declared in `Data.Core` and re-exported
here (that module's header says why: `Data.List` and `Data.Option`
name each other in signatures, so neither can be checked first).
This home carries the type's instance family and its operations.

[Source](/reference-sources/Data.List.cinch) · line 63

<a id="value-6e7468"></a>

## `nth`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nth</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-variable">a</span>)))</code></pre>

The element at index `n` (`None` past the end).

[Source](/reference-sources/Data.List.cinch) · line 115

<a id="value-6e7562"></a>

## `nub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">nub</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">eq</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Eq</span>.<span class="tok-constructor">Eq</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

The list with later duplicates removed, keeping the first
occurrence of each element (`base`'s `nub`; the `Eq` constraint is
what decides sameness).

[Source](/reference-sources/Data.List.cinch) · line 466

<a id="value-706172746974696f6e"></a>

## `partition`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">partition</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>), (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

The elements that satisfy `p` and the ones that do not, each in
input order (so the two results interleave back into the input).

[Source](/reference-sources/Data.List.cinch) · line 230

<a id="value-7265706c6963617465"></a>

## `replicate`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">replicate</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-variable">a</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

`x` repeated `n` times.

[Source](/reference-sources/Data.List.cinch) · line 125

<a id="value-72657665727365"></a>

## `reverse`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">reverse</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

Reversal (the accumulator is the reverse built so far, so the walk
is linear; `reverse-into` is the private twin every accumulator
walk here shares).

[Source](/reference-sources/Data.List.cinch) · line 90

<a id="value-7363616e6c"></a>

## `scanl`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanl</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">b</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">b</span>))))</code></pre>

The successive reductions from the left, the seed first: `scanl f z
xs` has one element more than `xs` (`base`'s `scanl`).

[Source](/reference-sources/Data.List.cinch) · line 308

<a id="value-7363616e72"></a>

## `scanr`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scanr</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-variable">b</span>)) -&gt; (<span class="tok-variable">b</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">b</span>))))</code></pre>

The successive reductions from the right: the first element is the
whole fold and the seed is last (`base`'s `scanr`).

[Source](/reference-sources/Data.List.cinch) · line 315

<a id="value-736f7274"></a>

## `sort`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sort</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">ord</span> :: <span class="tok-constructor">Relation</span>.<span class="tok-constructor">Ord</span>.<span class="tok-constructor">Ord</span> <span class="tok-variable">a</span>) =&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))</code></pre>

The list in ascending order under `Ord` (a stable merge sort).

[Source](/reference-sources/Data.List.cinch) · line 411

<a id="value-736f72742d6279"></a>

## `sort-by`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sort-by</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Data</span>.<span class="tok-constructor">Ordering</span>.<span class="tok-constructor">Ordering</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

Merge sort under a three-way comparator. Stable: the left element
is kept unless the right one compares strictly before it, so equal
elements stay in their input order (`base`'s `sortBy`).

[Source](/reference-sources/Data.List.cinch) · line 382

<a id="value-7370616e"></a>

## `span`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">span</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; <span class="tok-constructor">Bool</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>), (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

The longest prefix whose elements satisfy `p`, and the rest.

[Source](/reference-sources/Data.List.cinch) · line 209

<a id="value-73706c69742d6174"></a>

## `split-at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">split-at</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>), (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>))))</code></pre>

The first `n` elements and the rest; the split clamps at the end,
so `split-at n` of a shorter list is the whole list and `Nil`.

[Source](/reference-sources/Data.List.cinch) · line 200

<a id="value-73756273657175656e636573"></a>

## `subsequences`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">subsequences</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

Every subsequence, the empty one first, in `base`'s order:
`subsequences [a, b]` is `[[], [a], [b], [a, b]]`. The count is
`2^n` for a list of `n` elements, so this is the one combinator
here whose result can dwarf its input; it still ends, because the
input does.

[Source](/reference-sources/Data.List.cinch) · line 484

<a id="value-73756d2d6c697374"></a>

## `sum-list`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sum-list</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">add</span> :: <span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">Add</span> <span class="tok-variable">a</span>) =&gt; (<span class="tok-variable">a</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; <span class="tok-variable">a</span>))</code></pre>

[Source](/reference-sources/Data.List.cinch)

<a id="value-7461696c"></a>

## `tail`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">tail</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

[Source](/reference-sources/Data.List.cinch)

<a id="value-74616b65"></a>

## `take`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">take</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). (<span class="tok-constructor">Nat</span> -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

The first `n` elements (fewer when the list is shorter).

[Source](/reference-sources/Data.List.cinch) · line 99

<a id="value-7472616e73706f7365"></a>

## `transpose`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">transpose</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

The transpose of a ragged list of lists: round k is the k-th
element of every row that still has one, so a short row contributes
nothing to the later rounds and the result has as many rows as the
longest input row (`base`'s `transpose`). A rectangular `Matrix`
record keeps its own row-level helper; this is the general list
operation.

[Source](/reference-sources/Data.List.cinch) · line 360

<a id="value-756e666f6c64"></a>

## `unfold`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unfold</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>))) -&gt; (<span class="tok-variable">b</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>)))</code></pre>

The unfolding: `f` either yields the next element together with the
next seed, or stops. `base` spells this `unfoldr`; the `r` suffix
is historical (the walk is not a right fold), so the name here is
the plain one.

[Source](/reference-sources/Data.List.cinch) · line 290

<a id="value-756e7a6970"></a>

## `unzip`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">unzip</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>), (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">b</span>)))</code></pre>

The inverse of `zip`: the first components and the second
components, each in order.

[Source](/reference-sources/Data.List.cinch) · line 191

<a id="value-7a6970"></a>

## `zip`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">b</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>))))</code></pre>

Pairing, truncated at the shorter side (so a length mismatch
shortens the result, never faults).

[Source](/reference-sources/Data.List.cinch) · line 174

<a id="value-7a69702d77697468"></a>

## `zip-with`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">zip-with</span> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">a</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">b</span> :: <span class="tok-constructor">Type</span>) (<span class="tok-variable">c</span> :: <span class="tok-constructor">Type</span>). ((<span class="tok-variable">a</span> -&gt; (<span class="tok-variable">b</span> -&gt; <span class="tok-variable">c</span>)) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">a</span>) -&gt; ((<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">b</span>) -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">List</span> <span class="tok-variable">c</span>))))</code></pre>

`zip` with the pair built by `f`, truncated the same way.

[Source](/reference-sources/Data.List.cinch) · line 182

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.List.Alternative_List` | `Category.Alternative.Alternative Data.Core.List` | Implicit |
| `Data.List.Applicative_List` | `Category.Applicative.Applicative Data.Core.List` | Implicit |
| `Data.List.Foldable_List` | `Category.Foldable.Foldable Data.Core.List` | Implicit |
| `Data.List.FromList_List` | `Data.FromList.FromList (Data.Core.List e) e` | Implicit |
| `Data.List.Functor_List` | `Category.Functor.Functor Data.Core.List` | Implicit |
| `Data.List.Monad_List` | `Category.Monad.Monad Data.Core.List` | Implicit |
| `Data.List.Monoid_List` | `Algebra.Monoid.Monoid (Data.Core.List t)` | Implicit |
| `Data.List.Semigroup_List` | `Algebra.Semigroup.Semigroup (Data.Core.List t)` | Implicit |
| `Data.List.Traversable_List` | `Category.Traversable.Traversable Data.Core.List` | Implicit |

**`Data.List.Alternative_List`**: Choice is concatenation (moved from Category.Alternative).
**`Data.List.Applicative_List`**: `pure` is the singleton list; `<*>` applies every function to
every value in order (the list of all results; moved from
Category.Applicative).
**`Data.List.Foldable_List`**: The list fold (moved from Category.Foldable).
**`Data.List.Functor_List`**: Functor: map over every element (moved from Category.Functor).
**`Data.List.Monad_List`**: Bind is flatMap: every element's continuation result, in order
(moved from Category.Monad).
**`Data.List.Monoid_List`**: Monoid under append; the empty list is the identity (moved from
Algebra.Monoid).
**`Data.List.Semigroup_List`**: Semigroup under append: associative because list concatenation is
(moved from Algebra.Semigroup at the prelude split).
**`Data.List.Traversable_List`**: Left-to-right; the whole list rebuilds under one `f` (moved from
Category.Traversable).

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/List.cinch`. Source SHA-256: `f765133818f3970e81d9b8838a80afd539fc852b9041945ecff2f4a171282f8d`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
