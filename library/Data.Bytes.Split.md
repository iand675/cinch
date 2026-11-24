# Data.Bytes.Split

Retained byte splitting, persistent producer traversals, shared pull cursors and line iteration.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Data.Bytes.Split</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Data.Bytes.Split/source/">View source</a><a href="/packages/search/?q=%2BData.Bytes.Split">Search module</a><a href="/reference-sources/Data.Bytes.Split.cinch" download="Data.Bytes.Split.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.View](Data.Bytes.View.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Producer](Data.Producer.md).

<details class="doc-closure"><summary>Ship-source closure (4 files)</summary>

Copy these modules beside your entry file before importing `Data.Bytes.Split`: [Data.Bytes.Split](Data.Bytes.Split.md), [Data.Bytes.Utf8](Data.Bytes.Utf8.md), [Data.Bytes.View](Data.Bytes.View.md), [Data.Bytes.Word](Data.Bytes.Word.md).

```sh
cp seed/prelude/Data/Bytes/Split.cinch ./Data.Bytes.Split.cinch
cp seed/prelude/Data/Bytes/Utf8.cinch ./Data.Bytes.Utf8.cinch
cp seed/prelude/Data/Bytes/View.cinch ./Data.Bytes.View.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
```

</details>

<details class="doc-notes"><summary>Module notes</summary>

Retained byte splitting with explicit traversal factories, persistent producers and stateful pull cursors.
Result views share input mutations and retain the complete original allocation.
Raw separators are observed at each step; compiled separators retain their stable snapshots.

</details>

<details class="doc-synopsis"><summary>Synopsis (27 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ByteSplitCursor">ByteSplitCursor</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteSplitCursor</strong></code></li><li><a class="syn-name" href="#t-ByteSplitFactory">ByteSplitFactory</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteSplitFactory</strong></code></li><li><a class="syn-name" href="#t-ByteSplitMode">ByteSplitMode</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteSplitMode</strong></code></li><li><a class="syn-name" href="#t-ByteSplitState">ByteSplitState</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteSplitState</strong></code></li><li><a class="syn-name" href="#v-byte-split-close">byte-split-close</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-split-close</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitCursor" title="Data.Bytes.Split.ByteSplitCursor">ByteSplitCursor</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li><li><a class="syn-name" href="#v-byte-split-cursor">byte-split-cursor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-split-cursor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitCursor" title="Data.Bytes.Split.ByteSplitCursor">ByteSplitCursor</a></code></li><li><a class="syn-name" href="#v-byte-split-next">byte-split-next</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-split-next</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitCursor" title="Data.Bytes.Split.ByteSplitCursor">ByteSplitCursor</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-split-producer">byte-split-producer</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-split-producer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-lines">byte-view-lines</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-lines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-lines-factory">byte-view-lines-factory</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-lines-factory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a></code></li><li><a class="syn-name" href="#v-byte-view-split">byte-view-split</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-split</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-split-after">byte-view-split-after</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-split-after</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-split-after-limit">byte-view-split-after-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-split-after-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-split-after-seq">byte-view-split-after-seq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-split-after-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-split-factory">byte-view-split-factory</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-split-factory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitMode" title="Data.Bytes.Split.ByteSplitMode">ByteSplitMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a></code></li><li><a class="syn-name" href="#v-byte-view-split-limit">byte-view-split-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-split-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-split-pattern">byte-view-split-pattern</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-split-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitMode" title="Data.Bytes.Split.ByteSplitMode">ByteSplitMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-byte-view-split-pattern-factory">byte-view-split-pattern-factory</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-split-pattern-factory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitMode" title="Data.Bytes.Split.ByteSplitMode">ByteSplitMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a></code></li><li><a class="syn-name" href="#v-byte-view-split-seq">byte-view-split-seq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">byte-view-split-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-bytes-lines">bytes-lines</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-lines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-bytes-split">bytes-split</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-split</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-bytes-split-after">bytes-split-after</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-split-after</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-bytes-split-after-limit">bytes-split-after-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-split-after-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-bytes-split-after-seq">bytes-split-after-seq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-split-after-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-bytes-split-factory">bytes-split-factory</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-split-factory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitMode" title="Data.Bytes.Split.ByteSplitMode">ByteSplitMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a></code></li><li><a class="syn-name" href="#v-bytes-split-limit">bytes-split-limit</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-split-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li><li><a class="syn-name" href="#v-bytes-split-seq">bytes-split-seq</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">bytes-split-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ByteSplitCursor">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteSplitCursor</strong></code><span class="doc-sig-links"><a href="#t-ByteSplitCursor" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L49" title="View source at line 49">Source</a></span></div>
<div class="doc-body">

A separately named stateful pull cursor; copied handles share progress and closure state.

</div>
</article>

<article class="doc-entry" id="t-ByteSplitFactory">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteSplitFactory</strong></code><span class="doc-sig-links"><a href="#t-ByteSplitFactory" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L39" title="View source at line 39">Source</a></span></div>
<div class="doc-body">

An immutable traversal recipe retaining input, separator policy, limit and fragment placement.

</div>
</article>

<article class="doc-entry" id="t-ByteSplitMode">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteSplitMode</strong></code><span class="doc-sig-links"><a href="#t-ByteSplitMode" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L25" title="View source at line 25">Source</a></span></div>
<div class="doc-body">

Whether a separator belongs to the preceding output fragment.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-SplitBefore"><code><strong class="tok-constructor">SplitBefore</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitMode" title="Data.Bytes.Split.ByteSplitMode">ByteSplitMode</a></code></li><li id="k-SplitAfter"><code><strong class="tok-constructor">SplitAfter</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitMode" title="Data.Bytes.Split.ByteSplitMode">ByteSplitMode</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ByteSplitState">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ByteSplitState</strong></code><span class="doc-sig-links"><a href="#t-ByteSplitState" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L44" title="View source at line 44">Source</a></span></div>
<div class="doc-body">

Explicit persistent producer progress; advancing returns a new state without mutating this value.

</div>
</article>

# Values

<article class="doc-entry" id="v-byte-split-close">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-split-close</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitCursor" title="Data.Bytes.Split.ByteSplitCursor">ByteSplitCursor</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-byte-split-close" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L242" title="View source at line 242">Source</a></span></div>
<div class="doc-body">

Close a separately named pull cursor; later pulls return None without reading or searching input.

</div>
</article>

<article class="doc-entry" id="v-byte-split-cursor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-split-cursor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitCursor" title="Data.Bytes.Split.ByteSplitCursor">ByteSplitCursor</a></code><span class="doc-sig-links"><a href="#v-byte-split-cursor" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L226" title="View source at line 226">Source</a></span></div>
<div class="doc-body">

Begin a fresh stateful pull cursor without changing Producer replay semantics.
Copied cursor handles share progress; raw separator and input contents remain live.

</div>
</article>

<article class="doc-entry" id="v-byte-split-next">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-split-next</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitCursor" title="Data.Bytes.Split.ByteSplitCursor">ByteSplitCursor</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-split-next" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L230" title="View source at line 230">Source</a></span></div>
<div class="doc-body">

Pull the next mutable alias, returning None after exhaustion or explicit closure on every later call.

</div>
</article>

<article class="doc-entry" id="v-byte-split-producer">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-split-producer</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-split-producer" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Begin a fresh persistent producer traversal of a retained recipe.
Replaying this Producer follows the existing explicit-state contract and reads current mutable bytes again.

</div>
</article>

<article class="doc-entry" id="v-byte-view-lines">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-lines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-lines" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L171" title="View source at line 171">Source</a></span></div>
<div class="doc-body">

Begin a fresh persistent LF-line traversal including line terminators.

</div>
</article>

<article class="doc-entry" id="v-byte-view-lines-factory">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-lines-factory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a></code><span class="doc-sig-links"><a href="#v-byte-view-lines-factory" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Create an LF-line recipe that includes terminators and omits a trailing empty fragment.
CR bytes are preserved; an unterminated final line is returned intact.

</div>
</article>

<article class="doc-entry" id="v-byte-view-split">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-split</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-split" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L143" title="View source at line 143">Source</a></span></div>
<div class="doc-body">

Split into mutable aliases around each nonoverlapping raw separator.

</div>
</article>

<article class="doc-entry" id="v-byte-view-split-after">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-split-after</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-split-after" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L147" title="View source at line 147">Source</a></span></div>
<div class="doc-body">

Split after each nonoverlapping raw separator, retaining it in the preceding alias.

</div>
</article>

<article class="doc-entry" id="v-byte-view-split-after-limit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-split-after-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-split-after-limit" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L155" title="View source at line 155">Source</a></span></div>
<div class="doc-body">

Split after separators with an explicit optional maximum and an unsplit final remainder.

</div>
</article>

<article class="doc-entry" id="v-byte-view-split-after-seq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-split-after-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-split-after-seq" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L163" title="View source at line 163">Source</a></span></div>
<div class="doc-body">

Begin a fresh persistent producer that includes each delimiter in its preceding fragment.

</div>
</article>

<article class="doc-entry" id="v-byte-view-split-factory">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-split-factory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitMode" title="Data.Bytes.Split.ByteSplitMode">ByteSplitMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a></code><span class="doc-sig-links"><a href="#v-byte-view-split-factory" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

Create a reusable recipe without reading input or separator contents.
None means unlimited; Some zero yields nothing; the final limited fragment is the unsplit remainder.

</div>
</article>

<article class="doc-entry" id="v-byte-view-split-limit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-split-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-split-limit" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L151" title="View source at line 151">Source</a></span></div>
<div class="doc-body">

Split with an explicit optional maximum and an unsplit final remainder.

</div>
</article>

<article class="doc-entry" id="v-byte-view-split-pattern">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-split-pattern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitMode" title="Data.Bytes.Split.ByteSplitMode">ByteSplitMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-split-pattern" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L167" title="View source at line 167">Source</a></span></div>
<div class="doc-body">

Materialize fragments using an explicitly stable compiled separator snapshot.

</div>
</article>

<article class="doc-entry" id="v-byte-view-split-pattern-factory">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-split-pattern-factory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitMode" title="Data.Bytes.Split.ByteSplitMode">ByteSplitMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-BytePattern" title="Data.Bytes.View.BytePattern">BytePattern</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a></code><span class="doc-sig-links"><a href="#v-byte-view-split-pattern-factory" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L65" title="View source at line 65">Source</a></span></div>
<div class="doc-body">

Create a recipe with an explicitly stable compiled separator; input bytes remain live aliases.

</div>
</article>

<article class="doc-entry" id="v-byte-view-split-seq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">byte-view-split-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-byte-view-split-seq" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L159" title="View source at line 159">Source</a></span></div>
<div class="doc-body">

Begin a fresh persistent raw-separator producer, observing input and delimiter changes at each step.

</div>
</article>

<article class="doc-entry" id="v-bytes-lines">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-lines</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-lines" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L199" title="View source at line 199">Source</a></span></div>
<div class="doc-body">

Begin a fresh persistent LF-line traversal over a complete fixed buffer.

</div>
</article>

<article class="doc-entry" id="v-bytes-split">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-split</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-split" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L175" title="View source at line 175">Source</a></span></div>
<div class="doc-body">

Split complete fixed buffers into retained mutable input aliases.

</div>
</article>

<article class="doc-entry" id="v-bytes-split-after">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-split-after</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-split-after" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Split complete fixed buffers after each raw delimiter.

</div>
</article>

<article class="doc-entry" id="v-bytes-split-after-limit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-split-after-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-split-after-limit" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L187" title="View source at line 187">Source</a></span></div>
<div class="doc-body">

Split complete fixed buffers after delimiters with an explicit optional maximum.

</div>
</article>

<article class="doc-entry" id="v-bytes-split-after-seq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-split-after-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-split-after-seq" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L195" title="View source at line 195">Source</a></span></div>
<div class="doc-body">

Begin a fresh persistent delimiter-including traversal over complete fixed buffers.

</div>
</article>

<article class="doc-entry" id="v-bytes-split-factory">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-split-factory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitMode" title="Data.Bytes.Split.ByteSplitMode">ByteSplitMode</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitFactory" title="Data.Bytes.Split.ByteSplitFactory">ByteSplitFactory</a></code><span class="doc-sig-links"><a href="#v-bytes-split-factory" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Retain complete input and separator owners in a fresh raw-separator traversal recipe.

</div>
</article>

<article class="doc-entry" id="v-bytes-split-limit">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-split-limit</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Option/#t-Option" title="Data.Core.Option">Option</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-split-limit" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L183" title="View source at line 183">Source</a></span></div>
<div class="doc-body">

Split complete fixed buffers with an explicit optional maximum.

</div>
</article>

<article class="doc-entry" id="v-bytes-split-seq">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">bytes-split-seq</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Producer/#t-Producer" title="Data.Producer.Producer">Producer</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.Split/#t-ByteSplitState" title="Data.Bytes.Split.ByteSplitState">ByteSplitState</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes.View/#t-ByteView" title="Data.Bytes.View.ByteView">ByteView</a></code><span class="doc-sig-links"><a href="#v-bytes-split-seq" title="Permalink">#</a><a href="/packages/stdlib/Data.Bytes.Split/source/#L191" title="View source at line 191">Source</a></span></div>
<div class="doc-body">

Begin a fresh persistent traversal over complete input and raw separator buffers.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Data/Bytes/Split.cinch`](/packages/stdlib/Data.Bytes.Split/source/). Source SHA-256: `2b534f9538e49c5e0d364721a5fca28cbda3808586ceb8c4e163bec1e0b9d26d`.
