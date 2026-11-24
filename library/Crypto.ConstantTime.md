# Crypto.ConstantTime

Branch-free scalar comparison and selection, full-input byte equality, conditional copying and in-place XOR with public bounds and atomic length errors.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Crypto.ConstantTime</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Crypto.ConstantTime/source/">View source</a><a href="/packages/search/?q=%2BCrypto.ConstantTime">Search module</a><a href="/reference-sources/Crypto.ConstantTime.cinch" download="Crypto.ConstantTime.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md).

<details class="doc-closure"><summary>Ship-source closure (2 files)</summary>

Copy these modules beside your entry file before importing `Crypto.ConstantTime`: [Crypto.ConstantTime](Crypto.ConstantTime.md), [Data.Bytes.Word](Data.Bytes.Word.md).

```sh
cp seed/prelude/Crypto/ConstantTime.cinch ./Crypto.ConstantTime.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Branch-free scalar selection and byte operations with public-length traversal.
Byte equality returns early only for unequal lengths. Conditional copies visit
every destination byte for either selection value. Buffers must not be mutated
concurrently. Whole Bytes handles may alias exactly; partial raw-pointer aliases
are outside this interface. Data-independent instruction timing is a separate
processor/runtime capability and is not enabled by these operations.

</details>

<details class="doc-synopsis" open><summary>Synopsis (8 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ConstantTimeError">ConstantTimeError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ConstantTimeError</strong></code></li><li><a class="syn-name" href="#v-constant-byte-equal">constant-byte-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">constant-byte-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-constant-bytes-equal">constant-bytes-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">constant-bytes-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-constant-copy">constant-copy</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">constant-copy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ConstantTime/#t-ConstantTimeError" title="Crypto.ConstantTime.ConstantTimeError">ConstantTimeError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-constant-i32-equal">constant-i32-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">constant-i32-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-constant-less-or-equal">constant-less-or-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">constant-less-or-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 2147483647 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 2147483647 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-constant-select">constant-select</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">constant-select</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-xor-bytes">xor-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">xor-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ConstantTime/#t-ConstantTimeError" title="Crypto.ConstantTime.ConstantTimeError">ConstantTimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ConstantTimeError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ConstantTimeError</strong></code><span class="doc-sig-links"><a href="#t-ConstantTimeError" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ConstantTime/source/#L18" title="View source at line 18">Source</a></span></div>
<div class="doc-body">

Public buffer dimensions cannot satisfy a complete operation.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ConstantTimeLengthMismatch"><code><strong class="tok-constructor">ConstantTimeLengthMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ConstantTime/#t-ConstantTimeError" title="Crypto.ConstantTime.ConstantTimeError">ConstantTimeError</a></code></li><li id="k-ConstantTimeShortDestination"><code><strong class="tok-constructor">ConstantTimeShortDestination</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ConstantTime/#t-ConstantTimeError" title="Crypto.ConstantTime.ConstantTimeError">ConstantTimeError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-constant-byte-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">constant-byte-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-constant-byte-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ConstantTime/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Compare two bytes without branching on their contents.

</div>
</article>

<article class="doc-entry" id="v-constant-bytes-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">constant-bytes-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-constant-bytes-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ConstantTime/source/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

Compare equal-length buffers using complete SIMD blocks and an exact scalar tail.
Every byte is read even when an earlier byte differs; lengths are public.

</div>
</article>

<article class="doc-entry" id="v-constant-copy">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">constant-copy</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ConstantTime/#t-ConstantTimeError" title="Crypto.ConstantTime.ConstantTimeError">ConstantTimeError</a> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-constant-copy" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ConstantTime/source/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Conditionally copy a complete source, preserving the destination for False.
A length mismatch is returned before any byte is written.

</div>
</article>

<article class="doc-entry" id="v-constant-i32-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">constant-i32-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I32">I32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-constant-i32-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ConstantTime/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

Compare signed 32-bit words without a content-dependent branch.

</div>
</article>

<article class="doc-entry" id="v-constant-less-or-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">constant-less-or-equal</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 2147483647 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Algebra.Arithmetic/#t-InRange" title="Algebra.Arithmetic.InRange">InRange</a> 0 2147483647 <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-constant-less-or-equal" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ConstantTime/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

Compare nonnegative 31-bit values; invalid mathematical inputs are excluded by their types.

</div>
</article>

<article class="doc-entry" id="v-constant-select">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">constant-select</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-constant-select" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ConstantTime/source/#L45" title="View source at line 45">Source</a></span></div>
<div class="doc-body">

Select the first word for True and the second for False without branching on the flag.

</div>
</article>

<article class="doc-entry" id="v-xor-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">xor-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Crypto.ConstantTime/#t-ConstantTimeError" title="Crypto.ConstantTime.ConstantTimeError">ConstantTimeError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-xor-bytes" title="Permalink">#</a><a href="/packages/stdlib/Crypto.ConstantTime/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

XOR the shorter input into the destination, returning the complete written length.
A short destination is returned before any write; untouched trailing bytes remain unchanged.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Crypto/ConstantTime.cinch`](/packages/stdlib/Crypto.ConstantTime/source/). Source SHA-256: `f63a1679dd1f0a976cf8fc58ac306af48bd69afa9fd65b847696b025a1dd08e7`.
