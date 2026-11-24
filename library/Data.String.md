# Data.String

Immutable UTF-8 text with bounded-stack construction and joining, scalar/span decoding, SIMD scalar counting, slicing and comparison. Byte conversions preserve immutability.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.String</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.String/source/">View source</a><a href="/packages/search/?q=%2BData.String">Search module</a><a href="/reference-sources/Data.String.cinch" download="Data.String.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Arithmetic](Algebra.Arithmetic.md), [Algebra.Monoid](Algebra.Monoid.md), [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bytes](Data.Bytes.md), [Data.Char](Data.Char.md), [Data.Hash](Data.Hash.md), [Data.List](Data.List.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md).

<details class="doc-notes"><summary>Module notes</summary>

Data.String: runtime strings, UTF-8 over Bytes.

`String` is the runtime text type (decision D88). It is distinct
from the staging interpreter's compile-time string values, which
are the same type observed at compile time (D43's deferral
resolved). A `"..."` literal in a term is a runtime `String`: the
lexer's decoded bytes in one shared read-only data-section constant
(immutability makes the sharing semantics-transparent: every
evaluation references the same object and nothing allocates).

THE IMMUTABILITY CONTRACT: a `String` never changes and no API ever
exposes its backing mutably. Conversions copy on the way in
(`string-from-bytes`) and on the way out (`string-to-bytes`);
mutation exists only on plain `Bytes`. The representation
constructor and the backing reader are module-private, so once a
buffer is wrapped no alias can mutate it, and a literal's backing
(the read-only data-section constant itself) never reaches a write:
outside this module it cannot be named, and in it every use of a
backing is a read or the source of a copy.

The functions here are the REFERENCE SEMANTICS for UTF-8. The
runtime provides SIMD-accelerated implementations of the same rules
(`cinch_utf8_*` in `seed/runtime/src/string.rs`), tested to agree
with this code; the acceleration is a representation detail behind
the primitive operations chapter's justified bodies (D89).

Module scheme (design/base-library.md): the type's home carries its
instance family (the parked-instance deviation of D159 is over).

</details>

<details class="doc-synopsis"><summary>Synopsis (32 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-String">String</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">String</strong></code></li><li><a class="syn-name" href="#v-compare-bytes-from">compare-bytes-from</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">compare-bytes-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-copy-bytes-into">copy-bytes-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">copy-bytes-into</strong> :: (<span class="tok-variable">arg2</span> + <span class="tok-variable">arg4</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>), <span class="tok-variable">arg3</span> + <span class="tok-variable">arg4</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg1</span>)) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg3</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg4</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-empty-string">empty-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">empty-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-pack">pack</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">pack</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-byte-index-from">string-byte-index-from</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-byte-index-from</strong> :: <span class="tok-variable">arg1</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">arg1</span> &lt;= <span class="tok-variable">r</span> &amp;&amp; <span class="tok-variable">r</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>)) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-string-byte-length">string-byte-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-byte-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-string-char-at">string-char-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-char-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-string-char-at-from">string-char-at-from</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-char-at-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-string-clone">string-clone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-compare">string-compare</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-string-concat">string-concat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-concat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-concat-all">string-concat-all</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-concat-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-from-bytes">string-from-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-join">string-join</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-length">string-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-string-length-from">string-length-from</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-length-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-string-repeat">string-repeat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-repeat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-slice">string-slice</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-slice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-string-to-bytes">string-to-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-string-utf8-first">string-utf8-first</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-utf8-first</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-string-utf8-last">string-utf8-last</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">string-utf8-last</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-unpack">unpack</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">unpack</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code></li><li><a class="syn-name" href="#v-utf8-at">utf8-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-utf8-char">utf8-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code></li><li><a class="syn-name" href="#v-utf8-cont">utf8-cont</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-cont</strong> :: <span class="tok-variable">arg1</span> + <span class="tok-variable">arg2</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg3</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-encode-into">utf8-encode-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-encode-into</strong> :: <span class="tok-variable">arg1</span> + (<span class="tok-constructor">utf8-encode-width</span> <span class="tok-variable">arg2</span>) &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-encode-width">utf8-encode-width</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-encode-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 4 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-lead-payload">utf8-lead-payload</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-lead-payload</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-min">utf8-min</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-min</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-utf8-valid-from">utf8-valid-from</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-valid-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-utf8-width">utf8-width</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">utf8-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-String">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">String</strong></code><span class="doc-sig-links"><a href="#t-String" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

The representation constructor: module-private (the immutability
contract: no API hands out a string's backing). A `StrBuf` wraps a
buffer that is either fresh (`bytes-new` filled here) or a copy,
never one a caller retains.

<details class="doc-instances"><summary>Instances (8)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Compare/#c-Compare" title="Data.Compare.Compare">Compare</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Compare/#instances">Data.Compare</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Compare/#c-Default" title="Data.Compare.Default">Default</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Compare/#instances">Data.Compare</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.String/#instances">Data.String</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Show/#c-Show" title="Text.Show.Show">Show</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Text.Show/#instances">Text.Show</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-compare-bytes-from">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">compare-bytes-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-compare-bytes-from" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L450" title="View source at line 450">Source</a></span></div>
<div class="doc-body">

Byte-wise comparison; UTF-8 preserves scalar value order under
byte-wise lexicographic order, so this is also code point order.

</div>
</article>

<article class="doc-entry" id="v-copy-bytes-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">copy-bytes-into</strong> :: (<span class="tok-variable">arg2</span> + <span class="tok-variable">arg4</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>), <span class="tok-variable">arg3</span> + <span class="tok-variable">arg4</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg1</span>)) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg3</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg4</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-copy-bytes-into" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L268" title="View source at line 268">Source</a></span></div>
<div class="doc-body">

Concatenation (O(n + m) copies). The range copy is the runtime's
`memmove` (`bytes-copy-range`, with its two range obligations): a
loop here boxed a `Nat` for every index step.

</div>
</article>

<article class="doc-entry" id="v-empty-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">empty-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-empty-string" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L174" title="View source at line 174">Source</a></span></div>
</article>

<article class="doc-entry" id="v-pack">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">pack</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-pack" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L406" title="View source at line 406">Source</a></span></div>
<div class="doc-body">

Encode a list of Unicode scalar values into one fresh immutable UTF-8 backing.
A tail size pass and a checked tail encoding pass keep traversal stack bounded.

</div>
</article>

<article class="doc-entry" id="v-string-byte-index-from">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-byte-index-from</strong> :: <span class="tok-variable">arg1</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <span class="tok-variable">refined</span> (<span class="tok-variable">arg1</span> &lt;= <span class="tok-variable">r</span> &amp;&amp; <span class="tok-variable">r</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>)) <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-string-byte-index-from" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L382" title="View source at line 382">Source</a></span></div>
<div class="doc-body">

Byte offset of code point `n` from offset `i`; the byte length when
`n` reaches past the end. The offset never moves backwards and never
passes the end, and the result says so. A sequence whose width runs
past the end is malformed, as `utf8-at` decides; the step restates
that end test, so it answers `i` as a malformed sequence does.

</div>
</article>

<article class="doc-entry" id="v-string-byte-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-byte-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-string-byte-length" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L177" title="View source at line 177">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-char-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-char-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-string-char-at" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L228" title="View source at line 228">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-char-at-from">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-char-at-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-string-char-at-from" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L220" title="View source at line 220">Source</a></span></div>
<div class="doc-body">

The scalar value at code point index `n`, by scan (O(n)).

</div>
</article>

<article class="doc-entry" id="v-string-clone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-clone</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-clone" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Copy a string into independent immutable UTF-8 storage, including empty input.
The mutable backing remains private and cannot alias any caller-owned byte buffer.

</div>
</article>

<article class="doc-entry" id="v-string-compare">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-compare</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-string-compare" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L462" title="View source at line 462">Source</a></span></div>
<div class="doc-body">

Compares the two backings in place: `string-to-bytes` would copy
both operands first, and the lexer's keyword table compares every
identifier against dozens of literals.

</div>
</article>

<article class="doc-entry" id="v-string-concat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-concat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-concat" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L271" title="View source at line 271">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-concat-all">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-concat-all</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-concat-all" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L283" title="View source at line 283">Source</a></span></div>
<div class="doc-body">

Concatenate every string in list order into one fresh immutable backing.
A tail-recursive size pass and a checked bulk-copy pass use bounded traversal
stack, including lists of empty strings. Concatenation preserves valid UTF-8.

</div>
</article>

<article class="doc-entry" id="v-string-from-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-from-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L168" title="View source at line 168">Source</a></span></div>
</article>

<article class="doc-entry" id="v-string-join">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-join</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-join" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L341" title="View source at line 341">Source</a></span></div>
<div class="doc-body">

Join valid UTF-8 pieces with separators into one fresh immutable backing.
Empty pieces contribute adjacent separators; an empty list produces empty text.

</div>
</article>

<article class="doc-entry" id="v-string-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-length</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-string-length" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L213" title="View source at line 213">Source</a></span></div>
<div class="doc-body">

Count Unicode scalars in sealed-valid UTF-8 using packed leading-byte classification.
Private SIMD/SWAR blocks count non-continuation bytes; exact scalar fringes never overread.

</div>
</article>

<article class="doc-entry" id="v-string-length-from">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-length-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-string-length-from" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L203" title="View source at line 203">Source</a></span></div>
<div class="doc-body">

Length in Unicode scalar values (O(n) scan).

</div>
</article>

<article class="doc-entry" id="v-string-repeat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-repeat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-repeat" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L351" title="View source at line 351">Source</a></span></div>
<div class="doc-body">

Repeat a string the requested natural count into one fresh immutable backing.
A complete first copy seeds checked prefix doubling, giving logarithmically
many bulk-copy steps and linear copied bytes. Empty results need no fill loop.

</div>
</article>

<article class="doc-entry" id="v-string-slice">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-slice</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-string-slice" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L394" title="View source at line 394">Source</a></span></div>
<div class="doc-body">

The code point range [start, end); indices past the end clamp and a
reversed range yields the empty string (the span's subtraction
takes its `start < end` evidence from the test).

</div>
</article>

<article class="doc-entry" id="v-string-to-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-to-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-string-to-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

A copy of the backing (the conversion out copies: the caller may
mutate its copy freely and the `String` never changes).

</div>
</article>

<article class="doc-entry" id="v-string-utf8-first">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-utf8-first</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-string-utf8-first" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L182" title="View source at line 182">Source</a></span></div>
<div class="doc-body">

Observe the first scalar and its encoded byte width without copying private backing.
Empty text has no scalar; every present width is between one and four bytes.

</div>
</article>

<article class="doc-entry" id="v-string-utf8-last">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">string-utf8-last</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-string-utf8-last" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L198" title="View source at line 198">Source</a></span></div>
<div class="doc-body">

Observe the final scalar and its encoded byte width without scanning or copying the prefix.
Empty text has no scalar; reverse lookup reads at most four bytes before decoding it.

</div>
</article>

<article class="doc-entry" id="v-unpack">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">unpack</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code><span class="doc-sig-links"><a href="#v-unpack" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L435" title="View source at line 435">Source</a></span></div>
</article>

<article class="doc-entry" id="v-utf8-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-at</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-utf8-at" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L138" title="View source at line 138">Source</a></span></div>
</article>

<article class="doc-entry" id="v-utf8-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>)</code><span class="doc-sig-links"><a href="#v-utf8-char" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L151" title="View source at line 151">Source</a></span></div>
</article>

<article class="doc-entry" id="v-utf8-cont">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-cont</strong> :: <span class="tok-variable">arg1</span> + <span class="tok-variable">arg2</span> &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg3</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-cont" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L119" title="View source at line 119">Source</a></span></div>
<div class="doc-body">

The `k` continuation bytes from `i`: the caller has checked that
they lie inside the buffer (`i + k <= bytes-len b`), so each read
is in bounds and the obligation steps with the index.

</div>
</article>

<article class="doc-entry" id="v-utf8-encode-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-encode-into</strong> :: <span class="tok-variable">arg1</span> + (<span class="tok-constructor">utf8-encode-width</span> <span class="tok-variable">arg2</span>) &lt;= (<span class="tok-constructor">bytes-len</span> <span class="tok-variable">arg0</span>) <span class="tok-operator">=&gt;</span> (<span class="tok-variable">arg0</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg1</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> (<span class="tok-variable">arg2</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-encode-into" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L248" title="View source at line 248">Source</a></span></div>
<div class="doc-body">

Encode one scalar value at `pos` (bytes are raw values; division is
the `nat-div`/`nat-rem` function family). The caller proves the
encoding fits (`pos + utf8-encode-width cp <= bytes-len b`, a
buffer sized with `utf8-encode-width`), and the branches test that
same width, so each one knows how many bytes it may write. Every
byte is taken modulo its field width, which is the same number for
every value its width admits (`cp < 128` in the first branch, `cp /
64 < 32` in the second, `cp / 4096 < 16` in the third, and `cp <
2^22` in the last, past which the unmasked lead byte did not fit a
byte at all).

</div>
</article>

<article class="doc-entry" id="v-utf8-encode-width">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-encode-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 1 4 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-encode-width" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L231" title="View source at line 231">Source</a></span></div>
</article>

<article class="doc-entry" id="v-utf8-lead-payload">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-lead-payload</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-lead-payload" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

The lead byte's payload bits: `x - 192`, `x - 224`, `x - 240` for
the lead byte of a two-, three- and four-byte sequence, written as
the low bits, which is the same number for every lead byte
`utf8-width` admits and needs no `x >= 192` evidence.

</div>
</article>

<article class="doc-entry" id="v-utf8-min">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-min</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-min" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L109" title="View source at line 109">Source</a></span></div>
</article>

<article class="doc-entry" id="v-utf8-valid-from">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-valid-from</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-utf8-valid-from" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

Validate UTF-8 from a byte boundary. Complete ASCII words are checked
with SWAR; non-ASCII sequences and tails retain scalar validation.

</div>
</article>

<article class="doc-entry" id="v-utf8-width">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">utf8-width</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-utf8-width" title="Permalink">#</a><a href="/packages/stdlib/Data.String/source/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

UTF-8 decoding rules: a lead byte gives the width; continuation
bytes are 10xxxxxx; overlong encodings, surrogates, and values above
0x10FFFF are malformed.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_String"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">Implicit · <code>Data.String.Eq_String</code></span></li>
<li id="i-Hash_String"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Hash/#c-Hash" title="Data.Hash.Hash">Hash</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">Implicit · <code>Data.String.Hash_String</code></span><div class="doc-child-prose">The digest of the UTF-8 backing through the Bytes instance (Data.Bytes owns the byte fold, so the two instances agree by construction instead of keeping two copies of one algorithm).</div></li>
<li id="i-Monoid_String"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Monoid/#c-Monoid" title="Algebra.Monoid.Monoid">Monoid</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">Implicit · <code>Data.String.Monoid_String</code></span><div class="doc-child-prose">empty-string is the identity (moved from Algebra.Monoid); mconcat overrides the class's balanced fold with the one-pass concatenation.</div></li>
<li id="i-Ord_String"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Ord/#c-Ord" title="Relation.Ord.Ord">Ord</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">Implicit · <code>Data.String.Ord_String</code></span></li>
<li id="i-Semigroup_String"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Semigroup/#c-Semigroup" title="Algebra.Semigroup.Semigroup">Semigroup</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">Implicit · <code>Data.String.Semigroup_String</code></span><div class="doc-child-prose">Strings under concatenation over the UTF-8 Bytes backing (moved from Algebra.Semigroup at the prelude split; the instance lives with its type).</div></li>
</ul>
