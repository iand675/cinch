# Algebra.Group

Group-style inverse/subtraction vocabulary over admitted carrier instances.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Algebra.Group</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Algebra.Group/source/">View source</a><a href="/packages/search/?q=%2BAlgebra.Group">Search module</a><a href="/reference-sources/Algebra.Group.cinch" download="Algebra.Group.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

<details class="doc-notes"><summary>Module notes</summary>

Algebra.Group: carriers with a total inverse.

The base library scheme's group (design/base-library.md):
`Group` adds `invert` to the associative `<>` with identity
`mempty` (Algebra.Semigroup, Algebra.Monoid). A class constraint
entails nothing (the standing no-superbundle rule): the class
states `invert` alone, and the laws below are the law shape the P18
law-axiom layer will formalize.

Laws (asserted over sample values in seed/tests/prelude_split.rs):
  left inverse:  invert x <> x == mempty
  right inverse: x <> invert x == mempty
For `Int` the monoid is the semiring's additive one (`add`/`zero`,
Algebra.Semiring), the form the tests state.

Instance homes. The ownership rule is "instances live with their
type; structural instances over functions and tuples live with the
class module" (design/base-library.md). `Int`'s instance lives in
Data.Int with its family; `Dual`'s lives with its type in
Algebra.Monoid; the function and tuple instances are structural and
here. `Nat` gets no `Group` instance: its subtraction truncates
(there is no additive inverse below zero).

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-Group">Group</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Group</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Group-invert">invert</a><code class="syn-sig"><strong class="tok-variable">invert</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Group/#c-Group" title="Algebra.Group.Group">Group</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-Group">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Group</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Group" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Group/source/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Group-invert"><div class="doc-method-row"><code><strong class="tok-variable">invert</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Group/#c-Group" title="Algebra.Group.Group">Group</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Group-invert" title="Permalink">#</a><a href="/packages/stdlib/Algebra.Group/source/#L28">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (4)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Group/#c-Group" title="Algebra.Group.Group">Group</a> (<span class="tok-variable">t</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">u</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Algebra.Group/#instances">Algebra.Group</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Group/#c-Group" title="Algebra.Group.Group">Group</a> (<span class="tok-variable">t</span>, <span class="tok-variable">u</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Algebra.Group/#instances">Algebra.Group</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Group/#c-Group" title="Algebra.Group.Group">Group</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#t-Dual" title="Algebra.Monoid.Dual">Dual</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Algebra.Monoid/#instances">Algebra.Monoid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Group/#c-Group" title="Algebra.Group.Group">Group</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Int/#instances">Data.Int</a></span></li></ul></details>
</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Group_Function"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Group/#c-Group" title="Algebra.Group.Group">Group</a> (<span class="tok-variable">t</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">u</span>)</code> <span class="doc-inst-meta">Implicit · <code>Algebra.Group.Group_Function</code></span><div class="doc-child-prose">Structural: pointwise inversion of functions into a group.</div></li>
<li id="i-Group_Pair"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Group/#c-Group" title="Algebra.Group.Group">Group</a> (<span class="tok-variable">t</span>, <span class="tok-variable">u</span>)</code> <span class="doc-inst-meta">Implicit · <code>Algebra.Group.Group_Pair</code></span><div class="doc-child-prose">Structural: a pair inverts componentwise, each side under its own constraint.</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Algebra/Group.cinch`](/packages/stdlib/Algebra.Group/source/). Source SHA-256: `df69dec9c9aeb84c0979241629321d1e975609f5ec487f40ae30a9fa8131d0ee`.
