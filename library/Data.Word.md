# Data.Word

Bit lengths, byte reversal, signed rotations and checked carry/borrow, wide multiply, divide and remainder operations over fixed and native-width words.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.Word</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Word/source/">View source</a><a href="/packages/search/?q=%2BData.Word">Search module</a><a href="/reference-sources/Data.Word.cinch" download="Data.Word.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Machine](Data.Machine.md), [Data.Result](Data.Result.md).

<details class="doc-notes"><summary>Module notes</summary>

Fixed-width word utilities and checked double-width unsigned arithmetic.
Carry and borrow values use Bool. Wide products return high then low words;
division returns quotient then remainder and names zero-divisor/overflow errors.
Bit reversal is supplied by Data.Machine. Byte reversal and wide
arithmetic use inline LLVM with exact double-width intermediates. Native-width
operations follow Usize's target width.

</details>

<details class="doc-synopsis"><summary>Synopsis (33 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-WordDivisionError">WordDivisionError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">WordDivisionError</strong></code></li><li><a class="syn-name" href="#v-add-carry-u32">add-carry-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-carry-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-add-carry-u64">add-carry-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-carry-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-add-carry-usize">add-carry-usize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">add-carry-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-bit-length-u16">bit-length-u16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bit-length-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-bit-length-u32">bit-length-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bit-length-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-bit-length-u64">bit-length-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bit-length-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-bit-length-u8">bit-length-u8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bit-length-u8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-bit-length-usize">bit-length-usize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bit-length-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-div-wide-proven-u32">div-wide-proven-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">div-wide-proven-u32</strong> :: (<span class="tok-variable">arg2</span> /= 0, <span class="tok-variable">arg0</span> &lt; <span class="tok-variable">arg2</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>)</code></li><li><a class="syn-name" href="#v-div-wide-proven-u64">div-wide-proven-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">div-wide-proven-u64</strong> :: (<span class="tok-variable">arg2</span> /= 0, <span class="tok-variable">arg0</span> &lt; <span class="tok-variable">arg2</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-div-wide-u32">div-wide-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">div-wide-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>)</code></li><li><a class="syn-name" href="#v-div-wide-u64">div-wide-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">div-wide-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-div-wide-usize">div-wide-usize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">div-wide-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>)</code></li><li><a class="syn-name" href="#v-mul-wide-u32">mul-wide-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mul-wide-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>)</code></li><li><a class="syn-name" href="#v-mul-wide-u64">mul-wide-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mul-wide-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-mul-wide-usize">mul-wide-usize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">mul-wide-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>)</code></li><li><a class="syn-name" href="#v-rem-wide-u32">rem-wide-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rem-wide-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-rem-wide-u64">rem-wide-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rem-wide-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-rem-wide-usize">rem-wide-usize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rem-wide-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a></code></li><li><a class="syn-name" href="#v-reverse-bytes-u16">reverse-bytes-u16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">reverse-bytes-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-reverse-bytes-u32">reverse-bytes-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">reverse-bytes-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-reverse-bytes-u64">reverse-bytes-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">reverse-bytes-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-reverse-bytes-usize">reverse-bytes-usize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">reverse-bytes-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a></code></li><li><a class="syn-name" href="#v-rotate-signed-u16">rotate-signed-u16</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rotate-signed-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code></li><li><a class="syn-name" href="#v-rotate-signed-u32">rotate-signed-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rotate-signed-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-rotate-signed-u64">rotate-signed-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rotate-signed-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-rotate-signed-u8">rotate-signed-u8</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rotate-signed-u8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-rotate-signed-usize">rotate-signed-usize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">rotate-signed-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a></code></li><li><a class="syn-name" href="#v-sub-borrow-u32">sub-borrow-u32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sub-borrow-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-sub-borrow-u64">sub-borrow-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sub-borrow-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-sub-borrow-usize">sub-borrow-usize</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sub-borrow-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code></li><li><a class="syn-name" href="#v-word-bits">word-bits</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">word-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-WordDivisionError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">WordDivisionError</strong></code><span class="doc-sig-links"><a href="#t-WordDivisionError" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L26" title="View source at line 26">Source</a></span></div>
<div class="doc-body">

An undefined unsigned division is an explicit result, never a machine trap.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-WordDivideByZero"><code><strong class="tok-constructor">WordDivideByZero</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a></code></li><li id="k-WordQuotientOverflow"><code><strong class="tok-constructor">WordQuotientOverflow</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-add-carry-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-carry-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-add-carry-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

Add two 32-bit words and a carry, returning the low word and the outgoing carry.

</div>
</article>

<article class="doc-entry" id="v-add-carry-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-carry-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-add-carry-u64" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

Add two 64-bit words and a carry, returning the low word and the outgoing carry.

</div>
</article>

<article class="doc-entry" id="v-add-carry-usize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">add-carry-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-add-carry-usize" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L173" title="View source at line 173">Source</a></span></div>
<div class="doc-body">

Add native words and a carry, returning the low word and outgoing carry.

</div>
</article>

<article class="doc-entry" id="v-bit-length-u16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bit-length-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-bit-length-u16" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

The significant bit count of a U16; zero has length zero.

</div>
</article>

<article class="doc-entry" id="v-bit-length-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bit-length-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-bit-length-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

The significant bit count of a U32; zero has length zero.

</div>
</article>

<article class="doc-entry" id="v-bit-length-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bit-length-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-bit-length-u64" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L72" title="View source at line 72">Source</a></span></div>
<div class="doc-body">

The significant bit count of a U64; zero has length zero.

</div>
</article>

<article class="doc-entry" id="v-bit-length-u8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bit-length-u8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-bit-length-u8" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

The significant bit count of a U8; zero has length zero.

</div>
</article>

<article class="doc-entry" id="v-bit-length-usize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bit-length-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-bit-length-usize" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

The significant bit count of a Usize; zero has length zero.

</div>
</article>

<article class="doc-entry" id="v-div-wide-proven-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">div-wide-proven-u32</strong> :: (<span class="tok-variable">arg2</span> /= 0, <span class="tok-variable">arg0</span> &lt; <span class="tok-variable">arg2</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>)</code><span class="doc-sig-links"><a href="#v-div-wide-proven-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L149" title="View source at line 149">Source</a></span></div>
<div class="doc-body">

Divide a double-width dividend when the caller proves a nonzero divisor
and a fitting quotient. The tuple result avoids allocating an error sum.

</div>
</article>

<article class="doc-entry" id="v-div-wide-proven-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">div-wide-proven-u64</strong> :: (<span class="tok-variable">arg2</span> /= 0, <span class="tok-variable">arg0</span> &lt; <span class="tok-variable">arg2</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-div-wide-proven-u64" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

Divide a double-width dividend under nonzero-divisor and quotient-fit proofs.
The tuple result permits unboxed worker returns for already validated inputs.

</div>
</article>

<article class="doc-entry" id="v-div-wide-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">div-wide-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>)</code><span class="doc-sig-links"><a href="#v-div-wide-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

Divide a high/low 64-bit dividend by a 32-bit divisor, with a checked one-word quotient.

</div>
</article>

<article class="doc-entry" id="v-div-wide-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">div-wide-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-div-wide-u64" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L140" title="View source at line 140">Source</a></span></div>
<div class="doc-body">

Divide a high/low 128-bit dividend by a 64-bit divisor, with a checked one-word quotient.

</div>
</article>

<article class="doc-entry" id="v-div-wide-usize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">div-wide-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>)</code><span class="doc-sig-links"><a href="#v-div-wide-usize" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L197" title="View source at line 197">Source</a></span></div>
<div class="doc-body">

Divide a high/low native double word, checking zero and quotient overflow.

</div>
</article>

<article class="doc-entry" id="v-mul-wide-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mul-wide-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>)</code><span class="doc-sig-links"><a href="#v-mul-wide-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L122" title="View source at line 122">Source</a></span></div>
<div class="doc-body">

The exact 64-bit product of two 32-bit words, returned high word first.

</div>
</article>

<article class="doc-entry" id="v-mul-wide-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mul-wide-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-mul-wide-u64" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

The exact 128-bit product of two 64-bit words, returned high word first.

</div>
</article>

<article class="doc-entry" id="v-mul-wide-usize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">mul-wide-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>)</code><span class="doc-sig-links"><a href="#v-mul-wide-usize" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L189" title="View source at line 189">Source</a></span></div>
<div class="doc-body">

The exact double-width product of native words, returned high word first.

</div>
</article>

<article class="doc-entry" id="v-rem-wide-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rem-wide-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-rem-wide-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

The remainder of a high/low 64-bit dividend. The high word need not be below the divisor.

</div>
</article>

<article class="doc-entry" id="v-rem-wide-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rem-wide-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-rem-wide-u64" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L166" title="View source at line 166">Source</a></span></div>
<div class="doc-body">

The remainder of a high/low 128-bit dividend. The high word need not be below the divisor.

</div>
</article>

<article class="doc-entry" id="v-rem-wide-usize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rem-wide-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Word/#t-WordDivisionError" title="Data.Word.WordDivisionError">WordDivisionError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a></code><span class="doc-sig-links"><a href="#v-rem-wide-usize" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L207" title="View source at line 207">Source</a></span></div>
<div class="doc-body">

The remainder of a high/low native double word, checking only for a zero divisor.

</div>
</article>

<article class="doc-entry" id="v-reverse-bytes-u16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">reverse-bytes-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-reverse-bytes-u16" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Reverse byte order within a U16; bit order within each byte is preserved.

</div>
</article>

<article class="doc-entry" id="v-reverse-bytes-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">reverse-bytes-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-reverse-bytes-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L64" title="View source at line 64">Source</a></span></div>
<div class="doc-body">

Reverse byte order within a U32; bit order within each byte is preserved.

</div>
</article>

<article class="doc-entry" id="v-reverse-bytes-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">reverse-bytes-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-reverse-bytes-u64" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

Reverse byte order within a U64; bit order within each byte is preserved.

</div>
</article>

<article class="doc-entry" id="v-reverse-bytes-usize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">reverse-bytes-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a></code><span class="doc-sig-links"><a href="#v-reverse-bytes-usize" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

Reverse byte order within a Usize; bit order within each byte is preserved.

</div>
</article>

<article class="doc-entry" id="v-rotate-signed-u16">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rotate-signed-u16</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code><span class="doc-sig-links"><a href="#v-rotate-signed-u16" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

Rotate a U16 left by a signed count; negative counts rotate right and counts wrap by the width.

</div>
</article>

<article class="doc-entry" id="v-rotate-signed-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rotate-signed-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-rotate-signed-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

Rotate a U32 left by a signed count; negative counts rotate right and counts wrap by the width.

</div>
</article>

<article class="doc-entry" id="v-rotate-signed-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rotate-signed-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-rotate-signed-u64" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

Rotate a U64 left by a signed count; negative counts rotate right and counts wrap by the width.

</div>
</article>

<article class="doc-entry" id="v-rotate-signed-u8">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rotate-signed-u8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-rotate-signed-u8" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

Rotate a U8 left by a signed count; negative counts rotate right and counts wrap by the width.

</div>
</article>

<article class="doc-entry" id="v-rotate-signed-usize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">rotate-signed-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a></code><span class="doc-sig-links"><a href="#v-rotate-signed-usize" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

Rotate a Usize left by a signed count; negative counts rotate right and counts wrap by the width.

</div>
</article>

<article class="doc-entry" id="v-sub-borrow-u32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sub-borrow-u32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-sub-borrow-u32" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

Subtract a 32-bit word and an incoming borrow, returning the low word and outgoing borrow.

</div>
</article>

<article class="doc-entry" id="v-sub-borrow-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sub-borrow-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-sub-borrow-u64" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

Subtract a 64-bit word and an incoming borrow, returning the low word and outgoing borrow.

</div>
</article>

<article class="doc-entry" id="v-sub-borrow-usize">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sub-borrow-usize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="Usize">Usize</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a>)</code><span class="doc-sig-links"><a href="#v-sub-borrow-usize" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

Subtract native words and a borrow, returning the low word and outgoing borrow.

</div>
</article>

<article class="doc-entry" id="v-word-bits">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">word-bits</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-word-bits" title="Permalink">#</a><a href="/packages/stdlib/Data.Word/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

The native unsigned word width, 32 or 64, determined by the target's Usize operations.

</div>
</article>
