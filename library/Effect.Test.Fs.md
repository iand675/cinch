# Effect.Test.Fs

Immutable in-memory MapFs with synthesized directories, path validation, globbing, subtrees and filesystem conformance checks.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Effect.Test.Fs</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Effect.Test.Fs/source/">View source</a><a href="/packages/search/?q=%2BEffect.Test.Fs">Search module</a><a href="/reference-sources/Effect.Test.Fs.cinch" download="Effect.Test.Fs.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Algebra.Semigroup](Algebra.Semigroup.md), [Data.Bool](Data.Bool.md), [Data.Bytes](Data.Bytes.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Map](Data.Map.md), [Data.Nat](Data.Nat.md), [Data.Option](Data.Option.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Effect.Stream](Effect.Stream.md), [Text.Path](Text.Path.md).

<details class="doc-notes"><summary>Module notes</summary>

Immutable in-memory test filesystem and conformance verification suite.
Models slash-separated relative paths, automatic parent directory synthesis,
lexical directory listings, shell-pattern globbing, subtree projection,
stream opening, and full structural conformance checks.

</details>

<details class="doc-synopsis"><summary>Synopsis (33 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-MapDirEntry">MapDirEntry</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapDirEntry</strong></code></li><li><a class="syn-name" href="#t-MapFile">MapFile</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapFile</strong></code></li><li><a class="syn-name" href="#t-MapFileInfo">MapFileInfo</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapFileInfo</strong></code></li><li><a class="syn-name" href="#t-MapFs">MapFs</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapFs</strong></code></li><li><a class="syn-name" href="#t-MapFsError">MapFsError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapFsError</strong></code></li><li><a class="syn-name" href="#v-dir-entry-info">dir-entry-info</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dir-entry-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a></code></li><li><a class="syn-name" href="#v-dir-entry-is-dir">dir-entry-is-dir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dir-entry-is-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-dir-entry-name">dir-entry-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dir-entry-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-dir-entry-type">dir-entry-type</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">dir-entry-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-file-info-is-dir">file-info-is-dir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-info-is-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-file-info-mod-time">file-info-mod-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-info-mod-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-file-info-mode">file-info-mode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-info-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-file-info-name">file-info-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-info-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-file-info-size">file-info-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">file-info-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-fs-mode-dir">fs-mode-dir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fs-mode-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-fs-valid-path">fs-valid-path</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">fs-valid-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-map-dir">map-dir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a></code></li><li><a class="syn-name" href="#v-map-file">map-file</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-file</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a></code></li><li><a class="syn-name" href="#v-map-file-data">map-file-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-file-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-map-file-is-dir">map-file-is-dir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-file-is-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-map-file-mod-time">map-file-mod-time</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-file-mod-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code></li><li><a class="syn-name" href="#v-map-file-mode">map-file-mode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-file-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-map-fs-empty">map-fs-empty</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fs-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a></code></li><li><a class="syn-name" href="#v-map-fs-error-message">map-fs-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fs-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-map-fs-from-list">map-fs-from-list</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fs-from-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a></code></li><li><a class="syn-name" href="#v-map-fs-glob">map-fs-glob</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fs-glob</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code></li><li><a class="syn-name" href="#v-map-fs-insert">map-fs-insert</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fs-insert</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a></code></li><li><a class="syn-name" href="#v-map-fs-open-reader">map-fs-open-reader</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fs-open-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span>)</code></li><li><a class="syn-name" href="#v-map-fs-read-dir">map-fs-read-dir</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fs-read-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a>)</code></li><li><a class="syn-name" href="#v-map-fs-read-file">map-fs-read-file</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fs-read-file</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-map-fs-stat">map-fs-stat</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fs-stat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a></code></li><li><a class="syn-name" href="#v-map-fs-sub">map-fs-sub</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">map-fs-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a></code></li><li><a class="syn-name" href="#v-test-fs">test-fs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">test-fs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-MapDirEntry">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapDirEntry</strong></code><span class="doc-sig-links"><a href="#t-MapDirEntry" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L95" title="View source at line 95">Source</a></span></div>
<div class="doc-body">

Entry base name, directory flag, type mode bits, and full metadata.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MapDirEntry"><code><strong class="tok-constructor">MapDirEntry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MapFile">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapFile</strong></code><span class="doc-sig-links"><a href="#t-MapFile" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

File payload bytes, mode bits, modification timestamp in Unix seconds, and directory flag.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MapFile"><code><strong class="tok-constructor">MapFile</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MapFileInfo">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapFileInfo</strong></code><span class="doc-sig-links"><a href="#t-MapFileInfo" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Base name, byte size (zero for directories), mode bits, modification time, and directory flag.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MapFileInfo"><code><strong class="tok-constructor">MapFileInfo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MapFs">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapFs</strong></code><span class="doc-sig-links"><a href="#t-MapFs" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L138" title="View source at line 138">Source</a></span></div>
<div class="doc-body">

Ordered map from relative path strings to snapshotted MapFile values.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-MapFs"><code><strong class="tok-constructor">MapFs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Map/#t-Map" title="Data.Map.Map">Map</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-MapFsError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">MapFsError</strong></code><span class="doc-sig-links"><a href="#t-MapFsError" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L114" title="View source at line 114">Source</a></span></div>
<div class="doc-body">

Named errors returned by MapFs operations.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-FsInvalidPath"><code><strong class="tok-constructor">FsInvalidPath</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a></code></li><li id="k-FsNotFound"><code><strong class="tok-constructor">FsNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a></code></li><li id="k-FsIsDirectory"><code><strong class="tok-constructor">FsIsDirectory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a></code></li><li id="k-FsNotDirectory"><code><strong class="tok-constructor">FsNotDirectory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a></code></li><li id="k-FsBadPattern"><code><strong class="tok-constructor">FsBadPattern</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a></code></li></ul></div>
</div>
</article>

# Values

<article class="doc-entry" id="v-dir-entry-info">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dir-entry-info</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a></code><span class="doc-sig-links"><a href="#v-dir-entry-info" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L110" title="View source at line 110">Source</a></span></div>
<div class="doc-body">

Return the full MapFileInfo associated with a directory entry.

</div>
</article>

<article class="doc-entry" id="v-dir-entry-is-dir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dir-entry-is-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-dir-entry-is-dir" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L102" title="View source at line 102">Source</a></span></div>
<div class="doc-body">

Report whether a directory entry refers to a subdirectory.

</div>
</article>

<article class="doc-entry" id="v-dir-entry-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dir-entry-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-dir-entry-name" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L98" title="View source at line 98">Source</a></span></div>
<div class="doc-body">

Return the base name of a directory entry.

</div>
</article>

<article class="doc-entry" id="v-dir-entry-type">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">dir-entry-type</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-dir-entry-type" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L106" title="View source at line 106">Source</a></span></div>
<div class="doc-body">

Return the file type mode bits for a directory entry.

</div>
</article>

<article class="doc-entry" id="v-file-info-is-dir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-info-is-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-file-info-is-dir" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Report whether the described entry is a directory.

</div>
</article>

<article class="doc-entry" id="v-file-info-mod-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-info-mod-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-file-info-mod-time" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

Return the modification timestamp recorded for the entry.

</div>
</article>

<article class="doc-entry" id="v-file-info-mode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-info-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-file-info-mode" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

Return the mode bits recorded for the entry.

</div>
</article>

<article class="doc-entry" id="v-file-info-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-info-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-file-info-name" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L73" title="View source at line 73">Source</a></span></div>
<div class="doc-body">

Return the base name of the described entry, or "." for the root directory.

</div>
</article>

<article class="doc-entry" id="v-file-info-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">file-info-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-file-info-size" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L77" title="View source at line 77">Source</a></span></div>
<div class="doc-body">

Return the payload size in bytes (zero for directories).

</div>
</article>

<article class="doc-entry" id="v-fs-mode-dir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fs-mode-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-fs-mode-dir" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L30" title="View source at line 30">Source</a></span></div>
<div class="doc-body">

Mode bit marking a directory entry (bit 31, matching POSIX/virtual filesystem directory flag).

</div>
</article>

<article class="doc-entry" id="v-fs-valid-path">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">fs-valid-path</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-fs-valid-path" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L226" title="View source at line 226">Source</a></span></div>
<div class="doc-body">

Validate a relative slash-separated filesystem path.
"." is valid for the root directory; empty paths, leading or trailing slashes,
empty segments ("//"), and "." or ".." components are rejected.

</div>
</article>

<article class="doc-entry" id="v-map-dir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a></code><span class="doc-sig-links"><a href="#v-map-dir" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L47" title="View source at line 47">Source</a></span></div>
<div class="doc-body">

Construct an explicit empty directory MapFile with the given permission bits and timestamp.

</div>
</article>

<article class="doc-entry" id="v-map-file">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-file</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a></code><span class="doc-sig-links"><a href="#v-map-file" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L43" title="View source at line 43">Source</a></span></div>
<div class="doc-body">

Construct a regular MapFile with default mode and zero modification time.

</div>
</article>

<article class="doc-entry" id="v-map-file-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-file-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-map-file-data" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L51" title="View source at line 51">Source</a></span></div>
<div class="doc-body">

Read a fresh copy of the payload bytes stored in a MapFile.

</div>
</article>

<article class="doc-entry" id="v-map-file-is-dir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-file-is-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-map-file-is-dir" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L63" title="View source at line 63">Source</a></span></div>
<div class="doc-body">

Report whether a MapFile represents a directory via its flag or directory mode bit.

</div>
</article>

<article class="doc-entry" id="v-map-file-mod-time">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-file-mod-time</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="I64">I64</a></code><span class="doc-sig-links"><a href="#v-map-file-mod-time" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L59" title="View source at line 59">Source</a></span></div>
<div class="doc-body">

Return the modification timestamp of a MapFile.

</div>
</article>

<article class="doc-entry" id="v-map-file-mode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-file-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-map-file-mode" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L55" title="View source at line 55">Source</a></span></div>
<div class="doc-body">

Return the mode bits of a MapFile.

</div>
</article>

<article class="doc-entry" id="v-map-fs-empty">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fs-empty</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a></code><span class="doc-sig-links"><a href="#v-map-fs-empty" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Create an empty in-memory filesystem containing only the synthesized root directory ".".

</div>
</article>

<article class="doc-entry" id="v-map-fs-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fs-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-map-fs-error-message" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L127" title="View source at line 127">Source</a></span></div>
<div class="doc-body">

Render a human-readable diagnostic for a MapFsError.

</div>
</article>

<article class="doc-entry" id="v-map-fs-from-list">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fs-from-list</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a></code><span class="doc-sig-links"><a href="#v-map-fs-from-list" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L166" title="View source at line 166">Source</a></span></div>
<div class="doc-body">

Construct a MapFs from a list of (path, file) pairs, keeping later entries on duplicate keys.

</div>
</article>

<article class="doc-entry" id="v-map-fs-glob">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fs-glob</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>)</code><span class="doc-sig-links"><a href="#v-map-fs-glob" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L416" title="View source at line 416">Source</a></span></div>
<div class="doc-body">

Find all paths in a MapFs matching a shell pattern in ascending lexical order.

</div>
</article>

<article class="doc-entry" id="v-map-fs-insert">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fs-insert</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFile" title="Effect.Test.Fs.MapFile">MapFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a></code><span class="doc-sig-links"><a href="#v-map-fs-insert" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

Insert or replace a single path entry in a MapFs, snapshotting its byte payload.

</div>
</article>

<article class="doc-entry" id="v-map-fs-open-reader">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fs-open-reader</strong> :: <span class="tok-keyword">forall</span> (<span class="tok-variable">r</span> :: <span class="tok-constructor">Region</span>). <span class="tok-keyword">using</span> (<span class="tok-variable">io</span> :: <a class="doc-type-link tok-constructor" href="/reference/09-effects-and-capabilities/" title="Io">Io</a> <span class="tok-variable">r</span>) <span class="tok-operator">=&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Stream/#t-ByteReader" title="Effect.Stream.ByteReader">ByteReader</a> <span class="tok-variable">r</span>)</code><span class="doc-sig-links"><a href="#v-map-fs-open-reader" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L313" title="View source at line 313">Source</a></span></div>
<div class="doc-body">

Open a regular file in a MapFs as an Effect.Stream ByteReader.

</div>
</article>

<article class="doc-entry" id="v-map-fs-read-dir">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fs-read-dir</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapDirEntry" title="Effect.Test.Fs.MapDirEntry">MapDirEntry</a>)</code><span class="doc-sig-links"><a href="#v-map-fs-read-dir" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L319" title="View source at line 319">Source</a></span></div>
<div class="doc-body">

List direct children of a directory in ascending lexical order, deduplicating synthesized subdirectories.

</div>
</article>

<article class="doc-entry" id="v-map-fs-read-file">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fs-read-file</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-map-fs-read-file" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L297" title="View source at line 297">Source</a></span></div>
<div class="doc-body">

Read the complete byte contents of a regular file.

</div>
</article>

<article class="doc-entry" id="v-map-fs-stat">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fs-stat</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFileInfo" title="Effect.Test.Fs.MapFileInfo">MapFileInfo</a></code><span class="doc-sig-links"><a href="#v-map-fs-stat" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L282" title="View source at line 282">Source</a></span></div>
<div class="doc-body">

Inspect metadata for a file or directory, synthesizing parent directories automatically.

</div>
</article>

<article class="doc-entry" id="v-map-fs-sub">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">map-fs-sub</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFsError" title="Effect.Test.Fs.MapFsError">MapFsError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a></code><span class="doc-sig-links"><a href="#v-map-fs-sub" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L360" title="View source at line 360">Source</a></span></div>
<div class="doc-body">

Project a subdirectory of a MapFs into a new MapFs rooted at that directory.

</div>
</article>

<article class="doc-entry" id="v-test-fs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">test-fs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Effect.Test.Fs/#t-MapFs" title="Effect.Test.Fs.MapFs">MapFs</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a>) <a class="doc-type-link tok-constructor" href="/reference/04-types-and-kinds/" title="Unit">Unit</a></code><span class="doc-sig-links"><a href="#v-test-fs" title="Permalink">#</a><a href="/packages/stdlib/Effect.Test.Fs/source/#L469" title="View source at line 469">Source</a></span></div>
<div class="doc-body">

Run structural conformance checks on a MapFs and verify expected paths.
If expected is empty, the filesystem must contain no entries beneath root ".".
Otherwise, every listed path must exist and pass metadata, listing, read, glob,
subtree, and invalid-path rejection checks.

</div>
</article>
