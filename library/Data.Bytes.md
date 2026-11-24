# Data.Bytes

Mutable byte buffers with length, construction, copy, and bounds/range-aware access operations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">Embedded</span> <code>import Data.Bytes</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Bytes/source/">View source</a><a href="/packages/search/?q=%2BData.Bytes">Search module</a><a href="/reference-sources/Data.Bytes.cinch" download="Data.Bytes.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Data.Hash](Data.Hash.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.Bytes: boxed byte buffers.

`Bytes` is a core runtime type: indices and lengths are `Nat`, and a
byte is an `InRange 0 255 Nat`. Every operation is total by
construction (design/no-faults.md): a read or write takes evidence
that its index is below `bytes-len b`, a range copy evidence that both
ranges end inside their buffers, a read's result carries its range and
a write's value must prove it. A buffer's length is fixed when it is
made, so a fact over `bytes-len b` survives writes to `b`, and
`bytes-new n` gives `bytes-len out == n` to the name it is bound to.
A length no allocation can hold is the heap `Exhausted` exit.

Module scheme (design/base-library.md): the type's home carries its
instance family: `FromList` (`[1, 2, 3]` at `Bytes` fills a buffer)
over `U8` elements, so a literal's numerals are range-checked as
bytes at compile time, and the key pair `Eq`/`Hash` (a byte slice is
a plausible key: a compiler's names arrive as the lexer's bytes).

</details>

<details class="doc-synopsis" open><summary>Synopsis (10 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-bytes-compare">bytes-compare</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-bytes-copy-range">bytes-copy-range</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-copy-range</strong> :: (<span class="tok-variable">arg2</span> + <span class="tok-variable">arg4</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>), <span class="tok-variable">arg3</span> + <span class="tok-variable">arg4</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg1</span>)) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg3</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg4</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-bytes-data">bytes-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-bytes-from-list">bytes-from-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-from-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-bytes-from-u8s">bytes-from-u8s</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-from-u8s</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-bytes-get">bytes-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-get</strong> :: <span class="tok-variable">arg1</span> &lt; (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-bytes-len">bytes-len</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9223372036854775807 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-bytes-new">bytes-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-new</strong> :: ((n :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) -&gt; (refined ((b.bytesLen cinch-result) == n) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>))</code></li><li><a class="syn-name" href="#v-bytes-set">bytes-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-set</strong> :: <span class="tok-variable">arg1</span> &lt; (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-sha256-bytes">sha256-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">sha256-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-bytes-compare">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-bytes-compare" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

The lexicographic order of two buffers' contents as the raw -1/0/1
immediate (the `nat-cmp` convention), read in place: no copy and
no allocation.

</div>
</article>

<article class="doc-entry" id="v-bytes-copy-range">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-copy-range</strong> :: (<span class="tok-variable">arg2</span> + <span class="tok-variable">arg4</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>), <span class="tok-variable">arg3</span> + <span class="tok-variable">arg4</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg1</span>)) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg3</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg4</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-bytes-copy-range" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

`bytes-copy-range dst src di si n`: the `n` bytes of `src` from `si`
into `dst` at `di`, as one runtime `memmove` (the two may be the
same buffer and overlap). A copy loop written here would box a `Nat`
for every index step, so every bulk copy goes through this.

</div>
</article>

<article class="doc-entry" id="v-bytes-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-bytes-data" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

`bytes-data b`: the address of the buffer's payload as a machine
word. This is how a buffer reaches the operating system: a `prim`
body or a `foreign import ccall` takes the address and the length
separately, exactly as the C interface does. The buffer never moves
and the address is valid while the handle is live, so a caller that
wants a C string NUL-terminates it itself (a fresh buffer one byte
longer, the bytes copied in, the last byte zeroed).

</div>
</article>

<article class="doc-entry" id="v-bytes-from-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-from-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-bytes-from-list" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes/source/#L72" title="View source at line 72">Source</a></span></div>
<div class="doc-body">

Copy typed byte values in list order into one fresh mutable buffer.
A tail count pass and checked tail stores keep construction stack bounded.

</div>
</article>

<article class="doc-entry" id="v-bytes-from-u8s">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-from-u8s</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-bytes-from-u8s" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

Copy machine byte values into one fresh buffer without recursive unwinding.

</div>
</article>

<article class="doc-entry" id="v-bytes-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-get</strong> :: <span class="tok-variable">arg1</span> &lt; (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-bytes-get" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes/source/#L41" title="View source at line 41">Source</a></span></div>
</article>

<article class="doc-entry" id="v-bytes-len">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-len</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 9223372036854775807 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-bytes-len" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

`Bytes`: boxed byte buffers. `bytes-len` comes first: the other
signatures name it. A length is an `i64` in the runtime, so it is at
most `2^63 - 1` and every index below it converts to `I64`. The
index obligation is `nat-sub`'s shape: a loop's own guard (`if i <
bytes-len b`), a fresh buffer's length or an enclosing `using (i <=
bytes-len b)` supplies it. A string literal's read-only backing is
never one of these buffers: it lives inside a `String`, and every
conversion out of a `String` copies.

</div>
</article>

<article class="doc-entry" id="v-bytes-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-new</strong> :: ((n :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) -&gt; (refined ((b.bytesLen cinch-result) == n) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>))</code><span class="doc-sig-links"><a href="#v-bytes-new" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes/source/#L40" title="View source at line 40">Source</a></span></div>
</article>

<article class="doc-entry" id="v-bytes-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-set</strong> :: <span class="tok-variable">arg1</span> &lt; (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 255 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-bytes-set" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes/source/#L42" title="View source at line 42">Source</a></span></div>
</article>

<article class="doc-entry" id="v-sha256-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">sha256-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-sha256-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes/source/#L68" title="View source at line 68">Source</a></span></div>
<div class="doc-body">

SHA-256 (FIPS 180-4) at content-addressing strength over a
buffer (the P31 content identity; `seed/runtime/src/text.rs`,
the `cinch.runtime.textSha256` intrinsic, nm-verified there): the
digest as a fresh 32-byte buffer.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_Bytes"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Bytes.Eq_Bytes</code></span><div class="doc-child-prose">Byte-for-byte equality over the backing: the in-place compare answers equal when the lengths agree and every byte agrees, reading the common prefix a word at a time.</div></li>
<li id="i-FromList_Bytes"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.FromList/#c-FromList" title="Data.FromList.FromList">FromList</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Bytes.FromList_Bytes</code></span></li>
<li id="i-Hash_Bytes"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code> <span class="doc-inst-meta">Implicit · <code>Data.Bytes.Hash_Bytes</code></span><div class="doc-child-prose">The buffer's digest (instances live with their type).</div></li>
</ul>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Bytes.cinch`](/packages/stdlib/Data.Bytes/source/). Source SHA-256: `24bb9f3ca7c5420b3abde122e05762f25adaa72df41d27557d22fc69370585da`.
