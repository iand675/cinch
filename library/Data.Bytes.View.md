# Data.Bytes.View

Owner-retaining checked byte views, relative UTF-8 observations, stable compiled substring patterns and bounded search, cut and comparison operations.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Bytes.View</code></div><div class="doc-mod-bar-links"><a href="/library/source/Data.Bytes.View/">View source</a><a href="/library/search/?q=%2BData.Bytes.View">Search module</a><a href="/reference-sources/Data.Bytes.View.cinch" download="Data.Bytes.View.cinch">Download .cinch</a><a href="/library/00-index/">All modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md).

<details class="doc-closure"><summary>Ship-source closure (3 files)</summary>

Copy these modules beside your entry file before importing `Data.Bytes.View`: [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.View](Data.Bytes.View.md), [Data.Bytes.Word](Data.Bytes.Word.md).

```sh
cp seed/prelude/Data/Bytes/Utf8.cinch ./Data.Bytes.Utf8.cinch
cp seed/prelude/Data/Bytes/View.cinch ./Data.Bytes.View.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Bounded mutable aliases retaining an entire fixed byte-buffer owner.
Views share observable mutations with their owner and overlapping views.
A small view retains the complete owner allocation until the view is released.
Snapshots and clones copy into fresh storage. Private backing and addresses
are never exposed; all reads, writes and range copies carry checked bounds.

</details>

<details class="doc-synopsis"><summary>Synopsis (53 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-BytePattern">BytePattern</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BytePattern</strong></code></li><li><a class="syn-name" href="#t-ByteView">ByteView</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteView</strong></code></li><li><a class="syn-name" href="#t-ByteViewError">ByteViewError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteViewError</strong></code></li><li><a class="syn-name" href="#v-byte-pattern-bytes">byte-pattern-bytes</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-pattern-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-byte-pattern-length">byte-pattern-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-pattern-length</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-ascii-space-run-end">byte-view-ascii-space-run-end</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-ascii-space-run-end</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-clone">byte-view-clone</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-clone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-compare">byte-view-compare</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-compare</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-byte-view-contains">byte-view-contains</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-contains</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-byte-view-contains-pattern">byte-view-contains-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-contains-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-byte-view-copy-into">byte-view-copy-into</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-copy-into</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteViewError" title="Data.Bytes.View.ByteViewError">ByteViewError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-count">byte-view-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-count-pattern">byte-view-count-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-count-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-cut">byte-view-cut</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-cut</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code></li><li><a class="syn-name" href="#v-byte-view-cut-last">byte-view-cut-last</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-last</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code></li><li><a class="syn-name" href="#v-byte-view-cut-last-pattern">byte-view-cut-last-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-last-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code></li><li><a class="syn-name" href="#v-byte-view-cut-pattern">byte-view-cut-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code></li><li><a class="syn-name" href="#v-byte-view-cut-prefix">byte-view-cut-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-cut-prefix-pattern">byte-view-cut-prefix-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-prefix-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-cut-suffix">byte-view-cut-suffix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-suffix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-cut-suffix-pattern">byte-view-cut-suffix-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-suffix-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-drop">byte-view-drop</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-drop</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-equal">byte-view-equal</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-equal</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-byte-view-get">byte-view-get</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-get</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a></code></li><li><a class="syn-name" href="#v-byte-view-has-prefix">byte-view-has-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-has-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-byte-view-has-prefix-pattern">byte-view-has-prefix-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-has-prefix-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-byte-view-has-suffix">byte-view-has-suffix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-has-suffix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-byte-view-has-suffix-pattern">byte-view-has-suffix-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-has-suffix-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-byte-view-index">byte-view-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-index-byte">byte-view-index-byte</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-index-byte</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-index-pattern">byte-view-index-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-index-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-is-empty">byte-view-is-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-is-empty</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-byte-view-last-index">byte-view-last-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-last-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-last-index-pattern">byte-view-last-index-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-last-index-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-length">byte-view-length</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-length</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-byte-view-set">byte-view-set</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-set</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-byte-view-slice">byte-view-slice</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-slice</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteViewError" title="Data.Bytes.View.ByteViewError">ByteViewError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-snapshot">byte-view-snapshot</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-snapshot</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-byte-view-take">byte-view-take</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-take</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-utf8-at">byte-view-utf8-at</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-utf8-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code></li><li><a class="syn-name" href="#v-bytes-compile-pattern">bytes-compile-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-compile-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a></code></li><li><a class="syn-name" href="#v-bytes-contains">bytes-contains</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-contains</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-bytes-count">bytes-count</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-bytes-cut">bytes-cut</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-cut</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code></li><li><a class="syn-name" href="#v-bytes-cut-last">bytes-cut-last</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-cut-last</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code></li><li><a class="syn-name" href="#v-bytes-cut-prefix">bytes-cut-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-cut-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-bytes-cut-suffix">bytes-cut-suffix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-cut-suffix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-bytes-has-prefix">bytes-has-prefix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-has-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-bytes-has-suffix">bytes-has-suffix</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-has-suffix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-bytes-index">bytes-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-bytes-last-index">bytes-last-index</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-last-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-bytes-view">bytes-view</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-view</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-bytes-view-range">bytes-view-range</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-view-range</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteViewError" title="Data.Bytes.View.ByteViewError">ByteViewError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-BytePattern">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">BytePattern</strong></code><span class="doc-sig-links"><a href="#t-BytePattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L133" title="View source at line 133">Source</a></span></div>
<div class="doc-body">

A stable private needle snapshot and its complete proper-prefix failure table.

</div>
</article>

<article class="doc-entry" id="t-ByteView">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteView</strong></code><span class="doc-sig-links"><a href="#t-ByteView" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

An opaque fixed range whose endpoint refinements preserve the owner's bounds.

</div>
</article>

<article class="doc-entry" id="t-ByteViewError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteViewError</strong></code><span class="doc-sig-links"><a href="#t-ByteViewError" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

A refused view range or complete-copy destination; refusals never mutate storage.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ByteViewReversedRange"><code><strong class="tok-constructor">ByteViewReversedRange</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteViewError" title="Data.Bytes.View.ByteViewError">ByteViewError</a></code></li><li id="k-ByteViewPastEnd"><code><strong class="tok-constructor">ByteViewPastEnd</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteViewError" title="Data.Bytes.View.ByteViewError">ByteViewError</a></code></li><li id="k-ByteViewShortDestination"><code><strong class="tok-constructor">ByteViewShortDestination</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteViewError" title="Data.Bytes.View.ByteViewError">ByteViewError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-byte-pattern-bytes">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-pattern-bytes</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-byte-pattern-bytes" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L185" title="View source at line 185">Source</a></span></div>
<div class="doc-body">

Copy the compiled needle into fresh caller-owned storage.

</div>
</article>

<article class="doc-entry" id="v-byte-pattern-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-pattern-length</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-pattern-length" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

Count exact needle bytes without exposing private snapshot storage.

</div>
</article>

<article class="doc-entry" id="v-byte-view-ascii-space-run-end">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-ascii-space-run-end</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-view-ascii-space-run-end" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L903" title="View source at line 903">Source</a></span></div>
<div class="doc-body">

End a maximal ASCII whitespace or non-whitespace run from a relative position.
True selects exactly TAB through CR and SPACE; False selects other ASCII bytes.
A non-ASCII byte ends either run. End positions are valid; positions past the view return None.
Observes current aliases, retains the whole owner and never reads outside the logical view.

</div>
</article>

<article class="doc-entry" id="v-byte-view-clone">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-clone</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-clone" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

Clone a view into a new independent whole-buffer view, including empty ranges.

</div>
</article>

<article class="doc-entry" id="v-byte-view-compare">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-compare</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-byte-view-compare" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L741" title="View source at line 741">Source</a></span></div>
<div class="doc-body">

Compare raw view bytes lexicographically, returning exactly negative one, zero or one.

</div>
</article>

<article class="doc-entry" id="v-byte-view-contains">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-contains</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-byte-view-contains" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L763" title="View source at line 763">Source</a></span></div>
<div class="doc-body">

Test exact raw needle containment without copying the view.

</div>
</article>

<article class="doc-entry" id="v-byte-view-contains-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-contains-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-byte-view-contains-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L620" title="View source at line 620">Source</a></span></div>
<div class="doc-body">

Test whether a compiled needle occurs, including the empty needle in an empty view.

</div>
</article>

<article class="doc-entry" id="v-byte-view-copy-into">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-copy-into</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteViewError" title="Data.Bytes.View.ByteViewError">ByteViewError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-view-copy-into" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

Copy a complete view into a checked destination interval with overlap-safe range copying.
Destination refusal is atomic; the required and available sizes are preserved in the error.

</div>
</article>

<article class="doc-entry" id="v-byte-view-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-view-count" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L767" title="View source at line 767">Source</a></span></div>
<div class="doc-body">

Count a raw needle with bounded malformed-byte counting for empty patterns.

</div>
</article>

<article class="doc-entry" id="v-byte-view-count-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-count-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-view-count-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L694" title="View source at line 694">Source</a></span></div>
<div class="doc-body">

Count nonoverlapping compiled matches; an empty pattern counts bounded decoded scalars plus one.
Malformed or view-truncated bytes each make one replacement-scalar step without copying the view.

</div>
</article>

<article class="doc-entry" id="v-byte-view-cut">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-cut</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code><span class="doc-sig-links"><a href="#v-byte-view-cut" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L786" title="View source at line 786">Source</a></span></div>
<div class="doc-body">

Cut at the first raw match into aliases that retain the complete original owner.

</div>
</article>

<article class="doc-entry" id="v-byte-view-cut-last">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-last</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code><span class="doc-sig-links"><a href="#v-byte-view-cut-last" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L790" title="View source at line 790">Source</a></span></div>
<div class="doc-body">

Cut at the final raw match, including overlapping candidates.

</div>
</article>

<article class="doc-entry" id="v-byte-view-cut-last-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-last-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code><span class="doc-sig-links"><a href="#v-byte-view-cut-last-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L655" title="View source at line 655">Source</a></span></div>
<div class="doc-body">

Cut around the final compiled match, preserving overlapping last-match semantics.

</div>
</article>

<article class="doc-entry" id="v-byte-view-cut-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code><span class="doc-sig-links"><a href="#v-byte-view-cut-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L649" title="View source at line 649">Source</a></span></div>
<div class="doc-body">

Cut around the first compiled match; empty needles produce empty-before and the complete view.

</div>
</article>

<article class="doc-entry" id="v-byte-view-cut-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-cut-prefix" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L794" title="View source at line 794">Source</a></span></div>
<div class="doc-body">

Remove a raw prefix as an owner-retaining alias, including an unchanged empty prefix.

</div>
</article>

<article class="doc-entry" id="v-byte-view-cut-prefix-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-prefix-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-cut-prefix-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L661" title="View source at line 661">Source</a></span></div>
<div class="doc-body">

Remove an exact compiled prefix as an owner-retaining alias, or report no match.

</div>
</article>

<article class="doc-entry" id="v-byte-view-cut-suffix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-suffix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-cut-suffix" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L799" title="View source at line 799">Source</a></span></div>
<div class="doc-body">

Remove a raw suffix as an owner-retaining alias, including an unchanged empty suffix.

</div>
</article>

<article class="doc-entry" id="v-byte-view-cut-suffix-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-cut-suffix-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-cut-suffix-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L666" title="View source at line 666">Source</a></span></div>
<div class="doc-body">

Remove an exact compiled suffix as an owner-retaining alias, or report no match.

</div>
</article>

<article class="doc-entry" id="v-byte-view-drop">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-drop</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-drop" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L115" title="View source at line 115">Source</a></span></div>
<div class="doc-body">

Drop up to the requested leading bytes, clamping at an empty view of the original end.

</div>
</article>

<article class="doc-entry" id="v-byte-view-equal">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-equal</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-byte-view-equal" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L707" title="View source at line 707">Source</a></span></div>
<div class="doc-body">

Compare two exact view ranges using complete words and scalar fringes, preserving shared mutations.

</div>
</article>

<article class="doc-entry" id="v-byte-view-get">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-get</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a></code><span class="doc-sig-links"><a href="#v-byte-view-get" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

Read an exact relative byte, or no value when the coordinate is outside the view.

</div>
</article>

<article class="doc-entry" id="v-byte-view-has-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-has-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-byte-view-has-prefix" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L771" title="View source at line 771">Source</a></span></div>
<div class="doc-body">

Test an exact raw prefix with complete-word comparisons and no snapshots or pattern allocation.

</div>
</article>

<article class="doc-entry" id="v-byte-view-has-prefix-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-has-prefix-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-byte-view-has-prefix-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L624" title="View source at line 624">Source</a></span></div>
<div class="doc-body">

Test an exact compiled prefix using complete-word comparisons and checked view lengths.

</div>
</article>

<article class="doc-entry" id="v-byte-view-has-suffix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-has-suffix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-byte-view-has-suffix" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L778" title="View source at line 778">Source</a></span></div>
<div class="doc-body">

Test an exact raw suffix with no snapshots, pattern allocation or unrelated prefix scan.

</div>
</article>

<article class="doc-entry" id="v-byte-view-has-suffix-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-has-suffix-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-byte-view-has-suffix-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L632" title="View source at line 632">Source</a></span></div>
<div class="doc-body">

Test an exact compiled suffix without copying or scanning the unrelated prefix.

</div>
</article>

<article class="doc-entry" id="v-byte-view-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-view-index" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L755" title="View source at line 755">Source</a></span></div>
<div class="doc-body">

Find a raw needle relative to a retained view, compiling one stable snapshot for this query.

</div>
</article>

<article class="doc-entry" id="v-byte-view-index-byte">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-index-byte</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-view-index-byte" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L586" title="View source at line 586">Source</a></span></div>
<div class="doc-body">

Find the first exact byte relative to the checked view, or None when it is absent.
Every byte value is compared directly, including malformed UTF8 bytes; current aliases remain observable.

</div>
</article>

<article class="doc-entry" id="v-byte-view-index-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-index-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-view-index-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L595" title="View source at line 595">Source</a></span></div>
<div class="doc-body">

Find the first compiled needle match relative to the view; an empty needle matches zero.

</div>
</article>

<article class="doc-entry" id="v-byte-view-is-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-is-empty</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-byte-view-is-empty" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Test whether a retained range has no logical bytes.

</div>
</article>

<article class="doc-entry" id="v-byte-view-last-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-last-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-view-last-index" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L759" title="View source at line 759">Source</a></span></div>
<div class="doc-body">

Find the last raw needle, including overlapping matches and the empty end position.

</div>
</article>

<article class="doc-entry" id="v-byte-view-last-index-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-last-index-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-view-last-index-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L607" title="View source at line 607">Source</a></span></div>
<div class="doc-body">

Find the final compiled match, including overlaps; an empty needle matches the view end.

</div>
</article>

<article class="doc-entry" id="v-byte-view-length">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-length</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-byte-view-length" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

Count logical bytes in a view without copying or releasing its retained owner.

</div>
</article>

<article class="doc-entry" id="v-byte-view-set">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-set</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Machine/" title="U8">U8</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-byte-view-set" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Write one exact relative byte and report whether it fit; refusal leaves all aliases unchanged.

</div>
</article>

<article class="doc-entry" id="v-byte-view-slice">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-slice</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteViewError" title="Data.Bytes.View.ByteViewError">ByteViewError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-slice" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

Retain a checked half-open subrange relative to this view, preserving shared mutations.

</div>
</article>

<article class="doc-entry" id="v-byte-view-snapshot">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-snapshot</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-byte-view-snapshot" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Copy exactly the logical range into fresh mutable storage without retaining the source owner.

</div>
</article>

<article class="doc-entry" id="v-byte-view-take">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-take</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-take" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Retain up to the requested leading bytes, clamping the count to this view's length.

</div>
</article>

<article class="doc-entry" id="v-byte-view-utf8-at">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-utf8-at</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.Utf8/#t-Utf8Step" title="Data.Bytes.Utf8.Utf8Step">Utf8Step</a></code><span class="doc-sig-links"><a href="#v-byte-view-utf8-at" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L126" title="View source at line 126">Source</a></span></div>
<div class="doc-body">

Observe one bounded scalar at a relative byte position without copying or exposing backing storage.
The logical end returns Utf8End; positions past it return Utf8OutOfBounds.
Each malformed or view-truncated byte returns U+FFFD with width one and Utf8Invalid.

</div>
</article>

<article class="doc-entry" id="v-bytes-compile-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-compile-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a></code><span class="doc-sig-links"><a href="#v-bytes-compile-pattern" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L174" title="View source at line 174">Source</a></span></div>
<div class="doc-body">

Compile an independent needle snapshot and a reusable linear-search failure table.
Caller mutation after compilation never changes this pattern or its later search results.

</div>
</article>

<article class="doc-entry" id="v-bytes-contains">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-contains</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-bytes-contains" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L814" title="View source at line 814">Source</a></span></div>
<div class="doc-body">

Test raw needle containment in a complete fixed buffer.

</div>
</article>

<article class="doc-entry" id="v-bytes-count">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-count</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-bytes-count" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L818" title="View source at line 818">Source</a></span></div>
<div class="doc-body">

Count nonoverlapping raw matches, or decoded scalars plus one for an empty needle.

</div>
</article>

<article class="doc-entry" id="v-bytes-cut">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-cut</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code><span class="doc-sig-links"><a href="#v-bytes-cut" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L830" title="View source at line 830">Source</a></span></div>
<div class="doc-body">

Cut a complete fixed buffer at its first raw match into retained mutable aliases.

</div>
</article>

<article class="doc-entry" id="v-bytes-cut-last">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-cut-last</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> (<a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>, <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a>)</code><span class="doc-sig-links"><a href="#v-bytes-cut-last" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L834" title="View source at line 834">Source</a></span></div>
<div class="doc-body">

Cut a complete fixed buffer at its final raw match into retained mutable aliases.

</div>
</article>

<article class="doc-entry" id="v-bytes-cut-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-cut-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-cut-prefix" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L838" title="View source at line 838">Source</a></span></div>
<div class="doc-body">

Remove an exact prefix from a complete buffer while retaining its whole owner.

</div>
</article>

<article class="doc-entry" id="v-bytes-cut-suffix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-cut-suffix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-cut-suffix" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L842" title="View source at line 842">Source</a></span></div>
<div class="doc-body">

Remove an exact suffix from a complete buffer while retaining its whole owner.

</div>
</article>

<article class="doc-entry" id="v-bytes-has-prefix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-has-prefix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-bytes-has-prefix" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L822" title="View source at line 822">Source</a></span></div>
<div class="doc-body">

Test whether a complete fixed buffer starts with exact raw bytes.

</div>
</article>

<article class="doc-entry" id="v-bytes-has-suffix">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-has-suffix</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-bytes-has-suffix" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L826" title="View source at line 826">Source</a></span></div>
<div class="doc-body">

Test whether a complete fixed buffer ends with exact raw bytes.

</div>
</article>

<article class="doc-entry" id="v-bytes-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-bytes-index" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L806" title="View source at line 806">Source</a></span></div>
<div class="doc-body">

Find the first raw needle in a complete fixed buffer as an optional byte offset.

</div>
</article>

<article class="doc-entry" id="v-bytes-last-index">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-last-index</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-bytes-last-index" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L810" title="View source at line 810">Source</a></span></div>
<div class="doc-body">

Find the last raw needle in a complete fixed buffer, including overlaps.

</div>
</article>

<article class="doc-entry" id="v-bytes-view">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-view</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-view" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L48" title="View source at line 48">Source</a></span></div>
<div class="doc-body">

Retain the whole fixed mutable buffer without copying its contents.

</div>
</article>

<article class="doc-entry" id="v-bytes-view-range">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-view-range</strong> :: <a class="doc-type-link tok-constructor" href="/library/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/library/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteViewError" title="Data.Bytes.View.ByteViewError">ByteViewError</a> <a class="doc-type-link tok-constructor" href="/library/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-view-range" title="Permalink">#</a><a href="/library/source/Data.Bytes.View/#L52" title="View source at line 52">Source</a></span></div>
<div class="doc-body">

Retain an explicitly checked half-open owner range, including empty ranges at the end.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Bytes/View.cinch`](/library/source/Data.Bytes.View/). Source SHA-256: `3e3335b571ed40d23013ebdeb851311cbb8ad85d32e2841450050e5b5a332aeb`.
