# Data.Target

Target/platform and worker vocabulary used by the compiler-supported runtime and target examples.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Target</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Target/source/">View source</a><a href="/packages/search/?q=%2BData.Target">Search module</a><a href="/reference-sources/Data.Target.cinch" download="Data.Target.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Option](Data.Option.md), [Data.Producer](Data.Producer.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Target: target profiles and capability evidence.

Target profiles and vector shapes as marker types: the platform
chapter's `Target` kind and the compile-time lane width are
represented as types in the bootstrap (decision D58). The u32-sum
capability (`VecU32Target`) establishes that the sum intrinsics may
execute in the current environment (platform proofs and SIMD
specification section 2); its method is the width evidence a
generic proof consumes (section 6). `CapAvailable` is the
operational evidence that a capability is valid in the current
validity scope (platform section 2): the lexical region and thread
supplying it; a rewrite's `require` clause names it.

`simd-sum-wrap32` is the SIMD intrinsic (the scalar/SIMD pair): its
bootstrap lowering is the scalar loop, and the artifact manifest
records which lowering was checked (platform section 8).
`detect-vec-u32` is capability dispatch (platform section 7): the
bootstrap runtime performs no feature detection, so dispatch
conservatively selects the scalar path. The boundary is
replaceable: the optimizer never folds this call or the `case`
around it.

`SumProofs.scalar-to-simd` is the instruction law (platform section
5): the scalar model and the SIMD intrinsic compute the same sum.
Discharging it is a proof obligation of the target library; a
rewrite citing it records the assumption in the artifact manifest
(decision D55).

</details>

<details class="doc-synopsis" open><summary>Synopsis (11 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-AVX2_256">AVX2_256</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AVX2_256</strong></code></li><li><a class="syn-name" href="#t-CapAvailable">CapAvailable</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CapAvailable</strong> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code></li><li><a class="syn-name" href="#t-EightLanes">EightLanes</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">EightLanes</strong></code></li><li><a class="syn-name" href="#t-NEON64">NEON64</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">NEON64</strong></code></li><li><a class="syn-name" href="#t-ScalableShape">ScalableShape</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ScalableShape</strong></code></li><li><a class="syn-name" href="#c-SumProofs">SumProofs</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">SumProofs</strong></code></li><li class="syn-sub"><a class="syn-name" href="#m-SumProofs-scalar-to-simd">scalar-to-simd</a><code class="syn-sig"><strong class="tok-variable">scalar-to-simd</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span> (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-variable">region</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-SumProofs" title="Data.Target.SumProofs">SumProofs</a>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">vectors</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">xs</span> :: <a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Ref">Ref</a> <span class="tok-variable">region</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Vec" title="Vec">Vec</a> <span class="tok-variable">n</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Eq">Eq</a> (<span class="tok-constructor">scalar-sum-wrap32</span> @ <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> @ <span class="tok-variable">region</span> @ 1 (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <span class="tok-variable">n</span>) @ 4 <span class="tok-variable">region</span> <span class="tok-variable">xs</span>) (<span class="tok-constructor">simd-sum-wrap32</span> @ <span class="tok-variable">t</span> @ <span class="tok-variable">w</span> @ <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> @ <span class="tok-variable">region</span> @ 1 (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <span class="tok-variable">n</span>) @ 4 <span class="tok-variable">region</span> <span class="tok-variable">vectors</span> <span class="tok-variable">xs</span>)</code></li><li><a class="syn-name" href="#c-VecU32Target">VecU32Target</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">VecU32Target</strong> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-VecU32Target-positive-width">positive-width</a><code class="syn-sig"><strong class="tok-variable">positive-width</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-detect-vec-u32">detect-vec-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">detect-vec-u32</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#t-AVX2_256" title="Data.Target.AVX2_256">AVX2_256</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#t-EightLanes" title="Data.Target.EightLanes">EightLanes</a>)</code></li><li><a class="syn-name" href="#v-simd-sum-wrap32">simd-sum-wrap32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-sum-wrap32</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span> (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-variable">region</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Ref">Ref</a> <span class="tok-variable">region</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Vec" title="Vec">Vec</a> <span class="tok-variable">n</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-AVX2_256">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">AVX2_256</strong></code><span class="doc-sig-links"><a href="#t-AVX2_256" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Target profiles and vector shapes as marker types: the platform
chapter's `Target` kind and the compile-time lane width are
represented as types in the bootstrap (decision D58).

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-AVX2_256Tag"><code><strong class="tok-constructor">AVX2_256Tag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#t-AVX2_256" title="Data.Target.AVX2_256">AVX2_256</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-CapAvailable">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">CapAvailable</strong> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code><span class="doc-sig-links"><a href="#t-CapAvailable" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L69" title="View source at line 69">Source</a></span></div>
<div class="doc-body">

Operational evidence that a capability is valid in the current
validity scope (platform section 2): the lexical region and thread
supplying it. A rewrite's `require` clause names it.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Available"><code><strong class="tok-constructor">Available</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#t-CapAvailable" title="Data.Target.CapAvailable">CapAvailable</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-EightLanes">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">EightLanes</strong></code><span class="doc-sig-links"><a href="#t-EightLanes" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-EightLanesTag"><code><strong class="tok-constructor">EightLanesTag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#t-EightLanes" title="Data.Target.EightLanes">EightLanes</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-NEON64">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">NEON64</strong></code><span class="doc-sig-links"><a href="#t-NEON64" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-NEON64Tag"><code><strong class="tok-constructor">NEON64Tag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#t-NEON64" title="Data.Target.NEON64">NEON64</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ScalableShape">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ScalableShape</strong></code><span class="doc-sig-links"><a href="#t-ScalableShape" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">
<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ScalableShapeTag"><code><strong class="tok-constructor">ScalableShapeTag</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#t-ScalableShape" title="Data.Target.ScalableShape">ScalableShape</a></code></li></ul></div>
</div>
</article>

# Signatures

<article class="doc-entry" id="c-SumProofs">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">SumProofs</strong></code><span class="doc-sig-links"><a href="#c-SumProofs" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

The instruction law (platform section 5): the scalar model and the
SIMD intrinsic compute the same sum. Discharging it is a proof
obligation of the target library; a rewrite citing it records the
assumption in the artifact manifest (decision D55).

<div class="doc-children"><h4>Methods</h4><ul><li id="m-SumProofs-scalar-to-simd"><div class="doc-method-row"><code><strong class="tok-variable">scalar-to-simd</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span> (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-variable">region</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-SumProofs" title="Data.Target.SumProofs">SumProofs</a>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">vectors</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">xs</span> :: <a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Ref">Ref</a> <span class="tok-variable">region</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Vec" title="Vec">Vec</a> <span class="tok-variable">n</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Eq">Eq</a> (<span class="tok-constructor">scalar-sum-wrap32</span> @ <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> @ <span class="tok-variable">region</span> @ 1 (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <span class="tok-variable">n</span>) @ 4 <span class="tok-variable">region</span> <span class="tok-variable">xs</span>) (<span class="tok-constructor">simd-sum-wrap32</span> @ <span class="tok-variable">t</span> @ <span class="tok-variable">w</span> @ <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> @ <span class="tok-variable">region</span> @ 1 (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>, <span class="tok-variable">n</span>) @ 4 <span class="tok-variable">region</span> <span class="tok-variable">vectors</span> <span class="tok-variable">xs</span>)</code><span class="doc-sig-links"><a href="#m-SumProofs-scalar-to-simd" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L90">Source</a></span></div></li></ul></div>
</div>
</article>

<article class="doc-entry" id="c-VecU32Target">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">VecU32Target</strong> <span class="tok-variable">t</span> <span class="tok-variable">w</span></code><span class="doc-sig-links"><a href="#c-VecU32Target" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

The u32-sum capability: its construction establishes that the sum
intrinsics may execute in the current environment (platform proofs
and SIMD specification section 2). The method is the width evidence
a generic proof consumes (section 6).

<div class="doc-children"><h4>Methods</h4><ul><li id="m-VecU32Target-positive-width"><div class="doc-method-row"><code><strong class="tok-variable">positive-width</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#m-VecU32Target-positive-width" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L64">Source</a></span></div></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-detect-vec-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">detect-vec-u32</strong> :: <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#t-AVX2_256" title="Data.Target.AVX2_256">AVX2_256</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#t-EightLanes" title="Data.Target.EightLanes">EightLanes</a>)</code><span class="doc-sig-links"><a href="#v-detect-vec-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Capability dispatch (platform section 7): the bootstrap runtime
performs no feature detection, so dispatch conservatively selects
the scalar path. The boundary is replaceable: the optimizer never
folds this call or the `case` around it.

</div>
</article>

<article class="doc-entry" id="v-simd-sum-wrap32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-sum-wrap32</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">t</span> <span class="tok-variable">w</span> (<span class="tok-variable">n</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-variable">region</span>. <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Target/#c-VecU32Target" title="Data.Target.VecU32Target">VecU32Target</a> <span class="tok-variable">t</span> <span class="tok-variable">w</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/10-ownership-and-memory/" title="Ref">Ref</a> <span class="tok-variable">region</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Mutable/#t-Vec" title="Vec">Vec</a> <span class="tok-variable">n</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-simd-sum-wrap32" title="Permalink">#</a><a href="/packages/stdlib/Data.Target/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

The SIMD intrinsic (the scalar/SIMD pair): its bootstrap lowering is
the scalar loop, and the artifact manifest records which lowering
was checked (platform section 8).

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Target.cinch`](/packages/stdlib/Data.Target/source/). Source SHA-256: `a1d9ceb7eb29444f9a35ecf745234876432be54a81d615d2353b61a8cd1c411e`.
