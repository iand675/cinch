# Effect.Scanner.Split

Byte-preserving token splitters with explicit absent, empty and final token states.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Effect.Scanner.Split</code></div><div class="doc-mod-bar-links"><a href="/library/source/Effect.Scanner.Split/">View source</a><a href="/library/search/?q=%2BEffect.Scanner.Split">Search module</a><a href="/reference-sources/Effect.Scanner.Split.cinch" download="Effect.Scanner.Split.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Char](Data.Char.md), [Data.Int](Data.Int.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.String](Data.String.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md).

<details class="doc-closure"><summary>Ship-source closure (9 files)</summary>

Copy these modules beside your entry file before importing `Effect.Scanner.Split`: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Unicode.Kernel](Data.Unicode.Kernel.md), [Data.Unicode.Tables](Data.Unicode.Tables.md), [Effect.Buffered.Core](Effect.Buffered.Core.md), [Effect.Buffered.Internal.Cell](Effect.Buffered.Internal.Cell.md), [Effect.Buffered.Internal.Scan](Effect.Buffered.Internal.Scan.md), [Effect.Scanner.Split](Effect.Scanner.Split.md), [Effect.Stream](Effect.Stream.md).

```sh
cp seed/prelude/Data/Bytes/Utf8.cinch ./Data.Bytes.Utf8.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Data/Unicode/Kernel.cinch ./Data.Unicode.Kernel.cinch
cp seed/prelude/Data/Unicode/Tables.cinch ./Data.Unicode.Tables.cinch
cp seed/prelude/Effect/Buffered/Core.cinch ./Effect.Buffered.Core.cinch
cp seed/prelude/Effect/Buffered/Internal/Cell.cinch ./Effect.Buffered.Internal.Cell.cinch
cp seed/prelude/Effect/Buffered/Internal/Scan.cinch ./Effect.Buffered.Internal.Scan.cinch
cp seed/prelude/Effect/Scanner/Split.cinch ./Effect.Scanner.Split.cinch
cp seed/prelude/Effect/Stream.cinch ./Effect.Stream.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Byte-preserving token splitters with explicit absent, empty and final token states.
Built-in tokens are fresh. Custom callbacks receive a private snapshot and their
returned tokens are snapshotted before retention. Empty nonterminal direct calls
request more input; they never inspect storage outside the logical byte span.

</details>

<details class="doc-synopsis" open><summary>Synopsis (11 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ScanError">ScanError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ScanError</strong></code></li><li><a class="syn-name" href="#t-ScanSplit">ScanSplit</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ScanSplit</strong></code></li><li><a class="syn-name" href="#t-ScanStep">ScanStep</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ScanStep</strong></code></li><li><a class="syn-name" href="#v-scan-apply">scan-apply</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-apply</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanSplit" title="Effect.Scanner.Split.ScanSplit">ScanSplit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code></li><li><a class="syn-name" href="#v-scan-apply-between">scan-apply-between</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-apply-between</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanSplit" title="Effect.Scanner.Split.ScanSplit">ScanSplit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code></li><li><a class="syn-name" href="#v-scan-bytes">scan-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code></li><li><a class="syn-name" href="#v-scan-error-message">scan-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-scan-lines">scan-lines</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-lines</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code></li><li><a class="syn-name" href="#v-scan-runes">scan-runes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-runes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code></li><li><a class="syn-name" href="#v-scan-split">scan-split</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-split</strong> :: (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanSplit" title="Effect.Scanner.Split.ScanSplit">ScanSplit</a></code></li><li><a class="syn-name" href="#v-scan-words">scan-words</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">scan-words</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ScanError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ScanError</strong></code><span class="doc-sig-links"><a href="#t-ScanError" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

A checked scanner failure, final-token sentinel, source failure or explicit custom diagnostic.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ScanTooLong"><code><strong class="tok-constructor">ScanTooLong</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li id="k-ScanNegativeAdvance"><code><strong class="tok-constructor">ScanNegativeAdvance</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li id="k-ScanAdvanceTooFar"><code><strong class="tok-constructor">ScanAdvanceTooFar</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li id="k-ScanBadReadCount"><code><strong class="tok-constructor">ScanBadReadCount</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li id="k-ScanFinalToken"><code><strong class="tok-constructor">ScanFinalToken</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li id="k-ScanNoProgress"><code><strong class="tok-constructor">ScanNoProgress</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li id="k-ScanEmptyTokens"><code><strong class="tok-constructor">ScanEmptyTokens</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li id="k-ScanEmptyTokensAfter"><code><strong class="tok-constructor">ScanEmptyTokensAfter</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li id="k-ScanStarted"><code><strong class="tok-constructor">ScanStarted</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li id="k-ScanSourceError"><code><strong class="tok-constructor">ScanSourceError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Buffered.Core/#t-BufferError" title="Effect.Buffered.Core.BufferError">BufferError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li><li id="k-ScanCustomError"><code><strong class="tok-constructor">ScanCustomError</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ScanSplit">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ScanSplit</strong></code><span class="doc-sig-links"><a href="#t-ScanSplit" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

A pure split capability receiving owned bytes and a flag indicating any terminal source status.

</div>
</article>

<article class="doc-entry" id="t-ScanStep">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ScanStep</strong></code><span class="doc-sig-links"><a href="#t-ScanStep" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L67" title="View source at line 67">Source</a></span></div>
<div class="doc-body">

Final-token errors take precedence over the advance count; other counts are checked by the scanner.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ScanStep"><code><strong class="tok-constructor">ScanStep</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-scan-apply">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-apply</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanSplit" title="Effect.Scanner.Split.ScanSplit">ScanSplit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code><span class="doc-sig-links"><a href="#v-scan-apply" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

Invoke a split capability with a fresh input snapshot and snapshot any mutable returned token.

</div>
</article>

<article class="doc-entry" id="v-scan-apply-between">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-apply-between</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanSplit" title="Effect.Scanner.Split.ScanSplit">ScanSplit</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code><span class="doc-sig-links"><a href="#v-scan-apply-between" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L90" title="View source at line 90">Source</a></span></div>
<div class="doc-body">

Invoke a splitter over a checked owner span with exactly one callback-input snapshot and one retained-token snapshot.
Offsets are absolute and invalid bounds return a checked error without invoking the callback.

</div>
</article>

<article class="doc-entry" id="v-scan-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code><span class="doc-sig-links"><a href="#v-scan-bytes" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Split one arbitrary byte; an empty input safely requests more data or completes at end-of-input.

</div>
</article>

<article class="doc-entry" id="v-scan-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanError" title="Effect.Scanner.Split.ScanError">ScanError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-scan-error-message" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Explain a scanner failure while retaining exact counts and original causes in its structured value.

</div>
</article>

<article class="doc-entry" id="v-scan-lines">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-lines</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code><span class="doc-sig-links"><a href="#v-scan-lines" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L123" title="View source at line 123">Source</a></span></div>
<div class="doc-body">

Split LF-terminated lines, remove one optional preceding CR, and return a final nonempty unterminated line.

</div>
</article>

<article class="doc-entry" id="v-scan-runes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-runes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code><span class="doc-sig-links"><a href="#v-scan-runes" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L107" title="View source at line 107">Source</a></span></div>
<div class="doc-body">

Split one shortest-form Unicode scalar, replacing each malformed input byte with encoded U+FFFD.
An incomplete nonterminal encoding requests more input; invalid and truncated terminal bytes consume one byte each.

</div>
</article>

<article class="doc-entry" id="v-scan-split">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-split</strong> :: (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanSplit" title="Effect.Scanner.Split.ScanSplit">ScanSplit</a></code><span class="doc-sig-links"><a href="#v-scan-split" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

Define a custom tokenization capability without exposing the scanner's private buffer.

</div>
</article>

<article class="doc-entry" id="v-scan-words">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">scan-words</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Effect.Scanner.Split/#t-ScanStep" title="Effect.Scanner.Split.ScanStep">ScanStep</a></code><span class="doc-sig-links"><a href="#v-scan-words" title="Permalink">#</a><a href="/library/source/Effect.Scanner.Split/#L152" title="View source at line 152">Source</a></span></div>
<div class="doc-body">

Split nonempty words on the Unicode White_Space property, retaining arbitrary non-whitespace bytes exactly.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Effect/Scanner/Split.cinch`](/library/source/Effect.Scanner.Split/). Source SHA-256: `5bcdaea29e3d7bcc9a7f77aaaad39e5fcb8af0504862e891a4d853f2e8e2c48e`.
