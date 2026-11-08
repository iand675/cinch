# Data.Int

Mathematical signed integers, arithmetic, division/remainder, formatting support, and explicit conversions.

<span class="support-tag support-tag--current">Embedded</span> Available from the compiler's embedded base family.

Import with `import Data.Int`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Int.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Group](Algebra.Group.md), [Algebra.Semiring](Algebra.Semiring.md), [Data.Core](Data.Core.md), [Data.Hash](Data.Hash.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Int: the mathematical integers.

`Int` is a core runtime type (primitive types specification section
1): a boxed bignum, distinct from every machine scalar. Division
and remainder are total by construction: the divisor is a `NonZero
Int` (see Data.Nat).

Module scheme (design/base-library.md): the type's home carries its
instance family. `Int` gets no `Semigroup` instance (two monoids
live on it, chosen by `Sum`/`Product` in Algebra.Monoid); its
`Semiring` names both operations, and its `Group` is the additive
group of that semiring (`invert` is `int-neg`, the only total
inversion on `Int`).

</details>

# Types and constructors

This module exports no type declarations.

# Values

<a id="value-636865636b65642d696e742d746f2d693634"></a>

## `checked-int-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">checked-int-to-i64</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Data</span>.<span class="tok-constructor">Core</span>.<span class="tok-constructor">Option</span> <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Int.cinch)

<a id="value-696e742d616464"></a>

## `int-add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-add</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Int</span>))</code></pre>

[Source](/reference-sources/Data.Int.cinch)

<a id="value-696e742d636d70"></a>

## `int-cmp`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-cmp</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">I64</span>))</code></pre>

[Source](/reference-sources/Data.Int.cinch)

<a id="value-696e742d646976"></a>

## `int-div`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-div</span> :: (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">NonZero</span> <span class="tok-constructor">Int</span>) -&gt; <span class="tok-constructor">Int</span>))</code></pre>

[Source](/reference-sources/Data.Int.cinch)

<a id="value-696e742d66726f6d2d693634"></a>

## `int-from-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-from-i64</span> :: ((<span class="tok-variable">x</span> :: <span class="tok-constructor">I64</span>) -&gt; (<span class="tok-variable">refined</span> (<span class="tok-variable">cinch-result</span> <span class="tok-operator">==</span> <span class="tok-variable">x</span>) <span class="tok-constructor">Int</span>))</code></pre>

The `I64` conversions keep the value; an `Int` past the `I64`
range is no `I64`, so `int-to-i64` states the range it needs (a
guard or a conversion result supplies it) and `checked-int-to-i64`
answers `None` outside it.

[Source](/reference-sources/Data.Int.cinch) · line 35

<a id="value-696e742d6d6f642d6e6174"></a>

## `int-mod-nat`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-mod-nat</span> :: (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">NonZero</span> <span class="tok-constructor">Nat</span>) -&gt; <span class="tok-constructor">Nat</span>))</code></pre>

The floored residue of `x` modulo `d` as a `Nat`, `0 <= r < d`
for every `x` (a negative `x` counts up from `d`): an encoding into
`d` values that needs no range of `x`.

[Source](/reference-sources/Data.Int.cinch) · line 45

<a id="value-696e742d6d756c"></a>

## `int-mul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-mul</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Int</span>))</code></pre>

[Source](/reference-sources/Data.Int.cinch)

<a id="value-696e742d6e6567"></a>

## `int-neg`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-neg</span> :: (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Int</span>)</code></pre>

[Source](/reference-sources/Data.Int.cinch)

<a id="value-696e742d72656d"></a>

## `int-rem`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-rem</span> :: (<span class="tok-constructor">Int</span> -&gt; ((<span class="tok-constructor">Algebra</span>.<span class="tok-constructor">Arithmetic</span>.<span class="tok-constructor">NonZero</span> <span class="tok-constructor">Int</span>) -&gt; <span class="tok-constructor">Int</span>))</code></pre>

[Source](/reference-sources/Data.Int.cinch)

<a id="value-696e742d737562"></a>

## `int-sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-sub</span> :: (<span class="tok-constructor">Int</span> -&gt; (<span class="tok-constructor">Int</span> -&gt; <span class="tok-constructor">Int</span>))</code></pre>

[Source](/reference-sources/Data.Int.cinch)

<a id="value-696e742d746f2d693634"></a>

## `int-to-i64`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">int-to-i64</span> :: <span class="tok-keyword">using</span> (<span class="tok-number">-9223372036854775808</span> <span class="tok-operator">&lt;=</span> <span class="tok-variable">arg0</span>, <span class="tok-variable">arg0</span> <span class="tok-operator">&lt;=</span> <span class="tok-number">9223372036854775807</span>) =&gt; ((<span class="tok-variable">arg0</span> :: <span class="tok-constructor">Int</span>) -&gt; <span class="tok-variable">refined</span> (<span class="tok-variable">r</span> <span class="tok-operator">==</span> <span class="tok-variable">arg0</span>) <span class="tok-constructor">I64</span>)</code></pre>

[Source](/reference-sources/Data.Int.cinch)

# Instances

These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).

| Instance | Target | Resolution |
| --- | --- | --- |
| `Data.Int.Add_Int` | `Algebra.Arithmetic.Add Int` | Implicit |
| `Data.Int.Div_Int` | `Algebra.Arithmetic.Div Int` | Implicit |
| `Data.Int.Enum_Int` | `Enum Int` | Implicit |
| `Data.Int.Eq_Int` | `Relation.Eq.Eq Int` | Implicit |
| `Data.Int.Group_Int` | `Algebra.Group.Group Int` | Implicit |
| `Data.Int.Hash_Int` | `Data.Hash.Hash Int` | Implicit |
| `Data.Int.Mul_Int` | `Algebra.Arithmetic.Mul Int` | Implicit |
| `Data.Int.Neg_Int` | `Algebra.Arithmetic.Neg Int` | Implicit |
| `Data.Int.Ord_Int` | `Relation.Ord.Ord Int` | Implicit |
| `Data.Int.Rem_Int` | `Algebra.Arithmetic.Rem Int` | Implicit |
| `Data.Int.Semiring_Int` | `Algebra.Semiring.Semiring Int` | Implicit |
| `Data.Int.Sub_Int` | `Algebra.Arithmetic.Sub Int` | Implicit |

**`Data.Int.Enum_Int`**: The enumeration of mathematical integers (Data.Enum): total in
both directions, codes consecutive (instances live with their
type).
**`Data.Int.Eq_Int`**: Mathematical `Int` and `Nat` instances (primitive types
specification, section 1): comparison through the cmp prims'
immediates, arithmetic through the runtime. `-` on `Int` needs no
evidence; `-` on `Nat` is the exact difference and requires
`b <= a` (see Data.Nat).
**`Data.Int.Group_Int`**: The additive group of the integers: `invert x` is `-x` (the group
inverse laws are asserted over samples in
compiler/tests/prelude_split.rs; instances live with their type).
**`Data.Int.Hash_Int`**: The magnitude's digest: equal integers hash alike (the `Hash` law
holds by construction; instances live with their type). The
magnitude is the `Nat` of the same value, of any size, hashed as a
`Nat` is (no `I64` in between).
**`Data.Int.Semiring_Int`**: The semiring of integers: addition and multiplication with their
identities (see the header).

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Int.cinch`. Source SHA-256: `3298641d85d00bf4173a13060bbdfd1e8e63b8c3ff79b96e635525485bfd45ea`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
