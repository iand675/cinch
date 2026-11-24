# Prelude

The implicit, curated base surface available in every program. This facade re-exports selected defining modules; it is not the whole library.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> Implicitly in scope.</div><div class="doc-mod-bar-links"><a href="/library/source/Prelude/">View source</a><a href="/library/search/?q=%2BPrelude">Search module</a><a href="/reference-sources/Prelude.cinch" download="Prelude.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Bits](Algebra.Bits.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Enum](Data.Enum.md), [Data.FromList](Data.FromList.md), [Data.FromMap](Data.FromMap.md), [Data.Int](Data.Int.md), [Data.Json](Data.Json.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Map](Data.Map.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Ordering](Data.Ordering.md), [Data.Producer](Data.Producer.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Sym](Data.Sym.md), [Data.Target](Data.Target.md), [Effect.Concurrent](Effect.Concurrent.md), [Effect.Io](Effect.Io.md), [Effect.Sequencing](Effect.Sequencing.md), [Function.Combinators](Function.Combinators.md), `Prelude.Generated`, [Relation.Eq](Relation.Eq.md), [Relation.Ord](Relation.Ord.md), [Text.Show](Text.Show.md).

<details class="doc-notes"><summary>Module notes</summary>

Prelude: the curated re-export facade (decision D107's export
machinery, D143's surface rule, D146's re-export forms).

The prelude is no longer one big module: every type, class and
function family lives in its scheme home (design/base-library.md:
`Category.*`, `Algebra.*`, `Relation.*`, `Data.*`, `Function.*`,
`Text.*`, `Effect.*`), and this module is the implicit base layer
every program sees. Its surface is EXACTLY the names the old
single-module prelude declared (the compatibility rule is absolute:
every program that checked before the split checks unchanged), by
`export module X` re-exports of the homes' own curated surfaces and
`export A.b` items for the `Text.Show` conversions. Curation
trimming is deliberately NOT done here; a trim proposal is a
separate step. One addition is deliberate: `Data.Sym`'s `intern` and
`sym-name` (interned symbols, primitive types specification section
9) extend the surface, since that specification puts both in the
prelude.

The negative space is real: a name this list does not name is
unreachable through `Prelude` (reach it with an explicit `import`
of its home). That covers the new base-library families
(Algebra.Group, Algebra.Semiring, Data.Set, Data.Hash,
Data.HashMap, Text.Show's class, Text.Codec) and every home's
private helpers.

`Prelude.Generated` is the WIT adapter vocabulary module: the
generated vocabulary types (WebAssembly and WASI specification
section 3) are declared there and re-exported here, so they stay
part of the implicit surface exactly as when they appended to the
old single module.

The operator table (decision D90's order) is inherited through these
exports from the defining homes. Only an operator's declaring module
may declare its fixity; this facade carries those facts without
redeclaring them (user direction, 2026-10-30).

</details>

<details class="doc-synopsis"><summary>Synopsis (81 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-AVX2_256">AVX2_256</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AVX2_256</strong></code></li><li><a class="syn-name" href="#t-CapAvailable">CapAvailable</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CapAvailable</strong> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code></li><li><a class="syn-name" href="#t-Config">Config</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Config</strong></code></li><li><a class="syn-name" href="#t-ConfigError">ConfigError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ConfigError</strong></code></li><li><a class="syn-name" href="#t-Down">Down</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Down</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-EightLanes">EightLanes</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">EightLanes</strong></code></li><li><a class="syn-name" href="#t-InRange">InRange</a><code class="syn-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">InRange</strong> (<span class="tok-variable">lo</span> :: <span class="tok-constructor">Int</span>) (<span class="tok-variable">hi</span> :: <span class="tok-constructor">Int</span>) <span class="tok-variable">a</span> = <span class="tok-keyword">refine</span> (<span class="tok-variable">x</span> :: <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> (lo &lt;= x &amp;&amp; x &lt;= hi)</code></li><li><a class="syn-name" href="#t-Json">Json</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Json</strong></code></li><li><a class="syn-name" href="#t-List">List</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">List</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-LoadError">LoadError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LoadError</strong></code></li><li><a class="syn-name" href="#t-Map">Map</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Map</strong> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li><a class="syn-name" href="#t-NEON64">NEON64</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">NEON64</strong></code></li><li><a class="syn-name" href="#t-NonZero">NonZero</a><code class="syn-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">NonZero</strong> <span class="tok-variable">a</span> = <span class="tok-keyword">refine</span> (<span class="tok-variable">n</span> :: <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> NonZero n</code></li><li><a class="syn-name" href="#t-Option">Option</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Option</strong> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-Ordering">Ordering</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ordering</strong></code></li><li><a class="syn-name" href="#t-Path">Path</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Path</strong></code></li><li><a class="syn-name" href="#t-Producer">Producer</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Producer</strong> <span class="tok-variable">s</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-Request">Request</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Request</strong></code></li><li><a class="syn-name" href="#t-Result">Result</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Result</strong> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-ScalableShape">ScalableShape</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ScalableShape</strong></code></li><li><a class="syn-name" href="#t-Step">Step</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Step</strong> <span class="tok-variable">s</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#t-String">String</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">String</strong></code></li><li><a class="syn-name" href="#t-TakeSt">TakeSt</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TakeSt</strong> <span class="tok-variable">s</span></code></li><li><a class="syn-name" href="#c-Add">Add</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Add</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Add-op-2b">+</a><code class="syn-sig"><strong class="tok-variable">+</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-Bits">Bits</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Bits</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-bit-and">bit-and</a><code class="syn-sig"><strong class="tok-variable">bit-and</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-bit-or">bit-or</a><code class="syn-sig"><strong class="tok-variable">bit-or</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-bit-xor">bit-xor</a><code class="syn-sig"><strong class="tok-variable">bit-xor</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-bit-not">bit-not</a><code class="syn-sig"><strong class="tok-variable">bit-not</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-shift-left">shift-left</a><code class="syn-sig"><strong class="tok-variable">shift-left</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-shift-right">shift-right</a><code class="syn-sig"><strong class="tok-variable">shift-right</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-rotate-left">rotate-left</a><code class="syn-sig"><strong class="tok-variable">rotate-left</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-rotate-right">rotate-right</a><code class="syn-sig"><strong class="tok-variable">rotate-right</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-test-bit">test-bit</a><code class="syn-sig"><strong class="tok-variable">test-bit</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-set-bit">set-bit</a><code class="syn-sig"><strong class="tok-variable">set-bit</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-clear-bit">clear-bit</a><code class="syn-sig"><strong class="tok-variable">clear-bit</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-complement-bit">complement-bit</a><code class="syn-sig"><strong class="tok-variable">complement-bit</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-count-ones">count-ones</a><code class="syn-sig"><strong class="tok-variable">count-ones</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-leading-zeros">leading-zeros</a><code class="syn-sig"><strong class="tok-variable">leading-zeros</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li class="syn-sub"><a class="syn-name" href="#m-Bits-trailing-zeros">trailing-zeros</a><code class="syn-sig"><strong class="tok-variable">trailing-zeros</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#c-DirectSequencing">DirectSequencing</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">DirectSequencing</strong></code></li><li class="syn-sub"><a class="syn-name" href="#m-DirectSequencing-pure">pure</a><code class="syn-sig"><strong class="tok-variable">pure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-DirectSequencing" title="Effect.Sequencing.DirectSequencing">DirectSequencing</a>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-DirectSequencing-bind">bind</a><code class="syn-sig"><strong class="tok-variable">bind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> (<span class="tok-variable">ef</span> :: <span class="tok-constructor">Row</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-DirectSequencing" title="Effect.Sequencing.DirectSequencing">DirectSequencing</a>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{|ef} <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span>{|ef} <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#c-Div">Div</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Div</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Div-op-2f">/</a><code class="syn-sig"><strong class="tok-variable">/</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Div" title="Algebra.Arithmetic.Div">Div</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-Enum">Enum</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Enum</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Enum-succ">succ</a><code class="syn-sig"><strong class="tok-variable">succ</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Enum-pred">pred</a><code class="syn-sig"><strong class="tok-variable">pred</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Enum-to-enum">to-enum</a><code class="syn-sig"><strong class="tok-variable">to-enum</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Enum-from-enum">from-enum</a><code class="syn-sig"><strong class="tok-variable">from-enum</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#c-Eq">Eq</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Eq</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Eq-op-3d3d">==</a><code class="syn-sig"><strong class="tok-variable">==</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li class="syn-sub"><a class="syn-name" href="#m-Eq-op-2f3d">/=</a><code class="syn-sig"><strong class="tok-variable">/=</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#c-FromList">FromList</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">FromList</strong> <span class="tok-variable">c</span> <span class="tok-variable">e</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-FromList-from-list">from-list</a><code class="syn-sig"><strong class="tok-variable">from-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span> <span class="tok-variable">e</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> <span class="tok-variable">c</span> <span class="tok-variable">e</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code></li><li><a class="syn-name" href="#c-FromMap">FromMap</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">FromMap</strong> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-FromMap-from-map">from-map</a><code class="syn-sig"><strong class="tok-variable">from-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span></code></li><li><a class="syn-name" href="#c-Mul">Mul</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Mul</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Mul-op-2a">*</a><code class="syn-sig"><strong class="tok-variable">*</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-Neg">Neg</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Neg</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Neg-negate">negate</a><code class="syn-sig"><strong class="tok-variable">negate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-OptionSequencing">OptionSequencing</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">OptionSequencing</strong></code></li><li class="syn-sub"><a class="syn-name" href="#m-OptionSequencing-pure">pure</a><code class="syn-sig"><strong class="tok-variable">pure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-OptionSequencing" title="Effect.Sequencing.OptionSequencing">OptionSequencing</a>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-OptionSequencing-bind">bind</a><code class="syn-sig"><strong class="tok-variable">bind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> (<span class="tok-variable">ef</span> :: <span class="tok-constructor">Row</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-OptionSequencing" title="Effect.Sequencing.OptionSequencing">OptionSequencing</a>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{|ef} <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span>{|ef} <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#c-Ord">Ord</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Ord</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Ord-op-3c">&lt;</a><code class="syn-sig"><strong class="tok-variable">&lt;</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li class="syn-sub"><a class="syn-name" href="#m-Ord-op-3c3d">&lt;=</a><code class="syn-sig"><strong class="tok-variable">&lt;=</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li class="syn-sub"><a class="syn-name" href="#m-Ord-op-3e">&gt;</a><code class="syn-sig"><strong class="tok-variable">&gt;</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li class="syn-sub"><a class="syn-name" href="#m-Ord-op-3e3d">&gt;=</a><code class="syn-sig"><strong class="tok-variable">&gt;=</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#c-Rem">Rem</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Rem</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Rem-op-25">%</a><code class="syn-sig"><strong class="tok-variable">%</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Rem" title="Algebra.Arithmetic.Rem">Rem</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-ResultSequencing">ResultSequencing</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">ResultSequencing</strong> <span class="tok-variable">e</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-ResultSequencing-pure">pure</a><code class="syn-sig"><strong class="tok-variable">pure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-ResultSequencing" title="Effect.Sequencing.ResultSequencing">ResultSequencing</a> <span class="tok-variable">e</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-ResultSequencing-bind">bind</a><code class="syn-sig"><strong class="tok-variable">bind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> (<span class="tok-variable">ef</span> :: <span class="tok-constructor">Row</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-ResultSequencing" title="Effect.Sequencing.ResultSequencing">ResultSequencing</a> <span class="tok-variable">e</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{|ef} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span>{|ef} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">b</span></code></li><li><a class="syn-name" href="#c-Sub">Sub</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Sub</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Sub--">-</a><code class="syn-sig"><strong class="tok-variable">-</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code></li><li><a class="syn-name" href="#c-SumProofs">SumProofs</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">SumProofs</strong></code></li><li class="syn-sub"><a class="syn-name" href="#m-SumProofs-scalar-to-simd">scalar-to-simd</a><code class="syn-sig"><strong class="tok-variable">scalar-to-simd</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span> (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>) <span class="tok-variable">region</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Target/#c-SumProofs" title="Data.Target.SumProofs">SumProofs</a>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">vectors</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">xs</span> :: <a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Ref">Ref</a> <span class="tok-variable">region</span> <a class="doc-type-link tok-constructor" href="/library/Data.Mutable/#t-Vec" title="Vec">Vec</a> <span class="tok-variable">n</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Eq">Eq</a> (<span class="tok-constructor">scalar-sum-wrap32</span> @ <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> @ <span class="tok-variable">region</span> @ 1 (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <span class="tok-variable">n</span>) @ 4 <span class="tok-variable">region</span> <span class="tok-variable">xs</span>) (<span class="tok-constructor">simd-sum-wrap32</span> @ <span class="tok-variable">t</span> @ <span class="tok-variable">w</span> @ <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> @ <span class="tok-variable">region</span> @ 1 (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <span class="tok-variable">n</span>) @ 4 <span class="tok-variable">region</span> <span class="tok-variable">vectors</span> <span class="tok-variable">xs</span>)</code></li><li><a class="syn-name" href="#c-VecU32Target">VecU32Target</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">VecU32Target</strong> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-VecU32Target-positive-width">positive-width</a><code class="syn-sig"><strong class="tok-variable">positive-width</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-AVX2_256">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AVX2_256</strong></code><span class="doc-sig-links"><a href="#t-AVX2_256" title="Permalink">#</a><a href="/library/source/Data.Target/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Target profiles and vector shapes as marker types: the platform
chapter's `Target` kind and the compile-time lane width are
represented as types in the bootstrap (decision D58).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-AVX2_256Tag"><code><strong class="tok-constructor">AVX2_256Tag</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Target/#t-AVX2_256" title="Data.Target.AVX2_256">AVX2_256</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-CapAvailable">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CapAvailable</strong> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code><span class="doc-sig-links"><a href="#t-CapAvailable" title="Permalink">#</a><a href="/library/source/Data.Target/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Operational evidence that a capability is valid in the current
validity scope (platform section 2): the lexical region and thread
supplying it. A rewrite's `require` clause names it.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Available"><code><strong class="tok-constructor">Available</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span>. <a class="doc-type-link tok-constructor" href="/library/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Target/#t-CapAvailable" title="Data.Target.CapAvailable">CapAvailable</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Config">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Config</strong></code><span class="doc-sig-links"><a href="#t-Config" title="Permalink">#</a><a href="/library/source/Effect.Io/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Config"><code><strong class="tok-constructor">Config</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Io/#t-Config" title="Effect.Io.Config">Config</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ConfigError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ConfigError</strong></code><span class="doc-sig-links"><a href="#t-ConfigError" title="Permalink">#</a><a href="/library/source/Effect.Io/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

The worked vocabulary of the sequencing chapter's `loadConfig`
sketch (runtime capabilities, effects and sequencing
specification), provided over the filesystem like Effect.Fs's
operations. A `Path` (Data.Path) names bytes to read;
`read-config-bytes` performs the read under the `Io r`
capability and reports a missing or unreadable file as a
`ConfigError`. `parse-config` is pure over the read bytes: the
bootstrap's config document is the payload, and an empty
document is not a config.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-BadConfig"><code><strong class="tok-constructor">BadConfig</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Io/#t-ConfigError" title="Effect.Io.ConfigError">ConfigError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Down">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Down</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Down" title="Permalink">#</a><a href="/library/source/Relation.Ord/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

The order-reversing newtype: `Down x` is `x` with the order
swapped, so `comparing (\p -> Down (key p))` sorts by `key`
descending. Equality is unaffected by the flip (only the four
ordering methods reverse).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Down"><code><strong class="tok-constructor">Down</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#t-Down" title="Relation.Ord.Down">Down</a> <span class="tok-variable">a</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> (<a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#t-Down" title="Relation.Ord.Down">Down</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Relation.Ord/#instances">Relation.Ord</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> (<a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#t-Down" title="Relation.Ord.Down">Down</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Relation.Ord/#instances">Relation.Ord</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-EightLanes">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">EightLanes</strong></code><span class="doc-sig-links"><a href="#t-EightLanes" title="Permalink">#</a><a href="/library/source/Data.Target/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-EightLanesTag"><code><strong class="tok-constructor">EightLanesTag</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Target/#t-EightLanes" title="Data.Target.EightLanes">EightLanes</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-InRange">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">InRange</strong> (<span class="tok-variable">lo</span> :: <span class="tok-constructor">Int</span>) (<span class="tok-variable">hi</span> :: <span class="tok-constructor">Int</span>) <span class="tok-variable">a</span> = <span class="tok-keyword">refine</span> (<span class="tok-variable">x</span> :: <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> (lo &lt;= x &amp;&amp; x &lt;= hi)</code><span class="doc-sig-links"><a href="#t-InRange" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

A range of a number, the result type of every operation whose
result lies in known bounds by construction (`u32-to-i64 :: U32 ->
InRange 0 4294967295 I64`, `bytes-get`'s `InRange 0 255 Nat`,
`char-code`'s `InRange 0 1114111 Nat`). The bounds are inclusive
index parameters, so the predicate is two comparisons the
difference-logic discharge reads directly: a let-bound result
carries `lo <= x && x <= hi` as a fact, an application in an
obligation carries it for its term, and where no range is demanded
the value is its number (`c - 48`, `nat-add c 1`).

</div>
</article>

<article class="doc-entry" id="t-Json">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Json</strong></code><span class="doc-sig-links"><a href="#t-Json" title="Permalink">#</a><a href="/library/source/Data.Json/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

The `Json`-shaped value a file converter binds for a JSON file with
no declared type (surface grammar local amendments section 11); with
a declared record type the converter decodes into it instead. An
array is a cons spine (`JArr item tail`, ended by `JArrEnd`) and an
object entry is `JObj key value tail` (ended by `JObjEnd`): the
positivity discipline allows only direct recursive occurrences, so
the payloads stay inside the family.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-JNull"><code><strong class="tok-constructor">JNull</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JBool"><code><strong class="tok-constructor">JBool</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JNum"><code><strong class="tok-constructor">JNum</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JFloat"><code><strong class="tok-constructor">JFloat</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JStr"><code><strong class="tok-constructor">JStr</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JArr"><code><strong class="tok-constructor">JArr</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JObj"><code><strong class="tok-constructor">JObj</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JArrEnd"><code><strong class="tok-constructor">JArrEnd</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li><li id="k-JObjEnd"><code><strong class="tok-constructor">JObjEnd</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Text.Codec/#c-JsonCodec" title="Text.Codec.JsonCodec">JsonCodec</a> <a class="doc-type-link tok-constructor" href="/library/Data.Json/#t-Json" title="Data.Json.Json">Json</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Text.Json/#instances">Text.Json</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-List">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">List</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-List" title="Permalink">#</a><a href="/library/source/Data.Core/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Nil"><code><strong class="tok-constructor">Nil</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li><li id="k-Cons"><code><strong class="tok-constructor">Cons</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (9)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Foldable/#c-Foldable" title="Category.Foldable.Foldable">Foldable</a> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span>) <span class="tok-variable">e</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Traversable/#c-Traversable" title="Category.Traversable.Traversable">Traversable</a> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-LoadError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">LoadError</strong></code><span class="doc-sig-links"><a href="#t-LoadError" title="Permalink">#</a><a href="/library/source/Effect.Concurrent/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-LoadError"><code><strong class="tok-constructor">LoadError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Concurrent/#t-LoadError" title="Effect.Concurrent.LoadError">LoadError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Map">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Map</strong> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#t-Map" title="Permalink">#</a><a href="/library/source/Data.Map/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

The ordered map: an unbalanced binary search tree over `Ord k`
(decision D105). `map-insert` keeps the left subtree strictly
smaller and the right strictly greater, so a repeated key replaces
the stored value in place.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Tip"><code><strong class="tok-constructor">Tip</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <a class="doc-type-link tok-constructor" href="/library/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li><li id="k-Bin"><code><strong class="tok-constructor">Bin</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-variable">k</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-variable">k</span> <span class="tok-variable">v</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Map/#instances">Data.Map</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-NEON64">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">NEON64</strong></code><span class="doc-sig-links"><a href="#t-NEON64" title="Permalink">#</a><a href="/library/source/Data.Target/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-NEON64Tag"><code><strong class="tok-constructor">NEON64Tag</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Target/#t-NEON64" title="Data.Target.NEON64">NEON64</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-NonZero">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">NonZero</strong> <span class="tok-variable">a</span> = <span class="tok-keyword">refine</span> (<span class="tok-variable">n</span> :: <span class="tok-variable">a</span>) <span class="tok-keyword">with</span> NonZero n</code><span class="doc-sig-links"><a href="#t-NonZero" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

The divisor type, declared here because the `Div`/`Rem` classes
and the `Nat`/`Int` homes (checked right after this module) name
it in their signatures; its total constructors and instances live
in its public home, `Data.NonZero`. One refinement states the
predicate for every carrier it decides (`NonZero Nat`, `NonZero
Int`, a machine integer): the carrier is the container, so a
`NonZero Nat` IS its `Nat` at runtime, and a numeral decides its
predicate at compile time.

<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> (<a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.NonZero/#instances">Data.NonZero</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.NonZero/#instances">Data.NonZero</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-Option">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Option</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Option" title="Permalink">#</a><a href="/library/source/Data.Core/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-None"><code><strong class="tok-constructor">None</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li><li id="k-Some"><code><strong class="tok-constructor">Some</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (8)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Alternative/#c-Alternative" title="Category.Alternative.Alternative">Alternative</a> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Foldable/#c-Foldable" title="Category.Foldable.Foldable">Foldable</a> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">t</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Traversable/#c-Traversable" title="Category.Traversable.Traversable">Traversable</a> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-Ordering">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Ordering</strong></code><span class="doc-sig-links"><a href="#t-Ordering" title="Permalink">#</a><a href="/library/source/Data.Ordering/#L13" title="View source at line 13">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-LT"><code><strong class="tok-constructor">LT</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li id="k-EQ"><code><strong class="tok-constructor">EQ</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li><li id="k-GT"><code><strong class="tok-constructor">GT</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Ordering/#instances">Data.Ordering</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Ordering/#instances">Data.Ordering</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/library/Data.Ordering/#t-Ordering" title="Data.Ordering.Ordering">Ordering</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Text.Show/#instances">Text.Show</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-Path">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Path</strong></code><span class="doc-sig-links"><a href="#t-Path" title="Permalink">#</a><a href="/library/source/Data.Path/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

The path text (POSIX; UTF-8 at the boundary).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Path"><code><strong class="tok-constructor">Path</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Path/#t-Path" title="Data.Path.Path">Path</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Producer">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Producer</strong> <span class="tok-variable">s</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Producer" title="Permalink">#</a><a href="/library/source/Data.Producer/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

A producer: a state with its step function.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Producer"><code><strong class="tok-constructor">Producer</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>. (<span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Producer/#t-Step" title="Data.Producer.Step">Step</a> <span class="tok-variable">s</span> <span class="tok-variable">a</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <span class="tok-variable">s</span> <span class="tok-variable">a</span></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Request">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Request</strong></code><span class="doc-sig-links"><a href="#t-Request" title="Permalink">#</a><a href="/library/source/Effect.Concurrent/#L24" title="View source at line 24">Source</a></span></div>
<div class="doc-body">

Structured concurrency (concurrency specification, section 2): the
worked vocabulary of the spec's `loadPair` sketch. A `Request` names
bytes to read from the runtime's input source; `fetch` performs the
read under the `Io r` capability and reports a short read as a
`LoadError`. `withScope`, `spawn`, `joinAllOrCancel` and the `Io r`
operations (`ioWrite`, `ioRead`, `ioSleep`, `pollCancel`) are builtin
values with compiler-known schemes (like `newIO`), never foreign
imports.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Request"><code><strong class="tok-constructor">Request</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Concurrent/#t-Request" title="Effect.Concurrent.Request">Request</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Result">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Result</strong> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Result" title="Permalink">#</a><a href="/library/source/Data.Core/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

`Result` is here for the same reason: `Data.Result`'s
`partition-results` and `results` consume a `List`, and it too is
checked before `Data.List`.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Err"><code><strong class="tok-constructor">Err</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code></li><li id="k-Ok"><code><strong class="tok-constructor">Ok</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Applicative/#c-Applicative" title="Category.Applicative.Applicative">Applicative</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Result/#instances">Data.Result</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Functor/#c-Functor" title="Category.Functor.Functor">Functor</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Result/#instances">Data.Result</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Category.Monad/#c-Monad" title="Category.Monad.Monad">Monad</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Result/#instances">Data.Result</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-ScalableShape">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ScalableShape</strong></code><span class="doc-sig-links"><a href="#t-ScalableShape" title="Permalink">#</a><a href="/library/source/Data.Target/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ScalableShapeTag"><code><strong class="tok-constructor">ScalableShapeTag</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Target/#t-ScalableShape" title="Data.Target.ScalableShape">ScalableShape</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Step">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Step</strong> <span class="tok-variable">s</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#t-Step" title="Permalink">#</a><a href="/library/source/Data.Producer/#L17" title="View source at line 17">Source</a></span></div>
<div class="doc-body">

The vector step form (modules section 6): `Done`, `Skip(state)` and
`Yield(value, state)`. Producer pipelines lower to state machines.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Done"><code><strong class="tok-constructor">Done</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>. <a class="doc-type-link tok-constructor" href="/library/Data.Producer/#t-Step" title="Data.Producer.Step">Step</a> <span class="tok-variable">s</span> <span class="tok-variable">a</span></code></li><li id="k-Skip"><code><strong class="tok-constructor">Skip</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>. <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Producer/#t-Step" title="Data.Producer.Step">Step</a> <span class="tok-variable">s</span> <span class="tok-variable">a</span></code></li><li id="k-Yield"><code><strong class="tok-constructor">Yield</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span> <span class="tok-variable">a</span>. <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Producer/#t-Step" title="Data.Producer.Step">Step</a> <span class="tok-variable">s</span> <span class="tok-variable">a</span></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-String">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">String</strong></code><span class="doc-sig-links"><a href="#t-String" title="Permalink">#</a><a href="/library/source/Data.String/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

The representation constructor: module-private (the immutability
contract: no API hands out a string's backing). A `StrBuf` wraps a
buffer that is either fresh (`bytes-new` filled here) or a copy,
never one a caller retains.

<details class="doc-instances"><summary>Instances (8)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Compare/#c-Compare" title="Data.Compare.Compare">Compare</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Compare/#instances">Data.Compare</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Compare/#c-Default" title="Data.Compare.Default">Default</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Compare/#instances">Data.Compare</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Text.Show/#instances">Text.Show</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-TakeSt">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TakeSt</strong> <span class="tok-variable">s</span></code><span class="doc-sig-links"><a href="#t-TakeSt" title="Permalink">#</a><a href="/library/source/Data.Producer/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TakeSt"><code><strong class="tok-constructor">TakeSt</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Producer/#t-TakeSt" title="Data.Producer.TakeSt">TakeSt</a> <span class="tok-variable">s</span></code></li></ul></div>
</div>
</article>

# Signatures

<article class="doc-entry" id="c-Add">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Add</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Add" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Add-op-2b"><div class="doc-method-row"><code><strong class="tok-variable">+</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Add-op-2b" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L45">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/library/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Add" title="Algebra.Arithmetic.Add">Add</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Bits">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Bits</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Bits" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Bit manipulation (primitive types specification section 4).

The `Bits` class unifies the per-type primitive families so generic
code can be written over machine integers. All methods are total,
following the specification: the bitwise operations are total,
shifts reduce the count modulo the width (the wrapping family), and
rotates are total. The checked shift family stays as the per-type
`checkedShlX`/`checkedShrX` primitives. `test-bit`/`set-bit`/
`clear-bit`/`complement-bit` are expressed through the mask rule: bit
position n means position n modulo the width, consistent with the
shift rule.

<div class="doc-children"><h4>Methods</h4><ul><li id="m-Bits-bit-and"><div class="doc-method-row"><code><strong class="tok-variable">bit-and</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-bit-and" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L42">Source</a></span></div></li><li id="m-Bits-bit-or"><div class="doc-method-row"><code><strong class="tok-variable">bit-or</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-bit-or" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L43">Source</a></span></div></li><li id="m-Bits-bit-xor"><div class="doc-method-row"><code><strong class="tok-variable">bit-xor</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-bit-xor" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L44">Source</a></span></div></li><li id="m-Bits-bit-not"><div class="doc-method-row"><code><strong class="tok-variable">bit-not</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-bit-not" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L45">Source</a></span></div></li><li id="m-Bits-shift-left"><div class="doc-method-row"><code><strong class="tok-variable">shift-left</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-shift-left" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L46">Source</a></span></div></li><li id="m-Bits-shift-right"><div class="doc-method-row"><code><strong class="tok-variable">shift-right</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-shift-right" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L47">Source</a></span></div></li><li id="m-Bits-rotate-left"><div class="doc-method-row"><code><strong class="tok-variable">rotate-left</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-rotate-left" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L48">Source</a></span></div></li><li id="m-Bits-rotate-right"><div class="doc-method-row"><code><strong class="tok-variable">rotate-right</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-rotate-right" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L49">Source</a></span></div></li><li id="m-Bits-test-bit"><div class="doc-method-row"><code><strong class="tok-variable">test-bit</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#m-Bits-test-bit" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L50">Source</a></span></div></li><li id="m-Bits-set-bit"><div class="doc-method-row"><code><strong class="tok-variable">set-bit</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-set-bit" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L51">Source</a></span></div></li><li id="m-Bits-clear-bit"><div class="doc-method-row"><code><strong class="tok-variable">clear-bit</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-clear-bit" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L52">Source</a></span></div></li><li id="m-Bits-complement-bit"><div class="doc-method-row"><code><strong class="tok-variable">complement-bit</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Bits-complement-bit" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L53">Source</a></span></div></li><li id="m-Bits-count-ones"><div class="doc-method-row"><code><strong class="tok-variable">count-ones</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#m-Bits-count-ones" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L54">Source</a></span></div></li><li id="m-Bits-leading-zeros"><div class="doc-method-row"><code><strong class="tok-variable">leading-zeros</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#m-Bits-leading-zeros" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L55">Source</a></span></div></li><li id="m-Bits-trailing-zeros"><div class="doc-method-row"><code><strong class="tok-variable">trailing-zeros</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#m-Bits-trailing-zeros" title="Permalink">#</a><a href="/library/source/Algebra.Bits/#L56">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (12)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I16">I16</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I8">I8</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="Isize">Isize</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="Usize">Usize</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="I128">I128</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Wide/#instances">Data.Wide</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Bits/#c-Bits" title="Algebra.Bits.Bits">Bits</a> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Wide/#instances">Data.Wide</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-DirectSequencing">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">DirectSequencing</strong></code><span class="doc-sig-links"><a href="#c-DirectSequencing" title="Permalink">#</a><a href="/library/source/Effect.Sequencing/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

Sequencing providers for `do` notation (runtime capabilities,
effects and sequencing specification, "Unqualified do through
modules and typeclasses"; design section 5). The same `do` spelling
types differently per provider: direct sequencing is the default and
a `using` binder with a provider head selects another. `pure` and
`bind` are the provider's class methods (decision D2).

<div class="doc-children"><h4>Methods</h4><ul><li id="m-DirectSequencing-pure"><div class="doc-method-row"><code><strong class="tok-variable">pure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-DirectSequencing" title="Effect.Sequencing.DirectSequencing">DirectSequencing</a>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-DirectSequencing-pure" title="Permalink">#</a><a href="/library/source/Effect.Sequencing/#L29">Source</a></span></div></li><li id="m-DirectSequencing-bind"><div class="doc-method-row"><code><strong class="tok-variable">bind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> (<span class="tok-variable">ef</span> :: <span class="tok-constructor">Row</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-DirectSequencing" title="Effect.Sequencing.DirectSequencing">DirectSequencing</a>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{|ef} <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span>{|ef} <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#m-DirectSequencing-bind" title="Permalink">#</a><a href="/library/source/Effect.Sequencing/#L30">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-DirectSequencing" title="Effect.Sequencing.DirectSequencing">DirectSequencing</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Effect.Sequencing/#instances">Effect.Sequencing</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Div">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Div</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Div" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Div-op-2f"><div class="doc-method-row"><code><strong class="tok-variable">/</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Div" title="Algebra.Arithmetic.Div">Div</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Div-op-2f" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L69">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Div" title="Algebra.Arithmetic.Div">Div</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Div" title="Algebra.Arithmetic.Div">Div</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Enum">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Enum</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Enum" title="Permalink">#</a><a href="/library/source/Data.Enum/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Enum-succ"><div class="doc-method-row"><code><strong class="tok-variable">succ</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Enum-succ" title="Permalink">#</a><a href="/library/source/Data.Enum/#L31">Source</a></span></div></li><li id="m-Enum-pred"><div class="doc-method-row"><code><strong class="tok-variable">pred</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Enum-pred" title="Permalink">#</a><a href="/library/source/Data.Enum/#L32">Source</a></span></div></li><li id="m-Enum-to-enum"><div class="doc-method-row"><code><strong class="tok-variable">to-enum</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Enum-to-enum" title="Permalink">#</a><a href="/library/source/Data.Enum/#L33">Source</a></span></div></li><li id="m-Enum-from-enum"><div class="doc-method-row"><code><strong class="tok-variable">from-enum</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#m-Enum-from-enum" title="Permalink">#</a><a href="/library/source/Data.Enum/#L34">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.Enum/#c-Enum" title="Data.Enum.Enum">Enum</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Eq">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Eq</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Eq" title="Permalink">#</a><a href="/library/source/Relation.Eq/#L15" title="View source at line 15">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Eq-op-3d3d"><div class="doc-method-row"><code><strong class="tok-variable">==</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#m-Eq-op-3d3d" title="Permalink">#</a><a href="/library/source/Relation.Eq/#L16">Source</a></span></div></li><li id="m-Eq-op-2f3d"><div class="doc-method-row"><code><strong class="tok-variable">/=</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#m-Eq-op-2f3d" title="Permalink">#</a><a href="/library/source/Relation.Eq/#L17">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (40)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.BigDecimal/#instances">Data.BigDecimal</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Bool/#instances">Data.Bool</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Bytes/#instances">Data.Bytes</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Char/#instances">Data.Char</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Date/#instances">Data.Date</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.IpAddr/#instances">Data.IpAddr</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.IpAddr/#instances">Data.IpAddr</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.IpAddr/#instances">Data.IpAddr</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I16">I16</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I8">I8</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="Isize">Isize</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="Usize">Usize</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Nat/#instances">Data.Nat</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Rational/#instances">Data.Rational</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Sym/" title="Sym">Sym</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Sym/#instances">Data.Sym</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time/#instances">Data.Time</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Date/#instances">Data.Time.Date</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.DateTime/#instances">Data.Time.DateTime</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Duration/#instances">Data.Time.Duration</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Monotonic/#instances">Data.Time.Monotonic</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Offset/#instances">Data.Time.Offset</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.TimeOfDay/#instances">Data.Time.TimeOfDay</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Timestamp/#instances">Data.Time.Timestamp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Weekday/#instances">Data.Time.Weekday</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Zoned/#t-Zoned" title="Data.Time.Zoned.Zoned">Zoned</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Zoned/#instances">Data.Time.Zoned</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Uuid/#instances">Data.Uuid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="I128">I128</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Wide/#instances">Data.Wide</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Wide/#instances">Data.Wide</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> (<a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#t-Down" title="Relation.Ord.Down">Down</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Relation.Ord/#instances">Relation.Ord</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <span class="tok-constructor">MimeKey</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Text.Mime/#instances">Text.Mime</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/library/Text.Yaml/#t-YamlErrorKind" title="Text.Yaml.YamlErrorKind">YamlErrorKind</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Text.Yaml/#instances">Text.Yaml</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-FromList">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">FromList</strong> <span class="tok-variable">c</span> <span class="tok-variable">e</span></code><span class="doc-sig-links"><a href="#c-FromList" title="Permalink">#</a><a href="/library/source/Data.FromList/#L19" title="View source at line 19">Source</a></span></div>
<div class="doc-body">

Collection literals (surface grammar local amendments, section 10):
`[a, b, c]` desugars to `from-list` at the expected type's `FromList
c e` instance and `{k => v}` to `from-map` over a `List (k, v)` of
pairs at the expected type's `FromMap m k v` instance; unconstrained
they default to `List` and `Map`. Instances opt in per container.

<div class="doc-children"><h4>Methods</h4><ul><li id="m-FromList-from-list"><div class="doc-method-row"><code><strong class="tok-variable">from-list</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">c</span> <span class="tok-variable">e</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> <span class="tok-variable">c</span> <span class="tok-variable">e</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">c</span></code><span class="doc-sig-links"><a href="#m-FromList-from-list" title="Permalink">#</a><a href="/library/source/Data.FromList/#L20">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (4)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Array/" title="Array">Array</a> <span class="tok-variable">a</span>) <span class="tok-variable">a</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Array/#instances">Data.Array</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Bytes/#instances">Data.Bytes</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">e</span>) <span class="tok-variable">e</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.List/#instances">Data.List</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Set/#t-Set" title="Data.Set.Set">Set</a> <span class="tok-variable">a</span>) <span class="tok-variable">a</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Set/#instances">Data.Set</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-FromMap">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">FromMap</strong> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span></code><span class="doc-sig-links"><a href="#c-FromMap" title="Permalink">#</a><a href="/library/source/Data.FromMap/#L13" title="View source at line 13">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-FromMap-from-map"><div class="doc-method-row"><code><strong class="tok-variable">from-map</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> <span class="tok-variable">m</span> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.List/#t-List" title="Data.Core.List">List</a> (<span class="tok-variable">k</span>, <span class="tok-variable">v</span>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">m</span></code><span class="doc-sig-links"><a href="#m-FromMap-from-map" title="Permalink">#</a><a href="/library/source/Data.FromMap/#L14">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> (<a class="doc-type-link tok-constructor" href="/library/Data.HashMap/#t-HashMap" title="Data.HashMap.HashMap">HashMap</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-variable">k</span> <span class="tok-variable">v</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.HashMap/#instances">Data.HashMap</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Data.FromMap/#c-FromMap" title="Data.FromMap.FromMap">FromMap</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <span class="tok-variable">k</span> <span class="tok-variable">v</span>) <span class="tok-variable">k</span> <span class="tok-variable">v</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Map/#instances">Data.Map</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Mul">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Mul</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Mul" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Mul-op-2a"><div class="doc-method-row"><code><strong class="tok-variable">*</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Mul-op-2a" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L51">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/library/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Mul" title="Algebra.Arithmetic.Mul">Mul</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Neg">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Neg</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Neg" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Neg-negate"><div class="doc-method-row"><code><strong class="tok-variable">negate</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Neg-negate" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L54">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <a class="doc-type-link tok-constructor" href="/library/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Neg" title="Algebra.Arithmetic.Neg">Neg</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Int/#instances">Data.Int</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-OptionSequencing">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">OptionSequencing</strong></code><span class="doc-sig-links"><a href="#c-OptionSequencing" title="Permalink">#</a><a href="/library/source/Effect.Sequencing/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-OptionSequencing-pure"><div class="doc-method-row"><code><strong class="tok-variable">pure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-OptionSequencing" title="Effect.Sequencing.OptionSequencing">OptionSequencing</a>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-OptionSequencing-pure" title="Permalink">#</a><a href="/library/source/Effect.Sequencing/#L41">Source</a></span></div></li><li id="m-OptionSequencing-bind"><div class="doc-method-row"><code><strong class="tok-variable">bind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> (<span class="tok-variable">ef</span> :: <span class="tok-constructor">Row</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-OptionSequencing" title="Effect.Sequencing.OptionSequencing">OptionSequencing</a>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{|ef} <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span>{|ef} <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#m-OptionSequencing-bind" title="Permalink">#</a><a href="/library/source/Effect.Sequencing/#L42">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-OptionSequencing" title="Effect.Sequencing.OptionSequencing">OptionSequencing</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Option/#instances">Data.Option</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Ord">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Ord</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Ord" title="Permalink">#</a><a href="/library/source/Relation.Ord/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Ord-op-3c"><div class="doc-method-row"><code><strong class="tok-variable">&lt;</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#m-Ord-op-3c" title="Permalink">#</a><a href="/library/source/Relation.Ord/#L26">Source</a></span></div></li><li id="m-Ord-op-3c3d"><div class="doc-method-row"><code><strong class="tok-variable">&lt;=</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#m-Ord-op-3c3d" title="Permalink">#</a><a href="/library/source/Relation.Ord/#L27">Source</a></span></div></li><li id="m-Ord-op-3e"><div class="doc-method-row"><code><strong class="tok-variable">&gt;</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#m-Ord-op-3e" title="Permalink">#</a><a href="/library/source/Relation.Ord/#L28">Source</a></span></div></li><li id="m-Ord-op-3e3d"><div class="doc-method-row"><code><strong class="tok-variable">&gt;=</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#m-Ord-op-3e3d" title="Permalink">#</a><a href="/library/source/Relation.Ord/#L29">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (35)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.BigDecimal/#t-BigDecimal" title="Data.BigDecimal.BigDecimal">BigDecimal</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.BigDecimal/#instances">Data.BigDecimal</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Bool/#instances">Data.Bool</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Char/" title="Char">Char</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Char/#instances">Data.Char</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Date/#t-Date" title="Data.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Date/#instances">Data.Date</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-IpAddr" title="Data.IpAddr.IpAddr">IpAddr</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.IpAddr/#instances">Data.IpAddr</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-Ipv4" title="Data.IpAddr.Ipv4">Ipv4</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.IpAddr/#instances">Data.IpAddr</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.IpAddr/#t-Ipv6" title="Data.IpAddr.Ipv6">Ipv6</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.IpAddr/#instances">Data.IpAddr</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I16">I16</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I32">I32</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I8">I8</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="Isize">Isize</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U16">U16</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="Usize">Usize</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Machine/#instances">Data.Machine</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Nat/#instances">Data.Nat</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Rational/#t-Rational" title="Data.Rational.Rational">Rational</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Rational/#instances">Data.Rational</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Sym/" title="Sym">Sym</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Sym/#instances">Data.Sym</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time/#t-Time" title="Data.Time.Time">Time</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time/#instances">Data.Time</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Date/#t-Date" title="Data.Time.Date.Date">Date</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Date/#instances">Data.Time.Date</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.DateTime/#t-DateTime" title="Data.Time.DateTime.DateTime">DateTime</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.DateTime/#instances">Data.Time.DateTime</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Duration/#t-Duration" title="Data.Time.Duration.Duration">Duration</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Duration/#instances">Data.Time.Duration</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Monotonic/#t-Monotonic" title="Data.Time.Monotonic.Monotonic">Monotonic</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Monotonic/#instances">Data.Time.Monotonic</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Offset/#t-Offset" title="Data.Time.Offset.Offset">Offset</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Offset/#instances">Data.Time.Offset</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.TimeOfDay/#t-TimeOfDay" title="Data.Time.TimeOfDay.TimeOfDay">TimeOfDay</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.TimeOfDay/#instances">Data.Time.TimeOfDay</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Timestamp/#t-Timestamp" title="Data.Time.Timestamp.Timestamp">Timestamp</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Timestamp/#instances">Data.Time.Timestamp</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Time.Weekday/#t-Weekday" title="Data.Time.Weekday.Weekday">Weekday</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Time.Weekday/#instances">Data.Time.Weekday</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Uuid/#t-Uuid" title="Data.Uuid.Uuid">Uuid</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Uuid/#instances">Data.Uuid</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="I128">I128</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Wide/#instances">Data.Wide</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/library/Data.Wide/" title="U128">U128</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Wide/#instances">Data.Wide</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> (<a class="doc-type-link tok-constructor" href="/library/Relation.Ord/#t-Down" title="Relation.Ord.Down">Down</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/library/Relation.Ord/#instances">Relation.Ord</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Rem">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Rem</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Rem" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Modulo and division for the mathematical types (decision D91).
The divisor carries its evidence: a `NonZero a`, so no instance
can reach a zero divisor. A numeral divisor needs nothing written
(`n / 10` and `n % 10` decide `NonZero 10` at compile time); any
other divisor is a threaded `NonZero a` or the product of a
runtime decision (`decide (NonZero d)`). A total division whose
divisor may be zero is not this operation: `Data.Complex.div`'s
IEEE quotient stays a named function. Machine integers keep the
checked family only, per the primitive types specification.

<div class="doc-children"><h4>Methods</h4><ul><li id="m-Rem-op-25"><div class="doc-method-row"><code><strong class="tok-variable">%</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Rem" title="Algebra.Arithmetic.Rem">Rem</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#t-NonZero" title="Algebra.Arithmetic.NonZero">NonZero</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Rem-op-25" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L66">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Rem" title="Algebra.Arithmetic.Rem">Rem</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Rem" title="Algebra.Arithmetic.Rem">Rem</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-ResultSequencing">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">ResultSequencing</strong> <span class="tok-variable">e</span></code><span class="doc-sig-links"><a href="#c-ResultSequencing" title="Permalink">#</a><a href="/library/source/Effect.Sequencing/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-ResultSequencing-pure"><div class="doc-method-row"><code><strong class="tok-variable">pure</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-ResultSequencing" title="Effect.Sequencing.ResultSequencing">ResultSequencing</a> <span class="tok-variable">e</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-ResultSequencing-pure" title="Permalink">#</a><a href="/library/source/Effect.Sequencing/#L37">Source</a></span></div></li><li id="m-ResultSequencing-bind"><div class="doc-method-row"><code><strong class="tok-variable">bind</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-variable">b</span> (<span class="tok-variable">ef</span> :: <span class="tok-constructor">Row</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-ResultSequencing" title="Effect.Sequencing.ResultSequencing">ResultSequencing</a> <span class="tok-variable">e</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span>{|ef} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">b</span>) <span class="tok-operator">-&gt;</span>{|ef} <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <span class="tok-variable">e</span> <span class="tok-variable">b</span></code><span class="doc-sig-links"><a href="#m-ResultSequencing-bind" title="Permalink">#</a><a href="/library/source/Effect.Sequencing/#L38">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Effect.Sequencing/#c-ResultSequencing" title="Effect.Sequencing.ResultSequencing">ResultSequencing</a> <span class="tok-variable">e</span></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Result/#instances">Data.Result</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-Sub">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Sub</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-Sub" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Methods</h4><ul><li id="m-Sub--"><div class="doc-method-row"><code><strong class="tok-variable">-</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#m-Sub--" title="Permalink">#</a><a href="/library/source/Algebra.Arithmetic/#L48">Source</a></span></div></li></ul></div>
<details class="doc-instances"><summary>Instances (3)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/library/Data.Complex/#t-Complex" title="Data.Complex.Complex">Complex</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Complex/#instances">Data.Complex</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Int/#instances">Data.Int</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Arithmetic/#c-Sub" title="Algebra.Arithmetic.Sub">Sub</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Nat/#instances">Data.Nat</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="c-SumProofs">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">SumProofs</strong></code><span class="doc-sig-links"><a href="#c-SumProofs" title="Permalink">#</a><a href="/library/source/Data.Target/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

The instruction law (platform section 5): the scalar model and the
SIMD intrinsic compute the same sum. Discharging it is a proof
obligation of the target library; a rewrite citing it records the
assumption in the artifact manifest (decision D55).

<div class="doc-children"><h4>Methods</h4><ul><li id="m-SumProofs-scalar-to-simd"><div class="doc-method-row"><code><strong class="tok-variable">scalar-to-simd</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span> (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>) <span class="tok-variable">region</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Target/#c-SumProofs" title="Data.Target.SumProofs">SumProofs</a>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">vectors</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">xs</span> :: <a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Ref">Ref</a> <span class="tok-variable">region</span> <a class="doc-type-link tok-constructor" href="/library/Data.Mutable/#t-Vec" title="Vec">Vec</a> <span class="tok-variable">n</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Relation.Eq/#c-Eq" title="Eq">Eq</a> (<span class="tok-constructor">scalar-sum-wrap32</span> @ <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> @ <span class="tok-variable">region</span> @ 1 (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <span class="tok-variable">n</span>) @ 4 <span class="tok-variable">region</span> <span class="tok-variable">xs</span>) (<span class="tok-constructor">simd-sum-wrap32</span> @ <span class="tok-variable">t</span> @ <span class="tok-variable">w</span> @ <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> @ <span class="tok-variable">region</span> @ 1 (<a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a>, <span class="tok-variable">n</span>) @ 4 <span class="tok-variable">region</span> <span class="tok-variable">vectors</span> <span class="tok-variable">xs</span>)</code><span class="doc-sig-links"><a href="#m-SumProofs-scalar-to-simd" title="Permalink">#</a><a href="/library/source/Data.Target/#L90">Source</a></span></div></li></ul></div>
</div>
</article>

<article class="doc-entry" id="c-VecU32Target">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">VecU32Target</strong> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code><span class="doc-sig-links"><a href="#c-VecU32Target" title="Permalink">#</a><a href="/library/source/Data.Target/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

The u32-sum capability: its construction establishes that the sum
intrinsics may execute in the current environment (platform proofs
and SIMD specification section 2). The method is the width evidence
a generic proof consumes (section 6).

<div class="doc-children"><h4>Methods</h4><ul><li id="m-VecU32Target-positive-width"><div class="doc-method-row"><code><strong class="tok-variable">positive-width</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#m-VecU32Target-positive-width" title="Permalink">#</a><a href="/library/source/Data.Target/#L64">Source</a></span></div></li></ul></div>
</div>
</article>

# Re-exported values

Prelude re-exports a curated subset of the standard library. Follow the defining module link for documentation, or use [API Search](/library/search/) to query by type signature. Compiler-installed forms such as `ioWrite` are documented in [compiler vocabulary](../reference/20-compiler-vocabulary.md).

| Value | Signature | Defining module |
| --- | --- | --- |
| `&` | `forall a. using (bits :: Bits a) => a -> a -> a` | [Algebra.Bits](Algebra.Bits.md#v-op-26) |
| `&&` | `Bool -> Bool -> Bool` | [Data.Bool](Data.Bool.md#v-op-2626) |
| `<<` | `forall a. using (bits :: Bits a) => a -> U32 -> a` | [Algebra.Bits](Algebra.Bits.md#v-op-3c3c) |
| `>>` | `forall a. using (bits :: Bits a) => a -> U32 -> a` | [Algebra.Bits](Algebra.Bits.md#v-op-3e3e) |
| `^` | `forall a. using (bits :: Bits a) => a -> a -> a` | [Algebra.Bits](Algebra.Bits.md#v-op-5e) |
| `abs-f32` | `F32 -> F32` | [Data.Machine](Data.Machine.md#v-abs-f32) |
| `abs-f64` | `F64 -> F64` | [Data.Machine](Data.Machine.md#v-abs-f64) |
| `add-f32` | `F32 -> F32 -> F32` | [Data.Machine](Data.Machine.md#v-add-f32) |
| `add-f64` | `F64 -> F64 -> F64` | [Data.Machine](Data.Machine.md#v-add-f64) |
| `and` | `Bool -> Bool -> Bool` | [Data.Bool](Data.Bool.md#v-and) |
| `apply-n-times` | `forall a. Nat -> (a -> a) -> a -> a` | [Function.Combinators](Function.Combinators.md#v-apply-n-times) |
| `apply-to` | `forall a b. a -> (a -> b) -> b` | [Function.Combinators](Function.Combinators.md#v-apply-to) |
| `bit-and-i16` | `I16 -> I16 -> I16` | [Data.Machine](Data.Machine.md#v-bit-and-i16) |
| `bit-and-i32` | `I32 -> I32 -> I32` | [Data.Machine](Data.Machine.md#v-bit-and-i32) |
| `bit-and-i64` | `I64 -> I64 -> I64` | [Data.Machine](Data.Machine.md#v-bit-and-i64) |
| `bit-and-i8` | `I8 -> I8 -> I8` | [Data.Machine](Data.Machine.md#v-bit-and-i8) |
| `bit-and-isize` | `Isize -> Isize -> Isize` | [Data.Machine](Data.Machine.md#v-bit-and-isize) |
| `bit-and-u16` | `U16 -> U16 -> U16` | [Data.Machine](Data.Machine.md#v-bit-and-u16) |
| `bit-and-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-bit-and-u32) |
| `bit-and-u64` | `U64 -> U64 -> U64` | [Data.Machine](Data.Machine.md#v-bit-and-u64) |
| `bit-and-u8` | `U8 -> U8 -> U8` | [Data.Machine](Data.Machine.md#v-bit-and-u8) |
| `bit-and-usize` | `Usize -> Usize -> Usize` | [Data.Machine](Data.Machine.md#v-bit-and-usize) |
| `bit-not-i16` | `I16 -> I16` | [Data.Machine](Data.Machine.md#v-bit-not-i16) |
| `bit-not-i32` | `I32 -> I32` | [Data.Machine](Data.Machine.md#v-bit-not-i32) |
| `bit-not-i64` | `I64 -> I64` | [Data.Machine](Data.Machine.md#v-bit-not-i64) |
| `bit-not-i8` | `I8 -> I8` | [Data.Machine](Data.Machine.md#v-bit-not-i8) |
| `bit-not-isize` | `Isize -> Isize` | [Data.Machine](Data.Machine.md#v-bit-not-isize) |
| `bit-not-u16` | `U16 -> U16` | [Data.Machine](Data.Machine.md#v-bit-not-u16) |
| `bit-not-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-bit-not-u32) |
| `bit-not-u64` | `U64 -> U64` | [Data.Machine](Data.Machine.md#v-bit-not-u64) |
| `bit-not-u8` | `U8 -> U8` | [Data.Machine](Data.Machine.md#v-bit-not-u8) |
| `bit-not-usize` | `Usize -> Usize` | [Data.Machine](Data.Machine.md#v-bit-not-usize) |
| `bit-or-i16` | `I16 -> I16 -> I16` | [Data.Machine](Data.Machine.md#v-bit-or-i16) |
| `bit-or-i32` | `I32 -> I32 -> I32` | [Data.Machine](Data.Machine.md#v-bit-or-i32) |
| `bit-or-i64` | `I64 -> I64 -> I64` | [Data.Machine](Data.Machine.md#v-bit-or-i64) |
| `bit-or-i8` | `I8 -> I8 -> I8` | [Data.Machine](Data.Machine.md#v-bit-or-i8) |
| `bit-or-isize` | `Isize -> Isize -> Isize` | [Data.Machine](Data.Machine.md#v-bit-or-isize) |
| `bit-or-u16` | `U16 -> U16 -> U16` | [Data.Machine](Data.Machine.md#v-bit-or-u16) |
| `bit-or-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-bit-or-u32) |
| `bit-or-u64` | `U64 -> U64 -> U64` | [Data.Machine](Data.Machine.md#v-bit-or-u64) |
| `bit-or-u8` | `U8 -> U8 -> U8` | [Data.Machine](Data.Machine.md#v-bit-or-u8) |
| `bit-or-usize` | `Usize -> Usize -> Usize` | [Data.Machine](Data.Machine.md#v-bit-or-usize) |
| `bit-xor-i16` | `I16 -> I16 -> I16` | [Data.Machine](Data.Machine.md#v-bit-xor-i16) |
| `bit-xor-i32` | `I32 -> I32 -> I32` | [Data.Machine](Data.Machine.md#v-bit-xor-i32) |
| `bit-xor-i64` | `I64 -> I64 -> I64` | [Data.Machine](Data.Machine.md#v-bit-xor-i64) |
| `bit-xor-i8` | `I8 -> I8 -> I8` | [Data.Machine](Data.Machine.md#v-bit-xor-i8) |
| `bit-xor-isize` | `Isize -> Isize -> Isize` | [Data.Machine](Data.Machine.md#v-bit-xor-isize) |
| `bit-xor-u16` | `U16 -> U16 -> U16` | [Data.Machine](Data.Machine.md#v-bit-xor-u16) |
| `bit-xor-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-bit-xor-u32) |
| `bit-xor-u64` | `U64 -> U64 -> U64` | [Data.Machine](Data.Machine.md#v-bit-xor-u64) |
| `bit-xor-u8` | `U8 -> U8 -> U8` | [Data.Machine](Data.Machine.md#v-bit-xor-u8) |
| `bit-xor-usize` | `Usize -> Usize -> Usize` | [Data.Machine](Data.Machine.md#v-bit-xor-usize) |
| `bool` | `forall a. a -> a -> Bool -> a` | [Data.Bool](Data.Bool.md#v-bool) |
| `both` | `forall a b c. (a -> b) -> (a -> c) -> a -> (b, c)` | [Function.Combinators](Function.Combinators.md#v-both) |
| `break` | `forall a. (a -> Bool) -> List a -> (List a, List a)` | [Data.List](Data.List.md#v-break) |
| `bytes-compare` | `Bytes -> Bytes -> I64` | [Data.Bytes](Data.Bytes.md#v-bytes-compare) |
| `bytes-copy-range` | `(arg2 + arg4 <= (bytes-len arg0), arg3 + arg4 <= (bytes-len arg1)) => (arg0 :: Bytes) -> (arg1 :: Bytes) -> (arg2 :: Nat) -> (arg3 :: Nat) -> (arg4 :: Nat) -> Unit` | [Data.Bytes](Data.Bytes.md#v-bytes-copy-range) |
| `bytes-data` | `Bytes -> I64` | [Data.Bytes](Data.Bytes.md#v-bytes-data) |
| `bytes-from-list` | `List (InRange 0 255 Nat) -> Bytes` | [Data.Bytes](Data.Bytes.md#v-bytes-from-list) |
| `bytes-from-u8s` | `List U8 -> Bytes` | [Data.Bytes](Data.Bytes.md#v-bytes-from-u8s) |
| `bytes-get` | `arg1 < (bytes-len arg0) => (arg0 :: Bytes) -> (arg1 :: Nat) -> InRange 0 255 Nat` | [Data.Bytes](Data.Bytes.md#v-bytes-get) |
| `bytes-len` | `Bytes -> InRange 0 9223372036854775807 Nat` | [Data.Bytes](Data.Bytes.md#v-bytes-len) |
| `bytes-new` | `((n :: Nat) -> (refined ((b.bytesLen cinch-result) == n) Bytes))` | [Data.Bytes](Data.Bytes.md#v-bytes-new) |
| `bytes-set` | `arg1 < (bytes-len arg0) => (arg0 :: Bytes) -> (arg1 :: Nat) -> (arg2 :: InRange 0 255 Nat) -> Unit` | [Data.Bytes](Data.Bytes.md#v-bytes-set) |
| `cat-options` | `forall a. List (Option a) -> List a` | [Data.Option](Data.Option.md#v-cat-options) |
| `char-code` | `Char -> InRange 0 1114111 Nat` | [Data.Char](Data.Char.md#v-char-code) |
| `char-from-code` | `Nat -> Option Char` | [Data.Char](Data.Char.md#v-char-from-code) |
| `char-string` | `Char -> String` | [Text.Show](Text.Show.md#v-char-string) |
| `char-to-u32` | `(c :: Char) -> refined (0 <= cinch-result && cinch-result <= 1114111 && cinch-result == c) U32` | [Data.Char](Data.Char.md#v-char-to-u32) |
| `checked-add-i16` | `I16 -> I16 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-add-i16) |
| `checked-add-i32` | `I32 -> I32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-add-i32) |
| `checked-add-i64` | `I64 -> I64 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-add-i64) |
| `checked-add-i8` | `I8 -> I8 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-add-i8) |
| `checked-add-isize` | `Isize -> Isize -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-add-isize) |
| `checked-add-u16` | `U16 -> U16 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-add-u16) |
| `checked-add-u32` | `U32 -> U32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-add-u32) |
| `checked-add-u64` | `U64 -> U64 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-add-u64) |
| `checked-add-u8` | `U8 -> U8 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-add-u8) |
| `checked-add-usize` | `Usize -> Usize -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-add-usize) |
| `checked-div` | `Nat -> Nat -> Option Nat` | [Data.Nat](Data.Nat.md#v-checked-div) |
| `checked-div-i16` | `I16 -> I16 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-div-i16) |
| `checked-div-i32` | `I32 -> I32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-div-i32) |
| `checked-div-i64` | `I64 -> I64 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-div-i64) |
| `checked-div-i8` | `I8 -> I8 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-div-i8) |
| `checked-div-isize` | `Isize -> Isize -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-div-isize) |
| `checked-div-u16` | `U16 -> U16 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-div-u16) |
| `checked-div-u32` | `U32 -> U32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-div-u32) |
| `checked-div-u64` | `U64 -> U64 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-div-u64) |
| `checked-div-u8` | `U8 -> U8 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-div-u8) |
| `checked-div-usize` | `Usize -> Usize -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-div-usize) |
| `checked-f32-to-i16` | `F32 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-f32-to-i16) |
| `checked-f32-to-i32` | `F32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-f32-to-i32) |
| `checked-f32-to-i64` | `F32 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-f32-to-i64) |
| `checked-f32-to-i8` | `F32 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-f32-to-i8) |
| `checked-f32-to-isize` | `F32 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-f32-to-isize) |
| `checked-f32-to-u16` | `F32 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-f32-to-u16) |
| `checked-f32-to-u32` | `F32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-f32-to-u32) |
| `checked-f32-to-u64` | `F32 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-f32-to-u64) |
| `checked-f32-to-u8` | `F32 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-f32-to-u8) |
| `checked-f32-to-usize` | `F32 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-f32-to-usize) |
| `checked-f64-to-i16` | `F64 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-f64-to-i16) |
| `checked-f64-to-i32` | `F64 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-f64-to-i32) |
| `checked-f64-to-i64` | `F64 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-f64-to-i64) |
| `checked-f64-to-i8` | `F64 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-f64-to-i8) |
| `checked-f64-to-isize` | `F64 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-f64-to-isize) |
| `checked-f64-to-u16` | `F64 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-f64-to-u16) |
| `checked-f64-to-u32` | `F64 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-f64-to-u32) |
| `checked-f64-to-u64` | `F64 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-f64-to-u64) |
| `checked-f64-to-u8` | `F64 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-f64-to-u8) |
| `checked-f64-to-usize` | `F64 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-f64-to-usize) |
| `checked-i16-to-i16` | `I16 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-i16-to-i16) |
| `checked-i16-to-i32` | `I16 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-i16-to-i32) |
| `checked-i16-to-i64` | `I16 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-i16-to-i64) |
| `checked-i16-to-i8` | `I16 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-i16-to-i8) |
| `checked-i16-to-isize` | `I16 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-i16-to-isize) |
| `checked-i16-to-u16` | `I16 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-i16-to-u16) |
| `checked-i16-to-u32` | `I16 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-i16-to-u32) |
| `checked-i16-to-u64` | `I16 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-i16-to-u64) |
| `checked-i16-to-u8` | `I16 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-i16-to-u8) |
| `checked-i16-to-usize` | `I16 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-i16-to-usize) |
| `checked-i32-to-i16` | `I32 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-i32-to-i16) |
| `checked-i32-to-i32` | `I32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-i32-to-i32) |
| `checked-i32-to-i64` | `I32 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-i32-to-i64) |
| `checked-i32-to-i8` | `I32 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-i32-to-i8) |
| `checked-i32-to-isize` | `I32 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-i32-to-isize) |
| `checked-i32-to-u16` | `I32 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-i32-to-u16) |
| `checked-i32-to-u32` | `I32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-i32-to-u32) |
| `checked-i32-to-u64` | `I32 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-i32-to-u64) |
| `checked-i32-to-u8` | `I32 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-i32-to-u8) |
| `checked-i32-to-usize` | `I32 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-i32-to-usize) |
| `checked-i64-to-i16` | `I64 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-i64-to-i16) |
| `checked-i64-to-i32` | `I64 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-i64-to-i32) |
| `checked-i64-to-i64` | `I64 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-i64-to-i64) |
| `checked-i64-to-i8` | `I64 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-i64-to-i8) |
| `checked-i64-to-isize` | `I64 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-i64-to-isize) |
| `checked-i64-to-u16` | `I64 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-i64-to-u16) |
| `checked-i64-to-u32` | `I64 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-i64-to-u32) |
| `checked-i64-to-u64` | `I64 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-i64-to-u64) |
| `checked-i64-to-u8` | `I64 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-i64-to-u8) |
| `checked-i64-to-usize` | `I64 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-i64-to-usize) |
| `checked-i8-to-i16` | `I8 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-i8-to-i16) |
| `checked-i8-to-i32` | `I8 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-i8-to-i32) |
| `checked-i8-to-i64` | `I8 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-i8-to-i64) |
| `checked-i8-to-i8` | `I8 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-i8-to-i8) |
| `checked-i8-to-isize` | `I8 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-i8-to-isize) |
| `checked-i8-to-u16` | `I8 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-i8-to-u16) |
| `checked-i8-to-u32` | `I8 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-i8-to-u32) |
| `checked-i8-to-u64` | `I8 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-i8-to-u64) |
| `checked-i8-to-u8` | `I8 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-i8-to-u8) |
| `checked-i8-to-usize` | `I8 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-i8-to-usize) |
| `checked-int-to-i64` | `Int -> Option I64` | [Data.Int](Data.Int.md#v-checked-int-to-i64) |
| `checked-isize-to-i16` | `Isize -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-isize-to-i16) |
| `checked-isize-to-i32` | `Isize -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-isize-to-i32) |
| `checked-isize-to-i64` | `Isize -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-isize-to-i64) |
| `checked-isize-to-i8` | `Isize -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-isize-to-i8) |
| `checked-isize-to-isize` | `Isize -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-isize-to-isize) |
| `checked-isize-to-u16` | `Isize -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-isize-to-u16) |
| `checked-isize-to-u32` | `Isize -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-isize-to-u32) |
| `checked-isize-to-u64` | `Isize -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-isize-to-u64) |
| `checked-isize-to-u8` | `Isize -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-isize-to-u8) |
| `checked-isize-to-usize` | `Isize -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-isize-to-usize) |
| `checked-mul-i16` | `I16 -> I16 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-mul-i16) |
| `checked-mul-i32` | `I32 -> I32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-mul-i32) |
| `checked-mul-i64` | `I64 -> I64 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-mul-i64) |
| `checked-mul-i8` | `I8 -> I8 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-mul-i8) |
| `checked-mul-isize` | `Isize -> Isize -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-mul-isize) |
| `checked-mul-u16` | `U16 -> U16 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-mul-u16) |
| `checked-mul-u32` | `U32 -> U32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-mul-u32) |
| `checked-mul-u64` | `U64 -> U64 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-mul-u64) |
| `checked-mul-u8` | `U8 -> U8 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-mul-u8) |
| `checked-mul-usize` | `Usize -> Usize -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-mul-usize) |
| `checked-nat-from-i64` | `I64 -> Option Nat` | [Data.Nat](Data.Nat.md#v-checked-nat-from-i64) |
| `checked-nat-to-i64` | `Nat -> Option I64` | [Data.Nat](Data.Nat.md#v-checked-nat-to-i64) |
| `checked-neg-i16` | `I16 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-neg-i16) |
| `checked-neg-i32` | `I32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-neg-i32) |
| `checked-neg-i64` | `I64 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-neg-i64) |
| `checked-neg-i8` | `I8 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-neg-i8) |
| `checked-neg-isize` | `Isize -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-neg-isize) |
| `checked-neg-u16` | `U16 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-neg-u16) |
| `checked-neg-u32` | `U32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-neg-u32) |
| `checked-neg-u64` | `U64 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-neg-u64) |
| `checked-neg-u8` | `U8 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-neg-u8) |
| `checked-neg-usize` | `Usize -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-neg-usize) |
| `checked-rem` | `Nat -> Nat -> Option Nat` | [Data.Nat](Data.Nat.md#v-checked-rem) |
| `checked-rem-i16` | `I16 -> I16 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-rem-i16) |
| `checked-rem-i32` | `I32 -> I32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-rem-i32) |
| `checked-rem-i64` | `I64 -> I64 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-rem-i64) |
| `checked-rem-i8` | `I8 -> I8 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-rem-i8) |
| `checked-rem-isize` | `Isize -> Isize -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-rem-isize) |
| `checked-rem-u16` | `U16 -> U16 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-rem-u16) |
| `checked-rem-u32` | `U32 -> U32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-rem-u32) |
| `checked-rem-u64` | `U64 -> U64 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-rem-u64) |
| `checked-rem-u8` | `U8 -> U8 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-rem-u8) |
| `checked-rem-usize` | `Usize -> Usize -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-rem-usize) |
| `checked-shl-i16` | `I16 -> U32 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-shl-i16) |
| `checked-shl-i32` | `I32 -> U32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-shl-i32) |
| `checked-shl-i64` | `I64 -> U32 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-shl-i64) |
| `checked-shl-i8` | `I8 -> U32 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-shl-i8) |
| `checked-shl-isize` | `Isize -> U32 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-shl-isize) |
| `checked-shl-u16` | `U16 -> U32 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-shl-u16) |
| `checked-shl-u32` | `U32 -> U32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-shl-u32) |
| `checked-shl-u64` | `U64 -> U32 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-shl-u64) |
| `checked-shl-u8` | `U8 -> U32 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-shl-u8) |
| `checked-shl-usize` | `Usize -> U32 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-shl-usize) |
| `checked-shr-i16` | `I16 -> U32 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-shr-i16) |
| `checked-shr-i32` | `I32 -> U32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-shr-i32) |
| `checked-shr-i64` | `I64 -> U32 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-shr-i64) |
| `checked-shr-i8` | `I8 -> U32 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-shr-i8) |
| `checked-shr-isize` | `Isize -> U32 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-shr-isize) |
| `checked-shr-u16` | `U16 -> U32 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-shr-u16) |
| `checked-shr-u32` | `U32 -> U32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-shr-u32) |
| `checked-shr-u64` | `U64 -> U32 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-shr-u64) |
| `checked-shr-u8` | `U8 -> U32 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-shr-u8) |
| `checked-shr-usize` | `Usize -> U32 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-shr-usize) |
| `checked-sub` | `Nat -> Nat -> Option Nat` | [Data.Nat](Data.Nat.md#v-checked-sub) |
| `checked-sub-i16` | `I16 -> I16 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-sub-i16) |
| `checked-sub-i32` | `I32 -> I32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-sub-i32) |
| `checked-sub-i64` | `I64 -> I64 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-sub-i64) |
| `checked-sub-i8` | `I8 -> I8 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-sub-i8) |
| `checked-sub-isize` | `Isize -> Isize -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-sub-isize) |
| `checked-sub-u16` | `U16 -> U16 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-sub-u16) |
| `checked-sub-u32` | `U32 -> U32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-sub-u32) |
| `checked-sub-u64` | `U64 -> U64 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-sub-u64) |
| `checked-sub-u8` | `U8 -> U8 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-sub-u8) |
| `checked-sub-usize` | `Usize -> Usize -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-sub-usize) |
| `checked-u16-to-i16` | `U16 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-u16-to-i16) |
| `checked-u16-to-i32` | `U16 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-u16-to-i32) |
| `checked-u16-to-i64` | `U16 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-u16-to-i64) |
| `checked-u16-to-i8` | `U16 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-u16-to-i8) |
| `checked-u16-to-isize` | `U16 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-u16-to-isize) |
| `checked-u16-to-u16` | `U16 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-u16-to-u16) |
| `checked-u16-to-u32` | `U16 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-u16-to-u32) |
| `checked-u16-to-u64` | `U16 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-u16-to-u64) |
| `checked-u16-to-u8` | `U16 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-u16-to-u8) |
| `checked-u16-to-usize` | `U16 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-u16-to-usize) |
| `checked-u32-to-i16` | `U32 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-u32-to-i16) |
| `checked-u32-to-i32` | `U32 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-u32-to-i32) |
| `checked-u32-to-i64` | `U32 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-u32-to-i64) |
| `checked-u32-to-i8` | `U32 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-u32-to-i8) |
| `checked-u32-to-isize` | `U32 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-u32-to-isize) |
| `checked-u32-to-u16` | `U32 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-u32-to-u16) |
| `checked-u32-to-u32` | `U32 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-u32-to-u32) |
| `checked-u32-to-u64` | `U32 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-u32-to-u64) |
| `checked-u32-to-u8` | `U32 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-u32-to-u8) |
| `checked-u32-to-usize` | `U32 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-u32-to-usize) |
| `checked-u64-to-i16` | `U64 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-u64-to-i16) |
| `checked-u64-to-i32` | `U64 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-u64-to-i32) |
| `checked-u64-to-i64` | `U64 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-u64-to-i64) |
| `checked-u64-to-i8` | `U64 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-u64-to-i8) |
| `checked-u64-to-isize` | `U64 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-u64-to-isize) |
| `checked-u64-to-u16` | `U64 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-u64-to-u16) |
| `checked-u64-to-u32` | `U64 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-u64-to-u32) |
| `checked-u64-to-u64` | `U64 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-u64-to-u64) |
| `checked-u64-to-u8` | `U64 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-u64-to-u8) |
| `checked-u64-to-usize` | `U64 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-u64-to-usize) |
| `checked-u8-to-i16` | `U8 -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-u8-to-i16) |
| `checked-u8-to-i32` | `U8 -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-u8-to-i32) |
| `checked-u8-to-i64` | `U8 -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-u8-to-i64) |
| `checked-u8-to-i8` | `U8 -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-u8-to-i8) |
| `checked-u8-to-isize` | `U8 -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-u8-to-isize) |
| `checked-u8-to-u16` | `U8 -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-u8-to-u16) |
| `checked-u8-to-u32` | `U8 -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-u8-to-u32) |
| `checked-u8-to-u64` | `U8 -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-u8-to-u64) |
| `checked-u8-to-u8` | `U8 -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-u8-to-u8) |
| `checked-u8-to-usize` | `U8 -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-u8-to-usize) |
| `checked-usize-to-i16` | `Usize -> Option I16` | [Data.Machine](Data.Machine.md#v-checked-usize-to-i16) |
| `checked-usize-to-i32` | `Usize -> Option I32` | [Data.Machine](Data.Machine.md#v-checked-usize-to-i32) |
| `checked-usize-to-i64` | `Usize -> Option I64` | [Data.Machine](Data.Machine.md#v-checked-usize-to-i64) |
| `checked-usize-to-i8` | `Usize -> Option I8` | [Data.Machine](Data.Machine.md#v-checked-usize-to-i8) |
| `checked-usize-to-isize` | `Usize -> Option Isize` | [Data.Machine](Data.Machine.md#v-checked-usize-to-isize) |
| `checked-usize-to-u16` | `Usize -> Option U16` | [Data.Machine](Data.Machine.md#v-checked-usize-to-u16) |
| `checked-usize-to-u32` | `Usize -> Option U32` | [Data.Machine](Data.Machine.md#v-checked-usize-to-u32) |
| `checked-usize-to-u64` | `Usize -> Option U64` | [Data.Machine](Data.Machine.md#v-checked-usize-to-u64) |
| `checked-usize-to-u8` | `Usize -> Option U8` | [Data.Machine](Data.Machine.md#v-checked-usize-to-u8) |
| `checked-usize-to-usize` | `Usize -> Option Usize` | [Data.Machine](Data.Machine.md#v-checked-usize-to-usize) |
| `compare-bytes-from` | `Bytes -> Bytes -> Nat -> I64` | [Data.String](Data.String.md#v-compare-bytes-from) |
| `comparing` | `forall a b. using (ord :: Ord b) => (a -> b) -> a -> a -> Ordering` | [Relation.Ord](Relation.Ord.md#v-comparing) |
| `compose` | `forall a b c. (b -> c) -> (a -> b) -> a -> c` | [Function.Combinators](Function.Combinators.md#v-compose) |
| `const` | `forall a b. a -> b -> a` | [Function.Combinators](Function.Combinators.md#v-const) |
| `copy-bytes-into` | `(arg2 + arg4 <= (bytes-len arg0), arg3 + arg4 <= (bytes-len arg1)) => (arg0 :: Bytes) -> (arg1 :: Bytes) -> (arg2 :: Nat) -> (arg3 :: Nat) -> (arg4 :: Nat) -> Unit` | [Data.String](Data.String.md#v-copy-bytes-into) |
| `count-ones-i16` | `I16 -> U32` | [Data.Machine](Data.Machine.md#v-count-ones-i16) |
| `count-ones-i32` | `I32 -> U32` | [Data.Machine](Data.Machine.md#v-count-ones-i32) |
| `count-ones-i64` | `I64 -> U32` | [Data.Machine](Data.Machine.md#v-count-ones-i64) |
| `count-ones-i8` | `I8 -> U32` | [Data.Machine](Data.Machine.md#v-count-ones-i8) |
| `count-ones-isize` | `Isize -> U32` | [Data.Machine](Data.Machine.md#v-count-ones-isize) |
| `count-ones-u16` | `U16 -> U32` | [Data.Machine](Data.Machine.md#v-count-ones-u16) |
| `count-ones-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-count-ones-u32) |
| `count-ones-u64` | `U64 -> U32` | [Data.Machine](Data.Machine.md#v-count-ones-u64) |
| `count-ones-u8` | `U8 -> U32` | [Data.Machine](Data.Machine.md#v-count-ones-u8) |
| `count-ones-usize` | `Usize -> U32` | [Data.Machine](Data.Machine.md#v-count-ones-usize) |
| `curry` | `forall a b c. ((a, b) -> c) -> a -> b -> c` | [Function.Combinators](Function.Combinators.md#v-curry) |
| `cycle` | `forall a. Nat -> List a -> List a` | [Data.List](Data.List.md#v-cycle) |
| `detect-vec-u32` | `Unit -> Option (VecU32Target AVX2_256 EightLanes)` | [Data.Target](Data.Target.md#v-detect-vec-u32) |
| `digit-value` | `Char -> Option Nat` | [Data.Char](Data.Char.md#v-digit-value) |
| `div-f32` | `F32 -> F32 -> F32` | [Data.Machine](Data.Machine.md#v-div-f32) |
| `div-f64` | `F64 -> F64 -> F64` | [Data.Machine](Data.Machine.md#v-div-f64) |
| `drop` | `forall a. Nat -> List a -> List a` | [Data.List](Data.List.md#v-drop) |
| `either` | `forall e a b. (e -> b) -> (a -> b) -> Result e a -> b` | [Data.Result](Data.Result.md#v-either) |
| `elem-index` | `forall a. using (eq :: Eq a) => a -> List a -> Option Nat` | [Data.List](Data.List.md#v-elem-index) |
| `empty-string` | `String` | [Data.String](Data.String.md#v-empty-string) |
| `enum-from-then-to` | `forall a. using (e :: Enum a) => a -> a -> a -> List a` | [Data.Enum](Data.Enum.md#v-enum-from-then-to) |
| `enum-from-to` | `forall a. using (e :: Enum a) => a -> a -> List a` | [Data.Enum](Data.Enum.md#v-enum-from-to) |
| `eq-char` | `Char -> Char -> Bool` | [Data.Char](Data.Char.md#v-eq-char) |
| `eq-i16` | `I16 -> I16 -> Bool` | [Data.Machine](Data.Machine.md#v-eq-i16) |
| `eq-i32` | `I32 -> I32 -> Bool` | [Data.Machine](Data.Machine.md#v-eq-i32) |
| `eq-i64` | `I64 -> I64 -> Bool` | [Data.Machine](Data.Machine.md#v-eq-i64) |
| `eq-i8` | `I8 -> I8 -> Bool` | [Data.Machine](Data.Machine.md#v-eq-i8) |
| `eq-isize` | `Isize -> Isize -> Bool` | [Data.Machine](Data.Machine.md#v-eq-isize) |
| `eq-u16` | `U16 -> U16 -> Bool` | [Data.Machine](Data.Machine.md#v-eq-u16) |
| `eq-u32` | `U32 -> U32 -> Bool` | [Data.Machine](Data.Machine.md#v-eq-u32) |
| `eq-u64` | `U64 -> U64 -> Bool` | [Data.Machine](Data.Machine.md#v-eq-u64) |
| `eq-u8` | `U8 -> U8 -> Bool` | [Data.Machine](Data.Machine.md#v-eq-u8) |
| `eq-usize` | `Usize -> Usize -> Bool` | [Data.Machine](Data.Machine.md#v-eq-usize) |
| `errors` | `forall e a. List (Result e a) -> List e` | [Data.Result](Data.Result.md#v-errors) |
| `f32-to-f32` | `F32 -> F32` | [Data.Machine](Data.Machine.md#v-f32-to-f32) |
| `f32-to-f64` | `F32 -> F64` | [Data.Machine](Data.Machine.md#v-f32-to-f64) |
| `f64-from-decimal` | `Bytes -> F64` | [Data.Machine](Data.Machine.md#v-f64-from-decimal) |
| `f64-to-decimal` | `F64 -> Bytes` | [Data.Machine](Data.Machine.md#v-f64-to-decimal) |
| `f64-to-f32` | `F64 -> F32` | [Data.Machine](Data.Machine.md#v-f64-to-f32) |
| `f64-to-f64` | `F64 -> F64` | [Data.Machine](Data.Machine.md#v-f64-to-f64) |
| `feq-f32` | `F32 -> F32 -> Bool` | [Data.Machine](Data.Machine.md#v-feq-f32) |
| `feq-f64` | `F64 -> F64 -> Bool` | [Data.Machine](Data.Machine.md#v-feq-f64) |
| `fetch` | `forall (r :: Region). using (io :: Io r) => Request ->{Diverge, IO r} Result LoadError Bytes` | [Effect.Concurrent](Effect.Concurrent.md#v-fetch) |
| `fge-f32` | `F32 -> F32 -> Bool` | [Data.Machine](Data.Machine.md#v-fge-f32) |
| `fge-f64` | `F64 -> F64 -> Bool` | [Data.Machine](Data.Machine.md#v-fge-f64) |
| `fgt-f32` | `F32 -> F32 -> Bool` | [Data.Machine](Data.Machine.md#v-fgt-f32) |
| `fgt-f64` | `F64 -> F64 -> Bool` | [Data.Machine](Data.Machine.md#v-fgt-f64) |
| `find` | `forall a. (a -> Bool) -> List a -> Option a` | [Data.List](Data.List.md#v-find) |
| `find-index` | `forall a. (a -> Bool) -> List a -> Option Nat` | [Data.List](Data.List.md#v-find-index) |
| `fle-f32` | `F32 -> F32 -> Bool` | [Data.Machine](Data.Machine.md#v-fle-f32) |
| `fle-f64` | `F64 -> F64 -> Bool` | [Data.Machine](Data.Machine.md#v-fle-f64) |
| `flip` | `forall a b c. (a -> b -> c) -> b -> a -> c` | [Function.Combinators](Function.Combinators.md#v-flip) |
| `flt-f32` | `F32 -> F32 -> Bool` | [Data.Machine](Data.Machine.md#v-flt-f32) |
| `flt-f64` | `F64 -> F64 -> Bool` | [Data.Machine](Data.Machine.md#v-flt-f64) |
| `fne-f32` | `F32 -> F32 -> Bool` | [Data.Machine](Data.Machine.md#v-fne-f32) |
| `fne-f64` | `F64 -> F64 -> Bool` | [Data.Machine](Data.Machine.md#v-fne-f64) |
| `fold-left1` | `forall a. (a -> a -> a) -> List a -> Option a` | [Data.List](Data.List.md#v-fold-left1) |
| `fold-right1` | `forall a. (a -> a -> a) -> List a -> Option a` | [Data.List](Data.List.md#v-fold-right1) |
| `foldr` | `forall a b. (a -> b -> b) -> b -> List a -> b` | [Data.List](Data.List.md#v-foldr) |
| `from-option` | `forall a. a -> Option a -> a` | [Data.Option](Data.Option.md#v-from-option) |
| `from-result` | `forall e a. a -> Result e a -> a` | [Data.Result](Data.Result.md#v-from-result) |
| `fst` | `forall a b. (a, b) -> a` | [Function.Combinators](Function.Combinators.md#v-fst) |
| `ge-char` | `Char -> Char -> Bool` | [Data.Char](Data.Char.md#v-ge-char) |
| `ge-i16` | `I16 -> I16 -> Bool` | [Data.Machine](Data.Machine.md#v-ge-i16) |
| `ge-i32` | `I32 -> I32 -> Bool` | [Data.Machine](Data.Machine.md#v-ge-i32) |
| `ge-i64` | `I64 -> I64 -> Bool` | [Data.Machine](Data.Machine.md#v-ge-i64) |
| `ge-i8` | `I8 -> I8 -> Bool` | [Data.Machine](Data.Machine.md#v-ge-i8) |
| `ge-isize` | `Isize -> Isize -> Bool` | [Data.Machine](Data.Machine.md#v-ge-isize) |
| `ge-u16` | `U16 -> U16 -> Bool` | [Data.Machine](Data.Machine.md#v-ge-u16) |
| `ge-u32` | `U32 -> U32 -> Bool` | [Data.Machine](Data.Machine.md#v-ge-u32) |
| `ge-u64` | `U64 -> U64 -> Bool` | [Data.Machine](Data.Machine.md#v-ge-u64) |
| `ge-u8` | `U8 -> U8 -> Bool` | [Data.Machine](Data.Machine.md#v-ge-u8) |
| `ge-usize` | `Usize -> Usize -> Bool` | [Data.Machine](Data.Machine.md#v-ge-usize) |
| `group-by` | `forall a. (a -> a -> Bool) -> List a -> List (List a)` | [Data.List](Data.List.md#v-group-by) |
| `gt-char` | `Char -> Char -> Bool` | [Data.Char](Data.Char.md#v-gt-char) |
| `gt-i16` | `I16 -> I16 -> Bool` | [Data.Machine](Data.Machine.md#v-gt-i16) |
| `gt-i32` | `I32 -> I32 -> Bool` | [Data.Machine](Data.Machine.md#v-gt-i32) |
| `gt-i64` | `I64 -> I64 -> Bool` | [Data.Machine](Data.Machine.md#v-gt-i64) |
| `gt-i8` | `I8 -> I8 -> Bool` | [Data.Machine](Data.Machine.md#v-gt-i8) |
| `gt-isize` | `Isize -> Isize -> Bool` | [Data.Machine](Data.Machine.md#v-gt-isize) |
| `gt-u16` | `U16 -> U16 -> Bool` | [Data.Machine](Data.Machine.md#v-gt-u16) |
| `gt-u32` | `U32 -> U32 -> Bool` | [Data.Machine](Data.Machine.md#v-gt-u32) |
| `gt-u64` | `U64 -> U64 -> Bool` | [Data.Machine](Data.Machine.md#v-gt-u64) |
| `gt-u8` | `U8 -> U8 -> Bool` | [Data.Machine](Data.Machine.md#v-gt-u8) |
| `gt-usize` | `Usize -> Usize -> Bool` | [Data.Machine](Data.Machine.md#v-gt-usize) |
| `head` | `forall a. List a -> Option a` | [Data.List](Data.List.md#v-head) |
| `hex-digit-value` | `Char -> Option Nat` | [Data.Char](Data.Char.md#v-hex-digit-value) |
| `i16-to-f32` | `I16 -> F32` | [Data.Machine](Data.Machine.md#v-i16-to-f32) |
| `i16-to-f64` | `I16 -> F64` | [Data.Machine](Data.Machine.md#v-i16-to-f64) |
| `i16-to-i16` | `I16 -> I16` | [Data.Machine](Data.Machine.md#v-i16-to-i16) |
| `i16-to-i32` | `(x :: I16) -> refined (- 32768 <= cinch-result && cinch-result <= 32767 && cinch-result == x) I32` | [Data.Machine](Data.Machine.md#v-i16-to-i32) |
| `i16-to-i64` | `(x :: I16) -> refined (- 32768 <= cinch-result && cinch-result <= 32767 && cinch-result == x) I64` | [Data.Machine](Data.Machine.md#v-i16-to-i64) |
| `i16-to-isize` | `(a0 :: I16) -> refined (- 32768 <= r && r <= 32767 && r == a0) Isize` | [Data.Machine](Data.Machine.md#v-i16-to-isize) |
| `i32-to-f32` | `I32 -> F32` | [Data.Machine](Data.Machine.md#v-i32-to-f32) |
| `i32-to-f64` | `I32 -> F64` | [Data.Machine](Data.Machine.md#v-i32-to-f64) |
| `i32-to-i32` | `I32 -> I32` | [Data.Machine](Data.Machine.md#v-i32-to-i32) |
| `i32-to-i64` | `(x :: I32) -> refined (- 2147483648 <= cinch-result && cinch-result <= 2147483647 && cinch-result == x) I64` | [Data.Machine](Data.Machine.md#v-i32-to-i64) |
| `i32-to-isize` | `(a0 :: I32) -> refined (- 2147483648 <= r && r <= 2147483647 && r == a0) Isize` | [Data.Machine](Data.Machine.md#v-i32-to-isize) |
| `i64-le-byte` | `I64 -> U32 -> InRange 0 255 Nat` | [Data.Machine](Data.Machine.md#v-i64-le-byte) |
| `i64-to-f32` | `I64 -> F32` | [Data.Machine](Data.Machine.md#v-i64-to-f32) |
| `i64-to-f64` | `I64 -> F64` | [Data.Machine](Data.Machine.md#v-i64-to-f64) |
| `i64-to-i64` | `I64 -> I64` | [Data.Machine](Data.Machine.md#v-i64-to-i64) |
| `i64-to-isize` | `(a0 :: I64) -> refined (r == a0) Isize` | [Data.Machine](Data.Machine.md#v-i64-to-isize) |
| `i8-to-f32` | `I8 -> F32` | [Data.Machine](Data.Machine.md#v-i8-to-f32) |
| `i8-to-f64` | `I8 -> F64` | [Data.Machine](Data.Machine.md#v-i8-to-f64) |
| `i8-to-i16` | `(x :: I8) -> refined (- 128 <= cinch-result && cinch-result <= 127 && cinch-result == x) I16` | [Data.Machine](Data.Machine.md#v-i8-to-i16) |
| `i8-to-i32` | `(x :: I8) -> refined (- 128 <= cinch-result && cinch-result <= 127 && cinch-result == x) I32` | [Data.Machine](Data.Machine.md#v-i8-to-i32) |
| `i8-to-i64` | `(x :: I8) -> refined (- 128 <= cinch-result && cinch-result <= 127 && cinch-result == x) I64` | [Data.Machine](Data.Machine.md#v-i8-to-i64) |
| `i8-to-i8` | `I8 -> I8` | [Data.Machine](Data.Machine.md#v-i8-to-i8) |
| `i8-to-isize` | `(a0 :: I8) -> refined (- 128 <= r && r <= 127 && r == a0) Isize` | [Data.Machine](Data.Machine.md#v-i8-to-isize) |
| `identity` | `forall a. a -> a` | [Function.Combinators](Function.Combinators.md#v-identity) |
| `init` | `forall a. List a -> Option (List a)` | [Data.List](Data.List.md#v-init) |
| `int-add` | `Int -> Int -> Int` | [Data.Int](Data.Int.md#v-int-add) |
| `int-cmp` | `Int -> Int -> I64` | [Data.Int](Data.Int.md#v-int-cmp) |
| `int-div` | `Int -> NonZero Int -> Int` | [Data.Int](Data.Int.md#v-int-div) |
| `int-from-i64` | `(x :: I64) -> refined (cinch-result == x) Int` | [Data.Int](Data.Int.md#v-int-from-i64) |
| `int-mod-nat` | `Int -> NonZero Nat -> Nat` | [Data.Int](Data.Int.md#v-int-mod-nat) |
| `int-mul` | `Int -> Int -> Int` | [Data.Int](Data.Int.md#v-int-mul) |
| `int-neg` | `Int -> Int` | [Data.Int](Data.Int.md#v-int-neg) |
| `int-rem` | `Int -> NonZero Int -> Int` | [Data.Int](Data.Int.md#v-int-rem) |
| `int-sub` | `Int -> Int -> Int` | [Data.Int](Data.Int.md#v-int-sub) |
| `int-to-i64` | `(- 9223372036854775808 <= arg0, arg0 <= 9223372036854775807) => (arg0 :: Int) -> refined (r == arg0) I64` | [Data.Int](Data.Int.md#v-int-to-i64) |
| `int-to-string` | `Int -> String` | [Text.Show](Text.Show.md#v-int-to-string) |
| `intercalate` | `forall a. List a -> List (List a) -> List a` | [Data.List](Data.List.md#v-intercalate) |
| `intern` | `String -> Sym` | [Data.Sym](Data.Sym.md#v-intern) |
| `intersperse` | `forall a. a -> List a -> List a` | [Data.List](Data.List.md#v-intersperse) |
| `is-ascii-alpha` | `Char -> Bool` | [Data.Char](Data.Char.md#v-is-ascii-alpha) |
| `is-ascii-alpha-numeric` | `Char -> Bool` | [Data.Char](Data.Char.md#v-is-ascii-alpha-numeric) |
| `is-ascii-digit` | `Char -> Bool` | [Data.Char](Data.Char.md#v-is-ascii-digit) |
| `is-ascii-hex-digit` | `Char -> Bool` | [Data.Char](Data.Char.md#v-is-ascii-hex-digit) |
| `is-ascii-lower` | `Char -> Bool` | [Data.Char](Data.Char.md#v-is-ascii-lower) |
| `is-ascii-space` | `Char -> Bool` | [Data.Char](Data.Char.md#v-is-ascii-space) |
| `is-ascii-upper` | `Char -> Bool` | [Data.Char](Data.Char.md#v-is-ascii-upper) |
| `is-empty` | `forall a. List a -> Bool` | [Data.List](Data.List.md#v-is-empty) |
| `is-err` | `forall e a. Result e a -> Bool` | [Data.Result](Data.Result.md#v-is-err) |
| `is-finite-f32` | `F32 -> Bool` | [Data.Machine](Data.Machine.md#v-is-finite-f32) |
| `is-finite-f64` | `F64 -> Bool` | [Data.Machine](Data.Machine.md#v-is-finite-f64) |
| `is-infinite-f32` | `F32 -> Bool` | [Data.Machine](Data.Machine.md#v-is-infinite-f32) |
| `is-infinite-f64` | `F64 -> Bool` | [Data.Machine](Data.Machine.md#v-is-infinite-f64) |
| `is-na-nf32` | `F32 -> Bool` | [Data.Machine](Data.Machine.md#v-is-na-nf32) |
| `is-na-nf64` | `F64 -> Bool` | [Data.Machine](Data.Machine.md#v-is-na-nf64) |
| `is-none` | `forall a. Option a -> Bool` | [Data.Option](Data.Option.md#v-is-none) |
| `is-normal-f32` | `F32 -> Bool` | [Data.Machine](Data.Machine.md#v-is-normal-f32) |
| `is-normal-f64` | `F64 -> Bool` | [Data.Machine](Data.Machine.md#v-is-normal-f64) |
| `is-ok` | `forall e a. Result e a -> Bool` | [Data.Result](Data.Result.md#v-is-ok) |
| `is-some` | `forall a. Option a -> Bool` | [Data.Option](Data.Option.md#v-is-some) |
| `isize-to-f32` | `Isize -> F32` | [Data.Machine](Data.Machine.md#v-isize-to-f32) |
| `isize-to-f64` | `Isize -> F64` | [Data.Machine](Data.Machine.md#v-isize-to-f64) |
| `isize-to-i64` | `(a0 :: Isize) -> refined (r == a0) I64` | [Data.Machine](Data.Machine.md#v-isize-to-i64) |
| `isize-to-isize` | `Isize -> Isize` | [Data.Machine](Data.Machine.md#v-isize-to-isize) |
| `iterate` | `forall a. Nat -> (a -> a) -> a -> List a` | [Data.List](Data.List.md#v-iterate) |
| `last` | `forall a. List a -> Option a` | [Data.List](Data.List.md#v-last) |
| `le-char` | `Char -> Char -> Bool` | [Data.Char](Data.Char.md#v-le-char) |
| `le-i16` | `I16 -> I16 -> Bool` | [Data.Machine](Data.Machine.md#v-le-i16) |
| `le-i32` | `I32 -> I32 -> Bool` | [Data.Machine](Data.Machine.md#v-le-i32) |
| `le-i64` | `I64 -> I64 -> Bool` | [Data.Machine](Data.Machine.md#v-le-i64) |
| `le-i8` | `I8 -> I8 -> Bool` | [Data.Machine](Data.Machine.md#v-le-i8) |
| `le-isize` | `Isize -> Isize -> Bool` | [Data.Machine](Data.Machine.md#v-le-isize) |
| `le-u16` | `U16 -> U16 -> Bool` | [Data.Machine](Data.Machine.md#v-le-u16) |
| `le-u32` | `U32 -> U32 -> Bool` | [Data.Machine](Data.Machine.md#v-le-u32) |
| `le-u64` | `U64 -> U64 -> Bool` | [Data.Machine](Data.Machine.md#v-le-u64) |
| `le-u8` | `U8 -> U8 -> Bool` | [Data.Machine](Data.Machine.md#v-le-u8) |
| `le-usize` | `Usize -> Usize -> Bool` | [Data.Machine](Data.Machine.md#v-le-usize) |
| `leading-zeros-i16` | `I16 -> U32` | [Data.Machine](Data.Machine.md#v-leading-zeros-i16) |
| `leading-zeros-i32` | `I32 -> U32` | [Data.Machine](Data.Machine.md#v-leading-zeros-i32) |
| `leading-zeros-i64` | `I64 -> U32` | [Data.Machine](Data.Machine.md#v-leading-zeros-i64) |
| `leading-zeros-i8` | `I8 -> U32` | [Data.Machine](Data.Machine.md#v-leading-zeros-i8) |
| `leading-zeros-isize` | `Isize -> U32` | [Data.Machine](Data.Machine.md#v-leading-zeros-isize) |
| `leading-zeros-u16` | `U16 -> U32` | [Data.Machine](Data.Machine.md#v-leading-zeros-u16) |
| `leading-zeros-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-leading-zeros-u32) |
| `leading-zeros-u64` | `U64 -> U32` | [Data.Machine](Data.Machine.md#v-leading-zeros-u64) |
| `leading-zeros-u8` | `U8 -> U32` | [Data.Machine](Data.Machine.md#v-leading-zeros-u8) |
| `leading-zeros-usize` | `Usize -> U32` | [Data.Machine](Data.Machine.md#v-leading-zeros-usize) |
| `length` | `forall a. List a -> Int` | [Data.List](Data.List.md#v-length) |
| `list-length` | `forall a. List a -> Nat` | [Data.List](Data.List.md#v-list-length) |
| `list-to-option` | `forall a. List a -> Option a` | [Data.Option](Data.Option.md#v-list-to-option) |
| `lookup` | `forall k v. using (eq :: Eq k) => k -> List (k, v) -> Option v` | [Data.List](Data.List.md#v-lookup) |
| `lt-char` | `Char -> Char -> Bool` | [Data.Char](Data.Char.md#v-lt-char) |
| `lt-i16` | `I16 -> I16 -> Bool` | [Data.Machine](Data.Machine.md#v-lt-i16) |
| `lt-i32` | `I32 -> I32 -> Bool` | [Data.Machine](Data.Machine.md#v-lt-i32) |
| `lt-i64` | `I64 -> I64 -> Bool` | [Data.Machine](Data.Machine.md#v-lt-i64) |
| `lt-i8` | `I8 -> I8 -> Bool` | [Data.Machine](Data.Machine.md#v-lt-i8) |
| `lt-isize` | `Isize -> Isize -> Bool` | [Data.Machine](Data.Machine.md#v-lt-isize) |
| `lt-u16` | `U16 -> U16 -> Bool` | [Data.Machine](Data.Machine.md#v-lt-u16) |
| `lt-u32` | `U32 -> U32 -> Bool` | [Data.Machine](Data.Machine.md#v-lt-u32) |
| `lt-u64` | `U64 -> U64 -> Bool` | [Data.Machine](Data.Machine.md#v-lt-u64) |
| `lt-u8` | `U8 -> U8 -> Bool` | [Data.Machine](Data.Machine.md#v-lt-u8) |
| `lt-usize` | `Usize -> Usize -> Bool` | [Data.Machine](Data.Machine.md#v-lt-usize) |
| `map` | `forall a b. (a -> b) -> List a -> List b` | [Data.List](Data.List.md#v-map) |
| `map-delete` | `forall k v. using (ord :: Ord k) => k -> Map k v -> Map k v` | [Data.Map](Data.Map.md#v-map-delete) |
| `map-empty` | `forall k v. Map k v` | [Data.Map](Data.Map.md#v-map-empty) |
| `map-err` | `forall e f a. (e -> f) -> Result e a -> Result f a` | [Data.Result](Data.Result.md#v-map-err) |
| `map-filter` | `forall k v. (k -> v -> Bool) -> Map k v -> Map k v` | [Data.Map](Data.Map.md#v-map-filter) |
| `map-fold` | `forall k v b. (k -> v -> b -> b) -> b -> Map k v -> b` | [Data.Map](Data.Map.md#v-map-fold) |
| `map-from-pairs` | `forall k v. using (ord :: Ord k) => List (k, v) -> Map k v` | [Data.Map](Data.Map.md#v-map-from-pairs) |
| `map-insert` | `forall k v. using (ord :: Ord k) => k -> v -> Map k v -> Map k v` | [Data.Map](Data.Map.md#v-map-insert) |
| `map-is-empty` | `forall k v. Map k v -> Bool` | [Data.Map](Data.Map.md#v-map-is-empty) |
| `map-keys` | `forall k v. Map k v -> List k` | [Data.Map](Data.Map.md#v-map-keys) |
| `map-lookup` | `forall k v. using (ord :: Ord k) => k -> Map k v -> Option v` | [Data.Map](Data.Map.md#v-map-lookup) |
| `map-map` | `forall k v w. (v -> w) -> Map k v -> Map k w` | [Data.Map](Data.Map.md#v-map-map) |
| `map-option` | `forall a b. (a -> b) -> Option a -> Option b` | [Data.Option](Data.Option.md#v-map-option) |
| `map-result` | `forall e a b. (a -> b) -> Result e a -> Result e b` | [Data.Result](Data.Result.md#v-map-result) |
| `map-size` | `forall k v. Map k v -> Nat` | [Data.Map](Data.Map.md#v-map-size) |
| `map-to-list` | `forall k v. Map k v -> List (k, v)` | [Data.Map](Data.Map.md#v-map-to-list) |
| `map-union` | `forall k v. using (ord :: Ord k) => Map k v -> Map k v -> Map k v` | [Data.Map](Data.Map.md#v-map-union) |
| `map-union-with` | `forall k v. using (ord :: Ord k) => (v -> v -> v) -> Map k v -> Map k v -> Map k v` | [Data.Map](Data.Map.md#v-map-union-with) |
| `map-values` | `forall k v. Map k v -> List v` | [Data.Map](Data.Map.md#v-map-values) |
| `mask-i64` | `I64 -> (m :: InRange 0 9223372036854775807 I64) -> refined (0 <= cinch-result && cinch-result <= m) I64` | [Data.Machine](Data.Machine.md#v-mask-i64) |
| `max-bound-i16` | `refined (cinch-result == 32767) I16` | [Data.Machine](Data.Machine.md#v-max-bound-i16) |
| `max-bound-i32` | `refined (cinch-result == 2147483647) I32` | [Data.Machine](Data.Machine.md#v-max-bound-i32) |
| `max-bound-i64` | `refined (cinch-result == 9223372036854775807) I64` | [Data.Machine](Data.Machine.md#v-max-bound-i64) |
| `max-bound-i8` | `refined (cinch-result == 127) I8` | [Data.Machine](Data.Machine.md#v-max-bound-i8) |
| `max-bound-isize` | `Isize` | [Data.Machine](Data.Machine.md#v-max-bound-isize) |
| `max-bound-u16` | `refined (cinch-result == 65535) U16` | [Data.Machine](Data.Machine.md#v-max-bound-u16) |
| `max-bound-u32` | `refined (cinch-result == 4294967295) U32` | [Data.Machine](Data.Machine.md#v-max-bound-u32) |
| `max-bound-u64` | `refined (cinch-result == 18446744073709551615) U64` | [Data.Machine](Data.Machine.md#v-max-bound-u64) |
| `max-bound-u8` | `refined (cinch-result == 255) U8` | [Data.Machine](Data.Machine.md#v-max-bound-u8) |
| `max-bound-usize` | `Usize` | [Data.Machine](Data.Machine.md#v-max-bound-usize) |
| `maximum-f32` | `F32 -> F32 -> F32` | [Data.Machine](Data.Machine.md#v-maximum-f32) |
| `maximum-f64` | `F64 -> F64 -> F64` | [Data.Machine](Data.Machine.md#v-maximum-f64) |
| `maybe` | `forall a b. b -> (a -> b) -> Option a -> b` | [Data.Option](Data.Option.md#v-maybe) |
| `min-bound-i16` | `refined (cinch-result == - 32768) I16` | [Data.Machine](Data.Machine.md#v-min-bound-i16) |
| `min-bound-i32` | `refined (cinch-result == - 2147483648) I32` | [Data.Machine](Data.Machine.md#v-min-bound-i32) |
| `min-bound-i64` | `refined (cinch-result == - 9223372036854775808) I64` | [Data.Machine](Data.Machine.md#v-min-bound-i64) |
| `min-bound-i8` | `refined (cinch-result == - 128) I8` | [Data.Machine](Data.Machine.md#v-min-bound-i8) |
| `min-bound-isize` | `Isize` | [Data.Machine](Data.Machine.md#v-min-bound-isize) |
| `min-bound-u16` | `refined (cinch-result == 0) U16` | [Data.Machine](Data.Machine.md#v-min-bound-u16) |
| `min-bound-u32` | `refined (cinch-result == 0) U32` | [Data.Machine](Data.Machine.md#v-min-bound-u32) |
| `min-bound-u64` | `refined (cinch-result == 0) U64` | [Data.Machine](Data.Machine.md#v-min-bound-u64) |
| `min-bound-u8` | `refined (cinch-result == 0) U8` | [Data.Machine](Data.Machine.md#v-min-bound-u8) |
| `min-bound-usize` | `Usize` | [Data.Machine](Data.Machine.md#v-min-bound-usize) |
| `minimum-f32` | `F32 -> F32 -> F32` | [Data.Machine](Data.Machine.md#v-minimum-f32) |
| `minimum-f64` | `F64 -> F64 -> F64` | [Data.Machine](Data.Machine.md#v-minimum-f64) |
| `mul-f32` | `F32 -> F32 -> F32` | [Data.Machine](Data.Machine.md#v-mul-f32) |
| `mul-f64` | `F64 -> F64 -> F64` | [Data.Machine](Data.Machine.md#v-mul-f64) |
| `nat-add` | `Nat -> Nat -> Nat` | [Data.Nat](Data.Nat.md#v-nat-add) |
| `nat-cmp` | `Nat -> Nat -> I64` | [Data.Nat](Data.Nat.md#v-nat-cmp) |
| `nat-digit-count` | `Nat -> Nat` | [Text.Show](Text.Show.md#v-nat-digit-count) |
| `nat-digits-into` | `arg1 + arg3 <= (bytes-len arg0) => (arg0 :: Bytes) -> (arg1 :: Nat) -> (arg2 :: Nat) -> (arg3 :: Nat) -> Unit` | [Text.Show](Text.Show.md#v-nat-digits-into) |
| `nat-div` | `Nat -> NonZero Nat -> Nat` | [Data.Nat](Data.Nat.md#v-nat-div) |
| `nat-from-i64` | `0 <= arg0 => (arg0 :: I64) -> refined (r == arg0) Nat` | [Data.Nat](Data.Nat.md#v-nat-from-i64) |
| `nat-from-int` | `0 <= arg0 => (arg0 :: Int) -> refined (r == arg0) Nat` | [Data.Nat](Data.Nat.md#v-nat-from-int) |
| `nat-mul` | `Nat -> Nat -> Nat` | [Data.Nat](Data.Nat.md#v-nat-mul) |
| `nat-rem` | `Nat -> (d :: NonZero Nat) -> refined (cinch-result < d) Nat` | [Data.Nat](Data.Nat.md#v-nat-rem) |
| `nat-sub` | `arg1 <= arg0 => (arg0 :: Nat) -> (arg1 :: Nat) -> Nat` | [Data.Nat](Data.Nat.md#v-nat-sub) |
| `nat-to-i64` | `arg0 <= 9223372036854775807 => (arg0 :: Nat) -> refined (r == arg0) I64` | [Data.Nat](Data.Nat.md#v-nat-to-i64) |
| `nat-to-int` | `(n :: Nat) -> refined (cinch-result == n) Int` | [Data.Nat](Data.Nat.md#v-nat-to-int) |
| `nat-to-string` | `Nat -> String` | [Text.Show](Text.Show.md#v-nat-to-string) |
| `ne-char` | `Char -> Char -> Bool` | [Data.Char](Data.Char.md#v-ne-char) |
| `ne-i16` | `I16 -> I16 -> Bool` | [Data.Machine](Data.Machine.md#v-ne-i16) |
| `ne-i32` | `I32 -> I32 -> Bool` | [Data.Machine](Data.Machine.md#v-ne-i32) |
| `ne-i64` | `I64 -> I64 -> Bool` | [Data.Machine](Data.Machine.md#v-ne-i64) |
| `ne-i8` | `I8 -> I8 -> Bool` | [Data.Machine](Data.Machine.md#v-ne-i8) |
| `ne-isize` | `Isize -> Isize -> Bool` | [Data.Machine](Data.Machine.md#v-ne-isize) |
| `ne-u16` | `U16 -> U16 -> Bool` | [Data.Machine](Data.Machine.md#v-ne-u16) |
| `ne-u32` | `U32 -> U32 -> Bool` | [Data.Machine](Data.Machine.md#v-ne-u32) |
| `ne-u64` | `U64 -> U64 -> Bool` | [Data.Machine](Data.Machine.md#v-ne-u64) |
| `ne-u8` | `U8 -> U8 -> Bool` | [Data.Machine](Data.Machine.md#v-ne-u8) |
| `ne-usize` | `Usize -> Usize -> Bool` | [Data.Machine](Data.Machine.md#v-ne-usize) |
| `neg-f32` | `F32 -> F32` | [Data.Machine](Data.Machine.md#v-neg-f32) |
| `neg-f64` | `F64 -> F64` | [Data.Machine](Data.Machine.md#v-neg-f64) |
| `not` | `Bool -> Bool` | [Data.Bool](Data.Bool.md#v-not) |
| `nth` | `forall a. Nat -> List a -> Option a` | [Data.List](Data.List.md#v-nth) |
| `nub` | `forall a. using (eq :: Eq a) => List a -> List a` | [Data.List](Data.List.md#v-nub) |
| `on` | `forall a b c. (b -> b -> c) -> (a -> b) -> a -> a -> c` | [Function.Combinators](Function.Combinators.md#v-on) |
| `option-to-list` | `forall a. Option a -> List a` | [Data.Option](Data.Option.md#v-option-to-list) |
| `option-to-result` | `forall e a. e -> Option a -> Result e a` | [Data.Result](Data.Result.md#v-option-to-result) |
| `or` | `Bool -> Bool -> Bool` | [Data.Bool](Data.Bool.md#v-or) |
| `or-else` | `forall a. Option a -> Option a -> Option a` | [Data.Option](Data.Option.md#v-or-else) |
| `pack` | `List Char -> String` | [Data.String](Data.String.md#v-pack) |
| `parse-config` | `Bytes -> Result ConfigError Config` | [Effect.Io](Effect.Io.md#v-parse-config) |
| `partition` | `forall a. (a -> Bool) -> List a -> (List a, List a)` | [Data.List](Data.List.md#v-partition) |
| `partition-results` | `forall e a. List (Result e a) -> (List e, List a)` | [Data.Result](Data.Result.md#v-partition-results) |
| `pow10` | `Nat -> Nat` | [Text.Show](Text.Show.md#v-pow10) |
| `prim-and-bool` | `Bool -> Bool -> Bool` | [Data.Bool](Data.Bool.md#v-prim-and-bool) |
| `prim-eq-bool` | `Bool -> Bool -> Bool` | [Data.Bool](Data.Bool.md#v-prim-eq-bool) |
| `prim-ne-bool` | `Bool -> Bool -> Bool` | [Data.Bool](Data.Bool.md#v-prim-ne-bool) |
| `prim-not-bool` | `Bool -> Bool` | [Data.Bool](Data.Bool.md#v-prim-not-bool) |
| `prim-or-bool` | `Bool -> Bool -> Bool` | [Data.Bool](Data.Bool.md#v-prim-or-bool) |
| `prod-fold` | `forall s a b. b -> (b -> a -> b) -> Producer s a -> b` | [Data.Producer](Data.Producer.md#v-prod-fold) |
| `prod-fold-go` | `forall s a b. (s -> Step s a) -> (b -> a -> b) -> b -> s -> b` | [Data.Producer](Data.Producer.md#v-prod-fold-go) |
| `prod-map` | `forall s a b. (a -> b) -> Producer s a -> Producer s b` | [Data.Producer](Data.Producer.md#v-prod-map) |
| `prod-take` | `forall s a. Nat -> Producer s a -> Producer (TakeSt s) a` | [Data.Producer](Data.Producer.md#v-prod-take) |
| `prod-take-step` | `forall s a. (s -> Step s a) -> TakeSt s -> Step (TakeSt s) a` | [Data.Producer](Data.Producer.md#v-prod-take-step) |
| `read-config-bytes` | `forall (r :: Region). using (io :: Io r) => Path ->{IO r} Result ConfigError Bytes` | [Effect.Io](Effect.Io.md#v-read-config-bytes) |
| `replicate` | `forall a. Nat -> a -> List a` | [Data.List](Data.List.md#v-replicate) |
| `result-to-option` | `forall e a. Result e a -> Option a` | [Data.Result](Data.Result.md#v-result-to-option) |
| `results` | `forall e a. List (Result e a) -> List a` | [Data.Result](Data.Result.md#v-results) |
| `reverse` | `forall a. List a -> List a` | [Data.List](Data.List.md#v-reverse) |
| `reverse-bits-i16` | `I16 -> I16` | [Data.Machine](Data.Machine.md#v-reverse-bits-i16) |
| `reverse-bits-i32` | `I32 -> I32` | [Data.Machine](Data.Machine.md#v-reverse-bits-i32) |
| `reverse-bits-i64` | `I64 -> I64` | [Data.Machine](Data.Machine.md#v-reverse-bits-i64) |
| `reverse-bits-i8` | `I8 -> I8` | [Data.Machine](Data.Machine.md#v-reverse-bits-i8) |
| `reverse-bits-isize` | `Isize -> Isize` | [Data.Machine](Data.Machine.md#v-reverse-bits-isize) |
| `reverse-bits-u16` | `U16 -> U16` | [Data.Machine](Data.Machine.md#v-reverse-bits-u16) |
| `reverse-bits-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-reverse-bits-u32) |
| `reverse-bits-u64` | `U64 -> U64` | [Data.Machine](Data.Machine.md#v-reverse-bits-u64) |
| `reverse-bits-u8` | `U8 -> U8` | [Data.Machine](Data.Machine.md#v-reverse-bits-u8) |
| `reverse-bits-usize` | `Usize -> Usize` | [Data.Machine](Data.Machine.md#v-reverse-bits-usize) |
| `rotate-left-i16` | `I16 -> U32 -> I16` | [Data.Machine](Data.Machine.md#v-rotate-left-i16) |
| `rotate-left-i32` | `I32 -> U32 -> I32` | [Data.Machine](Data.Machine.md#v-rotate-left-i32) |
| `rotate-left-i64` | `I64 -> U32 -> I64` | [Data.Machine](Data.Machine.md#v-rotate-left-i64) |
| `rotate-left-i8` | `I8 -> U32 -> I8` | [Data.Machine](Data.Machine.md#v-rotate-left-i8) |
| `rotate-left-isize` | `Isize -> U32 -> Isize` | [Data.Machine](Data.Machine.md#v-rotate-left-isize) |
| `rotate-left-u16` | `U16 -> U32 -> U16` | [Data.Machine](Data.Machine.md#v-rotate-left-u16) |
| `rotate-left-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-rotate-left-u32) |
| `rotate-left-u64` | `U64 -> U32 -> U64` | [Data.Machine](Data.Machine.md#v-rotate-left-u64) |
| `rotate-left-u8` | `U8 -> U32 -> U8` | [Data.Machine](Data.Machine.md#v-rotate-left-u8) |
| `rotate-left-usize` | `Usize -> U32 -> Usize` | [Data.Machine](Data.Machine.md#v-rotate-left-usize) |
| `rotate-right-i16` | `I16 -> U32 -> I16` | [Data.Machine](Data.Machine.md#v-rotate-right-i16) |
| `rotate-right-i32` | `I32 -> U32 -> I32` | [Data.Machine](Data.Machine.md#v-rotate-right-i32) |
| `rotate-right-i64` | `I64 -> U32 -> I64` | [Data.Machine](Data.Machine.md#v-rotate-right-i64) |
| `rotate-right-i8` | `I8 -> U32 -> I8` | [Data.Machine](Data.Machine.md#v-rotate-right-i8) |
| `rotate-right-isize` | `Isize -> U32 -> Isize` | [Data.Machine](Data.Machine.md#v-rotate-right-isize) |
| `rotate-right-u16` | `U16 -> U32 -> U16` | [Data.Machine](Data.Machine.md#v-rotate-right-u16) |
| `rotate-right-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-rotate-right-u32) |
| `rotate-right-u64` | `U64 -> U32 -> U64` | [Data.Machine](Data.Machine.md#v-rotate-right-u64) |
| `rotate-right-u8` | `U8 -> U32 -> U8` | [Data.Machine](Data.Machine.md#v-rotate-right-u8) |
| `rotate-right-usize` | `Usize -> U32 -> Usize` | [Data.Machine](Data.Machine.md#v-rotate-right-usize) |
| `saturating-add-i16` | `I16 -> I16 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-add-i16) |
| `saturating-add-i32` | `I32 -> I32 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-add-i32) |
| `saturating-add-i64` | `I64 -> I64 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-add-i64) |
| `saturating-add-i8` | `I8 -> I8 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-add-i8) |
| `saturating-add-isize` | `Isize -> Isize -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-add-isize) |
| `saturating-add-u16` | `U16 -> U16 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-add-u16) |
| `saturating-add-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-add-u32) |
| `saturating-add-u64` | `U64 -> U64 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-add-u64) |
| `saturating-add-u8` | `U8 -> U8 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-add-u8) |
| `saturating-add-usize` | `Usize -> Usize -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-add-usize) |
| `saturating-f32-to-i16` | `F32 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-f32-to-i16) |
| `saturating-f32-to-i32` | `F32 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-f32-to-i32) |
| `saturating-f32-to-i64` | `F32 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-f32-to-i64) |
| `saturating-f32-to-i8` | `F32 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-f32-to-i8) |
| `saturating-f32-to-isize` | `F32 -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-f32-to-isize) |
| `saturating-f32-to-u16` | `F32 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-f32-to-u16) |
| `saturating-f32-to-u32` | `F32 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-f32-to-u32) |
| `saturating-f32-to-u64` | `F32 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-f32-to-u64) |
| `saturating-f32-to-u8` | `F32 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-f32-to-u8) |
| `saturating-f32-to-usize` | `F32 -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-f32-to-usize) |
| `saturating-f64-to-i16` | `F64 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-f64-to-i16) |
| `saturating-f64-to-i32` | `F64 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-f64-to-i32) |
| `saturating-f64-to-i64` | `F64 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-f64-to-i64) |
| `saturating-f64-to-i8` | `F64 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-f64-to-i8) |
| `saturating-f64-to-isize` | `F64 -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-f64-to-isize) |
| `saturating-f64-to-u16` | `F64 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-f64-to-u16) |
| `saturating-f64-to-u32` | `F64 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-f64-to-u32) |
| `saturating-f64-to-u64` | `F64 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-f64-to-u64) |
| `saturating-f64-to-u8` | `F64 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-f64-to-u8) |
| `saturating-f64-to-usize` | `F64 -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-f64-to-usize) |
| `saturating-i16-to-i16` | `I16 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-i16-to-i16) |
| `saturating-i16-to-i32` | `I16 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-i16-to-i32) |
| `saturating-i16-to-i64` | `I16 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-i16-to-i64) |
| `saturating-i16-to-i8` | `I16 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-i16-to-i8) |
| `saturating-i16-to-isize` | `I16 -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-i16-to-isize) |
| `saturating-i16-to-u16` | `I16 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-i16-to-u16) |
| `saturating-i16-to-u32` | `I16 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-i16-to-u32) |
| `saturating-i16-to-u64` | `I16 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-i16-to-u64) |
| `saturating-i16-to-u8` | `I16 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-i16-to-u8) |
| `saturating-i16-to-usize` | `I16 -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-i16-to-usize) |
| `saturating-i32-to-i16` | `I32 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-i32-to-i16) |
| `saturating-i32-to-i32` | `I32 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-i32-to-i32) |
| `saturating-i32-to-i64` | `I32 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-i32-to-i64) |
| `saturating-i32-to-i8` | `I32 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-i32-to-i8) |
| `saturating-i32-to-isize` | `I32 -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-i32-to-isize) |
| `saturating-i32-to-u16` | `I32 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-i32-to-u16) |
| `saturating-i32-to-u32` | `I32 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-i32-to-u32) |
| `saturating-i32-to-u64` | `I32 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-i32-to-u64) |
| `saturating-i32-to-u8` | `I32 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-i32-to-u8) |
| `saturating-i32-to-usize` | `I32 -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-i32-to-usize) |
| `saturating-i64-to-i16` | `I64 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-i64-to-i16) |
| `saturating-i64-to-i32` | `I64 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-i64-to-i32) |
| `saturating-i64-to-i64` | `I64 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-i64-to-i64) |
| `saturating-i64-to-i8` | `I64 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-i64-to-i8) |
| `saturating-i64-to-isize` | `I64 -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-i64-to-isize) |
| `saturating-i64-to-u16` | `I64 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-i64-to-u16) |
| `saturating-i64-to-u32` | `I64 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-i64-to-u32) |
| `saturating-i64-to-u64` | `I64 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-i64-to-u64) |
| `saturating-i64-to-u8` | `I64 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-i64-to-u8) |
| `saturating-i64-to-usize` | `I64 -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-i64-to-usize) |
| `saturating-i8-to-i16` | `I8 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-i8-to-i16) |
| `saturating-i8-to-i32` | `I8 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-i8-to-i32) |
| `saturating-i8-to-i64` | `I8 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-i8-to-i64) |
| `saturating-i8-to-i8` | `I8 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-i8-to-i8) |
| `saturating-i8-to-isize` | `I8 -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-i8-to-isize) |
| `saturating-i8-to-u16` | `I8 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-i8-to-u16) |
| `saturating-i8-to-u32` | `I8 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-i8-to-u32) |
| `saturating-i8-to-u64` | `I8 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-i8-to-u64) |
| `saturating-i8-to-u8` | `I8 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-i8-to-u8) |
| `saturating-i8-to-usize` | `I8 -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-i8-to-usize) |
| `saturating-isize-to-i16` | `Isize -> I16` | [Data.Machine](Data.Machine.md#v-saturating-isize-to-i16) |
| `saturating-isize-to-i32` | `Isize -> I32` | [Data.Machine](Data.Machine.md#v-saturating-isize-to-i32) |
| `saturating-isize-to-i64` | `Isize -> I64` | [Data.Machine](Data.Machine.md#v-saturating-isize-to-i64) |
| `saturating-isize-to-i8` | `Isize -> I8` | [Data.Machine](Data.Machine.md#v-saturating-isize-to-i8) |
| `saturating-isize-to-isize` | `Isize -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-isize-to-isize) |
| `saturating-isize-to-u16` | `Isize -> U16` | [Data.Machine](Data.Machine.md#v-saturating-isize-to-u16) |
| `saturating-isize-to-u32` | `Isize -> U32` | [Data.Machine](Data.Machine.md#v-saturating-isize-to-u32) |
| `saturating-isize-to-u64` | `Isize -> U64` | [Data.Machine](Data.Machine.md#v-saturating-isize-to-u64) |
| `saturating-isize-to-u8` | `Isize -> U8` | [Data.Machine](Data.Machine.md#v-saturating-isize-to-u8) |
| `saturating-isize-to-usize` | `Isize -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-isize-to-usize) |
| `saturating-mul-i16` | `I16 -> I16 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-mul-i16) |
| `saturating-mul-i32` | `I32 -> I32 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-mul-i32) |
| `saturating-mul-i64` | `I64 -> I64 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-mul-i64) |
| `saturating-mul-i8` | `I8 -> I8 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-mul-i8) |
| `saturating-mul-isize` | `Isize -> Isize -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-mul-isize) |
| `saturating-mul-u16` | `U16 -> U16 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-mul-u16) |
| `saturating-mul-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-mul-u32) |
| `saturating-mul-u64` | `U64 -> U64 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-mul-u64) |
| `saturating-mul-u8` | `U8 -> U8 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-mul-u8) |
| `saturating-mul-usize` | `Usize -> Usize -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-mul-usize) |
| `saturating-neg-i16` | `I16 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-neg-i16) |
| `saturating-neg-i32` | `I32 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-neg-i32) |
| `saturating-neg-i64` | `I64 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-neg-i64) |
| `saturating-neg-i8` | `I8 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-neg-i8) |
| `saturating-neg-isize` | `Isize -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-neg-isize) |
| `saturating-neg-u16` | `U16 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-neg-u16) |
| `saturating-neg-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-neg-u32) |
| `saturating-neg-u64` | `U64 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-neg-u64) |
| `saturating-neg-u8` | `U8 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-neg-u8) |
| `saturating-neg-usize` | `Usize -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-neg-usize) |
| `saturating-sub` | `Nat -> Nat -> Nat` | [Data.Nat](Data.Nat.md#v-saturating-sub) |
| `saturating-sub-i16` | `I16 -> I16 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-sub-i16) |
| `saturating-sub-i32` | `I32 -> I32 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-sub-i32) |
| `saturating-sub-i64` | `I64 -> I64 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-sub-i64) |
| `saturating-sub-i8` | `I8 -> I8 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-sub-i8) |
| `saturating-sub-isize` | `Isize -> Isize -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-sub-isize) |
| `saturating-sub-u16` | `U16 -> U16 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-sub-u16) |
| `saturating-sub-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-sub-u32) |
| `saturating-sub-u64` | `U64 -> U64 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-sub-u64) |
| `saturating-sub-u8` | `U8 -> U8 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-sub-u8) |
| `saturating-sub-usize` | `Usize -> Usize -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-sub-usize) |
| `saturating-u16-to-i16` | `U16 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-u16-to-i16) |
| `saturating-u16-to-i32` | `U16 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-u16-to-i32) |
| `saturating-u16-to-i64` | `U16 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-u16-to-i64) |
| `saturating-u16-to-i8` | `U16 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-u16-to-i8) |
| `saturating-u16-to-isize` | `U16 -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-u16-to-isize) |
| `saturating-u16-to-u16` | `U16 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-u16-to-u16) |
| `saturating-u16-to-u32` | `U16 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-u16-to-u32) |
| `saturating-u16-to-u64` | `U16 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-u16-to-u64) |
| `saturating-u16-to-u8` | `U16 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-u16-to-u8) |
| `saturating-u16-to-usize` | `U16 -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-u16-to-usize) |
| `saturating-u32-to-i16` | `U32 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-u32-to-i16) |
| `saturating-u32-to-i32` | `U32 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-u32-to-i32) |
| `saturating-u32-to-i64` | `U32 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-u32-to-i64) |
| `saturating-u32-to-i8` | `U32 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-u32-to-i8) |
| `saturating-u32-to-isize` | `U32 -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-u32-to-isize) |
| `saturating-u32-to-u16` | `U32 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-u32-to-u16) |
| `saturating-u32-to-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-u32-to-u32) |
| `saturating-u32-to-u64` | `U32 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-u32-to-u64) |
| `saturating-u32-to-u8` | `U32 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-u32-to-u8) |
| `saturating-u32-to-usize` | `U32 -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-u32-to-usize) |
| `saturating-u64-to-i16` | `U64 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-u64-to-i16) |
| `saturating-u64-to-i32` | `U64 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-u64-to-i32) |
| `saturating-u64-to-i64` | `U64 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-u64-to-i64) |
| `saturating-u64-to-i8` | `U64 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-u64-to-i8) |
| `saturating-u64-to-isize` | `U64 -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-u64-to-isize) |
| `saturating-u64-to-u16` | `U64 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-u64-to-u16) |
| `saturating-u64-to-u32` | `U64 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-u64-to-u32) |
| `saturating-u64-to-u64` | `U64 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-u64-to-u64) |
| `saturating-u64-to-u8` | `U64 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-u64-to-u8) |
| `saturating-u64-to-usize` | `U64 -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-u64-to-usize) |
| `saturating-u8-to-i16` | `U8 -> I16` | [Data.Machine](Data.Machine.md#v-saturating-u8-to-i16) |
| `saturating-u8-to-i32` | `U8 -> I32` | [Data.Machine](Data.Machine.md#v-saturating-u8-to-i32) |
| `saturating-u8-to-i64` | `U8 -> I64` | [Data.Machine](Data.Machine.md#v-saturating-u8-to-i64) |
| `saturating-u8-to-i8` | `U8 -> I8` | [Data.Machine](Data.Machine.md#v-saturating-u8-to-i8) |
| `saturating-u8-to-isize` | `U8 -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-u8-to-isize) |
| `saturating-u8-to-u16` | `U8 -> U16` | [Data.Machine](Data.Machine.md#v-saturating-u8-to-u16) |
| `saturating-u8-to-u32` | `U8 -> U32` | [Data.Machine](Data.Machine.md#v-saturating-u8-to-u32) |
| `saturating-u8-to-u64` | `U8 -> U64` | [Data.Machine](Data.Machine.md#v-saturating-u8-to-u64) |
| `saturating-u8-to-u8` | `U8 -> U8` | [Data.Machine](Data.Machine.md#v-saturating-u8-to-u8) |
| `saturating-u8-to-usize` | `U8 -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-u8-to-usize) |
| `saturating-usize-to-i16` | `Usize -> I16` | [Data.Machine](Data.Machine.md#v-saturating-usize-to-i16) |
| `saturating-usize-to-i32` | `Usize -> I32` | [Data.Machine](Data.Machine.md#v-saturating-usize-to-i32) |
| `saturating-usize-to-i64` | `Usize -> I64` | [Data.Machine](Data.Machine.md#v-saturating-usize-to-i64) |
| `saturating-usize-to-i8` | `Usize -> I8` | [Data.Machine](Data.Machine.md#v-saturating-usize-to-i8) |
| `saturating-usize-to-isize` | `Usize -> Isize` | [Data.Machine](Data.Machine.md#v-saturating-usize-to-isize) |
| `saturating-usize-to-u16` | `Usize -> U16` | [Data.Machine](Data.Machine.md#v-saturating-usize-to-u16) |
| `saturating-usize-to-u32` | `Usize -> U32` | [Data.Machine](Data.Machine.md#v-saturating-usize-to-u32) |
| `saturating-usize-to-u64` | `Usize -> U64` | [Data.Machine](Data.Machine.md#v-saturating-usize-to-u64) |
| `saturating-usize-to-u8` | `Usize -> U8` | [Data.Machine](Data.Machine.md#v-saturating-usize-to-u8) |
| `saturating-usize-to-usize` | `Usize -> Usize` | [Data.Machine](Data.Machine.md#v-saturating-usize-to-usize) |
| `scalar-sum-wrap32` | `forall (n :: Nat) region. Ref region Vec n U32 -> U32` | [Data.Producer](Data.Producer.md#v-scalar-sum-wrap32) |
| `scanl` | `forall a b. (b -> a -> b) -> b -> List a -> List b` | [Data.List](Data.List.md#v-scanl) |
| `scanr` | `forall a b. (a -> b -> b) -> b -> List a -> List b` | [Data.List](Data.List.md#v-scanr) |
| `sha256-bytes` | `Bytes -> Bytes` | [Data.Bytes](Data.Bytes.md#v-sha256-bytes) |
| `sign-bit-f32` | `F32 -> Bool` | [Data.Machine](Data.Machine.md#v-sign-bit-f32) |
| `sign-bit-f64` | `F64 -> Bool` | [Data.Machine](Data.Machine.md#v-sign-bit-f64) |
| `simd-sum-wrap32` | `forall t w (n :: Nat) region. VecU32Target t w -> Ref region Vec n U32 -> U32` | [Data.Target](Data.Target.md#v-simd-sum-wrap32) |
| `snd` | `forall a b. (a, b) -> b` | [Function.Combinators](Function.Combinators.md#v-snd) |
| `sort` | `forall a. using (ord :: Ord a) => List a -> List a` | [Data.List](Data.List.md#v-sort) |
| `sort-by` | `forall a. (a -> a -> Ordering) -> List a -> List a` | [Data.List](Data.List.md#v-sort-by) |
| `span` | `forall a. (a -> Bool) -> List a -> (List a, List a)` | [Data.List](Data.List.md#v-span) |
| `split-at` | `forall a. Nat -> List a -> (List a, List a)` | [Data.List](Data.List.md#v-split-at) |
| `sqrt-f32` | `F32 -> F32` | [Data.Machine](Data.Machine.md#v-sqrt-f32) |
| `sqrt-f64` | `F64 -> F64` | [Data.Machine](Data.Machine.md#v-sqrt-f64) |
| `string-byte-index-from` | `arg1 <= (bytes-len arg0) => (arg0 :: Bytes) -> (arg1 :: Nat) -> (arg2 :: Nat) -> refined (arg1 <= r && r <= (bytes-len arg0)) Nat` | [Data.String](Data.String.md#v-string-byte-index-from) |
| `string-byte-length` | `String -> Nat` | [Data.String](Data.String.md#v-string-byte-length) |
| `string-char-at` | `String -> Nat -> Option Char` | [Data.String](Data.String.md#v-string-char-at) |
| `string-char-at-from` | `Bytes -> Nat -> Nat -> Option Char` | [Data.String](Data.String.md#v-string-char-at-from) |
| `string-clone` | `String -> String` | [Data.String](Data.String.md#v-string-clone) |
| `string-compare` | `String -> String -> I64` | [Data.String](Data.String.md#v-string-compare) |
| `string-concat` | `String -> String -> String` | [Data.String](Data.String.md#v-string-concat) |
| `string-concat-all` | `List String -> String` | [Data.String](Data.String.md#v-string-concat-all) |
| `string-from-bytes` | `Bytes -> Option String` | [Data.String](Data.String.md#v-string-from-bytes) |
| `string-join` | `List String -> String -> String` | [Data.String](Data.String.md#v-string-join) |
| `string-length` | `String -> Nat` | [Data.String](Data.String.md#v-string-length) |
| `string-length-from` | `Bytes -> Nat -> Nat -> Nat` | [Data.String](Data.String.md#v-string-length-from) |
| `string-repeat` | `Nat -> String -> String` | [Data.String](Data.String.md#v-string-repeat) |
| `string-slice` | `String -> Nat -> Nat -> String` | [Data.String](Data.String.md#v-string-slice) |
| `string-to-bytes` | `String -> Bytes` | [Data.String](Data.String.md#v-string-to-bytes) |
| `string-utf8-first` | `String -> Option (Char, Nat)` | [Data.String](Data.String.md#v-string-utf8-first) |
| `string-utf8-last` | `String -> Option (Char, Nat)` | [Data.String](Data.String.md#v-string-utf8-last) |
| `sub-f32` | `F32 -> F32 -> F32` | [Data.Machine](Data.Machine.md#v-sub-f32) |
| `sub-f64` | `F64 -> F64 -> F64` | [Data.Machine](Data.Machine.md#v-sub-f64) |
| `subsequences` | `forall a. List a -> List (List a)` | [Data.List](Data.List.md#v-subsequences) |
| `sum-list` | `forall a. using (add :: Add a) => a -> List a -> a` | [Data.List](Data.List.md#v-sum-list) |
| `swap` | `forall a b. (a, b) -> (b, a)` | [Function.Combinators](Function.Combinators.md#v-swap) |
| `swap-bytes-i16` | `I16 -> I16` | [Data.Machine](Data.Machine.md#v-swap-bytes-i16) |
| `swap-bytes-i32` | `I32 -> I32` | [Data.Machine](Data.Machine.md#v-swap-bytes-i32) |
| `swap-bytes-i64` | `I64 -> I64` | [Data.Machine](Data.Machine.md#v-swap-bytes-i64) |
| `swap-bytes-i8` | `I8 -> I8` | [Data.Machine](Data.Machine.md#v-swap-bytes-i8) |
| `swap-bytes-isize` | `Isize -> Isize` | [Data.Machine](Data.Machine.md#v-swap-bytes-isize) |
| `swap-bytes-u16` | `U16 -> U16` | [Data.Machine](Data.Machine.md#v-swap-bytes-u16) |
| `swap-bytes-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-swap-bytes-u32) |
| `swap-bytes-u64` | `U64 -> U64` | [Data.Machine](Data.Machine.md#v-swap-bytes-u64) |
| `swap-bytes-u8` | `U8 -> U8` | [Data.Machine](Data.Machine.md#v-swap-bytes-u8) |
| `swap-bytes-usize` | `Usize -> Usize` | [Data.Machine](Data.Machine.md#v-swap-bytes-usize) |
| `sym-name` | `Sym -> String` | [Data.Sym](Data.Sym.md#v-sym-name) |
| `tail` | `forall a. List a -> Option (List a)` | [Data.List](Data.List.md#v-tail) |
| `take` | `forall a. Nat -> List a -> List a` | [Data.List](Data.List.md#v-take) |
| `to-lower-ascii` | `Char -> Char` | [Data.Char](Data.Char.md#v-to-lower-ascii) |
| `to-upper-ascii` | `Char -> Char` | [Data.Char](Data.Char.md#v-to-upper-ascii) |
| `total-order-f32` | `F32 -> F32 -> Ordering` | [Data.Machine](Data.Machine.md#v-total-order-f32) |
| `total-order-f64` | `F64 -> F64 -> Ordering` | [Data.Machine](Data.Machine.md#v-total-order-f64) |
| `trailing-zeros-i16` | `I16 -> U32` | [Data.Machine](Data.Machine.md#v-trailing-zeros-i16) |
| `trailing-zeros-i32` | `I32 -> U32` | [Data.Machine](Data.Machine.md#v-trailing-zeros-i32) |
| `trailing-zeros-i64` | `I64 -> U32` | [Data.Machine](Data.Machine.md#v-trailing-zeros-i64) |
| `trailing-zeros-i8` | `I8 -> U32` | [Data.Machine](Data.Machine.md#v-trailing-zeros-i8) |
| `trailing-zeros-isize` | `Isize -> U32` | [Data.Machine](Data.Machine.md#v-trailing-zeros-isize) |
| `trailing-zeros-u16` | `U16 -> U32` | [Data.Machine](Data.Machine.md#v-trailing-zeros-u16) |
| `trailing-zeros-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-trailing-zeros-u32) |
| `trailing-zeros-u64` | `U64 -> U32` | [Data.Machine](Data.Machine.md#v-trailing-zeros-u64) |
| `trailing-zeros-u8` | `U8 -> U32` | [Data.Machine](Data.Machine.md#v-trailing-zeros-u8) |
| `trailing-zeros-usize` | `Usize -> U32` | [Data.Machine](Data.Machine.md#v-trailing-zeros-usize) |
| `transpose` | `forall a. List (List a) -> List (List a)` | [Data.List](Data.List.md#v-transpose) |
| `u16-to-f32` | `U16 -> F32` | [Data.Machine](Data.Machine.md#v-u16-to-f32) |
| `u16-to-f64` | `U16 -> F64` | [Data.Machine](Data.Machine.md#v-u16-to-f64) |
| `u16-to-i32` | `(x :: U16) -> refined (0 <= cinch-result && cinch-result <= 65535 && cinch-result == x) I32` | [Data.Machine](Data.Machine.md#v-u16-to-i32) |
| `u16-to-i64` | `(x :: U16) -> refined (0 <= cinch-result && cinch-result <= 65535 && cinch-result == x) I64` | [Data.Machine](Data.Machine.md#v-u16-to-i64) |
| `u16-to-isize` | `(a0 :: U16) -> refined (0 <= r && r <= 65535 && r == a0) Isize` | [Data.Machine](Data.Machine.md#v-u16-to-isize) |
| `u16-to-nat` | `U16 -> InRange 0 65535 Nat` | [Data.Machine](Data.Machine.md#v-u16-to-nat) |
| `u16-to-u16` | `U16 -> U16` | [Data.Machine](Data.Machine.md#v-u16-to-u16) |
| `u16-to-u32` | `(x :: U16) -> refined (0 <= cinch-result && cinch-result <= 65535 && cinch-result == x) U32` | [Data.Machine](Data.Machine.md#v-u16-to-u32) |
| `u16-to-u64` | `(x :: U16) -> refined (0 <= cinch-result && cinch-result <= 65535 && cinch-result == x) U64` | [Data.Machine](Data.Machine.md#v-u16-to-u64) |
| `u16-to-usize` | `(a0 :: U16) -> refined (0 <= r && r <= 65535 && r == a0) Usize` | [Data.Machine](Data.Machine.md#v-u16-to-usize) |
| `u32-to-char` | `U32 -> Option Char` | [Data.Char](Data.Char.md#v-u32-to-char) |
| `u32-to-f32` | `U32 -> F32` | [Data.Machine](Data.Machine.md#v-u32-to-f32) |
| `u32-to-f64` | `U32 -> F64` | [Data.Machine](Data.Machine.md#v-u32-to-f64) |
| `u32-to-i64` | `(x :: U32) -> refined (0 <= cinch-result && cinch-result <= 4294967295 && cinch-result == x) I64` | [Data.Machine](Data.Machine.md#v-u32-to-i64) |
| `u32-to-isize` | `(a0 :: U32) -> refined (0 <= r && r <= 4294967295 && r == a0) Isize` | [Data.Machine](Data.Machine.md#v-u32-to-isize) |
| `u32-to-nat` | `U32 -> InRange 0 4294967295 Nat` | [Data.Machine](Data.Machine.md#v-u32-to-nat) |
| `u32-to-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-u32-to-u32) |
| `u32-to-u64` | `(x :: U32) -> refined (0 <= cinch-result && cinch-result <= 4294967295 && cinch-result == x) U64` | [Data.Machine](Data.Machine.md#v-u32-to-u64) |
| `u32-to-usize` | `(a0 :: U32) -> refined (0 <= r && r <= 4294967295 && r == a0) Usize` | [Data.Machine](Data.Machine.md#v-u32-to-usize) |
| `u64-to-f32` | `U64 -> F32` | [Data.Machine](Data.Machine.md#v-u64-to-f32) |
| `u64-to-f64` | `U64 -> F64` | [Data.Machine](Data.Machine.md#v-u64-to-f64) |
| `u64-to-u64` | `U64 -> U64` | [Data.Machine](Data.Machine.md#v-u64-to-u64) |
| `u64-to-usize` | `(a0 :: U64) -> refined (r == a0) Usize` | [Data.Machine](Data.Machine.md#v-u64-to-usize) |
| `u8-to-f32` | `U8 -> F32` | [Data.Machine](Data.Machine.md#v-u8-to-f32) |
| `u8-to-f64` | `U8 -> F64` | [Data.Machine](Data.Machine.md#v-u8-to-f64) |
| `u8-to-i16` | `(x :: U8) -> refined (0 <= cinch-result && cinch-result <= 255 && cinch-result == x) I16` | [Data.Machine](Data.Machine.md#v-u8-to-i16) |
| `u8-to-i32` | `(x :: U8) -> refined (0 <= cinch-result && cinch-result <= 255 && cinch-result == x) I32` | [Data.Machine](Data.Machine.md#v-u8-to-i32) |
| `u8-to-i64` | `(x :: U8) -> refined (0 <= cinch-result && cinch-result <= 255 && cinch-result == x) I64` | [Data.Machine](Data.Machine.md#v-u8-to-i64) |
| `u8-to-isize` | `(a0 :: U8) -> refined (0 <= r && r <= 255 && r == a0) Isize` | [Data.Machine](Data.Machine.md#v-u8-to-isize) |
| `u8-to-nat` | `U8 -> InRange 0 255 Nat` | [Data.Machine](Data.Machine.md#v-u8-to-nat) |
| `u8-to-u16` | `(x :: U8) -> refined (0 <= cinch-result && cinch-result <= 255 && cinch-result == x) U16` | [Data.Machine](Data.Machine.md#v-u8-to-u16) |
| `u8-to-u32` | `(x :: U8) -> refined (0 <= cinch-result && cinch-result <= 255 && cinch-result == x) U32` | [Data.Machine](Data.Machine.md#v-u8-to-u32) |
| `u8-to-u64` | `(x :: U8) -> refined (0 <= cinch-result && cinch-result <= 255 && cinch-result == x) U64` | [Data.Machine](Data.Machine.md#v-u8-to-u64) |
| `u8-to-u8` | `U8 -> U8` | [Data.Machine](Data.Machine.md#v-u8-to-u8) |
| `u8-to-usize` | `(a0 :: U8) -> refined (0 <= r && r <= 255 && r == a0) Usize` | [Data.Machine](Data.Machine.md#v-u8-to-usize) |
| `uncurry` | `forall a b c. (a -> b -> c) -> (a, b) -> c` | [Function.Combinators](Function.Combinators.md#v-uncurry) |
| `unfold` | `forall a b. (b -> Option (a, b)) -> b -> List a` | [Data.List](Data.List.md#v-unfold) |
| `unpack` | `String -> List Char` | [Data.String](Data.String.md#v-unpack) |
| `unzip` | `forall a b. List (a, b) -> (List a, List b)` | [Data.List](Data.List.md#v-unzip) |
| `usize-to-f32` | `Usize -> F32` | [Data.Machine](Data.Machine.md#v-usize-to-f32) |
| `usize-to-f64` | `Usize -> F64` | [Data.Machine](Data.Machine.md#v-usize-to-f64) |
| `usize-to-u64` | `(a0 :: Usize) -> refined (r == a0) U64` | [Data.Machine](Data.Machine.md#v-usize-to-u64) |
| `usize-to-usize` | `Usize -> Usize` | [Data.Machine](Data.Machine.md#v-usize-to-usize) |
| `utf8-at` | `Bytes -> Nat -> Option (Char, Nat)` | [Data.String](Data.String.md#v-utf8-at) |
| `utf8-char` | `Nat -> Nat -> Option (Char, Nat)` | [Data.String](Data.String.md#v-utf8-char) |
| `utf8-cont` | `arg1 + arg2 <= (bytes-len arg0) => (arg0 :: Bytes) -> (arg1 :: Nat) -> (arg2 :: Nat) -> (arg3 :: Nat) -> Option Nat` | [Data.String](Data.String.md#v-utf8-cont) |
| `utf8-encode-into` | `arg1 + (utf8-encode-width arg2) <= (bytes-len arg0) => (arg0 :: Bytes) -> (arg1 :: Nat) -> (arg2 :: Nat) -> Nat` | [Data.String](Data.String.md#v-utf8-encode-into) |
| `utf8-encode-width` | `Nat -> InRange 1 4 Nat` | [Data.String](Data.String.md#v-utf8-encode-width) |
| `utf8-lead-payload` | `Nat -> Nat -> Nat` | [Data.String](Data.String.md#v-utf8-lead-payload) |
| `utf8-min` | `Nat -> Nat` | [Data.String](Data.String.md#v-utf8-min) |
| `utf8-valid-from` | `Bytes -> Nat -> Bool` | [Data.String](Data.String.md#v-utf8-valid-from) |
| `utf8-width` | `Nat -> Nat` | [Data.String](Data.String.md#v-utf8-width) |
| `vec-stream` | `forall a (n :: Nat) region. Ref region Vec n a -> Producer (Ref region Vec n a) a` | [Data.Producer](Data.Producer.md#v-vec-stream) |
| `wrapping-add-i16` | `I16 -> I16 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-add-i16) |
| `wrapping-add-i32` | `I32 -> I32 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-add-i32) |
| `wrapping-add-i64` | `I64 -> I64 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-add-i64) |
| `wrapping-add-i8` | `I8 -> I8 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-add-i8) |
| `wrapping-add-isize` | `Isize -> Isize -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-add-isize) |
| `wrapping-add-u16` | `U16 -> U16 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-add-u16) |
| `wrapping-add-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-add-u32) |
| `wrapping-add-u64` | `U64 -> U64 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-add-u64) |
| `wrapping-add-u8` | `U8 -> U8 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-add-u8) |
| `wrapping-add-usize` | `Usize -> Usize -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-add-usize) |
| `wrapping-i16-to-i16` | `I16 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-i16-to-i16) |
| `wrapping-i16-to-i32` | `I16 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-i16-to-i32) |
| `wrapping-i16-to-i64` | `I16 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-i16-to-i64) |
| `wrapping-i16-to-i8` | `I16 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-i16-to-i8) |
| `wrapping-i16-to-isize` | `I16 -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-i16-to-isize) |
| `wrapping-i16-to-u16` | `I16 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-i16-to-u16) |
| `wrapping-i16-to-u32` | `I16 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-i16-to-u32) |
| `wrapping-i16-to-u64` | `I16 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-i16-to-u64) |
| `wrapping-i16-to-u8` | `I16 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-i16-to-u8) |
| `wrapping-i16-to-usize` | `I16 -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-i16-to-usize) |
| `wrapping-i32-to-i16` | `I32 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-i32-to-i16) |
| `wrapping-i32-to-i32` | `I32 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-i32-to-i32) |
| `wrapping-i32-to-i64` | `I32 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-i32-to-i64) |
| `wrapping-i32-to-i8` | `I32 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-i32-to-i8) |
| `wrapping-i32-to-isize` | `I32 -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-i32-to-isize) |
| `wrapping-i32-to-u16` | `I32 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-i32-to-u16) |
| `wrapping-i32-to-u32` | `I32 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-i32-to-u32) |
| `wrapping-i32-to-u64` | `I32 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-i32-to-u64) |
| `wrapping-i32-to-u8` | `I32 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-i32-to-u8) |
| `wrapping-i32-to-usize` | `I32 -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-i32-to-usize) |
| `wrapping-i64-to-i16` | `I64 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-i64-to-i16) |
| `wrapping-i64-to-i32` | `I64 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-i64-to-i32) |
| `wrapping-i64-to-i64` | `I64 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-i64-to-i64) |
| `wrapping-i64-to-i8` | `I64 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-i64-to-i8) |
| `wrapping-i64-to-isize` | `I64 -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-i64-to-isize) |
| `wrapping-i64-to-u16` | `I64 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-i64-to-u16) |
| `wrapping-i64-to-u32` | `I64 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-i64-to-u32) |
| `wrapping-i64-to-u64` | `I64 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-i64-to-u64) |
| `wrapping-i64-to-u8` | `I64 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-i64-to-u8) |
| `wrapping-i64-to-usize` | `I64 -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-i64-to-usize) |
| `wrapping-i8-to-i16` | `I8 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-i8-to-i16) |
| `wrapping-i8-to-i32` | `I8 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-i8-to-i32) |
| `wrapping-i8-to-i64` | `I8 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-i8-to-i64) |
| `wrapping-i8-to-i8` | `I8 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-i8-to-i8) |
| `wrapping-i8-to-isize` | `I8 -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-i8-to-isize) |
| `wrapping-i8-to-u16` | `I8 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-i8-to-u16) |
| `wrapping-i8-to-u32` | `I8 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-i8-to-u32) |
| `wrapping-i8-to-u64` | `I8 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-i8-to-u64) |
| `wrapping-i8-to-u8` | `I8 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-i8-to-u8) |
| `wrapping-i8-to-usize` | `I8 -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-i8-to-usize) |
| `wrapping-isize-to-i16` | `Isize -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-isize-to-i16) |
| `wrapping-isize-to-i32` | `Isize -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-isize-to-i32) |
| `wrapping-isize-to-i64` | `Isize -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-isize-to-i64) |
| `wrapping-isize-to-i8` | `Isize -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-isize-to-i8) |
| `wrapping-isize-to-isize` | `Isize -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-isize-to-isize) |
| `wrapping-isize-to-u16` | `Isize -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-isize-to-u16) |
| `wrapping-isize-to-u32` | `Isize -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-isize-to-u32) |
| `wrapping-isize-to-u64` | `Isize -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-isize-to-u64) |
| `wrapping-isize-to-u8` | `Isize -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-isize-to-u8) |
| `wrapping-isize-to-usize` | `Isize -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-isize-to-usize) |
| `wrapping-mul-i16` | `I16 -> I16 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-mul-i16) |
| `wrapping-mul-i32` | `I32 -> I32 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-mul-i32) |
| `wrapping-mul-i64` | `I64 -> I64 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-mul-i64) |
| `wrapping-mul-i8` | `I8 -> I8 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-mul-i8) |
| `wrapping-mul-isize` | `Isize -> Isize -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-mul-isize) |
| `wrapping-mul-u16` | `U16 -> U16 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-mul-u16) |
| `wrapping-mul-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-mul-u32) |
| `wrapping-mul-u64` | `U64 -> U64 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-mul-u64) |
| `wrapping-mul-u8` | `U8 -> U8 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-mul-u8) |
| `wrapping-mul-usize` | `Usize -> Usize -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-mul-usize) |
| `wrapping-neg-i16` | `I16 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-neg-i16) |
| `wrapping-neg-i32` | `I32 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-neg-i32) |
| `wrapping-neg-i64` | `I64 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-neg-i64) |
| `wrapping-neg-i8` | `I8 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-neg-i8) |
| `wrapping-neg-isize` | `Isize -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-neg-isize) |
| `wrapping-neg-u16` | `U16 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-neg-u16) |
| `wrapping-neg-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-neg-u32) |
| `wrapping-neg-u64` | `U64 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-neg-u64) |
| `wrapping-neg-u8` | `U8 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-neg-u8) |
| `wrapping-neg-usize` | `Usize -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-neg-usize) |
| `wrapping-shl-i16` | `I16 -> U32 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-shl-i16) |
| `wrapping-shl-i32` | `I32 -> U32 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-shl-i32) |
| `wrapping-shl-i64` | `I64 -> U32 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-shl-i64) |
| `wrapping-shl-i8` | `I8 -> U32 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-shl-i8) |
| `wrapping-shl-isize` | `Isize -> U32 -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-shl-isize) |
| `wrapping-shl-u16` | `U16 -> U32 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-shl-u16) |
| `wrapping-shl-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-shl-u32) |
| `wrapping-shl-u64` | `U64 -> U32 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-shl-u64) |
| `wrapping-shl-u8` | `U8 -> U32 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-shl-u8) |
| `wrapping-shl-usize` | `Usize -> U32 -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-shl-usize) |
| `wrapping-shr-i16` | `I16 -> U32 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-shr-i16) |
| `wrapping-shr-i32` | `I32 -> U32 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-shr-i32) |
| `wrapping-shr-i64` | `I64 -> U32 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-shr-i64) |
| `wrapping-shr-i8` | `I8 -> U32 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-shr-i8) |
| `wrapping-shr-isize` | `Isize -> U32 -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-shr-isize) |
| `wrapping-shr-u16` | `U16 -> U32 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-shr-u16) |
| `wrapping-shr-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-shr-u32) |
| `wrapping-shr-u64` | `U64 -> U32 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-shr-u64) |
| `wrapping-shr-u8` | `U8 -> U32 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-shr-u8) |
| `wrapping-shr-usize` | `Usize -> U32 -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-shr-usize) |
| `wrapping-sub-i16` | `I16 -> I16 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-sub-i16) |
| `wrapping-sub-i32` | `I32 -> I32 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-sub-i32) |
| `wrapping-sub-i64` | `I64 -> I64 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-sub-i64) |
| `wrapping-sub-i8` | `I8 -> I8 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-sub-i8) |
| `wrapping-sub-isize` | `Isize -> Isize -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-sub-isize) |
| `wrapping-sub-u16` | `U16 -> U16 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-sub-u16) |
| `wrapping-sub-u32` | `U32 -> U32 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-sub-u32) |
| `wrapping-sub-u64` | `U64 -> U64 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-sub-u64) |
| `wrapping-sub-u8` | `U8 -> U8 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-sub-u8) |
| `wrapping-sub-usize` | `Usize -> Usize -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-sub-usize) |
| `wrapping-u16-to-i16` | `U16 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-u16-to-i16) |
| `wrapping-u16-to-i32` | `U16 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-u16-to-i32) |
| `wrapping-u16-to-i64` | `U16 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-u16-to-i64) |
| `wrapping-u16-to-i8` | `U16 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-u16-to-i8) |
| `wrapping-u16-to-isize` | `U16 -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-u16-to-isize) |
| `wrapping-u16-to-u16` | `U16 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-u16-to-u16) |
| `wrapping-u16-to-u32` | `U16 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-u16-to-u32) |
| `wrapping-u16-to-u64` | `U16 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-u16-to-u64) |
| `wrapping-u16-to-u8` | `U16 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-u16-to-u8) |
| `wrapping-u16-to-usize` | `U16 -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-u16-to-usize) |
| `wrapping-u32-to-i16` | `U32 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-u32-to-i16) |
| `wrapping-u32-to-i32` | `U32 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-u32-to-i32) |
| `wrapping-u32-to-i64` | `U32 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-u32-to-i64) |
| `wrapping-u32-to-i8` | `U32 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-u32-to-i8) |
| `wrapping-u32-to-isize` | `U32 -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-u32-to-isize) |
| `wrapping-u32-to-u16` | `U32 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-u32-to-u16) |
| `wrapping-u32-to-u32` | `U32 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-u32-to-u32) |
| `wrapping-u32-to-u64` | `U32 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-u32-to-u64) |
| `wrapping-u32-to-u8` | `U32 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-u32-to-u8) |
| `wrapping-u32-to-usize` | `U32 -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-u32-to-usize) |
| `wrapping-u64-to-i16` | `U64 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-u64-to-i16) |
| `wrapping-u64-to-i32` | `U64 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-u64-to-i32) |
| `wrapping-u64-to-i64` | `U64 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-u64-to-i64) |
| `wrapping-u64-to-i8` | `U64 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-u64-to-i8) |
| `wrapping-u64-to-isize` | `U64 -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-u64-to-isize) |
| `wrapping-u64-to-u16` | `U64 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-u64-to-u16) |
| `wrapping-u64-to-u32` | `U64 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-u64-to-u32) |
| `wrapping-u64-to-u64` | `U64 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-u64-to-u64) |
| `wrapping-u64-to-u8` | `U64 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-u64-to-u8) |
| `wrapping-u64-to-usize` | `U64 -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-u64-to-usize) |
| `wrapping-u8-to-i16` | `U8 -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-u8-to-i16) |
| `wrapping-u8-to-i32` | `U8 -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-u8-to-i32) |
| `wrapping-u8-to-i64` | `U8 -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-u8-to-i64) |
| `wrapping-u8-to-i8` | `U8 -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-u8-to-i8) |
| `wrapping-u8-to-isize` | `U8 -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-u8-to-isize) |
| `wrapping-u8-to-u16` | `U8 -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-u8-to-u16) |
| `wrapping-u8-to-u32` | `U8 -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-u8-to-u32) |
| `wrapping-u8-to-u64` | `U8 -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-u8-to-u64) |
| `wrapping-u8-to-u8` | `U8 -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-u8-to-u8) |
| `wrapping-u8-to-usize` | `U8 -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-u8-to-usize) |
| `wrapping-usize-to-i16` | `Usize -> I16` | [Data.Machine](Data.Machine.md#v-wrapping-usize-to-i16) |
| `wrapping-usize-to-i32` | `Usize -> I32` | [Data.Machine](Data.Machine.md#v-wrapping-usize-to-i32) |
| `wrapping-usize-to-i64` | `Usize -> I64` | [Data.Machine](Data.Machine.md#v-wrapping-usize-to-i64) |
| `wrapping-usize-to-i8` | `Usize -> I8` | [Data.Machine](Data.Machine.md#v-wrapping-usize-to-i8) |
| `wrapping-usize-to-isize` | `Usize -> Isize` | [Data.Machine](Data.Machine.md#v-wrapping-usize-to-isize) |
| `wrapping-usize-to-u16` | `Usize -> U16` | [Data.Machine](Data.Machine.md#v-wrapping-usize-to-u16) |
| `wrapping-usize-to-u32` | `Usize -> U32` | [Data.Machine](Data.Machine.md#v-wrapping-usize-to-u32) |
| `wrapping-usize-to-u64` | `Usize -> U64` | [Data.Machine](Data.Machine.md#v-wrapping-usize-to-u64) |
| `wrapping-usize-to-u8` | `Usize -> U8` | [Data.Machine](Data.Machine.md#v-wrapping-usize-to-u8) |
| `wrapping-usize-to-usize` | `Usize -> Usize` | [Data.Machine](Data.Machine.md#v-wrapping-usize-to-usize) |
| `xor` | `Bool -> Bool -> Bool` | [Data.Bool](Data.Bool.md#v-xor) |
| `zero-u32` | `U32` | [Data.Machine](Data.Machine.md#v-zero-u32) |
| `zip` | `forall a b. List a -> List b -> List (a, b)` | [Data.List](Data.List.md#v-zip) |
| `zip-with` | `forall a b c. (a -> b -> c) -> List a -> List b -> List c` | [Data.List](Data.List.md#v-zip-with) |
| `\|` | `forall a. using (bits :: Bits a) => a -> a -> a` | [Algebra.Bits](Algebra.Bits.md#v-op-7c) |
| `\|\|` | `Bool -> Bool -> Bool` | [Data.Bool](Data.Bool.md#v-op-7c7c) |

# Operator fixities

| Operator | Associativity | Precedence |
| --- | --- | --- |
| `%` | `infixl` | `9` |
| `&` | `infixl` | `6` |
| `&&` | `infixr` | `2` |
| `*` | `infixl` | `9` |
| `+` | `infixl` | `8` |
| `-` | `infixl` | `8` |
| `/` | `infixl` | `9` |
| `/=` | `infix` | `3` |
| `<` | `infix` | `3` |
| `<<` | `infixl` | `7` |
| `<=` | `infix` | `3` |
| `==` | `infix` | `3` |
| `>` | `infix` | `3` |
| `>=` | `infix` | `3` |
| `>>` | `infixl` | `7` |
| `^` | `infixl` | `5` |
| `\|` | `infixl` | `4` |
| `\|\|` | `infixr` | `1` |

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Prelude.cinch`](/library/source/Prelude/). Source SHA-256: `7c66ae8376fdb2aec7c267b7a2a164e60eb7ada7c3b41fe8414cad81c55502f7`.
