# Text.Tar

Read V7, USTAR, PAX and GNU tape archives and write USTAR or PAX archives, with SIMD header checksums.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Tar</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Tar/source/">View source</a><a href="/packages/search/?q=%2BText.Tar">Search module</a><a href="/reference-sources/Text.Tar.cinch" download="Text.Tar.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Data.Wide](Data.Wide.md).

<details class="doc-notes"><summary>Module notes</summary>

Tape archives: read V7, USTAR, PAX and GNU tar streams and write USTAR or PAX archives.
An archive is a sequence of 512-byte header blocks, each followed by its
file data padded to a whole block, and ends with two zero blocks. Reading
verifies every header checksum (summed sixteen bytes at a time in parallel lanes,
accepting both the unsigned and the historical signed sum), applies PAX
extended records and GNU long names and links to the entry they precede,
returns PAX global headers as entries of their own, and decodes octal and
base-256 numeric fields. Writing chooses USTAR when every field fits
(splitting long names into prefix and name) and otherwise precedes the entry
with a PAX extended header holding the fields that do not fit; modification
times are whole seconds. Names, links and owner names are raw bytes.

</details>

<details class="doc-synopsis"><summary>Synopsis (43 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-TarEntry">TarEntry</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TarEntry</strong></code></li><li><a class="syn-name" href="#t-TarError">TarError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TarError</strong></code></li><li><a class="syn-name" href="#t-TarHeader">TarHeader</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TarHeader</strong></code></li><li><a class="syn-name" href="#v-tar-block">tar-block</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-char">tar-char</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-contiguous">tar-contiguous</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-contiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-devmajor">tar-devmajor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-devmajor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-devminor">tar-devminor</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-devminor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-directory">tar-directory</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-directory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-entry">tar-entry</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-entry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarEntry" title="Text.Tar.TarEntry">TarEntry</a></code></li><li><a class="syn-name" href="#v-tar-entry-data">tar-entry-data</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-entry-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarEntry" title="Text.Tar.TarEntry">TarEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tar-entry-header">tar-entry-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-entry-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarEntry" title="Text.Tar.TarEntry">TarEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code></li><li><a class="syn-name" href="#v-tar-error-message">tar-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-tar-fifo">tar-fifo</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-fifo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-format">tar-format</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-format</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-format-gnu">tar-format-gnu</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-format-gnu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-format-pax">tar-format-pax</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-format-pax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-format-ustar">tar-format-ustar</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-format-ustar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-format-v7">tar-format-v7</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-format-v7</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-gid">tar-gid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-gid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-gname">tar-gname</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-gname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tar-hardlink">tar-hardlink</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-hardlink</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-header">tar-header</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code></li><li><a class="syn-name" href="#v-tar-kind">tar-kind</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-link">tar-link</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-link</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tar-mode">tar-mode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-mtime">tar-mtime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-mtime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-tar-name">tar-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tar-pax">tar-pax</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-pax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code></li><li><a class="syn-name" href="#v-tar-pax-global">tar-pax-global</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-pax-global</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-read">tar-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarEntry" title="Text.Tar.TarEntry">TarEntry</a>)</code></li><li><a class="syn-name" href="#v-tar-regular">tar-regular</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-regular</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-size">tar-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-symlink">tar-symlink</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-symlink</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-uid">tar-uid</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-uid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-tar-uname">tar-uname</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-uname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-tar-with-device">tar-with-device</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-with-device</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code></li><li><a class="syn-name" href="#v-tar-with-link">tar-with-link</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-with-link</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code></li><li><a class="syn-name" href="#v-tar-with-mode">tar-with-mode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-with-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code></li><li><a class="syn-name" href="#v-tar-with-mtime">tar-with-mtime</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-with-mtime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code></li><li><a class="syn-name" href="#v-tar-with-owner">tar-with-owner</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-with-owner</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code></li><li><a class="syn-name" href="#v-tar-with-pax">tar-with-pax</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-with-pax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code></li><li><a class="syn-name" href="#v-tar-write">tar-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">tar-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarEntry" title="Text.Tar.TarEntry">TarEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-TarEntry">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TarEntry</strong></code><span class="doc-sig-links"><a href="#t-TarEntry" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

One archive member: its header and its data (empty for links, directories and devices).

</div>
</article>

<article class="doc-entry" id="t-TarError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TarError</strong></code><span class="doc-sig-links"><a href="#t-TarError" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L193" title="View source at line 193">Source</a></span></div>
<div class="doc-body">

A malformed archive or an entry that cannot be written.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-TarBadHeader"><code><strong class="tok-constructor">TarBadHeader</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a></code></li><li id="k-TarBadChecksum"><code><strong class="tok-constructor">TarBadChecksum</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a></code></li><li id="k-TarTruncated"><code><strong class="tok-constructor">TarTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a></code></li><li id="k-TarFieldTooLong"><code><strong class="tok-constructor">TarFieldTooLong</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a></code></li><li id="k-TarBadPax"><code><strong class="tok-constructor">TarBadPax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-TarHeader">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">TarHeader</strong></code><span class="doc-sig-links"><a href="#t-TarHeader" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L87" title="View source at line 87">Source</a></span></div>
<div class="doc-body">

Entry metadata: type byte, name, link target, mode, user and group ids, data size, modification
time in seconds since 1970, user and group names, device numbers, PAX records and the format read.

</div>
</article>

# Values

<article class="doc-entry" id="v-tar-block">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-block</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-block" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L50" title="View source at line 50">Source</a></span></div>
<div class="doc-body">

Block device, type byte '4'.

</div>
</article>

<article class="doc-entry" id="v-tar-char">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-char</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-char" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L46" title="View source at line 46">Source</a></span></div>
<div class="doc-body">

Character device, type byte '3'.

</div>
</article>

<article class="doc-entry" id="v-tar-contiguous">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-contiguous</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-contiguous" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L62" title="View source at line 62">Source</a></span></div>
<div class="doc-body">

Contiguous file, type byte '7', read as a regular file.

</div>
</article>

<article class="doc-entry" id="v-tar-devmajor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-devmajor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-devmajor" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L153" title="View source at line 153">Source</a></span></div>
<div class="doc-body">

Device major number.

</div>
</article>

<article class="doc-entry" id="v-tar-devminor">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-devminor</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-devminor" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L157" title="View source at line 157">Source</a></span></div>
<div class="doc-body">

Device minor number.

</div>
</article>

<article class="doc-entry" id="v-tar-directory">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-directory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-directory" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L54" title="View source at line 54">Source</a></span></div>
<div class="doc-body">

Directory, type byte '5'.

</div>
</article>

<article class="doc-entry" id="v-tar-entry">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-entry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarEntry" title="Text.Tar.TarEntry">TarEntry</a></code><span class="doc-sig-links"><a href="#v-tar-entry" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Pair a header with data; the written size is always the data length.

</div>
</article>

<article class="doc-entry" id="v-tar-entry-data">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-entry-data</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarEntry" title="Text.Tar.TarEntry">TarEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tar-entry-data" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

An entry's data.

</div>
</article>

<article class="doc-entry" id="v-tar-entry-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-entry-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarEntry" title="Text.Tar.TarEntry">TarEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code><span class="doc-sig-links"><a href="#v-tar-entry-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

An entry's header.

</div>
</article>

<article class="doc-entry" id="v-tar-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-tar-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L206" title="View source at line 206">Source</a></span></div>
<div class="doc-body">

Render a tar error.

</div>
</article>

<article class="doc-entry" id="v-tar-fifo">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-fifo</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-fifo" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

Named pipe, type byte '6'.

</div>
</article>

<article class="doc-entry" id="v-tar-format">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-format</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-format" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L165" title="View source at line 165">Source</a></span></div>
<div class="doc-body">

Format bits observed when reading; zero for headers built in memory.

</div>
</article>

<article class="doc-entry" id="v-tar-format-gnu">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-format-gnu</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-format-gnu" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L82" title="View source at line 82">Source</a></span></div>
<div class="doc-body">

Format bit: GNU extensions.

</div>
</article>

<article class="doc-entry" id="v-tar-format-pax">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-format-pax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-format-pax" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L78" title="View source at line 78">Source</a></span></div>
<div class="doc-body">

Format bit: POSIX PAX extended headers.

</div>
</article>

<article class="doc-entry" id="v-tar-format-ustar">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-format-ustar</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-format-ustar" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L74" title="View source at line 74">Source</a></span></div>
<div class="doc-body">

Format bit: POSIX USTAR.

</div>
</article>

<article class="doc-entry" id="v-tar-format-v7">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-format-v7</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-format-v7" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

Format bit: original Unix V7 headers.

</div>
</article>

<article class="doc-entry" id="v-tar-gid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-gid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-gid" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L133" title="View source at line 133">Source</a></span></div>
<div class="doc-body">

Owner group id.

</div>
</article>

<article class="doc-entry" id="v-tar-gname">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-gname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tar-gname" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L149" title="View source at line 149">Source</a></span></div>
<div class="doc-body">

Owner group name.

</div>
</article>

<article class="doc-entry" id="v-tar-hardlink">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-hardlink</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-hardlink" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L38" title="View source at line 38">Source</a></span></div>
<div class="doc-body">

Hard link to an earlier entry, type byte '1'.

</div>
</article>

<article class="doc-entry" id="v-tar-header">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-header</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code><span class="doc-sig-links"><a href="#v-tar-header" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

A header of the given type and name with mode 0644 (0755 for directories) and every other field zero or empty.

</div>
</article>

<article class="doc-entry" id="v-tar-kind">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-kind</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-kind" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Type byte.

</div>
</article>

<article class="doc-entry" id="v-tar-link">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-link</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tar-link" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

Link target for hard and symbolic links.

</div>
</article>

<article class="doc-entry" id="v-tar-mode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-mode" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

Permission and mode bits.

</div>
</article>

<article class="doc-entry" id="v-tar-mtime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-mtime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-tar-mtime" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L141" title="View source at line 141">Source</a></span></div>
<div class="doc-body">

Modification time in whole seconds since 1970-01-01 UTC.

</div>
</article>

<article class="doc-entry" id="v-tar-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tar-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

Entry name, after any USTAR prefix, GNU long name or PAX path.

</div>
</article>

<article class="doc-entry" id="v-tar-pax">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-pax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>)</code><span class="doc-sig-links"><a href="#v-tar-pax" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L161" title="View source at line 161">Source</a></span></div>
<div class="doc-body">

PAX records that apply to this entry and are not represented by another field, in archive order.

</div>
</article>

<article class="doc-entry" id="v-tar-pax-global">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-pax-global</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-pax-global" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L66" title="View source at line 66">Source</a></span></div>
<div class="doc-body">

PAX global header, type byte 'g'. Read as an entry holding only its name and records, with no data.

</div>
</article>

<article class="doc-entry" id="v-tar-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarEntry" title="Text.Tar.TarEntry">TarEntry</a>)</code><span class="doc-sig-links"><a href="#v-tar-read" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L347" title="View source at line 347">Source</a></span></div>
<div class="doc-body">

Read every entry of a complete archive, stopping at the first zero block (normally one of two).

</div>
</article>

<article class="doc-entry" id="v-tar-regular">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-regular</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-regular" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L34" title="View source at line 34">Source</a></span></div>
<div class="doc-body">

Regular file, type byte '0'.

</div>
</article>

<article class="doc-entry" id="v-tar-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

Data size in bytes as recorded in the archive.

</div>
</article>

<article class="doc-entry" id="v-tar-symlink">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-symlink</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-symlink" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L42" title="View source at line 42">Source</a></span></div>
<div class="doc-body">

Symbolic link, type byte '2'.

</div>
</article>

<article class="doc-entry" id="v-tar-uid">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-uid</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-tar-uid" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

Owner user id.

</div>
</article>

<article class="doc-entry" id="v-tar-uname">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-uname</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tar-uname" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L145" title="View source at line 145">Source</a></span></div>
<div class="doc-body">

Owner user name.

</div>
</article>

<article class="doc-entry" id="v-tar-with-device">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-with-device</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code><span class="doc-sig-links"><a href="#v-tar-with-device" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L185" title="View source at line 185">Source</a></span></div>
<div class="doc-body">

Set device major and minor numbers.

</div>
</article>

<article class="doc-entry" id="v-tar-with-link">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-with-link</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code><span class="doc-sig-links"><a href="#v-tar-with-link" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L169" title="View source at line 169">Source</a></span></div>
<div class="doc-body">

Set the link target.

</div>
</article>

<article class="doc-entry" id="v-tar-with-mode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-with-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code><span class="doc-sig-links"><a href="#v-tar-with-mode" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L173" title="View source at line 173">Source</a></span></div>
<div class="doc-body">

Set the mode bits.

</div>
</article>

<article class="doc-entry" id="v-tar-with-mtime">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-with-mtime</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code><span class="doc-sig-links"><a href="#v-tar-with-mtime" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L181" title="View source at line 181">Source</a></span></div>
<div class="doc-body">

Set the modification time in seconds since 1970.

</div>
</article>

<article class="doc-entry" id="v-tar-with-owner">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-with-owner</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code><span class="doc-sig-links"><a href="#v-tar-with-owner" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L177" title="View source at line 177">Source</a></span></div>
<div class="doc-body">

Set user and group ids and names.

</div>
</article>

<article class="doc-entry" id="v-tar-with-pax">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-with-pax</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> (<a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>, <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a>) <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarHeader" title="Text.Tar.TarHeader">TarHeader</a></code><span class="doc-sig-links"><a href="#v-tar-with-pax" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L189" title="View source at line 189">Source</a></span></div>
<div class="doc-body">

Set extra PAX records written with the entry (forcing a PAX header) and reported when read back.

</div>
</article>

<article class="doc-entry" id="v-tar-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">tar-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarEntry" title="Text.Tar.TarEntry">TarEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Tar/#t-TarError" title="Text.Tar.TarError">TarError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-tar-write" title="Permalink">#</a><a href="/packages/stdlib/Text.Tar/source/#L567" title="View source at line 567">Source</a></span></div>
<div class="doc-body">

Write entries as a complete archive ending in two zero blocks. Each entry is written as USTAR
when every field fits and otherwise with a preceding PAX extended header.

</div>
</article>
