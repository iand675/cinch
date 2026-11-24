# Data.ChaCha8

The ChaCha8 pseudorandom generator: four interleaved eight-round blocks per vector kernel call, byte reads and state encoding.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.ChaCha8</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.ChaCha8/source/">View source</a><a href="/packages/search/?q=%2BData.ChaCha8">Search module</a><a href="/reference-sources/Data.ChaCha8.cinch" download="Data.ChaCha8.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

<details class="doc-closure"><summary>Ship-source closure (3 files)</summary>

Copy these modules beside your entry file before importing `Data.ChaCha8`: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.ChaCha8](Data.ChaCha8.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Data/ChaCha8.cinch ./Data.ChaCha8.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

ChaCha8: a fast, seekable-by-state, cryptographically strong pseudorandom generator producing 64-bit
values from a 32-byte seed, with byte reads and a compact binary encoding of its state.

The generator runs four ChaCha blocks with eight rounds at a time, word-interleaved so every quarter
round is one <4 x i32> operation per word; one block call yields thirty-two 64-bit outputs. After four
calls the last four outputs of the fourth become the next seed instead of being returned, so earlier
outputs cannot be recovered from a captured state. A state is an immutable value: drawing returns the
value and the next state, so a retained state replays exactly. The encoding records the seed and the
number of outputs used within the current reseed period, plus any bytes a partial read left unread.

</details>

<details class="doc-synopsis" open><summary>Synopsis (9 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ChaCha8">ChaCha8</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ChaCha8</strong></code></li><li><a class="syn-name" href="#t-ChaCha8Error">ChaCha8Error</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ChaCha8Error</strong></code></li><li><a class="syn-name" href="#v-chacha8-error-message">chacha8-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha8-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-chacha8-marshal">chacha8-marshal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha8-marshal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-chacha8-new">chacha8-new</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha8-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code></li><li><a class="syn-name" href="#v-chacha8-read">chacha8-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha8-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a>)</code></li><li><a class="syn-name" href="#v-chacha8-reseed">chacha8-reseed</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha8-reseed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code></li><li><a class="syn-name" href="#v-chacha8-uint64">chacha8-uint64</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha8-uint64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a>)</code></li><li><a class="syn-name" href="#v-chacha8-unmarshal">chacha8-unmarshal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">chacha8-unmarshal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ChaCha8">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ChaCha8</strong></code><span class="doc-sig-links"><a href="#t-ChaCha8" title="Permalink">#</a><a href="/packages/stdlib/Data.ChaCha8/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Fields in the order listed above.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ChaCha8"><code><strong class="tok-constructor">ChaCha8</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Wide/" title="U128">U128</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code></li></ul></div>
<details class="doc-instances"><summary>Instances (1)</summary><ul><li><code><a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Rand/#c-Source" title="Data.Rand.Source">Source</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code> <span class="doc-inst-meta">implicit · <a href="/packages/stdlib/Data.Rand/#instances">Data.Rand</a></span></li></ul></details>
</div>
</article>

<article class="doc-entry" id="t-ChaCha8Error">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ChaCha8Error</strong></code><span class="doc-sig-links"><a href="#t-ChaCha8Error" title="Permalink">#</a><a href="/packages/stdlib/Data.ChaCha8/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Why a seed or an encoding was refused.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ChaCha8SeedSize"><code><strong class="tok-constructor">ChaCha8SeedSize</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a></code></li><li id="k-ChaCha8BadEncoding"><code><strong class="tok-constructor">ChaCha8BadEncoding</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a></code></li><li id="k-ChaCha8BadReadBuffer"><code><strong class="tok-constructor">ChaCha8BadReadBuffer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-chacha8-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha8-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-chacha8-error-message" title="Permalink">#</a><a href="/packages/stdlib/Data.ChaCha8/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

A readable description of an error.

</div>
</article>

<article class="doc-entry" id="v-chacha8-marshal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha8-marshal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-chacha8-marshal" title="Permalink">#</a><a href="/packages/stdlib/Data.ChaCha8/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

The state's encoding: "readbuf:", a length byte and the unread bytes when there are any, then "chacha8:",
the outputs used in this reseed period (big-endian 64-bit) and the seed (four little-endian 64-bit words).

</div>
</article>

<article class="doc-entry" id="v-chacha8-new">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha8-new</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code><span class="doc-sig-links"><a href="#v-chacha8-new" title="Permalink">#</a><a href="/packages/stdlib/Data.ChaCha8/source/#L94" title="View source at line 94">Source</a></span></div>
<div class="doc-body">

A generator from a 32-byte seed (four little-endian 64-bit words).

</div>
</article>

<article class="doc-entry" id="v-chacha8-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha8-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a>)</code><span class="doc-sig-links"><a href="#v-chacha8-read" title="Permalink">#</a><a href="/packages/stdlib/Data.ChaCha8/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

Read n pseudorandom bytes: unread bytes first, then whole values little-endian; a final partial value keeps
its remaining bytes for the next read.

</div>
</article>

<article class="doc-entry" id="v-chacha8-reseed">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha8-reseed</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code><span class="doc-sig-links"><a href="#v-chacha8-reseed" title="Permalink">#</a><a href="/packages/stdlib/Data.ChaCha8/source/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

Start over from a new 32-byte seed, dropping unread bytes.

</div>
</article>

<article class="doc-entry" id="v-chacha8-uint64">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha8-uint64</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a> <span class="tok-operator">-&gt;</span> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U64">U64</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a>)</code><span class="doc-sig-links"><a href="#v-chacha8-uint64" title="Permalink">#</a><a href="/packages/stdlib/Data.ChaCha8/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

The next 64-bit value and the following state.

</div>
</article>

<article class="doc-entry" id="v-chacha8-unmarshal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">chacha8-unmarshal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8Error" title="Data.ChaCha8.ChaCha8Error">ChaCha8Error</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.ChaCha8/#t-ChaCha8" title="Data.ChaCha8.ChaCha8">ChaCha8</a></code><span class="doc-sig-links"><a href="#v-chacha8-unmarshal" title="Permalink">#</a><a href="/packages/stdlib/Data.ChaCha8/source/#L205" title="View source at line 205">Source</a></span></div>
<div class="doc-body">

Restore a state from its encoding.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/ChaCha8.cinch`](/packages/stdlib/Data.ChaCha8/source/). Source SHA-256: `84cb4bed6d1e5788ab75b95e2315ef792c7376aefba51fb06cfddd2af67be699`.
