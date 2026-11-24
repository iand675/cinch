# Data.Ascii

ASCII-refined values and checked construction at text boundaries.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Data.Ascii</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.Ascii/">View source</a><a href="/library/search/?q=%2BData.Ascii">Search module</a><a href="/reference-sources/Data.Ascii.cinch" download="Data.Ascii.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.String](Data.String.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Ascii: the ASCII string as a refinement (refinement types
design; decision D151).

`type Ascii = refine (s :: String) with Ascii s`: `Ascii` IS `String`
at runtime (the witness is `Prop`-erased, decision D31). The
predicate `Ascii` decides text values (`String`, and `Bytes` for the
buffer-level twin); over anything else it is refused naming the
predicate. `String` wraps its `Bytes` backing (decision D88), so the
decision procedure reads the bytes directly.

The total constructor `to-ascii` discharges at runtime through the
`Dec`/`decide` world and returns `Option`; the transparent
constructor `Ascii s w` takes the string and its witness.

The instance below is the container's own `<>` (string
concatenation, the transfer rule): the concatenation of ASCII
strings is ASCII (preserved), and concatenation is associative;
both checked sample-wise by the law layer.

</details>

<details class="doc-synopsis" open><summary>Synopsis (2 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Ascii">Ascii</a><code class="syn-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">Ascii</strong> = <span class="tok-keyword">refine</span> (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-keyword">with</span> Ascii s</code></li><li><a class="syn-name" href="#v-to-ascii">to-ascii</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Ascii/#t-Ascii" title="Data.Ascii.Ascii">Ascii</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Ascii">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">type</span> <strong class="doc-decl-name tok-constructor">Ascii</strong> = <span class="tok-keyword">refine</span> (<span class="tok-variable">s</span> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a>) <span class="tok-keyword">with</span> Ascii s</code><span class="doc-sig-links"><a href="#t-Ascii" title="Permalink">#</a><a href="/library/source/Data.Ascii/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Data.Ascii/#t-Ascii" title="Data.Ascii.Ascii">Ascii</a></code> <span class="doc-inst-meta">implicit · <a href="/library/Data.Ascii/#instances">Data.Ascii</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-to-ascii">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">to-ascii</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Ascii/#t-Ascii" title="Data.Ascii.Ascii">Ascii</a></code><span class="doc-sig-links"><a href="#v-to-ascii" title="Permalink">#</a><a href="/library/source/Data.Ascii/#L28" title="View source at line 28">Source</a></span></div>
<div class="doc-body">

The total constructor (refinement types design: the total API).

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Semigroup_Ascii"><code><a class="doc-type-link tok-constructor" href="/library/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/library/Data.Ascii/#t-Ascii" title="Data.Ascii.Ascii">Ascii</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Ascii.Semigroup_Ascii</code></span></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Ascii.cinch`](/library/source/Data.Ascii/). Source SHA-256: `5ca0b2a99f6380073373fdff71fd0e3e95eb4575eb5ccc6aa56a407c8eb284b2`.
