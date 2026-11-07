# Data.Matrix

Matrix construction, shape checks, indexing, transformation, and arithmetic operations.

<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.

Import with `import Data.Matrix`. Selected exports may already be in Prelude.

[Download exact module source](/reference-sources/Data.Matrix.cinch) · [How library modules are loaded](00-index.md)

# Dependencies

[Data.Bool](Data.Bool.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

Non-embedded source closure: [Data.Matrix](Data.Matrix.md).

<details><summary>Behaviour and boundary notes from the source</summary>

Data.Matrix: dense `F64` matrices, dimensions checked at every
operation.

The Ruby `Matrix` family (design/stdlib-parity-ruby.md, the P38
numeric port: "Matrix as a concrete type with explicit operations,
the P2 discipline: types and operations, no numeric tower") as a
dense row-major `F64` matrix over `Data.List`: a value is rows of
`F64` cells, `rows`/`cols` are the dimensions, and every
shape-sensitive operation checks the dimensions and reports a NAMED
error rather than producing a wrong-shaped result. The shape
invariant (every row exactly `cols` long) is enforced at
construction (`matrix`), so it holds for every value the type
carries; the constructor stays module-private so nothing can build a
ragged one (decision D143's surface rule).

The laws the surface must satisfy (the Ruby behaviours pinned in the
parity note, asserted consumer-side in
compiler/tests/lib_matrix.rs): `identity n * m == m` for an
n-column m, `(a * b).transpose == b.transpose * a.transpose`,
`determinant (identity n) == 1`, and a singular matrix has
determinant 0 within a documented tolerance.

Module scheme (design/base-library.md): `Data.*` is the concrete
types, and this module is the type's home; `Text.*` owns conversion
to `String`, so `to-string` is a plain function here (there is no
`Show F64`: `Data.Machine` gives `F64` no Eq/Ord and no text
instance, decision D6/D106's dash-style names used throughout).

THE DETERMINANT ALGORITHM: cofactor expansion along the first row
(`det A = sum over j of (-1)^j * A[0][j] * det(minor(A, 0, j))`).
It is correct and short but FACTORIAL: O(n!) multiplications, each
minor rebuild costing O(n^2). It is deliberately adequate for the
small matrices this slice is verified on; an LU/QR family would
replace it and is refused below.

Named refusals (boundaries this module deliberately does not cross):
  sparse matrices: the representation is dense (a full `List` of
    `List F64`); a sparse family is a different type, not a mode of
    this one;
  decompositions (LU, QR, Cholesky) and `inverse`: none are shipped;
    the determinant uses cofactor expansion and no operation needs a
    factorization;
  integer or rational matrices: the cell type is `F64` only (an
    `Int`/`Rational` matrix is a different carrier, the parity
    note's `Rational` family, not a parameter of this one);
  `Matrix#/` and `Matrix#**`: not ported (division needs the refused
    inverse; the general power needs exponentiation by squaring over
    a decomposition).

</details>

# Types and constructors

<a id="type-4d6174726978"></a>

## `Matrix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">Matrix</span></code></pre>

The dense matrix: the dimensions and the row-major rows. The
constructor is module-private (the ragged invariant lives at
`matrix`, so no other value can exist ragged).

[Source](/reference-sources/Data.Matrix.cinch) · line 65

<a id="type-4d61747269784572726f72"></a>

## `MatrixError`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-keyword">data</span> <span class="tok-constructor">MatrixError</span></code></pre>

The named errors; every shape violation is one of these, never a
wrong-shaped result.

[Source](/reference-sources/Data.Matrix.cinch) · line 70

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">RaggedRows</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">MatrixError</span>)))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">DimensionMismatch</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">MatrixError</span>))))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">VectorMismatch</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">MatrixError</span>))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">IndexOutOfRange</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">MatrixError</span>))))</code></pre>

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-constructor">NotSquare</span> :: (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">MatrixError</span>))</code></pre>

# Values

<a id="value-616464"></a>

## `add`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">add</span> :: (<span class="tok-constructor">Matrix</span> -&gt; (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">MatrixError</span> <span class="tok-constructor">Matrix</span>))</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-6174"></a>

## `at`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">at</span> :: (<span class="tok-constructor">Matrix</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">MatrixError</span> <span class="tok-constructor">F64</span>)))</code></pre>

The cell, or a named `IndexOutOfRange`. Both indices are
zero-based; the guard makes `row-of`/`element-of` total so their
fallback is never observed.

[Source](/reference-sources/Data.Matrix.cinch) · line 180

<a id="value-636f6c73"></a>

## `cols`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">cols</span> :: (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-64657465726d696e616e74"></a>

## `determinant`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">determinant</span> :: (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">MatrixError</span> <span class="tok-constructor">F64</span>)</code></pre>

Cofactor expansion along the first row (see the header: O(n!) time,
each minor rebuild O(n^2)). A non-square matrix is refused before
any expansion.

[Source](/reference-sources/Data.Matrix.cinch) · line 275

<a id="value-657175616c73"></a>

## `equals`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">equals</span> :: (<span class="tok-constructor">Matrix</span> -&gt; (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">Bool</span>))</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-657175616c732d77697468696e"></a>

## `equals-within`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">equals-within</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">Matrix</span> -&gt; (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">Bool</span>)))</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-6964656e74697479"></a>

## `identity`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">identity</span> :: (<span class="tok-constructor">Nat</span> -&gt; <span class="tok-constructor">Matrix</span>)</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-6d6174726978"></a>

## `matrix`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">matrix</span> :: (<span class="tok-constructor">List</span> (<span class="tok-constructor">List</span> <span class="tok-constructor">F64</span>) -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">MatrixError</span> <span class="tok-constructor">Matrix</span>)</code></pre>

Construct a matrix from its rows: the first row fixes the width
and every later row must match it. Ragged input is refused here,
before any value exists, so `at`/`rows`/`cols` never see a ragged
matrix. An empty row list is the 0x0 matrix.

[Source](/reference-sources/Data.Matrix.cinch) · line 162

<a id="value-6d756c"></a>

## `mul`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mul</span> :: (<span class="tok-constructor">Matrix</span> -&gt; (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">MatrixError</span> <span class="tok-constructor">Matrix</span>))</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-6d756c2d766563746f72"></a>

## `mul-vector`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">mul-vector</span> :: (<span class="tok-constructor">Matrix</span> -&gt; (<span class="tok-constructor">List</span> <span class="tok-constructor">F64</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">MatrixError</span> (<span class="tok-constructor">List</span> <span class="tok-constructor">F64</span>)))</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-726f7773"></a>

## `rows`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">rows</span> :: (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">Nat</span>)</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-7363616c65"></a>

## `scale`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">scale</span> :: (<span class="tok-constructor">F64</span> -&gt; (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">Matrix</span>))</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-737562"></a>

## `sub`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">sub</span> :: (<span class="tok-constructor">Matrix</span> -&gt; (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">Result</span> <span class="tok-constructor">MatrixError</span> <span class="tok-constructor">Matrix</span>))</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-746f2d737472696e67"></a>

## `to-string`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">to-string</span> :: (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">String</span>)</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

<a id="value-7472616e73706f7365"></a>

## `transpose`

<pre class="cinch-highlight"><code class="language-cinch"><span class="tok-variable">transpose</span> :: (<span class="tok-constructor">Matrix</span> -&gt; <span class="tok-constructor">Matrix</span>)</code></pre>

[Source](/reference-sources/Data.Matrix.cinch)

# Reference provenance

Generated from the Rust compiler's checked exports and `compiler/prelude/Data/Matrix.cinch`. Source SHA-256: `415f1178cef0922afe6e0d13bad42bd3a495f5236c0e197ee471ba4543968510`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.
