# Text.Zip

Read stored, deflated and Zip64 zip archives with CRC-32 verification and write stored or deflated archives.

<div class="doc-mod-bar"><div class="doc-mod-bar-primary"><span class="support-tag support-tag--source">Ship source</span> <code>import Text.Zip</code></div><div class="doc-mod-bar-links"><a href="/packages/stdlib/Text.Zip/source/">View source</a><a href="/packages/search/?q=%2BText.Zip">Search module</a><a href="/reference-sources/Text.Zip.cinch" download="Text.Zip.cinch">Download .cinch</a><a href="/packages/stdlib/">All stdlib modules</a></div></div>

**Dependencies:** [Data.Bytes](Data.Bytes.md), [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Data.Int](Data.Int.md), [Data.List](Data.List.md), [Data.Machine](Data.Machine.md), [Data.Nat](Data.Nat.md), [Data.Result](Data.Result.md), [Data.String](Data.String.md), [Text.Checksum](Text.Checksum.md), [Text.Zlib](Text.Zlib.md).

<details class="doc-closure"><summary>Ship-source closure (8 files)</summary>

Copy these modules beside your entry file before importing `Text.Zip`: [Data.Bytes.Build](Data.Bytes.Build.md), [Data.Bytes.Word](Data.Bytes.Word.md), [Text.Adler32](Text.Adler32.md), [Text.Binary](Text.Binary.md), [Text.Checksum](Text.Checksum.md), [Text.Crc64](Text.Crc64.md), [Text.Zip](Text.Zip.md), [Text.Zlib](Text.Zlib.md).

```sh
cp seed/prelude/Data/Bytes/Build.cinch ./Data.Bytes.Build.cinch
cp seed/prelude/Data/Bytes/Word.cinch ./Data.Bytes.Word.cinch
cp seed/prelude/Text/Adler32.cinch ./Text.Adler32.cinch
cp seed/prelude/Text/Binary.cinch ./Text.Binary.cinch
cp seed/prelude/Text/Checksum.cinch ./Text.Checksum.cinch
cp seed/prelude/Text/Crc64.cinch ./Text.Crc64.cinch
cp seed/prelude/Text/Zip.cinch ./Text.Zip.cinch
cp seed/prelude/Text/Zlib.cinch ./Text.Zlib.cinch
```

</details>

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

<details class="doc-synopsis"><summary>Synopsis (29 declarations)</summary>
<ul class="doc-synopsis-list"><li><a class="syn-name" href="#t-ZipArchive">ZipArchive</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipArchive</strong></code></li><li><a class="syn-name" href="#t-ZipEntry">ZipEntry</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipEntry</strong></code></li><li><a class="syn-name" href="#t-ZipError">ZipError</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipError</strong></code></li><li><a class="syn-name" href="#t-ZipFile">ZipFile</a><code class="syn-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipFile</strong></code></li><li><a class="syn-name" href="#v-zip-comment">zip-comment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zip-deflate">zip-deflate</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-deflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entries">zip-entries</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a></code></li><li><a class="syn-name" href="#v-zip-entry-comment">zip-entry-comment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zip-entry-compressed-size">zip-entry-compressed-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-compressed-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-crc32">zip-entry-crc32</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-crc32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code></li><li><a class="syn-name" href="#v-zip-entry-creator">zip-entry-creator</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-creator</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-external-attrs">zip-entry-external-attrs</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-external-attrs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-extra">zip-entry-extra</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-extra</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zip-entry-flags">zip-entry-flags</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-is-directory">zip-entry-is-directory</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-is-directory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code></li><li><a class="syn-name" href="#v-zip-entry-method">zip-entry-method</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-mode">zip-entry-mode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-entry-modified">zip-entry-modified</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-modified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code></li><li><a class="syn-name" href="#v-zip-entry-name">zip-entry-name</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zip-entry-size">zip-entry-size</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-entry-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-error-message">zip-error-message</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code></li><li><a class="syn-name" href="#v-zip-file">zip-file</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-file</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code></li><li><a class="syn-name" href="#v-zip-file-with-comment">zip-file-with-comment</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-file-with-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code></li><li><a class="syn-name" href="#v-zip-file-with-mode">zip-file-with-mode</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-file-with-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code></li><li><a class="syn-name" href="#v-zip-file-with-modified">zip-file-with-modified</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-file-with-modified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code></li><li><a class="syn-name" href="#v-zip-open">zip-open</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li><li><a class="syn-name" href="#v-zip-read">zip-read</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a></code></li><li><a class="syn-name" href="#v-zip-store">zip-store</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-store</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code></li><li><a class="syn-name" href="#v-zip-write">zip-write</a><code class="syn-sig"><strong class="doc-decl-name tok-variable">zip-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code></li></ul>
</details>

# Types

<article class="doc-entry" id="t-ZipArchive">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipArchive</strong></code><span class="doc-sig-links"><a href="#t-ZipArchive" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L75" title="View source at line 75">Source</a></span></div>
<div class="doc-body">

A parsed archive: its entries in directory order and its comment.

</div>
</article>

<article class="doc-entry" id="t-ZipEntry">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipEntry</strong></code><span class="doc-sig-links"><a href="#t-ZipEntry" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L70" title="View source at line 70">Source</a></span></div>
<div class="doc-body">

A central directory entry: name, comment, extra fields, creator version, flags, method, MS-DOS time
and date, CRC-32, compressed and uncompressed sizes, external attributes, local header offset and
the modification time in Unix seconds.

</div>
</article>

<article class="doc-entry" id="t-ZipError">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipError</strong></code><span class="doc-sig-links"><a href="#t-ZipError" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L40" title="View source at line 40">Source</a></span></div>
<div class="doc-body">

A malformed archive, an unsupported entry or corrupted data.

<div class="doc-children"><h4>Constructors</h4><ul><li id="k-ZipNotFound"><code><strong class="tok-constructor">ZipNotFound</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipBadEntry"><code><strong class="tok-constructor">ZipBadEntry</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipTruncated"><code><strong class="tok-constructor">ZipTruncated</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipUnsupportedMethod"><code><strong class="tok-constructor">ZipUnsupportedMethod</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipChecksumMismatch"><code><strong class="tok-constructor">ZipChecksumMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipSizeMismatch"><code><strong class="tok-constructor">ZipSizeMismatch</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li><li id="k-ZipBadDeflate"><code><strong class="tok-constructor">ZipBadDeflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a></code></li></ul></div>
</div>
</article>

<article class="doc-entry" id="t-ZipFile">
<div class="doc-sig-bar"><code class="doc-sig"><span class="tok-keyword">data</span> <strong class="doc-decl-name tok-constructor">ZipFile</strong></code><span class="doc-sig-links"><a href="#t-ZipFile" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L320" title="View source at line 320">Source</a></span></div>
<div class="doc-body">

A file to write: name, comment, method, Unix mode (zero for none), modification time and data.

</div>
</article>

# Values

<article class="doc-entry" id="v-zip-comment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-comment" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L136" title="View source at line 136">Source</a></span></div>
<div class="doc-body">

Archive comment.

</div>
</article>

<article class="doc-entry" id="v-zip-deflate">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-deflate</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-deflate" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L36" title="View source at line 36">Source</a></span></div>
<div class="doc-body">

Method code for DEFLATE data.

</div>
</article>

<article class="doc-entry" id="v-zip-entries">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entries</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a></code><span class="doc-sig-links"><a href="#v-zip-entries" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L132" title="View source at line 132">Source</a></span></div>
<div class="doc-body">

Entries in central directory order.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-comment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-entry-comment" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L84" title="View source at line 84">Source</a></span></div>
<div class="doc-body">

Entry comment.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-compressed-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-compressed-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-compressed-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L108" title="View source at line 108">Source</a></span></div>
<div class="doc-body">

Compressed size in bytes.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-crc32">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-crc32</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Machine/" title="U32">U32</a></code><span class="doc-sig-links"><a href="#v-zip-entry-crc32" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L104" title="View source at line 104">Source</a></span></div>
<div class="doc-body">

Recorded CRC-32 of the uncompressed data.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-creator">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-creator</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-creator" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L92" title="View source at line 92">Source</a></span></div>
<div class="doc-body">

Version made by: the high byte names the creating system (3 is Unix).

</div>
</article>

<article class="doc-entry" id="v-zip-entry-external-attrs">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-external-attrs</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-external-attrs" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L116" title="View source at line 116">Source</a></span></div>
<div class="doc-body">

External attributes; for Unix creators the high sixteen bits hold the mode.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-extra">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-extra</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-entry-extra" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L88" title="View source at line 88">Source</a></span></div>
<div class="doc-body">

Raw central directory extra fields.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-flags">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-flags</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-flags" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L96" title="View source at line 96">Source</a></span></div>
<div class="doc-body">

General-purpose flags (bit 3: data descriptor, bit 11: UTF-8 name).

</div>
</article>

<article class="doc-entry" id="v-zip-entry-is-directory">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-is-directory</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bool/" title="Bool">Bool</a></code><span class="doc-sig-links"><a href="#v-zip-entry-is-directory" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L128" title="View source at line 128">Source</a></span></div>
<div class="doc-body">

Whether the entry names a directory (its name ends with a slash).

</div>
</article>

<article class="doc-entry" id="v-zip-entry-method">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-method</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-method" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L100" title="View source at line 100">Source</a></span></div>
<div class="doc-body">

Compression method.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-mode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-mode" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L124" title="View source at line 124">Source</a></span></div>
<div class="doc-body">

Unix permission and type bits for Unix creators, otherwise zero.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-modified">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-modified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a></code><span class="doc-sig-links"><a href="#v-zip-entry-modified" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L120" title="View source at line 120">Source</a></span></div>
<div class="doc-body">

Modification time in Unix seconds: the extended timestamp when present, otherwise the MS-DOS fields as UTC.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-name">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-name</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-entry-name" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L80" title="View source at line 80">Source</a></span></div>
<div class="doc-body">

Entry name.

</div>
</article>

<article class="doc-entry" id="v-zip-entry-size">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-entry-size</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-entry-size" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L112" title="View source at line 112">Source</a></span></div>
<div class="doc-body">

Uncompressed size in bytes.

</div>
</article>

<article class="doc-entry" id="v-zip-error-message">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-error-message</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.String/#t-String" title="Data.String.String">String</a></code><span class="doc-sig-links"><a href="#v-zip-error-message" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L57" title="View source at line 57">Source</a></span></div>
<div class="doc-body">

Render a zip error.

</div>
</article>

<article class="doc-entry" id="v-zip-file">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-file</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code><span class="doc-sig-links"><a href="#v-zip-file" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L326" title="View source at line 326">Source</a></span></div>
<div class="doc-body">

A file with the given name, method and data, no mode, no comment and no modification time.
Names ending in a slash are written as directories, always stored and without data.

</div>
</article>

<article class="doc-entry" id="v-zip-file-with-comment">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-file-with-comment</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code><span class="doc-sig-links"><a href="#v-zip-file-with-comment" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L338" title="View source at line 338">Source</a></span></div>
<div class="doc-body">

Attach a per-file comment.

</div>
</article>

<article class="doc-entry" id="v-zip-file-with-mode">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-file-with-mode</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code><span class="doc-sig-links"><a href="#v-zip-file-with-mode" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L330" title="View source at line 330">Source</a></span></div>
<div class="doc-body">

Record Unix permission and type bits (for example 0o100644 for a regular file or 0o40755 for a directory).

</div>
</article>

<article class="doc-entry" id="v-zip-file-with-modified">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-file-with-modified</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Int/" title="Int">Int</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a></code><span class="doc-sig-links"><a href="#v-zip-file-with-modified" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L334" title="View source at line 334">Source</a></span></div>
<div class="doc-body">

Record a modification time in Unix seconds (UTC).

</div>
</article>

<article class="doc-entry" id="v-zip-open">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-open</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipEntry" title="Text.Zip.ZipEntry">ZipEntry</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-open" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L290" title="View source at line 290">Source</a></span></div>
<div class="doc-body">

Read and verify an entry's data: stored bytes are copied, DEFLATE data is inflated, and the CRC-32
and uncompressed size must match the directory.

</div>
</article>

<article class="doc-entry" id="v-zip-read">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-read</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipArchive" title="Text.Zip.ZipArchive">ZipArchive</a></code><span class="doc-sig-links"><a href="#v-zip-read" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L178" title="View source at line 178">Source</a></span></div>
<div class="doc-body">

Parse the central directory of a complete archive.

</div>
</article>

<article class="doc-entry" id="v-zip-store">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-store</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Nat/" title="Nat">Nat</a></code><span class="doc-sig-links"><a href="#v-zip-store" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L32" title="View source at line 32">Source</a></span></div>
<div class="doc-body">

Method code for uncompressed data.

</div>
</article>

<article class="doc-entry" id="v-zip-write">
<div class="doc-sig-bar"><code class="doc-sig"><strong class="doc-decl-name tok-variable">zip-write</strong> :: <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.List/#t-List" title="Data.Core.List">List</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipFile" title="Text.Zip.ZipFile">ZipFile</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a> <span class="tok-operator">-&gt;</span> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Result/#t-Result" title="Data.Core.Result">Result</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Text.Zip/#t-ZipError" title="Text.Zip.ZipError">ZipError</a> <a class="doc-type-link tok-constructor" href="/packages/stdlib/Data.Bytes/" title="Bytes">Bytes</a></code><span class="doc-sig-links"><a href="#v-zip-write" title="Permalink">#</a><a href="/packages/stdlib/Text.Zip/source/#L342" title="View source at line 342">Source</a></span></div>
<div class="doc-body">

Write files and an archive comment as a complete archive.

</div>
</article>

# Reference provenance

Generated from checked `.cinchif` exports and [`seed/prelude/Text/Zip.cinch`](/packages/stdlib/Text.Zip/source/). Source SHA-256: `ffbb8942b896c6de9979d22637871b4673d6c7dcae70bdaaf408e8b7713e6ddd`.
