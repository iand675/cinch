# Data.MapHash

Fast keyed 64-bit non-cryptographic hash with persistent streaming state, one-shot byte/string fast paths and MapHashable typed comparable hashing.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Data.MapHash</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.MapHash/source/">View source</a><a href="/packages/search/?q=%2BData.MapHash">Search module</a><a href="/reference-sources/Data.MapHash.cinch" download="Data.MapHash.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md), [Relation.Eq](Relation.Eq.md).

<details class="doc-notes"><summary>Module notes</summary>

Fast, keyed, non-cryptographic 64-bit hashing for hash tables, byte streams and comparable values.

Each MapSeed carries a 128-bit secret key that selects an independent hash function.
Construct deterministic seeds with maphash-seed-from-u64, maphash-seed-from-u128 or
maphash-seed-from-bytes, or draw unpredictable per-table seeds from operating-system
entropy through Effect.MapHash.maphash-make-seed.

MapHash is an immutable persistent streaming state: retaining any intermediate value
acts as an O(1) snapshot without copying or aliasing mutable buffers. The computed
digest depends only on the seed and the sequence of written bytes, never on how writes
are split across maphash-write, maphash-write-string and maphash-write-byte. One-shot
maphash-bytes and maphash-string produce the exact same digest as streaming writes.

Typed values implement MapHashable using prefix-free canonical encodings so composite
values such as pairs, options and lists avoid boundary collisions, and floating-point
instances normalize negative zero to positive zero. Because MapHash is pure, NaN values
hash deterministically to a canonical quiet-NaN bit pattern.

These functions are intended for hash tables and in-memory indexing; they are not
cryptographic hashes.

</details>

<details class="doc-synopsis"><summary>Synopsis (23 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MapHash">MapHash</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapHash</strong></code></li><li><a class="syn-name" href="#t-MapSeed">MapSeed</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapSeed</strong></code></li><li><a class="syn-name" href="#c-MapHashable">MapHashable</a><code class="syn-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">MapHashable</strong> <span class="tok-variable">a</span></code></li><li class="syn-sub"><a class="syn-name" href="#m-MapHashable-maphash-write-value">maphash-write-value</a><code class="syn-sig"><strong class="tok-variable">maphash-write-value</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code></li><li><a class="syn-name" href="#v-maphash-block-size">maphash-block-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-maphash-bytes">maphash-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-maphash-comparable">maphash-comparable</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-comparable</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">mh</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-maphash-new">maphash-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code></li><li><a class="syn-name" href="#v-maphash-reset">maphash-reset</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code></li><li><a class="syn-name" href="#v-maphash-seed">maphash-seed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code></li><li><a class="syn-name" href="#v-maphash-seed-bytes">maphash-seed-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-seed-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-maphash-seed-from-bytes">maphash-seed-from-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-seed-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code></li><li><a class="syn-name" href="#v-maphash-seed-from-u128">maphash-seed-from-u128</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-seed-from-u128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code></li><li><a class="syn-name" href="#v-maphash-seed-from-u64">maphash-seed-from-u64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-seed-from-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code></li><li><a class="syn-name" href="#v-maphash-set-seed">maphash-set-seed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-set-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code></li><li><a class="syn-name" href="#v-maphash-size">maphash-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-maphash-string">maphash-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-maphash-sum">maphash-sum</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-maphash-sum64">maphash-sum64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-sum64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-maphash-write">maphash-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code></li><li><a class="syn-name" href="#v-maphash-write-byte">maphash-write-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-write-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code></li><li><a class="syn-name" href="#v-maphash-write-comparable">maphash-write-comparable</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-write-comparable</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">mh</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code></li><li><a class="syn-name" href="#v-maphash-write-string">maphash-write-string</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">maphash-write-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MapHash">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapHash</strong></code><span class="doc-sig-links"><a href="#t-MapHash" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Immutable streaming hash state holding its seed, 128-bit accumulator, 0..7-byte tail and total length.

</div>
</article>

<article class="doc-entry" id="t-MapSeed">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapSeed</strong></code><span class="doc-sig-links"><a href="#t-MapSeed" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L111" title="View source at line 111">Source</a></span></div>
<div class="doc-body">

A 128-bit key selecting a specific MapHash function.
Equal seeds produce identical digests for identical inputs; distinct seeds produce independent digests.

<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li></ul></details>
</div>
</article>

# Signatures

<article class="doc-entry" id="c-MapHashable">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">signature</span> <strong class="doc-decl-name tok-constructor">MapHashable</strong> <span class="tok-variable">a</span></code><span class="doc-sig-links"><a href="#c-MapHashable" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L380" title="View source at line 380">Source</a></span></div>
<div class="doc-body">

Types that admit canonical, prefix-free hashing into a MapHash stream.

<div class="doc-children"><h4>Methods</h4><ul><li id="m-MapHashable-maphash-write-value"><div class="doc-method-row"><code><strong class="tok-variable">maphash-write-value</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">dict</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code><span class="doc-sig-links"><a href="#m-MapHashable-maphash-write-value" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L382">Source</a></span></div><div class="doc-child-prose">Feed a canonical prefix-free representation of a value into a MapHash state.</div></li></ul></div>
<details class="doc-instances"><summary>Instances (21)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I16">I16</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I8">I8</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)</code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.MapHash/#instances">Data.MapHash</a></span></li></ul></details>
</div>
</article>

# Values

<article class="doc-entry" id="v-maphash-block-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-block-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-maphash-block-size" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

Preferred streaming write block size in bytes (128 bytes).

</div>
</article>

<article class="doc-entry" id="v-maphash-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-maphash-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L314" title="View source at line 314">Source</a></span></div>
<div class="doc-body">

Hash a byte buffer in one pass with the given seed, matching streaming maphash-write and maphash-sum64.

</div>
</article>

<article class="doc-entry" id="v-maphash-comparable">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-comparable</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">mh</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-maphash-comparable" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L389" title="View source at line 389">Source</a></span></div>
<div class="doc-body">

Compute the seeded 64-bit digest of a comparable value.

</div>
</article>

<article class="doc-entry" id="v-maphash-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code><span class="doc-sig-links"><a href="#v-maphash-new" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L170" title="View source at line 170">Source</a></span></div>
<div class="doc-body">

Create a new empty MapHash initialized with the given seed.

</div>
</article>

<article class="doc-entry" id="v-maphash-reset">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-reset</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code><span class="doc-sig-links"><a href="#v-maphash-reset" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Discard all previously written bytes while keeping the same seed.

</div>
</article>

<article class="doc-entry" id="v-maphash-seed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code><span class="doc-sig-links"><a href="#v-maphash-seed" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L178" title="View source at line 178">Source</a></span></div>
<div class="doc-body">

Return the MapSeed governing this MapHash.

</div>
</article>

<article class="doc-entry" id="v-maphash-seed-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-seed-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-maphash-seed-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L135" title="View source at line 135">Source</a></span></div>
<div class="doc-body">

Encode a MapSeed as a fresh 16-byte big-endian buffer.

</div>
</article>

<article class="doc-entry" id="v-maphash-seed-from-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-seed-from-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code><span class="doc-sig-links"><a href="#v-maphash-seed-from-bytes" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L144" title="View source at line 144">Source</a></span></div>
<div class="doc-body">

Decode a MapSeed from a 16-byte big-endian buffer, returning None for any other length.

</div>
</article>

<article class="doc-entry" id="v-maphash-seed-from-u128">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-seed-from-u128</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code><span class="doc-sig-links"><a href="#v-maphash-seed-from-u128" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L131" title="View source at line 131">Source</a></span></div>
<div class="doc-body">

Construct a MapSeed directly from a 128-bit unsigned key.

</div>
</article>

<article class="doc-entry" id="v-maphash-seed-from-u64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-seed-from-u64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code><span class="doc-sig-links"><a href="#v-maphash-seed-from-u64" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

Construct a 128-bit MapSeed deterministically from a 64-bit word.

</div>
</article>

<article class="doc-entry" id="v-maphash-set-seed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-set-seed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code><span class="doc-sig-links"><a href="#v-maphash-set-seed" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

Replace the seed and discard any previously written bytes.

</div>
</article>

<article class="doc-entry" id="v-maphash-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-maphash-size" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Digest width in bytes (8 bytes for a 64-bit digest).

</div>
</article>

<article class="doc-entry" id="v-maphash-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-maphash-string" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L322" title="View source at line 322">Source</a></span></div>
<div class="doc-body">

Hash a UTF-8 string in one pass with the given seed, matching maphash-write-string and maphash-sum64.

</div>
</article>

<article class="doc-entry" id="v-maphash-sum">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-sum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-maphash-sum" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L283" title="View source at line 283">Source</a></span></div>
<div class="doc-body">

Append the 8-byte big-endian digest of h to prefix, returning a fresh buffer.

</div>
</article>

<article class="doc-entry" id="v-maphash-sum64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-sum64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-maphash-sum64" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L277" title="View source at line 277">Source</a></span></div>
<div class="doc-body">

Return the current 64-bit digest of h without altering h.

</div>
</article>

<article class="doc-entry" id="v-maphash-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code><span class="doc-sig-links"><a href="#v-maphash-write" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L248" title="View source at line 248">Source</a></span></div>
<div class="doc-body">

Add all bytes of b to the stream hashed by h, returning the updated MapHash.

</div>
</article>

<article class="doc-entry" id="v-maphash-write-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-write-byte</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code><span class="doc-sig-links"><a href="#v-maphash-write-byte" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L261" title="View source at line 261">Source</a></span></div>
<div class="doc-body">

Add one byte to the stream hashed by h, returning the updated MapHash.

</div>
</article>

<article class="doc-entry" id="v-maphash-write-comparable">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-write-comparable</strong> :: <span class="tok-keyword">forall</span> <span class="tok-variable">a</span>. <span class="tok-keyword">using</span> (<span class="tok-variable">mh</span> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <span class="tok-variable">a</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <span class="tok-variable">a</span> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code><span class="doc-sig-links"><a href="#v-maphash-write-comparable" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L385" title="View source at line 385">Source</a></span></div>
<div class="doc-body">

Feed a comparable value into a MapHash stream.

</div>
</article>

<article class="doc-entry" id="v-maphash-write-string">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">maphash-write-string</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapHash" title="Data.MapHash.MapHash">MapHash</a></code><span class="doc-sig-links"><a href="#v-maphash-write-string" title="Permalink">#</a><a href="/packages/stdlib/Data.MapHash/source/#L257" title="View source at line 257">Source</a></span></div>
<div class="doc-body">

Add the UTF-8 bytes of s to the stream hashed by h, returning the updated MapHash.

</div>
</article>

# Instances

Exported checked instances declared in this module. For law checking, see [rewrites and optimization](../reference/12-rewrites-and-optimization.md).

<ul class="doc-module-instances">
<li id="i-Eq_MapSeed"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Relation.Eq/#c-Eq" title="Relation.Eq.Eq">Eq</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#t-MapSeed" title="Data.MapHash.MapSeed">MapSeed</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.Eq_MapSeed</code></span><div class="doc-child-prose">Equality on 128-bit hash seeds.</div></li>
<li id="i-MapHashable_Bool"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_Bool</code></span><div class="doc-child-prose">Hash a boolean as one byte (0 for False, 1 for True).</div></li>
<li id="i-MapHashable_Bytes"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_Bytes</code></span><div class="doc-child-prose">Hash a byte buffer framed by its 64-bit byte length so adjacent buffers cannot collide.</div></li>
<li id="i-MapHashable_Char"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Char/" title="Char">Char</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_Char</code></span><div class="doc-child-prose">Hash a Unicode scalar value by its 32-bit code point.</div></li>
<li id="i-MapHashable_F32"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F32">F32</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_F32</code></span><div class="doc-child-prose">Hash a 32-bit float, normalizing negative zero to positive zero and NaN to a canonical quiet NaN.</div></li>
<li id="i-MapHashable_F64"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="F64">F64</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_F64</code></span><div class="doc-child-prose">Hash a 64-bit float, normalizing negative zero to positive zero and NaN to a canonical quiet NaN.</div></li>
<li id="i-MapHashable_I128"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="I128">I128</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_I128</code></span><div class="doc-child-prose">Hash a signed 128-bit integer as sixteen little-endian two's-complement bytes.</div></li>
<li id="i-MapHashable_I16"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I16">I16</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_I16</code></span><div class="doc-child-prose">Hash a signed 16-bit integer as two little-endian two's-complement bytes.</div></li>
<li id="i-MapHashable_I32"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_I32</code></span><div class="doc-child-prose">Hash a signed 32-bit integer as four little-endian two's-complement bytes.</div></li>
<li id="i-MapHashable_I64"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_I64</code></span><div class="doc-child-prose">Hash a signed 64-bit integer as eight little-endian two's-complement bytes.</div></li>
<li id="i-MapHashable_I8"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I8">I8</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_I8</code></span><div class="doc-child-prose">Hash a signed 8-bit integer by its two's-complement byte.</div></li>
<li id="i-MapHashable_Int"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_Int</code></span><div class="doc-child-prose">Hash an arbitrary-precision signed integer with a sign byte followed by its magnitude limbs.</div></li>
<li id="i-MapHashable_List"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_List</code></span><div class="doc-child-prose">Hash a list in a single tail-recursive pass using prefix-free element tags.</div></li>
<li id="i-MapHashable_Nat"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_Nat</code></span><div class="doc-child-prose">Hash an arbitrary-precision natural number using a self-delimiting limb encoding.</div></li>
<li id="i-MapHashable_Option"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <span class="tok-variable">a</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_Option</code></span><div class="doc-child-prose">Hash an optional value with a presence tag byte before any contained value.</div></li>
<li id="i-MapHashable_Pair"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> (<span class="tok-variable">a</span>, <span class="tok-variable">b</span>)</code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_Pair</code></span><div class="doc-child-prose">Hash a pair by hashing its first and second components in order.</div></li>
<li id="i-MapHashable_String"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_String</code></span><div class="doc-child-prose">Hash a string framed by its 64-bit UTF-8 byte length so adjacent strings cannot collide.</div></li>
<li id="i-MapHashable_U128"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_U128</code></span><div class="doc-child-prose">Hash an unsigned 128-bit integer as sixteen little-endian bytes.</div></li>
<li id="i-MapHashable_U16"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U16">U16</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_U16</code></span><div class="doc-child-prose">Hash an unsigned 16-bit integer as two little-endian bytes.</div></li>
<li id="i-MapHashable_U32"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_U32</code></span><div class="doc-child-prose">Hash an unsigned 32-bit integer as four little-endian bytes.</div></li>
<li id="i-MapHashable_U64"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_U64</code></span><div class="doc-child-prose">Hash an unsigned 64-bit integer as eight little-endian bytes.</div></li>
<li id="i-MapHashable_U8"><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.MapHash/#c-MapHashable" title="Data.MapHash.MapHashable">MapHashable</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a></code> <span class="doc-inst-meta">Implicit · <code>Data.MapHash.MapHashable_U8</code></span><div class="doc-child-prose">Hash an unsigned 8-bit integer directly.</div></li>
</ul>
