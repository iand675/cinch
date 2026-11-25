# Text.Zip

Read stored, deflated and Zip64 zip archives with CRC-32 verification and write stored or Text.Flate-compressed deflated archives.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--current">cinch/stdlib</span> <code>import Text.Zip</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Zip/source/">View source</a><a href="/packages/search/?q=%2BText.Zip">Search module</a><a href="/reference-sources/Text.Zip.cinch" download="Text.Zip.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Checksum](Text.Checksum.md), [Text.Flate](Text.Flate.md), [Text.Zlib](Text.Zlib.md).

<details class="doc-notes"><summary>Module notes</summary>

Zip archives: read stored and deflated entries (including Zip64) and write stored or deflated archives.
Reading locates the end-of-central-directory record (and its Zip64
counterpart when present), decodes every central directory entry with its
Zip64 sizes and extended timestamp, and opens an entry on demand, inflating
DEFLATE data and verifying its CRC-32 and size. Writing emits each file's
local header, data and data descriptor followed by the central directory,
marking UTF-8 names, recording Unix modes and the modification time both as
MS-DOS fields and as an extended timestamp, and switching to Zip64 records
when sizes or offsets require them. Times are Unix seconds interpreted as
UTC; names and comments are raw bytes.

</details>

<details class="doc-synopsis"><summary>Synopsis (30 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ZipArchive">ZipArchive</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipArchive</strong></code></li><li><a class="syn-name" href="#t-ZipEntry">ZipEntry</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipEntry</strong></code></li><li><a class="syn-name" href="#t-ZipError">ZipError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipError</strong></code></li><li><a class="syn-name" href="#t-ZipFile">ZipFile</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipFile</strong></code></li><li><a class="syn-name" href="#v-zip-comment">zip-comment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zip-deflate">zip-deflate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-deflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entries">zip-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a></code></li><li><a class="syn-name" href="#v-zip-entry-comment">zip-entry-comment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zip-entry-compressed-size">zip-entry-compressed-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-compressed-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-crc32">zip-entry-crc32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-crc32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-zip-entry-creator">zip-entry-creator</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-creator</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-external-attrs">zip-entry-external-attrs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-external-attrs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-extra">zip-entry-extra</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-extra</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zip-entry-flags">zip-entry-flags</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-is-directory">zip-entry-is-directory</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-is-directory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-zip-entry-method">zip-entry-method</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-mode">zip-entry-mode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-modified">zip-entry-modified</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-modified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-zip-entry-name">zip-entry-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zip-entry-size">zip-entry-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-error-message">zip-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zip-file">zip-file</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-file</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code></li><li><a class="syn-name" href="#v-zip-file-with-comment">zip-file-with-comment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-file-with-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code></li><li><a class="syn-name" href="#v-zip-file-with-level">zip-file-with-level</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-file-with-level</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Flate/#t-FlateCompression" title="Text.Flate.FlateCompression">FlateCompression</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code></li><li><a class="syn-name" href="#v-zip-file-with-mode">zip-file-with-mode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-file-with-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code></li><li><a class="syn-name" href="#v-zip-file-with-modified">zip-file-with-modified</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-file-with-modified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code></li><li><a class="syn-name" href="#v-zip-open">zip-open</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zip-read">zip-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a></code></li><li><a class="syn-name" href="#v-zip-store">zip-store</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-store</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-write">zip-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ZipArchive">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipArchive</strong></code><span class="doc-sig-links"><a href="#t-ZipArchive" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L76" title="View source at line 76">Source</a></span></div>
<div class="doc-body">

A parsed archive: its entries in directory order and its comment.

</div>
</article>

<article class="doc-entry" id="t-ZipEntry">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipEntry</strong></code><span class="doc-sig-links"><a href="#t-ZipEntry" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L71" title="View source at line 71">Source</a></span></div>
<div class="doc-body">

A central directory entry: name, comment, extra fields, creator version, flags, method, MS-DOS time
and date, CRC-32, compressed and uncompressed sizes, external attributes, local header offset and
the modification time in Unix seconds.

</div>
</article>

<article class="doc-entry" id="t-ZipError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipError</strong></code><span class="doc-sig-links"><a href="#t-ZipError" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L41" title="View source at line 41">Source</a></span></div>
<div class="doc-body">

A malformed archive, an unsupported entry or corrupted data.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ZipNotFound"><code><strong class="tok-constructor">ZipNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipBadEntry"><code><strong class="tok-constructor">ZipBadEntry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipTruncated"><code><strong class="tok-constructor">ZipTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipUnsupportedMethod"><code><strong class="tok-constructor">ZipUnsupportedMethod</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipChecksumMismatch"><code><strong class="tok-constructor">ZipChecksumMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipSizeMismatch"><code><strong class="tok-constructor">ZipSizeMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipBadDeflate"><code><strong class="tok-constructor">ZipBadDeflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ZipFile">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipFile</strong></code><span class="doc-sig-links"><a href="#t-ZipFile" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L321" title="View source at line 321">Source</a></span></div>
<div class="doc-body">

A file to write: name, comment, method, compression level, Unix mode (zero for none), modification time and data.

</div>
</article>

# Values

<article class="doc-entry" id="v-zip-comment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-comment" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L137" title="View source at line 137">Source</a></span></div>
<div class="doc-body">

Archive comment.

</div>
</article>

<article class="doc-entry" id="v-zip-deflate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-deflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-deflate" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L37" title="View source at line 37">Source</a></span></div>
<div class="doc-body">

Method code for DEFLATE data.

</div>
</article>

<article class="doc-entry" id="v-zip-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a></code><span class="doc-sig-links"><a href="#v-zip-entries" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L133" title="View source at line 133">Source</a></span></div>
<div class="doc-body">

Entries in central directory order.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-comment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-entry-comment" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L85" title="View source at line 85">Source</a></span></div>
<div class="doc-body">

Entry comment.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-compressed-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-compressed-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-compressed-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L109" title="View source at line 109">Source</a></span></div>
<div class="doc-body">

Compressed size in bytes.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-crc32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-crc32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-zip-entry-crc32" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L105" title="View source at line 105">Source</a></span></div>
<div class="doc-body">

Recorded CRC-32 of the uncompressed data.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-creator">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-creator</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-creator" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L93" title="View source at line 93">Source</a></span></div>
<div class="doc-body">

Version made by: the high byte names the creating system (3 is Unix).

</div>
</article>

<article class="doc-entry" id="v-zip-entry-external-attrs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-external-attrs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-external-attrs" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L117" title="View source at line 117">Source</a></span></div>
<div class="doc-body">

External attributes; for Unix creators the high sixteen bits hold the mode.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-extra">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-extra</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-entry-extra" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L89" title="View source at line 89">Source</a></span></div>
<div class="doc-body">

Raw central directory extra fields.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-flags">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-flags" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L97" title="View source at line 97">Source</a></span></div>
<div class="doc-body">

General-purpose flags (bit 3: data descriptor, bit 11: UTF-8 name).

</div>
</article>

<article class="doc-entry" id="v-zip-entry-is-directory">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-is-directory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-zip-entry-is-directory" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L129" title="View source at line 129">Source</a></span></div>
<div class="doc-body">

Whether the entry names a directory (its name ends with a slash).

</div>
</article>

<article class="doc-entry" id="v-zip-entry-method">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-method" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L101" title="View source at line 101">Source</a></span></div>
<div class="doc-body">

Compression method.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-mode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-mode" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L125" title="View source at line 125">Source</a></span></div>
<div class="doc-body">

Unix permission and type bits for Unix creators, otherwise zero.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-modified">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-modified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-zip-entry-modified" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L121" title="View source at line 121">Source</a></span></div>
<div class="doc-body">

Modification time in Unix seconds: the extended timestamp when present, otherwise the MS-DOS fields as UTC.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-entry-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L81" title="View source at line 81">Source</a></span></div>
<div class="doc-body">

Entry name.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L113" title="View source at line 113">Source</a></span></div>
<div class="doc-body">

Uncompressed size in bytes.

</div>
</article>

<article class="doc-entry" id="v-zip-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zip-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L58" title="View source at line 58">Source</a></span></div>
<div class="doc-body">

Render a zip error.

</div>
</article>

<article class="doc-entry" id="v-zip-file">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-file</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code><span class="doc-sig-links"><a href="#v-zip-file" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L327" title="View source at line 327">Source</a></span></div>
<div class="doc-body">

A file with the given name, method and data, no mode, no comment and no modification time.
Names ending in a slash are written as directories, always stored and without data.

</div>
</article>

<article class="doc-entry" id="v-zip-file-with-comment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-file-with-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code><span class="doc-sig-links"><a href="#v-zip-file-with-comment" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L344" title="View source at line 344">Source</a></span></div>
<div class="doc-body">

Attach a per-file comment.

</div>
</article>

<article class="doc-entry" id="v-zip-file-with-level">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-file-with-level</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Flate/#t-FlateCompression" title="Text.Flate.FlateCompression">FlateCompression</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code><span class="doc-sig-links"><a href="#v-zip-file-with-level" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L332" title="View source at line 332">Source</a></span></div>
<div class="doc-body">

A deflated file with the given name, compression level and data, no mode, no comment and no modification time.
Names ending in a slash are written as directories, always stored and without data.

</div>
</article>

<article class="doc-entry" id="v-zip-file-with-mode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-file-with-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code><span class="doc-sig-links"><a href="#v-zip-file-with-mode" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L336" title="View source at line 336">Source</a></span></div>
<div class="doc-body">

Record Unix permission and type bits (for example 0o100644 for a regular file or 0o40755 for a directory).

</div>
</article>

<article class="doc-entry" id="v-zip-file-with-modified">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-file-with-modified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code><span class="doc-sig-links"><a href="#v-zip-file-with-modified" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L340" title="View source at line 340">Source</a></span></div>
<div class="doc-body">

Record a modification time in Unix seconds (UTC).

</div>
</article>

<article class="doc-entry" id="v-zip-open">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-open" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L291" title="View source at line 291">Source</a></span></div>
<div class="doc-body">

Read and verify an entry's data: stored bytes are copied, DEFLATE data is inflated, and the CRC-32
and uncompressed size must match the directory.

</div>
</article>

<article class="doc-entry" id="v-zip-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a></code><span class="doc-sig-links"><a href="#v-zip-read" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L179" title="View source at line 179">Source</a></span></div>
<div class="doc-body">

Parse the central directory of a complete archive.

</div>
</article>

<article class="doc-entry" id="v-zip-store">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-store</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-store" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L33" title="View source at line 33">Source</a></span></div>
<div class="doc-body">

Method code for uncompressed data.

</div>
</article>

<article class="doc-entry" id="v-zip-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-write" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L348" title="View source at line 348">Source</a></span></div>
<div class="doc-body">

Write files and an archive comment as a complete archive.

</div>
</article>
