# Data.Matrix

Matrix construction, shape checks, indexing, transformation, and arithmetic operations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Matrix</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Matrix/source/">View source</a><a href="/packages/search/?q=%2BData.Matrix">Search module</a><a href="/reference-sources/Data.Matrix.cinch" download="Data.Matrix.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md).

<details class="doc-closure"><summary>Ship-source closure (1 file)</summary>

Copy these modules beside your entry file before importing `Data.Matrix`: [Data.Matrix](Data.Matrix.md).

```sh
cp seed/prelude/Data/Matrix.cinch ./Data.Matrix.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

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
seed/tests/lib_matrix.rs): `identity n * m == m` for an
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

<details class="doc-synopsis" open><summary>Synopsis (17 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Matrix">Matrix</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Matrix</strong></code></li><li><a class="syn-name" href="#t-MatrixError">MatrixError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MatrixError</strong></code></li><li><a class="syn-name" href="#v-add">add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code></li><li><a class="syn-name" href="#v-at">at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-cols">cols</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-determinant">determinant</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">determinant</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code></li><li><a class="syn-name" href="#v-equals">equals</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">equals</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-equals-within">equals-within</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">equals-within</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-identity">identity</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">identity</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code></li><li><a class="syn-name" href="#v-matrix">matrix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">matrix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code></li><li><a class="syn-name" href="#v-mul">mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code></li><li><a class="syn-name" href="#v-mul-vector">mul-vector</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mul-vector</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code></li><li><a class="syn-name" href="#v-rows">rows</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rows</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-scale">scale</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scale</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code></li><li><a class="syn-name" href="#v-sub">sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code></li><li><a class="syn-name" href="#v-to-string">to-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-transpose">transpose</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">transpose</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Matrix">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Matrix</strong></code><span class="doc-sig-links"><a href="#t-Matrix" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

The dense matrix: the dimensions and the row-major rows. The
constructor is module-private (the ragged invariant lives at
`matrix`, so no other value can exist ragged).

</div>
</article>

<article class="doc-entry" id="t-MatrixError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MatrixError</strong></code><span class="doc-sig-links"><a href="#t-MatrixError" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

The named errors; every shape violation is one of these, never a
wrong-shaped result.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-RaggedRows"><code><strong class="tok-constructor">RaggedRows</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a></code></li><li id="k-DimensionMismatch"><code><strong class="tok-constructor">DimensionMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a></code></li><li id="k-VectorMismatch"><code><strong class="tok-constructor">VectorMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a></code></li><li id="k-IndexOutOfRange"><code><strong class="tok-constructor">IndexOutOfRange</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a></code></li><li id="k-NotSquare"><code><strong class="tok-constructor">NotSquare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code><span class="doc-sig-links"><a href="#v-add" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L205" title="View source at line 205">Source</a></span></div>
</article>

<article class="doc-entry" id="v-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-at" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L180" title="View source at line 180">Source</a></span></div>
<div class="doc-body">

The cell, or a named `IndexOutOfRange`. Both indices are
zero-based; the guard makes `row-of`/`element-of` total so their
fallback is never observed.

</div>
</article>

<article class="doc-entry" id="v-cols">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cols</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-cols" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L91" title="View source at line 91">Source</a></span></div>
</article>

<article class="doc-entry" id="v-determinant">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">determinant</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code><span class="doc-sig-links"><a href="#v-determinant" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L275" title="View source at line 275">Source</a></span></div>
<div class="doc-body">

Cofactor expansion along the first row (see the header: O(n!) time,
each minor rebuild O(n^2)). A non-square matrix is refused before
any expansion.

</div>
</article>

<article class="doc-entry" id="v-equals">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">equals</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-equals" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L317" title="View source at line 317">Source</a></span></div>
</article>

<article class="doc-entry" id="v-equals-within">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">equals-within</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-equals-within" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L344" title="View source at line 344">Source</a></span></div>
</article>

<article class="doc-entry" id="v-identity">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">identity</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code><span class="doc-sig-links"><a href="#v-identity" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L260" title="View source at line 260">Source</a></span></div>
</article>

<article class="doc-entry" id="v-matrix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">matrix</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code><span class="doc-sig-links"><a href="#v-matrix" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L162" title="View source at line 162">Source</a></span></div>
<div class="doc-body">

Construct a matrix from its rows: the first row fixes the width
and every later row must match it. Ragged input is refused here,
before any value exists, so `at`/`rows`/`cols` never see a ragged
matrix. An empty row list is the 0x0 matrix.

</div>
</article>

<article class="doc-entry" id="v-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code><span class="doc-sig-links"><a href="#v-mul" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L239" title="View source at line 239">Source</a></span></div>
</article>

<article class="doc-entry" id="v-mul-vector">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mul-vector</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>)</code><span class="doc-sig-links"><a href="#v-mul-vector" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L250" title="View source at line 250">Source</a></span></div>
</article>

<article class="doc-entry" id="v-rows">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rows</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-rows" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L87" title="View source at line 87">Source</a></span></div>
</article>

<article class="doc-entry" id="v-scale">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scale</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code><span class="doc-sig-links"><a href="#v-scale" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L217" title="View source at line 217">Source</a></span></div>
</article>

<article class="doc-entry" id="v-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-MatrixError" title="Data.Matrix.MatrixError">MatrixError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code><span class="doc-sig-links"><a href="#v-sub" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L211" title="View source at line 211">Source</a></span></div>
</article>

<article class="doc-entry" id="v-to-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-to-string" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L398" title="View source at line 398">Source</a></span></div>
</article>

<article class="doc-entry" id="v-transpose">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">transpose</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Matrix/#t-Matrix" title="Data.Matrix.Matrix">Matrix</a></code><span class="doc-sig-links"><a href="#v-transpose" title="Permalink">#</a><a href="/packages/stdlib/Data.Matrix/source/#L256" title="View source at line 256">Source</a></span></div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Matrix.cinch`](/packages/stdlib/Data.Matrix/source/). Source SHA-256: `1c0d2056a1a83186275ef0506d0df7d884ffb167771c00609e076874c88d6688`.
