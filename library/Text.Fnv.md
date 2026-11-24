# Text.Fnv

Incremental 32-, 64- and 128-bit FNV-1 and FNV-1a hashing with explicit wrapping arithmetic.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Fnv</code></div><div class="doc-mod-bar-links"><a href="/library/source/Text.Fnv/">View source</a><a href="/library/search/?q=%2BText.Fnv">Search module</a><a href="/reference-sources/Text.Fnv.cinch" download="Text.Fnv.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Text.Fnv`: [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Fnv](Text.Fnv.md).

```sh
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/Fnv.cinch ./Text.Fnv.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Deterministic 32-, 64- and 128-bit FNV-1 and FNV-1a hashing over bytes.
Overflow wraps explicitly. Update functions accept a finished hash, so chunked
and concatenated input agree. Empty input yields the offset basis, and empty
updates leave the state unchanged. Inline LLVM unrolls eight ordered steps at
a time; the recurrence stays sequential. These non-cryptographic hashes are
separate from the collection hashing vocabulary in Data.Hash.

</details>

<details class="doc-synopsis" open><summary>Synopsis (12 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#v-fnv128">fnv128</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv128</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-fnv128-update">fnv128-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv128-update</strong> :: (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-fnv128a">fnv128a</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv128a</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-fnv128a-update">fnv128a-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv128a-update</strong> :: (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code></li><li><a class="syn-name" href="#v-fnv32">fnv32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv32</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-fnv32-update">fnv32-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv32-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-fnv32a">fnv32a</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv32a</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-fnv32a-update">fnv32a-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv32a-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-fnv64">fnv64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv64</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-fnv64-update">fnv64-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv64-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-fnv64a">fnv64a</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv64a</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code></li><li><a class="syn-name" href="#v-fnv64a-update">fnv64a-update</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fnv64a-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code></li></ul>
</details>

# Values

<article class="doc-entry" id="v-fnv128">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv128</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-fnv128" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L260" title="View source at line 260">Source</a></span></div>
<div class="doc-body">

Compute 128-bit FNV-1 with the standard offset basis, returning high word then low word.
Two U64 words preserve the full hash on every target without a wide-scalar ABI dependency.

</div>
</article>

<article class="doc-entry" id="v-fnv128-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv128-update</strong> :: (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-fnv128-update" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L269" title="View source at line 269">Source</a></span></div>
<div class="doc-body">

Continue a 128-bit FNV-1 state given as high and low words. Empty input preserves both words.

</div>
</article>

<article class="doc-entry" id="v-fnv128a">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv128a</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-fnv128a" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L265" title="View source at line 265">Source</a></span></div>
<div class="doc-body">

Compute 128-bit FNV-1a, returning high word then low word.
This deterministic hash is non-cryptographic; serialization byte order is chosen separately.

</div>
</article>

<article class="doc-entry" id="v-fnv128a-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv128a-update</strong> :: (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a>)</code><span class="doc-sig-links"><a href="#v-fnv128a-update" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L273" title="View source at line 273">Source</a></span></div>
<div class="doc-body">

Continue a high/low 128-bit FNV-1a state; arbitrary input chunking preserves the final result.

</div>
</article>

<article class="doc-entry" id="v-fnv32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv32</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-fnv32" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L19" title="View source at line 19">Source</a></span></div>
<div class="doc-body">

Compute 32-bit FNV-1, multiplying before xor, with offset basis 0x811C9DC5.

</div>
</article>

<article class="doc-entry" id="v-fnv32-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv32-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-fnv32-update" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L35" title="View source at line 35">Source</a></span></div>
<div class="doc-body">

Continue a finished 32-bit FNV-1 hash with more bytes; an empty update preserves the hash.

</div>
</article>

<article class="doc-entry" id="v-fnv32a">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv32a</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-fnv32a" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L23" title="View source at line 23">Source</a></span></div>
<div class="doc-body">

Compute 32-bit FNV-1a, xor before multiplying, with offset basis 0x811C9DC5.

</div>
</article>

<article class="doc-entry" id="v-fnv32a-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv32a-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-fnv32a-update" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

Continue a finished 32-bit FNV-1a hash with more bytes; an empty update preserves the hash.

</div>
</article>

<article class="doc-entry" id="v-fnv64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv64</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-fnv64" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L27" title="View source at line 27">Source</a></span></div>
<div class="doc-body">

Compute 64-bit FNV-1, multiplying before xor, with offset basis 0xCBF29CE484222325.

</div>
</article>

<article class="doc-entry" id="v-fnv64-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv64-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-fnv64-update" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L43" title="View source at line 43">Source</a></span></div>
<div class="doc-body">

Continue a finished 64-bit FNV-1 hash with more bytes; an empty update preserves the hash.

</div>
</article>

<article class="doc-entry" id="v-fnv64a">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv64a</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-fnv64a" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L31" title="View source at line 31">Source</a></span></div>
<div class="doc-body">

Compute 64-bit FNV-1a, xor before multiplying, with offset basis 0xCBF29CE484222325.

</div>
</article>

<article class="doc-entry" id="v-fnv64a-update">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fnv64a-update</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U64">U64</a></code><span class="doc-sig-links"><a href="#v-fnv64a-update" title="Permalink">#</a><a href="/library/source/Text.Fnv/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Continue a finished 64-bit FNV-1a hash with more bytes; an empty update preserves the hash.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Fnv.cinch`](/library/source/Text.Fnv/). Source SHA-256: `b09f33aa54974a8b67ccac104a8e08a3a2d6e2bfd0f8756e9ea0764aff93f529`.
