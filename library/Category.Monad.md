# Category.Monad

Container-dependent composition through the admitted higher-kinded class vocabulary.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Category.Monad</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Category.Monad/source/">View source</a><a href="/packages/search/?q=%2BCategory.Monad">Search module</a><a href="/reference-sources/Category.Monad.cinch" download="Category.Monad.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Category.Applicative](Category.Applicative.md).

<details class="doc-notes"><summary>Module notes</summary>

Category.Monad: the monad class over type constructors.

Monad is `>>=` (infixl 1, decision D2's per-module fixity
declarations) and `join`. There is NO `return` method (`return` is
handler-clause syntax; decision D152): `pure` lives on
Category.Applicative, and the monad laws state `pure` under an added
`Applicative f` constraint (the bare-`pure` rule, decision D153).
The import below is explicit because a class constraint entails
nothing (the standing no-superbundle rule): `Monad` does not extend
`Applicative`, and every function states the constraints it uses.

`do` notation and these classes (decision D22): `do` picks its
provider lexically per definition; this class abstracts over
containers. The two mechanisms do not interact.

Instance homes: the function instance is structural and lives here
permanently; the List, Option and Result instances got theirs at
the prelude split (Data.List/Data.Option/Data.Result).

The combinators below the class are the free functions `base`'s
Control.Monad carries (`>=>`, `<=<`); each states its constraint and
is reached with an explicit `import Category.Monad` (the facade
re-exports no class home). The traversals that combine this class
with a container live in Category.Traversable, which imports this
module (the class homes collect in dependency order).

</details>

<details class="doc-synopsis" open><summary>Synopsis (5 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-Monad">Monad</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Monad</strong> <span class="tok-variable">f</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Monad-op-3e3e3d">&gt;&gt;=</a><code class="syn-sig"><strong class="tok-variable">&gt;&gt;=</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">b</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Monad-join">join</a><code class="syn-sig"><strong class="tok-variable">join</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">f</span> (<span class="tok-variable">f</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#v-op-3c3d3c">&lt;=&lt;</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">&lt;=&lt;</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">m</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">mon</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <span class="tok-variable">m</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">c</span></code></li><li><a class="syn-name" href="#v-op-3e3d3e">&gt;=&gt;</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">&gt;=&gt;</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">m</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">mon</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <span class="tok-variable">m</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">c</span></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-Monad">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Monad</strong> <span class="tok-variable">f</span></code><span class="doc-sig-links"><a href="#c-Monad" title="Permalink">#</a><a href="/packages/stdlib/Category.Monad/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Monad-op-3e3e3d"><div class="doc-method-row"><code><strong class="tok-variable">&gt;&gt;=</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#m-Monad-op-3e3e3d" title="Permalink">#</a><a href="/packages/stdlib/Category.Monad/source/#L31">Source</a></span></div></li><li id="m-Monad-join"><div class="doc-method-row"><code><strong class="tok-variable">join</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">f</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <span class="tok-variable">f</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">f</span> (<span class="tok-variable">f</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">f</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Monad-join" title="Permalink">#</a><a href="/packages/stdlib/Category.Monad/source/#L32">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (6)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> ((-&gt;) r)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Category.Monad/#instances">Category.Monad</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Identity/#t-Identity" title="Data.Identity.Identity">Identity</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Identity/#instances">Data.Identity</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Result/#instances">Data.Result</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stm/#t-STM" title="Effect.Stm.STM">STM</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Effect.Stm/#instances">Effect.Stm</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-op-3c3d3c">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">&lt;=&lt;</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">m</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">mon</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <span class="tok-variable">m</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#v-op-3c3d3c" title="Permalink">#</a><a href="/packages/stdlib/Category.Monad/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

Right to left: `g <=< f` is `f >=> g`, the direction that reads
like function composition. Eta-expanded to the signature's arity
(the bootstrap checks a definition against every arrow of its
signature, so a body that stops at the second arrow is read as arity
two against an arity-three signature).

</div>
</article>

<article class="doc-entry" id="v-op-3e3d3e">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">&gt;=&gt;</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">m</span> :: <span class="tok-constructor">Type</span> <span class="tok-operator">-&gt;</span> <span class="tok-constructor">Type</span>) <span class="tok-variable">a</span> <span class="tok-variable">b</span> <span class="tok-variable">c</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">mon</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <span class="tok-variable">m</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">b</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">c</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#v-op-3e3d3e" title="Permalink">#</a><a href="/packages/stdlib/Category.Monad/source/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

Left to right: `f >=> g` is the sequenced pair, the order effects
run and results flow in.

</div>
</article>

# Operator fixities

| Operator | Associativity | Precedence |
| --- | --- | --- |
| `<=<` | `infixr` | `1` |
| `>=>` | `infixr` | `1` |
| `>>=` | `infixl` | `1` |

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Monad_Function"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> ((-&gt;) r)</code> <span class="doc-inst-meta">Implicit · <code>Category.Monad.Monad_Function</code></span><div class="doc-child-prose">Structural: feed a function's result into the continuation.</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Category/Monad.cinch`](/packages/stdlib/Category.Monad/source/). Source SHA-256: `6118a2adc3dc804a4d442197f5197ef9975480487dfaa4a2df713b517fa8675f`.
