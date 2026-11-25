# Prim.Wide

Leaf inline-LLVM 128-bit SIMD lane primitives and target-guarded hardware AES/CLMUL instructions (AES-NI, ARMv8 AES, PCLMULQDQ, PMULL) with portable fallbacks, re-exported through Data.Wide and shared directly by Data.String.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Prim.Wide</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Prim.Wide/source/">View source</a><a href="/packages/search/?q=%2BPrim.Wide">Search module</a><a href="/reference-sources/Prim.Wide.cinch" download="Prim.Wide.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

<details class="doc-notes"><summary>Module notes</summary>

Portable 128-bit SIMD lane primitives and hardware AES/CLMUL leaf instructions over U128.
Every lane operation treats a U128 as a packed vector with lane 0 in the
least-significant bits. Target-guarded hardware bodies (AES-NI, ARMv8 AES,
PCLMULQDQ, PMULL) carry explicit `when` guards and portable fallbacks, and
are re-exported through Data.Wide.

</details>

<details class="doc-synopsis"><summary>Synopsis (51 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#c-ArmAes">ArmAes</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">ArmAes</strong></code></li><li><a class="syn-name" href="#v-aes-dec-last">aes-dec-last</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-dec-last</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-aes-dec-round">aes-dec-round</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-dec-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-aes-enc-last">aes-enc-last</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-enc-last</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-aes-enc-round">aes-enc-round</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">aes-enc-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-bit-and-u128">bit-and-u128</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bit-and-u128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-clmul-u64">clmul-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">clmul-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-cpu-has-aes">cpu-has-aes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cpu-has-aes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-cpu-has-clmul">cpu-has-clmul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">cpu-has-clmul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-simd-bitselect">simd-bitselect</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-bitselect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-i16x8-ashr">simd-i16x8-ashr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-i16x8-ashr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-i16x8-cmplt">simd-i16x8-cmplt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-i16x8-cmplt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-i16x8-smax">simd-i16x8-smax</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-i16x8-smax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-i16x8-smin">simd-i16x8-smin</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-i16x8-smin</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-i32x4-ashr">simd-i32x4-ashr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-i32x4-ashr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-i32x4-smax">simd-i32x4-smax</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-i32x4-smax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-i32x4-smin">simd-i32x4-smin</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-i32x4-smin</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u16x8-add">simd-u16x8-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u16x8-cmpeq">simd-u16x8-cmpeq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-cmpeq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u16x8-cmplt">simd-u16x8-cmplt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-cmplt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u16x8-mul">simd-u16x8-mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u16x8-shl">simd-u16x8-shl</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-shl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u16x8-shr">simd-u16x8-shr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-shr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u16x8-splat">simd-u16x8-splat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-splat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u16x8-sub">simd-u16x8-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-add">simd-u32x4-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-cmpeq">simd-u32x4-cmpeq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-cmpeq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-cmplt">simd-u32x4-cmplt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-cmplt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-mul">simd-u32x4-mul</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-pack">simd-u32x4-pack</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-pack</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-rotl">simd-u32x4-rotl</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-rotl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-rotr">simd-u32x4-rotr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-rotr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-shl">simd-u32x4-shl</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-shl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-shr">simd-u32x4-shr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-shr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-splat">simd-u32x4-splat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-splat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u32x4-sub">simd-u32x4-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u64x2-add">simd-u64x2-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u64x2-pack">simd-u64x2-pack</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-pack</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u64x2-rotl">simd-u64x2-rotl</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-rotl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u64x2-rotr">simd-u64x2-rotr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-rotr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u64x2-shl">simd-u64x2-shl</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-shl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u64x2-shr">simd-u64x2-shr</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-shr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u64x2-splat">simd-u64x2-splat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-splat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u64x2-sub">simd-u64x2-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u8x16-add">simd-u8x16-add</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u8x16-cmpeq">simd-u8x16-cmpeq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-cmpeq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u8x16-cmpgt">simd-u8x16-cmpgt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-cmpgt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u8x16-cmplt">simd-u8x16-cmplt</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-cmplt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u8x16-movemask">simd-u8x16-movemask</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-movemask</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-simd-u8x16-splat">simd-u8x16-splat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-splat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li><li><a class="syn-name" href="#v-simd-u8x16-sub">simd-u8x16-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code></li></ul>
</details>

# Signatures

<article class="doc-entry" id="c-ArmAes">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">ArmAes</strong></code><span class="doc-sig-links"><a href="#c-ArmAes" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L19" title="View source at line 19">Source</a></span></div>
<div class="doc-body">

Target-profile evidence that the ARMv8 AES and PMULL instructions may execute.
Only profiles that guarantee the cryptography extension should name this capability.

</div>
</article>

# Values

<article class="doc-entry" id="v-aes-dec-last">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-dec-last</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-aes-dec-last" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L559" title="View source at line 559">Source</a></span></div>
<div class="doc-body">

Final hardware AES equivalent-inverse decryption round (InvShiftRows, InvSubBytes, AddRoundKey); callers must guard with cpu-has-aes.

</div>
</article>

<article class="doc-entry" id="v-aes-dec-round">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-dec-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-aes-dec-round" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L538" title="View source at line 538">Source</a></span></div>
<div class="doc-body">

One hardware AES equivalent-inverse decryption round (InvShiftRows, InvSubBytes, InvMixColumns, AddRoundKey); callers must guard with cpu-has-aes.

</div>
</article>

<article class="doc-entry" id="v-aes-enc-last">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-enc-last</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-aes-enc-last" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L517" title="View source at line 517">Source</a></span></div>
<div class="doc-body">

Final hardware AES encryption round (SubBytes, ShiftRows, AddRoundKey); callers must guard with cpu-has-aes.

</div>
</article>

<article class="doc-entry" id="v-aes-enc-round">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">aes-enc-round</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-aes-enc-round" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L496" title="View source at line 496">Source</a></span></div>
<div class="doc-body">

One hardware AES encryption round (SubBytes, ShiftRows, MixColumns, AddRoundKey); callers must guard with cpu-has-aes.

</div>
</article>

<article class="doc-entry" id="v-bit-and-u128">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bit-and-u128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-bit-and-u128" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L24" title="View source at line 24">Source</a></span></div>
<div class="doc-body">

Bitwise AND over two U128 values.

</div>
</article>

<article class="doc-entry" id="v-clmul-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">clmul-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-clmul-u64" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L580" title="View source at line 580">Source</a></span></div>
<div class="doc-body">

64x64-bit hardware carry-less multiplication into U128; callers must guard with cpu-has-clmul.

</div>
</article>

<article class="doc-entry" id="v-cpu-has-aes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cpu-has-aes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-cpu-has-aes" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L600" title="View source at line 600">Source</a></span></div>
<div class="doc-body">

Report non-zero when hardware AES round instructions may execute on the target processor.

</div>
</article>

<article class="doc-entry" id="v-cpu-has-clmul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">cpu-has-clmul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-cpu-has-clmul" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L615" title="View source at line 615">Source</a></span></div>
<div class="doc-body">

Report non-zero when hardware carry-less multiplication instructions may execute on the target processor.

</div>
</article>

<article class="doc-entry" id="v-simd-bitselect">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-bitselect</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-bitselect" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Bitwise select across 128 bits: (mask & when-one) | (~mask & when-zero).

</div>
</article>

<article class="doc-entry" id="v-simd-i16x8-ashr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-i16x8-ashr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-i16x8-ashr" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L178" title="View source at line 178">Source</a></span></div>
<div class="doc-body">

Lane-wise arithmetic right shift (masked modulo 16) on 8 signed I16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-i16x8-cmplt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-i16x8-cmplt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-i16x8-cmplt" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

Lane-wise signed less-than mask on 8 I16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-i16x8-smax">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-i16x8-smax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-i16x8-smax" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L234" title="View source at line 234">Source</a></span></div>
<div class="doc-body">

Lane-wise signed maximum on 8 I16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-i16x8-smin">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-i16x8-smin</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-i16x8-smin" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L224" title="View source at line 224">Source</a></span></div>
<div class="doc-body">

Lane-wise signed minimum on 8 I16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-i32x4-ashr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-i32x4-ashr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-i32x4-ashr" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L307" title="View source at line 307">Source</a></span></div>
<div class="doc-body">

Lane-wise arithmetic right shift (masked modulo 32) on 4 signed I32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-i32x4-smax">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-i32x4-smax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-i32x4-smax" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L351" title="View source at line 351">Source</a></span></div>
<div class="doc-body">

Lane-wise signed maximum on 4 I32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-i32x4-smin">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-i32x4-smin</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-i32x4-smin" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L341" title="View source at line 341">Source</a></span></div>
<div class="doc-body">

Lane-wise signed minimum on 4 I32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u16x8-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u16x8-add" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L122" title="View source at line 122">Source</a></span></div>
<div class="doc-body">

Lane-wise wrapping addition on 8 U16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u16x8-cmpeq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-cmpeq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u16x8-cmpeq" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L191" title="View source at line 191">Source</a></span></div>
<div class="doc-body">

Lane-wise equality mask on 8 U16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u16x8-cmplt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-cmplt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u16x8-cmplt" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L202" title="View source at line 202">Source</a></span></div>
<div class="doc-body">

Lane-wise unsigned less-than mask on 8 U16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u16x8-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u16x8-mul" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L142" title="View source at line 142">Source</a></span></div>
<div class="doc-body">

Lane-wise wrapping multiplication on 8 U16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u16x8-shl">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-shl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u16x8-shl" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L152" title="View source at line 152">Source</a></span></div>
<div class="doc-body">

Lane-wise left shift (masked modulo 16) on 8 U16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u16x8-shr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-shr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u16x8-shr" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Lane-wise logical right shift (masked modulo 16) on 8 U16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u16x8-splat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-splat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u16x8-splat" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Broadcast one U16 into all 8 16-bit lanes of a U128.

</div>
</article>

<article class="doc-entry" id="v-simd-u16x8-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u16x8-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u16x8-sub" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Lane-wise wrapping subtraction on 8 U16 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-add" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L253" title="View source at line 253">Source</a></span></div>
<div class="doc-body">

Lane-wise wrapping addition on 4 U32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-cmpeq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-cmpeq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-cmpeq" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L361" title="View source at line 361">Source</a></span></div>
<div class="doc-body">

Lane-wise equality mask on 4 U32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-cmplt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-cmplt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-cmplt" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L372" title="View source at line 372">Source</a></span></div>
<div class="doc-body">

Lane-wise unsigned less-than mask on 4 U32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-mul">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-mul</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-mul" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L273" title="View source at line 273">Source</a></span></div>
<div class="doc-body">

Lane-wise wrapping multiplication on 4 U32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-pack">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-pack</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-pack" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L383" title="View source at line 383">Source</a></span></div>
<div class="doc-body">

Pack four U32 lanes into a U128 with lane 0 in the least-significant 32 bits.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-rotl">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-rotl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-rotl" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L319" title="View source at line 319">Source</a></span></div>
<div class="doc-body">

Lane-wise left rotation on 4 U32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-rotr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-rotr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-rotr" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L330" title="View source at line 330">Source</a></span></div>
<div class="doc-body">

Lane-wise right rotation on 4 U32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-shl">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-shl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-shl" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L283" title="View source at line 283">Source</a></span></div>
<div class="doc-body">

Lane-wise left shift (masked modulo 32) on 4 U32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-shr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-shr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-shr" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L295" title="View source at line 295">Source</a></span></div>
<div class="doc-body">

Lane-wise logical right shift (masked modulo 32) on 4 U32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-splat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-splat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-splat" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L244" title="View source at line 244">Source</a></span></div>
<div class="doc-body">

Broadcast one U32 into all 4 32-bit lanes of a U128.

</div>
</article>

<article class="doc-entry" id="v-simd-u32x4-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u32x4-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u32x4-sub" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L263" title="View source at line 263">Source</a></span></div>
<div class="doc-body">

Lane-wise wrapping subtraction on 4 U32 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u64x2-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u64x2-add" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L411" title="View source at line 411">Source</a></span></div>
<div class="doc-body">

Lane-wise wrapping addition on 2 U64 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u64x2-pack">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-pack</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u64x2-pack" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L481" title="View source at line 481">Source</a></span></div>
<div class="doc-body">

Pack two U64 lanes into a U128 with lane 0 in the least-significant 64 bits.

</div>
</article>

<article class="doc-entry" id="v-simd-u64x2-rotl">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-rotl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u64x2-rotl" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L457" title="View source at line 457">Source</a></span></div>
<div class="doc-body">

Lane-wise left rotation on 2 U64 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u64x2-rotr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-rotr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u64x2-rotr" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L469" title="View source at line 469">Source</a></span></div>
<div class="doc-body">

Lane-wise right rotation on 2 U64 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u64x2-shl">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-shl</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u64x2-shl" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L431" title="View source at line 431">Source</a></span></div>
<div class="doc-body">

Lane-wise left shift (masked modulo 64) on 2 U64 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u64x2-shr">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-shr</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u64x2-shr" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L444" title="View source at line 444">Source</a></span></div>
<div class="doc-body">

Lane-wise logical right shift (masked modulo 64) on 2 U64 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u64x2-splat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-splat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u64x2-splat" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L402" title="View source at line 402">Source</a></span></div>
<div class="doc-body">

Broadcast one U64 into both 64-bit lanes of a U128.

</div>
</article>

<article class="doc-entry" id="v-simd-u64x2-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u64x2-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u64x2-sub" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L421" title="View source at line 421">Source</a></span></div>
<div class="doc-body">

Lane-wise wrapping subtraction on 2 U64 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u8x16-add">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-add</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u8x16-add" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Lane-wise wrapping addition on 16 U8 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u8x16-cmpeq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-cmpeq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u8x16-cmpeq" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Lane-wise equality mask (0xFF per matching lane, 0x00 otherwise) on 16 U8 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u8x16-cmpgt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-cmpgt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u8x16-cmpgt" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

Lane-wise unsigned greater-than mask on 16 U8 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u8x16-cmplt">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-cmplt</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u8x16-cmplt" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

Lane-wise unsigned less-than mask on 16 U8 lanes.

</div>
</article>

<article class="doc-entry" id="v-simd-u8x16-movemask">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-movemask</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-simd-u8x16-movemask" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L103" title="View source at line 103">Source</a></span></div>
<div class="doc-body">

Extract the sign bit of each of the 16 byte lanes into the low 16 bits of a U32.

</div>
</article>

<article class="doc-entry" id="v-simd-u8x16-splat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-splat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u8x16-splat" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Broadcast one U8 into all 16 byte lanes of a U128.

</div>
</article>

<article class="doc-entry" id="v-simd-u8x16-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">simd-u8x16-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code><span class="doc-sig-links"><a href="#v-simd-u8x16-sub" title="Permalink">#</a><a href="/packages/stdlib/Prim.Wide/source/#L60" title="View source at line 60">Source</a></span></div>
<div class="doc-body">

Lane-wise wrapping subtraction on 16 U8 lanes.

</div>
</article>
