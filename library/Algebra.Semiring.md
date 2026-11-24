# Algebra.Semiring

Zero/one and additive/multiplicative structure, with its law vocabulary.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Algebra.Semiring</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Algebra.Semiring/source/">View source</a><a href="/packages/search/?q=%2BAlgebra.Semiring">Search module</a><a href="/reference-sources/Algebra.Semiring.cinch" download="Algebra.Semiring.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

<details class="doc-notes"><summary>Module notes</summary>

Algebra.Semiring: the two monoids of a ring-like carrier.

The base library scheme's semiring (design/base-library.md):
`Semiring` names BOTH operations of the `Sum`/`Product` story: `Sum`
(Algebra.Monoid) carries `Nat` under addition and `Product` carries
it under multiplication, and a `Semiring` is the coherent pair on
one carrier (zero and one are the two identities). This is why the
bare `Nat` and `Int` carry a `Semiring` where they carry no
`Semigroup`: a single `<>` cannot say which of the two monoids is
meant, while `add`/`mul` name each. NO Num tower: there is no
superclass chain and no numeric conversion hierarchy (the typed
families with explicit conversions are the spec's deliberate
choice), and machine scalars keep their explicit overflow families
with no `Semiring` instance (primitive types specification
section 8). A class constraint entails nothing (the standing
no-superbundle rule): the class states its four operations alone.

Laws (asserted over sample values in seed/tests/prelude_split.rs):
  add associativity:      add (add x y) z == add x (add y z)
  add commutativity:      add x y == add y x
  add identity:           add zero x == x, add x zero == x
  mul associativity:      mul (mul x y) z == mul x (mul y z)
  mul identity:           mul one x == x, mul x one == x
  distributivity:         mul x (add y z) == add (mul x y) (mul x z)
                          mul (add y z) x == add (mul y x) (mul z x)
  annihilation:           mul zero x == zero, mul x zero == zero

Instance homes. The ownership rule is "instances live with their
type": `Semiring Nat` lives in Data.Nat and `Semiring Int` in
Data.Int, beside their other instances.

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-Semiring">Semiring</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Semiring</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Semiring-zero">zero</a><code class="syn-sig"><strong class="tok-variable">zero</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Semiring-one">one</a><code class="syn-sig"><strong class="tok-variable">one</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Semiring-add">add</a><code class="syn-sig"><strong class="tok-variable">add</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Semiring-mul">mul</a><code class="syn-sig"><strong class="tok-variable">mul</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-Semiring">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Semiring</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Semiring" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Semiring/source/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Semiring-zero"><div class="doc-method-row"><code><strong class="tok-variable">zero</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Semiring-zero" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Semiring/source/#L36">Source</a></span></div></li><li id="m-Semiring-one"><div class="doc-method-row"><code><strong class="tok-variable">one</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Semiring-one" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Semiring/source/#L37">Source</a></span></div></li><li id="m-Semiring-add"><div class="doc-method-row"><code><strong class="tok-variable">add</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Semiring-add" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Semiring/source/#L38">Source</a></span></div></li><li id="m-Semiring-mul"><div class="doc-method-row"><code><strong class="tok-variable">mul</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Semiring-mul" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Semiring/source/#L39">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semiring/#c-Semiring" title="Algebra.Semiring.Semiring">Semiring</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>
