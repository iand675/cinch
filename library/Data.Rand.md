# Data.Rand

Pseudorandom sources as values (PCG and ChaCha8) and their distributions: bounded integers, floats, permutations, ziggurat normal and exponential variates, and Zipf.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Rand</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Rand/source/">View source</a><a href="/packages/search/?q=%2BData.Rand">Search module</a><a href="/reference-sources/Data.Rand.cinch" download="Data.Rand.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Array](Data.Array.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.ChaCha8](Data.ChaCha8.md), [Data.Float](Data.Float.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md).

<details class="doc-notes"><summary>Module notes</summary>

Pseudorandom generation over explicit sources: the PCG and ChaCha8 generators, and every distribution a
program needs from them (uniform integers in a range, floats in [0, 1), permutations and shuffles, standard
normal and exponential variates, and Zipf-distributed integers).

A source is any value that yields a 64-bit draw together with the source to draw from next (the Source
signature); PCG and ChaCha8 are instances. Every function takes a source and returns its result with the next
source, so generation is deterministic and replayable, and different parts of a program can hold independent
streams. Seed sources from Effect.Random when unpredictability matters; ChaCha8 is the one to use when the
outputs must also be hard to predict from earlier outputs.

PCG is the 128-bit linear congruential generator with the DXSM ("double xorshift multiply") output function:
one 128-bit multiply-add per draw and the reference generator's constants and encoding. Bounded integers use
Lemire's multiply-and-reject method (an unbiased result from one 64x64-bit product, redrawing only when the
low half lands in the biased sliver), with a mask for powers of two; results are those of a 64-bit host.
Normal and exponential variates use the Marsaglia-Tsang ziggurat with 128 and 256 strips; the tables are
derived from the construction by tooling/generate_rand_kernels.py and read as constant vectors. Zipf variates
use rejection-inversion. Outputs match the reference generators draw for draw.

Cost: each draw returns a new source value, and the current compiler boxes it (a single-field wrapper, or a
pair holding a 128-bit word, is a small heap object), so a draw costs one allocation: about 25 ns per raw
PCG draw on amd64, where the reference's mutable generator takes about 2 ns. The arithmetic itself is one
128-bit multiply-add and the DXSM fold.

</details>

<details class="doc-synopsis"><summary>Synopsis (27 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-Pcg">Pcg</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Pcg</strong></code></li><li><a class="syn-name" href="#t-PcgError">PcgError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PcgError</strong></code></li><li><a class="syn-name" href="#t-Zipf">Zipf</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Zipf</strong></code></li><li><a class="syn-name" href="#c-Source">Source</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Source</strong> <span class="tok-variable">s</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-Source-source-uint64">source-uint64</a><code class="syn-sig"><strong class="tok-variable">source-uint64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-pcg-error-message">pcg-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pcg-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-PcgError" title="Data.Rand.PcgError">PcgError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pcg-marshal">pcg-marshal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pcg-marshal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-pcg-new">pcg-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pcg-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code></li><li><a class="syn-name" href="#v-pcg-seed">pcg-seed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pcg-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code></li><li><a class="syn-name" href="#v-pcg-uint64">pcg-uint64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pcg-uint64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a>)</code></li><li><a class="syn-name" href="#v-pcg-unmarshal">pcg-unmarshal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pcg-unmarshal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-PcgError" title="Data.Rand.PcgError">PcgError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code></li><li><a class="syn-name" href="#v-rand-exp-float64">rand-exp-float64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-exp-float64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-float32">rand-float32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-float32</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-float64">rand-float64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-float64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-int32">rand-int32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-int32</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-int32-n">rand-int32-n</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-int32-n</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-int64">rand-int64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-int64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-int64-n">rand-int64-n</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-int64-n</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-norm-float64">rand-norm-float64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-norm-float64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-perm">rand-perm</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-perm</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-shuffle">rand-shuffle</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-shuffle</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-uint32">rand-uint32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-uint32</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-uint32-n">rand-uint32-n</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-uint32-n</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-uint64">rand-uint64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-uint64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-rand-uint64-n">rand-uint64-n</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rand-uint64-n</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <span class="tok-variable">s</span>)</code></li><li><a class="syn-name" href="#v-zipf-new">zipf-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zipf-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Zipf" title="Data.Rand.Zipf">Zipf</a></code></li><li><a class="syn-name" href="#v-zipf-uint64">zipf-uint64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zipf-uint64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Zipf" title="Data.Rand.Zipf">Zipf</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <span class="tok-variable">s</span>)</code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-Pcg">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Pcg</strong></code><span class="doc-sig-links"><a href="#t-Pcg" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L53" title="View source at line 53">Source</a></span></div>
<div class="doc-body">

The state, high word first.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Pcg"><code><strong class="tok-constructor">Pcg</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Rand/#instances">Data.Rand</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-PcgError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">PcgError</strong></code><span class="doc-sig-links"><a href="#t-PcgError" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L56" title="View source at line 56">Source</a></span></div>
<div class="doc-body">

Why a PCG encoding was refused.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-PcgBadEncoding"><code><strong class="tok-constructor">PcgBadEncoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-PcgError" title="Data.Rand.PcgError">PcgError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-Zipf">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">Zipf</strong></code><span class="doc-sig-links"><a href="#t-Zipf" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L371" title="View source at line 371">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-Zipf"><code><strong class="tok-constructor">Zipf</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Zipf" title="Data.Rand.Zipf">Zipf</a></code></li></ul></div>
</div>
</article>

# Signatures

<article class="doc-entry" id="c-Source">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">Source</strong> <span class="tok-variable">s</span></code><span class="doc-sig-links"><a href="#c-Source" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

A source of uniformly distributed 64-bit values: a draw and the source to draw from next.

<div class="doc-children"><h4>Methods</h4><ul><li id="m-Source-source-uint64"><div class="doc-method-row"><code><strong class="tok-variable">source-uint64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#m-Source-source-uint64" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L48">Source</a></span></div><div class="doc-child-prose">The next uniform 64-bit value and the source after it.</div></li></ul></div>
<details class="doc-instances"><summary>Instances (2)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Rand/#instances">Data.Rand</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Rand/#instances">Data.Rand</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-pcg-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pcg-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-PcgError" title="Data.Rand.PcgError">PcgError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pcg-error-message" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L61" title="View source at line 61">Source</a></span></div>
<div class="doc-body">

A readable description of an error.

</div>
</article>

<article class="doc-entry" id="v-pcg-marshal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pcg-marshal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-pcg-marshal" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

The state encoding: "pcg:" and the two halves big-endian (20 bytes).

</div>
</article>

<article class="doc-entry" id="v-pcg-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pcg-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code><span class="doc-sig-links"><a href="#v-pcg-new" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

A PCG generator from two seed words (the state's high and low halves).

</div>
</article>

<article class="doc-entry" id="v-pcg-seed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pcg-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code><span class="doc-sig-links"><a href="#v-pcg-seed" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L118" title="View source at line 118">Source</a></span></div>
<div class="doc-body">

The generator reseeded with two words (the same as a new generator).

</div>
</article>

<article class="doc-entry" id="v-pcg-uint64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pcg-uint64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a>)</code><span class="doc-sig-links"><a href="#v-pcg-uint64" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L122" title="View source at line 122">Source</a></span></div>
<div class="doc-body">

The next 64-bit value and generator.

</div>
</article>

<article class="doc-entry" id="v-pcg-unmarshal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pcg-unmarshal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-PcgError" title="Data.Rand.PcgError">PcgError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code><span class="doc-sig-links"><a href="#v-pcg-unmarshal" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

A generator from its encoding.

</div>
</article>

<article class="doc-entry" id="v-rand-exp-float64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-exp-float64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-exp-float64" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L345" title="View source at line 345">Source</a></span></div>
<div class="doc-body">

A standard exponential variate (rate 1).

</div>
</article>

<article class="doc-entry" id="v-rand-float32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-float32</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-float32" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L233" title="View source at line 233">Source</a></span></div>
<div class="doc-body">

A uniform binary32 value in [0, 1) with 24 random bits.

</div>
</article>

<article class="doc-entry" id="v-rand-float64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-float64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-float64" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L229" title="View source at line 229">Source</a></span></div>
<div class="doc-body">

A uniform value in [0, 1) with 53 random bits.

</div>
</article>

<article class="doc-entry" id="v-rand-int32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-int32</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-int32" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L171" title="View source at line 171">Source</a></span></div>
<div class="doc-body">

A uniform non-negative 32-bit integer (31 random bits).

</div>
</article>

<article class="doc-entry" id="v-rand-int32-n">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-int32-n</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-int32-n" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L221" title="View source at line 221">Source</a></span></div>
<div class="doc-body">

A uniform 32-bit integer in [0, n), or None when n is not positive.

</div>
</article>

<article class="doc-entry" id="v-rand-int64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-int64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-int64" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L163" title="View source at line 163">Source</a></span></div>
<div class="doc-body">

A uniform non-negative 64-bit integer (63 random bits).

</div>
</article>

<article class="doc-entry" id="v-rand-int64-n">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-int64-n</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-int64-n" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

A uniform integer in [0, n), or None when n is not positive.

</div>
</article>

<article class="doc-entry" id="v-rand-norm-float64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-norm-float64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-norm-float64" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L310" title="View source at line 310">Source</a></span></div>
<div class="doc-body">

A standard normal variate (mean 0, standard deviation 1).

</div>
</article>

<article class="doc-entry" id="v-rand-perm">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-perm</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-perm" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L245" title="View source at line 245">Source</a></span></div>
<div class="doc-body">

A random permutation of 0 .. n-1.

</div>
</article>

<article class="doc-entry" id="v-rand-shuffle">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-shuffle</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-shuffle" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L249" title="View source at line 249">Source</a></span></div>
<div class="doc-body">

The list's elements in a uniformly random order (the same order rand-perm's indices give).

</div>
</article>

<article class="doc-entry" id="v-rand-uint32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-uint32</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-uint32" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

A uniform 32-bit value (the draw's high half).

</div>
</article>

<article class="doc-entry" id="v-rand-uint32-n">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-uint32-n</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-uint32-n" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L217" title="View source at line 217">Source</a></span></div>
<div class="doc-body">

A uniform 32-bit value in [0, n), or None when n is 0.

</div>
</article>

<article class="doc-entry" id="v-rand-uint64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-uint64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-uint64" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

A uniform 64-bit value.

</div>
</article>

<article class="doc-entry" id="v-rand-uint64-n">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rand-uint64-n</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-rand-uint64-n" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L209" title="View source at line 209">Source</a></span></div>
<div class="doc-body">

A uniform value in [0, n), or None when n is 0.

</div>
</article>

<article class="doc-entry" id="v-zipf-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zipf-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Zipf" title="Data.Rand.Zipf">Zipf</a></code><span class="doc-sig-links"><a href="#v-zipf-new" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L382" title="View source at line 382">Source</a></span></div>
<div class="doc-body">

The distribution for s > 1 and v >= 1 (None otherwise) over [0, imax].

</div>
</article>

<article class="doc-entry" id="v-zipf-uint64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zipf-uint64</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">s</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">src</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <span class="tok-variable">s</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Zipf" title="Data.Rand.Zipf">Zipf</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">s</span> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <span class="tok-variable">s</span>)</code><span class="doc-sig-links"><a href="#v-zipf-uint64" title="Permalink">#</a><a href="/packages/stdlib/Data.Rand/source/#L396" title="View source at line 396">Source</a></span></div>
<div class="doc-body">

A Zipf-distributed integer, by rejection-inversion.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Source_ChaCha8"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Rand.Source_ChaCha8</code></span><div class="doc-child-prose">ChaCha8 as a source.</div></li>
<li id="i-Source_Pcg"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#t-Pcg" title="Data.Rand.Pcg">Pcg</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Rand.Source_Pcg</code></span><div class="doc-child-prose">PCG as a source.</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Rand.cinch`](/packages/stdlib/Data.Rand/source/). Source SHA-256: `36b9fee50dbb6086e686f303683d130764dd167aec5cd48dfb2efe3682677c53`.
